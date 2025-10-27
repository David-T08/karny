// use crate::ui::variable::VariableStore;

pub fn render(ui: &mut egui::Ui) {
    egui::TopBottomPanel::bottom("table_bottom")
        .resizable(true)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label("row");
                    ui.label("row2");
                    ui.label("row3");
                });
        });
}
