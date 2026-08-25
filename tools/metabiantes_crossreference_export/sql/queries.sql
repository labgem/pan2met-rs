-- name: get_complex?
-- Returns the list protein complexes
-- # Parameters
--
SELECT id
FROM polypeptide
WHERE type == "complex";
/
-- name: get_monomer?
-- Returns the list protein monomer
-- # Parameters
--
SELECT id
FROM polypeptide
WHERE type == "monomer";
/
-- name: get_complex_components?
-- Returns the components of a protein complex (can be complex themselves)
-- # Parameters
-- param: complex_id: &str - identifier of the complex (frame-name)
SELECT polypeptide_complex_component.component_id, component.type as component_type
FROM polypeptide_complex_component
INNER JOIN polypeptide component
ON component.id = component_id
WHERE polypeptide_complex_component.complex_id = :complex_id;
/
-- name: get_enzyme_reaction?
-- Returns the reaction catalyzed by an enzyùe
-- # Parameters
-- param: enzyme_id: &str - identifier of the polypeptide
SELECT reaction_enzyme.reaction_id
FROM reaction_enzyme
WHERE reaction_enzyme.enzyme_id = :enzyme_id;
/