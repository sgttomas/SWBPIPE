# N2 Amendment V2 — Exact Cleanup and Fresh Author Remediation

Status: `ACTIVE`

Authority: owner direct cleanup-and-resume approval, 2026-08-12.

The original A2-10 author HOLD remains immutable evidence. Its candidate is
preserved but unaccepted at SHA-256
`a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`.
The exact lockfile cleanup backcheck is PASS with zero ignored/staged drift.

N2 resumes through one fresh non-delegating
`A2-10-AUTHOR-REMEDIATION`, followed only after author acceptance by one fresh
non-repairing `A2-10-VERIFY`. Production scope remains exactly
`core/runner/headless/src/benchmark_binding.rs`; no conditional file is opened.

To prevent recurrence, no Cargo command may use the repository manifest path.
After source edits and exact format/diff checks, the author creates a fresh
external byte-bound shadow of the current `projects/chirality-piping` tree and
runs all Cargo commands against the shadow manifest. `CARGO_HOME`,
`CARGO_TARGET_DIR`, `TMPDIR`, Cargo lockfile, logs, binaries, and machine output
remain beneath that fresh external root. `HOME` is not repurposed. The source
tree's ignored inventory must remain zero throughout.

All original acceptance counts, negative behaviors, stress/nonlinear/C-B
gates, containment, and exclusions remain unchanged. Unexpected repository
side effect returns HOLD without deletion.
