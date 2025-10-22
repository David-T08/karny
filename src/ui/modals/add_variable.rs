use crate::{
    logic::variable::{BitValue, VariableKind},
    ui::{
        components::{cycle_button::labeled_cycle_button, textfield::labeled_textfield},
        events::{EventQueue, VariableEvent},
    },
};

#[derive(Clone, Debug, Default)]
pub struct AddVariableState {
    pub show: bool,

    pub name: String,
    pub kind: VariableKind,
    pub state: BitValue,
}

pub fn update(ctx: &egui::Context, modal_state: &mut AddVariableState, events: &mut EventQueue) {
    egui::Modal::new(egui::Id::new("add_variable")).show(ctx, |ui| {
        ui.set_max_size(egui::vec2(240.0, 80.0));

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

            labeled_cycle_button(
                ui,
                "Kind:",
                &mut modal_state.kind,
                &[VariableKind::Input, VariableKind::Output],
            );

            ui.separator();
        });

        ui.centered_and_justified(|ui| {
            if ui.button("Finalize").clicked() {
                events.push_variable(VariableEvent::Add { 
                    name: modal_state.name.clone(), 
                    kind: modal_state.kind, 
                    value: modal_state.state 
                });
                
                *modal_state = AddVariableState::default();
            }
        });
    });
}
