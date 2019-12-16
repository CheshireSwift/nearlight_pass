use serde::Deserialize;
use std::cmp::Ordering;

pub type Outcome = Ordering;

#[derive(Deserialize)]
pub struct Ship {
  pub name: String,
  pub strongs: i32,
}
