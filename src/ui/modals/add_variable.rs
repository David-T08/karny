use crate::{
    app::AppState,
    logic::variable::{BitValue, VariableKind},
    ui::components::{
        textfield::labeled_textfield,
        dropdown::labeled_dropdown,
        cycle_button::labeled_cycle_button
    },
};

#[derive(Clone, Debug, Default)]
pub struct AddVariableState {
    pub show: bool,

    pub name: String,
    pub kind: VariableKind,
    pub state: BitValue,
}

pub fn update(ctx: &egui::Context, app_state: &mut AppState) {
    let modal_state = &mut app_state.modal_states.add_variable;

    egui::Modal::new(egui::Id::new("add_variable")).show(ctx, |ui| {
        ui.set_max_size(egui::Vec2 { x: 240.0, y: 100.0 });

        ui.horizontal(|ui| {
            ui.heading("Add New Variable");
            ui.add_space(ui.available_width());
            if ui.button("×").clicked() {
                modal_state.show = false;
            }
        });

        ui.separator();

        ui.vertical(|ui| {
            ui.set_min_width(200.0);
            labeled_textfield(ui, "Name:", &mut modal_state.name, ui.available_width());
            labeled_cycle_button(ui, "Kind:", &mut modal_state.kind, &[
                VariableKind::Input,
                VariableKind::Output
            ]);
            labeled_cycle_button(ui, "Value:", &mut modal_state.state, &[
                BitValue::DontCare,
                BitValue::Zero,
                BitValue::One
            ]);
            
            ui.separator();
        });
        
        ui.centered_and_justified(|ui| {
            ui.add_sized([ui.available_width(), 24.0], egui::Button::new("Finalize"));
        })
    });
}
