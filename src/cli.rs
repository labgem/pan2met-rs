//! Command Line Interface of pan2met-rs

/* std use */
use std::path::PathBuf;

/* crate use */

/* project use */

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "pan2met-rs",
    bin_name = "pan2met-rs",
    version = "0.1.0",
    author = "Samuel Ortion <samuel@ortion.fr>"
)]
pub struct Arguments {
    /// Input path to a file listing reactions
    #[clap(short = 'r', long = "reactions", help = "Input list of reactions")]
    reactions: PathBuf,

    /// Input PADMet reference knowledge base
    #[clap(long = "reference", help = "Reference PADMet file")]
    padmet: PathBuf,

    /// Ouput path to a file listing predicted metabolic pathways
    #[clap(
        short = 'o',
        long = "output",
        help = "Ouput list of metabolic pathways"
    )]
    output: PathBuf,

    /// Taxon identifier
    #[clap(short = 't', long = "taxon-id", help = "NCBI Taxonomy identifier")]
    taxon_id: Option<u32>,

    /// Config
    #[clap(short = 'c', long = "config", help = "Configuration yaml file")]
    config: String,

    // Generic parameter
    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
}

impl Arguments {
    /// Get reactions input path
    pub fn reactions(&self) -> PathBuf {
        self.reactions.clone()
    }

    /// Get reference PADMet file
    pub fn padmet(&self) -> PathBuf {
        self.padmet.clone()
    }

    /// Get a taxon id
    pub fn taxon_id(&self) -> Option<u32> {
        self.taxon_id
    }

    /// Get output filename
    pub fn output(&self) -> PathBuf {
        self.output.clone()
    }

    /// Get verbosity
    pub fn verbosity(&self) -> usize {
        self.verbosity as usize
    }

    /// Get quiet
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    /// Get timestamp granularity
    pub fn timestamp(&self) -> stderrlog::Timestamp {
        self.ts.unwrap_or(stderrlog::Timestamp::Off)
    }

    pub fn config(&self) -> String {
        self.config.clone()
    }
}
