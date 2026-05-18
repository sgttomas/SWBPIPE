# Procedure: DEL-04-03 Linear support and restraint models

## Purpose

Use this procedure to turn the sealed DEL-04-03 setup into a future implementation brief and local acceptance checks without crossing into solver implementation during setup.

## Prerequisites

- Confirm the active scope is DEL-04-03 / PKG-04 and SOW-011 only.
- Read `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, and the SOFTWARE_DECOMP rows for SOW-011, OBJ-003, DEL-04-03, DEL-04-04, and AB-00-01/02/03/06/08.
- Confirm protected data and professional-compliance hard stops before drafting support examples or tests.
- Confirm any coordinate convention, stiffness representation, solver-library, or boundary-condition assembly decision is either already accepted or marked `TBD`.

## Steps

1. Define the support family taxonomy for anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data.
2. For each family, identify the target reference, affected direction or degree of freedom, unit-bearing numerical fields, and provenance/source field needs.
3. Record linear-only behavior and route nonlinear one-way, gap, lift-off, and friction behavior to DEL-04-04 unless a later sealed brief changes scope.
4. Map support data to the six node degrees of freedom while preserving the coordinate-frame convention as `TBD` until decided.
5. Define missing-data and invalid-data diagnostics with AB-00-06 fields.
6. Add a future review checkpoint for singularity, overconstraint, and invalid-restraint diagnostics with DEL-04-06 without editing that deliverable from this setup task.
7. Draft linear restraint tests using original or permissively licensed fixture data only.
8. Verify that results and reports do not claim code compliance, certification, sealing, professional approval, or standards-body endorsement.
9. Record any resolved architecture or representation choices through the approved decision-record path. The exact repo-level ADR location is outside this setup task and remains `TBD`.

## Verification

- Four-document kit exists and uses consistent terminology for linear support/restraint models.
- Required support-data gaps are represented as `TBD` or diagnostics, not defaults.
- Test plan references deterministic mechanics verification and unit/dimensional checks for support stiffness and imposed displacement values.
- No protected standards text, copied formulas, protected tables, proprietary vendor data, or certification/compliance claims are introduced.
- Nonlinear support behavior is not implemented or specified as part of this linear deliverable.

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` setup artifacts.
- `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
- `_run_records/TASK_RUN_*.md` entries for four-documents P1/P2, semantic matrix, lens register, four-documents P3, and dependency extraction.
