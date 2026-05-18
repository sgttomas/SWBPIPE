# Dependencies: DEL-01-04 Professional responsibility and product-claims policy

## Generated Dependency Register

- **Status:** REFRESHED_LOCAL_EXTRACTION
- **Source of Truth:** Deliverable-local `Dependencies.csv` evidence register for this bounded refresh.
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 10 ACTIVE; 1 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary

- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed deliverable evidence surface, not an independent graph authority.
- Candidate or uncertain rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Class | Status | Count |
|---|---:|---:|
| ANCHOR | ACTIVE | 2 |
| EXECUTION | ACTIVE | 8 |
| EXECUTION | RETIRED | 1 |

| DependencyID | Class | Direction | Type | Target | Status | Confidence |
|---|---|---|---|---|---|---|
| DEP-DEL-01-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-034 Professional responsibility boundaries | ACTIVE | HIGH |
| DEP-DEL-01-04-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-011 Preserve professional responsibility | ACTIVE | HIGH |
| DAG-002-E0013 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | HIGH |
| DAG-002-E0014 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | HIGH |
| DAG-002-E0015 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | HIGH |
| DAG-002-E0016 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | HIGH |
| DAG-002-E0392 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-01 Project governance baseline | RETIRED | MEDIUM |
| DEP-DEL-01-04-E001 | EXECUTION | UPSTREAM | PREREQUISITE | OPS-CONTRACT Invariant catalog | ACTIVE | HIGH |
| DEP-DEL-01-04-E002 | EXECUTION | UPSTREAM | CONSTRAINT | OPS-IP-DATA-BOUNDARY Protected-content and private-data boundary | ACTIVE | HIGH |
| DEP-DEL-01-04-E003 | EXECUTION | DOWNSTREAM | HANDOVER | docs/PROFESSIONAL_BOUNDARY.md | ACTIVE | HIGH |
| DEP-DEL-01-04-E004 | EXECUTION | DOWNSTREAM | HANDOVER | report-notice-template | ACTIVE | HIGH |

## Run Notes

- TaskSkill: `dependency-extract`
- Mode: `UPDATE`
- Strictness: `CONSERVATIVE`
- Consumer context: `RECONCILIATION`
- Scope: `DEL-01-04`
- Run root: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- Decomposition status: found and used for anchor validation/label resolution.
- Source docs: `AUTO`
- Doc role map: `DEFAULT`
- Anchor doc selected: `Datasheet.md`
- Execution doc order selected: `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`
- Existing `DAG-002` mirror rows were preserved non-destructively. Four architecture-basis rows remain ACTIVE because `_CONTEXT.md` still explicitly lists the applicable basis IDs.
- Prior inferred row `DAG-002-E0392` was marked RETIRED, not deleted, because the current conservative local extraction did not reconfirm an explicit execution dependency from DEL-01-01.
- Legacy/noncanonical enum values from the prior synchronized register were normalized to Dependencies.csv v3.1 values.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` parent anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` parent anchor is present.
- No protected standards content, private engineering data, engineering defaults, legal conclusions, or professional/code-compliance claims were inferred.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Rows |
|---|---|---|---|---|---:|
| 2026-05-03 | SYNCHRONIZE | N/A | `execution/_DAG/DAG-002/DependencyEdges.csv` | Aggregate authority boundary applies. | 5 |
| 2026-05-10 21:40 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` found | Legacy enum normalization; prior inferred governance-predecessor row retired as unreconfirmed local evidence. | 10 |

## Lifecycle Summary

- ACTIVE rows: 10
- RETIRED rows: 1
- Closure state breakdown:
  - `SATISFIED`: 8 total; 8 ACTIVE.
  - `TBD`: 3 total; 2 ACTIVE and 1 RETIRED.
- Dependency class breakdown:
  - `ANCHOR`: 2 ACTIVE.
  - `EXECUTION`: 8 ACTIVE, 1 RETIRED.

## Downstream Handoff Notes

- For `RECONCILIATION`, treat this file as deliverable-local evidence only.
- Do not promote `DAG-002-E0392` back into aggregate authority without explicit reconciliation/change approval; it is preserved as retired evidence of the earlier inferred aggregate row.
- Downstream handoff rows for `docs/PROFESSIONAL_BOUNDARY.md` and `report-notice-template` identify anticipated artifacts and unresolved locations/wording; they do not authorize repo-level edits or issued policy claims.
- Active architecture-basis prerequisites are context-evidence rows, not proof that any PKG-00 artifact is issued.
