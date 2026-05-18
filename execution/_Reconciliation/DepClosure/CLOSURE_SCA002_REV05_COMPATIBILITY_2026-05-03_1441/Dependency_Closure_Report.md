---
doc_id: DEP-CLOSURE-SCA002-REV05-COMPATIBILITY-REPORT
doc_kind: audit.dependency_closure_report
status: complete_with_warnings
created: 2026-05-03
---

# Dependency Closure Report

## Verdict

Historical DEV-001 local dependency mirrors are schema-valid and acyclic, but
they are incomplete for SCA-002 revision `0.5`. They remain evidence only and
must not be used as dispatch, blocker, lifecycle, or sequencing authority for
revision `0.5`.

## Historical Local Mirror Findings

| Finding | Result |
|---|---:|
| Local dependency registers | 65 |
| Rows loaded | 624 |
| Schema-invalid registers | 0 |
| Active SCCs | 0 |
| Bidirectional active pairs | 0 |
| Orphans in existing local mirror graph | 0 |
| Hub nodes at threshold 20 | 11 |

## Revision 0.5 Gaps

| Gap | Count / IDs |
|---|---|
| Accepted deliverables | 92 |
| Historical `DAG-001` nodes | 73 |
| Missing revision `0.5` DAG nodes | 19: `DEL-07-08`, `DEL-08-06`, `DEL-13-01` through `DEL-16-04` |
| Missing package folders | 4: `PKG-13`, `PKG-14`, `PKG-15`, `PKG-16` |
| Missing contexts | 19 SCA-002 added deliverables |
| Existing contexts with retired decomposition path | 73 |
| Existing contexts with revision `0.4` references | 65 |
| Historical dispatch briefs blocked from reuse | 47 |

## Implementation Evidence Mapping

All 47 committed implementation-evidence rows still map to deliverable IDs that
exist in revision `0.5`. Two rows require targeted review before downstream use
because their direct scope/objective mappings changed under SCA-002:

- `DEL-01-04` - Professional responsibility and product-claims policy
- `DEL-02-01` - Canonical domain model schema

Revision `0.5` has 45 deliverables without committed implementation evidence.
That count includes new SCA-002 deliverables and historical deliverables that
were not implemented before SCA-002.

## Candidate Questions

The run recorded nine candidate question areas in
`Evidence/revision05_candidate_dependency_questions.csv`, covering:

- physical model vs. design knowledge/transformation;
- constraint engine prerequisites;
- model operations vs. GUI comparison workspace;
- state/run record prerequisites;
- handoff workflow dependencies;
- report sections for state/comparison/handoff;
- expanded professional-boundary governance;
- existing `DAG-001` candidate edge restatement or retirement;
- Chirality corpus compatibility memo requirements.

These are questions only. No edge was proposed, approved, promoted, or used for
blocker computation.
