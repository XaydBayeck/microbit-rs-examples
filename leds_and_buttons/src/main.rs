#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use microbit::{
    hal::{prelude::*, saadc::SaadcConfig, Saadc},
    Board,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut column1 = board.display_pins.col1.into_floating_input();

    let saadc_config = SaadcConfig::default();
    let mut saadc = Saadc::new(board.SAADC, saadc_config);

    loop {
        let result = saadc.read(&mut column1).unwrap();
        rprintln!("Light strength: {}", result);
    }
}
