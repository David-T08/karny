pub mod add_variable;

use crate::app::AppState;

#[derive(Debug, Default)]
pub struct ModalState {
    pub add_variable: add_variable::AddVariableState,
}

pub fn update(ctx: &egui::Context, app_state: &mut AppState) {
    if app_state.modal_states.add_variable.show {
        add_variable::update(ctx, app_state);
    }
}
