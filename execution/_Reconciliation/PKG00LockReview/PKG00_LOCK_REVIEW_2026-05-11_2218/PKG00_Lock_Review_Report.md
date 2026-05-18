---
doc_id: PKG00-LOCK-REVIEW-REPORT
doc_kind: reconciliation.lock_review_report
status: recommend_only
created: 2026-05-11
---

# PKG-00 Controlled Architecture Lock Review

## Executive Finding

`PKG-00` should remain a `SEMANTIC_READY` architecture-basis package for now. The disseminated basis is useful and no immediate corrective action is required. Three architecture areas contain material implementation-level TBDs that should not be treated as settled: persistence packaging/migration, external API/import-export shape, and concrete quality-gate thresholds/tooling.

This review makes no lifecycle, DAG, blocker, approval, or deliverable-local changes.

## Lock Candidates

The following basis rows are stable enough to be treated as locked downstream dispatch constraints after human acceptance of this recommendation:

| Basis | Stable Constraint |
|---|---|
| `AB-00-01` | ADR/decision-record discipline, reconsideration triggers, affected-package tracking, and code-neutral rationale. |
| `AB-00-02` | Layer/module responsibilities and no-bypass dependency direction. |
| `AB-00-03` | Command/query/job/result-envelope separation and distinct solved/checked/human-accepted states. |
| `AB-00-05` | Durable/transient GUI state separation, command-routed mutation, and scoped undo/redo. |
| `AB-00-06` | Diagnostic/result-envelope fields, warning classes, provenance, remediation, and professional-boundary controls. |

## Refinement Candidates

The following basis rows should stay authoritative for their stable subparts while their detailed implementation choices remain deferred. This does not block downstream development unless a sealed brief needs to decide one of the deferred details:

| Basis | Stable Subpart | Refinement Needed |
|---|---|---|
| `AB-00-04` | Versioned schema-governed persistence and JCS-compatible JSON payload hashing. | Physical package/container, migration framework/tooling, binary asset packaging, schema file layout. |
| `AB-00-07` | No-bypass API, adapter, plugin, validation, sandboxing, envelope, and report-control rules. | Public transport protocol, concrete import/export formats, code-generation/tooling. |
| `AB-00-08` | Layered test categories, protected-content/provenance gates, deterministic solver/rule verification gates. | CI provider, coverage thresholds, performance thresholds, exact harness libraries. |

## Downstream Propagation

The current repo has 92 `_CONTEXT.md` files and 92 `_DEPENDENCIES.md` files under deliverable working folders. Of these, 84 downstream contexts contain the SCA-001 architecture-basis injection block. PKG-00 local contexts remain separate and were not changed by this review.

Reference counts by basis ID:

| Basis | `_CONTEXT.md` refs | `_DEPENDENCIES.md` refs |
|---|---:|---:|
| `AB-00-01` | 84 | 20 |
| `AB-00-02` | 84 | 19 |
| `AB-00-03` | 62 | 13 |
| `AB-00-04` | 51 | 11 |
| `AB-00-05` | 8 | 2 |
| `AB-00-06` | 84 | 18 |
| `AB-00-07` | 64 | 14 |
| `AB-00-08` | 84 | 18 |

The propagation pattern is consistent with the decomposition: broad rows appear across most downstream contexts, while GUI-specific `AB-00-05` appears only in GUI-facing contexts.

## Contradictions and Drift

No positive PKG-00 promotion wording was found in the scanned context/dependency surfaces. References to professional/code-compliance boundaries in scanned files are prohibitive or limiting statements, not reliance claims.

Observed tension: blocker queue surfaces treat PKG-00 provider edges as satisfied by the accepted architecture baseline. That is consistent with current coordination rules, but it should continue to be described as architecture-basis satisfaction only, not implementation completion.

## Closeout Recommendation

Close this tranche as a recommend-only reconciliation review. Do not change `PKG-00` lifecycle state. No immediate architecture-hardening tranche is required. Use the decision queue in `Decision_Recommendations.md` only when downstream work needs one of the deferred details resolved.
