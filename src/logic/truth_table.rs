use crate::logic::variable::{BitValue, VariableKind, VariableStore};

#[derive(Debug, Default)]
pub struct TruthTable {
    pub variables: VariableStore,
    pub rows: Vec<TruthRow>,
}

#[derive(Clone, Debug)]
pub struct TruthRow {
    pub inputs: Vec<BitValue>,
    pub outputs: Vec<BitValue>,
}

impl TruthRow {
    /// Returns Some((&BitValue, kind)) if `index` is in-bounds, otherwise None.
    pub fn get(&self, index: usize) -> Option<(&BitValue, VariableKind)> {
        let input_len = self.inputs.len();
        if index < input_len {
            self.inputs.get(index).map(|v| (v, VariableKind::Input))
        } else {
            self.outputs
                .get(index - input_len)
                .map(|v| (v, VariableKind::Output))
        }
    }
}

impl TruthTable {
    pub fn new(variables: VariableStore) -> Self {
        let num_rows = 1 << variables.inputs.len();
        let rows = (0..num_rows)
            .map(|i| {
                let bits = (0..variables.inputs.len())
                    .map(|b| {
                        if (i >> (variables.inputs.len() - 1 - b)) & 1 == 1 {
                            BitValue::One
                        } else {
                            BitValue::Zero
                        }
                    })
                    .collect();
                TruthRow {
                    inputs: bits,
                    outputs: vec![BitValue::DontCare; variables.outputs.len()],
                }
            })
            .collect();

        Self {
            variables: variables,
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

    /// Returns Some((&BitValue, kind)) if row and index are in-bounds; otherwise None.
    pub fn get(&self, row: usize, index: usize) -> Option<(&BitValue, VariableKind)> {
        self.rows.get(row).and_then(|r| r.get(index))
    }
}
