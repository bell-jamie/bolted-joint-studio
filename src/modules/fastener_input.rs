use crate::modules::{
    elements::{Bolt, Stud},
    thread::{Thread, ThreadForm, ThreadHand, Unit},
};
use egui::Ui;
use egui_flex::{Flex, FlexAlignContent, item};

/// State for the fastener input UI
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct FastenerInputState {
    pub name: String,
    pub fastener_type: FastenerType,
    pub thread_unit: Unit,
    pub thread_form: ThreadForm,
    pub nominal_size: String,
    pub pitch: f64,
    pub thread_length: f32,
    pub waisted: bool,
    pub waist_diameter: f32,
    pub grip_length: f32,

    // Stud-specific
    pub thread_length_b: f32,
    pub shank_diameter: f32,
    pub shank_length: f32,

    // Bolt-specific
    pub head_thickness: f32,
    pub bearing_od: f32,

    // Advanced options visibility
    pub show_advanced: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq)]
pub enum FastenerType {
    Bolt,
    Stud,
}

impl Default for FastenerInputState {
    fn default() -> Self {
        Self {
            name: String::new(),
            fastener_type: FastenerType::Bolt,
            thread_unit: Unit::Metric,
            thread_form: ThreadForm::ISO,
            nominal_size: "M8".to_string(),
            pitch: 1.25,
            thread_length: 20.0,
            waisted: false,
            waist_diameter: 6.647, // M8 minor diameter
            grip_length: 30.0,
            thread_length_b: 20.0,
            shank_diameter: 8.0,
            shank_length: 40.0,
            head_thickness: 5.3,
            bearing_od: 13.0,
            show_advanced: false,
        }
    }
}

