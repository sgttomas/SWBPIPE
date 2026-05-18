# Dependencies: DEL-07-03 Material, component, and rule-pack editors

## Extracted Dependency Register

- **Status:** REFRESHED_FOR_RECONCILIATION
- **Register schema:** `v3.1`
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer context:** `RECONCILIATION`
- **Source of current rows:** dependency-extract refresh using DEL-07-03 local source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Prior evidence:** existing `DAG-002` mirror row IDs and target structure preserved where matchable
- **Local register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-10

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEP-DEL-07-03-001 | ANCHOR | UPSTREAM | OTHER | SOW-021 | ACTIVE | SATISFIED |
| DAG-002-E0204 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0205 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0206 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | SATISFIED |
| DAG-002-E0207 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-05 | ACTIVE | SATISFIED |
| DAG-002-E0208 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0209 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | SATISFIED |
| DAG-002-E0210 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0489 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-01 | ACTIVE | PENDING |
| DAG-002-E0490 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-02 | ACTIVE | PENDING |
| DAG-002-E0491 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-07 | ACTIVE | PENDING |
| DAG-002-E0492 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-01 | ACTIVE | PENDING |
| DAG-002-E0493 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-04 | ACTIVE | PENDING |
| DAG-002-E0494 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-12-01 | ACTIVE | PENDING |
| DEV-001-STAGE2-DEL-07-03-PKG02-001 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE | SATISFIED |

## DEV-001 Stage 2 Addendum

- Added direct package-local PKG-02 compatibility dependency `DEV-001-STAGE2-DEL-07-03-PKG02-001` to `DEL-02-02`.
- Evidence: `core/gui/editors/engine.py` now emits `unit_contract` and per-field `unit_metadata`; `tests/test_gui_editors_contract.py` covers canonical dimension metadata and blocking missing-unit diagnostics.
- This is local technical evidence for PKG-07 review closure only. It does not edit aggregate DAG files, candidate rows, lifecycle state, or human disposition.

## Run Notes

- Default source discovery used `Specification.md` as the anchor-capable source and `Procedure.md`, `Guidance.md`, `Datasheet.md`, `_CONTEXT.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`, and `_REFERENCES.md` as deliverable-local execution/evidence inputs.
- Decomposition path was explicitly supplied as `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Parent anchor check: PASS. One ACTIVE `DependencyClass=ANCHOR` / `AnchorType=IMPLEMENTS_NODE` row exists for `SOW-021`.
- Existing local `DAG-002` mirror execution rows were treated as prior evidence, not aggregate graph authority. Matchable row IDs were preserved for reconciliation traceability.
- Write-form enum normalization was applied for the refreshed register: prior mirror-specific values such as `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `GOVERNANCE_PREDECESSOR`, `RULE_PACK_PREDECESSOR`, `SECURITY_PREDECESSOR`, `CONTEXT`, `DECOMPOSITION`, `INFERRED_DIRECT`, and `UNKNOWN` satisfaction were converted to canonical v3.1 enum values while preserving the prior classification in `Notes`.
- No candidate rows were promoted. No downstream graph authority was changed.
- No protected standards content, private data, engineering defaults, code-compliance claims, certification claims, sealing claims, approval claims, or professional acceptance claims were introduced.

## Run History

| Timestamp | Mode | Strictness | Consumer context | Decomposition | Result |
|---|---|---|---|---|---|
| 2026-04-30 10:44 | UPDATE | CONSERVATIVE | NONE | docs/_Decomposition/SOFTWARE_DECOMP.md | Prior dependency-extract run reported 14 ACTIVE rows with warning for legacy ID validator mismatch. |
| 2026-05-03 | DAG-002 mirror sync | N/A | COORDINATION | execution/_DAG/DAG-002/DependencyEdges.csv | Local register synchronized to 13 ACTIVE DAG-002 execution mirror rows, no anchor row. |
| 2026-05-10 22:44 | UPDATE | CONSERVATIVE | RECONCILIATION | execution/_Decomposition/SOFTWARE_DECOMP.md | Refreshed local dependency-extract surface: 14 ACTIVE rows, 1 parent anchor, 13 execution prerequisites, canonical enum write form. |
| 2026-05-16 | DEV-001 Stage 2 | BOUNDED | PKG-07 TASK | accepted PKG-02 contract | Added 1 ACTIVE package-local DEL-02-02 compatibility row with code/test evidence. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 15 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 9 |
| PENDING | 6 |

## Downstream Handoff Notes

- Consumer context is `RECONCILIATION`; this file is evidence for later graph reconciliation, not an approval record.
- Reconciliation should compare the added parent anchor `DEP-DEL-07-03-001` against aggregate DAG surfaces, which intentionally do not use local anchor rows as execution edges.
- Reconciliation should preserve the distinction between the canonical v3.1 enum write form in this local register and the more specific historical DAG-002 mirror classifications recorded in `Notes`.
- The six non-architecture upstream execution prerequisites remain `PENDING` until the downstream graph authority confirms maturity/closure; this TASK did not inspect other deliverable folders.
