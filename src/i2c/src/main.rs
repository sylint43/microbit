#![no_std]
#![no_main]

use core::{fmt::Write, str::from_utf8};
use cortex_m::prelude::{_embedded_hal_serial_Read, _embedded_hal_serial_Write};
use cortex_m_rt::entry;
use heapless::Vec;
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};
use microbit::{
    hal::{
        twim,
        uarte::{self, Baudrate, Parity},
    },
    pac::twim0::frequency::FREQUENCY_A,
    Board,
};
use panic_halt as _;
use serial_setup::UartePort;

mod serial_setup;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let byte = nb::block!(serial.read()).unwrap();
            // echo byte
            nb::block!(serial.write(byte)).unwrap();
            nb::block!(serial.flush()).unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "Error: buffer is full.\r\n").unwrap();
                break;
            }

            if byte == 13 {
                break;
            }
        }

        let command = from_utf8(&buffer).unwrap();

        match command.trim() {
            "accelerometer" => {
                while !sensor.accel_status().unwrap().xyz_new_data {}
                let data = sensor.accel_data().unwrap();
                write!(
                    serial,
                    "Accelerometer: x {} y {} z{}\r\n",
                    data.x, data.y, data.z
                )
                .unwrap();
            }
            "magnetometer" => {
                while !sensor.mag_status().unwrap().xyz_new_data {}
                let data = sensor.mag_data().unwrap();
                write!(
                    serial,
                    "Magnetometer: x {} y {} z {}\r\n",
                    data.x, data.y, data.z
                )
                .unwrap();
            }
            _ => {
                write!(serial, "Unknown command: {}\r\n", command).unwrap();
            }
        }
    }
}
