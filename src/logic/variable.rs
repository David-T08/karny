use std::fmt;

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
