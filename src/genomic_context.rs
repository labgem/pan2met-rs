//! # Genomic context around a gene family in a pangenome graph
//!
//!
/* std use */
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

/* crate use */
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;

use gt_reader::read_gt;
use gt_reader::GraphToolGraph;

use bidirectional_map::Bimap;

/* project use */

/// A struct to store information on a pangenome graph
///
/// We want the gene family name of a vertex,
/// the graph topology, and strains set on both edges and vertices.
pub struct PangenomeGraph {
    pub graph: UnGraph<usize, ()>,
    /// Two-way mapping of graph's NodeIndex to the usize node identifier in GraphToolGraph
    pub node_index_to_gt_node_mapping: Bimap<NodeIndex, usize>,
    /// Mapping of graph's NodeIndex to the gene family identifier
    pub node_index_to_family_mapping: HashMap<NodeIndex, String>,
    /// Mapping of a family to all NodeIndex being of this gene family
    pub family_to_node_index_mapping: HashMap<String, Vec<NodeIndex>>,
    /// A reference to the loaded graph from graph-tools' gt file format, with graph-, vertex- and edge-properties.
    pub graph_tool_graph: GraphToolGraph,
}

impl PangenomeGraph {
    pub fn from_graph_tool_graph(graph_tool_graph: GraphToolGraph) -> Self {
        let mut graph = UnGraph::new_undirected();

        let vmap = populate_undirected_graph_edges(&mut graph, &graph_tool_graph);
        // Create a bidirectional mapping from graph-tools graph node index to petgraph node index
        let mut node_index_to_gt_node_mapping: Bimap<NodeIndex, usize> = Bimap::new();
        for vertex_graph_tool in 0..vmap.len() {
            let vertex_graph = vmap[vertex_graph_tool];
            node_index_to_gt_node_mapping.insert(vertex_graph, vertex_graph_tool);
        }
        // Create a mapping from pangenome gene family identifier and petgraph node index
        let mut node_index_to_family_mapping: HashMap<NodeIndex, String> = HashMap::new();
        let mut family_to_node_index_mapping: HashMap<String, Vec<NodeIndex>> = HashMap::new();
        for vertex_graph_tool in 0..vmap.len() {
            let vertex_graph = vmap[vertex_graph_tool];
            let family = graph_tool_graph
                .vertex_properties
                .string_maps
                .get(&String::from("nid"))
                .expect("Error: pangenome gt file should contain a vertex property map named \"nid\" of type `vector<string>`.")
                .get(&vertex_graph_tool)
                .expect(&format!("Error: pangenome gt file vertex property map \"nid\" should contain a property for the vertex {}.", vertex_graph_tool));
            node_index_to_family_mapping.insert(vertex_graph, family.to_owned());

            if let Some(family_vertices) = family_to_node_index_mapping.get_mut(family) {
                family_vertices.push(vertex_graph);
            } else {
                let family_vertices = vec![vertex_graph];
                family_to_node_index_mapping.insert(family.to_string(), family_vertices);
            }
        }
        // Instantiate the struct
        PangenomeGraph {
            graph,
            graph_tool_graph,
            node_index_to_gt_node_mapping,
            node_index_to_family_mapping,
            family_to_node_index_mapping,
        }
    }

    /// Load the pangenome graph from a .gt file in graph-tools' binary gt file
    /// The gt file should contains the following properties:
    /// - **Vertex properties**
    ///   - "nid" - `string` -- the  identifier of the gene family associated with the vertex
    ///   - "strains" - `vector<string>` -- the set of strains associated with the vertex
    /// - **Edge properties**
    ///    - "strains" - `vector<string` -- the set of strains associated with the edge (i.e, if edge $(u, v)$ has strain `A` in the set of property "strains" it means the strain A has gene members of family u and v colocalized in their genomes)
    pub fn from_gt<P>(gt_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        // Read the binary gt file format
        let graph_tool_graph = read_gt(gt_path).unwrap();
        Self::from_graph_tool_graph(graph_tool_graph)
    }

    /// Get the list of strains associated with a vertex
    pub fn vertex_strains(&self, node_index: NodeIndex) -> Option<&Vec<String>> {
        let vertex_graph_tool = self.node_index_to_gt_node_mapping.get_fwd(&node_index)?;
        let strains = self
            .graph_tool_graph
            .vertex_properties
            .string_vector_maps
            .get(&String::from("strains"))?
            .get(&vertex_graph_tool)?;
        Some(strains)
    }

