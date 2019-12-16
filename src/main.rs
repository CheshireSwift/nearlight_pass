mod io;
mod resolve;
mod types;

use std::cmp::Ordering;
use std::process;
use types::Ship;

fn read_ship_or_bail(filename: &str) -> Ship {
    io::read_ship(filename).unwrap_or_else(|err| {
        eprintln!("Unable to parse {}: {:?}", filename, err);
        process::exit(1);
    })
}

fn main() {
    let a = read_ship_or_bail("a.ron");
    let b = read_ship_or_bail("b.ron");

    let winner = match resolve::resolve(&a, &b) {
        Ordering::Less => b.name,
        Ordering::Greater => a.name,
        Ordering::Equal => "audience".to_string(),
    };

    println!("{} win", winner);
}
