use crate::logic::truth_table::TruthTable;
use egui::{Frame, Margin, Vec2};

mod interactive;
mod simple;

pub fn render(ui: &mut egui::Ui, table: &mut TruthTable) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;

    ui.add_space(8.0);
    ui.vertical_centered(|ui| {
        ui.label("Table View");
    });
    ui.separator();

    Frame::default().inner_margin(Margin::ZERO).show(ui, |ui| {
        ui.set_min_size(Vec2::new(ui.available_width(), ui.available_height()));

        simple::render(ui, table);
        interactive::render(ui);
    });
}
