use rustix::termios::{self, SpecialCodeIndex, tcgetwinsize};
use rustix::termios::{Winsize, tcgetattr};
use std::cell::RefCell;
use std::error::Error;
use std::io::{BufRead, StdinLock, StdoutLock, Write, stdout};
use std::{io::stdin, os::fd::AsFd};

use crate::editor::editor_refresh_screen;

pub struct TerminalConfig<'a> {
    pub terminal_in: RefCell<StdinLock<'a>>,
    pub terminal_out: RefCell<StdoutLock<'a>>,
    pub screen_rows: u16,
    pub screen_cols: u16,
    pub mod_terminal: RefCell<termios::Termios>,
    pub orig_terminal: termios::Termios,
    pub cursor_x: RefCell<u16>,
    pub cursor_y: RefCell<u16>,
}
impl<'a> TerminalConfig<'a> {
    pub fn new(
        orig_terminal: termios::Termios,
        mod_terminal: termios::Termios,
        screen_rows: u16,
        screen_cols: u16,
        terminal_in: StdinLock<'a>,
        terminal_out: StdoutLock<'a>,
    ) -> Self {
        let mod_terminal = RefCell::new(mod_terminal);
        Self {
            mod_terminal,
            orig_terminal,
            screen_rows,
            screen_cols,
            terminal_in: RefCell::from(terminal_in),
            terminal_out: RefCell::from(terminal_out),
            cursor_x: RefCell::from(0),
            cursor_y: RefCell::from(0),
        }
    }
}
impl<'a> Drop for TerminalConfig<'a> {
    fn drop(&mut self) {
        match disable_raw_mode(&self.orig_terminal) {
            Ok(_) => (),
            Err(e) => panic!("Error disabling raw mode: {e}"),
        }
        *self.cursor_x.borrow_mut() = 0;
        *self.cursor_y.borrow_mut() = 0;
        match editor_refresh_screen(self) {
            Ok(_) => (),
            Err(e) => panic!("Error refreshing screen: {e}"),
        }
    }
}

pub fn enable_raw_mode(mut terminal: termios::Termios) -> Result<termios::Termios, Box<dyn Error>> {
    terminal.make_raw();
    terminal.special_codes[SpecialCodeIndex::VMIN] = 0;
    terminal.special_codes[SpecialCodeIndex::VTIME] = 1;
    termios::tcsetattr(stdin().as_fd(), termios::OptionalActions::Flush, &terminal)?;
    Ok(terminal)
}

fn disable_raw_mode(terminal: &termios::Termios) -> Result<(), Box<dyn Error>> {
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

pub fn get_window_size(
    terminal_out: &mut StdoutLock,
    terminal_in: &mut StdinLock,
) -> Result<(u16, u16), Box<dyn Error>> {
    let ws = match tcgetwinsize(&terminal_out) {
        Ok(ws) => ws,
        Err(_) => {
            terminal_out.write_all(b"\x1b[999C\x1b[999B")?; // go to the last place on scren
            get_cursor_position(terminal_out, terminal_in)?
        }
    };
    Ok((ws.ws_row, ws.ws_col))
}

pub fn init_terminal_config() -> Result<TerminalConfig<'static>, Box<dyn Error>> {
    let orig_terminal = tcgetattr(stdin().as_fd())?;
    let mod_terminal = enable_raw_mode(orig_terminal.clone())?;
    let mut terminal_out = stdout().lock();
    let mut terminal_in = stdin().lock();
    let ws = get_window_size(&mut terminal_out, &mut terminal_in)?;

    let terminal_config = TerminalConfig::new(
        orig_terminal,
        mod_terminal,
        ws.0,
        ws.1,
        terminal_in,
        terminal_out,
    );

    Ok(terminal_config)
}

fn get_cursor_position(
    terminal_out: &mut StdoutLock,
    terminal_in: &mut StdinLock,
) -> Result<Winsize, Box<dyn Error>> {
    terminal_out.write_all(b"\x1b[6n")?;
    terminal_out.flush()?;
    let mut buffer = vec![];
    terminal_in.read_until(b'R', &mut buffer)?;
    buffer.pop();
    if !buffer.starts_with(b"\x1b[") {
        return Err("Not expected bytes".into());
    }
    let mut iterator = buffer[2..].split(|byte| byte == &b';');
    let rows = String::from_utf8(Vec::from(iterator.next().unwrap()))?.parse::<u16>()?;
    let cols = String::from_utf8(Vec::from(iterator.next().unwrap()))?.parse::<u16>()?;
    Ok(Winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 1,
        ws_ypixel: 1,
    })
}
