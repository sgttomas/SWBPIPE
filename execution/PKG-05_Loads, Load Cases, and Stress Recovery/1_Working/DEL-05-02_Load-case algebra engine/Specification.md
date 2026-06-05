# Specification: DEL-05-02 Load-case algebra engine

## Scope

This specification records the bounded implementation evidence for a backend load-case algebra deliverable covering unit-aware user-defined combinations and result-state subtraction/ranging. The implemented crate is `core/loads/load_case_algebra`. It does not define code-specific combinations, choose a general expression grammar/library, bundle protected rule data, execute arbitrary rule-pack code, or claim code compliance.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-05-02-001 | The deliverable shall preserve unit-aware algebra for user-defined combinations. | SOW-014 |
| REQ-05-02-002 | Combination expressions shall reject or report dimensionally incompatible operations as explicit findings. | OPS-K-UNIT-1; OPS-K-DATA-2 |
| REQ-05-02-003 | Result-state subtraction and ranging shall remain mechanics-result operations and shall not imply code compliance. | DEL-05-02 description; OPS-K-MECH-2 |
| REQ-05-02-004 | Code-specific load combinations shall be supplied by user rule packs, not by bundled public defaults. | SOW-014 notes; OPS-K-DATA-1 |
| REQ-05-02-005 | The algebra surface shall not require arbitrary executable rules. | OPS-K-RULE-2 |
| REQ-05-02-006 | Missing primitive load cases, referenced result states, or rule-required inputs shall be reported explicitly, not silently defaulted. | OPS-K-DATA-2 |
| REQ-05-02-007 | Command/query/job result envelopes shall preserve mechanics solved, user-rule checked, and human-approved state separation. | AB-00-03 |
| REQ-05-02-008 | Diagnostics produced by the algebra layer shall carry provenance-compatible envelope fields when applicable. | AB-00-06 |
| REQ-05-02-009 | Deterministic tests shall cover valid unit-compatible combinations, invalid dimensional mixes, missing operands, subtraction/ranging, and no-default rule-pack boundaries before release use. | OPS-K-SOLVER-1; AB-00-08 |

## Standards

No external protected standard text is introduced by this deliverable. Governing local standards are the project invariant catalog, the sealed architecture-basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-05-02-001 | Implemented unit test `linear_combination_sums_compatible_quantities` validates explicit factors over compatible mechanics quantities. |
| REQ-05-02-002 | Implemented test `dimension_mismatch_blocks_result` verifies incompatible dimensions block the result and emit `FindingCode::DimensionMismatch`. |
| REQ-05-02-003 | Implemented tests `subtraction_requires_distinct_compatible_result_states` and `range_envelope_selects_extreme_value` verify mechanics-only subtraction and range-envelope behavior. |
| REQ-05-02-004 | Crate README/source boundary states no bundled design-code combinations, public default factors, protected standards content, or professional/code-compliance claims. |
| REQ-05-02-005 | Implementation uses explicit `AlgebraExpression` enum variants and does not include a parser, arbitrary execution path, or rule-pack expression evaluator. |
| REQ-05-02-006 | Implemented tests and finding paths cover missing operands, duplicate operands, missing result states, empty expressions, and non-finite factors as explicit findings or boundary errors. |
| REQ-05-02-007 | Implemented status propagation preserves mechanics/rule statuses, adds `HumanReviewRequired`, and blocks automatic `HumanApprovedForProject` propagation. |
| REQ-05-02-008 | Implemented result-boundary tests verify explicit unit metadata, provenance-compatible record fields, result-schema binding, payload/hash references, and rejection of load-record schema binding for result quantities. |
| REQ-05-02-009 | Targeted crate validation is `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`; project CI/release-gate inclusion remains TBD. |

## Documentation

Required deliverable-local artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`. Implementation evidence is read-only for this TASK run at `core/loads/load_case_algebra/README.md` and `core/loads/load_case_algebra/src/lib.rs`.