impl FastenerInputState {
    /// Get metric thread sizes
    pub fn metric_sizes() -> Vec<&'static str> {
        vec![
            "M1", "M1.2", "M1.6", "M2", "M2.5", "M3", "M4", "M5", "M6", "M8", "M10", "M12", "M14",
            "M16", "M18", "M20", "M22", "M24", "M27", "M30", "M33", "M36", "M39", "M42",
        ]
    }

    /// Get imperial thread sizes
    pub fn imperial_sizes() -> Vec<&'static str> {
        vec![
            "#0", "#1", "#2", "#4", "#6", "#8", "#10", "#12", "1/4", "5/16", "3/8", "7/16", "1/2",
            "9/16", "5/8", "3/4", "7/8", "1", "1-1/8", "1-1/4", "1-3/8", "1-1/2",
        ]
    }

    /// Get metric pitches for a given size
    pub fn metric_pitches(size: &str) -> Vec<f64> {
        match size {
            "M1" => vec![0.25],
            "M1.2" => vec![0.25],
            "M1.6" => vec![0.35],
            "M2" => vec![0.4],
            "M2.5" => vec![0.45],
            "M3" => vec![0.5],
            "M4" => vec![0.7],
            "M5" => vec![0.8],
            "M6" => vec![1.0],
            "M8" => vec![1.25, 1.0],
            "M10" => vec![1.5, 1.25, 1.0],
            "M12" => vec![1.75, 1.5, 1.25],
            "M14" => vec![2.0, 1.5],
            "M16" => vec![2.0, 1.5],
            "M18" => vec![2.5, 2.0, 1.5],
            "M20" => vec![2.5, 2.0, 1.5],
            "M22" => vec![2.5, 2.0, 1.5],
            "M24" => vec![3.0, 2.0],
            "M27" => vec![3.0, 2.0],
            "M30" => vec![3.5, 2.0],
            "M33" => vec![3.5, 2.0],
            "M36" => vec![4.0, 3.0],
            "M39" => vec![4.0, 3.0],
            "M42" => vec![4.5, 3.0],
            _ => vec![1.0],
        }
    }

    /// Get imperial threads per inch for a given size and form
    pub fn imperial_tpi(size: &str, form: ThreadForm) -> Vec<f64> {
        match (size, form) {
            ("#0", ThreadForm::UNC) => vec![80.0],
            ("#1", ThreadForm::UNC) => vec![64.0],
            ("#2", ThreadForm::UNC) => vec![56.0],
            ("#4", ThreadForm::UNC) => vec![40.0],
            ("#6", ThreadForm::UNC) => vec![32.0],
            ("#8", ThreadForm::UNC) => vec![32.0],
            ("#10", ThreadForm::UNC) => vec![24.0],
            ("#12", ThreadForm::UNC) => vec![24.0],
            ("1/4", ThreadForm::UNC) => vec![20.0],
            ("5/16", ThreadForm::UNC) => vec![18.0],
            ("3/8", ThreadForm::UNC) => vec![16.0],
            ("1/2", ThreadForm::UNC) => vec![13.0],
            ("5/8", ThreadForm::UNC) => vec![11.0],
            ("3/4", ThreadForm::UNC) => vec![10.0],
            ("1", ThreadForm::UNC) => vec![8.0],

            ("#0", ThreadForm::UNF) => vec![80.0],
            ("#1", ThreadForm::UNF) => vec![72.0],
            ("#2", ThreadForm::UNF) => vec![64.0],
            ("#4", ThreadForm::UNF) => vec![48.0],
            ("#6", ThreadForm::UNF) => vec![40.0],
            ("#8", ThreadForm::UNF) => vec![36.0],
            ("#10", ThreadForm::UNF) => vec![32.0],
            ("#12", ThreadForm::UNF) => vec![28.0],
            ("1/4", ThreadForm::UNF) => vec![28.0],
            ("5/16", ThreadForm::UNF) => vec![24.0],
            ("3/8", ThreadForm::UNF) => vec![24.0],
            ("1/2", ThreadForm::UNF) => vec![20.0],
            ("5/8", ThreadForm::UNF) => vec![18.0],
            ("3/4", ThreadForm::UNF) => vec![16.0],
            ("1", ThreadForm::UNF) => vec![12.0],

            _ => vec![20.0],
        }
    }

    /// Parse nominal size to major diameter
    pub fn parse_nominal_diameter(&self) -> f64 {
        match self.thread_unit {
            Unit::Metric => {
                // Extract number from "M8" format
                self.nominal_size
                    .trim_start_matches('M')
                    .parse()
                    .unwrap_or(8.0)
            }
            Unit::Imperial => {
                // Parse imperial sizes
                match self.nominal_size.as_str() {
                    "#0" => 0.0600,
                    "#1" => 0.0730,
                    "#2" => 0.0860,
                    "#4" => 0.1120,
                    "#6" => 0.1380,
                    "#8" => 0.1640,
                    "#10" => 0.1900,
                    "#12" => 0.2160,
                    "1/4" => 0.2500,
                    "5/16" => 0.3125,
                    "3/8" => 0.3750,
                    "7/16" => 0.4375,
                    "1/2" => 0.5000,
                    "9/16" => 0.5625,
                    "5/8" => 0.6250,
                    "3/4" => 0.7500,
                    "7/8" => 0.8750,
                    "1" => 1.0000,
                    "1-1/8" => 1.1250,
                    "1-1/4" => 1.2500,
                    "1-3/8" => 1.3750,
                    "1-1/2" => 1.5000,
                    _ => 0.25,
                }
            }
        }
    }

    /// Calculate minor diameter from major diameter and pitch
    pub fn calculate_minor_diameter(&self) -> f64 {
        let major = self.parse_nominal_diameter();
        match self.thread_unit {
            Unit::Metric => major - 1.226869 * self.pitch,
            Unit::Imperial => major - 0.6495 / self.pitch,
        }
    }

    /// Build a Thread object from current state
    pub fn build_thread(&self) -> Thread {
        let major_diameter = self.parse_nominal_diameter();
        let minor_diameter = self.calculate_minor_diameter();

        match self.thread_unit {
            Unit::Metric => Thread {
                unit: Unit::Metric,
                form: self.thread_form,
                major_diameter,
                minor_diameter,
                pitch: self.pitch,
                threads_per_unit: None,
                length: Some(self.thread_length as f64),
                hand: ThreadHand::Right,
                angle: 60.0,
                tolerance_class: Some("6g".to_string()),
                note: None,
            },
            Unit::Imperial => Thread {
                unit: Unit::Imperial,
                form: self.thread_form,
                major_diameter,
                minor_diameter,
                pitch: 1.0 / self.pitch, // Convert TPI to pitch
                threads_per_unit: Some(self.pitch),
                length: Some(self.thread_length as f64),
                hand: ThreadHand::Right,
                angle: 60.0,
                tolerance_class: Some("2A".to_string()),
                note: None,
            },
        }
    }

    /// Build a Bolt from current state
    pub fn build_bolt(&self) -> Bolt {
        let thread = self.build_thread();
        let name = if self.name.is_empty() {
            "Unnamed Bolt".to_string()
        } else {
            self.name.clone()
        };

        Bolt {
            name,
            thread,
            thread_length: self.thread_length,
            head_thickness: self.head_thickness,
            bearing_od: self.bearing_od,
            root_fillet: Some(0.5),
            waisted: self.waisted,
            waist_diameter: if self.waisted {
                Some(self.waist_diameter)
            } else {
                None
            },
            grip_length: self.grip_length,
        }
    }

    /// Build a Stud from current state
    pub fn build_stud(&self) -> Stud {
        let thread = self.build_thread();
        let name = if self.name.is_empty() {
            "Unnamed Stud".to_string()
        } else {
            self.name.clone()
        };

        Stud {
            name,
            thread_a: thread.clone(),
            thread_length_a: self.thread_length,
            thread_b: thread,
            thread_length_b: self.thread_length_b,
            shank_diameter: self.shank_diameter,
            shank_length: self.shank_length,
            nipple_id: 0.0,
            nipple_od: 0.0,
            nipple_angle: 0.0,
            waisted: self.waisted,
            waist_diameter: if self.waisted {
                Some(self.waist_diameter)
            } else {
                None
            },
        }
    }

    /// Load a Bolt into the input state
    pub fn load_from_bolt(&mut self, bolt: &Bolt) {
        self.name = bolt.name.clone();
        self.fastener_type = FastenerType::Bolt;

        // Parse thread information
        self.thread_unit = bolt.thread.unit;
        self.thread_form = bolt.thread.form;
        self.pitch = match bolt.thread.unit {
            Unit::Metric => bolt.thread.pitch,
            Unit::Imperial => bolt.thread.threads_per_unit.unwrap_or(20.0),
        };

        // Set nominal size based on major diameter
        self.nominal_size = match bolt.thread.unit {
            Unit::Metric => format!("M{}", bolt.thread.major_diameter as i32),
            Unit::Imperial => {
                // Find closest imperial size
                let diameter = bolt.thread.major_diameter;
                if diameter < 0.1 {
                    "#0".to_string()
                } else if diameter < 0.11 {
                    "#2".to_string()
                } else if diameter < 0.15 {
                    "#8".to_string()
                } else if diameter < 0.22 {
                    "#10".to_string()
                } else if diameter < 0.29 {
                    "1/4".to_string()
                } else if diameter < 0.34 {
                    "5/16".to_string()
                } else if diameter < 0.41 {
                    "3/8".to_string()
                } else if diameter < 0.47 {
                    "7/16".to_string()
                } else {
                    "1/2".to_string()
                }
            }
        };

        self.thread_length = bolt.thread_length;
        self.waisted = bolt.waisted;
        self.waist_diameter = bolt
            .waist_diameter
            .unwrap_or(bolt.thread.minor_diameter as f32);
        self.grip_length = bolt.grip_length;
        self.head_thickness = bolt.head_thickness;
        self.bearing_od = bolt.bearing_od;
    }

    /// Load a Stud into the input state
    pub fn load_from_stud(&mut self, stud: &Stud) {
        self.name = stud.name.clone();
        self.fastener_type = FastenerType::Stud;

        // Parse thread information from thread_a
        self.thread_unit = stud.thread_a.unit;
        self.thread_form = stud.thread_a.form;
        self.pitch = match stud.thread_a.unit {
            Unit::Metric => stud.thread_a.pitch,
            Unit::Imperial => stud.thread_a.threads_per_unit.unwrap_or(20.0),
        };

        // Set nominal size based on major diameter
        self.nominal_size = match stud.thread_a.unit {
            Unit::Metric => format!("M{}", stud.thread_a.major_diameter as i32),
            Unit::Imperial => {
                let diameter = stud.thread_a.major_diameter;
                if diameter < 0.1 {
                    "#0".to_string()
                } else if diameter < 0.11 {
                    "#2".to_string()
                } else if diameter < 0.15 {
                    "#8".to_string()
                } else if diameter < 0.22 {
                    "#10".to_string()
                } else if diameter < 0.29 {
                    "1/4".to_string()
                } else if diameter < 0.34 {
                    "5/16".to_string()
                } else if diameter < 0.41 {
                    "3/8".to_string()
                } else if diameter < 0.47 {
                    "7/16".to_string()
                } else {
                    "1/2".to_string()
                }
            }
        };

        self.thread_length = stud.thread_length_a;
        self.thread_length_b = stud.thread_length_b;
        self.waisted = stud.waisted;
        self.waist_diameter = stud.waist_diameter.unwrap_or(stud.shank_diameter);
        self.shank_diameter = stud.shank_diameter;
        self.shank_length = stud.shank_length;
    }
}

