# Return — A2-10 Runner Consumption Author

Verdict: `HOLD — UNEXPECTED IGNORED CARGO LOCKFILE SIDE EFFECT`

## Scope and accepted basis

- Instance: `A2-10-AUTHOR`, parent
  `N2-WORKING-ITEMS-PKG10-DEL1005`.
- Branch / HEAD:
  `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- Adopted candidate SHA-256:
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
- Accepted prerequisite mechanics source remains SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`.
- DEC-046 C-B remains byte-identical at SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

## Production result before stop

The only production source edited by this instance is the authorized
`core/runner/headless/src/benchmark_binding.rs`.

- Source SHA-256:
  `a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`.
- Source Git blob: `1a9254cef16414a1252597cae5cc7135c33cc743`.
- Baseline delta: 122 insertions, 71 deletions.
- `git diff --check` passed.

The implementation leaves every existing explicit 11-case mechanics arm on
its original evaluator and comparison path. Only the previous catch-all now
calls `mechanics::fixture_observations`. Each returned name/value is compared
inside the common reporting projection by
`mechanics::fixture_recorded_comparison_holds(fixture_id, name, observed,
recorded)`. Any structured observation API error becomes an execution-failed,
blocked case; the suite API therefore remains the owner of unknown,
non-finite, duplicate, incomplete, extra/name-mismatch, and execution-failure
semantics. Stress and nonlinear source paths were not edited.

No runner binary or Python contract test was edited. The planned in-file
focused tests were not added before the mandatory stop, so the author return
cannot be accepted even apart from the ignored side effect.

## Completed validation evidence

All Cargo/cache/target/temp/log outputs were directed below the fresh external
root `/private/tmp/a2-10-author-20260811-001`. The already-provisioned N1
external Cargo cache was copied into that external Cargo home; no network or
repository-local target was used.

1. Start ignored inventory: zero paths. Start staged inventory: zero paths.
2. `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check` with the
   external environment: PASS, exit 0.
3. `cargo test --offline --manifest-path core/runner/headless/Cargo.toml` with
   the external environment: PASS, exit 0:
   - library: 30/30;
   - preview binary: 1/1;
   - final runner binary: 15/15;
   - doc tests: 0 failures.
4. Exact whole-suite `openpipestress-runner run-benchmark` against the frozen
   25-case input, with `--explicit-local-private-intent` and external output:
   PASS, exit 0. Machine inspection reported:
   - 25 requested / 25 executed-and-matched / 0 mismatched / 0 blocked;
   - 206 populated finite observations, every comparison true;
   - exact former-blocked projection 14 cases / 115 values;
   - exact frozen original projection 11 cases / 91 values, with every full
     case JSON object byte-semantically equal to the frozen reconnaissance;
   - no diagnostics and wrapper `blocked=false`.
5. External whole-suite output SHA-256:
   `12901f73ffdf7d828c6e179f25e672aa152006f183c5331e2c64b80354d1962d`.

These results are partial author evidence only. Fresh verifier, explicit
runner-local negative tests, and final containment remain required.

## Mandatory stop and containment finding

The first Cargo test invocation printed `Locking 48 packages` and created one
unexpected ignored repository path:

- path: `projects/chirality-piping/core/runner/headless/Cargo.lock`;
- regular file and not a symlink;
- mode `-rw-r--r--`, size 10,114 bytes;
- device 16777232, inode 33897855;
- creation/mtime `2026-08-11T23:44:45-0600`;
- SHA-256:
  `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`;
- ignored by `core/runner/headless/.gitignore:2` (`Cargo.lock`).

Finish ignored inventory is exactly this one path. The source index remains
unstaged. On manager detection I stopped without modifying or deleting the
lockfile, without further production edits or tests, and without cleanup.

Repository tracked production deltas at stop are exactly the accepted N1
mechanics `lib.rs` plus this instance's runner binding. Candidate and managed
run-root files are parent/control evidence. No fixture, case page, binary,
Python test, stress, nonlinear, C-B, state, receipt, lifecycle, evidence, or
Git operation was changed by this instance.

## Disposition and rerun trigger

N2 must remain held. A separately authorized exact cleanup/disposition is
required for the ignored `Cargo.lock`; this instance confers no deletion
authority. After terminal zero ignored drift, route a fresh bounded author or
author-remediation instance under the same one-file production fence to review
the source delta, add the missing in-file acceptance/negative tests, rerun the
sealed gates, and return a new exact source identity. Do not release the fresh
verifier from this HOLD return.
