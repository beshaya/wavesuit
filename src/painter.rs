extern crate rand;
use rand::prelude::*;

use base::Color;
use base::PainterParams;


pub trait Painter {
    fn paint(&mut self);
    fn length(&self) -> usize;
    fn get(&self, index:usize) -> Color;
    fn set_params(&mut self, params: PainterParams);
}

#[derive(Copy, Clone)]
struct Bounds {
    height: usize,
    width: usize,
}

impl Bounds {
    pub fn in_x(&self, x: i32) -> bool {x >= 0 && x < (self.width as i32)}
    pub fn in_y(&self, y: f32) -> bool {y >= 0.0 && y <= self.height as f32 - 0.5}
}

struct SweepPainter {
    height: usize,
    width: usize,
    leds: LedString,
    tick: usize,
    params: PainterParams,
}

impl SweepPainter {
    fn new(width: usize, height: usize, params: PainterParams) -> Self {
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
    fn new(width: usize, height: usize, params: PainterParams) -> Self {
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

pub struct FadePainter {
    bounds: Bounds,
    params: PainterParams,
    leds: LedString,
    tick: f32,
}

impl FadePainter {
    fn new(bounds: Bounds, params: PainterParams) -> Self {
        return FadePainter { bounds: bounds, params: params, leds: new_led_string(bounds.width * bounds.height),
                             tick: bounds.height as f32 };
    }
}
impl Painter for FadePainter {
    fn paint(&mut self) {
        let length: f32 = 0.7;
        for y in 0..self.bounds.height {
            for x in 0..self.bounds.width {
                let offset_y: f32 = (y as f32) + if x % 2 == 0 {0.0} else {0.5};
                let index: f32 = ((self.tick - offset_y) / (self.bounds.height as f32) / length) %
                    (self.params.secondary_colors.len() as f32);
                let color: Color = self.params.secondary_colors[index as usize];
                self.leds[get_offset_index(self.bounds.height, x, offset_y)] = color;
            }
        }
        self.tick += self.params.speed;
        if self.tick >= (self.bounds.height * self.params.secondary_colors.len()) as f32 * length * 2.0 {
            self.tick -= (self.bounds.height * self.params.secondary_colors.len()) as f32 * length;
        }
    }
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }
}

struct Trail {
    head_x: usize,
    head_y: f32,
    x_dir: i32,
    y_diag_start: f32,
}

pub struct LinePainter {
    bounds: Bounds,
    params: PainterParams,
    leds: LedString,
    tick: f32,
    start_color_idx: usize,
    trails: Vec<Trail>,
    rng: ThreadRng,
}

impl LinePainter {
    fn new(width: usize, height: usize, params: PainterParams) -> Self {
        let mut line = LinePainter { bounds: Bounds{height: height, width: width}, params: params,
                                     leds: new_led_string(width * height), tick: 0.0, start_color_idx: 0,
                                     trails: Vec::with_capacity(2), rng: rand::thread_rng()};
        line.trails.push(Trail {head_x: 0, head_y: 0.0, x_dir: 1, y_diag_start: 20.});
        line.trails.push(Trail {head_x: width - 1, head_y: 0.5, x_dir: -1, y_diag_start: 20.});
        return line;
    }
}

impl Painter for LinePainter {
    fn paint(&mut self) {
        let fade: f32 = 0.85;
        // Advance on integers.
        let advance: bool = self.tick.floor() < (self.tick + self.params.speed).floor();
        self.tick += self.params.speed;
        for idx in 0..self.leds.len() {
            self.leds[idx] *= fade;
        }
        let mut reset = false;
        for mut trail in self.trails.iter_mut() {
            if self.bounds.in_y(trail.head_y) {
                self.leds[get_offset_index(self.bounds.height, trail.head_x, trail.head_y)] = self.params.color;
            }
            if advance {
                if trail.head_y > trail.y_diag_start && self.bounds.in_x(trail.head_x as i32 + trail.x_dir) {
                    trail.head_y += 0.5;
                    trail.head_x = (trail.head_x as i32 + trail.x_dir) as usize;
                } else {
                    trail.head_y += 1.0;
                }
            }
            if trail.head_y > (self.bounds.height + 10) as f32 {
                reset = true;
            }
        }
        if reset {
            let y_diag = self.rng.gen_range(5.0, self.bounds.height as f32 - 10.0);
            for mut trail in self.trails.iter_mut() {
                // Reset eventually.
                trail.head_y = if trail.head_x % 2 == 0 {0.0} else {0.5};
                trail.x_dir *= -1;
                trail.y_diag_start = y_diag;
            }
        }

    }
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }
}

pub fn make_painter(width: usize, height: usize, params: PainterParams) -> Box<dyn Painter> {
    let bounds = Bounds { width: width, height: height };
    if params.painter == "hex" {
        return Box::new(HexPainter::new(width, height, params));
    }
    if params.painter == "line" {
        return Box::new(LinePainter::new(width, height, params));
    }
    if params.painter == "fade" {
        return Box::new(FadePainter::new(bounds, params));
    }
    return Box::new(SweepPainter::new(width, height, params));
}
