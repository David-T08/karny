use crate::app::AppState;

mod list;
mod row;

pub fn render(ui: &mut egui::Ui, app_state: &mut AppState) {
    ui.collapsing("Variables", |ui| {
        if ui.button("Add new variable").clicked() {
            app_state.modals.add_variable.show = true;
        }

        ui.collapsing("Inputs", |ui| {
            list::render(ui, &mut app_state.variables.inputs, "inputs");
        });
        
        ui.collapsing("Outputs", |ui| {
            list::render(ui, &mut app_state.variables.outputs, "outputs");
        });
    });
}
