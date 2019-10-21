/**
 * The "real" output Display for wavesuit. This uses blinkt to write SPI to a chain of
 * APA102 LED's.
 */
use std::error::Error;

use crate::display::Display;

#[cfg(target_arch = "arm")]
use blinkt::Blinkt;

pub struct BlinktDisplay {
    blinkt: Blinkt,
}

impl Display for BlinktDisplay {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
        self.blinkt.set_pixel(index, r, g, b);
    }
    fn show(&mut self) -> Result<(), Box<dyn Error>> {
        self.blinkt.show()?;
        Ok(())
    }
}

pub fn make_display(pixels: usize) -> Result<BlinktDisplay, Box<dyn Error>> {
    println!("Using a blinkt display");
    Ok(BlinktDisplay {blinkt: Blinkt::with_spi(16_000_000, pixels)?})
}
