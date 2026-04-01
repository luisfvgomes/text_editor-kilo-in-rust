use core::panic;
use std::{
    io::{self, Read, stdin},
    os::fd::AsRawFd,
};
use termios::*;
pub mod terminal;
use terminal::{OrigTerminos, enable_raw_mode};

fn main() {
    let orig = match Termios::from_fd(stdin().as_raw_fd()) {
        Ok(termios) => termios,
        Err(e) => panic!("Error: {e}"),
    };
    let orig = OrigTerminos::new(orig);
    match enable_raw_mode(&orig.get_data()) {
        Ok(_) => (),
        Err(e) => panic!("Error: {e}"),
    };

    println!("\r");
    let mut c = [0];
    while c[0] as char != 'q' {
        match io::stdin().read(&mut c) {
            Ok(_) => (),
            Err(e) => panic!("Error: {e}"),
        };
        println!("{}\r", c[0] as char);
    }
}
