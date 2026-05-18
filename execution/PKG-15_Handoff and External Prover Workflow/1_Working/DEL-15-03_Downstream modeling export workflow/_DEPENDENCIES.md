# Dependencies: DEL-15-03 Downstream modeling export workflow

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FROM_APPROVED_DAG002_PLUS_EXPLICIT_ANCHORS
- **Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 16 total; 16 ACTIVE; 0 RETIRED; 0 CANDIDATE.
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
| DEL-15-03-A001 | ANCHOR | UPSTREAM | OTHER | SOW-074 Schema-compliant handoff package generation scope item | ACTIVE | `Datasheet.md#Identification` |
| DEL-15-03-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-017 Traceable handoff and professional validation workflow objective | ACTIVE | `Datasheet.md#Identification` |
| DAG-002-E0716 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0717 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0718 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0719 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0720 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0721 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0722 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0811 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-15-01 Canonical handoff package schema and manifest | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0812 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-15-02 Target mapping and unsupported-behavior contract | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0813 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-10-02 Import/export adapter framework | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0814 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-10-03 Local FEA handoff data contract | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0815 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-12-02 Private data redaction and export controls | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0816 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-04 Physical-to-analytical transformation contract | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0817 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-05 Comparison mapping, tolerance, and export contracts | ACTIVE | `Procedure.md#Prerequisites` |

## Run Notes
- **ScopePath:** `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-03_Downstream modeling export workflow`
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for `SOW-074`, `OBJ-017`, `PKG-15`, and `DEL-15-03` label confirmation.
- **Approved graph authority:** `execution/_DAG/DAG-002`; status: read-only authority for preserved mirror rows.
- **Preliminary graph:** `execution/_DAG/DAG-003`; status: intentionally not used for approval or promotion.
- **Anchor doc:** `Datasheet.md`; explicit identifiers found for `DEL-15-03`, `PKG-15`, `SOW-074`, and `OBJ-017`.
- **Execution docs:** `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Guidance.md`, `_REFERENCES.md`.
- **Preservation rule:** all 14 existing DAG-002 mirror rows were preserved as ACTIVE.
- **Normalization:** local-only enum fields were normalized for dependency-extract validation: aggregate-DAG dependency labels were retained in `Notes` while `DependencyType` became `PREREQUISITE`, `AnchorType` became `NOT_APPLICABLE`, inferred explicitness became `IMPLICIT`, graph/context origins became `DECLARED`, and unknown satisfaction became `TBD`.
- **Conservative additions:** added only two explicit ANCHOR rows from deliverable/decomposition evidence; no downstream consumer rows were activated to avoid introducing local active cycles or duplicate graph-direction interpretations.
- **Warnings:** no `FLOATING_NODE` warning after adding `DEL-15-03-A001`; no `AMBIGUOUS_ANCHOR`; no `MISSING_DECOMPOSITION`.
- **Open source gaps:** handoff target list, canonical package container, target-specific mapping strategy, exact handoff schema fields, target mapping taxonomy, fixture format, and protected-content review evidence remain `TBD` per existing DEL-15-03 source documents.

## Run History

| Timestamp | Mode | Strictness | Consumer | Decomposition | Active Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-05-03 | SYNCHRONIZE | n/a | DAG-002 mirror | `execution/_DAG/DAG-002/DependencyEdges.csv` | 14 | Local mirror only; no anchors. |
| 2026-05-11 | UPDATE | CONSERVATIVE | RECONCILIATION | `execution/_Decomposition/SOFTWARE_DECOMP.md` | 16 | None blocking; DAG-003 not promoted. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 16 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| Class | Count |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 14 |

| Type | Count |
|---|---:|
| OTHER | 2 |
| PREREQUISITE | 14 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 9 |
| TBD | 7 |

## Downstream Handoff Notes
- For RECONCILIATION, treat this register as a normalized local evidence surface over the approved DAG-002 mirror, not as approval of DAG-003.
- The seven PKG-00 rows remain architecture-basis context evidence and should not be interpreted as direct product implementation dispatch authority.
- The seven non-architecture upstream execution dependencies remain active prerequisites: `DEL-15-01`, `DEL-15-02`, `DEL-10-02`, `DEL-10-03`, `DEL-12-02`, `DEL-13-04`, and `DEL-14-05`.
- Known DAG-002 consumers of DEL-15-03 include `DEL-15-04` and `DEL-08-06`; no local downstream rows were added during this conservative refresh.
- No blocker requires human ruling before reconciliation, but unresolved target mapping/package details remain `TBD` and should not be promoted into schema fields, defaults, or commercial-parser behavior.
