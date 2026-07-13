# Guidance: DEL-05-02 Load-case algebra engine

## Purpose

This deliverable documents the bounded load-case algebra implementation that combines explicit mechanics quantities and result states in a unit-aware way while leaving code-specific combinations and acceptance decisions to user-owned rule packs and human authority.

## Principles

- Keep algebra mechanics-focused: addition, scaling, subtraction, and ranging describe load/result relationships, not code compliance.
- Treat units and dimensions as first-class constraints; an expression that mixes incompatible dimensions is a finding, not a repair opportunity.
- Keep code-specific combinations outside the product baseline unless supplied by lawful/private user rule packs.
- Keep rule-pack evaluation declarative and sandboxed; this implementation does not include arbitrary executable rules or the rule-pack expression evaluator.
- Preserve status separation between mechanics solved, rule-pack checked, and human-approved outputs.

## Considerations

| Topic | Guidance | Evidence |
|---|---|---|
| Expression grammar | General grammar/library remains TBD. Current implementation uses explicit `AlgebraExpression` variants and `CombinationTerm` values, not a parser or code-specific syntax. | `_CONTEXT.md` Still TBD; `core/loads/load_case_algebra/src/lib.rs`; OPS-K-RULE-2 |
| Primitive load and result dependencies | Inputs must arrive as explicit mechanics/result operands with dimensions and boundary metadata already governed upstream. The DEL-05-01 and DEL-05-04 dependency rows are `SATISFIED` under the recorded 2026-06-05 blocker-closure ruling. | `Dependencies.csv`; `_DEPENDENCIES.md`; `core/loads/load_case_algebra/README.md` |
| Rule-pack combinations | User rule packs may supply code-specific combinations through a governed interface, but the crate does not provide code-specific defaults or a rule-pack evaluator. The DEL-06-02 evaluator interface remains TBD/low-confidence. | SOW-014; OBJ-005; `_DEPENDENCIES.md` |
| Diagnostics | Missing operands, empty expressions, non-finite factors, dimensional mismatch, duplicate operands, unsupported expression shapes, missing result states, and status-boundary violations are deterministic findings or boundary errors. | OPS-K-DATA-2; AB-00-06; `core/loads/load_case_algebra/src/lib.rs` |
| Result boundary metadata | Boundary export requires explicit unit metadata, result schema binding, provenance reference, payload reference, and payload-hash reference; load-record schema binding is rejected for result quantities. | `core/loads/load_case_algebra/README.md`; `core/loads/load_case_algebra/src/lib.rs` |

## Trade-offs

- A more expressive grammar can help users define combinations, but increases validation and sandboxing risk. Grammar choice remains TBD.
- Hard-coded combinations might appear convenient, but would violate the data boundary for code-specific and project-specific rule content.
- Implemented subtraction/ranging support deliberately stays deterministic and mechanics-focused instead of broadening into expression-language flexibility.

## Examples

Examples remain intentionally abstract. No code-specific load combination, allowable, or protected standard formula is provided in this deliverable.

## Conflict Table (for human ruling)

| Conflict ID | Topic | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified during this implementation-evidence alignment. | N/A | N/A |
