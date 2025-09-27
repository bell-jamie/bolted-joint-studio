use crate::modules::{material::Material, thread::Thread};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Clamped {
    id: f32,
    od: Option<f32>,
    thickness: f32,
    material: Material,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Threaded {
    thread: Thread,
    thread_length: f32,
    stud_bearing: Option<f32>,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Nut {
    thread: Thread,
    bearing_id: f32,
    bearing_od: f32,
    thickness: f32,
    prev_trq: Option<f32>,
    mass_on: Option<f32>,
    drive: DriveType,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
enum DriveType {
    #[default]
    Hex,
    BiHex,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Stud {
    thread_a: Thread,
    thread_length_a: f32,
    thread_b: Thread,
    thread_length_b: f32,
    shank_diameter: f32,
    shank_length: f32,
    nipple_id: f32,
    nipple_od: f32,
    nipple_angle: f32,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Bolt {
    thread: Thread,
    thread_length: f32,
    head_thickness: f32,
    bearing_od: f32,
    root_fillet: Option<f32>,
}
