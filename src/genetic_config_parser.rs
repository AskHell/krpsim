extern crate serde_json;

use serde_json::Result;

use std::fs::File;
use std::io::Read;

use crate::solver::Config;

pub fn parse_genetic_config() -> Result<Config> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"{
		"mutation_chance": 0.01,
		"max_depth": 10,
		"generation_size": 100,
		"iterations": 10
	}"#;

    // Parse the string of data into a GeneticConfig object
    let genetic_config: Config = serde_json::from_str(data)?;

    Ok(genetic_config)
}
