# Orchestration Plan — DEC-025 Mechanics Runner Binding

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

Parent: HELP_HUMAN (Agent 0)

Authority: owner adoption of
`CB-2026-08-11-DEL1005-MECHANICS-RUNNER-BINDING-001`, exact candidate
SHA-256 `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.

Accepted base: `f1e311fb7ab1c2a0800b1d32c59445368428dee9` on
`codex/piping-dec025-case-runner-binding-20260811`.

Posture: serialized cross-package Agent 0 fan-in:

1. N1 WORKING_ITEMS / PKG-09 / DEL-09-01: one bounded Agent 2 author, then
   one fresh non-repairing Agent 2 verifier; accept exact `lib.rs` bytes or
   HOLD.
2. N2 WORKING_ITEMS / PKG-10 / DEL-10-05 starts only from accepted N1.
3. N3 fresh cross-package verification.
4. N4 integration/state/evidence closeout, then CHANGE.

N1 is accepted at mechanics `lib.rs` SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`
and Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`. This revision activates N2
only. N2 production write ownership is exactly
`core/runner/headless/src/benchmark_binding.rs`; the binary and Python runner
contract test remain read-only unless strict necessity is proved and routed
to the manager before an edit. Fixture/page/state/receipt/Git writes and N3/N4
writes are excluded. Runtime telemetry is derivative evidence and provides no
authority.

N2 is held before author acceptance because the first headless Cargo run
created the ignored repository path `core/runner/headless/Cargo.lock`. The
author stopped without deletion. N3/N4 remain unreleased; exact cleanup and a
fresh bounded author-remediation run are required before N2 may resume.

Owner cleanup-and-resume authority was received on 2026-08-12. The exact
ignored lockfile identity passed before deletion; one explicit non-recursive
deletion removed only that file; terminal ignored and staged inventories are
zero; every pre-existing source, candidate, and run-record hash remained
unchanged. N2 resumes only through fresh `A2-10-AUTHOR-REMEDIATION`, followed
after acceptance by fresh non-repairing `A2-10-VERIFY`. Repository-local Cargo
execution is prohibited: test compilation and lock generation occur only in
an external byte-bound shadow of the current Piping tree with external Cargo
home, target, temp, logs, and outputs.
