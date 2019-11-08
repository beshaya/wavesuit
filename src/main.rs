use std::error::Error;

use base::Color;
use base::PainterParams;
use base::rocket_server;

mod display;
mod painter;
use painter::{Bounds,Painter};

#[cfg_attr(feature = "emulator", path = "runner/emulator.rs")]
#[cfg_attr(not(feature = "emulator"), path = "runner/default_runner.rs")]
pub mod runner;


fn main() -> Result<(), Box<dyn Error>> {

    let mut params = match PainterParams::load() {
        Ok(loaded_params) => loaded_params,
        Err(e) => {
            println!("Unable to load from file: {}", e);
            PainterParams {
                painter: String::from("hex"),
                global_brightness: 0.1,
                speed: 0.8,
                color: Color::new(0xFFFFFF),
                secondary_colors: vec![
                    Color::new(0x4267B2),  // FB blue.
                    Color::new(0x898F9C),  // FB grey.
                    Color::new(0xAC0000),
                    Color::new(0x8A8A00),
                    Color::new(0x8A008A),
                ],
                fade: 0.9,
                bidirectional: true,
                fade_after: true,
                color_index: 0,
            }
        }
    };

    let webserver = rocket_server(params.clone())?;

    params.apply_dimming();  // Apply dimming after caching the web version.

    let areas = vec![
        Bounds{height: 30, width: 16},
        Bounds{height: 30, width: 4},
        Bounds{height: 22, width: 4},
    ];

    let dots: usize = areas.iter().map(|&x: &Bounds| x.size()).sum();

    // Remember to enable spi via raspi-config!
    let mut display = runner::get_display(dots)?;

    let mut painters: Vec<Box<dyn Painter>> = areas.iter().map(|&x: &Bounds| {
        painter::make_painter(x, params.clone())
    }).collect();

    runner::run(move || {
        let mut led: usize = 0;
        for painter in painters.iter_mut() {
            painter.paint();
            for pix in 0..painter.length() {
                let pixel = painter.get(pix);
                display.set_pixel(led, pixel.r, pixel.g, pixel.b);
                led += 1;
            }
        }
        display.show();
        match webserver.try_recv() {
            Ok(new_params) => {
                if new_params.painter != params.painter {
                    painters = areas.iter().map(|&x: &Bounds| {
                        painter::make_painter(x, params.clone())
                    }).collect();
                } else {
                    for painter in painters.iter_mut() {
                        painter.set_params(new_params.clone());
                    }
                }
                params = new_params;
            },
            Err(_) => {}
        }
    })
}
