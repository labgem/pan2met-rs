use std::collections::HashSet;

use padmet::spec::PadmetSpec;

/// Sorted list of reactions from a PADMet object
pub fn padmet_reaction_order(
    pathway_id: &String,
    padmet_object: &PadmetSpec,
) -> Option<Vec<String>> {
    if let Some(node) = padmet_object.dic_of_nodes.get(pathway_id) {
        if let Some(order) = node.node_misc.get("REACTION-ORDER") {
            let order = &order[0];
            return Some(order.split(",").map(|e| e.to_owned()).collect());
        }
    }
    None
}

/// Parent pathway ontology classes
///
/// get all target of PADMet `is_a` relations, coming from the pathway node pathway_id
pub fn padmet_pathway_ontology(pathway_id: &String, padmet_object: &PadmetSpec) -> HashSet<String> {
    padmet_object
        .get_relations_type_id_in(&"is_a".to_owned(), pathway_id)
        .iter()
        .map(|relation| relation.id_out.clone())
        .collect()
}

/// Get a misc value of a padmet node
pub fn padmet_get_misc_value(
    pathway_id: &String,
    padmet_object: &PadmetSpec,
    key: &String,
) -> Option<Vec<String>> {
    if let Some(node) = padmet_object.dic_of_nodes.get(pathway_id) {
        if let Some(values) = node.node_misc.get(key) {
            return Some(values.clone());
        }
    }
    None
}

/// Get key reactions of a pathway
pub fn padmet_pathway_key_reactions(
    pathway_id: &String,
    padmet_object: &PadmetSpec,
) -> Option<Vec<String>> {
    padmet_get_misc_value(pathway_id, padmet_object, &"KEY-REACTIONS".to_owned())
}

/// Get expected taxonomic range of a pathway as NCBI-Taxonomy identifier
pub fn padmet_taxonomic_range(
    pathway_id: &String,
    padmet_object: &PadmetSpec,
) -> Option<Vec<String>> {
    match padmet_get_misc_value(pathway_id, padmet_object, &"TAXONOMIC-RANGE".to_owned()) {
        None => None,
        Some(ranges) => {
            let taxa: Vec<String> = ranges
                .iter()
                .filter(|taxon| taxon.starts_with("TAX-"))
                .map(|taxon| taxon.replace("TAX-", ""))
                .collect();
            Some(taxa)
        }
    }
}

/// Count how many pathway has a reaction
pub fn padmet_count_pathways_with_reaction(
    reaction_id: &String,
    padmet_object: &PadmetSpec,
) -> usize {
    padmet_object
        .get_relations_type_id_in(&"is_in_pathway".to_owned(), reaction_id)
        .len()
}

pub fn is_spontaneous_reaction(reaction: &String, padmet_object: &PadmetSpec) -> bool {
    if let Some(reaction_node) = padmet_object.dic_of_nodes.get(reaction) {
        if let Some(spontaneous_p) = reaction_node.node_misc.get("SPONTANEOUS") {
            if spontaneous_p[0] == "T" {
                return true;
            }
        }
    }
    return false;
}

pub fn filter_non_orphan_non_spontaneous_reactions(
    reactions: &Vec<String>,
    padmet_object: &PadmetSpec,
) -> Vec<String> {
    reactions
        .iter()
        .filter(|reaction| !is_spontaneous_reaction(reaction, padmet_object))
        .cloned()
        .collect()
}
