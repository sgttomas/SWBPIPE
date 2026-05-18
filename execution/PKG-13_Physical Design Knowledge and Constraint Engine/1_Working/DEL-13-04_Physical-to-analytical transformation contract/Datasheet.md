# Datasheet: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Initial draft from four-documents P1/P2
**Source posture:** Conservative; unsupported particulars are marked `TBD` or `ASSUMPTION`.

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-13-04 | `_CONTEXT.md` |
| Name | Physical-to-analytical transformation contract | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-04 |
| Package ID | PKG-13 | `_CONTEXT.md` |
| Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 |
| Deliverable type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-04 |
| Scope coverage | SOW-066 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-066 |
| Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective mapping |
| Context envelope | L, WATCH | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-13-04 |

## Attributes

| Attribute | Current value | Source / note |
|---|---|---|
| Primary function | Derive a solver-ready analytical model from the physical model. | SOW-066 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Determinism requirement | Transformation is deterministic. | SOW-066 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-066 |
| Warning obligation | Transformation warnings are recorded when physical design data cannot be represented analytically. | SOW-066 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 row |
| Traceability obligation | Transformation traceability is part of OBJ-014 and DEL-13-04 decomposition context. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014 and DEL-13-04 |
| Physical model role | Physical source-of-truth model anchors editable design data. | `docs/SPEC.md` section 3; `docs/TYPES.md` model registry |
| Analytical model role | Solver-ready idealization / analysis basis derived from the physical model. | `execution/_Decomposition/SOFTWARE_DECOMP.md` glossary row "Analytical Model" |
| Target mechanics boundary | Primary global analysis model is a 3D centerline/frame model. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` project principles |
| Unit handling | Unit-bearing physical values crossing boundaries require explicit unit metadata. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| Missing required values | Missing solve-required values are explicit findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` missing-data warning classes |
| Protected-data boundary | Public artifacts must not bundle protected standards data, tables, or proprietary values. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` |
| Professional boundary | Software must not claim certification, sealing, approval, authentication, or code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `INIT.md` |
| Implementation module path | TBD | No source in this deliverable fixes module or file path. |
| Exact transform-loss taxonomy | TBD | Open issue OI-012 notes that loss classes require technical architecture detail. |
| Exact dependency versions | TBD | `_CONTEXT.md` Architecture Basis Injection "Still TBD". |

## Conditions

### Upstream Inputs

The local `Dependencies.csv` is an approved DAG-002 mirror/evidence surface. It lists these ACTIVE upstream dependencies for DEL-13-04:

| Target | Relationship summary | Source |
|---|---|---|
| DEL-00-01 Architecture decision record baseline | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0660 |
| DEL-00-02 Repository and module boundary architecture | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0661 |
| DEL-00-03 Application service command-query-job model | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0662 |
| DEL-00-04 Persistence and schema versioning architecture | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0663 |
| DEL-00-06 Diagnostics, warning, and result-envelope contract | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0664 |
| DEL-00-07 API boundary and adapter contract map | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0665 |
| DEL-00-08 Layered software test and acceptance strategy | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0666 |
| DEL-02-01 Canonical domain model schema | Physical-to-analytical transform starts from the canonical physical/domain model. | `Dependencies.csv` row DAG-002-E0772 |
| DEL-13-01 Design knowledge schema and provenance model | Transform consumes design-knowledge schema. | `Dependencies.csv` row DAG-002-E0773 |
| DEL-13-02 Constraint entity and provenance model | Transform consumes constraint entities. | `Dependencies.csv` row DAG-002-E0774 |
| DEL-13-03 Constraint validation engine | Transform consumes constraint validation. | `Dependencies.csv` row DAG-002-E0775 |
| DEL-04-01 3D frame stiffness kernel | Transform targets the frame kernel model. | `Dependencies.csv` row DAG-002-E0776 |
| DEL-04-03 Linear support and restraint models | Transform needs support/boundary-condition modeling. | `Dependencies.csv` row DAG-002-E0777 |
| DEL-05-01 Primitive load case engine | Transform needs primitive load semantics. | `Dependencies.csv` row DAG-002-E0778 |

### Boundary Conditions

| Condition | Value |
|---|---|
| Engineering standards text availability | Not locally available for this deliverable; do not derive clause-level requirements. |
| PRD v0.2 source availability | Referenced by SOW-066 but not present in `_REFERENCES.md` as a local source; PRD-derived particulars remain `TBD`. |
| Public data policy | No protected standards text, copied formulas, protected tables, protected examples, proprietary commercial data, or code-specific public defaults. |
| Solver acceptance policy | Deterministic mechanics tests are required before release; code-compliance acceptance remains outside software authority. |

## Construction

The deliverable should materialize as a contract plus transform warning tests, as stated in `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`.

| Construct | Required / expected content | Status |
|---|---|---|
| Transform contract | Inputs, outputs, deterministic behavior, traceability, warning/diagnostic behavior, and unsupported-data handling for physical-to-analytical conversion. | Draft scope only; implementation contract file path `TBD`. |
| Analytical model output | Solver-ready analytical model compatible with upstream frame-kernel, support/restraint, and load semantics. | Target surface identified; exact schema details `TBD` until upstream contracts are read in a sealed implementation task. |
| Warning records | Deterministic warnings or diagnostics for physical design data that cannot be represented analytically. | Required by SOW-066; warning code taxonomy `TBD`. |
| Traceability records | Links from physical/source objects to analytical output objects, omissions, assumptions, or warnings. | Required by OBJ-014 context; exact record structure `TBD`. |
| Tests | Transform warning tests. | Required artifact; exact fixtures and assertions `TBD`. |

## References

- `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and accepted decomposition reference.
- `_REFERENCES.md` - governing reference list and source boundary.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-002 mirror/evidence surface.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.5 package, scope, objective, and deliverable entries.
- `docs/_Registers/Deliverables.csv` - row DEL-13-04.
- `docs/_Registers/ScopeLedger.csv` - row SOW-066.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-13-04.
- `docs/CONTRACT.md` - invariant catalog, especially OPS-K-IP-1, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-MECH-1, OPS-K-UNIT-1, OPS-K-SOLVER-1.
- `docs/SPEC.md` - physical model source-of-truth, unit contract, and missing-data warning classes.
- `docs/TYPES.md` - model, model role, traceability, diagnostic, and mechanics-boundary registry meanings.
- `docs/IP_AND_DATA_BOUNDARY.md` - protected-content handling policy.
- `INIT.md` - project principles and stop rules.
