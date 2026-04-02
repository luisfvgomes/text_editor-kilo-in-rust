use crate::editor::{editor_process_keypress, editor_refresh_screen};
use crate::terminal::{Terminal, enable_raw_mode};
use core::panic;
use rustix::termios::tcgetattr;
use std::io::stdout;
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

    let mut terminal_in = stdin().lock();
    let mut terminal_out = stdout().lock();

    loop {
        match editor_refresh_screen(&mut terminal_out) {
            Ok(_) => (),
            Err(e) => panic!("Error refreshing screen: {e}"),
        };
        if editor_process_keypress(&mut terminal_in) {
            break;
        }
    }
}
