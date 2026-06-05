# Datasheet: DEL-05-02 Load-case algebra engine

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-05-02 |
| Name | Load-case algebra engine |
| Package | PKG-05 Loads, Load Cases, and Stress Recovery |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-014 |
| Objectives | OBJ-003, OBJ-005 |
| Context envelope | M |

## Attributes

| Attribute | Evidence value | Source |
|---|---|---|
| Primary subject | Unit-aware algebra for user-defined load-case combinations and result-state subtraction/ranging. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` row DEL-05-02 |
| Mechanics boundary | Combination results remain mechanics solver/result states; code acceptability is evaluated by user rule packs. | `docs/CONTRACT.md` OPS-K-MECH-2 |
| Unit discipline | Implemented algebra quantities carry `LoadDimension`; combinations, subtraction, and range envelopes block incompatible dimensions instead of converting or repairing them. | `core/loads/load_case_algebra/README.md`; `core/loads/load_case_algebra/src/lib.rs` |
| Result boundary metadata | Result-boundary export requires explicit unit metadata, provenance reference, result schema binding, payload reference, and payload-hash reference. Load-record schema bindings are rejected for result quantities. | `core/loads/load_case_algebra/README.md`; `core/loads/load_case_algebra/src/lib.rs` |
| Data boundary | Code-specific combinations, allowables, protected standards content, and project-specific rule content are not bundled defaults. | `docs/CONTRACT.md` OPS-K-DATA-1; `core/loads/load_case_algebra/README.md` |
| Rule-pack boundary | The crate does not implement the rule-pack expression evaluator or arbitrary executable rules; rule-pack integration remains outside this deliverable slice. | SOW-014; OPS-K-RULE-2; `core/loads/load_case_algebra/README.md` |
| Expression grammar/library | General expression grammar/library remains TBD. The implemented API uses explicit enum variants and terms rather than a parser or code-specific syntax. | `_CONTEXT.md` Still TBD; `core/loads/load_case_algebra/src/lib.rs` |

## Conditions

- Missing combination inputs, missing result states, duplicate operands, non-finite factors, incompatible dimensions, empty expressions, unsupported expression shapes, and status-boundary violations are represented as deterministic findings rather than silent defaults. Source: `core/loads/load_case_algebra/README.md`; `core/loads/load_case_algebra/src/lib.rs`; OPS-K-DATA-2.
- Solver/rule changes require deterministic verification before release. Current crate evidence includes targeted unit tests for the bounded algebra surface. Source: OPS-K-SOLVER-1; AB-00-08; `core/loads/load_case_algebra/src/lib.rs`.
- Result envelopes must preserve mechanics-solved, user-rule-checked, human-review-required, and human-approved distinctions; the crate rejects automatic propagation of `HumanApprovedForProject`. Source: AB-00-03; `core/loads/load_case_algebra/src/lib.rs`.
- Diagnostics and result envelopes must carry code, class, severity, source, affected object, message, remediation, and provenance where applicable. Source: AB-00-06.

## Construction

| Artifact | Description | Evidence status |
|---|---|---|
| Combination engine | `open_pipe_stress_load_case_algebra` exposes explicit linear combinations, result-state subtraction, and range envelopes over compatible mechanics quantities. | Implemented in `core/loads/load_case_algebra` |
| Expression tests | Unit tests cover compatible combinations, subtraction, range envelopes, dimension mismatch, missing/duplicate operands, non-finite factors, result boundary metadata, status propagation, and human-approval rejection. | Implemented in `core/loads/load_case_algebra/src/lib.rs` |
| Rule-pack handoff | Boundary allows explicit mechanics/result operands while excluding rule-pack expression evaluation, code-specific defaults, protected standard formulas, and professional reliance claims. | Implemented boundary; evaluator/interface details remain TBD |
| Diagnostics | Finding vocabulary covers missing operands, empty expressions, non-finite factors, dimension mismatches, duplicate operands, unsupported expression shapes, missing result states, and status-boundary violations. | Implemented finding vocabulary and deterministic blocking behavior |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- `docs/_Registers/Deliverables.csv` row DEL-05-02
- `docs/_Registers/ScopeLedger.csv` row SOW-014
- `docs/_Registers/ContextBudgetQA.csv` row DEL-05-02
- `docs/CONTRACT.md`
