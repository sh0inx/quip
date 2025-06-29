mod gui;
mod keyboard;
mod lock;
mod replace;
mod config;

use std::{process, thread};
use std::time::Duration;
use nix::unistd::Uid;

fn main() {

    check_perms();
    check_lock();
    connect_keyboard();
    
    // connect
    // check if typed value matches regex pattern in config
    // if so, display UI with associated values
    // upon selection, replace character
    // ???
    // profit
}

fn check_perms() {
    // permission check
    // needs updated to reflect modern security practices
    if !Uid::effective().is_root() {
        panic!("Permission denied: must be run as SU.");
        process::exit(1);
    }
}

fn check_lock() { // mutex lock?
    if lock::check_lock() == false { process::exit(1); }
    lock::lock();
}

fn connect_keyboard() {
    // pull these from config and add no-timeout option
    const MAX_ATTEMPTS: u32 = 5;
    const RETRY_DELAY_SECS: u64 = 5;

    for _ in 0..MAX_ATTEMPTS {
        
        if keyboard::keyboard_fetch() { return; }
        
        println!("Keyboard not found. Retrying in {} seconds...", RETRY_DELAY_SECS);
        thread::sleep(Duration::from_secs(RETRY_DELAY_SECS));
    }

    println!("Keyboard not found after {} attempts. Exiting.", MAX_ATTEMPTS);
    std::process::exit(1);
}