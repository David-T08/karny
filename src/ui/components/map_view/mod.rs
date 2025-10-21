use egui::Vec2;

pub fn render(ui: &mut egui::Ui) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        ui.label("Map View");
        ui.separator();
    });
}