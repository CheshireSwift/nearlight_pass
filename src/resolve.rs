use crate::types::{Ship, Outcome};

pub fn resolve(a: &Ship, b: &Ship) -> Outcome {
  // a.strongs.cmp(&b.strongs)
  Outcome::Equal
}

#[cfg(test)]
mod tests {
  use crate::types::*;

  use super::{resolve};

  #[test]
  fn flak_beats_fluid_formation() {
    let flakky = Ship {
      name: "Flakky".to_string(),
      phase_plans: PhasePlans {
        contact: Plan {
          weapon: WeaponType::Flak,
          formation: FormationType::None
        },
        approach: Plan {
          weapon: WeaponType::Flak,
          formation: FormationType::None
        },
        passing: Plan {
          weapon: WeaponType::Flak,
          formation: FormationType::None
        },
      }
    };

    let unlucky = Ship {
      name: "Unlucky".to_string(),
      phase_plans: PhasePlans {
        contact: Plan {
          weapon: WeaponType::None,
          formation: FormationType::Fluid
        },
        approach: Plan {
          weapon: WeaponType::None,
          formation: FormationType::Fluid
        },
        passing: Plan {
          weapon: WeaponType::None,
          formation: FormationType::Fluid
        },
      }
    };

    assert_eq!(resolve(&flakky, &unlucky), Outcome::Greater);
  }

  #[test]
  fn fighters_beat_far_formation() {}
  #[test]
  fn beam_beats_close_formation() {}
}
