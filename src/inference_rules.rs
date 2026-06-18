//! # Decision rules for metabolic pathway inference
//!

/* std use */
use std::collections::HashSet;

/* crate use */
use rule_kit::Rule;
use taxonomy::GeneralTaxonomy;

/* project use */
use crate::config;
use crate::{pathway_score, taxonomy::taxid_is_parent_of_taxid};

/// Final decision: reject or accept a metabolic pathway
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Decision {
    Reject,
    Accept,
}

#[derive(Debug)]
pub struct PathwayInference<'a> {
    pub pathway_id: &'a String,
    pub catalyzed_reactions: &'a HashSet<String>,
    pub reactome: &'a HashSet<String>,
    pub pathway_classes: &'a HashSet<String>,
    pub missing_reactions: &'a Vec<String>,
    pub taxon_id: Option<String>,
    pub ncbi_taxonomy: &'a Option<GeneralTaxonomy>,
    pub reaction_order: &'a Option<Vec<String>>,
    pub decision: Option<Decision>,
    pub padmet_object: &'a padmet::spec::PadmetSpec,
}

#[derive(Debug)]
pub enum PathwayInferenceError {
    AlreadyDecided,
    NoReactionInPathway,
    NoPathwayOntology,
}

impl<'a> PathwayInference<'a> {
    pub fn new(
        pathway_id: &'a String,
        pathway_classes: &'a HashSet<String>,
        catalyzed_reactions: &'a HashSet<String>,
        reactome: &'a HashSet<String>,
        missing_reactions: &'a Vec<String>,
        taxon_id: Option<String>,
        ncbi_taxonomy: &'a Option<GeneralTaxonomy>,
        reaction_order: &'a Option<Vec<String>>,
        padmet_object: &'a padmet::spec::PadmetSpec,
    ) -> Self {
        PathwayInference {
            pathway_id,
            catalyzed_reactions,
            pathway_classes,
            missing_reactions,
            reactome,
            taxon_id,
            ncbi_taxonomy,
            reaction_order,
            decision: None,
            padmet_object,
        }
    }

    pub fn reject(&mut self) -> Result<(), PathwayInferenceError> {
        if self.decision.is_some() {
            Err(PathwayInferenceError::AlreadyDecided)
        } else {
            self.decision = Some(Decision::Reject);
            Ok(())
        }
    }

