use crate::{app::AppState, logic::variable::*};

pub enum VariableEvent {
    Add {
        name: String,
        kind: VariableKind,
        // value: BitValue,
    },

    Remove(VariableId),
}

fn handle_variable_event(state: &mut AppState, ev: VariableEvent) {
    match ev {
        VariableEvent::Add { name, kind } => {
            let mut store = state.variables.borrow_mut();

            store.add(name, kind);
            state.table.add_column(kind);
        }
        VariableEvent::Remove(id) => {
            let (kind, flat_idx, input_idx_opt) = {
                let store = state.variables.borrow();

                if let Some(i) = store.inputs.iter().position(|v| v.id == id) {
                    (VariableKind::Input, i, Some(i))
                } else if let Some(j) = store.outputs.iter().position(|v| v.id == id) {
                    let flat = store.inputs.len() + j;
                    (VariableKind::Output, flat, None)
                } else {
                    return;
                }
            };

            {
                let mut store = state.variables.borrow_mut();
                store.remove(id);
            }
            match kind {
                VariableKind::Output => {
                    state.table.remove_column(flat_idx);
                }
                VariableKind::Input => {
                    state.table.remove_input_and_compact(input_idx_opt.unwrap());
                }
            }
        }
    }

    state.table.sync_output_order();
}

pub enum Event {
    Variable(VariableEvent),
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

impl EventQueue {
    pub fn push(&mut self, ev: Event) {
        self.events.push(ev);
    }

    pub fn push_variable(&mut self, ev: VariableEvent) {
        self.push(Event::Variable(ev));
    }

    pub fn take_all(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
}

pub fn dispatch_all<I>(state: &mut AppState, events: I)
where
    I: IntoIterator<Item = Event>,
{
    for ev in events {
        match ev {
            Event::Variable(ev) => handle_variable_event(state, ev),
        }
    }
}
