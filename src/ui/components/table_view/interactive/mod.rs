use egui::Vec2;

use super::TableViewState;
use crate::logic::{truth_table::TruthTable, variable::VarStoreHandle};

mod cell;
mod header;
mod row;

pub fn render(
    ui: &mut egui::Ui,
    table: &mut TruthTable,
    variables: VarStoreHandle,
    state: &mut TableViewState,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::default().inner_margin(egui::Margin::same(12)))
        .show_inside(ui, |ui| {
            let rect = ui.max_rect();

            egui::Area::new("table_area".into())
                .constrain_to(rect)
                .pivot(egui::Align2::CENTER_CENTER)
                .fixed_pos(rect.center())
                .interactable(false)
                .movable(false)
                .show(ui.ctx(), |ui| {
                    ui.allocate_ui(egui::Vec2::ZERO, |ui| {
                        egui::Frame::new()
                            .fill(ui.visuals().faint_bg_color)
                            .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
                            .show(ui, |ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                                    header::render(ui, variables, state);

                                    let count = table.rows.len();
                                    for (index, row) in table.rows.iter_mut().enumerate() {
                                        row::render(ui, row, state, index, index != count - 1);
                                    }
                                });
                            });
                    });
                });
        });
}
