use core::{error::Error, panic};
use rustix::termios::{self, SpecialCodeIndex};
use std::cell::RefCell;
use std::{io::stdin, os::fd::AsFd};

pub struct Terminal<'a> {
    pub mod_terminal: RefCell<termios::Termios>,
    pub orig_terminal: &'a termios::Termios,
}
impl<'a> Terminal<'a> {
    pub fn new(terminal: &'a termios::Termios) -> Self {
        let mod_term = RefCell::new(terminal.clone());
        Self {
            mod_terminal: mod_term,
            orig_terminal: terminal,
        }
    }
}
impl<'a> Drop for Terminal<'a> {
    fn drop(&mut self) {
        match disable_raw_mode(self.orig_terminal) {
            Ok(_) => (),
            Err(e) => panic!("Error disabling raw mode: {e}"),
        }
    }
}

pub fn enable_raw_mode(terminal: &mut termios::Termios) -> Result<(), Box<dyn Error>> {
    terminal.make_raw();
    terminal.special_codes[SpecialCodeIndex::VMIN] = 0;
    terminal.special_codes[SpecialCodeIndex::VTIME] = 1;
    termios::tcsetattr(stdin().as_fd(), termios::OptionalActions::Flush, terminal)?;
    Ok(())
}

pub fn disable_raw_mode(terminal: &termios::Termios) -> Result<(), Box<dyn Error>> {
    termios::tcsetattr(stdin().as_fd(), termios::OptionalActions::Flush, terminal)?;
    Ok(())
}

pub fn iscntrl(c: u8) -> bool {
    if c < 0x20 || c == 0x7F {
        return true;
    }
    false
}
