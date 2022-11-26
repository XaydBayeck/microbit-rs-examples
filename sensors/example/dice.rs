#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};
use microbit::{
    hal::{twim::{Frequency, Twim}, Rng, Timer},
    Board, display::blocking::Display,
};
use panic_halt as _;

const DICE:[[[u8;5];5];6] = [
    [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,1,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ],
    [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,1,0,1,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ],
    [
        [0,0,0,0,1],
        [0,0,0,0,0],
        [0,0,1,0,0],
        [0,0,0,0,0],
        [1,0,0,0,0],
    ],
    [
        [0,0,0,0,0],
        [0,1,0,1,0],
        [0,0,0,0,0],
        [0,1,0,1,0],
        [0,0,0,0,0],
    ],
    [
        [1,0,0,0,1],
        [0,0,0,0,0],
        [0,0,1,0,0],
        [0,0,0,0,0],
        [1,0,0,0,1],
    ],
    [
        [0,1,0,1,0],
        [0,0,0,0,0],
        [0,1,0,1,0],
        [0,0,0,0,0],
        [0,1,0,1,0],
    ],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), Frequency::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    let mut rng = Rng::new(board.RNG);

    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);

    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();

            if data.x.abs() > 800 || data.y.abs() > 1600 || data.z.abs() > 4000 {
                let idx = rng.random_u8() as usize * 5 / 255;
                display.show(&mut timer, DICE[idx], 2000);
            }
        }
    }
}
