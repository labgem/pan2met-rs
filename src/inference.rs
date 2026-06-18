//! # A metabolic pathway inference decision algorithm

/* std use */
use std::collections::HashSet;

/* crate use */

/* project use */

/// Identify the missing reactions of a metabolic pathway
/// ## Arguments
/// - pathway : a pathway identifier
/// - reactome : a set of reaction of the reactome (known to be catalyzed in the organism)
/// - pathway_to_reactions : a mapping from pathway identifier to a set of its reactions that are required to consider it complete (may/should ignore both non-spontaneous and non-orphan reactions of the metabolic pathway)
pub fn get_missing_reactions(
    pathway_reactions: &HashSet<String>,
    reactome: &HashSet<String>,
) -> Vec<String> {
    let missing_reactions = pathway_reactions.difference(reactome);
    let missing_reactions: Vec<String> = missing_reactions.into_iter().cloned().collect();
    missing_reactions
}
