use crate::modules::elements::{Bolt, Clamped, Nut, Stud, Threaded};
use crate::modules::joint::BoltedJoint;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Library {
    bolt: Vec<Bolt>,
    stud: Vec<Stud>,
    nut: Vec<Nut>,
    threaded: Vec<Threaded>,
    clamped: Vec<Clamped>,
    joint: Vec<BoltedJoint>,
}
