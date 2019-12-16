use crate::types::{Outcome, Ship};

pub fn resolve(a: &Ship, b: &Ship) -> Outcome {
  a.strongs.cmp(&b.strongs)
}
