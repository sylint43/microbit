#![no_std]
#![no_main]

use core::f32::consts::PI;

use calibration::{calibrated_measurement, Calibration};
use cortex_m_rt::entry;
use led::{direction_to_led, Direction};
use libm::atan2f;
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate, Measurement};
use microbit::{
    display::blocking::Display,
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
    Board,
};
use panic_halt as _;

mod calibration;
mod led;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    // Found using calc_calibration
    let calibration = Calibration {
        center: Measurement {
            x: -12104,
            y: 6434,
            z: -9734,
        },
        scale: Measurement {
            x: 1223,
            y: 1173,
            z: 1209,
        },
        radius: 70827,
    };

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let mut data = sensor.mag_data().unwrap();
        data = calibrated_measurement(data, &calibration);
        let theta = atan2f(data.y as f32, data.x as f32);

        let dir = if theta < -7. * PI / 8. {
            Direction::West
        } else if theta < -5. * PI / 8. {
            Direction::SouthWest
        } else if theta < -3. * PI / 8. {
            Direction::South
        } else if theta < -PI / 8. {
            Direction::SouthEast
        } else if theta < PI / 8. {
            Direction::East
        } else if theta < 3. * PI / 8. {
            Direction::NorthEast
        } else if theta < 5. * PI / 8. {
            Direction::North
        } else if theta < 7. * PI / 8. {
            Direction::NorthWest
        } else {
            Direction::West
        };

        display.show(&mut timer, direction_to_led(dir), 100);
    }
}
