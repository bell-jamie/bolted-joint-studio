use crate::modules::elements::{Bolt, Clamped, Nut, Stud, Threaded};
use crate::modules::joint::BoltedJoint;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Library {
    pub bolt: Vec<Bolt>,
    pub stud: Vec<Stud>,
    pub nut: Vec<Nut>,
    pub threaded: Vec<Threaded>,
    pub clamped: Vec<Clamped>,
    pub joint: Vec<BoltedJoint>,
}
