# Genomic context

When `GenomicContextBoost` rule is activated in configuration `pathway_score.components`:

```yaml
[...]
pathway_score:
    components:
        - GenomicContextBoost
        [...]
```
`pan2met` requires the pangenome graph in [graph_tool binary gt format](https://graph-tool.skewed.de/static/docs/stable/gt_format.html) with the topology of the (pan)genome graph.

`pan2met` will check for each pathway if all their non spontaneous (catalyzed) reaction have enzymes disposed on the pangenome graph in a transitive closure with allowed gaps in the graph.

> [!WARNING]
> The computation of the transitive closure takes time. It can make `pan2met` run time pass from ~15s to 10 min, with this rule enabled.

## 

## Expected format of the pangenome gt graph

The pangenome graph provided to option `--pangenome` of `pan2met` should contain the edges of the pangenome linking gene family nodes.
The graph should have the edge property `strains` listing the organism where the edge link was found. It should have the node property `nid` for each gene family, populated with the identifier of the gene family. This identifier should be the same that is used in the table provided in option `--gene-reaction` to name the gene associated with a catalyzis.
The nodes should also be associated with the property `strains` listing the organism where this specific gene family node is found.

This kind of file can be produced by PPanGGOLiN ([PR](https://github.com/labgem/PPanGGOLiN/pull/380)), e.g.: 
```bash
ppanggolin write_pangenome -p pangenome.h5 --gt --output output/ -f
```
The graph_tool gt file will be in `./output/pangenomeGraph.gt`.

