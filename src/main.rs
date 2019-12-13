use std::fs;
use std::process;

use std::cmp::Ordering;

use serde::Deserialize;
use ron::de as ron_de;

#[derive(Deserialize)]
struct Ship {
    name: String,
    strongs: i32,
}

fn read_ship(filename: &str) -> Result<Ship, ron_de::Error> {
    let data_string = fs::read_to_string(filename)?;
    return ron_de::from_str(&data_string);
}

fn read_ship_or_bail(filename: &str) -> Ship {
    return read_ship(filename).unwrap_or_else(|err| {
        eprintln!("Unable to parse {}: {:?}", filename, err);
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
