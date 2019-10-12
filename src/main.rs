#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::response::content;

use std::{error::Error, thread};
use std::sync::{Mutex};
use std::time::Duration;

use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use crossbeam_channel::{bounded, tick, Receiver, select};

mod display;
mod painter;

#[get("/")]
fn index(params: State<Mutex<painter::PainterParams>>) -> content::Json<String> {
    let data = params.lock().unwrap();
    content::Json(data.serialize())
}

// Set up signal handlers to listen on their own thread.
fn ctrl_channel() -> Result<Receiver<()>, Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT, SIGTERM])?;

    let (sender, receiver) = bounded(100);
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            let _ = sender.send(());
        }
    });

    Ok(receiver)
}

fn rocket_channel(params: painter::PainterParams) -> Result<Receiver<()>, Box<dyn Error>> {
    let (sender, receiver) = bounded(5);
    thread::spawn(move || {
        rocket::ignite()
            .manage(Mutex::new(params))
            .manage(sender)
            .mount("/", routes![index]).launch();
    });

    Ok(receiver)
}

fn main() -> Result<(), Box<dyn Error>> {
    let params = painter::PainterParams {global_brightness: 1.0,
                                         speed: 0.5,
                                         color: painter::Color::new(0xFFFFFF),
                                         secondary_colors: vec![
                                             painter::Color::new(0x4267B2),  // FB blue.
                                             painter::Color::new(0xFF5000),  // red-orange.
                                             painter::Color::new(0x898F9C),  // FB grey.
                                         ]};

    let ctrl_c_events = ctrl_channel()?;
    let webserver = rocket_channel(params.clone())?;
    let ticks = tick(Duration::from_millis(30));


    let width: usize = 4;
    let height: usize = 30;
    let dots: usize = width * height;

    // Remember to enable spi via raspi-config!
    let mut display = display::new(dots)?;

    let mut arm_painter = painter::make_painter("hex", width, height, params);

    loop {
        select! {
            recv(ticks) -> _ => {
                arm_painter.paint();
                for i in 0..dots {
                    let pixel = arm_painter.get(i);
                    display.set_pixel(i, pixel.r, pixel.g, pixel.b);
                }
                display.show()?;
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye");
                break;
            }
        }
    }
    return Ok(());
}
