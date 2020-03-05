use std::error::Error;

#[cfg_attr( target_arch = "arm", path = "blinkt_display.rs")]
#[cfg_attr( not(target_arch = "arm"), path = "fake_display.rs")]
pub mod display_impl;

/**
 * Provides a common interface for pattern output. We will only build one Display type.
 * Use new() to get the appropriate Display for the build configuration.
 */
pub trait Display {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8);
    fn show(&mut self) -> Result<(), Box<dyn Error>>;
    fn set_offset(&mut self, count: usize);
}

pub fn new(pixels: usize) -> Result<Box<dyn Display>, Box<dyn Error>> {
    return Ok(Box::new( display_impl::make_display(pixels)? ));
}
