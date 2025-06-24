use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use egui::{Button, Color32, RichText, Widget};

// egui_keyboard is available on Android and Desktop
use egui_keyboard::Keyboard;

const COLUMNS: usize = 3;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Verby {
    #[serde(skip)]
    first: String,
    #[serde(skip)]
    second: String,
    #[serde(skip)]
    third: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    edit_mode: bool,

    verbs: Vec<(String, String, String)>,
    labels: Vec<String>,
    selection: Vec<(usize, usize)>,
    deleted: Vec<usize>,

    // Keyboard support
    #[serde(skip)]
    keyboard: Keyboard,
    #[serde(skip)]
    show_keyboard: bool,
    #[serde(skip)]
    active_field: Option<usize>, // 0=first, 1=second, 2=third
}

impl Default for Verby {
    fn default() -> Self {
        Self {
            first: String::new(),
            second: String::new(),
            third: String::new(),
            edit_mode: false,
            verbs: vec![(
                String::from("first"),
                String::from("second"),
                String::from("third"),
            )],
            labels: vec![],
            selection: Vec::new(),
            deleted: Vec::new(),

            // Keyboard fields
            keyboard: Keyboard::default(),
            show_keyboard: false,
            active_field: None,
        }
    }
}

impl Verby {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Get current context style
        let mut style = (*cc.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(25.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Monospace, FontId::new(20.0, Proportional)),
            (Button, FontId::new(20.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();

        // Mutate global style with above changes
        cc.egui_ctx.set_style(style);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        log::info!("Checking for storage availability...");
        if let Some(storage) = cc.storage {
            log::info!("✅ Storage is available! Loading previous state...");
            let loaded_app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            log::info!(
                "✅ Successfully loaded {} verbs from storage",
                loaded_app.verbs.len()
            );
            return loaded_app;
        } else {
            log::warn!("❌ No storage available! Android persistence may not be working");
            log::warn!("Creating new default app - state will not persist");
        }

        Default::default()
    }

    fn verbs_list(&mut self, ui: &mut egui::Ui) {
        if self.labels.is_empty() {
            for verbs in &self.verbs {
                self.labels.push(verbs.0.clone());
                self.labels.push(verbs.1.clone());
                self.labels.push(verbs.2.clone());
            }
        }
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing = [10.0, 10.0].into();
            ui.columns(COLUMNS, |cols| {
                // let width = (ui.available_width() - margin * 2.0) / 3.0;
                for (row_idx, row) in self.labels.chunks(COLUMNS).enumerate() {
                    for (col_idx, label) in &mut row.iter().enumerate() {
                        cols[col_idx].vertical_centered_justified(|ui| {
                            let index =
                                self.selection.iter().position(|s| s == &(row_idx, col_idx));
                            let mut text = label.to_owned();
                            if let Some(idx) = index {
                                text = format!("{} #{}", label, idx + 1);
                            }
                            let resp = ui.selectable_label(
                                index.is_some(),
                                RichText::new(text).monospace().strong(),
                            );
                            if resp.clicked() {
                                if let Some(idx) = index {
                                    self.selection.remove(idx);
                                } else {
                                    if self.selection.len() > 2 {
                                        self.selection.truncate(2);
                                    }
                                    self.selection.push((row_idx, col_idx));
                                }
                            }
                        });
                    }
                }
            });
            if self.selection.len() == COLUMNS && self.check_selection() {
                ui.ctx().request_repaint();
            }
        });
    }
    fn check_selection(&mut self) -> bool {
        let idx1 = self.selection[0].0 * COLUMNS + self.selection[0].1;
        let idx2 = self.selection[1].0 * COLUMNS + self.selection[1].1;
        let idx3 = self.selection[2].0 * COLUMNS + self.selection[2].1;
        let row = &(
            self.labels[idx1].clone(),
            self.labels[idx2].clone(),
            self.labels[idx3].clone(),
        );
        if !self.verbs.contains(row) {
            return false;
        }
        let mut to_remove = [idx1, idx2, idx3];
        to_remove.sort();

        for (sub, idx) in to_remove.iter().enumerate() {
            let idx_to_remove = idx - sub;
            log::debug!("remove idx {idx_to_remove}");
            self.labels.remove(idx_to_remove);
        }
        self.selection.clear();
        true
    }

