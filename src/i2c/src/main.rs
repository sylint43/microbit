#![no_std]
#![no_main]

use cortex_m_rt::entry;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A, Board};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}