//! # Scoring the likelihood of a pathway

use std::collections::HashMap;
use std::collections::HashSet;

/* crate use */
use padmet::spec::PadmetSpec;

/* project use */
use crate::config;
use crate::genomic_context::PangenomeGraph;
use crate::padmet::padmet_count_pathways_with_reaction;
use crate::padmet::padmet_pathway_key_reactions;

pub struct PathwayScore<'a> {
    pub pathway_id: &'a String,
    pub pathway_reactions: &'a HashSet<String>,
    pub padmet_object: &'a PadmetSpec,
    pub reactome: &'a HashSet<String>,
    pub pathway_key_reactions: HashSet<String>,
    pub reaction_to_families: &'a HashMap<String, Vec<String>>,
    pub families_to_reactions: &'a HashMap<String, Vec<String>>,
    pub pangenome: Option<&'a PangenomeGraph>,
}

impl<'a> PathwayScore<'a> {
    pub fn new(
        pathway_id: &'a String,
        pathway_reactions: &'a HashSet<String>,
        padmet_object: &'a PadmetSpec,
        reactome: &'a HashSet<String>,
        reaction_to_families: &'a HashMap<String, Vec<String>>,
        families_to_reactions: &'a HashMap<String, Vec<String>>,
        pangenome: Option<&'a PangenomeGraph>,
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
            reaction_to_families,
            families_to_reactions,
            pangenome,
        }
    }

    pub fn presence_score(&self, reaction_id: &String) -> f64 {
        if config::config()
            .pathway_score
            .components
            .contains(&String::from("PresenceScore"))
            && self.reactome.contains(reaction_id)
        {
            0.2
        } else {
            0.0
        }
    }

    pub fn uniqueness_score(&self, reaction_id: &String) -> f64 {
        if config::config()
            .pathway_score
            .components
            .contains(&String::from("UniquenessScore"))
        {
            (-(padmet_count_pathways_with_reaction(reaction_id, self.padmet_object) as f64) / 10.0)
                .exp()
        } else {
            0.0
        }
    }

    pub fn key_reaction_score(&self, reaction_id: &String) -> f64 {
        if config::config()
            .pathway_score
            .components
            .contains(&String::from("KeyReactionScore"))
            && self.pathway_key_reactions.contains(reaction_id)
        {
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
                / (self.pathway_reactions.iter().count() as f64) * self.genomic_context_boost()
        }
    }

    pub fn is_in_transitive_closure(&self) -> bool {
        let reactions_vec: Vec<String> = self.pathway_reactions.iter().cloned().collect();
        let non_spontaneous_non_orphan_reaction_vec: Vec<String> =
            crate::padmet::filter_non_orphan_non_spontaneous_reactions(
                &reactions_vec,
                self.padmet_object,
            );
        if non_spontaneous_non_orphan_reaction_vec.len() >= 2 {
            let pangenome: &PangenomeGraph = self
                .pangenome
                .expect("Error: pangenome should be loaded when using the genomic context search");
            if let Ok(result) =
                crate::genomic_context::pathway_is_in_a_transitive_closure_context_graph(
                    pangenome,
                    &non_spontaneous_non_orphan_reaction_vec,
                    self.reaction_to_families,
                    self.families_to_reactions,
                    config::config().genomic_context.transitive_closure_gaps,
                )
            {
                return result;
            }
        }
        return false;
    }

    pub fn genomic_context_boost(&self) -> f64 {
        if config::config()
            .pathway_score
            .components
            .contains(&String::from("GenomicContextBoost"))
            && self.is_in_transitive_closure()
        {
            1.3
        } else {
            0.0
        }
    }
}
