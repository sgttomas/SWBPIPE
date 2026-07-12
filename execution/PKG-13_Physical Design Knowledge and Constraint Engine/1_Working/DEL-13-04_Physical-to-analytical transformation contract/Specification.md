# Specification: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Evidence refresh applied 2026-06-07
**Source posture:** Requirements below are limited to accessible local sources; unresolved particulars remain `TBD`.

## Scope

DEL-13-04 specifies the backend contract for transforming the schema-backed physical model into a solver-ready analytical model and recording traceable transformation warnings when physical design data cannot be represented analytically.

### In Scope

- A deterministic physical-to-analytical transformation contract for SOW-066.
- Warning / diagnostic behavior for omitted, unsupported, incomplete, or non-representable physical design data.
- Traceability from source physical model records to derived analytical model records, warnings, omissions, or assumptions.
- Contract-level tests for transformation warnings.
- Integration posture with architecture-basis constraints: Rust core/application services, schema-first command/query/job result envelopes, JSON Schema 2020-12 contracts, diagnostics/result envelopes, and deterministic tests where applicable, per `_CONTEXT.md`.

### Out of Scope

- Protected standards data, code-specific defaults, protected tables, copied formulas, owner standards, proprietary catalog values, or private project data.
- Final engineering acceptance, code compliance, certification, sealing, approval, or professional reliance claims.
- Owner standards or final acceptance logic for physical design constraints, per PKG-13 package exclusions.
- Final transform-loss taxonomy, release thresholds, dependency versions, external prover behavior, GUI/runtime/API integration, persisted/handoff readiness, broader physical-record coverage, and human/professional acceptance.

## Implementation Evidence

Current implementation evidence exists for the DEL-13-04 transform and internal solver-boundary adapter:

- `core/model_transform/physical_to_analytical/contract.py` implements `transform_physical_to_analytical(...)`, `TransformSettings`, `TransformResult`, deterministic ordering, `analytical_solver_model` output, `PTA-*` diagnostics, and physical-to-analytical traceability links.
- `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` implements the internal `adapt_analytical_solver_model(...)` DTO boundary for strict `analytical_solver_model` mappings. It emits deterministic node DTOs, straight-pipe connectivity, property bindings, support targets, load-case records/applications, adapter DTO identity/hash/source-chain records, `load_case_diagnostics`, and adapter diagnostics.
- `fixtures/domain/invented_physical_source_of_truth_model.json` is the canonical invented physical source-of-truth fixture for current DEL-13-04 tests. It contains public-permissive invented nodes, straight-pipe element, governed `y_reference`, material/section quantities, support, strict load records, visible assumption/diagnostic evidence, and traceability.
- `tests/test_physical_to_analytical_transform.py` covers deterministic transform behavior, source model preservation, canonical fixture/schema validation, traceability, unit metadata failures, unsupported physical-record/component behavior, strict load metadata, noncanonical quantity dimensions, and forbidden authority-claim text scanning.
- `tests/test_analytical_solver_boundary_adapter.py` covers deterministic solver-boundary DTO emission, missing-property no-default diagnostics, noncanonical load semantic rejection, invalid orientation diagnostics, load span/runtime diagnostics, adapter DTO hash/source-chain evidence, and analytical-model role enforcement.

This evidence does not implement a GUI command path, runtime service envelope, public API, persistence/handoff package readiness, external prover behavior, full release threshold policy, or human/professional acceptance.

## Requirements

