#![no_std]
#![no_main]

use core::fmt::Write;
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

    write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
    nb::block!(serial.flush()).unwrap();

    loop {}
}
