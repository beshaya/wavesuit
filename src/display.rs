use std::error::Error;

#[cfg(target_arch = "arm")]
use blinkt::Blinkt;

pub trait Display {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8);
    fn show(&mut self) -> Result<(), Box<dyn Error>>;
}

// Arm implementation.
#[cfg(target_arch = "arm")]
pub struct BlinktDisplay {
    blinkt: Blinkt,
}
#[cfg(target_arch = "arm")]
impl Display for BlinktDisplay {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
        self.blinkt.set_pixel(index, r, g, b);
    }
    fn show(&mut self) -> Result<(), Box<dyn Error>> {
        self.blinkt.show()?;
        Ok(())
    }
}
#[cfg(target_arch = "arm")]
pub fn make_display(pixels: usize) -> Result<BlinktDisplay, Box<dyn Error>> {
    Ok(BlinktDisplay {blinkt: Blinkt::with_spi(16_000_000, pixels)?})
}

// Generic Implementation
#[cfg(not(target_arch = "arm"))]
struct FakeDisplay {}
#[cfg(not(target_arch = "arm"))]
impl Display for FakeDisplay {
    fn set_pixel(&mut self, _index: usize, _r: u8, _g: u8, _b: u8) {}
    fn show(&mut self) -> Result<(), Box<dyn Error> {
        Ok(())
    }
}
#[cfg(not(target_arch = "arm"))]
fn make_display(pixels: usize) -> Result<FakeDisplay, Box<dyn Error>> {
    Ok(FakeDisplay {})
}

pub fn new(pixels: usize) -> Result<Box<dyn Display>, Box<dyn Error>> {
    return Ok(Box::new( make_display(pixels)? ));
}
