# Sealed Brief — A2-09 Mechanics Observation API Author

RequestedBy: `N1-WORKING-ITEMS-PKG09-DEL0901`

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `N1-WORKING-ITEMS-PKG09-DEL0901`

ChildInstanceID: `A2-09-AUTHOR`

Agent form: fresh non-delegating ephemeral Agent 2

PackageID: `PKG-09`

DeliverableID: `DEL-09-01`

ScopePath: `${WORKING_ROOT}/validation/benchmarks/mechanics/src/lib.rs`

## Objective

Add one suite-owned additive value-addressable observation API to the mechanics
benchmark crate and in-file tests proving complete observations for exactly the
14 formerly blocked current cases and their 115 named values. Observations must
be computed from existing public-original mechanics/fixture logic, never copied
from or derived from `expected_values`. Preserve inventory, expected values,
encoded comparison predicate, provenance, all existing behavior and tests.

## Accepted basis and dependencies

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`; set
  `WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping`.
- Branch/HEAD: `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- Adopted candidate SHA-256:
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
- Baseline `lib.rs` SHA-256:
  `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26`.
- Frozen current runner output SHA-256:
  `e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.
- C-B SHA-256:
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.
- Read root/project AGENTS, adopted candidate, mechanics Cargo.toml and source,
  relevant open solver/kernel APIs, DEL-09-01 SOW/status/memory/context/
  dependencies/recent run records, and software workflow profile.

## Allowed tools

Read-only shell/Git/hashing/search; `apply_patch` only for allowed writes;
`cargo fmt --check` and `cargo test` for the mechanics manifest. Externalize
`CARGO_TARGET_DIR`, `CARGO_HOME` (using an existing cache only), and temporary
outputs beneath a new `/private/tmp` directory. No network or provisioning.
Do not delegate.

## Allowed write targets

- `projects/chirality-piping/validation/benchmarks/mechanics/src/lib.rs`
- this instance's `RETURN.md`
- this instance's `STATUS.json`

## Acceptance criteria

1. Public additive API has deterministic fixture-owned names/order and
   structured unknown/execution/nonfinite/incomplete/duplicate/name-mismatch
   failure semantics appropriate to the suite boundary.
2. All 14 named cases produce exactly all 115 expected observation names;
   every value is finite; no missing/extra/duplicate names.
3. Source audit and tests prove `expected_values` are not used as observation
   values. Existing inventory, expected values, predicates, provenance, and
   existing behavior remain unchanged.
4. Mechanics format check and full crate test suite PASS.
5. C-B remains exact and no mechanics comparison reads it.
6. Only the allowed source and instance records change. Begin with ignored
   inventory baseline; if any repository side effect appears, stop and
   inventory it without deletion.

Return the exact API signature/semantics, after-hash/blob, coverage table,
commands/results, containment proof, side-effect inventory, blockers, and
rerun triggers. HOLD rather than widen scope. Do not stage, commit, push, PR,
merge, fetch, rebase, reset, clean, delete, or edit any other path.
