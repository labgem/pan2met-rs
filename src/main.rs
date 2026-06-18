//! predict metabolic pathways

#![warn(missing_docs)]

/* std use */
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;
use padmet::spec::PadmetSpec;
use rule_kit::Rule as _;
use rule_kit::RuleEngineBuilder;

/* project use */
use pan2met::config;
use pan2met::error;
use pan2met::inference_rules::{self, PathwayInferenceRule};
use pan2met::{cli, inference};

use pan2met::padmet::{padmet_pathway_ontology, padmet_reaction_order};

mod input;
use input::read_set;

/// Run the inference
fn pathway_inference(
    reactome: &HashSet<String>,
    padmet_object: &PadmetSpec,
    rules: &[inference_rules::PathwayInferenceRule],
    taxon_id: Option<String>,
    tax: &Option<taxonomy::GeneralTaxonomy>,
) -> Vec<String> {
    let mut infered: Vec<String> = Vec::new();
    for (pathway, pathway_reactions) in padmet_object.get_pathways_reactions() {
        let pathway_classes = padmet_pathway_ontology(&pathway, padmet_object);
        let reaction_order = padmet_reaction_order(&pathway, padmet_object);
        let missing_reactions = inference::get_missing_reactions(&pathway_reactions, reactome);

        let mut inference = inference_rules::PathwayInference::new(
            &pathway,
            &pathway_classes,
            &pathway_reactions,
            reactome,
            &missing_reactions,
            taxon_id.clone(),
            tax,
            &reaction_order,
            padmet_object,
        );
        let mut engine = RuleEngineBuilder::new()
            .with_rules(rules.to_vec())
            .priority_asc()
            .build();

        let res = engine.evaluate_all(&mut inference);
        if res.is_ok()
            && inference
                .decision
                .unwrap_or(inference_rules::Decision::Reject)
                == inference_rules::Decision::Accept
        {
            infered.push(pathway);
        }
    }
    infered
}

#[hotpath::main]
fn main() -> error::Result<()> {
    // Parse argument
    let arguments = cli::Arguments::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(arguments.quiet())
        .verbosity(arguments.verbosity())
        .timestamp(arguments.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    log::info!("Loading config from {}", arguments.config());
    let configuration: config::Config =
        config::read_config(arguments.config()).expect("Error reading config");
    config::init_config(configuration);

    log::info!("Loading catalyzed reactions.");
    let reactome: HashSet<String> = read_set(arguments.reactions())?;
    log::info!("Loading PADMet reference.");
    let padmet_object: PadmetSpec = PadmetSpec::from_file(arguments.padmet())?;

    let mut tax_id: Option<String> = None;
    let mut some_taxonomy: Option<taxonomy::GeneralTaxonomy> = None;
    if let Some(taxon_id) = arguments.taxon_id() {
        let ncbi_directory = config::config().reference.ncbi_taxonomy;
        let tax = taxonomy::ncbi::load(ncbi_directory)?;
        if tax.to_internal_index(&taxon_id.to_string()).is_err() {
            log::error!("taxon id {taxon_id:#} not found in the NCBI-Taxonomy.");
            return Ok(());
        }
        // Ensure NCBI-Taxonomy contains the given taxon id
        else {
            tax_id = Some(taxon_id.to_string());
            some_taxonomy = Some(tax);
        }
    }

    log::info!("Start a metabolic pathway prediction.");
    let rules = PathwayInferenceRule::all();
    let selected_rules: HashSet<String> =
        config::config().rules.iter().cloned().collect();
    let rules: Vec<PathwayInferenceRule> = rules
        .iter()
        .filter(|&rule| selected_rules.contains(rule.name()))
        .cloned()
        .collect();
    let infered = pathway_inference(&reactome, &padmet_object, &rules, tax_id, &some_taxonomy);

    let mut output_file = File::create(arguments.output())?;
    for pathway in infered {
        writeln!(output_file, "{pathway:#}")?;
    }

    log::info!("End a metabolic pathway prediction.");

    Ok(())
}
