# Superseded Interim Package Return — N2 WORKING_ITEMS PKG-10 / DEL-10-05

Verdict: `HOLD — UNEXPECTED IGNORED CARGO LOCKFILE SIDE EFFECT`

**Superseded status:** This truthful interim HOLD was cleared only by exact
owner-authorized cleanup, fresh author remediation, and fresh non-repairing
verification. The terminal N2 result is recorded in `RETURN_FINAL.md`; this
record remains immutable failure/recovery evidence.

## Accepted upstream and attempted output

N1 remains accepted at mechanics `lib.rs` SHA-256
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`
and Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`. C-B remains exact at SHA-256
`1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.

The A2-10 author produced only the authorized runner source candidate:

- `core/runner/headless/src/benchmark_binding.rs`;
- SHA-256 `a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`;
- Git blob `1a9254cef16414a1252597cae5cc7135c33cc743`;
- baseline delta 122 insertions / 71 deletions.

This source is preserved but **not accepted**. It keeps all 11 explicit
mechanics evaluator arms on their original paths and changes only the prior
fallback to consume `fixture_observations` and
`fixture_recorded_comparison_holds`. No binary or Python test was edited.

## Partial validation before mandatory stop

- headless fmt: PASS;
- headless Cargo tests: library 30/30, preview binary 1/1, final runner binary
  15/15, doc tests zero failures;
- exact 25-case run: 25/25 matched, zero mismatch/block, 206 finite populated
  values, former-blocked 14/115 complete, frozen original 11/91 case objects
  semantically exact, diagnostics empty;
- external whole-suite output SHA-256
  `12901f73ffdf7d828c6e179f25e672aa152006f183c5331e2c64b80354d1962d`;
- `git diff --check`: PASS;
- start staged and ignored inventories: zero.

All Cargo home/cache/target/temp/log paths were directed beneath
`/private/tmp/a2-10-author-20260811-001`. These are disposable external
working outputs, not accepted durable evidence.

## Mandatory stop

The first Cargo test invocation created exactly one ignored repository path:

- `projects/chirality-piping/core/runner/headless/Cargo.lock`;
- regular non-symlink file, mode `-rw-r--r--`;
- 10,114 bytes; device 16777232; inode 33897855;
- SHA-256 `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`;
- ignored by `core/runner/headless/.gitignore:2`.

The author and manager stopped without deleting or modifying it. Finish
ignored inventory is exactly this one path. No fresh verifier was released.

## Complete repository inventory at HOLD

Nonignored status comprises exactly:

1. modified accepted N1 `validation/benchmarks/mechanics/src/lib.rs`;
2. modified, unaccepted N2
   `core/runner/headless/src/benchmark_binding.rs`;
3. the adopted candidate brief;
4. the managed run root containing N1 records plus the N2 manager/author HOLD
   records, runtime ledger, graph, plan, and status.

There are zero staged paths. The only ignored path is the exact Cargo lockfile
above. There are no other tracked/untracked/ignored production paths. No
fixture, case page, binary, Python contract test, stress/nonlinear/C-B, witness,
deliverable state, receipt, evidence, lifecycle, or Git state was changed by
N2.

## Unresolved acceptance assertions

- author in-file acceptance and negative tests were not added;
- explicit unknown/incomplete/duplicate/non-finite/suite-failure/name-mismatch
  runner-local gates remain to be executed;
- stress and nonlinear preservation gates remain to be freshly rerun;
- final C-B/nonlinear-only and zero-ignored containment remain to be rerun;
- fresh non-repairing A2 verifier remains unreleased.

## Resume contract

After separately authorized exact deletion and terminal absence verification,
route a **fresh** `A2-10-AUTHOR-REMEDIATION` under the same one-file production
fence. It reviews rather than presumes acceptance of the preserved candidate,
adds the missing in-file tests, runs all author gates without repository-local
side effects, and returns a new exact identity. Only an accepted remediation
return may release the fresh non-repairing verifier. N3/N4 remain held.

No waiver, lifecycle effect, or publication/Git authority is conferred.
