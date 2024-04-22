use egui::{Button, Color32, RichText, TextBuffer, Widget};

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
    labels: Vec<Vec<String>>,
    selection: Vec<(usize, usize)>,
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
            labels: vec![
                vec![
                    String::from("Label 1"),
                    String::from("Label 2"),
                    String::from("Label 3"),
                ],
                vec![
                    String::from("Row 2.1"),
                    String::from("Row 2.2"),
                    String::from("Row 2.3"),
                ],
            ],
            selection: Vec::new(),
        }
    }
}

impl Verby {
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

    fn verbs_list(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);

        ui.spacing_mut().item_spacing = [10.0, 10.0].into();
        ui.columns(3, |cols| {
            // let width = (ui.available_width() - margin * 2.0) / 3.0;
            for (row_idx, row) in &mut self.labels.iter_mut().enumerate() {
                for (col_idx, label) in &mut row.iter_mut().enumerate() {
                    cols[col_idx].vertical_centered_justified(|ui| {
                        let index = self.selection.iter().position(|s| s == &(row_idx, col_idx));
                        let mut text = label.to_owned();
                        if let Some(idx) = index {
                            text = format!("{} #{}", label, idx + 1);
                        }
                        let resp = ui.selectable_label(
                            index.is_some(),
                            RichText::new(text)
                                .monospace()
                                .text_style(egui::TextStyle::Button)
                                .size(30.0),
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
    }

    fn edit_mode(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};
        ui.columns(3, |cols| {
            cols[0].vertical_centered_justified(|ui| ui.text_edit_singleline(&mut self.first));
            cols[1].vertical_centered_justified(|ui| ui.text_edit_singleline(&mut self.second));
            cols[2].vertical_centered_justified(|ui| ui.text_edit_singleline(&mut self.third));
        });
        ui.add_space(10.0);
        ui.vertical_centered_justified(|ui| {
            let btn = Button::new(RichText::new("Add verb").color(Color32::BLACK).heading());

            let len1 = self.first.len();
            let len2 = self.second.len();
            let len3 = self.third.len();
            let item = (self.first.clone(), self.second.clone(), self.third.clone());
            if len1 == 0 || len2 == 0 || len3 == 0 || self.verbs.contains(&item){
                ui.add_enabled(false, btn);
            } else if len1 < 3 || len2 < 3 || len3 < 3 {
                btn.fill(Color32::LIGHT_RED).ui(ui);
            } else if btn.fill(Color32::LIGHT_GREEN).ui(ui).clicked() {
                self.verbs.push(item);
            }
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                // ui.spacing_mut().item_spacing.x = 0.0;
                let table = TableBuilder::new(ui)
                    .columns(Column::auto(), 3)
                    .striped(true)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ));
                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("First form");
                        });
                        header.col(|ui| {
                            ui.heading("Second form");
                        });
                        header.col(|ui| {
                            ui.heading("Third form");
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
                        });
                    });
            });
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.0);

            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.add_space(16.0);
                    ui.toggle_value(&mut self.edit_mode, RichText::new("Edit verbs").heading());
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
