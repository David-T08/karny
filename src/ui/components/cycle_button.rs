use std::fmt::Debug;

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
