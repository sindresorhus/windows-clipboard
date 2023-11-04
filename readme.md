# windows-clipboard

> Access the Windows clipboard (copy/paste)

With full UTF-8 support.

## Install

[Download](https://github.com/sindresorhus/windows-clipboard/releases/latest) the binaries and put them somewhere in your [`%path%`](http://stackoverflow.com/a/28778358/64949).

## Usage

```
$ clipboard --copy < echo unicorn
$ clipboard --paste
unicorn
```

## Build

Install [`Rust`](https://rustup.rs) and run:

```sh
cargo build --release
```

## Related

- [clipboardy](https://github.com/sindresorhus/clipboardy) - Access the system clipboard from Node.js, cross-platform
- [clipboard-cli](https://github.com/sindresorhus/clipboard-cli) - Access the system clipboard from the command-line, cross-platform
