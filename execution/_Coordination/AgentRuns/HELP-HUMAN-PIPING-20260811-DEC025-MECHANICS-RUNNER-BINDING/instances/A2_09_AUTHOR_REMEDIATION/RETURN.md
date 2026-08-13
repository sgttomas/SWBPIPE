# Return — A2-09 Author Remediation

Verdict: `PASS`

## Result

Implemented in the sole authorized production file:

- `pub fn fixture_observations(fixture_id: &str) -> Result<Vec<MechanicsObservation>, MechanicsObservationError>`
- `pub fn fixture_recorded_comparison_holds(fixture_id: &str, value_name: &str, observed: f64, recorded: f64) -> bool`

The first API deterministically dispatches all 14 current formerly blocked
fixture IDs through existing public-original solver, adapter, load-generation,
support-preparation, and fixture-input paths. It returns fixture-owned names in
fixture order and numeric observations only. Its production region contains
zero reads of `expected_values`; expected values supply only the boundary's
name-completeness audit and test comparison arguments.

The comparison API preserves `recorded_comparison_holds` for existing callers
and delegates only to already-encoded suite predicates/constants for DEC-092,
expansion-loop, curved-bend distributed-load, and curved-bend pressure-thrust
values. It introduces no numeric constant or policy. Unknown fixture/value
names and non-finite inputs fail closed. Structured errors cover unknown ID,
execution failure, non-finite value, duplicate name, incomplete set, and name
mismatch.

## Coverage

Counts by fixture, in frozen order: `2, 2, 6, 3, 3, 2, 1, 2, 21, 42, 12,
6, 8, 5` = **14/14 cases and 115/115 values**. The in-file test proves exact
expected-name order, finite values, no missing/extra/duplicate name, and PASS
under the suite-owned comparison dispatch for all 115 observations.

## Validation

Cargo state was externalized beneath `/private/tmp/a2-09-remediation-cargo`;
the existing registry/git cache was copied there and commands ran offline.

1. `cargo fmt --manifest-path projects/chirality-piping/validation/benchmarks/mechanics/Cargo.toml --check` — PASS.
2. `cargo test --offline --manifest-path projects/chirality-piping/validation/benchmarks/mechanics/Cargo.toml` — PASS: 41 unit tests passed, 0 failed, 0 ignored; doc tests 0 failed.
3. `git diff --check -- projects/chirality-piping/validation/benchmarks/mechanics/src/lib.rs` — PASS.
4. Observation-producer source audit for `expected_values`, `recorded.value`, or `expected.value` — zero findings.
5. C-B SHA-256 remains `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

The full run emitted one non-failing pre-existing unused-import warning.

## Identity and containment

- Baseline SHA-256: `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26`
- After SHA-256: `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`
- After Git blob: `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`
- Source delta: 906 insertions, 0 deletions.

This child changed only `validation/benchmarks/mechanics/src/lib.rs` and this
instance's `RETURN.md`/`STATUS.json`. Repository ignored drift remained zero.
Other run-root artifacts are parent/sibling state, not this child's writes.
External build/cache/temp files remain in the predeclared `/private/tmp`
directory.

No blocker. Rerun verification if accepted `lib.rs`, fixture inventory/names,
underlying mechanics APIs, or C-B identity changes. N2 must consume this exact
blob and both suite-owned accessors; it must not replace fixture-specific
comparison dispatch with the generic absolute accessor.
