extern crate gio;
extern crate cairo;
extern crate gtk;

use std::env::args;
use std::{error::Error, thread};
use std::f64::consts::PI;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::Context;

use crate::display::Display;

// Based on https://github.com/gtk-rs/examples/blob/master/src/bin/cairotest.rs
fn build_ui(application: &gtk::Application) {
    drawable(application, 500, 500, |_, cr: &Context| {
        println!("draw");
        cr.scale(500f64, 500f64);

        cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        cr.arc(0.04, 0.53, 0.02, 0.0, PI * 2.);
        cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);
        cr.fill();

        Inhibit(false)
    });
}

fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where
    F: Fn(&DrawingArea, &Context) -> Inhibit + 'static,
{
    println!("drawable");
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    println!("1");
    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);
    println!("2");

    window.add(&drawing_area);
    println!("3");
    window.show_all();

    println!("4");

    let tick = move || {
        drawing_area.queue_draw_area(0, 0, width, height);
        gtk::Continue(true)
    };
    gtk::timeout_add_seconds(1, tick);

}

pub struct Emulator {

}

impl Display for Emulator {
    fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
    }
    fn show(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn make_display(pixels: usize) -> Result<Emulator, Box<dyn Error>> {

    thread::Builder::new().name("gtk".to_string()).spawn(move || {

        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
        }
        let application = gtk::Application::new(
            Some("com.github.gtk-rs.examples.cairotest"),
            Default::default(),
        )
            .expect("Initialization failed...");
        application.connect_activate(|app| {
            build_ui(app);
        });
        application.run(&Vec::new());
        println!("ran");
    });
    Ok(Emulator {})
}
