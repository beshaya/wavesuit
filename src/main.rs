use std::{error::Error, thread, mem};
use std::time::Duration;

use blinkt::Blinkt;

use signal_hook::{iterator::Signals, SIGINT};

use crossbeam_channel::{bounded, tick, Receiver, select};

fn ctrl_channel() -> Result<Receiver<()>, Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT])?;

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
    let ticks = tick(Duration::from_millis(250));

    // Remember to enable spi via raspi-config!
    let mut blinkt = Blinkt::with_spi(16_000_000, 144)?;
    let (red, green, blue) = (&mut 64, &mut 0, &mut 0);

    loop {
        select! {
           recv(ticks) -> _ => {
                   blinkt.set_all_pixels(*red, *green, *blue);
                   blinkt.show()?;
                   mem::swap(red, green);
                   mem::swap(red, blue);
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
