# Example

`pan2met` is a command line. It takes as input a reference knowledgebase on metabolism in PADMet format, a pangenome graph in graph-tool binary gt format and a list of reaction catalyzis that can be associated with a confidence score.
`pan2met` returns a list of metabolic pathways predicted to be realized by the organism.
There is no guarantee that all predicted pathways are effectively realized by the organism, nor that the rejected pathways are not realized at all by the organism, and the results of `pan2met` should always be taken with caution.

To use `pan2met`, you can use the following command line:

```bash
pan2met --config "configuration.yaml" \
    --reactions "reactions.list" \
    --padmet "metacyc.padmet" \
    --output "pathways.list" \
    --taxon-id "562" # NCBI taxonomy identifier of E. coli.
```