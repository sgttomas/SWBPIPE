# Return — A2-10 Fresh Non-Repairing Verifier

Verdict: `PASS — ACCEPT EXACT N2 RUNNER CANDIDATE`

## Identity and independent shadow

- Instance `A2-10-VERIFY`, parent
  `N2-WORKING-ITEMS-PKG10-DEL1005`; non-delegating and non-repairing.
- Branch / HEAD:
  `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- Owner-adopted brief SHA-256:
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
- Exact runner candidate: SHA-256
  `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`,
  Git blob `455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`.
- Accepted N1 mechanics source: SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
  Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`.
- DEC-046 C-B: SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

A fresh complete Piping-tree shadow was created at
`/private/tmp/a2-10-verify-20260812-77uFjL/shadow/chirality-piping`.
Repository and shadow hashes matched for the runner, N1 mechanics source, and
C-B policy. `CARGO_HOME`, `CARGO_TARGET_DIR`, and `TMPDIR` were independent
directories under that external root; `HOME` was not repurposed. Cargo was
offline, used only shadow manifest paths, and used no network. All build,
lockfile, cache, temporary, binary, and output artifacts remained external.

## Independent inspection

The complete source diff was inspected. The original explicit 11-case
evaluator match arms are unchanged; only the prior missing-case fallback calls
`mechanics::fixture_observations`. Each fallback value is compared through
`mechanics::fixture_recorded_comparison_holds` with fixture ID, value name,
observed value, and recorded value. The shared reporting refactor introduces
no tolerance. Runtime projection backchecked every original case object
byte-for-semantic-JSON equality against the frozen output: exact 11 cases / 91
values.

The runner seam rejects duplicate names, non-finite observations, missing
names, extra/name-mismatched observations, suite execution errors, and unknown
fixture IDs. All six behaviors have direct in-file tests and all passed in the
fresh full runner test run.

## Commands and results

1. External-shadow `cargo fmt --manifest-path .../core/runner/headless/Cargo.toml --check`:
   PASS, exit 0.
2. External-shadow `cargo test --offline --manifest-path .../core/runner/headless/Cargo.toml`:
   PASS, exit 0:
   - runner library 38/38, including whole-suite acceptance and all six
     negative behaviors;
   - preview binary 1/1;
   - final runner binary 15/15;
   - doc tests: zero failures.
3. External-shadow `cargo test --offline --manifest-path .../validation/benchmarks/mechanics/Cargo.toml`:
   PASS, exit 0, 41/41; doc tests zero failures.
4. External-shadow `openpipestress-runner run-benchmark` with the exact
   registered whole-suite mechanics input and
   `--explicit-local-private-intent`: PASS, exit 0. Machine inspection proved:
   - 25 requested / 25 executed-and-matched / 0 mismatched / 0 blocked;
   - 206/206 populated finite values and 206/206 within the suite-owned basis;
   - new 14 cases / 115 values complete with unique names;
   - frozen original 11 cases / 91 values exact as complete serialized case
     objects;
   - zero diagnostics and wrapper `blocked=false`.
   Output bytes: 1,561,214; SHA-256
   `12901f73ffdf7d828c6e179f25e672aa152006f183c5331e2c64b80354d1962d`;
   stdout was byte-identical to the output file.

## Preservation and containment

- Stress source is unchanged from HEAD at SHA-256
  `ad7239a073f5ca6c161ee3f63642d487414efed6e3176ae86a2af7979b73210c`.
  The full runner test proved the exact existing behavior: zero mismatches,
  exactly the three historical fail-closed cases, and all other inventoried
  cases matched.
- Nonlinear source is unchanged from HEAD at SHA-256
  `ff3f318224fa2c55392bfa17569182768453b3157cf35fc20d763f54bf02fd12`.
  The full runner test proved 5/5 matched, zero mismatch and zero block.
- C-B is byte-identical at its accepted hash. No `C-B`, policy-file path, or
  `release_convergence_policy` token is consumed by the mechanics runner path;
  the only DEC-046 mentions are the pre-existing non-promotion claim posture.
- `git diff --check`: PASS. Piping production deltas are exactly the accepted
  N1 mechanics source and this one N2 runner source. No binary, Python test,
  fixture, case page, stress, nonlinear, C-B, schema, evidence, receipt,
  lifecycle, state, or Git surface changed.
- Final repository staged inventory: zero. Final ignored inventory: zero.

## Findings and rerun triggers

No acceptance blocker or repair finding was observed. The mechanics crate
emits the already-present unused-import warning for two N1 imports; it does not
affect compilation or the 41/41 result and this verifier made no repair.

Rerun if runner bytes, accepted N1 mechanics bytes, frozen 25-case output,
stress/nonlinear sources, C-B bytes, Cargo dependency resolution, or shadow
binding changes, or if repository-local Cargo/ignored/staged drift appears.
This PASS authorizes N2 fan-in only; it does not authorize N3/N4, staging,
commit, push, PR, merge, cleanup, or publication.
