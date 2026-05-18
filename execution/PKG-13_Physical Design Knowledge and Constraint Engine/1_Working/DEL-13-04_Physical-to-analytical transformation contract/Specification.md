# Specification: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Initial draft from four-documents P1/P2
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
- Exact module path, dependency versions, transform-loss taxonomy, and warning code catalog until a later implementation brief resolves them with source support.

## Requirements

| ID | Requirement | Verification | Source |
|---|---|---|---|
| DEL-13-04-REQ-001 | The contract shall transform the physical model into a solver-ready analytical model deterministically. | Deterministic repeat test over the same source model, units, and configured contract version; exact fixture `TBD`. | `_CONTEXT.md` SOW-066; `docs/_Registers/ScopeLedger.csv` SOW-066 |
| DEL-13-04-REQ-002 | The contract shall record transformation warnings when physical design data cannot be represented analytically. | Warning tests covering at least one source-grounded unsupported / omitted / missing representation case; exact cases `TBD`. | `_CONTEXT.md` SOW-066; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 |
| DEL-13-04-REQ-003 | The contract shall preserve the physical model as the editable source-of-truth and produce the analytical model as a derived solver-oriented view. | Schema/contract review confirms physical model role is not overwritten or reclassified by transform output. | `docs/SPEC.md` section 3; `docs/TYPES.md` `Model` and `ModelRole` rows |
| DEL-13-04-REQ-004 | The contract shall preserve traceability from physical source records to analytical records, omissions, assumptions, warnings, or other transform outcomes. | Contract review and tests confirm each transform outcome has an explicit traceability path or a `TBD` gap is recorded. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014; `docs/TYPES.md` `TraceabilityLink` row |
| DEL-13-04-REQ-005 | Unit-bearing physical values crossing the transform boundary shall carry explicit unit metadata unless explicitly dimensionless. | Unit-aware schema/contract test; missing or ambiguous units produce diagnostics rather than supplied defaults. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| DEL-13-04-REQ-006 | Missing solve-required values shall be represented as explicit findings and shall not be silently defaulted by the transformation contract. | Negative transform test with missing solve-required data expects a deterministic diagnostic/warning. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` missing-data warning classes |
| DEL-13-04-REQ-007 | The analytical output shall target the project primary global mechanics boundary: a 3D centerline/frame model. | Contract review confirms no routine shell/solid FEA target is introduced for this deliverable. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` principles |
| DEL-13-04-REQ-008 | The contract shall not bundle protected standards content, code-specific default values, protected tables, copied formulas, proprietary catalog values, or owner standards. | Protected-content review / contribution gate; public fixtures use invented or permissive data only. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` |
| DEL-13-04-REQ-009 | The contract and its warnings shall not claim certification, sealing, approval, authentication, engineering acceptance, or code compliance. | Text/schema review for forbidden authority claims; report/result wording remains diagnostic only. | `docs/CONTRACT.md` OPS-K-AUTH-1; `INIT.md` |
| DEL-13-04-REQ-010 | Solver-facing changes resulting from this contract shall have deterministic verification tests before release. | Test-gate evidence for transform contract and warning behavior. | `docs/CONTRACT.md` OPS-K-SOLVER-1; `Dependencies.csv` row DAG-002-E0666 |
| DEL-13-04-REQ-011 | The contract shall treat upstream dependency surfaces as prerequisite context and shall not reinterpret the approved DAG-002 mirror as independent dispatch authority. | Dependency artifact review confirms DAG-002 rows are preserved as ACTIVE and not reclassified. | `_DEPENDENCIES.md` Authority Boundary; `Dependencies.csv` |

## Standards

No external engineering standard text is locally available or authorized as source material for this deliverable. The governing project standards for this draft are:

| Standard / policy surface | Applicability |
|---|---|
| `docs/CONTRACT.md` | Invariants for data boundary, missing values, professional authority, mechanics boundary, units, and solver tests. |
| `docs/SPEC.md` | Schema-first model role, unit metadata, diagnostics/warnings, and warning class context. |
| `docs/TYPES.md` | Vocabulary for model roles, traceability, diagnostics, frame kernel, supports, and load semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Protected-data and private-data handling. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 | Accepted decomposition and SOW/OBJ mapping. |

Referenced PRD source material (`PRD v0.2 Section8.3`, `FR-MOD-007`) is not locally available in this deliverable's references. Any requirement depending on the PRD beyond the SOW-066 text remains `TBD`.

## Verification

| Verification item | Required check | Current status |
|---|---|---|
| Deterministic transform | Same input and configuration produce equivalent analytical output and equivalent warnings. | Required; fixture `TBD`. |
| Warning behavior | Non-representable physical data produces deterministic transformation warnings. | Required; warning classes `TBD`. |
| Traceability coverage | Physical-to-analytical mappings, omissions, warnings, and assumptions carry traceability links or explicit gaps. | Required; schema surface `TBD`. |
| Unit metadata | Unit-bearing values are not accepted silently without explicit units. | Required by project unit contract. |
| No silent defaults | Missing solve-required physical data yields findings rather than inferred defaults. | Required by OPS-K-DATA-2. |
| Protected-content boundary | Public contract/tests do not include protected standards text, values, tables, or proprietary data. | Required by OPS-K-IP-1. |
| Professional boundary | Outputs remain diagnostics / transform artifacts, not compliance or professional-approval claims. | Required by OPS-K-AUTH-1. |
| DAG mirror preservation | All approved DAG-002 rows in `Dependencies.csv` remain ACTIVE and are not retired, deleted, or reclassified by this setup run. | Required by user instruction and `_DEPENDENCIES.md`. |

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

Implementation artifact paths, exact schemas, and test fixture names remain `TBD` until a sealed implementation task resolves them.
