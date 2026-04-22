use crate::{
    editor::{editor_process_keypress, editor_refresh_screen},
    terminal::init_terminal_config,
};
use core::panic;
pub mod editor;
pub mod terminal;
const VERSION: &str = "0.0.1";

fn main() {
    let terminal_config = match init_terminal_config() {
        Ok(tc) => tc,
        Err(e) => panic!("Error init terminal: {e}"),
    };
    loop {
        match editor_refresh_screen(&terminal_config) {
            Ok(_) => (),
            Err(e) => panic!("Error refreshing screen: {e}"),
        };
        if editor_process_keypress(&terminal_config) {
            break;
        }
    }
}
