extern crate serde_json;

use serde_json::Result;

use std::fs::File;
use std::io::Read;

use crate::solver::Config;

pub fn parse_genetic_config(file_content: &str) -> Result<Config> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // Parse the string of data into a GeneticConfig object
    let genetic_config: Config = serde_json::from_str(file_content)?;

    Ok(genetic_config)
}
