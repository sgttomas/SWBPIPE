# Procedure: DEL-05-02 Load-case algebra engine

## Purpose

Define the procedure for maintaining and verifying the bounded load-case algebra implementation evidence without introducing code-specific defaults, protected content, or professional/code-compliance claims.

## Prerequisites

- Sealed deliverable context in `_CONTEXT.md`.
- Governing invariants in `docs/CONTRACT.md`.
- Decomposition and register rows for DEL-05-02, SOW-014, OBJ-003, and OBJ-005.
- Read-only implementation evidence in `core/loads/load_case_algebra/README.md` and `core/loads/load_case_algebra/src/lib.rs`.
- Human authorization before implementing a general expression grammar/library, code-specific load combinations/defaults, protected rule content, or rule-pack execution behavior.

## Steps

1. Confirm the deliverable scope is limited to unit-aware load-case algebra, user-defined explicit combinations, result-state subtraction, and range envelopes.
2. Confirm exclusions: no code-specific load combinations/defaults, no protected standards content, no arbitrary executable rules, no rule-pack evaluator, and no certification or professional-reliance claims.
3. Maintain the data boundary: code-specific combinations are supplied by user rule packs and remain outside bundled public defaults.
4. Verify deterministic implementation evidence for compatible-unit combinations, incompatible dimensions, missing operands, duplicate operands, missing result states, non-finite factors, subtraction/ranging, result-boundary metadata, status propagation, and human-approval rejection.
5. Record unknowns as `TBD`, including general expression grammar/library, final rule-pack evaluator/interface behavior, final result-envelope/persistence integration, and release/CI gate policy.
6. Preserve deliverable evidence in the four documents, semantic files, dependency artifacts, status history, memory, and run records.

## Verification

| Check | Expected result |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| Implementation evidence present | `core/loads/load_case_algebra/README.md`, `Cargo.toml`, and `src/lib.rs` exist and remain read-only for documentation-alignment runs. |
| Algebra crate tests | `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml` passes before claiming current implementation evidence is validated. |
| Data boundary preserved | No code-specific combinations, allowables, or protected standard formulas are introduced. |
| Dependency register unchanged | `Dependencies.csv`, `_DEPENDENCIES.md`, and `_STATUS.md` remain read-only unless a separate authorized dependency or status task permits edits. |
| Status safe | `_STATUS.md` is not `ISSUED`. |

## Records

- Four-document kit.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`.
- `Dependencies.csv` and `_DEPENDENCIES.md`.
- `MEMORY.md`.
- `_run_records/` entries for the requested TASK sequence.
