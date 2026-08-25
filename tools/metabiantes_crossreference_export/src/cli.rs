/* std use */
use std::path::PathBuf;

/* crate use */
use clap::Parser;

/* project use */

/// A program to identify the complete set of polypeptide, monomer and complexes that could be formed, given a set of polypeptide monomers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a metabiantes sqlite database
    #[arg(short='d', long="database")]
    pub db: PathBuf,

    /// Path to output TSV with column 1 'monomer identifier' and column 2 'reaction identifier'
    #[arg(long="monomer-to-reaction")]
    pub monomer_to_reactions: PathBuf,

    /// Path to output TSV with column 1 'complex identifier' and column 2 'reaction identifier'
    #[arg(long="complex-to-reaction")]
    pub complex_to_reactions: PathBuf,
}