/// Render the fastener input UI
pub fn render_fastener_input(ui: &mut Ui, state: &mut FastenerInputState) {
    // Name field
    ui.label(egui::RichText::new("Name").strong());
    ui.add_space(4.0);
    ui.add(
        egui::TextEdit::singleline(&mut state.name)
            .hint_text("Enter fastener name")
            .desired_width(f32::INFINITY),
    );

    ui.add_space(12.0);

    // Fastener Type Selection
    ui.label(egui::RichText::new("Fastener Type").strong());
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.fastener_type, FastenerType::Bolt, "Bolt");
        ui.selectable_value(&mut state.fastener_type, FastenerType::Stud, "Stud");
    });

    ui.add_space(12.0);

    // Thread Specification
    ui.label(egui::RichText::new("Thread Specification").strong());
    ui.add_space(4.0);

    // Row 1: Unit and Form
    Flex::horizontal()
        .align_content(FlexAlignContent::Stretch)
        .w_full()
        .show(ui, |flex| {
            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                ui.label("Unit:");
            });
            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                ui.label("Form:");
            });
        });

    ui.add_space(2.0);

    Flex::horizontal()
        .align_content(FlexAlignContent::Stretch)
        .w_full()
        .show(ui, |flex| {
            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                egui::ComboBox::from_id_salt("unit_combo")
                    .selected_text(format!("{:?}", state.thread_unit))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.thread_unit, Unit::Metric, "Metric");
                        ui.selectable_value(&mut state.thread_unit, Unit::Imperial, "Imperial");
                    });
            });

            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                let forms = match state.thread_unit {
                    Unit::Metric => vec![ThreadForm::ISO],
                    Unit::Imperial => vec![ThreadForm::UNC, ThreadForm::UNF],
                };

                egui::ComboBox::from_id_salt("form_combo")
                    .selected_text(format!("{:?}", state.thread_form))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for form in forms {
                            ui.selectable_value(
                                &mut state.thread_form,
                                form,
                                format!("{:?}", form),
                            );
                        }
                    });
            });
        });

    ui.add_space(8.0);

    // Row 2: Size and Pitch
    Flex::horizontal()
        .align_content(FlexAlignContent::Stretch)
        .w_full()
        .show(ui, |flex| {
            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                ui.label("Size:");
            });
            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                ui.label(match state.thread_unit {
                    Unit::Metric => "Pitch (mm):",
                    Unit::Imperial => "TPI:",
                });
            });
        });

    ui.add_space(2.0);

    Flex::horizontal()
        .align_content(FlexAlignContent::Stretch)
        .w_full()
        .show(ui, |flex| {
            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                let sizes = match state.thread_unit {
                    Unit::Metric => FastenerInputState::metric_sizes(),
                    Unit::Imperial => FastenerInputState::imperial_sizes(),
                };

                egui::ComboBox::from_id_salt("size_combo")
                    .selected_text(&state.nominal_size)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for size in sizes {
                            if ui
                                .selectable_label(state.nominal_size == size, size)
                                .clicked()
                            {
                                state.nominal_size = size.to_string();
                                // Auto-update pitch to first option
                                let pitches = match state.thread_unit {
                                    Unit::Metric => FastenerInputState::metric_pitches(size),
                                    Unit::Imperial => {
                                        FastenerInputState::imperial_tpi(size, state.thread_form)
                                    }
                                };
                                if let Some(&first_pitch) = pitches.first() {
                                    state.pitch = first_pitch;
                                }
                            }
                        }
                    });
            });

            flex.add_ui(item().grow(1.0).basis(0.0), |ui| {
                let pitches = match state.thread_unit {
                    Unit::Metric => FastenerInputState::metric_pitches(&state.nominal_size),
                    Unit::Imperial => {
                        FastenerInputState::imperial_tpi(&state.nominal_size, state.thread_form)
                    }
                };

                egui::ComboBox::from_id_salt("pitch_combo")
                    .selected_text(format!("{:.2}", state.pitch))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for pitch in pitches {
                            ui.selectable_value(&mut state.pitch, pitch, format!("{:.2}", pitch));
                        }
                    });
            });
        });

    ui.add_space(12.0);

    // Dimensions Section
    ui.label(egui::RichText::new("Dimensions").strong());
    ui.add_space(4.0);

    match state.fastener_type {
        FastenerType::Bolt => render_bolt_dimensions(ui, state),
        FastenerType::Stud => render_stud_dimensions(ui, state),
    }

    ui.add_space(8.0);

    // Waisted Section
    ui.horizontal(|ui| {
        ui.checkbox(&mut state.waisted, "Waisted");
        if state.waisted {
            ui.label("Diameter:");
            ui.add(
                egui::DragValue::new(&mut state.waist_diameter)
                    .speed(0.1)
                    .range(0.1..=100.0)
                    .suffix(" mm"),
            );
        }
    });

    ui.add_space(8.0);

    // Calculated values display
    ui.separator();
    ui.add_space(4.0);
    ui.label(egui::RichText::new("Calculated Values").size(11.0).weak());
    ui.horizontal(|ui| {
        ui.label(format!("Major ⌀: {:.3} mm", state.parse_nominal_diameter()));
        ui.separator();
        ui.label(format!(
            "Minor ⌀: {:.3} mm",
            state.calculate_minor_diameter()
        ));
    });
}

