use crate::{app::AppState};

pub use crate::ui::variable::VariableEvent;

fn handle_variable_event(state: &mut AppState, ev: VariableEvent) {
    match ev {
        VariableEvent::Add { name, kind, value } => state.variables.add(name, kind, value),
        VariableEvent::Remove(id) => state.variables.remove(id),
        VariableEvent::Rename(id, new_name) => state.variables.rename(id, new_name),
    }
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