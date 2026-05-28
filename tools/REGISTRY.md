# Tool Registry

| Name | Category | Language | Purpose | Inputs | Outputs |
|------|----------|----------|---------|--------|---------|
| `maintain_dev001_coordination` | coordination | Python 3 | Validate DEV-001 implementation evidence and regenerate derivative blocker queue mirrors | `DEV-001_IMPLEMENTATION_EVIDENCE.csv`, `DeliverableNodes.csv`, `DependencyEdges.csv`; `--dag`, `--repo-root`, `--check`/`--write` | `DEV-001_BLOCKER_QUEUE.csv` and `.md` in `execution/_Coordination/` and `execution/_DAG/<dag>/` |

## Examples

```bash
python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check
python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --write
```
