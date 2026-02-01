#[cfg(windows)]
use clipboard_win::{formats, get_clipboard_string, is_format_avail, set_clipboard_string};

use std::env;
#[cfg(windows)]
use std::io::{self, Read};

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

#[cfg(windows)]
fn copy() -> std::io::Result<()> {
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer)?;
	set_clipboard_string(&buffer).map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;
	Ok(())
}

#[cfg(not(windows))]
fn copy() -> std::io::Result<()> {
	eprintln!("This program only works on Windows.");
	std::process::exit(1);
}

#[cfg(windows)]
fn paste() -> std::io::Result<()> {
	if !is_format_avail(formats::CF_UNICODETEXT) {
		return Ok(());
	}

	let clipboard_text = get_clipboard_string().map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;
	print!("{clipboard_text}");
	Ok(())
}

#[cfg(not(windows))]
fn paste() -> std::io::Result<()> {
	eprintln!("This program only works on Windows.");
	std::process::exit(1);
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("You should specify `--copy` or `--paste` mode. See `--help` for usage examples.");
		return;
	}

	let cmd = &args[1];

	let result = match &cmd[..] {
		"--copy" => copy(),
		"--paste" => paste(),
		_ => {
			help();
			return;
		}
	};

	if let Err(error) = result {
		eprintln!("Error: {error}");
		std::process::exit(1);
	}
}
