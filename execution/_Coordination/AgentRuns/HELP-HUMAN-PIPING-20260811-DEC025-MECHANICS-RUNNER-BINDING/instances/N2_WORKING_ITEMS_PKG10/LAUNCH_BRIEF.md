# Launch Brief — N2 WORKING_ITEMS / PKG-10 / DEL-10-05

RequestedBy: HELP_HUMAN (Agent 0)

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ChildInstanceID: `N2-WORKING-ITEMS-PKG10-DEL1005`

PackageID: `PKG-10`

DeliverableID: `DEL-10-05`

Objective: manage only the accepted N2 runner-consumption node of owner-adopted
`CB-2026-08-11-DEL1005-MECHANICS-RUNNER-BINDING-001` and return an accepted
exact runner delta or HOLD.

AcceptedBasis:

- pinned branch/base `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`;
- adopted candidate SHA-256
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`;
- accepted N1 return and handoff;
- mechanics `lib.rs` SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
  Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`;
- additive suite APIs `fixture_observations` and
  `fixture_recorded_comparison_holds`;
- C-B SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

Dependencies: N1 `ACCEPT`; fresh N1 verifier `PASS`; mechanics 41/41; exact
14/14 and 115/115 suite observation coverage; original 11/91 frozen output
preserved.

WorkGraph: one bounded non-delegating ephemeral Agent 2 author followed, only
after author acceptance, by one fresh read-only non-repairing Agent 2 verifier.
No concurrent production writes.

DeclaredReads: adopted candidate; N1 durable returns and exact source;
DEL-10-05 SOW/status/memory/context/dependencies/recent run records; runner
manifest, binding module, binary and contract tests; mechanics/stress/nonlinear
public suite surfaces; frozen whole-suite output; software workflow profile;
project/root instructions.

AllowedWriteTargets:

- manager control evidence beneath this run root;
- production author: exactly
  `core/runner/headless/src/benchmark_binding.rs`;
- `core/runner/headless/src/bin/openpipestress-runner.rs` and
  `tests/test_headless_runner_contract.py` remain read-only unless strict
  necessity is documented before edit and separately dispositioned by N2.

AcceptanceCriteria: `25/25`, `206/206`, new `14/14` and `115/115`, exact
original 11-case/91-observation output projection, zero mismatch/block,
fail-closed unknown/incomplete/duplicate/non-finite/suite-failure/name-mismatch
behavior, stress unchanged, nonlinear 5/5, C-B byte-identical and nonlinear-
only, package tests and containment PASS, zero repository build side effect.

Exclusions: all mechanics source/fixtures/pages, C-B/nonlinear, witness/state/
run-record/receipt/candidate/TM/lifecycle/Git surfaces; N3/N4; network,
dependency provisioning, cleanup, publication, staging, commit, push, PR,
merge, fetch, rebase, reset, or clean.

Escalation: HOLD rather than widen scope. A verifier never repairs. An author
defect may route only to a fresh bounded remediation author under the same
fence.

ReturnContract: exact changed-path/hash manifest, commands/results, full count
evidence, original-91 proof, negative behavior, stress/nonlinear/C-B evidence,
ignored start/finish inventory, containment, blockers/rerun triggers, and N2
`ACCEPT` or `HOLD`.
