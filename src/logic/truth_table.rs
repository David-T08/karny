use std::fmt;

use crate::logic::variable::BitValue;

#[derive(Clone, Debug)]
pub struct TruthTable {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,

    pub rows: Vec<TruthRow>,
}

#[derive(Clone, Debug)]
pub struct TruthRow {
    pub inputs: Vec<BitValue>,
    pub outputs: Vec<BitValue>,
}

impl TruthTable {
    pub fn new<I, O>(inputs: &[I], outputs: &[O]) -> Self
    where
        I: Clone + Into<String>,
        O: Clone + Into<String>,
    {
        let inputs: Vec<String> = inputs.iter().cloned().map(Into::into).collect();
        let outputs: Vec<String> = outputs.iter().cloned().map(Into::into).collect();

        let num_rows = 1 << inputs.len();
        let rows = (0..num_rows)
            .map(|i| {
                let bits = (0..inputs.len())
                    .map(|b| {
                        if (i >> (inputs.len() - 1 - b)) & 1 == 1 {
                            BitValue::One
                        } else {
                            BitValue::Zero
                        }
                    })
                    .collect();
                TruthRow {
                    inputs: bits,
                    outputs: vec![BitValue::DontCare; outputs.len()],
                }
            })
            .collect();

        Self {
            inputs,
            outputs,
            rows,
        }
    }

    fn get_output_cell(&mut self, row: usize, output_index: usize) -> Option<&mut BitValue> {
        self.rows
            .get_mut(row)
            .and_then(|r| r.outputs.get_mut(output_index))
    }

    pub fn toggle(&mut self, row: usize, output_index: usize) {
        if let Some(cell) = self.get_output_cell(row, output_index) {
            cell.toggle();
        }
    }

    pub fn set(&mut self, row: usize, output_index: usize, value: BitValue) {
        if let Some(cell) = self.get_output_cell(row, output_index) {
            cell.set(value);
        }
    }
}

impl fmt::Display for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut col_widths: Vec<usize> = self
            .inputs
            .iter()
            .chain(self.outputs.iter())
            .map(|name| name.len().max(1)) // at least 1 wide
            .collect();

        for row in &self.rows {
            for (i, val) in row.inputs.iter().chain(row.outputs.iter()).enumerate() {
                col_widths[i] = col_widths[i].max(val.to_string().len());
            }
        }

        for (i, name) in self.inputs.iter().enumerate() {
            write!(f, "{:>width$}", name, width = col_widths[i])?;
            if i + 1 < self.inputs.len() {
                write!(f, " | ")?;
            }
        }

        if !self.outputs.is_empty() {
            write!(f, " || ")?;
            for (j, name) in self.outputs.iter().enumerate() {
                let idx = self.inputs.len() + j;
                write!(f, "{:>width$}", name, width = col_widths[idx])?;
                if j + 1 < self.outputs.len() {
                    write!(f, " | ")?;
                }
            }
        }
        writeln!(f)?;

        let mut total_width = 0;
        for (i, w) in col_widths.iter().enumerate() {
            total_width += *w;
            if i + 1 < col_widths.len() {
                total_width += 3; // spacing for separators
            }
        }
        writeln!(f, "{}", "-".repeat(total_width))?;

        for row in &self.rows {
            for (i, val) in row.inputs.iter().enumerate() {
                write!(f, "{:>width$}", val, width = col_widths[i])?;
                if i + 1 < self.inputs.len() {
                    write!(f, " | ")?;
                }
            }

            if !self.outputs.is_empty() {
                write!(f, " || ")?;
                for (j, val) in row.outputs.iter().enumerate() {
                    let idx = self.inputs.len() + j;
                    write!(f, "{:>width$}", val, width = col_widths[idx])?;
                    if j + 1 < self.outputs.len() {
                        write!(f, " | ")?;
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
