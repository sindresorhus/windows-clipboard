extern crate clipboard_win;
use clipboard_win::set_clipboard_string;

use std::io::{self, Read};

fn copy() -> std::io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    set_clipboard_string(&buffer)?;
    Ok(())
}

fn main() {
    copy().expect("could not copy to clipboard");
}