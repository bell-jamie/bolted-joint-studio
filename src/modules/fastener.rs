use crate::modules::{material::Material, thread::Thread};
use std::rc::Rc;

/// Bolt head types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadType {
    Hex,
    HexFlange,
    SocketCap,
    Countersunk,
    Pan,
    Button,
    Other,
}

/// Bolt drive types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveType {
    Hex,
    Allen,
    Torx,
    Slotted,
    Phillips,
    Other,
}

/// Bolt class or grade (mechanical strength)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoltGrade {
    Metric(u8),   // e.g., 8.8, 10.9
    Imperial(u8), // e.g., 2, 5, 8 (ASTM F568)
    Custom,
}

/// Bolt description
#[derive(Debug, Clone)]
pub struct Bolt {
    // Identification
    pub name: String,
    pub thread: Thread,
    pub material: Rc<Material>, // shared pointer for reuse
    pub grade: Option<BoltGrade>,

    // Geometry
    pub length: f64, // length of the bolt (mm or inches depending on thread)
    pub head_type: HeadType,
    pub drive_type: DriveType,
    pub head_height: Option<f64>, // mm or inches
    pub head_diameter: Option<f64>,
    pub shank_diameter: Option<f64>, // diameter of unthreaded portion if any
    pub washer_face_diameter: Option<f64>, // optional for flange bolts

    // Optional metadata
    pub note: Option<String>,
}

impl Bolt {
    /// Create a new bolt with a given thread, material, and length
    pub fn new<S: Into<String>>(
        name: S,
        thread: Thread,
        material: Rc<Material>,
        length: f64,
    ) -> Self {
        Self {
            name: name.into(),
            thread,
            material,
            grade: None,
            length,
            head_type: HeadType::Hex,
            drive_type: DriveType::Hex,
            head_height: None,
            head_diameter: None,
            shank_diameter: None,
            washer_face_diameter: None,
            note: None,
        }
    }

    /// Set the bolt grade
    pub fn set_grade(&mut self, grade: BoltGrade) {
        self.grade = Some(grade);
    }

    /// Set the head geometry
    pub fn set_head_geometry(
        &mut self,
        head_type: HeadType,
        drive_type: DriveType,
        height: f64,
        diameter: f64,
    ) {
        self.head_type = head_type;
        self.drive_type = drive_type;
        self.head_height = Some(height);
        self.head_diameter = Some(diameter);
    }

    /// Set the shank diameter and optional washer face diameter
    pub fn set_shank_geometry(&mut self, shank_diameter: f64, washer_face_diameter: Option<f64>) {
        self.shank_diameter = Some(shank_diameter);
        self.washer_face_diameter = washer_face_diameter;
    }

    /// Set a descriptive note
    pub fn set_note<S: Into<String>>(&mut self, note: S) {
        self.note = Some(note.into());
    }

    /// Approximate bolt cross-sectional area (for strength calculations)
    pub fn tensile_area(&self) -> f64 {
        // Use the minor diameter of the thread for standard tensile area approximation
        let d = self.thread.minor_diameter;
        std::f64::consts::PI * d * d / 4.0
    }

    /// Approximate bolt weight
    pub fn weight(&self) -> Option<f64> {
        self.material.density.map(|rho| {
            let volume =
                std::f64::consts::PI * (self.thread.major_diameter / 2.0).powi(2) * self.length;
            rho * volume
        })
    }
}
