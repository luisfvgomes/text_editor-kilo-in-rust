use crate::CTRL_KEY;
use core::{error::Error, panic};
use std::io::{ErrorKind, Read, StdinLock, StdoutLock, Write};

pub fn editor_process_keypress(terminal_in: &mut StdinLock) -> bool {
    let c = editor_read_key(terminal_in);

    match c {
        _ if c == CTRL_KEY!(b'q') => return true,
        _ => (),
    };
    false
}

fn editor_read_key(terminal_in: &mut StdinLock) -> u8 {
    let mut buffer = [0];
    match terminal_in.read_exact(&mut buffer) {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::UnexpectedEof => buffer[0] = 0,
            _ => panic!("Error reading: {error}"),
        },
    };
    buffer[0]
}

pub fn editor_refresh_screen(terminal_out: &mut StdoutLock) -> Result<(), Box<dyn Error>> {
    terminal_out.write_all(b"\x1b[2J")?; //clean terminal
    terminal_out.write_all(b"\x1b[H")?; //reposition the cursor
    terminal_out.flush()?;
    Ok(())
}
