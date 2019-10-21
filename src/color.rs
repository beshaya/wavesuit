use std::ops;
use serde::{Serialize, Deserialize};

// A single RGB color.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Construct a new color from a 24 bit RGB hex code.
    pub fn new(hex_code: i32) -> Color {
        let r = (hex_code >> 16) as u8;
        let g = (hex_code >> 8) as u8;
        let b = hex_code as u8;
        Color { r: log_scale(r), g: log_scale(g), b: log_scale(b) }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        return Color {r: ((self.r as f32) * rhs ) as u8,
                      g: ((self.g as f32) * rhs ) as u8,
                      b: ((self.b as f32) * rhs ) as u8};
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = ((self.r as f32) * rhs ) as u8;
        self.g = ((self.g as f32) * rhs ) as u8;
        self.b = ((self.b as f32) * rhs ) as u8;
    }
}

fn log_scale(lin_scale: u8) -> u8 {
    //let exponent_factor = 255.0_f64.log2() / 255.0;
    //let val = 2.0_f64.powf(lin_scale as f64 * exponent_factor) - 1.0;
    //return val as u8;
    return lin_scale;
}
