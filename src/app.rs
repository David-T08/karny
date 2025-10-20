use eframe::egui;

use crate::ui::{self, modals};

#[derive(Default)]
pub struct AppState {
    pub modal_states: modals::ModalState,
}

pub fn app() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::topbar::update(ctx);

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .width_range(180.0..=720.0)
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.take_available_space();
                ui.collapsing("Variables", |ui| {
                    if ui.button("Add new variable").clicked() {
                        self.modal_states.add_variable.show = true;
                    }

                    ui.collapsing("Inputs", |_ui| {});
                    ui.collapsing("Outputs", |_ui| {});
                });
            });

        // 3) Central panel LAST so it takes the remaining space cleanly
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Karnaugh Map");
            ui.label("Main content goes here");
        });

        modals::update(ctx, self);
    }
}
