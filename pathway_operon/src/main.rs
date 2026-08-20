use std::collections::HashMap;
use std::collections::HashSet;

use padmet::spec::PadmetSpec;
use pan2met::genomic_context::PangenomeGraph;
use pan2met::genomic_context::pathway_is_in_a_transitive_closure_context_graph;
use pan2met::input::{read_mapping, reverse_mapping};

fn main() {
    let pangenome = PangenomeGraph::from_gt(
        "../tests/test_data/s__Escherichia_coli_GTDB_all_v1.0.0/pangenomeGraph.gt",
    )
    .unwrap();
    let padmet_object: PadmetSpec =
        PadmetSpec::from_file("/mnt/shared/bank/metacyc.padmet").unwrap();
    let family_to_reactions = read_mapping("../tests/test_data/s__Escherichia_coli_GTDB_all_v1.0.0/pan2met-wf/s__Escherichia_coli/merged_with_ec.asso").unwrap();
    let reactions_to_families: HashMap<String, Vec<String>> = reverse_mapping(&family_to_reactions);

    let transitive = 3;

    let pathway_to_reactions: HashMap<String, HashSet<String>> =
        padmet_object.get_pathways_reactions();
    for (pathway, reactions) in pathway_to_reactions {
        let reactions_vec: Vec<String> = reactions.iter().cloned().collect();
        let non_spontaneous_non_orphan_reaction_vec: Vec<String> =
            pan2met::padmet::filter_non_orphan_non_spontaneous_reactions(&reactions_vec, &padmet_object);
        if non_spontaneous_non_orphan_reaction_vec.len() >= 2 {

            if let Ok(result) = pathway_is_in_a_transitive_closure_context_graph(
                &pangenome,
                &non_spontaneous_non_orphan_reaction_vec,
                &reactions_to_families,
                &family_to_reactions,
                transitive,
            ) {
                if result {
                    println!("operonic: {pathway:#}");
                } else {
                    // println!("not operonic: {pathway:#}");
                }
            }
        }
    }
}
