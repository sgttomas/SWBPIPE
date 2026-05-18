---
doc_id: AUDIT-DEP-CLOSURE-RUN-SUMMARY-PKG-12-POST-SETUP-2026-04-30
doc_kind: audit.run_summary
run_status: WARNINGS
created: 2026-04-30
---

# Dependency Closure Run Summary

## Tool Output

The deterministic analyzer found:

- `Dependencies.csv` files: 65
- Total dependency rows: 503
- Schema-valid files: 65
- Schema-invalid files: 0
- `IMPLEMENTS_NODE` anchors present: 65
- Missing `IMPLEMENTS_NODE` anchors: 0
- Evidence coverage: 503/503 rows
- Graph nodes: 62
- Graph edges: 165
- Orphans: 11
- Strongly connected components larger than 1: 1
- Bidirectional pairs: 3
- Hub nodes above threshold: 0
- ID normalizations: 0

## Verdict

`WARNINGS`.

PKG-12 dependency schemas and anchors pass. Warnings remain for proposed dependency rows, two isolated PKG-12 deliverables in the execution graph, and global non-PKG-12 graph cycles/bidirectional pairs.
