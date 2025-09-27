/// Material type: metal, polymer, ceramic, etc.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaterialType {
    #[default]
    Metal,
    Polymer,
    Ceramic,
    Composite,
    Wood,
    Other,
}

/// Material standard or source (optional)
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct MaterialStandard {
    pub designation: String,  // e.g., "ASTM A36", "ISO 898-1"
    pub organization: String, // e.g., "ASTM", "ISO"
    pub note: Option<String>,
}

/// Material struct for engineering
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct Material {
    // Identification
    pub name: String,                       // common name, e.g., "Steel A36"
    pub material_type: MaterialType,        // e.g., Metal
    pub standard: Option<MaterialStandard>, // optional standard

    // Mechanical properties
    pub density: Option<f64>,          // kg/m³
    pub youngs_modulus: Option<f64>,   // Pa
    pub shear_modulus: Option<f64>,    // Pa
    pub poisson_ratio: Option<f64>,    // unitless
    pub tensile_strength: Option<f64>, // Pa
    pub yield_strength: Option<f64>,   // Pa
    pub hardness: Option<f64>,         // Vickers, Brinell, Rockwell

    // Thermal properties
    pub thermal_conductivity: Option<f64>, // W/(m·K)
    pub thermal_expansion: Option<f64>,    // 1/K
    pub specific_heat: Option<f64>,        // J/(kg·K)
    pub melting_point: Option<f64>,        // °C

    // Electrical properties
    pub electrical_conductivity: Option<f64>, // S/m
    pub resistivity: Option<f64>,             // Ω·m

    // Optional metadata
    pub cost_per_kg: Option<f64>, // currency/kg
    pub note: Option<String>,     // any descriptive notes
}

impl Material {
    /// Create a new basic material
    pub fn new<S: Into<String>>(name: S, material_type: MaterialType) -> Self {
        Self {
            name: name.into(),
            material_type,
            standard: None,
            density: None,
            youngs_modulus: None,
            shear_modulus: None,
            poisson_ratio: None,
            tensile_strength: None,
            yield_strength: None,
            hardness: None,
            thermal_conductivity: None,
            thermal_expansion: None,
            specific_heat: None,
            melting_point: None,
            electrical_conductivity: None,
            resistivity: None,
            cost_per_kg: None,
            note: None,
        }
    }

    /// Set a standard for the material
    pub fn set_standard<S1: Into<String>, S2: Into<String>>(
        &mut self,
        designation: S1,
        organization: S2,
        note: Option<String>,
    ) {
        self.standard = Some(MaterialStandard {
            designation: designation.into(),
            organization: organization.into(),
            note,
        });
    }

    /// Set mechanical properties
    pub fn set_mechanical(
        &mut self,
        density: f64,
        youngs_modulus: f64,
        shear_modulus: f64,
        poisson_ratio: f64,
        tensile_strength: f64,
        yield_strength: f64,
        hardness: f64,
    ) {
        self.density = Some(density);
        self.youngs_modulus = Some(youngs_modulus);
        self.shear_modulus = Some(shear_modulus);
        self.poisson_ratio = Some(poisson_ratio);
        self.tensile_strength = Some(tensile_strength);
        self.yield_strength = Some(yield_strength);
        self.hardness = Some(hardness);
    }

    /// Set thermal properties
    pub fn set_thermal(
        &mut self,
        thermal_conductivity: f64,
        thermal_expansion: f64,
        specific_heat: f64,
        melting_point: f64,
    ) {
        self.thermal_conductivity = Some(thermal_conductivity);
        self.thermal_expansion = Some(thermal_expansion);
        self.specific_heat = Some(specific_heat);
        self.melting_point = Some(melting_point);
    }

    /// Set electrical properties
    pub fn set_electrical(&mut self, conductivity: f64, resistivity: f64) {
        self.electrical_conductivity = Some(conductivity);
        self.resistivity = Some(resistivity);
    }

    /// Add a descriptive note
    pub fn set_note<S: Into<String>>(&mut self, note: S) {
        self.note = Some(note.into());
    }
}
