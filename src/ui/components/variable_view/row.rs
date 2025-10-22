use crate::ui::variable::Variable;
use egui::{Margin, Stroke, TextEdit};
use egui_dnd::Handle;

pub fn render(ui: &mut egui::Ui, variable: &mut Variable, handle: Handle, index: String) {
    ui.push_id(variable.id, |ui| {
        egui::Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .stroke(Stroke::new(
                1.0,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .corner_radius(2.0) // rounded corners
            .inner_margin(Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.take_available_space();

                    handle.ui(ui, |ui| {
                        ui.label(index);
                    });

                    ui.separator();
                    ui.add(
                        TextEdit::singleline(&mut variable.name)
                            .char_limit(16)
                            .desired_width(ui.available_width() - 40.0)
                            .id_source("name"),
                    );

                    ui.add_space(4.0);

                    let resp = ui.button("X");
                    if resp.clicked() {
                        variable.value.toggle();
                    }

                    resp
                });
            });
    });
}
