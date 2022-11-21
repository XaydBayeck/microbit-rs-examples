#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use microbit::{
    gpio::DisplayPins,
    hal::{
        gpio::{
            p0::{P0_11, P0_15, P0_19, P0_21, P0_22, P0_24, P0_28, P0_30, P0_31},
            p1::P1_05,
            Level, Output, PushPull,
        },
        prelude::*,
        saadc::SaadcConfig,
        timer::{Instance, Timer},
        Saadc,
    },
    Board,
};
use panic_halt as _;

const LIGHT: [[u8; 5]; 5] = [
    [1, 0, 1, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 1, 0, 1],
];

struct DisplayByLight {
    col1: P0_28<Output<PushPull>>,
    col2: P0_11<Output<PushPull>>,
    col3: P0_31<Output<PushPull>>,
    col4: P1_05<Output<PushPull>>,
    col5: P0_30<Output<PushPull>>,
    row1: P0_21<Output<PushPull>>,
    row2: P0_22<Output<PushPull>>,
    row3: P0_15<Output<PushPull>>,
    row4: P0_24<Output<PushPull>>,
    row5: P0_19<Output<PushPull>>,
    display_duration_ms: u32,
}

impl DisplayByLight {
    pub fn new(pins: DisplayPins) -> Self {
        Self {
            col1: pins.col1,
            col2: pins.col2,
            col3: pins.col3,
            col4: pins.col4,
            col5: pins.col5,
            row1: pins.row1,
            row2: pins.row2,
            row3: pins.row3,
            row4: pins.row4,
            row5: pins.row5,
            display_duration_ms: 1,
        }
    }

    pub fn read(self, saadc: &mut Saadc) -> nb::Result<(i16, Self), ()> {
        let mut col3 = self.col3.into_floating_input();
        let light_level = saadc.read(&mut col3)?;
        Ok((
            light_level,
            Self {
                col3: col3.into_push_pull_output(Level::High),
                ..self
            },
        ))
    }

    pub fn set_duration(&mut self, duration_ms: u32) {
        self.display_duration_ms = duration_ms;
    }

    pub fn show<T: Instance>(
        &mut self,
        timer: &mut Timer<T>,
        image: [[u8; 5]; 5],
        duration_ms: u32,
    ) {
        for _ in 0..(duration_ms / (4 * self.display_duration_ms)) {
            // row1
            self.row1.set_high().unwrap();
            if image[0][0] == 1 {
                self.col1.set_low().unwrap();
            } else {
                self.col1.set_high().unwrap();
            }
            if image[0][1] == 1 {
                self.col2.set_low().unwrap();
            } else {
                self.col2.set_high().unwrap();
            }
            if image[0][2] == 1 {
                self.col3.set_low().unwrap();
            } else {
                self.col3.set_high().unwrap();
            }
            if image[0][3] == 1 {
                self.col4.set_low().unwrap();
            } else {
                self.col4.set_high().unwrap();
            }
            if image[0][4] == 1 {
                self.col5.set_low().unwrap();
            } else {
                self.col5.set_high().unwrap();
            }
            timer.delay_ms(self.display_duration_ms);
            self.row1.set_low().unwrap();
            // row2
            self.row2.set_high().unwrap();
            if image[1][0] == 1 {
                self.col1.set_low().unwrap();
            } else {
                self.col1.set_high().unwrap();
            }
            if image[1][1] == 1 {
                self.col2.set_low().unwrap();
            } else {
                self.col2.set_high().unwrap();
            }
            if image[1][2] == 1 {
                self.col3.set_low().unwrap();
            } else {
                self.col3.set_high().unwrap();
            }
            if image[1][3] == 1 {
                self.col4.set_low().unwrap();
            } else {
                self.col4.set_high().unwrap();
            }
            if image[1][4] == 1 {
                self.col5.set_low().unwrap();
            } else {
                self.col5.set_high().unwrap();
            }
            timer.delay_ms(self.display_duration_ms);
            self.row2.set_low().unwrap();
            // row3
            self.row3.set_high().unwrap();
            if image[2][0] == 1 {
                self.col1.set_low().unwrap();
            } else {
                self.col1.set_high().unwrap();
            }
            if image[2][1] == 1 {
                self.col2.set_low().unwrap();
            } else {
                self.col2.set_high().unwrap();
            }
            if image[2][2] == 1 {
                self.col3.set_low().unwrap();
            } else {
                self.col3.set_high().unwrap();
            }
            if image[2][3] == 1 {
                self.col4.set_low().unwrap();
            } else {
                self.col4.set_high().unwrap();
            }
            if image[2][4] == 1 {
                self.col5.set_low().unwrap();
            } else {
                self.col5.set_high().unwrap();
            }
            timer.delay_ms(self.display_duration_ms);
            self.row3.set_low().unwrap();
            // row4
            self.row4.set_high().unwrap();
            if image[3][0] == 1 {
                self.col1.set_low().unwrap();
            } else {
                self.col1.set_high().unwrap();
            }
            if image[3][1] == 1 {
                self.col2.set_low().unwrap();
            } else {
                self.col2.set_high().unwrap();
            }
            if image[3][2] == 1 {
                self.col3.set_low().unwrap();
            } else {
                self.col3.set_high().unwrap();
            }
            if image[3][3] == 1 {
                self.col4.set_low().unwrap();
            } else {
                self.col4.set_high().unwrap();
            }
            if image[3][4] == 1 {
                self.col5.set_low().unwrap();
            } else {
                self.col5.set_high().unwrap();
            }
            timer.delay_ms(self.display_duration_ms);
            self.row4.set_low().unwrap();
            // row5
            self.row5.set_high().unwrap();
            if image[4][0] == 1 {
                self.col1.set_low().unwrap();
            } else {
                self.col1.set_high().unwrap();
            }
            if image[4][1] == 1 {
                self.col2.set_low().unwrap();
            } else {
                self.col2.set_high().unwrap();
            }
            if image[4][2] == 1 {
                self.col3.set_low().unwrap();
            } else {
                self.col3.set_high().unwrap();
            }
            if image[4][3] == 1 {
                self.col4.set_low().unwrap();
            } else {
                self.col4.set_high().unwrap();
            }
            if image[4][4] == 1 {
                self.col5.set_low().unwrap();
            } else {
                self.col5.set_high().unwrap();
            }
            timer.delay_ms(self.display_duration_ms);
            self.row5.set_low().unwrap();
        }
    }

    pub fn clear(&mut self) {
        self.col1.set_high().unwrap();
        self.col2.set_high().unwrap();
        self.col3.set_high().unwrap();
        self.col4.set_high().unwrap();
        self.col5.set_high().unwrap();
        self.row1.set_low().unwrap();
        self.row2.set_low().unwrap();
        self.row3.set_low().unwrap();
        self.row4.set_low().unwrap();
        self.row5.set_low().unwrap();
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut display = DisplayByLight::new(board.display_pins);

    let mut timer = Timer::new(board.TIMER0);

    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());

    loop {
        let (light_level, new_display) = display.read(&mut saadc).unwrap();
        display = new_display;
        if light_level < 1000 {
            display.show(&mut timer, LIGHT, 500);
            display.clear();
        }
    }
}
