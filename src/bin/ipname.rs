use std::error::Error;
use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;

use clap::Parser;
extern crate dns_lookup;

#[derive(Parser)]
/// Print the DNS name for the given IP address as stupid as possible.
struct Args {
    #[arg(help = "DNS name")]
    addr: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let addr = IpAddr::from_str(&args.addr)?;
    let name = dns_lookup::lookup_addr(&addr)?;
    println!("{}", name);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ipaddr: {}", e);
        exit(1);
    }
}
