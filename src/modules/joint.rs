#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct BoltedJoint {
    pub name: String,
    pub description: String,
    pub bolt_id: Option<usize>,
    pub stud_id: Option<usize>,
    pub nut_id: Option<usize>,
    pub threaded_id: Option<usize>,
    pub clamped_ids: Option<usize>,
}
