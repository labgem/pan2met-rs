use clap::Parser;

/// A program to identify the complete set of polypeptide, monomer and complexes that could be formed, given a set of polypeptide monomers
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a metabiantes sqlite database
    #[arg(short='d', long="database")]
    db: String,

    /// Path to the list of monomer identifiers
    #[arg(short='p', long="polypeptides")]
    polypeptides: String
}

