use serde::Deserialize;
use std::cmp::Ordering;

pub type Outcome = Ordering;

#[derive(Deserialize)]
pub struct Ship {
  pub name: String,
  pub phase_plans: PhasePlans,
}

#[derive(Deserialize)]
pub struct PhasePlans {
  pub contact: Plan,
  pub approach: Plan,
  pub passing: Plan,
}

#[derive(Deserialize)]
pub struct Plan {
  pub weapon: WeaponType,
  pub formation: FormationType,
}

#[derive(Deserialize)]
pub enum WeaponType {
  Flak,
  Beam,
  Fighters,
  None,
}

#[derive(Deserialize)]
pub enum FormationType {
  Close,
  Wide,
  Fluid,
  None,
}
