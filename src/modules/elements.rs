use crate::modules::{material::Material, thread::Thread};

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Clamped {
    pub name: String,
    pub id: f32,
    pub od: Option<f32>,
    pub thickness: f32,
    pub material: Material,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Threaded {
    pub name: String,
    pub thread: Thread,
    pub thread_length: f32,
    pub stud_bearing: Option<f32>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Nut {
    pub name: String,
    pub thread: Thread,
    pub bearing_id: f32,
    pub bearing_od: f32,
    pub thickness: f32,
    pub prev_trq: Option<f32>,
    pub mass_on: Option<f32>,
    pub drive: DriveType,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub enum DriveType {
    #[default]
    Hex,
    BiHex,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Stud {
    pub name: String,
    pub thread_a: Thread,
    pub thread_length_a: f32,
    pub thread_b: Thread,
    pub thread_length_b: f32,
    pub shank_diameter: f32,
    pub shank_length: f32,
    pub nipple_id: f32,
    pub nipple_od: f32,
    pub nipple_angle: f32,
    pub waisted: bool,
    pub waist_diameter: Option<f32>,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Bolt {
    pub name: String,
    pub thread: Thread,
    pub thread_length: f32,
    pub head_thickness: f32,
    pub bearing_od: f32,
    pub root_fillet: Option<f32>,
    pub waisted: bool,
    pub waist_diameter: Option<f32>,
    pub grip_length: f32,
}

impl Bolt {
    /// Get the effective shank diameter (waist or minor diameter)
    pub fn get_shank_diameter(&self) -> f64 {
        if self.waisted {
            self.waist_diameter
                .unwrap_or(self.thread.minor_diameter as f32) as f64
        } else {
            self.thread.minor_diameter
        }
    }
}

impl Stud {
    /// Get the effective shank diameter (waist or shank diameter)
    pub fn get_shank_diameter(&self) -> f64 {
        if self.waisted {
            self.waist_diameter.unwrap_or(self.shank_diameter) as f64
        } else {
            self.shank_diameter as f64
        }
    }
}
