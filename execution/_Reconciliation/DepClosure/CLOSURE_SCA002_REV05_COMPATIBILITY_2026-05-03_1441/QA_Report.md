---
doc_id: DEP-CLOSURE-SCA002-REV05-COMPATIBILITY-QA
doc_kind: audit.qa_report
status: complete
created: 2026-05-03
---

# QA Report

## Deterministic Local Closure Check

`tools/coordination/analyze_dep_closure.py` wrote the closure evidence under
`Evidence/`.

| Check | Result |
|---|---:|
| Local dependency registers found | 65 |
| Rows loaded | 624 |
| Schema-valid registers | 65 |
| Schema-invalid registers | 0 |
| Evidence populated | 624 / 624 |
| Active SCCs | 0 |
| Bidirectional pairs | 0 |
| ID normalizations | 0 |

## Coverage Limitation

This check scans existing deliverable-local `Dependencies.csv` files. It does
not create missing package folders, missing contexts, missing dependency
mirrors, or a revision `0.5` graph. Therefore the clean closure result applies
only to historical DEV-001 local mirrors.

## Revision 0.5 Compatibility Evidence

Additional evidence files were generated in `Evidence/`:

- `revision05_compatibility_summary.json`
- `revision05_context_refresh_worklist.csv`
- `revision05_implementation_evidence_mapping.csv`
- `revision05_dispatch_brief_reuse_blocklist.csv`
- `revision05_scope_addition_dependency_assumptions.csv`
- `revision05_candidate_dependency_questions.csv`
- `dag001_candidate_edge_reconciliation_worklist.csv`
- `revision05_missing_control_surfaces.csv`
- `revision05_changed_existing_deliverables.csv`

## Guardrails Checked

- No `DAG-001` mutation was performed.
- No `DAG-002` proposal or approval artifact was created.
- No blocker queue was regenerated.
- No lifecycle state was changed.
- No implementation evidence was updated.
- No Type 2 dispatch brief was generated or reused.
- The Chirality app corpus remained quarantined reference material.
