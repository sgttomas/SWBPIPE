# Tool Registry

| Name | Category | Language | Purpose | Inputs | Outputs |
|------|----------|----------|---------|--------|---------|
| `list_deliverable_status` | coordination | Python 3 | Read-only discovery of deliverable-local `_STATUS.md` lifecycle values with optional DAG node context | `execution/PKG-*/1_Working/DEL-*/_STATUS.md`, `DeliverableNodes.csv`; `--dag`, `--repo-root`, `--format`, optional `--status` | stdout table, CSV, or Markdown; no repo-tracked writes |
| `dependency_type_rectification` | coordination | Python 3 | Canonicalize dependency registers, move candidate rows to non-authoritative worklists, build the `DAG-007` canonical successor from refreshed local registers, consolidate duplicate aggregate edges, and emit after-drift evidence | `--local`, `--dag007`, `--after-drift`, optional `--working-root`, `--today` | Updated local dependency artifacts, `execution/_DAG/DAG-007/`, rectification evidence CSVs |
| `dependency_semantic_refresh_fanin` | coordination | Python 3 | Inventory and validate deliverable-local dependency registers during ORCHESTRATOR semantic refresh fan-in | `--working-root`, `--output-dir`, optional `--label`, `--validate` | Fan-in register inventories, validation results, summary JSON |
| `validate_dependencies_schema` | validation | Python 3 | Validate `Dependencies.csv`/`DependencyEdges.csv` v3.1 shape plus canonical enum and row-rule semantics; `--schema-only` is reserved for historical legacy snapshots | CSV path, optional `--schema-only` | VALID/INVALID report and exit code |

## Examples

```bash
python3 tools/coordination/list_deliverable_status.py --dag DAG-007 --format table --summary
python3 tools/coordination/list_deliverable_status.py --dag DAG-007 --status IN_PROGRESS --format csv
```
