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
pub struct Bounds {
    pub height: usize,
    pub width: usize,
}

impl Bounds {
    pub fn size(&self) -> usize { self.height * self.width }
    pub fn in_x(&self, x: i32) -> bool {x >= 0 && x < (self.width as i32)}
    pub fn in_y(&self, y: f32) -> bool {y >= 0.0 && y <= self.height as f32 - 0.5}
    pub fn in_(&self, x: i32, y: f32) -> bool {self.in_x(x) && self.in_y(y)}
    pub fn in_scale(&self, x: i32, y: f32, scale: f32) -> bool {
        (x as f32) > self.width as f32 * (1.0 - scale) / 2.0 &&
            (x as f32) < self.width as f32 * (1.0 + scale) / 2.0 &&
            y > self.height as f32 * (1.0 - scale) / 2.0 &&
            y < self.height as f32 * (1.0 + scale) / 2.0
    }
    pub fn flip_u(&self, y: usize) -> usize {self.height - y - 1}
    pub fn flip_x(&self, x: i32) -> i32 {self.width as i32 - x - 1}
    fn get_offset_index(&self, x: usize, y: f32) -> usize {
        if x % 2 == 0 { // Even rows are slightly higher; treat as integer y.
            assert!(y - y.floor() == 0.0);
            return get_index(self.height, x, y as usize);
        }
        // Odd rows are slightly lower; treat as Z + 0.5 y where Z is an integer.
        assert!(y - y.floor() == 0.5);
        return get_index(self.height, x, (y - 0.5) as usize);
    }
}

struct SweepPainter {
    height: usize,
    width: usize,
    leds: LedString,
    tick: usize,
    params: PainterParams,
    flip: bool,
    bounds: Bounds,
}

impl SweepPainter {
    fn new(width: usize, height: usize, params: PainterParams) -> Self {
        return SweepPainter { height: height, width: width, params: params,
                              leds: new_led_string(width * height), tick: 0,
                              flip: false, bounds: Bounds{height, width}};
    }
}

