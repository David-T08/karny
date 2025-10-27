use std::{
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub enum BitValue {
    Zero,
    One,
    #[default]
    DontCare,
}

impl BitValue {
    pub fn toggle(&mut self) {
        *self = match self {
            BitValue::Zero => BitValue::One,
            BitValue::One => BitValue::DontCare,
            BitValue::DontCare => BitValue::Zero,
        };
    }

    pub fn set(&mut self, value: BitValue) {
        *self = value;
    }

    pub fn as_u8(self) -> Option<u8> {
        match self {
            BitValue::Zero => Some(0),
            BitValue::One => Some(1),
            BitValue::DontCare => None,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => BitValue::Zero,
            1 => BitValue::One,
            _ => BitValue::DontCare,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            BitValue::Zero => '0',
            BitValue::One => '1',
            BitValue::DontCare => 'x',
        }
    }
}

impl From<u8> for BitValue {
    fn from(value: u8) -> Self {
        BitValue::from_u8(value)
    }
}

impl fmt::Display for BitValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Hash, Eq)]
pub enum VariableKind {
    #[default]
    Input,
    Output,
}

impl fmt::Display for VariableKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VariableKind::Input => "Input",
                VariableKind::Output => "Output",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariableId(u64);

#[derive(Debug, Default, Clone)]
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

    pub fn add(&mut self, name: String, kind: VariableKind) {
        let id = self.next_variable_id();
        let vec = self.get_corresponding_vec_mut(kind);

        vec.push(Variable {
            id,
            name: name,
            kind,
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

    pub fn iter(&self) -> VariableStoreIter {
        VariableStoreIter {
            inputs: self.inputs.iter(),
            outputs: self.outputs.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> VariableStoreIterMut {
        VariableStoreIterMut {
            inputs: self.inputs.iter_mut(),
            outputs: self.outputs.iter_mut(),
        }
    }
}

pub struct VariableStoreIter<'a> {
    inputs: std::slice::Iter<'a, Variable>,
    outputs: std::slice::Iter<'a, Variable>,
}

impl<'a> Iterator for VariableStoreIter<'a> {
    type Item = &'a Variable;

    fn next(&mut self) -> Option<Self::Item> {
        self.inputs.next().or_else(|| self.outputs.next())
    }
}

pub struct VariableStoreIterMut<'a> {
    inputs: std::slice::IterMut<'a, Variable>,
    outputs: std::slice::IterMut<'a, Variable>,
}

impl<'a> Iterator for VariableStoreIterMut<'a> {
    type Item = &'a mut Variable;

    fn next(&mut self) -> Option<Self::Item> {
        self.inputs.next().or_else(|| self.outputs.next())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub kind: VariableKind,
    pub id: VariableId,
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
