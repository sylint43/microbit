#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{display::blocking::Display, board::Board, hal::{timer::Timer, prelude::* }};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let light_it_all = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    loop {
        display.show(&mut timer, light_it_all, 1000);
        display.clear();
        timer.delay_ms(1000_u32);
    }
}