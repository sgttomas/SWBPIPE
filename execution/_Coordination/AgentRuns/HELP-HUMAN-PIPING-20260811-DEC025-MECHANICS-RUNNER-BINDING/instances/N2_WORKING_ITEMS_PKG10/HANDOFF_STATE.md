# Superseded Interim Handoff State — N2 PKG-10 to HELP_HUMAN

Closure verdict: `N2 HOLD — DO NOT RELEASE N3`

**Superseded status:** Exact owner-authorized cleanup and the required fresh
author-remediation/verifier sequence later passed. The terminal handoff is
`HANDOFF_STATE_FINAL.md`; this record remains immutable interim evidence.

Accepted upstream: N1 source SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`.

Preserved but unaccepted N2 source: `benchmark_binding.rs` SHA-256
`a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`,
blob `1a9254cef16414a1252597cae5cc7135c33cc743`.

Blocker: unexpected ignored regular non-symlink
`core/runner/headless/Cargo.lock`, 10,114 bytes, SHA-256
`7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`.
It remains untouched. Ignored finish inventory contains exactly this path;
staged inventory is empty.

Partial evidence is promising but non-terminal: fmt and headless tests passed;
whole mechanics execution reported 25/25, 206/206, new 14/115, exact original
11/91, zero mismatch/block. Missing in-file negative tests, fresh stress/
nonlinear/C-B/containment checks, and the fresh non-repairing verifier prevent
acceptance.

Derivative status: these managed records are execution evidence only. They do
not substitute for package/decomposition truth or authorize lifecycle,
reliance, cleanup, Git, or publication acts.

Next owner/action: HELP_HUMAN may request exact cleanup-and-resume authority.
After the lock is verified absent, release only a fresh bounded author-
remediation child under the same one-file fence; N3/N4 remain held until a
fresh verifier accepts the completed N2 output.
