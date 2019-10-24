#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::response::content;
use std::sync::{Mutex};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::{error::Error, thread};

mod color;
mod painter_params;

pub use color::Color;
pub use painter_params::PainterParams;

#[get("/")]
fn get(params: State<Mutex<PainterParams>>) -> content::Json<String> {
    let data = params.lock().unwrap();
    content::Json(data.serialize())
}

#[post("/", format = "application/json", data = "<json_params>")]
fn post(json_params: String,
        params: State<Mutex<PainterParams>>,
        sender: State<Sender<PainterParams>>) -> Result<(), Box<dyn Error>> {
    let mut new_params = PainterParams::deserialize(&json_params)?;
    let mut old_params = params.lock().unwrap();
    *old_params = new_params.clone();
    match new_params.save() {
        Err(e) => println!("Error writing to file: {}", e),
        Ok(()) => {}
    }
    new_params.apply_dimming();
    sender.send(new_params).unwrap();
    Ok(())
}

pub fn rocket_server(params: PainterParams) -> Result<Receiver<PainterParams>, Box<dyn Error>> {
    let (sender, receiver) = bounded::<PainterParams>(5);
    thread::spawn(move || {
        rocket::ignite()
            .manage(Mutex::new(params))
            .manage(sender)
            .mount("/", routes![get, post]).launch();
    });

    Ok(receiver)
}
