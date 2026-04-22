use crate::{CTRL_KEY, VERSION, terminal::TerminalConfig};
use core::error::Error;
use std::io::{ErrorKind, Read, StdinLock, Write};

#[derive(PartialEq)]
enum ArrowKeys {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
enum KeyType {
    Letter(u8),
    Arrow(ArrowKeys),
}

const MOVE_KEYS: [KeyType; 4] = [
    KeyType::Arrow(ArrowKeys::Left),
    KeyType::Arrow(ArrowKeys::Right),
    KeyType::Arrow(ArrowKeys::Up),
    KeyType::Arrow(ArrowKeys::Down),
];

pub fn editor_process_keypress(terminal_config: &TerminalConfig) -> bool {
    let c = match editor_read_key(&mut terminal_config.terminal_in.borrow_mut()) {
        Ok(c) => c,
        Err(e) => panic!("Error reading key: {e}"),
    };

    match c {
        _ if c == KeyType::Letter(CTRL_KEY!(b'q')) => return true,
        _ if MOVE_KEYS.contains(&c) => move_cursor(terminal_config, c),
        _ => (),
    };
    false
}

fn editor_read_key(terminal_in: &mut StdinLock) -> Result<KeyType, Box<dyn Error>> {
    let mut buffer = [0];
    match terminal_in.read_exact(&mut buffer) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::UnexpectedEof => buffer[0] = 0,
            _ => return Err(Box::from(e)),
        },
    };
    if buffer[0] == b'\x1b' {
        let mut seq = [0; 3];
        match terminal_in.read_exact(&mut seq) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => (),
                _ => return Err(Box::from(e)),
            },
        };
        if seq[0] == b'[' {
            match seq[1] {
                b'A' => return Ok(KeyType::Arrow(ArrowKeys::Up)),
                b'B' => return Ok(KeyType::Arrow(ArrowKeys::Down)),
                b'C' => return Ok(KeyType::Arrow(ArrowKeys::Right)),
                b'D' => return Ok(KeyType::Arrow(ArrowKeys::Left)),
                _ => (),
            }
        }
    }
    Ok(KeyType::Letter(buffer[0]))
}

pub fn editor_refresh_screen(terminal_config: &TerminalConfig) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    buf.push_str("\x1b[?25l"); // hide cursor
    buf.push_str("\x1b[H"); //reposition cursor
    drawn_rows(terminal_config, &mut buf)?;
    buf.push_str(
        format!(
            "\x1b[{};{}H",
            *terminal_config.cursor_y.borrow() + 1,
            *terminal_config.cursor_x.borrow() + 1,
        )
        .as_str(),
    ); // reposition cursor
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

fn move_cursor(terminal_config: &TerminalConfig, key: KeyType) {
    let cx = *terminal_config.cursor_x.borrow();
    let cy = *terminal_config.cursor_y.borrow();
    match key {
        KeyType::Arrow(ArrowKeys::Left) => {
            if cx > 0 {
                *terminal_config.cursor_x.borrow_mut() -= 1;
            }
        }
        KeyType::Arrow(ArrowKeys::Right) => {
            if cx < terminal_config.screen_cols - 1 {
                *terminal_config.cursor_x.borrow_mut() += 1;
            }
        }
        KeyType::Arrow(ArrowKeys::Up) => {
            if cy > 0 {
                *terminal_config.cursor_y.borrow_mut() -= 1;
            }
        }
        KeyType::Arrow(ArrowKeys::Down) => {
            if cy < terminal_config.screen_rows - 1 {
                *terminal_config.cursor_y.borrow_mut() += 1;
            }
        }
        _ => panic!("Not a moving key"),
    }
}
