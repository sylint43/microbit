#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    hal::{prelude::*, twim},
    pac::twim0::frequency::FREQUENCY_A,
    Board,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    loop {}
}
