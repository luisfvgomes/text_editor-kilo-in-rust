use core::{error::Error, panic};
use std::{io::stdin, os::fd::AsRawFd};
use termios::*;

pub struct OrigTerminos {
    data: Termios,
}
impl OrigTerminos {
    pub fn new(orig: Termios) -> Self {
        Self { data: orig }
    }
    pub fn get_data(self) -> Termios {
        self.data
    }
}
impl Drop for OrigTerminos {
    fn drop(&mut self) {
        match disable_raw_mode(&self.data) {
            Ok(_) => (),
            Err(e) => panic!("Error: {e}"),
        };
    }
}

pub fn enable_raw_mode(orig: &Termios) -> Result<(), Box<dyn Error>> {
    let raw = &mut orig.clone();
    raw.c_iflag &= !(IXON | ICRNL);
    raw.c_oflag &= !(OPOST);
    raw.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, raw)?;
    Ok(())
}

fn disable_raw_mode(orig: &Termios) -> Result<(), Box<dyn Error>> {
    tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, orig)?;
    Ok(())
}
