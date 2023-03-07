#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut current_index = (0, 0);

    loop {
        {
            leds[current_index.0][current_index.1] = 0;
            current_index = match current_index {
                (row, col) if row == 0 && col < 4 => (row, col + 1),
                (row, col) if row < 4 && col == 4 => (row + 1, col),
                (row, col) if row == 4 && col > 0 => (row, col - 1),
                (row, col) if row <= 4 && col == 0 => (row - 1, col),
                (_, _) => unreachable!(),
            };
            leds[current_index.0][current_index.1] = 1;
        }
        display.show(&mut timer, leds, 30);
    }
}
