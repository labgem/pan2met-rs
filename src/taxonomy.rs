//! # Taxonomy related helper functions

/* crate use */
use taxonomy::Taxonomy;

/// Check if a NCBI-Taxonomy id is a parent of another NCBI-Taxonomy id
pub fn taxid_is_parent_of_taxid(
    parent_taxid: &str,
    child_taxid: &str,
    tax: &taxonomy::GeneralTaxonomy,
) -> Option<bool> {
    for child in tax.traverse(parent_taxid).unwrap() {
        if child.0 == child_taxid {
            return Some(true);
        }
    }
    Some(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_is_parent_of_child() {
    //     let ncbi_directory = "/mnt/shared/bank/NCBI-Taxonomy/taxdmp_2026-01-01";
    //     let tax = taxonomy::ncbi::load(ncbi_directory).unwrap();

    //     assert!(taxid_is_parent_of_taxid("561", "562", &tax).unwrap());

    //     assert!(!taxid_is_parent_of_taxid("562", "561", &tax).unwrap());
    // }
}
