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
    let leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let led_indexes: [(usize, usize); 16] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
        (4, 3),
        (4, 2),
        (4, 1),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0),
    ];

    let led_displays = led_indexes.iter().map(|(row, col)| {
        let mut leds = leds;
        leds[*row][*col] = 1;
        leds
    });

    for leds_display in led_displays.cycle() {
        display.show(&mut timer, leds_display, 30);
    }

    loop {}
}
