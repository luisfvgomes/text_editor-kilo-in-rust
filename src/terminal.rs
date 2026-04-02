use core::panic;
use rustix::io::Result;
use rustix::termios::tcgetattr;
use rustix::termios::{self, SpecialCodeIndex, tcgetwinsize};
use std::cell::RefCell;
use std::io::{StdinLock, StdoutLock, stdout};
use std::{io::stdin, os::fd::AsFd};

use crate::editor::editor_refresh_screen;

pub struct TerminalConfig<'a> {
    pub terminal_in: RefCell<StdinLock<'a>>,
    pub terminal_out: RefCell<StdoutLock<'a>>,
    pub screen_rows: u16,
    pub screen_cols: u16,
    pub mod_terminal: RefCell<termios::Termios>,
    pub orig_terminal: termios::Termios,
}
impl<'a> TerminalConfig<'a> {
    pub fn new(
        orig_terminal: termios::Termios,
        screen_rows: u16,
        screen_cols: u16,
        terminal_in: StdinLock<'a>,
        terminal_out: StdoutLock<'a>,
    ) -> Self {
        let mod_term = RefCell::new(orig_terminal.clone());
        Self {
            mod_terminal: mod_term,
            orig_terminal,
            screen_rows,
            screen_cols,
            terminal_in: RefCell::from(terminal_in),
            terminal_out: RefCell::from(terminal_out),
        }
    }
}
impl<'a> Drop for TerminalConfig<'a> {
    fn drop(&mut self) {
        match disable_raw_mode(&self.orig_terminal) {
            Ok(_) => (),
            Err(e) => panic!("Error disabling raw mode: {e}"),
        }
        match editor_refresh_screen(self) {
            Ok(_) => (),
            Err(e) => panic!("Error refreshing screen: {e}"),
        }
    }
}

pub fn enable_raw_mode(terminal: &mut termios::Termios) -> Result<()> {
    terminal.make_raw();
    terminal.special_codes[SpecialCodeIndex::VMIN] = 0;
    terminal.special_codes[SpecialCodeIndex::VTIME] = 1;
    termios::tcsetattr(stdin().as_fd(), termios::OptionalActions::Flush, terminal)?;
    Ok(())
}

fn disable_raw_mode(terminal: &termios::Termios) -> Result<()> {
    termios::tcsetattr(stdin().as_fd(), termios::OptionalActions::Flush, terminal)?;
    Ok(())
}

pub fn iscntrl(c: u8) -> bool {
    if c < 0x20 || c == 0x7F {
        return true;
    }
    false
}

#[macro_export]
macro_rules! CTRL_KEY {
    ($k:expr) => {
        $k & 0x1f
    };
}

pub fn get_window_size(terminal_out: &StdoutLock) -> Result<(u16, u16)> {
    let ws = tcgetwinsize(terminal_out)?;
    Ok((ws.ws_row, ws.ws_col))
}

pub fn init_terminal_config() -> Result<TerminalConfig<'static>> {
    let terminal = match tcgetattr(stdin().as_fd()) {
        Ok(terminal) => terminal,
        Err(e) => panic!("Error in get terminal: {e}"),
    };
    let terminal_out = stdout().lock();
    let terminal_in = stdin().lock();
    let ws = get_window_size(&terminal_out)?;

    let terminal_config = TerminalConfig::new(terminal, ws.0, ws.1, terminal_in, terminal_out);

    match enable_raw_mode(&mut terminal_config.mod_terminal.borrow_mut()) {
        Ok(_) => (),
        Err(e) => panic!("Error enabling raw mode: {e}"),
    };
    Ok(terminal_config)
}
