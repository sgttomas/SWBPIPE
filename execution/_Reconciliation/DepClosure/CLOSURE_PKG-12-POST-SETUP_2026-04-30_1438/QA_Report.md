---
doc_id: AUDIT-DEP-CLOSURE-QA-PKG-12-POST-SETUP-2026-04-30
doc_kind: audit.qa_report
status: completed
created: 2026-04-30
---

# Dependency Closure QA Report

## Checks

| Check | Result | Evidence |
|---|---|---|
| Analyzer completed | PASS | `Evidence/closure_summary.json` |
| Schema validity | PASS | 65 valid, 0 invalid |
| Anchor coverage | PASS | 65 `IMPLEMENTS_NODE` present, 0 missing |
| Evidence coverage | PASS | 503/503 rows |
| ID normalization | PASS | 0 normalization events |
| Orphans | WARNING | 11 total; PKG-12 includes DEL-12-03 and DEL-12-05 |
| Cycles | WARNING | One SCC of size 9 outside PKG-12 |
| Bidirectional pairs | WARNING | Three pairs outside PKG-12 |
| Hub threshold | PASS | 0 hubs |

## Notes

The analyzer is full-workspace only. PKG-12 interpretation is separated from global graph warnings in the report and issue log.
