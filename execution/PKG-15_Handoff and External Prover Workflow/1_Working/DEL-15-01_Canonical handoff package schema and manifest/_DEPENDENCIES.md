# Dependencies: DEL-15-01 Canonical handoff package schema and manifest

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FROM_APPROVED_DAG002_PLUS_EXPLICIT_ANCHORS
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
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
| DEL-15-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-074 Schema-compliant handoff package scope item | ACTIVE | `Datasheet.md#Identification` |
| DEL-15-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-017 Traceable handoff packages objective | ACTIVE | `Datasheet.md#Identification` |
| DAG-002-E0702 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0703 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0704 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0705 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0706 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0707 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0708 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | `_CONTEXT.md#Architecture-Basis-Injection` |
| DAG-002-E0799 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-10-03 Local FEA handoff data contract | ACTIVE | `Procedure.md#Prerequisites` |
| DAG-002-E0800 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-04 Result export format | ACTIVE | `Datasheet.md#Conditions` |
| DAG-002-E0801 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-02 Audit manifest and model hash | ACTIVE | `Datasheet.md#Conditions` |
| DAG-002-E0802 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-01 Immutable model state records | ACTIVE | `Datasheet.md#Conditions` |
| DAG-002-E0803 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-02 Analysis run records | ACTIVE | `Datasheet.md#Conditions` |
| DAG-002-E0804 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | ACTIVE | `Datasheet.md#Conditions` |

## Run Notes
- **ScopePath:** `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-01_Canonical handoff package schema and manifest`
- **Decomposition path:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for `SOW-074`, `OBJ-017`, `PKG-15`, and `DEL-15-01` label confirmation.
- **Prior graph evidence:** `execution/_DAG/DAG-002`; status: read-only evidence for preserved mirror rows. Current graph authority is `execution/_DAG/DAG-006/`.
- **Preliminary graph:** `execution/_DAG/DAG-003`; status: intentionally not used for approval or promotion.
- **Anchor doc:** `Datasheet.md`; explicit identifiers found for `DEL-15-01`, `PKG-15`, `SOW-074`, and `OBJ-017`.
- **Execution docs:** `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Guidance.md`, `_REFERENCES.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`.
- **Preservation rule:** all 13 existing DAG-002 mirror rows were preserved as ACTIVE.
- **Normalization:** local-only enum fields were normalized for dependency-extract validation: aggregate-DAG dependency labels were retained in `Notes` while `DependencyType` became `PREREQUISITE`, `AnchorType` became `NOT_APPLICABLE`, inferred explicitness became `IMPLICIT`, graph/context origins became `DECLARED`, and unknown satisfaction became `TBD`.
- **Conservative additions:** added only two explicit ANCHOR rows from deliverable/decomposition evidence; no downstream consumer rows were activated to avoid introducing local active cycles or duplicate graph-direction interpretations.
- **Warnings:** no `FLOATING_NODE` warning after adding `DEL-15-01-A001`; no `AMBIGUOUS_ANCHOR`; no `MISSING_DECOMPOSITION`.
- **Validation warning:** `tools/validation/validate_id_format.sh` rejected canonical current-project IDs `DEL-15-01`, `PKG-15`, and `SOW-074` because the helper expects older zero-padded forms (`DEL-000-00`, `PKG-000`, `SOW-0000`). The register preserves the IDs from `_CONTEXT.md`, decomposition, and registers.
- **Open source gaps:** exact schema property names, `$id` values, package container, supported target list, target-specific mapping strategy, executable fixtures, and external-tool behavior remain `TBD` per existing DEL-15-01 source documents and OI-015.

## Run History

| Timestamp | Mode | Strictness | Consumer | Decomposition | Active Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-05-03 | SYNCHRONIZE | n/a | DAG-002 mirror | `execution/_DAG/DAG-006/DependencyEdges.csv` | 13 | Local mirror only; no anchors. |
| 2026-05-11 | UPDATE | CONSERVATIVE | RECONCILIATION | `execution/_Decomposition/SOFTWARE_DECOMP.md` | 15 | ID-format helper mismatch for current project ID style; none blocking; DAG-003 not promoted. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 15 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| Class | Count |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 13 |

| Type | Count |
|---|---:|
| OTHER | 2 |
| PREREQUISITE | 13 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 9 |
| TBD | 6 |

## Downstream Handoff Notes
- For RECONCILIATION, treat this register as a normalized local evidence surface over the approved DAG-002 mirror, not as approval of DAG-003.
- The seven PKG-00 rows remain architecture-basis context evidence and should not be interpreted as direct product implementation dispatch authority.
- The six non-architecture upstream execution dependencies remain active prerequisites: `DEL-10-03`, `DEL-08-04`, `DEL-08-02`, `DEL-14-01`, `DEL-14-02`, and `DEL-02-01`.
- Known DAG-002 consumers of DEL-15-01 include `DEL-15-02`, `DEL-15-03`, `DEL-15-04`, and `DEL-08-06`; no local downstream rows were added during this conservative refresh.
- No blocker requires human ruling before reconciliation, but unresolved contract details remain `TBD` and should not be promoted into default target lists, package container choices, mapping semantics, or external prover approval claims.
