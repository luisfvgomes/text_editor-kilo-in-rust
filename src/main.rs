use crate::terminal::{Terminal, enable_raw_mode, iscntrl};
use core::panic;
use rustix::termios::tcgetattr;
use std::{
    io::{self, ErrorKind, Read, stdin},
    os::fd::AsFd,
};
pub mod terminal;

fn main() {
    let terminal = match tcgetattr(stdin().as_fd()) {
        Ok(terminal) => terminal,
        Err(e) => panic!("Error in get terminal: {e}"),
    };
    let terminal = Terminal::new(&terminal);

    match enable_raw_mode(&mut terminal.mod_terminal.borrow_mut()) {
        Ok(_) => (),
        Err(e) => panic!("Error enabling raw mode: {e}"),
    };

    println!("\r");
    let mut c = [0];
    while c[0] as char != 'q' {
        match io::stdin().read_exact(&mut c) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::UnexpectedEof => c[0] = 0,
                _ => panic!("Error reading: {error}"),
            },
        };
        if iscntrl(c[0]) {
            println!("{}\r", c[0]);
        } else {
            println!("{} ({})\r", c[0], c[0] as char);
        }
    }
}
