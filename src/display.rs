#[cfg_attr(arm)]
use blinkt::Blinkt;

pub trait Display {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8);
    fn show(&mut self);
}

// Arm implementation.
#[cfg_attr(arm)]
pub struct BlinktDisplay {
    blinkt: Blinkt,
}
#[cfg_attr(arm)]
impl Display for BlinktDisplay {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
        self.blinkt.set_pixel(index, r, g, b);
    }
    fn show(&mut self) {
        self.blinkt.show();
    }
}
#[cfg_attr(arm)]
pub fn new_display(pixels: usize) {
    BlinktDisplay {binkt: Blinkt::with_spi(16_000_000, pixels)}
}

// Generic Implementation
#[cfg_attr(not(arm))]
pub struct FakeDisplay {}
#[cfg_attr(not(arm))]
impl display::Display for FakeDisplay {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {}
    fn show(&mut self) {}
}
#[cfg_attr(not(arm))]
pub fn new(pixels: usize) {
    FakeDisplay {};
}
