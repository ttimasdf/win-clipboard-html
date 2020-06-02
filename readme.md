# win-clipboard

> Access the Windows clipboard (copy/paste) from command line!

With full UTF-8 support. This is a forked version, with revised command line parsing mechanism, and support clipping HTML to clipboard according to [MSDN HTML Clipboard Format](https://docs.microsoft.com/en-us/windows/win32/dataxchg/html-clipboard-format). That is, you can copy and paste **rich text content** in any supported app! e.g. WYSIWYG rich text H5 editors in common browsers.


Command line options: 

```
clipboard 1.0
ttimasdf
Access the Windows clipboard (copy/paste)

USAGE:
    clipboard.exe [FLAGS] [OPTIONS] <--copy|--paste>

FLAGS:
    -c, --copy       stores stdin into clipboard
    -d, --delete     delete file after copied, useful for automatic pipeline
    -h, --help       Prints help information
    -H, --html       manipulate clipboard content as HTML instead of plain text
    -p, --paste      pastes clipboard content to stdout
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      input file to put into clipboard
    -o, --output <output>    output file to store data from clipboard
```

## Install

[Download](https://github.com/sindresorhus/win-clipboard/releases/latest) the binaries and put them somewhere in your [`%path%`](http://stackoverflow.com/a/28778358/64949).


## Usage

```
$ clipboard --copy < echo unicorn
$ clipboard --paste
unicorn
```


## Build

Install [`Rust`](https://rustup.rs) and run:

```
$ cargo build --release
```


## Related

- [clipboardy](https://github.com/sindresorhus/clipboardy) - Access the system clipboard from Node.js, cross-platform
- [clipboard-cli](https://github.com/sindresorhus/clipboard-cli) - Access the system clipboard from the command-line, cross-platform


## Maintainers

- [Sindre Sorhus](https://github.com/sindresorhus)
- [Vsevolod Strukchinsky](https://github.com/floatdrop)


## License

MIT
