# Final Package Return — N2 WORKING_ITEMS PKG-10 / DEL-10-05

Verdict: `ACCEPT`

## Accepted basis and output

- owner-adopted candidate SHA-256
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`;
- pinned branch/base
  `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`;
- accepted N1 mechanics source SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
  Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`;
- accepted N2 runner source
  `core/runner/headless/src/benchmark_binding.rs`, SHA-256
  `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`,
  Git blob `455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`, baseline delta
  428 insertions / 70 deletions.

No conditional binary or Python test file was needed or edited. The runner
preserves every original explicit 11-case evaluator/output path. Only the
formerly missing fallback consumes `fixture_observations`; every fallback
value comparison uses `fixture_recorded_comparison_holds` with fixture ID,
name, observed, and recorded values. No runner tolerance, expected value,
fixture formula, or mechanics C-B behavior was introduced.

## Accepted child returns and recovery history

- A2-10 author attempt 1: truthful HOLD after Cargo created one ignored
  repository lockfile. Partial behavior passed but source was not accepted.
- Exact owner-authorized cleanup: PASS; only the verified 10,114-byte lockfile
  was deleted; terminal absence, parent retention, byte preservation, zero
  staged, and zero ignored drift proved.
- Fresh A2-10 author remediation: PASS; completed in-file acceptance and six
  fail-closed negative tests, used external byte-bound Cargo shadow, and
  returned the exact accepted source above.
- Fresh non-repairing A2-10 verifier: PASS; independently inspected and reran
  the complete acceptance contract without repairs.

## Independent validation

- runner fmt: PASS;
- runner tests: library 38/38, preview binary 1/1, final binary 15/15, doc
  tests zero failures;
- mechanics tests: 41/41, doc tests zero failures;
- exact whole-suite mechanics execution: 25/25 matched, 0 mismatched,
  0 blocked, 206/206 finite populated values within the suite-owned basis;
- formerly blocked projection: exact 14/115 complete with unique names;
- frozen original projection: exact 11/91 at complete serialized case-object
  level;
- wrapper diagnostic/block posture: zero diagnostics, `blocked=false`;
- independent output: 1,561,214 bytes, SHA-256
  `12901f73ffdf7d828c6e179f25e672aa152006f183c5331e2c64b80354d1962d`;
- six fail-closed behaviors PASS: unknown ID, incomplete set, duplicate name,
  non-finite value, suite execution failure, expected/observed name mismatch;
- stress source and exact historical behavior unchanged: zero mismatches,
  three pre-existing fail-closed cases, all remaining inventoried cases
  matched;
- nonlinear source/behavior unchanged: 5/5 matched, zero mismatch/block;
- C-B byte-identical at SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`
  and absent from mechanics runner paths;
- `git diff --check`: PASS; production delta exactly accepted N1 `lib.rs` plus
  accepted N2 `benchmark_binding.rs`; staged and ignored inventories zero.

All accepted Cargo execution occurred offline in fresh complete Piping-tree
shadows with external Cargo home, target, temp, lockfile, logs, binaries, and
outputs. `HOME` was not repurposed; no network was used.

## Disposition

N2 is accepted as bounded implementation evidence. DEL-10-05 remains
`IN_PROGRESS`; no deliverable state, lifecycle, evidence sweep, receipt,
fixture/page/policy/TM, or Git surface changed. The managed run evidence is
derivative execution evidence and does not replace decomposition truth.

No N2 blocker or waiver remains. Rerun if accepted N1 or N2 bytes, frozen
25-case output, stress/nonlinear sources, C-B bytes, Cargo resolution, or
shadow binding changes, or if staged/ignored drift appears.

Next owner: HELP_HUMAN for N2 fan-in and possible N3 release. This return does
not itself authorize N3/N4, staging, commit, push, PR, merge, or publication.
