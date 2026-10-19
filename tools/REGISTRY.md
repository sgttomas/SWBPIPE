# Tool Registry

| Name | Category | Language | Purpose | Inputs | Outputs |
|------|----------|----------|---------|--------|---------|
| `deliverables` | coordination | Python 3 | Derive neighbourhood, impact and changed edges from deliverable YAML | Root `python3 -m tools.deliverables --project projects/chirality-piping` | JSON facts; no stored lifecycle |
| `retained_precision_synthetic_bases` | fixtures | Python 3 (standard library) | Regenerate the shared retained-precision corpus's synthetic reader-control bases: derived rows from their own rows by the producer's binary64 sequences, then the corpus rehash rule; producer-solved and hook bases stay with their producer writers | `python3 tools/fixtures/retained_precision_synthetic_bases.py [--check]`, or `tools/fixtures/retained_precision_corpus.py synthetic-bases [--check]` | The corpus (and B2 probe shapes) rewritten in place; `--check` writes nothing and exits 1 at the first difference |
| `retained_pricing` | pricing | Python 3 | Price the retained route's memory at a revision (SQ2's TEXT + cap-priced chain under the O-10 rules) and write the registered GENERATED PROFILE block; see its README | `price.py run <rev> --out DIR`; `generate <tree> <retained_memory.rs>`; full clone with basis `2243be380b` | Profile tree, per-point summaries, exit code; the regenerated block |
| `validate_dependencies_schema` | validation | Python 3 | Validate `Dependencies.csv`/`DependencyEdges.csv` v3.1 shape plus canonical enum and row-rule semantics; `--schema-only` is reserved for historical legacy snapshots | CSV path, optional `--schema-only` | VALID/INVALID report and exit code |

## Example (from repository root)

```bash
python3 -m tools.deliverables --project projects/chirality-piping neighborhood DEL-04-07
```
