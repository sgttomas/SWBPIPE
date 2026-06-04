# Tool Registry

| Name | Category | Language | Purpose | Inputs | Outputs |
|------|----------|----------|---------|--------|---------|
| `list_deliverable_status` | coordination | Python 3 | Read-only discovery of deliverable-local `_STATUS.md` lifecycle values with optional DAG node context | `execution/PKG-*/1_Working/DEL-*/_STATUS.md`, `DeliverableNodes.csv`; `--dag`, `--repo-root`, `--format`, optional `--status` | stdout table, CSV, or Markdown; no repo-tracked writes |

## Examples

```bash
python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary
python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --status IN_PROGRESS --format csv
```
