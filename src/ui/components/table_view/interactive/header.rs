use egui::{Align, Frame, Layout, Margin, TextStyle, Vec2, vec2};

use super::TableViewState;
use crate::{
    logic::variable::{VariableKind, VariableStore},
    ui::components::label::{self, rotated_label},
};

// TODO: Rename variables from header
pub fn render(ui: &mut egui::Ui, variables: &mut VariableStore, state: &TableViewState) {
    let stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
    let sep_w = stroke.width.max(1.0);
    let mut sep_xs: Vec<f32> = Vec::new();

    let frame_out = Frame::new().inner_margin(Margin::same(8)).show(ui, |ui| {
        ui.allocate_ui_with_layout(Vec2::ZERO, Layout::left_to_right(Align::Min), |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            let mut separated = false;
            let total = variables.iter().count();

            variables.iter_mut().enumerate().for_each(|(i, var)| {
                if !separated && var.kind == VariableKind::Output {
                    // TODO: Better separator for I/O switch
                    separated = true;
                }

                if var.name.len() > 1 && (state.vertical_names) {
                    rotated_label(
                        ui,
                        &var.name,
                        TextStyle::Body,
                        match state.clockwise {
                            true => label::Rotation::Clockwise,
                            false => label::Rotation::CounterClockwise,
                        },
                        None,
                    );
                } else {
                    ui.label(&var.name);
                }

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
