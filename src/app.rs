use crate::modules::{
    geometry, joint::BoltedJoint, library::Library, state::UIState, utils::text_width,
};
use egui::{vec2, Frame, Rounding, Stroke, Vec2};
// use egui_flex::{Flex, FlexAlign, FlexAlignContent, FlexDirection, FlexItem, item};
use hello_egui_utils::center::Center;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Studio {
    joint: BoltedJoint,
    library: Library,
    state: UIState,
}

impl Default for Studio {
    fn default() -> Self {
        Self {
            joint: BoltedJoint::default(),
            library: Library::default(),
            state: UIState::default(),
        }
    }
}
struct GridHelper {
    cell_size: egui::Vec2,
    gap: f32,
    padding: f32,
    grid_cols: usize,
}

impl GridHelper {
    fn new(
        mut cell_size: Vec2,
        gap: f32,
        padding: f32,
        grid_cols: usize,
        window_width: f32,
    ) -> Self {
        // Adjust cell width based on window size
        let pad_sum_x = padding * 2.0 + gap * (grid_cols - 1) as f32;
        let cell_widest = ((window_width - 32.0) - pad_sum_x) / grid_cols as f32; // magic 32.0 -> 4 * 8.0?
        cell_size.x = cell_size.x.max(cell_widest);

        Self {
            cell_size,
            gap,
            padding,
            grid_cols,
        }
    }

    fn card_size(&self, width_units: usize, height_units: usize) -> Vec2 {
        vec2(
            (width_units as f32) * self.cell_size.x + ((width_units - 1) as f32) * (self.gap + 8.0), // not sure where this 8.0 comes from
            (height_units as f32) * self.cell_size.y + ((height_units - 1) as f32) * self.gap,
        )
    }

    fn total_width(&self) -> f32 {
        self.card_size(self.grid_cols, 1).x + (self.padding * 2.0)
    }
}

