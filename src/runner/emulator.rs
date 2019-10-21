extern crate gio;
extern crate cairo;
extern crate gtk;

use std::env::args;
use std::error::Error;
use std::f64::consts::PI;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::Context;

use crate::runner::Runnable;

static mut x: f64 = 0.0;

// Based on https://github.com/gtk-rs/examples/blob/master/src/bin/cairotest.rs
fn build_ui(application: &gtk::Application)
{
    drawable(application, 500, 500, move |_, cr: &Context| {
        cr.scale(500f64, 500f64);

        unsafe {
            cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
            cr.arc(x, 0.53, 0.02, 0.0, PI * 2.);
            cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);
            cr.fill();
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
    gtk::timeout_add(30, tick);

}

pub fn run(mut core_alg: Box<dyn Runnable>) {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.cairotest"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    let tick = move || {
        core_alg.run();
        unsafe {
            x+= 0.001;
        }
        gtk::Continue(true)
    };
    gtk::timeout_add(30, tick);

    application.run(&Vec::new());
}
