use egui::{Align, Frame, Layout, Margin, TextStyle, Vec2, vec2};

use super::TableViewState;
use crate::{
    logic::variable::{VariableKind, VariableStore},
    ui::components::label::{self, rotated_label},
};

// TODO: Rename variables from header
pub fn render(ui: &mut egui::Ui, variables: &mut VariableStore, state: &mut TableViewState) {
    let mut stroke = ui.style_mut().visuals.widgets.noninteractive.bg_stroke;
    let mut sep_xs: Vec<(f32, f32)> = Vec::new();

    let mut sep_w = stroke.width.max(1.0);

    let frame_out = Frame::new().inner_margin(Margin::same(8)).show(ui, |ui| {
        ui.allocate_ui_with_layout(Vec2::ZERO, Layout::left_to_right(Align::Min), |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            let total = variables.iter().count();

            let mut iter = variables.iter_mut().peekable();
            let mut i = 0;

            while let Some(var) = iter.next() {
                if let Some(next) = iter.peek() {
                    if var.kind == VariableKind::Input && next.kind == VariableKind::Output {
                        sep_w = 4.0;
                    }
                }

                let resp = if var.name.len() > 1 && (state.vertical_names) {
                    rotated_label(
                        ui,
                        &var.name,
                        TextStyle::Body,
                        match state.clockwise {
                            true => label::Rotation::Clockwise,
                            false => label::Rotation::CounterClockwise,
                        },
                        None,
                    )
                } else {
                    ui.label(&var.name)
                };

                let w = resp.rect.width();
                if state.col_widths.len() <= i {
                    state.col_widths.push(w);
                } else {
                    state.col_widths[i] = w;
                }

                if i < total - 1 {
                    let (_, r) = ui.allocate_space(vec2(sep_w, 0.0));
                    sep_xs.push((r.center().x, sep_w));
                }

                i += 1;
                sep_w = stroke.width.max(1.0);
            }
        });
    });

    let outer_rect = frame_out.response.rect;
    let painter = ui.painter();
    let y_range = outer_rect.y_range();

    for x in sep_xs {
        stroke.width = x.1;
        painter.vline(x.0, y_range, stroke);
    }

    painter.hline(
        outer_rect.x_range(),
        outer_rect.bottom(),
        ui.style().visuals.widgets.noninteractive.bg_stroke,
    );
}
