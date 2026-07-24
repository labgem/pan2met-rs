//! predict metabolic pathways

/* std use */

/* crate use */

/* module declaration */

pub mod cli;
pub mod config;
pub mod error;
pub mod inference;
pub mod inference_rules;
pub mod padmet;
pub mod pathway_score;
pub mod taxonomy;
pub mod genomic_context;
pub mod input;

/* project use */

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;
}
