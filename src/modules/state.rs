#[derive(serde::Deserialize, serde::Serialize)]
pub struct UIState {
    pub show_nav_panel: bool,
    pub show_prop_panel: bool,
    pub show_settings: bool,
}

impl UIState {
    pub fn default() -> Self {
        Self {
            show_nav_panel: true,
            show_prop_panel: true,
            show_settings: false,
        }
    }
}