    /// Get the list of strains associated with an edge
    pub fn edge_strains(&self, u_index: NodeIndex, v_index: NodeIndex) -> Option<&Vec<String>> {
        let u_vertex_graph_tool = self.node_index_to_gt_node_mapping.get_fwd(&u_index)?;
        let v_vertex_graph_tool = self.node_index_to_gt_node_mapping.get_fwd(&v_index)?;
        let strains_properties = self
            .graph_tool_graph
            .edge_properties
            .string_vector_maps
            .get(&String::from("strains"))?;
        // Try both (u, v) and (v, u) keys, as the pangenome graph is undirected, but the property might have been recorded for only one of the direction of the edge.
        if let Some(strains) = strains_properties.get(&(*u_vertex_graph_tool, *v_vertex_graph_tool))
        {
            return Some(strains);
        }
        if let Some(strains) = strains_properties.get(&(*v_vertex_graph_tool, *u_vertex_graph_tool))
        {
            return Some(strains);
        }
        None
    }

    /// Get the gene family identifier associated with a vertex
    pub fn vertex_family(&self, node_index: NodeIndex) -> Option<&String> {
        self.node_index_to_family_mapping.get(&node_index)
    }

    /// Get the vertex associated with a gene family
    pub fn family_vertex(&self, family: &String) -> Option<&Vec<NodeIndex>> {
        self.family_to_node_index_mapping.get(family)
    }
}

/// Populate an unweighted directed graph with edges from a `GraphToolGraph` edge adjacency list
pub fn populate_undirected_graph_edges(
    graph: &mut UnGraph<usize, ()>,
    graph_tool_graph: &GraphToolGraph,
) -> Vec<NodeIndex> {
    let n_vertices = graph_tool_graph.edges.len();
    let mut vmap: Vec<NodeIndex> = Vec::with_capacity(n_vertices);
    for u in 0..n_vertices {
        let u_index = graph.add_node(u);
        vmap.push(u_index);
        dbg!(&u_index);
    }
    for u in 0..n_vertices {
        let u_index = vmap[u];
        for &v in &graph_tool_graph.edges[u] {
            let v_index = vmap[v];
            graph.add_edge(u_index, v_index, ());
            graph.add_edge(v_index, u_index, ());
        }
    }
    vmap
}

pub fn get_node_index(
    map: &mut HashMap<NodeIndex, NodeIndex>,
    original_node_index: NodeIndex,
    closure_graph: &mut UnGraph<NodeIndex, ()>,
) -> NodeIndex {
    if let Some(&closure_node_index) = map.get(&original_node_index) {
        closure_node_index
    } else {
        let closure_node_index = closure_graph.add_node(original_node_index); // Give the closure graph node weight the original node index it hat in the global graph.
        map.insert(original_node_index, closure_node_index);
        closure_node_index
    }
}

/// Compute the transitive closure of a graph starting from a set of nodes
///
/// ## Arguments
/// - **graph** -- the pangenome graph, each node represents a gene family. The nodes are linked by edges with labels that corresponds to a set of genomes where the two gene families are colocalized
/// - **families** -- a set of gene families we consider to reset the number of gaps in the transitive closure
/// - **seed_family** -- starting node to compute the transitive closure
/// - **transitive** -- size of the transitive closure (allowed gaps between two nodes in `families`) to build the graph
///
/// ## Returns
/// The constructed gene context graph and the set of
/// gene families corresponding to the context that exist in at least one genome
pub fn compute_gene_context_graph(
    pangenome: &PangenomeGraph,
    families: &HashSet<String>,
    seed_family: &String,
    transitive: usize,
) -> (UnGraph<NodeIndex, ()>, HashSet<String>) {
    let mut closure_graph: UnGraph<NodeIndex, ()> = UnGraph::new_undirected();
    let mut min_node_depth: HashMap<NodeIndex, usize> = HashMap::new();
    let mut visited_families: HashSet<String> = HashSet::new();
    let mut visited_nodes: HashSet<(NodeIndex, usize)> = HashSet::new();
    let mut queue: Vec<(NodeIndex, usize)> = Vec::new(); // the current node in the depth first search and its depth compared to the last node belonging to the families set
    let family_nodes: &Vec<NodeIndex> = pangenome.family_vertex(seed_family).expect(&format!(
        "Error: seed family `{seed_family:#}` not found in the pangenome graph."
    ));

    let mut closure_graph_node_vmap: HashMap<NodeIndex, NodeIndex> = HashMap::new();

    // TODO support returning multiple closure graph starting from each family vertex.
    let node = family_nodes[0];
    queue.push((node, 0));
    dbg!(node);
    while let Some((node, depth)) = queue.pop() {
        if visited_nodes.contains(&(node, depth)) {
            continue;
        } 
        visited_nodes.insert((node, depth));
        
        // Do not continue to deepen the search if we already reach a gap depth higher than the transitive size
        if depth > transitive {
            continue;
        }
        if let Some(last_depth) = min_node_depth.get(&node) {
            // If the node was already visited at the same depth or in a shallower depth, skip it.
            if *last_depth <= depth {
                continue;
            } else {
                // If the node was already visited but at a deeper location
                min_node_depth.insert(node, depth);
            }
        }
        // Set the depth to 0 if the node belongs to the family set
        let node_family: &String = pangenome.vertex_family(node).expect(&format!(
            "Error: node `{:?}` have not family set in property maps",
            node
        ));
        let mut effective_depth = depth;
        if families.contains(node_family) {
            effective_depth = 0;
            visited_families.insert(node_family.to_owned()); // We visited a node with this family
        }
        // Add the successor nodes to the queue
        for edge in pangenome.graph.edges(node.into()) {
            let target = edge.target();
            queue.push((target.into(), effective_depth + 1));
            let closure_u = get_node_index(
                &mut closure_graph_node_vmap,
                node.into(),
                &mut closure_graph,
            );
            let closure_v = get_node_index(
                &mut closure_graph_node_vmap,
                target.into(),
                &mut closure_graph,
            );
            closure_graph.add_edge(closure_u, closure_v, ());
        }
    }

    (closure_graph, visited_families)
}

