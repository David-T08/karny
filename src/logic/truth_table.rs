use crate::logic::variable::{BitValue, VarStoreHandle, VariableId, VariableKind};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TruthTable {
    pub variables: VarStoreHandle,
    pub rows: Vec<TruthRow>,

    cached_output_ids: Vec<VariableId>,
}

#[derive(Clone, Debug)]
pub struct TruthRow {
    pub inputs: Vec<BitValue>,
    pub outputs: Vec<BitValue>,
}

impl TruthRow {
    /// Returns `Some((&BitValue, kind))` if `index` is in-bounds, otherwise `None`.
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

    pub fn get_mut(&mut self, index: usize) -> Option<(&mut BitValue, VariableKind)> {
        let input_len = self.inputs.len();
        if index < input_len {
            self.inputs.get_mut(index).map(|v| (v, VariableKind::Input))
        } else {
            self.outputs
                .get_mut(index - input_len)
                .map(|v| (v, VariableKind::Output))
        }
    }
}

impl TruthTable {
    pub fn new(variables: VarStoreHandle) -> Self {
        let (in_len, out_len) = {
            let v = variables.borrow();
            (v.inputs.len(), v.outputs.len())
        };

        let num_rows = 1 << in_len;
        let rows = (0..num_rows)
            .map(|i| {
                let bits = (0..in_len)
                    .map(|b| {
                        if (i >> (in_len - 1 - b)) & 1 == 1 {
                            BitValue::One
                        } else {
                            BitValue::Zero
                        }
                    })
                    .collect();
                TruthRow {
                    inputs: bits,
                    outputs: vec![BitValue::Zero; out_len],
                }
            })
            .collect();

        Self {
            variables,
            rows,
            cached_output_ids: Vec::new(),
        }
    }

    /// Remove an input column and compact rows, preserving outputs from the branch
    /// where the removed input was 0. Keeps other inputs in order.
    pub fn remove_input_and_compact(&mut self, input_idx: usize) {
        if self.rows.is_empty() {
            return;
        }
        let old_in_len = self.rows[0].inputs.len();
        if input_idx >= old_in_len {
            return;
        }

        let old_rows = std::mem::take(&mut self.rows);
        let new_in_len = old_in_len - 1;
        let new_num_rows = 1 << new_in_len;

        let pos = old_in_len - 1 - input_idx;

        self.rows = (0..new_num_rows)
            .map(|new_i| {
                let low_mask = (1usize << pos) - 1;
                let low = new_i & low_mask;
                let high = new_i & !low_mask;
                let old_i = (high << 1) | low;

                let mut row = old_rows[old_i].clone();
                row.inputs.remove(input_idx);
                row.outputs.iter_mut().for_each(|v| {
                    *v = BitValue::Zero;
                });
                
                row
            })
            .collect();
    }

    /// Adds a new column to the end of each row
    pub fn add_column(&mut self, kind: VariableKind) {
        match kind {
            VariableKind::Output => {
                for row in &mut self.rows {
                    row.outputs.push(BitValue::Zero);
                }
            }

            VariableKind::Input => {
                let mut new_rows = Vec::with_capacity(self.rows.len() * 2);

                for r in &self.rows {
                    let out_len = r.outputs.len();

                    let mut inputs0 = r.inputs.clone();
                    inputs0.push(BitValue::Zero);
                    new_rows.push(TruthRow {
                        inputs: inputs0,
                        outputs: vec![BitValue::Zero; out_len],
                    });

                    let mut inputs1 = r.inputs.clone();
                    inputs1.push(BitValue::One);
                    new_rows.push(TruthRow {
                        inputs: inputs1,
                        outputs: vec![BitValue::Zero; out_len],
                    });
                }

                self.rows = new_rows;
            }
        }
    }

    /// Keeps output columns in the same order as their corresponding variables
    /// when the `VariableStore` is modified.
    pub fn sync_output_order(&mut self) {
        let new_ids: Vec<VariableId> = {
            let v = self.variables.borrow();
            v.outputs.iter().map(|o| o.id).collect()
        };
        let new_len = new_ids.len();

        for r in &mut self.rows {
            if r.outputs.len() != new_len {
                r.outputs.resize(new_len, BitValue::Zero);
            }
        }

        if self.cached_output_ids == new_ids {
            return;
        }

        let index_of_old: HashMap<VariableId, usize> = self
            .cached_output_ids
            .iter()
            .enumerate()
            .map(|(i, id)| (*id, i))
            .collect();

        for r in &mut self.rows {
            let old = r.outputs.clone();
            let mut new_outputs = Vec::with_capacity(new_len);
            for id in &new_ids {
                if let Some(&old_idx) = index_of_old.get(id) {
                    if old_idx < old.len() {
                        new_outputs.push(old[old_idx]);
                    } else {
                        new_outputs.push(BitValue::Zero);
                    }
                } else {
                    new_outputs.push(BitValue::Zero);
                }
            }
            r.outputs = new_outputs;
        }

        self.cached_output_ids = new_ids;
    }

    pub fn remove_column(&mut self, index: usize) {
        for r in &mut self.rows {
            let input_len = r.inputs.len();
            if index < input_len {
                r.inputs.remove(index);
            } else {
                r.outputs.remove(index - input_len);
            }
        }

        let new_ids: Vec<VariableId> = {
            let v = self.variables.borrow();
            v.outputs.iter().map(|o| o.id).collect()
        };

        self.cached_output_ids = new_ids;
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