    fn edit_mode(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        // IMPORTANT: pump_events must be called before any widgets are created
        self.keyboard.pump_events(ui.ctx());

        ui.columns(3, |cols| {
            cols[0].vertical_centered_justified(|ui| {
                let response = ui.text_edit_singleline(&mut self.first);
                if response.clicked() {
                    self.show_keyboard = true;
                    self.active_field = Some(0);
                }
                // Auto-focus when this field is active
                if self.active_field == Some(0) {
                    response.request_focus();
                }
            });
            cols[1].vertical_centered_justified(|ui| {
                let response = ui.text_edit_singleline(&mut self.second);
                if response.clicked() {
                    self.show_keyboard = true;
                    self.active_field = Some(1);
                }
                // Auto-focus when this field is active
                if self.active_field == Some(1) {
                    response.request_focus();
                }
            });
            cols[2].vertical_centered_justified(|ui| {
                let response = ui.text_edit_singleline(&mut self.third);
                if response.clicked() {
                    self.show_keyboard = true;
                    self.active_field = Some(2);
                }
                // Auto-focus when this field is active
                if self.active_field == Some(2) {
                    response.request_focus();
                }
            });
        });

        // Show keyboard if requested
        if self.show_keyboard {
            egui::TopBottomPanel::bottom("keyboard").show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Hide Keyboard").clicked() {
                        self.show_keyboard = false;
                        self.active_field = None;
                    }
                });

                // Show the virtual keyboard
                self.keyboard.show(ui.ctx());

                // Show which field is active
                if let Some(field_idx) = self.active_field {
                    let field_name = match field_idx {
                        0 => "First form",
                        1 => "Second form",
                        2 => "Third form",
                        _ => "Unknown",
                    };
                    ui.label(format!("Editing: {}", field_name));
                }
            });
        }

        ui.add_space(5.0);
        ui.vertical_centered_justified(|ui| {
            let btn = Button::new("Add verb");

            let len1 = self.first.len();
            let len2 = self.second.len();
            let len3 = self.third.len();
            let item = (self.first.clone(), self.second.clone(), self.third.clone());
            if len1 == 0 || len2 == 0 || len3 == 0 || self.verbs.contains(&item) {
                ui.add_enabled(false, btn);
            } else if len1 < 3 || len2 < 3 || len3 < 3 {
                btn.fill(Color32::LIGHT_RED).ui(ui);
            } else if btn.fill(Color32::LIGHT_GREEN).ui(ui).clicked() {
                self.verbs.push(item);
                self.first.clear();
                self.second.clear();
                self.third.clear();
            }
        });
        ui.add_space(10.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            let _width = ui.available_width() - 30.0;
            ui.spacing_mut().item_spacing.x = 0.0;
            //ui.spacing_mut().item_spacing.x += 5.0;
            let table = TableBuilder::new(ui)
                .columns(Column::auto(), 3)
                .column(Column::exact(30.0))
                .striped(true)
                .cell_layout(egui::Layout::centered_and_justified(
                    egui::Direction::LeftToRight,
                ));
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("First form");
                    });
                    header.col(|ui| {
                        ui.strong("Second form");
                    });
                    header.col(|ui| {
                        ui.strong("Third form");
                    });
                    header.col(|ui| {
                        ui.strong("Del");
                    });
                })
                .body(|body| {
                    body.rows(20.0, self.verbs.len(), |mut row| {
                        let idx = row.index();
                        row.col(|ui| {
                            ui.label(&self.verbs[idx].0);
                        });
                        row.col(|ui| {
                            ui.label(&self.verbs[idx].1);
                        });
                        row.col(|ui| {
                            ui.label(&self.verbs[idx].2);
                        });
                        row.col(|ui| {
                            if ui.button(RichText::new("❌").color(Color32::RED)).clicked() {
                                self.deleted.push(idx);
                            }
                        });
                    });
                });
            if !self.deleted.is_empty() {
                for idx in &self.deleted {
                    self.verbs.remove(*idx);
                }
                self.selection.clear();
                self.labels.clear();
            }
            self.deleted.clear();
        });
    }
}

impl eframe::App for Verby {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Reserve some space at the top so the UI isn't hidden behind the android status bar
        // TODO: This is a workaround until safe_area is implemented for android
        #[cfg(target_os = "android")]
        egui::TopBottomPanel::top("status_bar_space").show(ctx, |ui| {
            ui.set_height(32.0);
        });

        // Put your widget
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.0);

            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    // Custom theme buttons (Light/Dark only, no System)
                    let visuals = ui.ctx().style().visuals.clone();
                    if visuals.dark_mode {
                        if ui.button("🌙").on_hover_text("Dark mode").clicked() {
                            ui.ctx().set_visuals(egui::Visuals::light());
                        }
                    } else if ui.button("☀").on_hover_text("Light mode").clicked() {
                        ui.ctx().set_visuals(egui::Visuals::dark());
                    }
                    ui.add_space(16.0);

                    ui.toggle_value(&mut self.edit_mode, "Edit verbs");
                    if ui.button("Reset").clicked() {
                        self.labels.clear();
                        self.selection.clear();
                    }
                });
            });
            ui.add_space(10.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.edit_mode {
                self.edit_mode(ui);
            } else {
                self.verbs_list(ui);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
                ui.separator();
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
