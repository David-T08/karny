use egui::{Align, Frame, Layout, Margin, Vec2, vec2, RichText, Color32};

use super::TableViewState;
use crate::{
    logic::{
        truth_table::TruthRow,
        variable::{BitValue, VariableKind},
    },
    ui::components::cycle_button::cycle_button,
};

fn bit_label(v: &BitValue) -> RichText {
    let rich = RichText::new(v.to_string()).monospace();
    match v {
        BitValue::Zero     => rich.strong(),
        BitValue::One      => rich.strong(),
        BitValue::DontCare => rich,
    }
}

fn bit_color(ctx: &egui::Context, v: &BitValue) -> Color32 {
    let dark = ctx.style().visuals.dark_mode;
    
    match (v, dark) {
            (BitValue::Zero, true)      => Color32::from_rgb(45, 45, 50),
            (BitValue::Zero, false)     => Color32::from_rgb(220, 220, 230),
    
            (BitValue::One, true)       => Color32::from_hex("#27926e90").unwrap(),
            (BitValue::One, false)      => Color32::from_rgb(0, 120, 80),
    
            (BitValue::DontCare, true)  => Color32::from_hex("#50415e").unwrap(),
            (BitValue::DontCare, false) => Color32::from_rgb(200, 200, 210),
        }
}

fn alternating_fill(ui: &egui::Ui, row_idx: usize) -> Color32 {
    let v = &ui.style().visuals;
    
    let a = v.panel_fill;
    let b = v.faint_bg_color;
    if row_idx % 2 == 0 { a } else { b }
}

pub fn render(ui: &mut egui::Ui, row: &mut TruthRow, state: &TableViewState, index: usize, draw_sep: bool) {
    let mut stroke = ui.style_mut().visuals.widgets.noninteractive.bg_stroke;
    let mut sep_w = stroke.width.max(1.0);
    let mut sep_xs: Vec<(f32, f32)> = Vec::new();

    let frame_out = Frame::new().fill(alternating_fill(ui, index)).inner_margin(Margin::same(8)).show(ui, |ui| {
        ui.allocate_ui_with_layout(Vec2::ZERO, Layout::left_to_right(Align::Min), |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            let mut _separated = false;
            let total = state.col_widths.len();

            for (i, width) in state.col_widths.iter().enumerate() {
                let (text, kind) = row.get_mut(i).unwrap();
                let row_h = ui.text_style_height(&egui::TextStyle::Monospace);

                match kind {
                    VariableKind::Input => ui.add_sized(
                        egui::vec2(*width, row_h),
                        egui::Label::new(text.to_string()),
                    ),
                    VariableKind::Output => cycle_button(
                        ui,
                        egui::vec2(*width, row_h),
                        &mut *text,
                        &[BitValue::Zero, BitValue::One, BitValue::DontCare],
                        bit_label,
                        bit_color
                    ),
                };

                if i < total - 1 {
                    if let Some((_, next_kind)) = row.get(i + 1) {
                        if kind == VariableKind::Input && next_kind == VariableKind::Output {
                            sep_w = 4.0;
                        }
                    }

                    let (_, r) = ui.allocate_space(vec2(sep_w, 0.0));
                    sep_xs.push((r.center().x, sep_w))
                }

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

    if draw_sep {
        painter.hline(
            outer_rect.x_range(),
            outer_rect.bottom(),
            ui.style().visuals.widgets.noninteractive.bg_stroke,
        );
    }
}
