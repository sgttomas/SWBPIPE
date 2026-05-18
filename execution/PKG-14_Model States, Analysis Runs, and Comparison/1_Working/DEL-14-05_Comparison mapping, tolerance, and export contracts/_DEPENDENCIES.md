# Dependencies: DEL-14-05 Comparison mapping, tolerance, and export contracts

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FROM_APPROVED_DAG002_PLUS_EXPLICIT_ANCHORS
- **Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-11 by dependency-extract TASK, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.

## Authority Boundary
- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed evidence surface, not an independent graph authority.
- `DAG-003` was not approved, promoted, or used as graph authority during this refresh.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| DEL-14-05-A001 | ANCHOR | UPSTREAM | OTHER | SOW-073 Deterministic comparison scope item | ACTIVE | `Datasheet.md#Identification` |
| DEL-14-05-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-016 Model states analysis runs and deterministic comparisons objective | ACTIVE | `Datasheet.md#Identification` |
| DAG-002-E0695 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0696 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0697 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0698 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0699 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0700 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0701 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0788 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-01 Immutable model state records | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0789 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-02 Analysis run records | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0790 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-04 Result export format | ACTIVE | `Specification.md#Requirements` |
| DAG-002-E0791 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE | `Specification.md#Requirements` |

## Run Notes
- **ScopePath:** `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts`
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for `SOW-073`, `OBJ-016`, `PKG-14`, and `DEL-14-05` label confirmation.
- **Approved graph authority:** `execution/_DAG/DAG-002`; status: read-only authority for preserved mirror rows.
- **Preliminary graph:** `execution/_DAG/DAG-003`; status: intentionally not used for approval or promotion.
- **Anchor doc:** `Datasheet.md`; explicit identifiers found for `DEL-14-05`, `PKG-14`, `SOW-073`, and `OBJ-016`.
- **Execution docs:** `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Guidance.md`, `_REFERENCES.md`.
- **Preservation rule:** all 11 existing DAG-002 mirror rows were preserved as ACTIVE.
- **Normalization:** local-only enum fields were normalized for dependency-extract validation: aggregate-DAG dependency labels were retained in `Notes` while `DependencyType` became `PREREQUISITE`, `AnchorType` became `NOT_APPLICABLE`, inferred explicitness became `IMPLICIT`, graph/context origins became `DECLARED`, and unknown satisfaction became `TBD`.
- **Conservative additions:** added only two explicit ANCHOR rows from deliverable/decomposition evidence; no downstream consumer rows were activated to avoid introducing local active cycles or duplicate graph-direction interpretations.
- **Warnings:** no `FLOATING_NODE` warning after adding `DEL-14-05-A001`; no `AMBIGUOUS_ANCHOR`; no `MISSING_DECOMPOSITION`.
- **Open source gaps:** tolerance defaults, exact mapping workflow authority, unmatched classification enum values, CSV columns, JSON property names, and report-section layout remain `TBD` per existing DEL-14-05 source documents.

## Run History

| Timestamp | Mode | Strictness | Consumer | Decomposition | Active Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-05-03 | SYNCHRONIZE | n/a | DAG-002 mirror | `execution/_DAG/DAG-002/DependencyEdges.csv` | 11 | Local mirror only; no anchors. |
| 2026-05-11 | UPDATE | CONSERVATIVE | RECONCILIATION | `execution/_Decomposition/SOFTWARE_DECOMP.md` | 13 | None blocking; DAG-003 not promoted. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 13 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| Class | Count |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 11 |

| Type | Count |
|---|---:|
| OTHER | 2 |
| PREREQUISITE | 11 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 9 |
| TBD | 4 |

## Downstream Handoff Notes
- For RECONCILIATION, treat this register as a normalized local evidence surface over the approved DAG-002 mirror, not as approval of DAG-003.
- The seven PKG-00 rows remain architecture-basis context evidence and should not be interpreted as direct product implementation dispatch authority.
- The four non-architecture upstream execution dependencies remain active prerequisites: `DEL-14-01`, `DEL-14-02`, `DEL-08-04`, and `DEL-02-02`.
- Known DAG-002 consumers of DEL-14-05 include `DEL-14-03`, `DEL-14-04`, `DEL-15-02`, `DEL-15-03`, `DEL-16-02`, `DEL-07-08`, and `DEL-08-06`; no local downstream rows were added during this conservative refresh.
- No blocker requires human ruling before reconciliation, but unresolved contract details remain `TBD` and should not be promoted into default tolerance values or export field claims.
