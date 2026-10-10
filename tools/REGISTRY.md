# Tool Registry

| Name | Category | Language | Purpose | Inputs | Outputs |
|------|----------|----------|---------|--------|---------|
| `deliverables` | coordination | Python 3 | Derive neighbourhood, impact and changed edges from deliverable YAML | Root `python3 -m tools.deliverables --project projects/chirality-piping` | JSON facts; no stored lifecycle |
| `validate_dependencies_schema` | validation | Python 3 | Validate `Dependencies.csv`/`DependencyEdges.csv` v3.1 shape plus canonical enum and row-rule semantics; `--schema-only` is reserved for historical legacy snapshots | CSV path, optional `--schema-only` | VALID/INVALID report and exit code |

## Example (from repository root)

```bash
python3 -m tools.deliverables --project projects/chirality-piping neighborhood DEL-04-07
```