fn render_bolt_dimensions(ui: &mut Ui, state: &mut FastenerInputState) {
    ui.horizontal(|ui| {
        ui.label("Thread Length:");
        ui.add(
            egui::DragValue::new(&mut state.thread_length)
                .speed(0.5)
                .range(1.0..=500.0)
                .suffix(" mm"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Grip Length:");
        ui.add(
            egui::DragValue::new(&mut state.grip_length)
                .speed(0.5)
                .range(1.0..=500.0)
                .suffix(" mm"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Head Thickness:");
        ui.add(
            egui::DragValue::new(&mut state.head_thickness)
                .speed(0.1)
                .range(0.5..=50.0)
                .suffix(" mm"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Bearing OD:");
        ui.add(
            egui::DragValue::new(&mut state.bearing_od)
                .speed(0.1)
                .range(1.0..=100.0)
                .suffix(" mm"),
        );
    });
}

fn render_stud_dimensions(ui: &mut Ui, state: &mut FastenerInputState) {
    ui.horizontal(|ui| {
        ui.label("Thread Length A:");
        ui.add(
            egui::DragValue::new(&mut state.thread_length)
                .speed(0.5)
                .range(1.0..=500.0)
                .suffix(" mm"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Thread Length B:");
        ui.add(
            egui::DragValue::new(&mut state.thread_length_b)
                .speed(0.5)
                .range(1.0..=500.0)
                .suffix(" mm"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Shank Diameter:");
        ui.add(
            egui::DragValue::new(&mut state.shank_diameter)
                .speed(0.1)
                .range(0.5..=100.0)
                .suffix(" mm"),
        );
    });

    ui.horizontal(|ui| {
        ui.label("Shank Length:");
        ui.add(
            egui::DragValue::new(&mut state.shank_length)
                .speed(0.5)
                .range(1.0..=500.0)
                .suffix(" mm"),
        );
    });
}
