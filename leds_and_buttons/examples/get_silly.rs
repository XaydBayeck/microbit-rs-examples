#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use microbit::display::blocking::Display;
use microbit::hal::{prelude::*, twim, Timer};
use microbit::Board;

use lsm303agr::{AccelOutputDataRate, Lsm303agr, Measurement};

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

const SILLY: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 1],
    [0, 0, 0, 1, 1],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let button_a = board.buttons.button_a.into_floating_input();
    let button_b = board.buttons.button_b.into_floating_input();

    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        twim::Frequency::K100,
    );

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    let mut old_data = sensor.accel_data().unwrap();

    let mut emotion = HAPP;

    loop {
        if button_a.is_low().unwrap() {
            emotion = HAPP;
        } else if button_b.is_low().unwrap() {
            emotion = SAD;
        } else if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            const THRESHOLD: u32 = 1000000 * 3 * 4;
            if distance(&data, &old_data) > THRESHOLD {
                old_data = data;
                emotion = SILLY;
            }
        };

        display.show(&mut timer, emotion, 50);
        display.clear();
    }
}

fn distance(lhs: &Measurement, rhs: &Measurement) -> u32 {
    ((lhs.x - rhs.x).pow(2) + (lhs.y - rhs.y).pow(2) + (lhs.z - rhs.z).pow(2)) as u32
}
