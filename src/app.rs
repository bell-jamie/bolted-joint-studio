use crate::modules::{geometry, joint::BoltedJoint, library::Library, state::UIState};
use egui::{Frame, Rounding, Stroke};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, FlexDirection, FlexItem, item};
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
    ui.add_space(8.0);
    
    egui::ScrollArea::vertical()
        .auto_shrink([false, true])
        .show(ui, |ui| {
            // Account for scroll area margins and potential scrollbar
            let available_width = ui.available_width() - 24.0; // Give some breathing room
            let card_spacing = 12.0;
            let min_card_width = 200.0;
            
            // Add padding to the entire content area
            ui.add_space(12.0);
            
            // Calculate how many cards can fit per row
            let cards_per_row = ((available_width + card_spacing) / (min_card_width + card_spacing)).floor() as usize;
            let cards_per_row = cards_per_row.max(1);
            
            // Hero/Banner Card - Full Width  
            ui.horizontal(|ui| {
                ui.add_space(12.0); // Left padding
                ui.vertical(|ui| {
                    Self::hero_card(ui, available_width - 24.0); // Account for left+right padding
                });
                ui.add_space(12.0); // Right padding
            });
            ui.add_space(card_spacing);
            
            // Main Feature Cards Grid
            ui.horizontal(|ui| {
                ui.add_space(12.0); // Left padding
                
                ui.vertical(|ui| {
                    Flex::horizontal()
                        .wrap(true)
                        .align_content(FlexAlignContent::Start)
                        .gap(egui::vec2(card_spacing, card_spacing))
                        .show(ui, |flex| {
                            // Joint Design Card - Large
                            flex.add_ui(
                                FlexItem::new()
                                    .basis(min_card_width * 1.4)
                                    .shrink()
                                    .grow(1.4),
                                |ui| {
                                    Self::card(ui, "Joint Design", "⚙️", true, |ui| {
                                        ui.label("Configure your bolted joint parameters:");
                                        ui.add_space(8.0);
                                        
                                        ui.horizontal(|ui| {
                                            ui.label("Diameter:");
                                            ui.add(egui::Slider::new(&mut 12.0, 6.0..=30.0).suffix(" mm"));
                                        });
                                        
                                        ui.horizontal(|ui| {
                                            ui.label("Length:");
                                            ui.add(egui::Slider::new(&mut 50.0, 20.0..=200.0).suffix(" mm"));
                                        });
                                        
                                        ui.horizontal(|ui| {
                                            ui.label("Grade:");
                                            egui::ComboBox::new("grade", "")
                                                .selected_text("ISO 8.8")
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(&mut "", "ISO 8.8", "ISO 8.8");
                                                    ui.selectable_value(&mut "", "ISO 10.9", "ISO 10.9");
                                                });
                                        });
                                    });
                                }
                            );

                            // 3D Preview Card - Large
                            flex.add_ui(
                                FlexItem::new()
                                    .basis(min_card_width * 1.4)
                                    .shrink()
                                    .grow(1.4),
                                |ui| {
                                    Self::card(ui, "3D Preview", "👁️", true, |ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(20.0);
                                            ui.label(egui::RichText::new("🔩").size(48.0));
                                            ui.add_space(8.0);
                                            ui.label("Interactive 3D Model");
                                            ui.add_space(12.0);
                                            if ui.button("Launch Viewer").clicked() {
                                                // Launch 3D viewer
                                            }
                                        });
                                    });
                                }
                            );

                            // Stress Analysis Card
                            flex.add_ui(
                                FlexItem::new()
                                    .basis(min_card_width)
                                    .shrink()
                                    .grow(1.0),
                                |ui| {
                                    Self::card(ui, "Stress Analysis", "📊", false, |ui| {
                                        ui.label("Latest Results:");
                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            ui.label("Max Stress:");
                                            ui.label(egui::RichText::new("245 MPa").strong());
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Safety Factor:");
                                            ui.label(egui::RichText::new("2.8").color(egui::Color32::from_rgb(0, 150, 0)));
                                        });
                                        if ui.small_button("View Details").clicked() {
                                            // Show analysis details
                                        }
                                    });
                                }
                            );

                            // Load Calculator Card
                            flex.add_ui(
                                FlexItem::new()
                                    .basis(min_card_width)
                                    .shrink()
                                    .grow(1.0),
                                |ui| {
                                    Self::card(ui, "Load Calculator", "🔢", false, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("Applied Load:");
                                            ui.add(egui::DragValue::new(&mut 1000).suffix(" N"));
                                        });
                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            ui.label("Preload:");
                                            ui.label(egui::RichText::new("2800 N").strong());
                                        });
                                        if ui.small_button("Calculate").clicked() {
                                            // Perform calculation
                                        }
                                    });
                                }
                            );

                            // Material Library Card
                            flex.add_ui(
                                FlexItem::new()
                                    .basis(min_card_width)
                                    .shrink()
                                    .grow(1.0),
                                |ui| {
                                    Self::card(ui, "Material Library", "📚", false, |ui| {
                                        ui.label("Quick Access:");
                                        ui.add_space(4.0);
                                        if ui.small_button("ISO Standards").clicked() {}
                                        if ui.small_button("ASTM Standards").clicked() {}
                                        if ui.small_button("DIN Standards").clicked() {}
                                    });
                                }
                            );

                            // Reports Card
                            flex.add_ui(
                                FlexItem::new()
                                    .basis(min_card_width)
                                    .shrink()
                                    .grow(1.0),
                                |ui| {
                                    Self::card(ui, "Reports", "📄", false, |ui| {
                                        ui.label("Generate documentation:");
                                        ui.add_space(4.0);
                                        if ui.small_button("Calculation Report").clicked() {}
                                        if ui.small_button("Export PDF").clicked() {}
                                    });
                                }
                            );
                        });
                });
                
                ui.add_space(12.0); // Right padding
            });
            
            ui.add_space(card_spacing * 2.0);
            
            // Add bottom padding
            ui.add_space(12.0);
        });
}

