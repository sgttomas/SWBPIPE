# Architecture Decision Records — Index

This is the ADR surface anticipated by deliverable **DEL-00-01 — Architecture
decision record baseline** (PKG-00 Software Architecture Runway; SOW-056,
OBJ-013; anticipated artifacts `docs/architecture/adr/index.md` and
`docs/architecture/adr/template.md`). It lists architecture decision records
and their lifecycle status. New ADRs start from
[template.md](template.md).

## Status vocabulary

Per DEL-00-01 Specification REQ-01-01, this index can list decisions in any
of four states:

| Status | Meaning |
|---|---|
| proposed | Framed for review; no human ruling cited; the choice remains `TBD` |
| accepted | A human ruling is cited; the decision is current design basis |
| superseded | Replaced by a later ADR (named in the entry) |
| rejected | Considered and declined by human ruling |

Only humans author binding rulings. An ADR in any state is a development
record, never a release-readiness, professional-approval, certification, or
engineering-code-compliance claim.

## Numbering

ADRs are numbered `ADR-NNNN` (four digits, zero-padded), starting at
`ADR-0001`, one file per ADR named `ADR-NNNN_<slug>.md`. The numbering
sequence beyond this baseline was an open `TBD` in the DEL-00-01 Datasheet;
this convention was applied by tranche `TP-SEAM-DECISION-001` (recorded as an
ASSUMPTION in that tranche's run record) and stands until a human ruling
changes it.

## Relationship to the decision log

Accepted architecture decisions that predate this surface are recorded as
`DEC-001`..`DEC-019` rows in `execution/_Decomposition/SOFTWARE_DECOMP.md`
§12. That decision log remains in force; ADRs recorded here cross-reference
their `DEC-NNN` row rather than replacing it. Backfilling earlier `DEC` rows
into ADR form is unscheduled (`TBD`).

## Records

| ID | Title | Status | Date | Human authority / ruling record |
|---|---|---|---|---|
| [ADR-0001](ADR-0001_operation_seam_engine_unification.md) | Operation-seam engine unification — wasm32 `operation_applier` as the sole browser-mode engine | accepted | 2026-06-11 | Human project authority ruling 2026-06-11, `plans/PLAN_2026-06-11_operation_seam_unification.md` §2; `DEC-020` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row `D-13` |
