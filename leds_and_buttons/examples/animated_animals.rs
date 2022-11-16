#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use microbit::Board;
use panic_halt as _;

const ANIMAL: [[[u8; 5]; 5]; 2] = [
    [
        [0, 1, 1, 0, 0],
        [1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
    ],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    loop {
        display.show(&mut timer, ANIMAL[0], 500);
        display.show(&mut timer, ANIMAL[1], 500);
    }
}
