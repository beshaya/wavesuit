#[cfg_attr(feature = "emulator", path = "emulator.rs")]
#[cfg_attr(not(feature = "emulator"), path = "emulator.rs")]
pub mod run_impl;

pub trait Runnable {
    fn run(&mut self);
}
