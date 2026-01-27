use crate::schema::{NeuroQuitSessionRow, TobaccoFootprintRow};
use crate::safety::{NeuroQuitError, enforce_row_safety};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

pub fn load_sessions(path: &str) -> Result<Vec<NeuroQuitSessionRow>, NeuroQuitError> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut out = Vec::new();
    for result in rdr.deserialize::<NeuroQuitSessionRow>() {
        let row = result?;
        enforce_row_safety(&row)?;
        out.push(row);
    }
    Ok(out)
}

pub fn load_footprint(path: &str) -> Result<HashMap<String, TobaccoFootprintRow>, NeuroQuitError> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut map = HashMap::new();
    for result in rdr.deserialize::<TobaccoFootprintRow>() {
        let row = result?;
        map.insert(row.region.clone(), row);
    }
    Ok(map)
}
