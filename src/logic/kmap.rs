use crate::logic::{
    gray::{self, gray_sequence},
    truth_table::TruthTable,
    variable::BitValue,
};

use std::fmt;

#[derive(Clone, Debug)]
pub struct KMapFormat {
    pub row_vars: Vec<String>,
    pub col_vars: Vec<String>,
}

impl KMapFormat {
    pub fn auto<V>(variables: &[V]) -> Self
    where
        V: Clone + Into<String>,
    {
        let variables: Vec<String> = variables.iter().cloned().map(Into::into).collect();

        let n = variables.len();
        let r = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };

        Self {
            row_vars: variables[0..r].to_vec(),
            col_vars: variables[r..].to_vec(),
        }
    }

    pub fn split<V>(variables: &[V], rows: usize, columns: usize) -> Self
    where
        V: Clone + Into<String>,
    {
        let variables: Vec<String> = variables.iter().cloned().map(Into::into).collect();

        assert_eq!(
            rows + columns,
            variables.len(),
            "row_bits + col_bits must equal total number of variables"
        );

        Self {
            row_vars: variables[..rows].to_vec(),
            col_vars: variables[rows..].to_vec(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KMap {
    pub variables: Vec<String>,
    pub format: KMapFormat,

    pub rows: usize,
    pub cols: usize,

    pub grid: Vec<Vec<BitValue>>, // row[col]
}

#[allow(dead_code)]
impl KMap {
    /// Creates a new KMap from an existing truth table
    /// ### Parameters
    /// - `table`: A reference to the truth table
    /// - `format`: Determines row and column count
    /// - `output_index`: Optional index used for a table with multiple outputs
    pub fn from_table(table: &TruthTable, format: KMapFormat, output_index: Option<usize>) -> Self {
        let variables = table.inputs.clone();

        let row_count = 1 << format.row_vars.len();
        let col_count = 1 << format.col_vars.len();

        let mut grid = vec![vec![BitValue::Zero; col_count]; row_count];
        for minterm in 0..1u8 << variables.len() {
            let value = &table.rows[minterm as usize].outputs[output_index.unwrap_or(0)];
            let (r, c) =
                gray::extract_row_col(minterm, format.row_vars.len(), format.col_vars.len());

            grid[r as usize][c as usize] = value.clone();
        }

        Self {
            rows: row_count,
            cols: col_count,

            format,
            variables,

            grid: grid,
        }
    }
}

impl fmt::Display for KMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_vars = self.format.row_vars.join("");
        let col_vars = self.format.col_vars.join("");

        let rbits = row_vars.len();
        let cbits = col_vars.len();

        let col_inner_w = cbits.max(1);
        let row_pad = rbits.max(1) + 1;
        let left_pad = if cbits > rbits { cbits + 1 } else { rbits + 1 };

        let cell = |s: &str| format!(" {:^w$} ", s, w = col_inner_w);
        let dash_cell = || "-".repeat(col_inner_w + 2);

        let col_hdr_cells = gray_sequence(cbits as u8)
            .into_iter()
            .map(|v| gray::format_bits(v, cbits as u8))
            .map(|s| cell(&s))
            .collect::<Vec<_>>();

        let border = (0..col_hdr_cells.len())
            .map(|_| dash_cell())
            .collect::<Vec<_>>()
            .join("+");

        let header_line = col_hdr_cells.join("|");

        // Row gray labels
        let row_labels = gray_sequence(rbits as u8)
            .into_iter()
            .map(|v| gray::format_bits(v, rbits as u8))
            .collect::<Vec<_>>();

        // Top header
        writeln!(
            f,
            "{space:>rp$}{cv}{hdr}|",
            space = "",
            rp = row_pad,
            cv = col_vars,
            hdr = header_line
        )?;

        // Header separator
        writeln!(
            f,
            "{row_vars:>row_pad$}{:>col_width$}+{border}+",
            "",
            row_vars = row_vars,
            row_pad = row_pad - 1,
            col_width = col_vars.len(),
            border = border
        )?;

        // Rows
        for (i, row) in self.grid.iter().enumerate() {
            let values = row
                .iter()
                .map(|v| cell(&v.to_char().to_string()))
                .collect::<Vec<_>>()
                .join("|");

            writeln!(
                f,
                "{label:<rp$}|{vals}|",
                label = row_labels[i],
                rp = left_pad,
                vals = values
            )?;
        }

        // Bottom border
        writeln!(
            f,
            "{space:>rp$}+{brd}+",
            space = "",
            rp = left_pad,
            brd = border
        )?;
        Ok(())
    }
}
