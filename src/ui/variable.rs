use crate::logic::variable::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariableId(u64);

#[derive(Debug, Default)]
pub struct VariableStore {
    next_variable_id: u64,

    pub inputs: Vec<Variable>,
    pub outputs: Vec<Variable>,
}

impl VariableStore {
    pub fn next_variable_id(&mut self) -> VariableId {
        let id = VariableId(self.next_variable_id);
        self.next_variable_id += 1;
        id
    }

    pub fn add(&mut self, name: String, kind: VariableKind, value: BitValue) {
        let id = self.next_variable_id();
        let vec = self.get_corresponding_vec_mut(kind);

        vec.push(Variable {
            id,
            name: name,
            kind,
            value,
        });
    }

    pub fn remove(&mut self, id: VariableId) {
        if let Some(var) = self.get_variable_by_id(id) {
            let vec = self.get_corresponding_vec_mut(var.kind);

            vec.retain(|v| v.id != id);
        }
    }

    pub fn rename(&mut self, id: VariableId, new_name: String) {
        if let Some(var) = self.get_variable_by_id_mut(id) {
            var.name = new_name;
        }
    }

    pub fn get_variable_by_id(&self, id: VariableId) -> Option<&Variable> {
        self.inputs
            .iter()
            .chain(self.outputs.iter())
            .find(|v| v.id == id)
    }

    pub fn get_variable_by_id_mut(&mut self, id: VariableId) -> Option<&mut Variable> {
        self.inputs
            .iter_mut()
            .chain(self.outputs.iter_mut())
            .find(|v| v.id == id)
    }

    fn get_corresponding_vec(&self, kind: VariableKind) -> &Vec<Variable> {
        match kind {
            VariableKind::Input => &self.inputs,
            VariableKind::Output => &self.outputs,
        }
    }

    fn get_corresponding_vec_mut(&mut self, kind: VariableKind) -> &mut Vec<Variable> {
        match kind {
            VariableKind::Input => &mut self.inputs,
            VariableKind::Output => &mut self.outputs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: BitValue,
    pub kind: VariableKind,
    pub id: VariableId,
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub enum VariableEvent {
    Add {
        name: String,
        kind: VariableKind,
        value: BitValue,
    },

    Remove(VariableId),
    Rename(VariableId, String),
}