fn hero_card(ui: &mut egui::Ui, width: f32) {
    Frame::group(ui.style())
        .corner_radius(12)
        .stroke(Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color.gamma_multiply(0.3)))
        .fill(ui.visuals().panel_fill)
        .inner_margin(16)
        .show(ui, |ui| {
            ui.set_width(width - 24.0);
            
            Flex::horizontal()
                .align_items(FlexAlign::Center)
                .show(ui, |flex| {
                    flex.add_ui(FlexItem::new().grow(1.0), |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(8.0);
                            
                            // Large title
                            ui.label(egui::RichText::new("Advanced Bolted Joint Analysis")
                                .size(22.0)
                                .strong()
                                .color(ui.visuals().text_color()));
                            
                            ui.add_space(6.0);
                            
                            // Subtitle
                            ui.label(egui::RichText::new("Professional engineering tool for bolt design, analysis, and verification")
                                .size(14.0)
                                .color(ui.visuals().weak_text_color()));
                            
                            ui.add_space(12.0);
                            
                            // Action button
                            if ui.add(egui::Button::new("Start New Project")
                                .min_size(egui::vec2(140.0, 36.0)))
                                .clicked() {
                                // Handle new project
                            }
                        });
                    });
                    
                    flex.add_ui(FlexItem::new().basis(80.0), |ui| {
                        // Large icon or illustration
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("🔩").size(48.0));
                        });
                    });
                });
        });
}

fn card<F>(ui: &mut egui::Ui, title: &str, icon: &str, is_large: bool, content: F) 
where
    F: FnOnce(&mut egui::Ui),
{
    let card_height = if is_large { 140.0 } else { 120.0 };
    
    Frame::group(ui.style())
        .corner_radius(10)
        .stroke(Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color.gamma_multiply(0.5)))
        .fill(ui.visuals().panel_fill)
        .inner_margin(14)
        .show(ui, |ui| {
            ui.set_min_height(card_height - 28.0);
            
            ui.vertical(|ui| {
                // Header with icon and title
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icon).size(20.0));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(title)
                        .size(16.0)
                        .strong()
                        .color(ui.visuals().text_color()));
                });
                
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);
                
                // Custom content
                content(ui);
            });
        });
}

    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                Flex::horizontal()
                    .align_content(FlexAlignContent::Stretch)
                    .w_full()
                    .show(ui, |flex| {
                        flex.add_ui(item(), |ui| ui.label("Ready"));
                        flex.add_ui(item().grow(1.0), |ui| ui.label("Modified: Today"));
                        flex.add_ui(item(), |ui| ui.label("v0.0.1"));
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

        self.show_main_panel(ctx);
    }
}
