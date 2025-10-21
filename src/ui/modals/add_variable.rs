use crate::{
    app::AppState,
    logic::variable::{BitValue, VariableKind},
    ui::components::{cycle_button::labeled_cycle_button, textfield::labeled_textfield},
};

#[derive(Clone, Debug, Default)]
pub struct AddVariableState {
    pub show: bool,

    pub name: String,
    pub kind: VariableKind,
    pub state: BitValue,
}

pub fn update(ctx: &egui::Context, app_state: &mut AppState) {
    let mut to_finalize: Option<(String, VariableKind, BitValue)> = None;

    egui::Modal::new(egui::Id::new("add_variable")).show(ctx, |ui| {
        let modal_state = &mut app_state.modals.add_variable;

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
                to_finalize = Some((
                    modal_state.name.clone(),
                    modal_state.kind.clone(),
                    modal_state.state,
                ));
            }
        });
    });

    if let Some((name, kind, state)) = to_finalize {
        app_state.add_variable(&name, kind, state);
        app_state.modals.add_variable = AddVariableState::default();
    }
}
