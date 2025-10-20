mod logic;
use logic::{
    kmap::{KMap, KMapFormat},
    truth_table::TruthTable,
    variable::BitValue,
};

mod app;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let inputs = vec!["A", "B"];
    let outputs = vec!["S", "Co"];

    let mut table = TruthTable::new(&inputs, &outputs);

    table.set(0, 0, BitValue::One);
    table.set(3, 0, BitValue::One);

    let map = KMap::from_table(&table, KMapFormat::auto(&inputs), None);
    println!("{map}");

    app::app()
}
