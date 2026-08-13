# Package Return — N1 WORKING_ITEMS PKG-09 / DEL-09-01

Verdict: `ACCEPT`

## Coverage and accepted output

The owner-adopted N1 objective is complete. Accepted source:

- `validation/benchmarks/mechanics/src/lib.rs`
- SHA-256: `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`
- Git blob: `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`
- Baseline delta: 906 insertions, 0 deletions

Accepted additive APIs:

- `fixture_observations(fixture_id: &str) -> Result<Vec<MechanicsObservation>, MechanicsObservationError>`
- `fixture_recorded_comparison_holds(fixture_id: &str, value_name: &str, observed: f64, recorded: f64) -> bool`

They cover the exact frozen 14-case count vector
`2,2,6,3,3,2,1,2,21,42,12,6,8,5` = `115/115` values. Observation values are
produced from existing suite solver/adapter/load/support/fixture-input logic;
the producer audit found zero expected numeric reads. The comparison dispatch
uses only existing suite predicates/constants and adds no tolerance or public
policy. Unknown/non-finite/malformed conditions fail closed.

## Accepted child returns and validation

- Attempt 1: truthfully rejected and interrupted for non-execution, with no
  source write or technical blocker; routed as fresh bounded author remediation.
- Fresh author remediation: PASS; 14/14, 115/115, fmt/diff PASS, full offline
  mechanics suite 41/41 PASS.
- Fresh non-repairing verifier: PASS; independently confirmed exact bytes,
  coverage/name/order/finite/comparison predicates, zero expected numeric
  observation reads, 906/0 addition-only preservation, C-B exact/non-mechanics,
  original frozen 11-case/91-observation output unchanged, 41/41 tests, and
  containment.

C-B remains SHA-256
`1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.
Frozen original runner output remains SHA-256
`e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.
Repository ignored and staged drift are zero.

## Effects, blockers, and handoff

This return accepts bounded implementation evidence only. DEL-09-01 remains
`IN_PROGRESS`; no state, lifecycle, derivative evidence, receipt, fixture,
manual page, policy, C-B, PKG-10, or Git path changed. No notice, waiver, or
blocker remains in N1.

N2 may start only from the exact accepted `lib.rs` bytes and this return. It
must consume both suite-owned accessors, preserve the original 11/91 output,
and must not replace fixture-specific suite comparison dispatch with a runner
tolerance. Rerun N1 verification if accepted source bytes, fixture inventory/
names, underlying mechanics APIs, frozen runner output, or C-B identity change.

Derivative status: managed run records and runtime telemetry are derivative
execution evidence, not decomposition truth or lifecycle authority. Next
owner: HELP_HUMAN for serialized N1 fan-in and N2 release.

