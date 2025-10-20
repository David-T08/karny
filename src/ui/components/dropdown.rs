use std::fmt::{Debug, Display};

pub fn labeled_dropdown<T>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    options: &[T],
) -> egui::Response
where
    T: PartialEq + Clone + Debug,
{
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(label)
            .selected_text(format!("{:?}", value))
            .show_ui(ui, |ui| {
                for opt in options {
                    ui.selectable_value(value, opt.clone(), format!("{:?}", opt));
                }
            })
            .response
    })
    .inner
}
