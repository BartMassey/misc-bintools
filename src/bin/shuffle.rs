use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Error, Read, Write};
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
extern crate fastrand;

#[derive(Parser)]
/// Shuffle a file. With no argument, acts as a shuffle filter.
struct Args {
    #[arg(long, help = "Write just one random line to stdout")]
    one: bool,
    #[arg(help = "File to shuffle")]
    filename: Option<PathBuf>,
}

fn read_file<F: Read>(input: F) -> Result<String, Error> {
    // XXX Should work on non-UTF8, but won't right now. Ah well.
    let mut input = BufReader::new(input);

    let mut text = String::new();
    input.read_to_string(&mut text)?;
    Ok(text)
}

fn write_shuffle<F: Write>(output: F, text: String) -> Result<(), Error> {
    let mut output = BufWriter::new(output);

    let mut lines: Vec<&str> = text.lines().collect();
    fastrand::shuffle(&mut lines);
    for l in lines {
        writeln!(output, "{}", l)?;
    }
    Ok(())
}

#[test]
fn test_write_shuffle() {
    use std::io::Cursor;

    for _ in 0..10 {
        let text = "a\nb\nc\n";
        let mut dest = Cursor::new(Vec::new());
        write_shuffle(&mut dest, text.to_string()).unwrap();
        let mut dest = dest.into_inner();
        dest.sort();
        assert_eq!(&dest, b"\n\n\nabc");
    }
}

fn print_random_line(text: String) {
    let lines: Vec<&str> = text.lines().collect();
    if let Some(chosen) = fastrand::choice(lines) {
        println!("{}", chosen);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();

    if let Some(filename) = args.filename {
        let f = File::open(&filename)?;
        let text = read_file(f)?;
        if args.one {
            print_random_line(text);
        } else {
            let f = File::create(&filename)?;
            write_shuffle(f, text)?;
        }
    } else {
        let text = read_file(stdin().lock())?;
        if args.one {
            print_random_line(text);
        } else {
            write_shuffle(stdout().lock(), text)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("shuffle: {}", e);
        exit(1);
    }
}
