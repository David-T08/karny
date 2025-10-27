use egui::epaint::TextShape;
use egui::{Color32, FontId, Sense, Stroke, TextStyle, Ui, Vec2, vec2};
use std::f32::consts::FRAC_PI_2;

#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

pub fn rotated_label(
    ui: &mut Ui,
    text: &str,
    style: TextStyle,
    rot: Rotation,
    color: Option<Color32>,
) -> egui::Response {
    let effective_style = ui
        .style()
        .override_text_style
        .as_ref()
        .cloned()
        .unwrap_or(style);

    let font_id: FontId = effective_style.resolve(ui.style());
    let color = color.unwrap_or(ui.visuals().text_color());

    let galley = ui.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), font_id.clone(), color));
    let size = galley.size();

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(size.y, size.x), Sense::hover());

    let mut pos = match rot {
        Rotation::Clockwise => rect.left_top() + vec2(0.0, size.x),
        Rotation::CounterClockwise => rect.left_top() + vec2(size.y, 0.0),
    };
    pos = pos.round();

    ui.painter().with_clip_rect(rect).add(TextShape {
        pos,
        galley,
        underline: Stroke::NONE,
        override_text_color: Some(color),
        angle: match rot {
            Rotation::Clockwise => -FRAC_PI_2,
            Rotation::CounterClockwise => FRAC_PI_2,
        },
        fallback_color: color,
        opacity_factor: 1.0,
    });

    resp
}