    pub fn accept(&mut self) -> Result<(), PathwayInferenceError> {
        if self.decision.is_some() {
            Err(PathwayInferenceError::AlreadyDecided)
        } else {
            self.decision = Some(Decision::Accept);
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum PathwayInferenceRule {
    /// Reject if the pathway is a transport pathway
    TransportPathway,
    /// Reject if the pathway is a signaling pathway
    SignalingPathway,
    /// Accept if all reactions are catalyzed
    AllReactionsCatalyzed,
    /// Reject if none of the non-spontaneous reactions are catalyzed
    AllReactionsMissing,
    /// Reject if the pathway is out of the taxonomic range
    OutOfTaxonomicRange,
    /// Reject if a key reaction is missing
    KeyReaction,
    /// Reject if the pathway is a synthesis pathway (MetaCyc class "Biosynthesis") and is missing the last reaction leading the the metabolite of interest
    SynthesisMissingLast,
    /// Reject if the pathway is a catabolysis pathway (MetaCyc class "Degradation") and is missing the first reaction degrading the metabolite of interest
    DegradationMissingFirst,
    /// Reject if the pathway is a pathway of the energy metabolism and half non-spontaneous reactions are not catalyzed
    EnergyMissingHalf,
    /// Accept the pathway if the score of the pathway is above a threshold
    PathwayScore,
}

impl PathwayInferenceRule {
    pub fn all() -> Vec<PathwayInferenceRule> {
        vec![
            Self::TransportPathway,
            Self::SignalingPathway,
            Self::AllReactionsCatalyzed,
            Self::AllReactionsMissing,
            Self::OutOfTaxonomicRange,
            Self::KeyReaction,
            Self::SynthesisMissingLast,
            Self::DegradationMissingFirst,
            Self::EnergyMissingHalf,
            Self::PathwayScore,
        ]
    }
}

impl Rule<PathwayInference<'_>> for PathwayInferenceRule {
    type RuleError = PathwayInferenceError;

    fn name(&self) -> &str {
        match self {
            PathwayInferenceRule::TransportPathway => "TransportPathway",
            PathwayInferenceRule::SignalingPathway => "SignalingPathway",
            PathwayInferenceRule::AllReactionsCatalyzed => "AllReactionsCatalyzed",
            PathwayInferenceRule::AllReactionsMissing => "AllReactionsMissing",
            PathwayInferenceRule::OutOfTaxonomicRange => "AllReactionsMissing",
            PathwayInferenceRule::KeyReaction => "KeyReaction",
            PathwayInferenceRule::SynthesisMissingLast => "SynthesisMissingLast",
            PathwayInferenceRule::DegradationMissingFirst => "DegradationMissingFirst",
            PathwayInferenceRule::EnergyMissingHalf => "EnergyMissingHalf",
            PathwayInferenceRule::PathwayScore => "PathwayScore",
        }
    }

    fn priority(&self) -> u32 {
        match self {
            PathwayInferenceRule::TransportPathway => 1,
            PathwayInferenceRule::SignalingPathway => 2,
            PathwayInferenceRule::AllReactionsCatalyzed => 3,
            PathwayInferenceRule::AllReactionsMissing => 4,
            PathwayInferenceRule::OutOfTaxonomicRange => 5,
            PathwayInferenceRule::KeyReaction => 6,
            PathwayInferenceRule::SynthesisMissingLast => 7,
            PathwayInferenceRule::DegradationMissingFirst => 8,
            PathwayInferenceRule::EnergyMissingHalf => 9,
            PathwayInferenceRule::PathwayScore => 10,
        }
    }

    /// Return true if the rule can be evaluated.
    fn evaluate(&self, ctx: &PathwayInference) -> Result<bool, Self::RuleError> {
        match self {
            PathwayInferenceRule::TransportPathway => {
                if ctx.decision.is_some() {
                    Ok(false) // We pass the rule, if we already have a decision.
                } else {
                    Ok(ctx.pathway_classes.contains("Transport-Pathways"))
                }
            }
            PathwayInferenceRule::SignalingPathway => {
                if ctx.decision.is_some() {
                    Ok(false) // We pass the rule, if we already have a decision.
                } else {
                    Ok(ctx.pathway_classes.contains("Signaling-Pathways"))
                }
            }
            PathwayInferenceRule::AllReactionsCatalyzed => {
                if ctx.decision.is_some() {
                    Ok(false)
                } else {
                    Ok(ctx.missing_reactions.is_empty())
                }
            }
            PathwayInferenceRule::AllReactionsMissing => {
                if ctx.decision.is_some() {
                    Ok(false)
                } else {
                    Ok(ctx.missing_reactions.len() == ctx.catalyzed_reactions.len())
                }
            }
            PathwayInferenceRule::OutOfTaxonomicRange => {
                if ctx.decision.is_some() {
                    Ok(false)
                } else {
                    if let Some(taxon_id) = &ctx.taxon_id {
                        if let Some(ranges) =
                            crate::padmet::padmet_taxonomic_range(ctx.pathway_id, ctx.padmet_object)
                        {
                            for taxonomic_range in ranges {
                                if let Some(true) = taxid_is_parent_of_taxid(
                                    &taxonomic_range,
                                    taxon_id,
                                    ctx.ncbi_taxonomy.as_ref().unwrap(),
                                ) {
                                    return Ok(false);
                                }
                            }
                            return Ok(true); // no taxonomic range encompasses the taxonomic id
                        }
                    }
                    Ok(false)
                }
            }
            PathwayInferenceRule::KeyReaction => {
                if ctx.decision.is_some() {
                    return Ok(false);
                } else {
                    // Apply this rule, if any key reaction is missing
                    if let Some(key_reactions) = crate::padmet::padmet_pathway_key_reactions(
                        ctx.pathway_id,
                        ctx.padmet_object,
                    ) {
                        for key_reaction in key_reactions {
                            if ctx.missing_reactions.contains(&key_reaction) {
                                return Ok(true);
                            }
                        }
                    }
                }
                Ok(false)
            }
            PathwayInferenceRule::SynthesisMissingLast => {
                if ctx.decision.is_some() {
                    return Ok(false);
                }
                if let Some(order) = &ctx.reaction_order {
                    let last_reaction = &order[order.len() - 1];
                    return Ok(ctx.pathway_classes.contains("Biosynthesis")
                        && ctx.missing_reactions.contains(last_reaction));
                }
                Ok(false)
            }
            PathwayInferenceRule::DegradationMissingFirst => {
                if ctx.decision.is_some() {
                    return Ok(false);
                }
                if let Some(order) = &ctx.reaction_order {
                    let first_reaction = &order[0];
                    Ok(ctx.pathway_classes.contains("Degradation")
                        && ctx.missing_reactions.contains(first_reaction))
                } else {
                    Ok(false)
                }
            }
            PathwayInferenceRule::EnergyMissingHalf => Ok(ctx.decision.is_none()
                && ctx.pathway_classes.contains("Degradation")
                && ctx.missing_reactions.len() > ctx.catalyzed_reactions.len() / 2),
            PathwayInferenceRule::PathwayScore => {
                if ctx.decision.is_some() {
                    Ok(false)
                } else {
                    let score = pathway_score::PathwayScore::new(
                        ctx.pathway_id,
                        ctx.catalyzed_reactions,
                        ctx.padmet_object,
                        ctx.reactome,
                    );
                    Ok(score.pathway_score() >= config::config().pathway_score_threshold)
                }
            }
        }
    }

    /// Note: `apply` takes `&mut self` and `&mut ctx`, allowing rule and context mutation.
    fn apply(&mut self, ctx: &mut PathwayInference) -> Result<(), Self::RuleError> {
        match self {
            PathwayInferenceRule::TransportPathway => ctx.reject(),
            PathwayInferenceRule::SignalingPathway => ctx.reject(),
            PathwayInferenceRule::AllReactionsCatalyzed => ctx.accept(),
            PathwayInferenceRule::AllReactionsMissing => ctx.reject(),
            PathwayInferenceRule::OutOfTaxonomicRange => ctx.reject(),
            PathwayInferenceRule::KeyReaction => ctx.reject(),
            PathwayInferenceRule::SynthesisMissingLast => ctx.reject(),
            PathwayInferenceRule::DegradationMissingFirst => ctx.reject(),
            PathwayInferenceRule::EnergyMissingHalf => ctx.reject(),
            PathwayInferenceRule::PathwayScore => ctx.accept(),
        }
    }

    fn before_apply(&self, ctx: &PathwayInference) {
        log::debug!(
            "Apply rule {:?} to pathway {:?}",
            self.name(),
            ctx.pathway_id
        );
    }

    fn after_apply(&self, ctx: &PathwayInference) {
        log::debug!(
            "Decision after rule {:?} on pathway {:?}: {:?}",
            self.name(),
            ctx.pathway_id,
            ctx.decision.as_ref().unwrap()
        );
    }
}
