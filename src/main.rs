#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::response::content;

use std::{error::Error, thread};
use std::sync::{Mutex};

use crossbeam_channel::{bounded, Receiver, Sender};

mod color;
use crate::color::Color;
mod display;
mod painter;

#[cfg_attr(feature = "emulator", path = "runner/emulator.rs")]
#[cfg_attr(not(feature = "emulator"), path = "runner/default_runner.rs")]
pub mod runner;

#[get("/")]
fn get(params: State<Mutex<painter::PainterParams>>) -> content::Json<String> {
    let data = params.lock().unwrap();
    content::Json(data.serialize())
}

#[post("/", format = "application/json", data = "<json_params>")]
fn post(json_params: String,
        params: State<Mutex<painter::PainterParams>>,
        sender: State<Sender<painter::PainterParams>>) -> Result<(), Box<dyn Error>> {
    let mut new_params = painter::PainterParams::deserialize(&json_params)?;
    let mut old_params = params.lock().unwrap();
    *old_params = new_params.clone();
    new_params.apply_dimming();
    sender.send(new_params).unwrap();
    Ok(())
}

fn rocket_channel(params: painter::PainterParams) -> Result<Receiver<painter::PainterParams>, Box<dyn Error>> {
    let (sender, receiver) = bounded::<painter::PainterParams>(5);
    thread::spawn(move || {
        rocket::ignite()
            .manage(Mutex::new(params))
            .manage(sender)
            .mount("/", routes![get, post]).launch();
    });

    Ok(receiver)
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut params = painter::PainterParams {
        painter: String::from("fade"),
        global_brightness: 0.5,
        speed: 0.5,
        color: Color::new(0xFFFFFF),
        secondary_colors: vec![
            Color::new(0x4267B2),  // FB blue.
            Color::new(0x898F9C),  // FB grey.
        ]};

    let webserver = rocket_channel(params.clone())?;

    params.apply_dimming();  // Apply dimming after caching the web version.

    let width: usize = 4;
    let height: usize = 30;
    let dots: usize = width * height;

    // Remember to enable spi via raspi-config!
    let mut display = runner::get_display(dots)?;

    let mut arm_painter = painter::make_painter(width, height, params.clone());

    runner::run(move || {
        arm_painter.paint();
        for i in 0..dots {
            let pixel = arm_painter.get(i);
            display.set_pixel(i, pixel.r, pixel.g, pixel.b);
        }
        match webserver.try_recv() {
            Ok(new_params) => {
                if new_params.painter != params.painter {
                    arm_painter = painter::make_painter(width, height, new_params.clone());
                } else {
                    arm_painter.set_params(new_params.clone());
                }
                params = new_params;
            },
            Err(_) => {}
        }
    })
}
