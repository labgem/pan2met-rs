use std::collections::HashMap;
use std::collections::HashSet;

use padmet::spec::PadmetSpec;

use pan2met::input::{reverse_mapping, read_mapping};
use pan2met::genomic_context::PangenomeGraph;

fn reactions_list_to_families_list(reactions: &HashSet<String>, reactions_to_families: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut v: HashSet<String> = HashSet::new();
    for reaction in reactions {
        if let Some(families) = reactions_to_families.get(reaction) {
            for family in families {
                v.insert(family.to_owned());
            }
        }
    }
    v
}

fn is_in_a_transitive_closure(reactions_to_families: &HashMap<String, Vec<String>>, reactions: &HashSet<String>, pangenome: &PangenomeGraph, transitive: usize, minimal_proportion: f64) -> bool {
    let families = reactions_list_to_families_list(reactions, reactions_to_families);
    let families_set = HashSet::from_iter(families.iter().map(|item| item.to_owned()));
    for seed_family in families {
        if let Ok(closure) = pan2met::genomic_context::compute_gene_context_graph(pangenome, &families_set, &seed_family, transitive) {
            if closure.len() as f64 >= (minimal_proportion * families_set.len() as f64 ){
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let pangenome = PangenomeGraph::from_gt(
        "../tests/test_data/s__Escherichia_coli_GTDB_all_v1.0.0/pangenomeGraph.gt",
    ).unwrap();
    let padmet_object: PadmetSpec =
        PadmetSpec::from_file("/mnt/shared/bank/metacyc.padmet").unwrap();
    let family_to_reactions = read_mapping("../tests/test_data/s__Escherichia_coli_GTDB_all_v1.0.0/pan2met-wf/s__Escherichia_coli/merged_with_ec.asso").unwrap();
    let reactions_to_families: HashMap<String, Vec<String>> = reverse_mapping(&family_to_reactions);

    let transitive = 5;
    let minimal_proportion = 0.4;

    let mut pathways: HashSet<String> = HashSet::new();

    let pathway_to_reactions: HashMap<String, HashSet<String>> = padmet_object.get_pathways_reactions();
    for (pathway, reactions) in pathway_to_reactions {
        if is_in_a_transitive_closure(&reactions_to_families, &reactions, &pangenome, transitive, minimal_proportion) {
            pathways.insert(pathway);
        }
    }
    for pathway in pathways {
        println!("{pathway:#}");
    }
}