impl Painter for SweepPainter {
    fn paint(&mut self) {
        let speed = self.params.speed;
        let growth = 2.0;
        let center: f32 = self.tick as f32 * speed;
        for y in 0..self.height {
            let mut val: f32;
            let y_float = y as f32;
            if y_float >= center {
                val = (1.0 - (y_float - center) * 0.5).powf(3.);
            } else {
                val = (1.0 - (center - y_float) * 0.1).powf(3.);
            }
            if val < 0.0 {
                val = 0.0;
            }
            for x in 0..self.width {
                let index = get_index(self.height, x, if self.flip {self.bounds.flip_u(y)} else {y});
                self.leds[index] =  self.params.color * val;
            }
        }
        self.tick += 1;
        if center > self.height as f32 * growth {
            self.tick = 0;
            if self.params.bidirectional {
                self.flip = !self.flip;
            }
        }
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

// A line of LED's. This may be the entire strand, or a slice of them.
type LedString = Vec<Color>;

fn fade_all(leds: &mut LedString, fade_multiplier: f32) {
    for i in 0..leds.len() {
        leds[i] *= fade_multiplier;
    }
}

fn fill_every_other(parity: usize, color: Color, leds: &mut LedString) {
    for i in 0..leds.len() {
        if i % 2 == parity {
            leds[i] = color;
        }
    }
}

fn should_advance(tick: f32, speed: f32) -> bool {
    return (tick + speed).floor() != tick.floor();
}

fn new_led_string (size: usize) -> LedString {
    let mut led_string = Vec::with_capacity(size);
    led_string.resize(size, Color::new(0x000000));
    return led_string
}

#[derive(Copy, Clone)]
struct Hex {
    x: i32,
    y: f32,
    color: Color,
}

pub struct HexPainter {
    bounds: Bounds,
    params: PainterParams,
    leds: LedString,
    tick: f32,
    hexes: Vec<Hex>,
    rng: ThreadRng,
    hold_frames: i32,
    fade_frames: i32,
}

impl HexPainter {
    fn new(bounds: Bounds, params: PainterParams) -> Self {
        return HexPainter { bounds: bounds, params: params,
                            leds: new_led_string(bounds.width * bounds.height), tick: 0.0,
                            hexes: Vec::new(), rng: rand::thread_rng(), hold_frames: 0, fade_frames: 0};
    }
    // Paint a hexagonal region around [x, y]
    fn paint_hex(&mut self, x: usize, y: f32, color: Color) {
        assert!(x >= 1);
        assert!(x < self.bounds.width - 1);
        assert!(y >= 1.0);
        assert!(y < (self.bounds.height - 1) as f32);

        self.leds[self.bounds.get_offset_index(x, y)] = color;
        self.leds[self.bounds.get_offset_index(x, y + 1.0)] = color;
        self.leds[self.bounds.get_offset_index(x, y - 1.0)] = color;
        self.leds[self.bounds.get_offset_index(x - 1, y - 0.5)] = color;
        self.leds[self.bounds.get_offset_index(x - 1, y + 0.5)] = color;
        self.leds[self.bounds.get_offset_index(x + 1, y - 0.5)] = color;
        self.leds[self.bounds.get_offset_index(x + 1, y + 0.5)] = color;
    }
}

impl Painter for HexPainter {
    fn paint(&mut self) {
        let advance: bool = should_advance(self.tick, self.params.speed / 2.0);
        self.tick += self.params.speed / 2.0;
        let mut new_hexes: Vec<Hex> = Vec::new();
        if !self.params.fade_after {
            fade_all(&mut self.leds, self.params.fade);
        }
        if self.hold_frames > 0 {
            self.hold_frames -= 1;
            if self.hold_frames == 0 {
                self.fade_frames = 60;
            }
            return;
        }
        if self.fade_frames > 0 {
            self.fade_frames -= 1;
            fade_all(&mut self.leds, self.params.fade);
            return;
        }
        if self.hexes.len() == 0 {
            self.hexes.push(Hex{x: self.bounds.width as i32 - 2,
                                y: 1.0,
                                color: self.params.next_color()});
        }
        for i in 0..self.hexes.len() {
            let x = self.hexes[i].x;
            let y = self.hexes[i].y;
            self.paint_hex(x as usize, y, self.hexes[i].color);

            if advance {
                // Figure out where adjacent hexes are. Turns out this sucks in my grid.
                if self.bounds.flip_x(x) % 6 == 1 {
                    if self.bounds.in_(x - 2, y + 3.5) {
                        new_hexes.push(Hex{x: x - 1, y: y + 2.5, color: self.params.next_color()});
                    }
                    if self.bounds.in_(x - 4, y + 1.5) {
                        new_hexes.push(Hex{x: x - 3, y: y + 0.5, color: self.params.next_color()});
                    }
                } else if self.bounds.flip_x(x) % 6 == 2 {
                    if self.bounds.in_(x + 2, y + 3.5) {
                        new_hexes.push(Hex{x: x + 1, y: y + 2.5, color: self.params.next_color()});
                    }
                    if self.bounds.in_(x - 4, y + 1.5) {
                        new_hexes.push(Hex{x: x - 3, y: y + 0.5, color: self.params.next_color()});
                    }
                } else if self.bounds.flip_x(x) % 6 == 4 {
                    if self.bounds.in_(x - 4, y - 1.5) {
                        new_hexes.push(Hex{x: x - 3, y: y - 0.5, color: self.params.next_color()});
                    }
                    if self.bounds.in_(x - 2, y + 3.5) {
                        new_hexes.push(Hex{x: x - 1, y: y + 2.5, color: self.params.next_color()});
                    }
                } else if self.bounds.flip_x(x) % 6 == 5 {
                    if self.bounds.in_(x + 2, y + 3.5) {
                        new_hexes.push(Hex{x: x + 1, y: y + 2.5, color: self.params.next_color()});
                    }
                    if self.bounds.in_(x - 4, y - 1.5) {
                        new_hexes.push(Hex{x: x - 3, y: y - 0.5, color: self.params.next_color()});
                    }
                }
            }
        }
        if advance {
            self.hexes = Vec::new();
            if new_hexes.len() == 0 {
                if self.params.fade_after {
                    self.hold_frames = 40;
                } else {
                    self.fade_frames = 40;
                }
                return;
            }
            let mut pick: Vec<f32> = Vec::new();
            pick.resize_with(new_hexes.len(), || { self.rng.gen() });
            pick[self.rng.gen_range(0, new_hexes.len())] = 1.0;
            for i in 0..pick.len() {
                if pick[i] > 0.3 {
                    self.hexes.push(new_hexes[i]);
                }
            }
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
                self.leds[self.bounds.get_offset_index(x, offset_y)] = color;
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
    head_x: i32,
    head_y: f32,
    x_dir: i32,
    y_dir: f32,
    y_diag_start: f32,
}

pub struct LinePainter {
    bounds: Bounds,
    params: PainterParams,
    leds: LedString,
    tick: f32,
    color_index: usize,
    trails: Vec<Trail>,
    rng: ThreadRng,
}

impl LinePainter {
    fn new(width: usize, height: usize, params: PainterParams) -> Self {
        let mut line = LinePainter { bounds: Bounds{height: height, width: width}, params: params,
                                     leds: new_led_string(width * height), tick: 0.0, color_index: 0,
                                     trails: Vec::with_capacity(2), rng: rand::thread_rng()};
        line.trails.push(Trail {head_x: 0, head_y: 0.0, x_dir: 0, y_dir: 1.0, y_diag_start: 20.});
        line.trails.push(Trail {head_x: width as i32 - 1, head_y: 0.5, x_dir: 0, y_dir: 1.0, y_diag_start: 20.});
        return line;
    }
}

impl Painter for LinePainter {
    fn paint(&mut self) {
        // Advance on integers.
        self.tick += self.params.speed;
        let advance: bool = self.tick >= 1.0;  // Hex's are about 2 leds large
        if advance {
            self.tick = 0.0;
        }
        fade_all(&mut self.leds, self.params.fade);
        let mut reset = false;
        for mut trail in self.trails.iter_mut() {
            if self.bounds.in_(trail.head_x, trail.head_y) {
                self.leds[self.bounds.get_offset_index(trail.head_x as usize, trail.head_y)] =
                    self.params.secondary_colors[self.color_index % self.params.secondary_colors.len()];
            }
            if advance {
                if trail.head_y.floor() == trail.y_diag_start.floor() && trail.x_dir == 0 {
                    trail.y_dir /= 2.0;
                    trail.x_dir = if trail.head_x == 0 { 1 } else { -1 };
                }
                if trail.head_x == self.bounds.width as i32 / 2 || trail.head_x == self.bounds.width as i32 / 2- 1 {
                    trail.x_dir *= -1;
                }
                if !self.bounds.in_x(trail.head_x + trail.x_dir) {
                    trail.y_dir *= 2.0;
                    trail.x_dir = 0;
                }
                trail.head_x += trail.x_dir;
                trail.head_y += trail.y_dir;
                if trail.head_y > (self.bounds.height + 10) as f32 || trail.head_y < -10.0 {
                    reset = true;
                }
            }
        }
        if reset {
            self.color_index = self.color_index + 1;
            let y_diag = self.rng.gen_range(5.0, self.bounds.height as f32 - 5.0);
            for mut trail in self.trails.iter_mut() {
                trail.x_dir *= -1;
                trail.y_diag_start = y_diag;
                if self.params.bidirectional {
                    trail.y_dir *= -1.0;
                } else {
                    trail.head_y = if trail.head_x % 2 == 0 {0.0} else {0.5};
                }
            }
        }

    }
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }
}

struct Raindrops {
    bounds: Bounds,
    params: PainterParams,
    leds: LedString,
    tick: f32,
    trails: Vec<Trail>,
    rng: ThreadRng,
}

impl Raindrops {
    fn new(bounds: Bounds, params: PainterParams) -> Self {
        let mut trails = Vec::with_capacity(12);
        let mut rng = rand::thread_rng();
        trails.resize_with(8, || {
            Trail {head_x: rng.gen_range(0, bounds.width) as i32, head_y: rng.gen_range(-1 * (bounds.height as i32), 0) as f32,
                   x_dir: 0, y_dir: 1.0, y_diag_start: 0.0}});
        return Raindrops { bounds: bounds, params: params,
                                   leds: new_led_string(bounds.width * bounds.height), tick: 0.0,
                                   trails: trails, rng: rng};
    }
}

impl Painter for Raindrops {
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }
    fn paint(&mut self) {
        // Advance on integers.
        self.tick += self.params.speed;
        let advance: bool = self.tick >= 1.0;
        if advance {
            self.tick = 0.0;
        }
        for idx in 0..self.leds.len() {
            self.leds[idx] *= self.params.fade;
        }
        for mut trail in self.trails.iter_mut() {
            if self.bounds.in_y(trail.head_y) {
                self.leds[get_index(self.bounds.height, trail.head_x as usize, trail.head_y as usize)] = self.params.color;
            }
            if advance {
                trail.head_y += trail.y_dir;
            }
            if !self.bounds.in_scale(trail.head_x, trail.head_y, 2.0) {
                trail.head_x = self.rng.gen_range(0, self.bounds.width) as i32;
                if !self.params.bidirectional || self.rng.gen::<f32>() > 0.5 {
                    trail.head_y = self.rng.gen_range(-1 * (self.bounds.height as i32), 0) as f32;
                    trail.y_dir = 1.0;
                } else {
                    trail.head_y = self.rng.gen_range(self.bounds.height, self.bounds.height * 2) as f32;
                    trail.y_dir = -1.0;
                }
            }
        }
    }
}

struct Disco {
    params: PainterParams,
    leds: LedString,
    tick: f64,
    last_beat: f64,
}

impl Disco {
    fn new(bounds: Bounds, mut params: PainterParams) -> Self {
        if params.secondary_colors.len() < 1 {
            params.secondary_colors.push(Color::new(0xFF0000));
        }
        if params.secondary_colors.len() < 2 {
            params.secondary_colors.push(Color::new(0x0000FF));
        }
        Disco {params: params, leds: new_led_string(bounds.size()), tick: 0.0,
               last_beat: 0.0}
    }
}

impl Painter for Disco {
    fn length(&self) -> usize { self.leds.len() }
    fn get(&self, index: usize) -> Color { self.leds[index] }
    fn set_params(&mut self, params: PainterParams) { self.params = params; }

    fn paint(&mut self) {
        let bpm: f64 = (self.params.speed * 130.0).into();  // normalize to 130 bpm
        let bps: f64 = bpm / 60.0;
        let second: f64 = self.tick * 0.03;
        let beat: f64 = bps * second;
        fade_all(&mut self.leds, self.params.fade);
        if self.last_beat.floor() != beat.floor() {
            fill_every_other((beat.floor() as usize) % 2,
                           self.params.next_color(),
                           &mut self.leds);
        }

        self.last_beat = beat;
        self.tick += 1.0;
    }
}

pub fn make_painter(bounds: Bounds, params: PainterParams) -> Box<dyn Painter> {
    if params.painter == "hex" {
        return Box::new(HexPainter::new(bounds, params));
    }
    if params.painter == "line" {
        return Box::new(LinePainter::new(bounds.width, bounds.height, params));
    }
    if params.painter == "fade" {
        return Box::new(FadePainter::new(bounds, params));
    }
    if params.painter == "rain" {
        return Box::new(Raindrops::new(bounds, params));
    }
    if params.painter == "disco" {
        return Box::new(Disco::new(bounds, params));
    }
    return Box::new(SweepPainter::new(bounds.width, bounds.height, params));
}
