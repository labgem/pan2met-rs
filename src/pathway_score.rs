//! # Scoring the likelihood of a pathway

use std::collections::HashSet;

/* crate use */
use padmet::spec::PadmetSpec;

/* project use */
use crate::padmet::padmet_count_pathways_with_reaction;
use crate::padmet::padmet_pathway_key_reactions;

pub struct PathwayScore<'a> {
    pub pathway_id: &'a String,
    pub pathway_reactions: &'a HashSet<String>,
    pub padmet_object: &'a PadmetSpec,
    pub reactome: &'a HashSet<String>,
    pub pathway_key_reactions: HashSet<String>,
}

impl<'a> PathwayScore<'a> {
    pub fn new(
        pathway_id: &'a String,
        pathway_reactions: &'a HashSet<String>,
        padmet_object: &'a PadmetSpec,
        reactome: &'a HashSet<String>,
    ) -> Self {
        let key_reactions: HashSet<String> =
            padmet_pathway_key_reactions(pathway_id, padmet_object)
                .unwrap_or_default()
                .into_iter()
                .collect();
        PathwayScore {
            pathway_id,
            pathway_reactions,
            padmet_object,
            reactome,
            pathway_key_reactions: key_reactions,
        }
    }

    pub fn presence_score(&self, reaction_id: &String) -> f64 {
        if self.reactome.contains(reaction_id) {
            0.2
        } else {
            0.0
        }
    }

    pub fn uniqueness_score(&self, reaction_id: &String) -> f64 {
        (-(padmet_count_pathways_with_reaction(reaction_id, self.padmet_object) as f64) / 10.0)
            .exp()
    }

    pub fn key_reaction_score(&self, reaction_id: &String) -> f64 {
        if self.pathway_key_reactions.contains(reaction_id) {
            0.5
        } else {
            0.0
        }
    }

    pub fn reaction_score(&self, reaction_id: &String) -> f64 {
        if self.reactome.contains(reaction_id) {
            self.presence_score(reaction_id)
                + self.uniqueness_score(reaction_id)
                + self.key_reaction_score(reaction_id)
        } else {
            0.0
        }
    }

    pub fn pathway_score(&self) -> f64 {
        if self.pathway_reactions.is_empty() {
            0.0
        } else {
            self.pathway_reactions
                .iter()
                .map(|reaction_id| self.reaction_score(reaction_id))
                .sum::<f64>()
                / (self.pathway_reactions.iter().count() as f64)
        }
    }
}
