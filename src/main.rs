use std::{error::Error, thread};
use std::time::Duration;

use blinkt::Blinkt;

use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use crossbeam_channel::{bounded, tick, Receiver, select};

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

// Assumes vertical indexing, going down on first path.
fn get_index(height:usize, x:usize, y:usize) -> usize {
    let offset = x * height;
    if x % 2  == 0 {
        return offset + y;
    }
    return offset + height - y - 1;
}

fn vertical_pulse(width:usize, height:usize, tick:usize, r:&mut[u8], g:&mut[u8], b:&mut[u8]) {
    let speed = 0.8;
    let growth = 1.4;
    let center: f32 = (tick as f32 * speed) % (height as f32 * growth);
    for y in 0..height {
        let val: f32;
        let y_float = y as f32;
        if y_float >= center {
            val = (1.0 - (y_float - center) * 0.5).powf(3.);
        } else {
            val = (1.0 - (center - y_float) * 0.1).powf(3.);
        }
        for x in 0..width {
            let index = get_index(height, x, y);
            r[index] = (val * 255.) as u8;
            g[index] = (val * 255.) as u8;
            b[index] = (val * 255.) as u8;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_millis(30));
    let mut tick = 0;

    // Remember to enable spi via raspi-config!
    let mut blinkt = Blinkt::with_spi(16_000_000, 144)?;

    let width: usize = 4;
    let height: usize = 30;
    let dots: usize = width * height;
    let mut red = Vec::with_capacity(dots);
    let mut green = Vec::with_capacity(dots);
    let mut blue = Vec::with_capacity(dots);
    red.resize(dots, 0);
    green.resize(dots, 0);
    blue.resize(dots, 0);

    loop {
        select! {
            recv(ticks) -> _ => {
                vertical_pulse(width, height, tick, &mut red, &mut green, &mut blue);
                for i in 0..dots {
                    blinkt.set_pixel(i, red[i], green[i], blue[i]);
                }
                blinkt.show()?;
                tick += 1;
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
