use crate::{CTRL_KEY, terminal::editor_read_key};

pub fn editor_process_keypress() -> bool {
    let c = editor_read_key();

    match c {
        _ if c == CTRL_KEY!(b'q') => return false,
        _ => (),
    }
    true
}