impl Studio {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn show_main_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            self.show_main_menu(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_central_content(ui);
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.show_status_bar(ui);
        });

        if self.state.show_settings {
            self.show_settings_window(ctx);
        }
    }

    fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    // Handle new file
                    ui.close();
                }
                if ui.button("Open").clicked() {
                    // Handle open file
                    ui.close();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    // Handle exit
                    ui.close();
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Settings").clicked() {
                    self.state.show_settings = true;
                    ui.close();
                }
            });

            ui.menu_button("View", |ui| {
                // Toggle panels visibility here
                egui::widgets::global_theme_preference_buttons(ui);
                ui.checkbox(&mut self.state.show_nav_panel, "Nav Panel");
                ui.checkbox(&mut true, "Properties Panel");
            });

            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    // Show about dialog
                    ui.close();
                }
            });
        });
    }

    fn show_central_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Bolted Joint Studio");
        ui.add_space(12.0);

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                // Base grid configuration
                let base_cell_size = egui::vec2(240.0, 160.0); // Base unit dimensions
                let gap = 10.0; // Gap between cards
                let padding = 10.0; // Padding around grid
                let grid_cols = 4; // Total columns in grid

                // Calculate grid helper
                let grid = GridHelper::new(
                    base_cell_size,
                    gap,
                    padding,
                    grid_cols,
                    ui.available_width(),
                );

                // Set minimum size for the entire grid area
                let total_width = grid.total_width();
                ui.set_min_size(egui::vec2(total_width, 600.0));

                ui.add_space(padding);

                ui.horizontal(|ui| {
                    ui.add_space(padding);

                    ui.vertical(|ui| {
                        // Row 1
                        ui.horizontal(|ui| {
                            // Large design card (2x1)
                            ui.allocate_ui(grid.card_size(2, 1), |ui| {
                                Self::sized_card(ui, "Joint Design", "⚙️", |ui| {
                                    ui.columns(2, |cols| {
                                        cols[0].label("Diameter:");
                                        cols[0].add(egui::Slider::new(&mut 12.0, 6.0..=30.0));

                                        cols[1].label("Length:");
                                        cols[1].add(egui::Slider::new(&mut 50.0, 20.0..=200.0));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Grade:");
                                        egui::ComboBox::new("grade", "")
                                            .selected_text("ISO 8.8")
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut "", "ISO 8.8", "ISO 8.8");
                                                ui.selectable_value(
                                                    &mut "", "ISO 10.9", "ISO 10.9",
                                                );
                                            });
                                    });
                                });
                            });

                            ui.add_space(gap);

                            // Preview card (2x1)
                            ui.allocate_ui(grid.card_size(2, 1), |ui| {
                                Self::sized_card(ui, "3D Preview", "👁️", |ui| {
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new("🔩").size(48.0));
                                            ui.label("Interactive Model");
                                        });
                                        ui.separator();
                                        ui.vertical(|ui| {
                                            ui.label("Controls:");
                                            if ui.button("Launch Viewer").clicked() {}
                                            if ui.button("Export STL").clicked() {}
                                            ui.checkbox(&mut true, "Show dimensions");
                                        });
                                    });
                                });
                            });
                        });

                        ui.add_space(gap);

                        // Row 2
                        ui.horizontal(|ui| {
                            // Analysis card (1x1)
                            ui.allocate_ui(grid.card_size(1, 1), |ui| {
                                Self::sized_card(ui, "Analysis", "📊", |ui| {
                                    ui.label("Results:");
                                    ui.horizontal(|ui| {
                                        ui.label("Stress:");
                                        ui.label(egui::RichText::new("245 MPa").strong());
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Factor:");
                                        ui.label(
                                            egui::RichText::new("2.8").color(egui::Color32::GREEN),
                                        );
                                    });
                                });
                            });

                            ui.add_space(gap);

                            // Calculator card (1x1)
                            ui.allocate_ui(grid.card_size(1, 1), |ui| {
                                Self::sized_card(ui, "Calculator", "🔢", |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Load:");
                                        ui.add(egui::DragValue::new(&mut 1000).suffix(" N"));
                                    });
                                    ui.label("Preload: 2800 N");
                                });
                            });

                            ui.add_space(gap);

                            // Materials card (1x1)
                            ui.allocate_ui(grid.card_size(1, 1), |ui| {
                                Self::sized_card(ui, "Materials", "📚", |ui| {
                                    ui.label("Standards:");
                                    if ui.small_button("ISO 8.8").clicked() {}
                                    if ui.small_button("ISO 10.9").clicked() {}
                                    if ui.small_button("ASTM").clicked() {}
                                });
                            });

                            ui.add_space(gap);

                            // Reports card (1x1)
                            ui.allocate_ui(grid.card_size(1, 1), |ui| {
                                Self::sized_card(ui, "Reports", "📄", |ui| {
                                    ui.label("Export:");
                                    if ui.small_button("PDF Report").clicked() {}
                                    if ui.small_button("Data CSV").clicked() {}
                                });
                            });
                        });

                        ui.add_space(gap);

                        // Row 3 - Example of tall card
                        ui.horizontal(|ui| {
                            // Settings card (1x2) - tall card
                            ui.allocate_ui(grid.card_size(1, 2), |ui| {
                                Self::sized_card(ui, "Settings", "⚙️", |ui| {
                                    ui.label("Preferences:");
                                    ui.checkbox(&mut true, "Auto-save");
                                    ui.checkbox(&mut false, "Dark mode");
                                    ui.separator();
                                    ui.label("Units:");
                                    ui.radio_value(&mut 0, 0, "Metric");
                                    ui.radio_value(&mut 0, 1, "Imperial");
                                    ui.separator();
                                    ui.label("Precision:");
                                    ui.add(egui::Slider::new(&mut 3, 1..=6).text("Decimals"));
                                });
                            });

                            ui.add_space(gap);

                            // Log/Status card (3x2) - wide card
                            ui.allocate_ui(grid.card_size(3, 2), |ui| {
                                Self::sized_card(ui, "System Log", "📋", |ui| {
                                    egui::ScrollArea::vertical()
                                        .max_height(60.0)
                                        .show(ui, |ui| {
                                            ui.label("✓ Bolt analysis completed");
                                            ui.label("⚠ Material database needs update");
                                            ui.label("ℹ Last calculation: 2 minutes ago");
                                            ui.label("✓ Export successful");
                                        });
                                });
                            });
                        });
                    });

                    ui.add_space(padding);
                });

                ui.add_space(padding);
            });
    }

    fn sized_card<F>(ui: &mut egui::Ui, title: &str, icon: &str, content: F)
    where
        F: FnOnce(&mut egui::Ui),
    {
        Frame::group(ui.style())
            .corner_radius(10.0)
            .stroke(Stroke::new(
                1.0,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .fill(ui.visuals().panel_fill)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.expand_to_include_rect(ui.max_rect());

                ui.vertical(|ui| {
                    // Header
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(icon).size(16.0));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(title).size(14.0).strong());
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);

                    // Content fills remaining space
                    content(ui);
                });
            });
    }

    /// egui_flex version
    // fn show_status_bar(&mut self, ui: &mut egui::Ui) {
    //     egui::Frame::new()
    //         .inner_margin(egui::Margin::symmetric(8, 4))
    //         .show(ui, |ui| {
    //             Flex::horizontal()
    //                 .align_content(FlexAlignContent::Stretch)
    //                 .w_full()
    //                 .show(ui, |flex| {
    //                     flex.add_ui(item(), |ui| ui.label("Ready"));
    //                     flex.add_ui(item().grow(1.0), |ui| ui.label("Modified: Today"));
    //                     flex.add_ui(item(), |ui| ui.label("v0.0.1"));
    //                 });
    //         });
    // }

    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let version = "v0.0.1";
                    let font_size = ui.style().text_styles[&egui::TextStyle::Body].size;

                    // Left item
                    ui.label("Ready");

                    // Center item - use allocate_space to take remaining space minus right item
                    let available_width = ui.available_width();
                    let right_item_width = text_width(ui, &version, font_size).x + 5.0; // magic 5.0

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(available_width - right_item_width, ui.available_height()),
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.label("Modified: Today");
                        },
                    );

                    // Right item
                    ui.label(version);
                });
            });
    }

    fn show_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .open(&mut self.state.show_settings)
            .vscroll(true)
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Application Settings");
                ui.separator();
            });
    }
}

impl eframe::App for Studio {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:

        //     egui::MenuBar::new().ui(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }

        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });
        // });

        // ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(vec2(2000.0, 600.0))); // doesn't seem to work?

        self.show_main_panel(ctx);
    }
}
