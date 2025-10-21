use eframe::egui;
use std::hash::{Hash, Hasher};

use crate::{
    logic::variable::{BitValue, VariableKind},
    ui::{self, components::variable_view, modals},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariableId(u64);

#[derive(Default)]
pub struct AppState {
    pub modals: modals::ModalState,
    pub variables: Variables,

    next_variable_id: u64,
}

impl AppState {
    pub fn next_variable_id(&mut self) -> VariableId {
        let id = VariableId(self.next_variable_id);
        self.next_variable_id += 1;
        id
    }

    pub fn add_variable(&mut self, name: &str, kind: VariableKind, value: BitValue) {
        let id = self.next_variable_id();
        let vec = match kind {
            VariableKind::Input => &mut self.variables.inputs,
            VariableKind::Output => &mut self.variables.outputs,
        };

        vec.push(Variable {
            id,
            name: name.to_owned(),
            kind,
            value,
        });
    }
}

#[derive(Debug, Default)]
pub struct Variables {
    pub inputs: Vec<Variable>,
    pub outputs: Vec<Variable>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: BitValue,
    pub kind: VariableKind,
    pub id: VariableId,
}

// Hash by id ONLY
impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn app() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::topbar::update(ctx);

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .width_range(180.0..=720.0)
            .default_width(280.0)
            .show(ctx, |ui| {
                variable_view::render(ui, self);
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Karnaugh Map");
            ui.label("Main content goes here");
        });

        modals::update(ctx, self);
    }
}
