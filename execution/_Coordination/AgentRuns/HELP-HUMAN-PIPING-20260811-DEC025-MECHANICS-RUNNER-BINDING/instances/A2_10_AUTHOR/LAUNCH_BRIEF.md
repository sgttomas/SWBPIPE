# Sealed Launch Brief — A2-10 Runner Consumption Author

RequestedBy: N2 WORKING_ITEMS / PKG-10 / DEL-10-05

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `N2-WORKING-ITEMS-PKG10-DEL1005`

ChildInstanceID: `A2-10-AUTHOR`

AgentType: ephemeral generalist Agent 2; non-delegating

Objective: replace only the missing mechanics fallback in the headless runner
with thin consumption of the accepted suite-owned observation and comparison
APIs while preserving the original 11-case output exactly.

AcceptedBasis: owner-adopted candidate SHA-256
`c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`;
base `f1e311fb7ab1c2a0800b1d32c59445368428dee9`; N1 return `ACCEPT`;
mechanics `lib.rs` SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`
and blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197` exposing
`fixture_observations` and `fixture_recorded_comparison_holds`; frozen original
runner output SHA-256
`e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`;
C-B SHA-256
`1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

DeclaredReads: root/project instructions; AGENT_WORKING_ITEMS and software
profile; adopted candidate; N1 returns/source; DEL-10-05 context; runner
manifest/module/binary/tests; mechanics/stress/nonlinear public suite APIs;
frozen 25-case output.

AllowedTools: read/search, apply_patch, cargo fmt/test with all Cargo home,
target, temp, and logs externalized beneath a fresh `/private/tmp` directory;
read-only Git inspection and hashing. No network or provisioning.

AllowedWriteTargets: exactly
`core/runner/headless/src/benchmark_binding.rs`. Tests must live in its existing
Rust test module. The binary and `tests/test_headless_runner_contract.py` are
read-only. If either is strictly necessary, document the reason and stop for
manager disposition before editing.

ImplementationConstraints:

1. Preserve the existing 11 fixture arms and their output byte/semantic
   projection exactly; do not route those arms through the new generic API.
2. Replace only the catch-all for the accepted 14 cases with
   `mechanics::fixture_observations`; use
   `mechanics::fixture_recorded_comparison_holds` for each returned named value.
3. The mechanics suite remains owner of inventory, expected values,
   observations, predicates, and tolerances. Do not encode formulas, expected
   values, fixture internals, a runner tolerance, or C-B mechanics behavior.
4. Map structured suite errors to fail-closed execution/block outcomes without
   silent skip. Preserve unknown ID, non-finite, incomplete, duplicate,
   extra/name mismatch, and suite failure semantics.
5. Preserve stress and nonlinear behavior. C-B remains byte-identical and is
   nonlinear-only.
6. Truthfully narrow stale module claim posture only within this file where the
   now-public suite surface makes the old catch-all claim false.

ExpectedOutputs: one-file source delta with in-file focused tests; durable
`RETURN.md` and terminal `STATUS.json` in this instance directory; exact source
hash/blob; start/finish ignored inventory; commands/results; count/output/
negative/stress/nonlinear/C-B/containment evidence.

AcceptanceCriteria: whole mechanics suite `25/25` executed-and-matched,
`206/206` populated finite comparisons, exact new `14/115`, exact frozen
original `11/91` projection, zero mismatch/block/missing/extra/duplicate;
negative behaviors fail closed; runner Cargo fmt/test PASS; mechanics 41/41
remains PASS if rerun; stress unchanged; nonlinear 5/5; C-B exact and no
mechanics read; only authorized production file changed; no staged or ignored
side effect.

Exclusions: every other production/project/state/evidence/receipt/Git path;
N3/N4; staging, commit, push, PR, merge, fetch, rebase, reset, clean, deletion,
dependency provisioning, external service, or publication.

Escalation: stop and return HOLD on strict need for a conditional file,
unexpected side effect, missing prerequisite, acceptance failure, or scope
need. Do not repair outside the fence and do not delegate.
