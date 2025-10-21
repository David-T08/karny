use egui_dnd::dnd;

use super::row;
use crate::app::Variable;

pub fn render(ui: &mut egui::Ui, variables: &mut Vec<Variable>, id: &'static str) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        dnd(ui, id).show_vec(variables, |ui, var, handle, item_state| {
            row::render(ui, var, handle, item_state.index.to_string());
        });
    });
}
