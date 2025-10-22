pub mod add_variable;

use crate::{
    app::AppState,
    ui::events::EventQueue
};

pub enum ModalKind {
    AddVariable
}

#[derive(Debug, Default)]
pub struct ModalState {
    pub add_variable: add_variable::AddVariableState,
}

pub fn update(ctx: &egui::Context, app_state: &mut AppState) {
    if app_state.modals.add_variable.show {
        add_variable::update(ctx, &mut app_state.modals.add_variable, &mut app_state.events);
    }
}
