# PROJECT_SETUP run — lockless Cargo prerequisite preflight

## Basis and scope

- Parent role: `HELP_HUMAN`
- Manager role: `PROJECT_SETUP`
- Branch: `codex/piping-lockless-cargo-preflight`
- Examined-through/base SHA: `0d3eb2249cdb09a2a4bda8b7c73f187a8835467b`
- Authorized item: `TM-PIP-029`
- Product write scope:
  - `tools/release/run_evidence_sweep.py`
  - `tests/test_evidence_sweep.py`

## Implemented contract

The evidence-sweep prerequisite preflight now classifies Cargo locks from the
Git index, not filesystem existence. The 20 tracked-lock manifests continue to
run `cargo fetch --locked --offline` against the checkout. The 16 lockless
manifests run `cargo fetch --offline` against one temporary projection of
`core/` and `validation/benchmarks/` that excludes every `Cargo.lock` and
`target/`. The projection preserves the relative layout required by local path
dependencies and is removed automatically. Failure to establish Git lock
ownership is a preflight error rather than a guess.

## Verification

- Focused evidence-sweep pytest: `31 passed`.
- Full Piping pytest: `569 passed`.
- Live prerequisite probe: 36 manifests, 20 tracked and 16 lockless, zero
  errors; all commands were offline.
- Ignored-lock integrity: SHA-256, byte size, and nanosecond mtime were captured
  before and after the live prerequisite probe for all 16 ignored source
  `Cargo.lock` files; every tuple was identical.
- Redirected sandbox-capability sweep: PASS for Cargo, Python, desktop Vitest
  (29 files / 523 tests), and desktop production build, with
  `CARGO_TARGET_DIR` redirected to a temporary target directory. Summary:
  `validation/evidence/sweeps/SWEEP_20260815T230054Z_0d3eb2249cdb-dirty.json`.
- Practitioner-harness pytest: `349 passed`; repository self-check exit 0;
  receipt validator and `git diff --check`: PASS.
- Independent Agent 2 verification: PASS, no findings; implementation diff
  digest `8b15c4b70a0d45a39812a0cc9d45e5529d687bb4fe078ff68fafebd2501c2f0d`.

The first sandboxed sweep attempt stopped before surface 1 because the pinned
`wasm-bindgen` executable was absent. `wasm-bindgen-cli 0.2.123` was then built
offline into `/tmp` only; no prerequisite was installed into or tracked by the
repository. The passing rerun used that temporary executable.

## Fan-in and publication gate

An ephemeral Agent 2 implementation child made the two bounded product edits;
a separate read-only ephemeral Agent 2 independently verified them. Neither
child delegated. Playwright remains the host-capability surface. A requested
host-context full sweep was rejected at the escalation gate under the existing
no-escalation/Playwright fence and was not retried or worked around. Therefore
the code candidate and four sandboxed surfaces are green, but the plan's full
five-surface publication gate remains blocked pending an owner-approved host
run. No commit, push, PR, merge, lifecycle transition, release act, or
professional-approval effect was performed by this manager run.
