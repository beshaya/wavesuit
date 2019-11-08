extern crate gio;
extern crate cairo;
extern crate gtk;

use std::error::Error;
use std::f64::consts::PI;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::Context;

use crate::display::Display;
use base::Color;

static mut LEDS: Vec<Color> = Vec::new();

struct EmulatorDisplay {
    offset: usize,
}

impl Display for EmulatorDisplay {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
        unsafe {
            LEDS[index + self.offset] = Color{r: r, g: g, b: b};
        }
    }

    fn show(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn set_count(&mut self, count: usize) {
        unsafe {
            self.offset = LAYOUT.count - count;
        }
    }
}

struct LedStrip {
    count: usize,
    x_start: f64,
    y_start: f64,
    x_spacing: f64,
    y_spacing: f64,
    backwards: bool,
}

struct LedLayout {
    strips: Vec<LedStrip>,
    count: usize,
}

impl LedLayout {
    fn add_strip(&mut self, strip: LedStrip) {
        self.count += strip.count;
        if !strip.backwards {
            self.strips.push(strip);
            return;
        }
        let backwards_strip = LedStrip{count: strip.count,
                                       x_start: strip.x_start + strip.x_spacing * (strip.count as f64 - 1.0),
                                       y_start: strip.y_start + strip.y_spacing * (strip.count as f64 - 1.0),
                                       x_spacing: -strip.x_spacing,
                                       y_spacing: -strip.y_spacing,
                                       backwards: false};
        self.strips.push(backwards_strip);
    }
    fn add_offset_panel(&mut self, mut orig_x: f64, mut orig_y: f64,
                        strip_x: f64, strip_y: f64, strip_len: usize,
                        panel_x: f64, panel_y: f64, panel_len: usize) {
        let mut backwards = false;
        for _i in 0..panel_len {
            self.add_strip(LedStrip{count: strip_len, x_start: orig_x, y_start: orig_y,
                                    x_spacing: strip_x, y_spacing: strip_y, backwards: backwards});
            orig_x += panel_x;
            orig_y += panel_y;

            // Add offset:
            backwards = !backwards;
            if backwards {
                orig_x += strip_x / 2.0;
                orig_y += strip_y / 2.0;
            } else {
                orig_x -= strip_x / 2.0;
                orig_y -= strip_y / 2.0;
            }
        }
    }
}

static mut LAYOUT: LedLayout = LedLayout{strips: Vec::new(), count: 0};

// Based on https://github.com/gtk-rs/examples/blob/master/src/bin/cairotest.rs
fn build_ui(application: &gtk::Application)
{
    drawable(application, 500, 500, move |_, cr: &Context| {
        cr.scale(500f64, 500f64);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.rectangle(0.0, 0.0, 1.0, 1.0);
        cr.fill();

        unsafe {
            let mut led_index: usize = 0;
            for strip in LAYOUT.strips.iter() {
                let mut x = strip.x_start;
                let mut y = strip.y_start;
                for _strip_index in 0..strip.count {
                    let color = LEDS[led_index];
                    cr.set_source_rgb(color.r as f64 / 255.0,
                                       color.g as f64 / 255.0,
                                       color.b as f64 / 255.0);
                    cr.arc(x, y, 0.007, 0.0, PI * 2.);
                    cr.fill();
                    x += strip.x_spacing;
                    y += strip.y_spacing;
                    led_index += 1;
                }
            }
        }
        Inhibit(false)
    });
}

fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where
    F: Fn(&DrawingArea, &Context) -> Inhibit + 'static,
{
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);

    window.add(&drawing_area);
    window.show_all();
    let tick = move || {
        drawing_area.queue_draw_area(0, 0, 500, 500);
        gtk::Continue(true)
    };
    gtk::timeout_add(42, tick);

}

pub fn get_display(dots: usize) -> Result<Box<dyn Display>, Box<dyn Error>> {
    unsafe {
        // Back
        let unit: f64 = 1.0 / 46.0;
        LAYOUT.add_offset_panel(0.5 + unit * 9.0 /* orig_x */, unit * 4.0 /* orig_y */,
                                0.0 /* strip_x */, unit /* strip_y */, 30 /* strip_len */,
                                -unit /* panel_x */, 0.0 /* panel_y */, 8 /* panel_len */);
        LAYOUT.add_offset_panel(0.5 - unit * 1.0 /* orig_x */, unit * 4.0 /* orig_y */,
                                0.0 /* strip_x */, unit /* strip_y */, 30 /* strip_len */,
                                -unit /* panel_x */, 0.0 /* panel_y */, 8 /* panel_len */);

        LAYOUT.add_offset_panel(unit * 7.0 /* orig_x */, unit * 4.0 /* orig_y */,
                                0.0 /* strip_x */, unit /* strip_y */, 30 /* strip_len */,
                                -unit /*panel_x */, 0.0 /* panel_y */ , 4 /* panel_len */);

        // Belt
        LAYOUT.add_offset_panel(0.5 - unit * 10.0 /* orig_x */, 1.0 - unit * 5.0 /* orig_y */,
                                unit /* strip_x */, 0.0 /* strip_y */, 22 /* strip_len */,
                                0.0 /* panel_x */, -unit /* panel_y */, 4 /* panel_len */);
    }

    println!("Using an emulator display");
    unsafe {
        LEDS.resize_with(dots, || {Color{r: 0, g: 0, b: 0}});
    }
    Ok(Box::new(EmulatorDisplay{offset: 0}))
}

pub fn run<F>(mut core_alg: F) -> Result<(), Box<dyn Error>>
where F: FnMut() + 'static {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.cairotest"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    let tick = move || {
        core_alg();
        gtk::Continue(true)
    };
    gtk::timeout_add(42, tick);

    application.run(&Vec::new());
    Ok(())
}
