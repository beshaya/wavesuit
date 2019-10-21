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

    let mut params = PainterParams {
        painter: String::from("line"),
        global_brightness: 0.5,
        speed: 0.5,
        color: Color::new(0xFFFFFF),
        secondary_colors: vec![
            Color::new(0x4267B2),  // FB blue.
            Color::new(0x898F9C),  // FB grey.
        ]};

    let webserver = rocket_server(params.clone())?;

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
        display.show();
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
