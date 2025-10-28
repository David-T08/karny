use egui::{Button, Color32, CornerRadius, CursorIcon, Key, RichText, Sense, Stroke, Ui, Vec2};

pub fn labeled_cycle_button<T>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    options: &[T],
) -> egui::Response
where
    T: PartialEq + Clone + std::fmt::Display,
{
    ui.horizontal(|ui| {
        ui.label(label);

        let current_index = options.iter().position(|v| v == value).unwrap_or(0);
        let next_index = (current_index + 1) % options.len();

        let resp = ui.button(format!("{value}"));

        if resp.clicked() {
            *value = options[next_index].clone();
        }

        resp
    })
    .inner
}

pub fn cycle_button<T, F, G>(
    ui: &mut Ui,
    size: Vec2,
    value: &mut T,
    options: &[T],
    to_label: F,
    to_color: G,
) -> egui::Response
where
    T: PartialEq + Clone,
    F: Fn(&T) -> RichText,
    G: Fn(&egui::Context, &T) -> Color32,
{
    let cur_idx = options.iter().position(|v| v == value).unwrap_or(0);
    let next_idx = (cur_idx + 1) % options.len();

    let base = to_color(ui.ctx(), value);

    let rounding = CornerRadius::same(6);
    let stroke = Stroke::new(1.0, Color32::from_gray(120));

    let btn = Button::new(to_label(value))
        .min_size(size)
        .corner_radius(rounding)
        .stroke(stroke)
        .fill(base);

    let resp = ui
        .add(btn.sense(Sense::click()))
        .on_hover_cursor(CursorIcon::PointingHand);

    if resp.clicked() {
        *value = options[next_idx].clone();
        ui.memory_mut(|m| m.request_focus(resp.id));
    }

    if resp.has_focus() && (ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Space))) {
        *value = options[next_idx].clone();
    }

    if resp.hovered() || resp.is_pointer_button_down_on() {
        let overlay = if resp.is_pointer_button_down_on() {
            Color32::from_black_alpha(80)
        } else {
            Color32::from_white_alpha(18)
        };
        ui.painter().rect(
            resp.rect,
            rounding,
            overlay,
            Stroke::NONE,
            egui::StrokeKind::Inside,
        );
    }

    resp
}
