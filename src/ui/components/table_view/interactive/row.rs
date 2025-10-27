use egui::{Align, Frame, Layout, Margin, Vec2, vec2};

use super::TableViewState;
use crate::logic::{
    truth_table::TruthRow,
    variable::{VariableKind, VariableStore},
};

pub fn render(
    ui: &mut egui::Ui,
    _index: usize,
    variables: &VariableStore,
    row: &mut TruthRow,
    _state: &TableViewState,
) {
    let stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
    let sep_w = stroke.width.max(1.0);
    let mut sep_xs: Vec<f32> = Vec::new();

    let frame_out = Frame::new().inner_margin(Margin::same(8)).show(ui, |ui| {
        ui.allocate_ui_with_layout(Vec2::ZERO, Layout::left_to_right(Align::Min), |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            let mut separated = false;
            let total = variables.iter().count();

            variables.iter().enumerate().for_each(|(i, var)| {
                if !separated && var.kind == VariableKind::Output {
                    // TODO: Better separator for I/O switch
                    separated = true;
                }

                let cell = row.get(i);
                ui.label(cell.unwrap().to_char().to_string());

                if i < total - 1 {
                    let (_, r) = ui.allocate_space(vec2(sep_w, 0.0));
                    sep_xs.push(r.center().x);
                }
            });
        });
    });

    let outer_rect = frame_out.response.rect;
    let painter = ui.painter();
    let y_range = outer_rect.y_range();

    for x in sep_xs {
        painter.vline(x, y_range, stroke);
    }
}
