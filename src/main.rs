use crate::editor::editor_process_keypress;
use crate::terminal::{Terminal, enable_raw_mode};
use core::panic;
use rustix::termios::tcgetattr;
use std::{io::stdin, os::fd::AsFd};
pub mod editor;
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

    let mut is_loop = true;
    while is_loop {
        is_loop = editor_process_keypress();
    }
}
