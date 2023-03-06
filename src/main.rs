#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    loop {}
}