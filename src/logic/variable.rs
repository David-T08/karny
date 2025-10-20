use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitValue {
    Zero,
    One,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    text: String,
    vertical: bool,
    clockwise: bool,
}

impl Label {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            vertical: false,
            clockwise: true,
        }
    }

    /// Make the label vertical.
    /// `clockwise = true` → top-to-bottom
    /// `clockwise = false` → bottom-to-top
    pub fn vertical(mut self, clockwise: bool) -> Self {
        self.vertical = true;
        self.clockwise = clockwise;
        self
    }
}

impl From<String> for Label {
    fn from(s: String) -> Self {
        Label::new(s)
    }
}
impl From<&str> for Label {
    fn from(s: &str) -> Self {
        Label::new(s)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.vertical {
            return f.write_str(&self.text);
        }

        let iter = self.text.chars();
        if !self.clockwise {
            let v: Vec<char> = iter.collect();
            for (i, ch) in v.iter().rev().enumerate() {
                if i > 0 {
                    f.write_str("\n")?;
                }
                write!(f, "{}", ch)?;
            }
            return Ok(());
        }

        for (i, ch) in iter.enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    value: BitValue,
    name: Label,
}

impl Variable {
    pub fn new<T: Into<Label>>(name: T) -> Self {
        Self {
            name: name.into(),
            value: BitValue::Zero,
        }
    }

    pub fn toggle(&mut self) {
        self.value.toggle();
    }

    pub fn as_char(&self) -> char {
        self.value.to_char()
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
