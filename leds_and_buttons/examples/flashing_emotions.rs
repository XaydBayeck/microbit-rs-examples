#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::hal::{prelude::*, Timer};
use microbit::Board;
use panic_halt as _;

const HAPP: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

const SAD: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let button_a = board.buttons.button_a.into_floating_input();
    let button_b = board.buttons.button_b.into_floating_input();

    loop {
        if button_a.is_low().unwrap() {
            Some(HAPP)
        } else if button_b.is_low().unwrap() {
            Some(SAD)
        } else { None }
        .map(|emotion| {
            for _ in 0..4 {
                display.show(&mut timer, emotion, 200);
                display.clear();
                timer.delay_ms(200_u32);
            }
        });
    }
}
