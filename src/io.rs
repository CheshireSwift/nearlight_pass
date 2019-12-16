use crate::types::Ship;
use ron::de as ron_de;
use std::fs;

pub fn read_ship(filename: &str) -> Result<Ship, ron_de::Error> {
  let data_string = fs::read_to_string(filename)?;
  ron_de::from_str(&data_string)
}
