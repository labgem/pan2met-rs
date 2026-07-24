//! Error definition of pan2met-rs

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    #[error("No corresponding gene family node found in pangenome graph for gene family {0}")]
    GeneFamilyNotFoundInPangenomeGraph(String)
}

/// Alias of result
pub type Result<T> = anyhow::Result<T>;
