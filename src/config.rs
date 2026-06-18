/* std use */
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

/* crate use */
use serde::{Deserialize, Serialize};

/* project use */

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub reference: ConfigReference,
    pub pathway_score_threshold: f64,
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigReference {
    pub padmet: String,
    pub ncbi_taxonomy: String,
}

pub fn read_config<P>(config_path: P) -> Result<Config, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let yaml = read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&yaml)?;
    Ok(config)
}
