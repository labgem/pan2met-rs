//!
//! Identify potential protein complexes from a set of protein monomers and protein complex composition

/* std use */
use std::fs::File;
use std::io::Write;

/* crate use */
use clap::Parser;

use pan2met::error::Result;

/* project use */

mod cli;
mod metabiantes;

use metabiantes::Metabiantes;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let metabiantes = Metabiantes::new(args.db)?;

    let mut monomer_to_reaction_file = File::create(args.monomer_to_reactions)?;
    for enzyme in metabiantes.list_monomers()? {
        let reactions = metabiantes.list_reaction_catalyzed_by_polypeptide(&enzyme)?;
        if !reactions.is_empty() {
            let _ = write!(monomer_to_reaction_file, "{enzyme:#}\t");
            let joined = reactions.join(",");
            let _ = writeln!(monomer_to_reaction_file, "{}", joined);
        }
    }

    let mut complex_to_reaction_file = File::create(args.complex_to_reactions)?;
    for enzyme in metabiantes.list_complexes()? {
        let reactions = metabiantes.list_reaction_catalyzed_by_polypeptide(&enzyme)?;
        if !reactions.is_empty() {
            let _ = write!(complex_to_reaction_file, "{enzyme:#}\t");
            let joined = reactions.join(",");
            let _ = writeln!(complex_to_reaction_file, "{}", joined);
        }
    }

    Ok(())  
}
