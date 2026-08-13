# Sealed Launch Brief — A2-10 Fresh Non-Repairing Verifier

RequestedBy: N2 WORKING_ITEMS / PKG-10 / DEL-10-05

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `N2-WORKING-ITEMS-PKG10-DEL1005`

ChildInstanceID: `A2-10-VERIFY`

AgentType: fresh ephemeral generalist Agent 2; non-delegating; non-repairing

Objective: independently verify the exact N2 author candidate and return PASS
or BLOCK without modifying production.

AcceptedBasis:

- owner-adopted candidate SHA-256
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`;
- base `f1e311fb7ab1c2a0800b1d32c59445368428dee9`;
- accepted N1 mechanics source SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
  blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`;
- exact N2 author candidate `benchmark_binding.rs` SHA-256
  `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`,
  blob `455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`;
- C-B SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`;
- author remediation PASS and cleanup backcheck PASS; repository staged and
  ignored inventories zero.

DeclaredReads: complete candidate delta and frozen basis; adopted brief; N1
and N2 durable returns/amendment/cleanup evidence; runner module/manifest/
binary/tests; suite public sources; frozen whole-suite output; project/root
instructions and software profile.

AllowedWrites: only this instance's `RETURN.md` and terminal `STATUS.json`,
plus one fresh external `/private/tmp` verification root. Production is wholly
read-only.

AllowedTools: read/search/hash/diff/inventory, exact-file format check, copy of
the complete current Piping tree into a fresh byte-bound external shadow, and
Cargo/test execution only against the shadow. Set external `CARGO_HOME`,
`CARGO_TARGET_DIR`, and `TMPDIR`; never repurpose `HOME`; no network.

IndependentChecks:

1. inspect the complete candidate and prove only the missing fallback consumes
   `fixture_observations` plus per-value
   `fixture_recorded_comparison_holds`;
2. prove every original explicit 11-case evaluator/output path remains exact;
3. verify 25/25, 206/206 finite populated values, new 14/115 complete/unique,
   exact frozen original 11/91 complete case-object projection, zero mismatch/
   block/diagnostic;
4. verify fail-closed unknown, incomplete, duplicate, non-finite, suite-error,
   and expected/observed name mismatch behavior;
5. verify stress source/behavior unchanged, nonlinear 5/5, C-B byte-identical
   and absent from mechanics runner paths;
6. independently run format/full runner/mechanics tests, diff/one-file
   production containment, and final staged/ignored zero checks.

Acceptance: all above PASS with exact evidence. Any defect returns `BLOCK`; the
verifier never repairs, deletes, or widens scope.

Exclusions: all production/source edits; binary/Python test; fixtures/pages/
policies/state/memory/run records/receipts/evidence/TM/lifecycle/Git; N3/N4;
network, cleanup, stage, commit, push, PR, merge, fetch, rebase, reset, clean,
or deletion.

ReturnContract: exact identities, independent inspection, commands/results,
counts/negative/original/stress/nonlinear/C-B evidence, external-shadow
binding, containment/inventories, findings/rerun triggers, and `PASS` or
`BLOCK`.
