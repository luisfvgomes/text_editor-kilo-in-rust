use crate::{CTRL_KEY, VERSION, terminal::TerminalConfig};
use core::{error::Error, panic};
use std::io::{ErrorKind, Read, StdinLock, Write};

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

pub fn editor_refresh_screen(terminal_config: &TerminalConfig) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    buf.push_str("\x1b[?25l"); // hide cursor
    buf.push_str("\x1b[H"); //reposition the cursor
    drawn_rows(terminal_config, &mut buf)?;
    buf.push_str("\x1b[H");
    buf.push_str("\x1b[?25h"); // show cursor
    terminal_config
        .terminal_out
        .borrow_mut()
        .write_all(buf.as_bytes())?;
    terminal_config.terminal_out.borrow_mut().flush()?;
    Ok(())
}

fn drawn_rows(terminal_config: &TerminalConfig, buf: &mut String) -> Result<(), Box<dyn Error>> {
    for i in 0..terminal_config.screen_rows {
        buf.push('~');

        //Welcome mensage
        if i == terminal_config.screen_rows / 3 {
            buf.pop();
            let welcome = format!("kilo editor -- version {}", VERSION);
            let mut welcomelen = welcome.len() as u16;
            if welcomelen > terminal_config.screen_cols {
                welcomelen = terminal_config.screen_cols;
            }
            let mut padding = (terminal_config.screen_cols - welcomelen) / 2;
            if padding != 0 {
                buf.push('~');
                padding -= 1;
            }
            for _ in 0..padding {
                buf.push(' ');
            }
            buf.push_str(welcome.as_str());
        }

        buf.push_str("\x1b[K"); // clear line

        if i < terminal_config.screen_rows - 1 {
            buf.push_str("\r\n");
        }
    }

    terminal_config
        .terminal_out
        .borrow_mut()
        .write_all(buf.as_bytes())?;
    Ok(())
}