#[cfg(test)]
mod tests {
    use gt_reader::property_maps::PropertyMaps;

    use super::*;

    #[test]
    fn test_reading_pangenome() {
        let gt_path = "/home/sortion/data/pangbank/s__Methanofastidiosum_sp001587595_id4147/pangenomeGraph.gt";
        let _pangenome = PangenomeGraph::from_gt(&gt_path);
    }

    fn fictive_simple_gt() -> GraphToolGraph {
        let mut edge_properties: PropertyMaps<(usize, usize)> = PropertyMaps::new();
        let mut vertex_properties: PropertyMaps<usize> = PropertyMaps::new();
        // Create a fictive "nid" hashmap mapping gt node index to the corresponding gene family name
        let vertex_family_map_pairs = vec![
            (0, String::from("fam1")),
            (1, String::from("fam2")),
            (2, String::from("fam3")),
            (3, String::from("fam4")),
        ];
        let vertex_family_map: HashMap<usize, String> =
            vertex_family_map_pairs.into_iter().collect();
        vertex_properties
            .string_maps
            .insert(String::from("nid"), vertex_family_map);

        // Create a fictive "strains" hashmap mapping gt node index to the corresponding vector of strain names
        let vertex_strains_map_pairs = vec![
            (
                0,
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                1,
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                2,
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                3,
                vec![String::from("organism1"), String::from("organism2")],
            ),
        ];
        let vertex_strains_map: HashMap<usize, Vec<String>> =
            vertex_strains_map_pairs.into_iter().collect();
        vertex_properties
            .string_vector_maps
            .insert(String::from("strains"), vertex_strains_map);

        // Create a fictive "strains" hashmap mapping gt (u,v) edges to the corresponding vector of strain names
        let edge_strains_map_pairs = vec![
            (
                (0, 1),
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                (0, 2),
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                (0, 3),
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                (1, 3),
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                (2, 3),
                vec![String::from("organism1"), String::from("organism2")],
            ),
            (
                (3, 0),
                vec![String::from("organism1"), String::from("organism2")],
            ),
        ];
        let edge_strains_map: HashMap<(usize, usize), Vec<String>> =
            edge_strains_map_pairs.into_iter().collect();
        edge_properties
            .string_vector_maps
            .insert(String::from("strains"), edge_strains_map);

        let graph_properties: PropertyMaps<usize> = PropertyMaps::new();
        let graph_tool_graph: GraphToolGraph = GraphToolGraph {
            directed: false,
            edges: vec![vec![1, 2, 3], vec![3], vec![3], vec![0]],
            edge_properties,
            vertex_properties,
            graph_properties,
            comment: String::from("basic fictive pangenome graph"),
        };
        graph_tool_graph
    }

    fn fictive_simple_pangenome() -> PangenomeGraph {
        PangenomeGraph::from_graph_tool_graph(fictive_simple_gt())
    }

    #[test]
    fn test_closure_pangenome() {
        let pangenome = fictive_simple_pangenome();
        let mut families: HashSet<String> = HashSet::new();
        families.insert(String::from("fam1"));
        families.insert(String::from("fam2"));
        families.insert(String::from("fam4"));
        let seed_family = String::from("fam1");
        let res = compute_gene_context_graph(&pangenome, &families, &seed_family, 1);
        let mut expected_visited: HashSet<String> = HashSet::new();
        expected_visited.insert(String::from("fam1"));
        expected_visited.insert(String::from("fam2"));
        expected_visited.insert(String::from("fam4"));
        assert_eq!(expected_visited, res.1);
    }
}
