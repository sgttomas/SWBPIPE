# Tool Registry

| Name | Category | Language | Purpose | Inputs | Outputs |
|------|----------|----------|---------|--------|---------|
| `list_deliverable_status` | coordination | Python 3 | Read-only discovery of deliverable-local `_STATUS.md` lifecycle values with optional DAG node context | `execution/PKG-*/1_Working/DEL-*/_STATUS.md`, `DeliverableNodes.csv`; `--dag`, `--repo-root`, `--format`, optional `--status` | stdout table, CSV, or Markdown; no repo-tracked writes |
| `maintain_dev001_coordination` | coordination | Python 3 | Legacy DEV-001 historical compatibility check and derivative blocker queue regeneration; not active work-selection authority | `DEV-001_IMPLEMENTATION_EVIDENCE.csv`, `DeliverableNodes.csv`, `DependencyEdges.csv`; `--dag`, `--repo-root`, `--check`/`--write` | `DEV-001_BLOCKER_QUEUE.csv` and `.md` in `execution/_Coordination/` and `execution/_DAG/<dag>/` only when legacy `--write` is explicitly used |

## Examples

```bash
python3 tools/coordination/list_deliverable_status.py --dag DAG-005 --format table --summary
python3 tools/coordination/list_deliverable_status.py --dag DAG-005 --status IN_PROGRESS --format csv
# Legacy historical compatibility only:
python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check
```
