use std::error::Error;

use crate::display::Display;

pub struct FakeDisplay {
    pixels: usize,
}
impl Display for FakeDisplay {
    fn set_pixel(&mut self, _index: usize, _r: u8, _g: u8, _b: u8) {
        assert!(_index < self.pixels);
    }
    fn show(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn set_offset(&mut self, count: usize) {}
}

pub fn make_display(pixels: usize) -> Result<FakeDisplay, Box<dyn Error>> {
    println!("Using a null display");
    Ok(FakeDisplay {pixels: pixels})
}
