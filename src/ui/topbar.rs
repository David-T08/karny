use egui::TopBottomPanel;
use egui_extras::{Size, StripBuilder};

pub fn file_menu(ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        ui.label("Project");
        ui.separator();

        if ui.button("New Project").clicked() {}

        if ui.button("Open Project").clicked() {}

        if ui.button("Save Project").clicked() {}

        ui.add_space(8.0);
        ui.label("Window");
        ui.separator();
        if ui.button("Quit").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

pub fn window_menu(ui: &mut egui::Ui) {
    ui.menu_button("Window", |ui| {
        ui.checkbox(&mut false, "Table View");
        ui.checkbox(&mut false, "Map View");
        ui.checkbox(&mut false, "Expression View");
    });
}

pub fn preference_menu(ui: &mut egui::Ui) {
    ui.menu_button("Preferences", |ui| {
        if ui.button("Settings").clicked() {
            println!("hi");
        }
    });
}

pub fn update(ctx: &egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            let full_width = ui.available_width();
            let body_h = ui.text_style_height(&egui::TextStyle::Body);
            let left_w = body_h * 16.0;
            let right_w = body_h * 10.0;

            StripBuilder::new(ui)
                .size(Size::exact(left_w))
                .size(Size::remainder())
                .size(Size::exact(right_w))
                .horizontal(|mut strip| {
                    // Left
                    strip.cell(|ui| {
                        ui.horizontal(|ui| {
                            file_menu(ui);
                            window_menu(ui);
                            preference_menu(ui);
                        });
                    });

                    // Center
                    strip.cell(|ui| {
                        let desired = 140.0;
                        let cur_x = ui.cursor().min.x;
                        let global_center = full_width * 0.5;
                        let left_pad = (global_center - desired * 0.5 - cur_x).max(0.0);

                        ui.add_space(left_pad);
                        ui.label("Middle");
                    });

                    // Right
                    strip.cell(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("v0.1.0");
                            ui.separator();
                            ui.label("Karny");
                        });
                    });
                });
        });
    });
}
