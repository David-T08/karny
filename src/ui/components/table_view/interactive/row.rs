use egui::{Align, Frame, Layout, Margin, Vec2, vec2};

use super::TableViewState;
use crate::logic::truth_table::TruthRow;

pub fn render(ui: &mut egui::Ui, row: &mut TruthRow, state: &TableViewState, draw_sep: bool) {
    let stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
    let sep_w = stroke.width.max(1.0);
    let mut sep_xs: Vec<f32> = Vec::new();

    let frame_out = Frame::new().inner_margin(Margin::same(8)).show(ui, |ui| {
        ui.allocate_ui_with_layout(Vec2::ZERO, Layout::left_to_right(Align::Min), |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            let mut _separated = false;
            let total = state.col_widths.len();

            for (i, width) in state.col_widths.iter().enumerate() {
                let text = row.get(i).map(|c| c.to_char()).unwrap_or(' ');
                let row_h = ui.text_style_height(&egui::TextStyle::Monospace);

                ui.add_sized(
                    egui::vec2(*width, row_h),
                    egui::Label::new(text.to_string()),
                );

                if i < total - 1 {
                    let (_, r) = ui.allocate_space(vec2(sep_w, 0.0));
                    sep_xs.push(r.center().x)
                }
            }
        });
    });

    let outer_rect = frame_out.response.rect;
    let painter = ui.painter();
    let y_range = outer_rect.y_range();

    for x in sep_xs {
        painter.vline(x, y_range, stroke);
    }
    
    if draw_sep {
        painter.hline(outer_rect.x_range(), outer_rect.bottom(), ui.style().visuals.widgets.noninteractive.bg_stroke);
    }
}
