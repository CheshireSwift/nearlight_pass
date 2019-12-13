use std::io;
use std::fs;
use std::process;

use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Deserialize)]
struct Ship {
    name: String,
    strongs: i32,
}

fn read_ship(filename: &str) -> io::Result<Ship> {
    return fs::read_to_string(filename).map(|data_string| ron::de::from_str(&data_string).unwrap());
}

fn read_ship_or_bail(filename: &str) -> Ship {
    return read_ship(filename).unwrap_or_else(|err| {
        eprintln!("Unable to parse a.ron: {:?}", err);
        process::exit(1);
    });
}

fn main() {
    let a = read_ship_or_bail("a.ron");
    let b = read_ship_or_bail("b.ron");

    let winner = match a.strongs.cmp(&b.strongs) {
        Ordering::Less => b.name,
        Ordering::Greater => a.name,
        Ordering::Equal => "audience".to_string(),
    };

    println!("{} win", winner);
}
