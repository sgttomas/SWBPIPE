# Return — A2-10 Fresh Runner Author Remediation

Verdict: `PASS — ACCEPT-READY ONE-FILE RUNNER CANDIDATE`

## Scope and accepted basis

- Instance: `A2-10-AUTHOR-REMEDIATION`, parent
  `N2-WORKING-ITEMS-PKG10-DEL1005`; non-delegating.
- Branch / HEAD:
  `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- Preserved runner input identity was exact at SHA-256
  `a86576c6143db678b6437d41f9c8904a548ec2135ff8ecc235cbb456ab57376d`.
- Accepted N1 mechanics source remained exact at SHA-256
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`,
  Git blob `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`.
- DEC-046 C-B remained exact at SHA-256
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.
- Start ignored inventory: zero. Start staged inventory: zero.

## Production result

The only production file edited by this instance was the authorized
`core/runner/headless/src/benchmark_binding.rs`.

- Final SHA-256:
  `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`.
- Final Git blob: `455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`.
- Delta from pinned base: 428 insertions, 70 deletions.
- `git diff --check`: PASS.

The candidate retains every original explicit 11-case evaluator and output
projection. Only the missing fallback consumes
`mechanics::fixture_observations`; every fallback value calls
`mechanics::fixture_recorded_comparison_holds` with fixture ID, name,
observed value, and recorded value. The runner seam now additionally rejects
duplicate, non-finite, missing, and extra observation names before comparison,
without adding a tolerance or expected-value implementation.

The in-file test module adds direct acceptance and fail-closed coverage for:

- 25/25 mechanics cases, 206/206 finite populated comparisons, zero mismatch
  and zero block;
- exact new 14/115 projection and unique names;
- exact semantic equality of every frozen original 11/91 case object;
- unknown fixture ID;
- incomplete observation set;
- duplicate observation name;
- non-finite observation;
- suite execution error;
- expected/observed name mismatch;
- unchanged stress behavior and nonlinear 5/5 behavior.

## External-shadow binding and execution

All accepted Cargo execution used the fresh external root
`/private/tmp/a2-10-author-remediation-final-20260812-pApJHt`. The complete
current `projects/chirality-piping` tree was copied byte-for-byte to
`shadow/chirality-piping` after the final edit. Repository and shadow runner
hashes both equaled
`4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`;
repository and shadow mechanics hashes both equaled
`18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`.

`CARGO_HOME`, `CARGO_TARGET_DIR`, and `TMPDIR` were separate directories
beneath that external root. `HOME` was not repurposed. Cargo ran offline and
only with shadow manifest paths. Generated lockfiles, cache, target, temporary,
binary, log, stdout, and machine-output files remained external. No network
was used.

Accepted commands/results:

1. Exact-file `rustfmt --edition 2021` and repository `git diff --check`:
   PASS.
2. External-shadow `cargo fmt --manifest-path .../core/runner/headless/Cargo.toml --check`:
   PASS.
3. External-shadow `cargo test --offline --manifest-path .../core/runner/headless/Cargo.toml`:
   PASS:
   - library 38/38, including eight `benchmark_binding` acceptance/negative
     tests;
   - preview binary 1/1;
   - final runner binary 15/15;
   - doc tests: zero failures.
4. External-shadow `cargo test --offline --manifest-path .../validation/benchmarks/mechanics/Cargo.toml`:
   PASS, 41/41; doc tests zero failures.
5. External-shadow runner binary, exact frozen whole-suite mechanics input,
   `run-benchmark --explicit-local-private-intent`: PASS, exit 0.

The machine inspection of step 5 proved:

- 25 requested / 25 executed-and-matched / 0 mismatched / 0 blocked;
- 206/206 observations populated, finite, and within the suite-owned basis;
- new 14 cases / 115 observations complete, without duplicate names;
- frozen original 11 cases / 91 observations exact at the complete serialized
  case-object level;
- no diagnostics and wrapper `blocked=false`.

The output was 1,561,214 bytes with SHA-256
`12901f73ffdf7d828c6e179f25e672aa152006f183c5331e2c64b80354d1962d`.

## Stress, nonlinear, C-B, and containment

- Stress source is unchanged from HEAD at SHA-256
  `ad7239a073f5ca6c161ee3f63642d487414efed6e3176ae86a2af7979b73210c`.
  The runner test binds its unchanged behavior: zero mismatches, exactly the
  pre-existing three fail-closed input-restatement cases, and every remaining
  inventoried case matched.
- Nonlinear source is unchanged from HEAD at SHA-256
  `ff3f318224fa2c55392bfa17569182768453b3157cf35fc20d763f54bf02fd12`.
  The runner test executed 5/5 nonlinear cases matched with zero mismatch or
  block.
- C-B is byte-identical at the accepted SHA-256 above. No `C-B`, policy-file
  path, or `release_convergence_policy` token occurs in the runner binding;
  mechanics comparison paths do not consume it.
- Final repository staged inventory: zero. Final repository ignored
  inventory: zero.
- Tracked Piping production deltas are exactly the accepted N1 mechanics
  source and this runner source. This instance changed no binary, Python test,
  fixture, case page, stress, nonlinear, C-B, schema, receipt, lifecycle,
  evidence, state, or Git surface.

One preliminary external-shadow run compiled successfully and passed 37/38;
its only failure was an over-strong new test assertion that expected the
stress suite to have no historical blocks. The final test binds the exact
unchanged three-block stress posture and all accepted commands above pass.
Both preliminary and accepted artifacts stayed outside the repository.

## Rerun triggers

Rerun this author gate if the runner source, accepted N1 mechanics source,
frozen 25-case output, stress/nonlinear sources, C-B bytes, Cargo dependency
graph, or external-shadow binding changes. Any repository-local Cargo side
effect, conditional-file need, or acceptance-count drift requires HOLD.

N2 may route one fresh non-repairing `A2-10-VERIFY` against the exact final
runner identity. This return does not authorize N3/N4, staging, commit, push,
PR, merge, cleanup, or publication.
