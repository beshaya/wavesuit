use std::ops::Mul;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PainterParams {
    pub global_brightness: f32,
    pub speed: f32,
    pub color: Color,
    pub secondary_colors: Vec<Color>,
}

impl PainterParams {
    pub fn serialize(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
    pub fn deserialize(string: &str) -> Result<PainterParams, Box<dyn Error>> {
        let p: PainterParams = serde_json::from_str(string)?;
        return Ok(p);
    }
}

pub trait Painter {
    fn paint(&mut self);
    fn length(&self) -> usize;
    fn get(&self, index:usize) -> Color;
    fn set_params(&mut self, params: PainterParams);
}

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

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        return Color {r: ((self.r as f32) * rhs ) as u8,
                      g: ((self.g as f32) * rhs ) as u8,
                      b: ((self.b as f32) * rhs ) as u8};
    }
}

struct SweepPainter {
    height: usize,
    width: usize,
    leds: LedString,
    tick: usize,
    params: PainterParams,
}

impl SweepPainter {
    pub fn new(width: usize, height: usize, params: PainterParams) -> Self {
        return SweepPainter { height: height, width: width, params: params, leds: new_led_string(width * height), tick: 0 };
    }
}

impl Painter for SweepPainter {
    fn paint(&mut self) {
        let speed = 0.8;
        let growth = 1.4;
        let center: f32 = (self.tick as f32 * speed) % (self.height as f32 * growth);
        for y in 0..self.height {
            let val: f32;
            let y_float = y as f32;
            if y_float >= center {
                val = (1.0 - (y_float - center) * 0.5).powf(3.);
            } else {
                val = (1.0 - (center - y_float) * 0.1).powf(3.);
            }
            for x in 0..self.width {
                let index = get_index(self.height, x, y);
                self.leds[index] =  self.params.color * val;
            }
        }
        self.tick += 1;
    }
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }
}

// Assumes vertical indexing, going down on first path.
fn get_index(height:usize, x:usize, y:usize) -> usize {
    let offset = x * height;
    if x % 2  == 0 {
        return offset + y;
    }
    return offset + height - y - 1;
}

fn get_offset_index(height:usize, x: usize, y: f32) -> usize {
    if x % 2 == 0 { // Even rows are slightly higher; treat as integer y.
        assert!(y - y.floor() == 0.0);
        return get_index(height, x, y as usize);
    }
    // Odd rows are slightly lower; treat as Z + 0.5 y where Z is an integer.
    assert!(y - y.floor() == 0.5);
    return get_index(height, x, (y - 0.5) as usize);
}

fn log_scale(lin_scale: u8) -> u8 {
    let exponent_factor = 255.0_f64.log2() / 255.0;
    let val = 2.0_f64.powf(lin_scale as f64 * exponent_factor) - 1.0;
    return val as u8;
}

// A line of LED's. This may be the entire strand, or a slice of them.
type LedString = Vec<Color>;

fn new_led_string (size: usize) -> LedString {
    let mut led_string = Vec::with_capacity(size);
    led_string.resize(size, Color::new(0x000000));
    return led_string
}

pub struct HexPainter {
    height: usize,
    width: usize,
    params: PainterParams,
    leds: LedString,
    tick: usize,
    start_color_idx: usize,
}

impl HexPainter {
    pub fn new(width: usize, height: usize, params: PainterParams) -> Self {
        return HexPainter { height: height, width: width, params: params,
                            leds: new_led_string(width * height), tick: 0, start_color_idx: 0};
    }
    // Paint a hexagonal region around [x, y]
    fn paint_hex(&mut self, x: usize, y: f32, color: Color) {
        assert!(x >= 1);
        assert!(x < self.width - 1);
        assert!(y >= 1.0);
        assert!(y < (self.height - 1) as f32);

        self.leds[get_offset_index(self.height, x, y)] = color;
        self.leds[get_offset_index(self.height, x, y + 1.0)] = color;
        self.leds[get_offset_index(self.height, x, y - 1.0)] = color;
        self.leds[get_offset_index(self.height, x - 1, y - 0.5)] = color;
        self.leds[get_offset_index(self.height, x - 1, y + 0.5)] = color;
        self.leds[get_offset_index(self.height, x + 1, y - 0.5)] = color;
        self.leds[get_offset_index(self.height, x + 1, y + 0.5)] = color;
    }
}

impl Painter for HexPainter {
    fn paint(&mut self) {
        let speed = 0.3;
        let growth = 1.8;
        let width: f32 = 0.0;
        let center: f32 = self.tick as f32 * speed;
        let mut y: f32 = 1.0;
        let mut x: usize = 2;
        let mut color_idx = self.start_color_idx;
        while y < self.height as f32 {
            let mut val: f32;
            if y >= center - width {
                val = 1.0 - (y - center) * 0.25;
                if val < 0.0 {
                    val = 0.0;
                } else if val > 1.0 {
                    val = 1.0;
                }
                self.paint_hex(x, y, self.params.color * val);
            } else {
                val = 1.0 - (center - width - y) * 0.05;
                self.paint_hex(x, y, self.params.secondary_colors[color_idx] * val);
            }
            x = if x == 2 { 1 } else { 2 };
            y += 2.5;
            color_idx = (color_idx + 1) % self.params.secondary_colors.len();
        }

        self.tick += 1;
        if center > self.height as f32 * growth {
            self.tick = 0;
            self.start_color_idx = (self.start_color_idx + 1) % self.params.secondary_colors.len();
        }
    }
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }
}

pub fn make_painter(painter_type: &str, width: usize, height: usize, params: PainterParams
                    ) -> Box<dyn Painter> {
    if painter_type == "hex" {
        return Box::new(HexPainter::new(width, height, params));
    }
    return Box::new(SweepPainter::new(width, height, params));
}
