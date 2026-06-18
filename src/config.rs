/* std use */
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::OnceLock;

/* crate use */
use serde::{Deserialize, Serialize};

/* project use */

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    pub reference: ConfigReference,
    pub pathway_score_threshold: f64,
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

static CONFIGURATION: OnceLock<Config> = OnceLock::new();

pub fn init_config(config: Config) {
    CONFIGURATION.set(config).unwrap();
}

pub fn config() -> Config {
    let config = CONFIGURATION.get().unwrap();
    let config = (*config).clone();
    config
}
