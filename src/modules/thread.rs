/// Unit system for threads
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Unit {
    #[default]
    Metric, // millimeters
    Imperial, // inches
}

/// Thread form (standard profiles)
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThreadForm {
    #[default]
    ISO, // Metric ISO 68-1
    UNC,         // Unified Coarse
    UNF,         // Unified Fine
    Acme,        // Acme trapezoidal
    Trapezoidal, // ISO trapezoidal
    Custom,
}

/// Direction of the thread
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThreadHand {
    #[default]
    Right,
    Left,
}

/// Complete thread description
/// #[derive(serde::Deserialize, serde::Serialize)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct Thread {
    pub unit: Unit,                      // Metric or Imperial
    pub form: ThreadForm,                // ISO, UNC, UNF, etc.
    pub major_diameter: f64,             // outer diameter (mm or inches)
    pub minor_diameter: f64,             // root diameter (mm or inches)
    pub pitch: f64,                      // distance between threads (mm or inches)
    pub threads_per_unit: Option<f64>,   // optional: for imperial threads (TPI)
    pub length: Option<f64>,             // optional: thread length
    pub hand: ThreadHand,                // right or left-hand
    pub angle: f64, // thread angle in degrees (e.g., 60 for metric, 55 for UNC)
    pub tolerance_class: Option<String>, // e.g., "6g" for metric, "2A" for UNC
    pub note: Option<String>, // optional description
}

impl Thread {
    /// Create a new metric thread
    pub fn new_metric(
        major_diameter: f64,
        pitch: f64,
        length: Option<f64>,
        hand: ThreadHand,
        tolerance_class: Option<String>,
    ) -> Self {
        let minor_diameter = major_diameter - 1.226869 * pitch; // approximate ISO 68-1 formula
        Self {
            unit: Unit::Metric,
            form: ThreadForm::ISO,
            major_diameter,
            minor_diameter,
            pitch,
            threads_per_unit: None,
            length,
            hand,
            angle: 60.0,
            tolerance_class,
            note: None,
        }
    }

    /// Create a new imperial UNC/UNF thread
    pub fn new_imperial(
        major_diameter: f64,
        threads_per_inch: f64,
        length: Option<f64>,
        hand: ThreadHand,
        form: ThreadForm,
        tolerance_class: Option<String>,
    ) -> Self {
        let pitch = 1.0 / threads_per_inch;
        let minor_diameter = major_diameter - 0.6495 * pitch; // approximate root for 60° thread
        Self {
            unit: Unit::Imperial,
            form,
            major_diameter,
            minor_diameter,
            pitch,
            threads_per_unit: Some(threads_per_inch),
            length,
            hand,
            angle: 60.0,
            tolerance_class,
            note: None,
        }
    }

    /// Returns the pitch in the correct unit
    pub fn get_pitch(&self) -> f64 {
        self.pitch
    }

    /// Returns the thread depth (approximate)
    pub fn depth(&self) -> f64 {
        (self.major_diameter - self.minor_diameter) / 2.0
    }

    /// Set a note for the thread
    pub fn set_note<S: Into<String>>(&mut self, note: S) {
        self.note = Some(note.into());
    }
}
