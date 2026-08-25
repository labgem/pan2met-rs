//! # metabiantes complex sqlite wrapper
/* std use */
use std::path::Path;

/* crate use */
use include_sqlite_sql::{impl_sql, include_sql};
use rusqlite::{Connection, Result};

/* project use */

include_sql!("/sql/queries.sql");

pub struct Metabiantes {
    pub conn: Connection,
}

impl Metabiantes {
    /// Open the SQLite database connection
    pub fn new<P>(sqlite_database_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let conn: Connection = Connection::open(sqlite_database_path)?;

        Ok(Metabiantes { conn })
    }

    /// List the protein complexes
    pub fn list_complexes(&self) -> Result<Vec<String>> {
        let mut complexes: Vec<String> = Vec::new();
        let _result = self.conn.get_complex(|row| {
            let id: &str = row.get_ref("id")?.as_str()?;
            complexes.push(id.to_string());
            Ok(())
        });
        Ok(complexes)
    }

    /// Recursively list the protein monomer components of a complex
    pub fn complex_components(&self, complex_id: &str) -> Result<Vec<String>> {
        let mut components: Vec<String> = Vec::new();
        let _result = self.conn.get_complex_components(complex_id, |row| {
            let component_id: &str = row.get_ref("component_id")?.as_str()?;
            let component_type: &str = row.get_ref("component_type")?.as_str()?;
            if component_type == "complex" {
                for inner_component_id in self.complex_components(component_id)? {
                    components.push(inner_component_id.to_string())
                }
            } else {
                components.push(component_id.to_string());
            }
            Ok(())
        });
        Ok(components)
    }

    pub fn list_monomers(&self) -> Result<Vec<String>> {
        let mut monomers: Vec<String> = Vec::new();
        let _result = self.conn.get_monomer(|row| {
            let id: &str = row.get_ref("id")?.as_str()?;
            monomers.push(id.trim().to_string());
            Ok(())
        });
        Ok(monomers)
    }

    pub fn list_reaction_catalyzed_by_polypeptide(&self, polypeptide: &str) -> Result<Vec<String>> {
        let mut reactions: Vec<String> = Vec::new();
        let _result = self.conn.get_enzyme_reaction(polypeptide, |row| {
            let reaction_id: &str = row.get_ref("reaction_id")?.as_str()?;
            reactions.push(reaction_id.trim().to_string());
            Ok(())
        });
        Ok(reactions)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let metabiantes_database_sqlite = "/mnt/shared/bank/metabiantes_metacyc_29.5.db";
        let metabiantes = Metabiantes::new(metabiantes_database_sqlite).unwrap();
    }

    #[test]
    fn test_list_complexes() {
        let metabiantes_database_sqlite = "/mnt/shared/bank/metabiantes_metacyc_29.5.db";
        let metabiantes = Metabiantes::new(metabiantes_database_sqlite).unwrap();

        let complexes = metabiantes.list_complexes();
        assert!(complexes.is_ok());
        assert!(!complexes.unwrap_or(Vec::new()).is_empty());
    }

    #[test]
    fn test_list_first_complex_component() {
        let metabiantes_database_sqlite = "/mnt/shared/bank/metabiantes_metacyc_29.5.db";
        let metabiantes = Metabiantes::new(metabiantes_database_sqlite).unwrap();

        let complexes = metabiantes.list_complexes().unwrap();
        let first_complex = complexes.first().unwrap();
        let first_complex_components = metabiantes.complex_components(first_complex).unwrap();
        assert!(!first_complex_components.is_empty());
    }

}
