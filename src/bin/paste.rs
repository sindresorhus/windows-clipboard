extern crate clipboard_win;
use clipboard_win::get_clipboard_string;
use std::io::{self, Write};

fn paste() -> std::io::Result<()> {
    io::stdout().write(&(get_clipboard_string()?).into_bytes())?;
    Ok(())
}

fn main() {
    paste().expect("could not paste from clipboard");
}