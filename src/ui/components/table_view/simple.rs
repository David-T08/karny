use crate::logic::truth_table::TruthTable;

pub fn render(ui: &mut egui::Ui, _table: &TruthTable) {
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
