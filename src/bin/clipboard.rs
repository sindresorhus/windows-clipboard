extern crate clipboard_win;
use clipboard_win::get_clipboard_string;
use clipboard_win::set_clipboard_string;

use std::env;
use std::io::{self, Read, Write};

fn help() {
    println!("clipboard.exe – Access the Windows clipboard (copy/paste)");
    println!("");
    println!("Usage:");
    println!("  clipboard.exe --copy < echo Hello");
    println!("  clipboard.exe --paste");
    println!("");
    println!("    --copy  - stores stdin into clipboard");
    println!("    --paste - pastes clipboard content to stdout");
    println!("");
    println!("MIT © Sindre Sorhus");
}

fn copy() -> std::io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    set_clipboard_string(&buffer)?;
    Ok(())
}

fn paste() -> std::io::Result<()> {
    io::stdout().write(&(get_clipboard_string()?).into_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You should specify `--copy` or `--paste` mode. See `--help` for usage examples.");
        return;
    }

    let cmd = &args[1];

    match &cmd[..] {
        "--copy" => copy().expect("Error: Could not copy to clipboard"),
        "--paste" => paste().expect("Error: Could not paste from clipboard"),
        _ => help(),
    }
}