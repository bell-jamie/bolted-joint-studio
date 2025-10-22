use crate::modules::fastener_input::FastenerInputState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UIState {
    pub show_library_panel: bool,
    pub show_settings: bool,
    pub fastener_input: FastenerInputState,
    pub fastener_autosave: bool,
}

impl UIState {
    pub fn default() -> Self {
        Self {
            show_library_panel: true,
            show_settings: false,
            fastener_input: FastenerInputState::default(),
            fastener_autosave: true,
        }
    }
}
