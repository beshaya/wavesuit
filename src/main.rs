use std::error::Error;

use base::Color;
use base::PainterParams;
use base::rocket_server;

mod display;
mod painter;

#[cfg_attr(feature = "emulator", path = "runner/emulator.rs")]
#[cfg_attr(not(feature = "emulator"), path = "runner/default_runner.rs")]
pub mod runner;


fn main() -> Result<(), Box<dyn Error>> {

    let mut params = match PainterParams::load() {
        Ok(loaded_params) => loaded_params,
        Err(e) => {
            println!("Unable to load from file: {}", e);
            PainterParams {
                painter: String::from("rain"),
                global_brightness: 0.5,
                speed: 0.5,
                color: Color::new(0xFFFFFF),
                secondary_colors: vec![
                    Color::new(0x4267B2),  // FB blue.
                    Color::new(0x898F9C),  // FB grey.
                ],
                fade: 0.7,
                bidirectional: true,
            }
        }
    };

    let webserver = rocket_server(params.clone())?;

    params.apply_dimming();  // Apply dimming after caching the web version.

    let width: usize = 4;
    let height: usize = 30;
    let back_height: usize = 30;
    let back_width: usize = 16;
    let dots: usize = width * height + back_height * back_width;

    // Remember to enable spi via raspi-config!
    let mut display = runner::get_display(dots)?;

    let mut painters = vec![
        painter::make_painter(back_width, back_height, params.clone()),
        painter::make_painter(width, height, params.clone()),
    ];

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
                    painters[0] = painter::make_painter(back_width, back_height, new_params.clone());
                    painters[1] = painter::make_painter(width, height, new_params.clone());
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
