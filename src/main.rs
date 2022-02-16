#[macro_use] mod error;
mod device;

use crate::{ error::Result, device::SerialDevice };
use signal_hook::consts::TERM_SIGNALS;
use std::{
    env, io, process, thread, str::FromStr, time::Duration,
    sync::{
        Arc,
        atomic::{ AtomicBool, Ordering }
    }
};


/// Catches Ctrl+C and terminates gracefully
fn catch_ctrl_c() {
    // Register term signals
    let flag = Arc::new(AtomicBool::default());
    for signal in TERM_SIGNALS {
        // Exit if we cannot register the signal
        if let Err(e) = signal_hook::flag::register(*signal, flag.clone()) {
            eprintln!("Failed to register hook for signal {signal} ({e})");
            process::exit(1);
        }
    }

    // Spinloop
    loop {
        // Check if we got the signal
        if flag.load(Ordering::Relaxed) {
            eprintln!("Reveived signal; exiting...");
            process::exit(1);
        }

        // Sleep some short amount of time
        thread::sleep(Duration::from_millis(77));
    }
}


/// The real main function
fn try_main() -> Result {
    // Get the path from argv[1]
    let path = env::args().nth(1)
        .ok_or(einval!("Missing path to serial device as argument 1"))?;

    // Get the bauds from argv[2]
    let bauds = env::args().nth(2)
        .map(|b| u32::from_str(&b)).transpose()?
        .unwrap_or(115200);

    // Open the serial device
    let device = SerialDevice::new(&path, bauds)?;
    let (mut rx, mut tx) = device.rx_tx();

    // Start the runloop threads
    let stdin = thread::spawn(move || io::copy(&mut io::stdin(), &mut tx));
    let stdout = thread::spawn(move || io::copy(&mut rx, &mut io::stdout()));

    // Wait for stdin and stdout to stop
    stdin.join().expect("stdin-thread panicked?!")?;
    stdout.join().expect("stdout-thread panicked?!")?;
    Ok(())
}


pub fn main() {
    // Start signal handler
    thread::spawn(catch_ctrl_c);

    // Run main
    if let Err(e) = try_main() {
        eprintln!("Fatal error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}
