pub fn labeled_textfield(
    ui: &mut egui::Ui,
    label: &str,
    text: &mut String,
    entry_width: f32,
) -> egui::Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::TextEdit::singleline(text).desired_width(entry_width))
    })
    .response
}
