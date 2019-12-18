use crate::types::{Outcome, Ship};

pub fn resolve(a: &Ship, b: &Ship) -> Outcome {
  a.strongs.cmp(&b.strongs)
}

#[cfg(test)]
mod tests {
  use super::{resolve, Outcome, Ship};

  #[test]
  fn stronger_ship_wins() {
    let muscle_and_gainz = Ship {
      name: "Muscle and Gainz".to_string(),
      strongs: 284,
    };
    let tiny_baby = Ship {
      name: "Tiny Baby".to_string(),
      strongs: 2,
    };
    assert_eq!(resolve(&muscle_and_gainz, &tiny_baby), Outcome::Greater);
  }
}
