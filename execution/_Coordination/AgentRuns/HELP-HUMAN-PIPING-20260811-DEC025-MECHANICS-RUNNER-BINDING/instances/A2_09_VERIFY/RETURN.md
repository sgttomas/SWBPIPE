# Return — A2-09 Fresh Observation API Verifier

Verdict: `PASS`

## Exact identities

- Branch / HEAD: `codex/piping-dec025-case-runner-binding-20260811` / `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- Adopted candidate SHA-256: `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
- Accepted `lib.rs` SHA-256: `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`.
- Accepted `lib.rs` Git blob: `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`.
- Baseline `lib.rs` SHA-256 / blob: `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26` / `eb65e53075110995a4ddcd93b4181b15392f91d5`.
- Frozen runner output SHA-256 remains `e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.
- C-B SHA-256 remains `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

## Independent verification

The two additive public APIs are:

- `fixture_observations(fixture_id: &str) -> Result<Vec<MechanicsObservation>, MechanicsObservationError>`
- `fixture_recorded_comparison_holds(fixture_id: &str, value_name: &str, observed: f64, recorded: f64) -> bool`

Independent inspection and the full test suite verify the exact frozen case order and per-case counts `2, 2, 6, 3, 3, 2, 1, 2, 21, 42, 12, 6, 8, 5`, totaling `14/14` cases and `115/115` observations. Each returned name is in fixture-owned expected-name order, every value is finite, no name is missing, extra, or duplicated, and all 115 comparisons pass through the suite-owned predicate dispatch.

The producer region (`fixture_observations` and its helpers) has zero reads of `expected_values`, `recorded.value`, or `expected.value`. Expected records are used only for completeness/name auditing and as comparison arguments. Observations are produced from existing solver, adapter, load-generation, support-preparation, and fixture-input paths. No expected numeric value is used as an observation.

Unknown fixture/value names and non-finite comparison inputs fail closed. Structured observation errors cover unknown fixture, execution failure, non-finite value, duplicate name, incomplete set, and extra/name mismatch. The API is deterministic because dispatch, producer iteration, and returned ordering are fixed. Specialized DEC-092, expansion-loop, curved-bend distributed, and curved-bend pressure predicates reuse existing suite constants/helpers; all other values reuse `recorded_comparison_holds`. No new numeric tolerance or public policy was added.

## Original-behavior proof

- `git diff --numstat` for `lib.rs`: `906` insertions, `0` deletions.
- Baseline content is therefore preserved byte-for-byte as a subsequence; fixture inventory, expected-value construction, predicates/constants, provenance, and pre-existing tests are unmodified.
- Frozen original whole-suite runner output remains byte-identical at the SHA-256 above, proving the existing 11-case / 91-observation record has not changed during N1.
- Full mechanics suite: 41 tests passed, 0 failed, 0 ignored; this includes all pre-existing tests and the two additive API tests. Doc tests: 0 failed.

## Commands and results

Cargo cache, target, temporary files, and logs were externalized beneath `/private/tmp/a2-09-verify-cargo-20260811-001`.

1. `cargo fmt --manifest-path projects/chirality-piping/validation/benchmarks/mechanics/Cargo.toml --check` with externalized Cargo/TMP environment — PASS, exit 0.
2. `cargo test --offline --manifest-path projects/chirality-piping/validation/benchmarks/mechanics/Cargo.toml` with externalized Cargo/TMP environment — PASS, exit 0: 41 passed, 0 failed; doc tests 0 failed.
3. `git diff --check -- projects/chirality-piping/validation/benchmarks/mechanics/src/lib.rs` — PASS.
4. Producer-region search for `expected_values|recorded.value|expected.value` — zero findings.
5. Mechanics-tree search for C-B / DEC-046 policy references — zero findings.
6. `git diff --name-status` — only `M projects/chirality-piping/validation/benchmarks/mechanics/src/lib.rs` among tracked content; source delta is 906 additions / 0 deletions.
7. `git diff --cached --name-only` — empty; ignored-path inventory — empty.

The test run emitted one non-failing pre-existing unused-import warning for `EquivalentStaticMechanicsBasis` and `prepare_equivalent_static_loads`.

## Containment and disposition

N1 production containment passes: the only tracked source mutation is the authorized `lib.rs`; no fixture, page, Cargo manifest/lock, deliverable state, evidence, receipt, C-B/nonlinear, PKG-10, or Git state was mutated. Candidate and managed run-root files are authorized parent/control-plane state. This verifier wrote only this `RETURN.md` and its `STATUS.json`; repository build side effects are zero.

No findings or blockers. Rerun if the accepted `lib.rs` bytes, fixture inventory/names, underlying mechanics APIs, frozen runner output, or C-B identity changes.
