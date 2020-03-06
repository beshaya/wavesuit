use std::thread;
use std::time::Duration;
use std::error::Error;

use crossbeam_channel::{bounded, tick, Receiver, select};
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

use crate::display;

// Set up signal handlers to listen on their own thread.
fn ctrl_channel() -> Result<Receiver<()>, Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT, SIGTERM])?;

    let (sender, receiver) = bounded(5);
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            let _ = sender.send(());
        }
    });

    Ok(receiver)
}

pub fn get_display(dots: usize) -> Result<Box<dyn display::Display>, Box<dyn Error>> {
    display::new(dots)
}

pub fn run<F>(mut core_alg: F) -> Result<(), Box<dyn Error>>
where F: FnMut() + 'static {
    let ticks = tick(Duration::from_millis(30));
    let ctrl_c_events = ctrl_channel()?;

    loop {
        select! {
            recv(ticks) -> _ => {
                core_alg();
            }
            recv(ctrl_c_events) -> _ => {
                println!("Goodbye");
                break;
            }
        }
    }
    Ok(())
}
