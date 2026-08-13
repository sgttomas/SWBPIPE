# Sealed Launch Brief — A2-10 Fresh Runner Author Remediation

RequestedBy: N2 WORKING_ITEMS / PKG-10 / DEL-10-05

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `N2-WORKING-ITEMS-PKG10-DEL1005`

ChildInstanceID: `A2-10-AUTHOR-REMEDIATION`

AgentType: fresh ephemeral generalist Agent 2; non-delegating

Objective: independently review the preserved runner candidate, repair only
within the original one-file fence, add the missing in-file acceptance and
negative tests, execute every author gate without repository-local Cargo side
effects, and return an exact accept-ready candidate or HOLD.

AcceptedBasis:

- owner-adopted candidate SHA-256
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`;
- base `f1e311fb7ab1c2a0800b1d32c59445368428dee9`;
- accepted N1 mechanics source SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
  blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`;
- preserved unaccepted runner candidate SHA-256
  `a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`,
  blob `1a9254cef16414a1252597cae5cc7135c33cc743`;
- C-B SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`;
- exact cleanup backcheck PASS; ignored and staged inventories zero.

DeclaredReads: root/project instructions, adopted brief, N1 returns/source,
prior A2-10 HOLD, N2 amendment and cleanup record, DEL-10-05 context, runner
module/manifest/binary/tests, mechanics/stress/nonlinear public suite surfaces,
frozen 25-case output, and software workflow profile.

AllowedWriteTargets:

- production: exactly
  `{WORKING_ROOT}/core/runner/headless/src/benchmark_binding.rs`;
- control: this instance's `RETURN.md` and terminal `STATUS.json`;
- test scratch: one fresh external root beneath `/private/tmp` containing an
  exact byte-bound shadow of the current Piping tree plus all Cargo/cache/
  target/temp/lock/log/output artifacts.

AllowedTools: read/search, apply_patch, exact-file rustfmt, read-only Git/hash/
inventory, deterministic external shadow copy, and Cargo/test execution only
against the external shadow. No network or provisioning.

ImplementationRequirements:

1. Review rather than presume the preserved candidate is correct.
2. Keep every original explicit 11-case evaluator and output projection exact.
3. Only the missing fallback may consume `fixture_observations`; each fallback
   comparison must call `fixture_recorded_comparison_holds` with fixture ID,
   name, observed, and recorded values.
4. Add in-file tests proving 25/25, 206/206, new 14/115, exact frozen original
   11/91 projection, zero mismatch/block, and no duplicate/extra/missing name.
5. Add focused fail-closed tests for unknown ID, incomplete observations,
   duplicate names, non-finite values, suite execution error, and expected/
   observed name mismatch at the runner seam. Test-only injection is permitted
   only inside this file's `#[cfg(test)]` module; production semantics may not
   weaken suite ownership.
6. Prove stress behavior unchanged, nonlinear 5/5, and C-B byte-identical and
   absent from mechanics comparison paths.
7. Add no tolerance, expected value, fixture formula, C-B mechanics behavior,
   schema change, or claim widening.

ExecutionContainment:

- never repurpose `HOME`;
- start with ignored/staged inventory zero;
- do not run Cargo against a repository manifest;
- after authorized source edits and formatting, copy the current Piping tree
  to a fresh external shadow and bind its source hashes to the repository
  candidate before testing;
- set external `CARGO_HOME`, `CARGO_TARGET_DIR`, and `TMPDIR`; lock generation
  must occur only beside the external shadow manifest;
- run all Cargo tests and exact suite execution from the shadow;
- finish with repository ignored/staged inventory zero;
- stop on any new repository side effect without deletion.

AcceptanceCriteria: exact counts and original projection above; runner fmt and
full tests PASS; focused negative tests PASS; stress unchanged; nonlinear 5/5;
C-B exact/nonlinear-only; `git diff --check` PASS; exact one-file production
containment; no ignored/staged drift; no binary/Python test modification.

Exclusions: all other production, fixture, case page, candidate, policy,
state/memory/run-record/receipt/evidence/TM/lifecycle/Git surfaces; N3/N4;
network, dependency provisioning, cleanup, stage, commit, push, PR, merge,
fetch, rebase, reset, or clean.

Escalation: HOLD rather than widen scope. Do not delegate. Do not delete any
repository side effect. A PASS return must include exact source hash/blob,
test commands/results, count/negative/original/stress/nonlinear/C-B evidence,
external-shadow binding, inventories, containment, and rerun triggers.
