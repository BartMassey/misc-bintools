use std::io::Error;
use std::net::ToSocketAddrs;
use std::process::exit;

use clap::Parser;

#[derive(Parser)]
/// Print the IP address for the given name as stupid as possible.
struct Args {
    #[arg(help = "DNS name")]
    name: String,
}

fn run() -> Result<(), Error> {
    let args = Args::parse();

    // XXX Rust std::net doesn't believe IP addresses exist
    // without an associated port. Bleah.
    let name = format!("{}:0", args.name);
    let addrs = name.to_socket_addrs()?;
    for a in addrs {
        println!("{}", a.ip());
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ipaddr: {}", e);
        exit(1);
    }
}
