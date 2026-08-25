//! 
//! Identify potential protein complexes from a set of protein monomers and protein complex composition

/* std use */

/* crate use */
use clap::Parser;

/* project use */

mod cli;
mod metabiantes_complex;

fn main() {
    let args = cli::Args::parse();

}
