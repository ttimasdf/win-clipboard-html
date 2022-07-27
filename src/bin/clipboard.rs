extern crate clap;
extern crate clipboard_win;
extern crate encoding;
extern crate regex;
use clap::{App, Arg, ArgGroup};
use clipboard_win::get_clipboard_string;
use clipboard_win::raw::register_format;
use clipboard_win::set_clipboard_string;
use clipboard_win::Clipboard;

use std::fs;
use std::io::BufRead;
use std::io::{self, Read, Write};

struct Input<'a> {
    source: Box<dyn BufRead + 'a>,
}

impl<'a> Input<'a> {
    fn console(stdin: &'a io::Stdin) -> Input<'a> {
        Input {
            source: Box::new(stdin.lock()),
        }
    }

    fn file(path: &str) -> io::Result<Input<'a>> {
        fs::File::open(path).map(|file| Input {
            source: Box::new(io::BufReader::new(file)),
        })
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
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

fn copy_html(mut stream: Input) -> std::io::Result<()> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    let clip = Clipboard::new()?;
    let format_id = register_format("text/html")?;
    clip.set(format_id, buffer.as_bytes())?;

    let html_formatted = format!(
        "Version:0.9\r\n\
        StartHTML:{:08}\r\n\
        EndHTML:{:08}\r\n\
        StartFragment:{:08}\r\n\
        EndFragment:{:08}\r\n\
        <html><body>\r\n\
        <!--StartFragment-->{}<!--EndFragment-->\r\n\
        </body></html>",
        97,
        131 + buffer.len() + 34,
        131,
        131 + buffer.len(),
        buffer
    );

    let format_id_html = register_format("HTML Format")?;
    clip.set(format_id_html, html_formatted.as_bytes())?;

    println!("{} bytes copied to clipboard!, HTML Format binary size: {}", buffer.len(), html_formatted.len());
    Ok(())
}

fn paste_html<W: Write>(mut stream: W) -> std::io::Result<()> {
    let mut buffer = [0u8; 500];
    let clip = Clipboard::new()?;
    let format_id = register_format("HTML Format")?;
    clip.get(format_id, &mut buffer)?;

    stream.write(&buffer)?;
    Ok(())
}

fn main() {
    let args = App::new("clipboard")
        .version("1.0")
        .author("ttimasdf")
        .about("Access the Windows clipboard (copy/paste)")
        .arg(
            Arg::with_name("copy")
                .short("c")
                .long("copy")
                .takes_value(false)
                .help("stores stdin into clipboard"),
        )
        .arg(
            Arg::with_name("paste")
                .short("p")
                .long("paste")
                .takes_value(false)
                .help("pastes clipboard content to stdout"),
        )
        .group(
            ArgGroup::with_name("mode")
                .args(&["copy", "paste"])
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("input file to put into clipboard"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output file to store data from clipboard"),
        )
        .arg(
            Arg::with_name("html")
                .short("H")
                .long("html")
                .takes_value(false)
                .help("manipulate clipboard content as HTML instead of plain text"),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .takes_value(false)
                .help("delete file after copied, useful for automatic pipeline"),
        )
        .get_matches();

    let input_filename = args.value_of("input").unwrap_or("-");
    let stdio = io::stdin();
    let input: Input = match input_filename {
        "-" => Input::console(&stdio),
        _ => Input::file(input_filename).unwrap(),
    };

    match (args.is_present("copy"), args.is_present("html")) {
        // copy text
        (true, false) => copy().expect("Error: Could not copy to clipboard"),
        // paste text
        (false, false) => paste().expect("Error: Could not paste from clipboard"),
        // copy html
        (true, true) => copy_html(input).expect("Error: Could not copy to clipboard"),
        // paste html
        (false, true) => paste_html(io::stdout()).expect("Error: Could not paste from clipboard"),
    }
    if args.is_present("delete") {
        println!("Deleting file after copied");
        fs::remove_file(input_filename).expect("File cannot be removed");
    }
}
