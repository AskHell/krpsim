extern crate serde_json;

use crate::solver::Config;

pub fn parse_genetic_config<'a>(file_content: String) -> Result<Config, &'a str> {
    // Parse the string of data into a GeneticConfig object
    serde_json::from_str(&file_content)
		.map_err(|err| {
			println!("{:?}", err);
			"Parse error"
		})
}
