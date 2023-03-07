#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
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

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let byte = nb::block!(serial.read()).unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "Error: buffer is full\r\n").unwrap();
                break;
            }

            if byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }
        }

        nb::block!(serial.flush()).unwrap();
    }
}
