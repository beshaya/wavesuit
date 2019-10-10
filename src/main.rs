use std::{error::Error, thread};
use std::time::Duration;

use blinkt::Blinkt;

use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use crossbeam_channel::{bounded, tick, Receiver, select};

mod painter;

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

fn main() -> Result<(), Box<dyn Error>> {
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_millis(30));

    // Remember to enable spi via raspi-config!
    let mut blinkt = Blinkt::with_spi(16_000_000, 144)?;

    let width: usize = 4;
    let height: usize = 30;
    let dots: usize = width * height;
    let fade_colors = vec![
        painter::Color::new(0x4267B2),  // FB blue.
        painter::Color { r: 255, g: 80, b: 0 },
        painter::Color::new(0x898F9C),  // FB grey.
    ];
    let color = painter::Color { r: 255, g: 255, b: 255 };
    let mut arm_painter = painter::make_painter("hex", width, height, color, fade_colors);

    loop {
        select! {
            recv(ticks) -> _ => {
                arm_painter.paint();
                for i in 0..dots {
                    let pixel = arm_painter.get(i);
                    blinkt.set_pixel(i, pixel.r, pixel.g, pixel.b);
                }
                blinkt.show()?;
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
