use egui::{Direction, Frame, Layout, Margin};

pub fn render(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(Frame::default().inner_margin(Margin::same(6)))
        .show_inside(ui, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.label("test");
                },
            );
        });
}
