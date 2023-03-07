#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    hal::{
        prelude::*,
        uarte::{self, Baudrate, Parity},
    },
    Board,
};
use panic_halt as _;
use serial_setup::UartePort;

mod serial_setup;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        nb::block!(serial.write(byte)).unwrap();
        nb::block!(serial.flush()).unwrap();
    }
}
