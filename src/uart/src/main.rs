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
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use serial_setup::UartePort;

mod serial_setup;

#[entry]
fn main() -> ! {
    rtt_init_print!();

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

    nb::block!(serial.write(b'X')).unwrap();
    nb::block!(serial.flush()).unwrap();

    loop {}
}