| ID | Requirement | Verification | Source |
|---|---|---|---|
| DEL-13-04-REQ-001 | The contract shall transform the physical model into a solver-ready analytical model deterministically. | `tests/test_physical_to_analytical_transform.py` repeats transform over the canonical fixture and deep-copied inputs and expects identical output. | `_CONTEXT.md` SOW-066; `docs/_Registers/ScopeLedger.csv` SOW-066; `core/model_transform/physical_to_analytical/contract.py` |
| DEL-13-04-REQ-002 | The contract shall record transformation warnings or diagnostics when physical design data cannot be represented analytically. | Transform tests cover missing units, unsupported element/component data, unresolved references, noncanonical dimensions, and unresolved load dimensions. Adapter tests cover unsupported/noncanonical load semantics, invalid orientation, missing properties, unresolved targets, and nonfinite quantities. | `_CONTEXT.md` SOW-066; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04; `tests/test_physical_to_analytical_transform.py`; `tests/test_analytical_solver_boundary_adapter.py` |
| DEL-13-04-REQ-003 | The contract shall preserve the physical model as the editable source-of-truth and produce the analytical model as a derived solver-oriented view. | Schema/contract review confirms physical model role is not overwritten or reclassified by transform output. | `docs/SPEC.md` section 3; `docs/TYPES.md` `Model` and `ModelRole` rows |
| DEL-13-04-REQ-004 | The contract shall preserve traceability from physical source records to analytical records, omissions, assumptions, warnings, or other transform outcomes. | Transform tests assert physical-to-analytical links for nodes, elements, materials, sections, supports, load cases, components, and diagnostics; adapter tests assert DTO `source_chain` and hash anchors for emitted load applications. Field-level scalar traceability remains `TBD`. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014; `docs/TYPES.md` `TraceabilityLink` row; `core/model_transform/physical_to_analytical/contract.py`; `_solver_boundary_adapter.py` |
| DEL-13-04-REQ-005 | Unit-bearing physical values crossing the transform boundary shall carry explicit unit metadata unless explicitly dimensionless. | Transform tests reject missing, empty, `TBD`, and noncanonical dimensions; adapter tests require finite `Quantity` records with expected dimensions and do not convert or infer units. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| DEL-13-04-REQ-006 | Missing solve-required values shall be represented as explicit findings and shall not be silently defaulted by the transformation contract. | Negative transform and adapter tests expect deterministic `PTA-*` or `ASBA-*` diagnostics instead of defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` missing-data warning classes |
| DEL-13-04-REQ-007 | The analytical output shall target the project primary global mechanics boundary: a 3D centerline/frame model. | Contract review confirms no routine shell/solid FEA target is introduced for this deliverable. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` principles |
| DEL-13-04-REQ-008 | The contract shall not bundle protected standards content, code-specific default values, protected tables, copied formulas, proprietary catalog values, or owner standards. | Protected-content review / contribution gate; public fixtures use invented or permissive data only. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` |
| DEL-13-04-REQ-009 | The contract and its warnings shall not claim certification, sealing, approval, authentication, engineering acceptance, or code compliance. | Text/schema review for forbidden authority claims; report/result wording remains diagnostic only. | `docs/CONTRACT.md` OPS-K-AUTH-1; `INIT.md` |
| DEL-13-04-REQ-010 | Solver-facing changes resulting from this contract shall have deterministic verification tests before release. | Current focused tests are executable with `python3 tests/test_physical_to_analytical_transform.py`, `python3 tests/test_analytical_solver_boundary_adapter.py`, and joint `pytest`; release thresholds remain `TBD`. | `docs/CONTRACT.md` OPS-K-SOLVER-1; `Dependencies.csv` row DAG-002-E0666 |
| DEL-13-04-REQ-011 | The contract shall treat upstream dependency surfaces as prerequisite context and shall not reinterpret the local dependency mirror as independent dispatch authority. | Dependency artifact review confirms current local dependency rows remain evidence under DAG-006 coordination; this refresh does not edit dependency files. | `_DEPENDENCIES.md` Authority Boundary; `Dependencies.csv`; `_COORDINATION.md` |
| DEL-13-04-REQ-012 | The internal solver-boundary adapter shall accept only strict `analytical_solver_model` mappings and shall not become a public API, GUI/runtime path, external prover path, or final solver acceptance claim. | Adapter tests enforce analytical-model role, strict load union handling, adapter DTO evidence, and diagnostics for unsupported runtime conditions. | `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`; `tests/test_analytical_solver_boundary_adapter.py`; `MEMORY.md` |

## Standards

No external engineering standard text is locally available or authorized as source material for this deliverable. The governing project standards for this draft are:

| Standard / policy surface | Applicability |
|---|---|
| `docs/CONTRACT.md` | Invariants for data boundary, missing values, professional authority, mechanics boundary, units, and solver tests. |
| `docs/SPEC.md` | Schema-first model role, unit metadata, diagnostics/warnings, and warning class context. |
| `docs/TYPES.md` | Vocabulary for model roles, traceability, diagnostics, frame kernel, supports, and load semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Protected-data and private-data handling. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | Accepted decomposition and SOW/OBJ mapping. |
| `core/model_transform/physical_to_analytical/contract.py` | Current transform implementation. |
| `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` | Current internal adapter implementation. |
| `tests/test_physical_to_analytical_transform.py` and `tests/test_analytical_solver_boundary_adapter.py` | Current focused verification evidence. |

Referenced PRD source material (`PRD v0.2 Section8.3`, `FR-MOD-007`) is not locally available in this deliverable's references. Any requirement depending on the PRD beyond the SOW-066 text remains `TBD`.

## Verification

| Verification item | Required check | Current status |
|---|---|---|
| Deterministic transform | Same input and configuration produce equivalent analytical output and equivalent warnings/diagnostics. | Implemented in `tests/test_physical_to_analytical_transform.py` using `fixtures/domain/invented_physical_source_of_truth_model.json`. |
| Warning/diagnostic behavior | Non-representable physical data produces deterministic transformation diagnostics. | Implemented for current `PTA-*` and `ASBA-*` coverage; final release taxonomy remains `TBD`. |
| Traceability coverage | Physical-to-analytical mappings, omissions, warnings, assumptions, adapter DTO hashes, and source chains carry traceability links or explicit gaps. | Implemented at object/DTO level; field-level scalar traceability and full runtime result trace-chain production remain `TBD`. |
| Unit metadata | Unit-bearing values are not accepted silently without explicit units. | Implemented in transform and adapter tests; no unit conversion is performed. |
| No silent defaults | Missing solve-required physical data yields findings rather than inferred defaults. | Implemented in transform and adapter tests. |
| Protected-content boundary | Public contract/tests do not include protected standards text, values, tables, or proprietary data. | Required by OPS-K-IP-1. |
| Professional boundary | Outputs remain diagnostics / transform artifacts, not compliance or professional-approval claims. | Required by OPS-K-AUTH-1. |
| DAG mirror preservation | All current local dependency rows remain unchanged by this evidence refresh. | Required by user instruction and `_DEPENDENCIES.md`; dependency files are outside this write scope. |
| Internal adapter DTO evidence | Strict analytical solver-boundary mappings produce deterministic DTOs or diagnostics. | Implemented in `tests/test_analytical_solver_boundary_adapter.py`. |

## Documentation

Required local artifacts for this setup pass:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`

Anticipated implementation artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:

- physical-to-analytical transform contract
- transform warning tests

Current implementation artifact paths and focused test fixture names are no longer `TBD`; they are listed above. Remaining `TBD` items are transform-loss taxonomy, release thresholds, external prover behavior, GUI/runtime/API integration, persisted/handoff readiness, broader physical-record coverage, human acceptance, and professional/code-compliance boundaries.

## D-41 R5 T2B PDU-047 Evidence Disposition (2026-07-12)

The TP-PHYS-015 production section-property binding is relevant downstream evidence, but it does not independently validate REQ-007's full 3D centerline/frame target suitability. That broader mechanics-validation basis remains held. This tranche changes neither the transform contract nor its output.

## D-41 R5 T2C PDU-023 E6 Evidence (2026-07-12)

REQ-004 now has a minimal field-scalar link for valid copied component geometry quantities. Each link pairs source and target scalar paths on the existing object references. Incomplete quantity metadata is diagnosed and emits no scalar link. Runtime result-envelope continuation remains held.
