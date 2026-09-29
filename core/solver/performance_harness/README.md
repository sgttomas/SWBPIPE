# Solver Performance Harness

This crate is the bounded implementation slice for `DEL-04-05`. It provides a deterministic regression harness around the frame-kernel solve boundary so solver performance and conditioning evidence can be recorded without changing solver logic. Since the human ruling `DEC-023` it measures the in-repo sparse skyline path (`core/solver/sparse_direct`) alongside the dense path on the same reduced systems.

## Scope

- Invented frame-chain and planar grid-frame benchmark fixtures with explicit public provenance posture.
- Fixture unit-system and unit identifiers for frame-kernel length, force,
  moment, stress, area, `second_moment_area`, displacement, and rotation
  quantities.
- Repeat-run regression records for the same fixture, solver version, and harness settings, including per-repeat residual and solution-delta observations.
- Matrix size, nonzero-count, residual, repeatability, diagonal conditioning observations, and condition-ratio estimates for the dense path.
- Sparse-path observations per record (`SparseSolveObservation`): deterministic RCM ordering identity, original/ordered profile entry counts and half-bandwidths, deterministic f64 value-storage byte counts, accepted-pivot extrema, the pivot-ratio conditioning proxy, nonpositive-pivot count, sparse-vs-dense parity delta, sparse residual, repeat determinism delta, and elapsed-time measurement.
- Sparse-suitability observation records over explicit invented planar-grid
  size bands (`SparseSuitabilityObservationRecord`) for the `DEC-050` evidence
  lane. These records preserve dense as default and carry bounded generated-grid
  threshold status for sparse-vs-dense parity, sparse residual, repeatability,
  nonpositive pivots, and the sparse factorization pivot-ratio conditioning
  proxy.
- Sparse default-promotion observation records (`SparseDefaultPromotionObservationRecord`)
  for `DEC-053`: 9 bounded chain/grid/product-proxy observations with practical
  size-band labels, dense/sparse timing observations, deterministic value-storage
  observations, hardware metadata binding, dense/sparse parity, residual,
  repeat determinism, pivot-ratio proxy, and true condition number computed from
  the reduced dense symmetric matrix for the observation set only.
- Elapsed-time measurements for the first dense solve and the first sparse order+factor+solve, plus deterministic reduced-dense and sparse-profile f64 value-storage byte counts. Timing observations are environment-dependent; storage observations exclude allocator/container overhead. No timing or memory thresholds are asserted (thresholds remain governed by D-04).
- Deterministic suite runs over explicit invented cantilever-chain fixture
  sizes with per-fixture records and suite-level summary counts, including
  sparse-observation aggregates.
- Integration with solver diagnostics in a fixed, documented diagnostic order: the DEC-023 sparse-solver adoption-status diagnostic, the tolerance-policy `TBD` diagnostic, dense conditioning classification, dense solve failures, sparse factorization-report diagnostics (nonpositive pivots), sparse conditioning classification, and sparse solve failures.

## Boundary

This crate does not set release-quality timing, allocator/RSS memory, CI, or
hardware-normalized thresholds, alter the frame kernel or the sparse solver,
define code-specific checks, encode protected standards examples, or make
professional/code-compliance claims.

Per `DEC-023` the dense path serves as the parity oracle for the in-repo sparse skyline path measured here. Per `DEC-050`, sparse is present as a live evidence lane in product/nonlinear paths. Per `DEC-053`, sparse interactive is promoted as the default preview/render iteration path and dense scrutiny remains explicitly selectable for review/parity. Timing/RSS/hardware observations are recorded for the bounded local evidence packet; no timing, memory, cross-machine, hosted-CI, release, professional, or code-compliance threshold is asserted.

The bounded generated-grid pivot-conditioning policy uses the sparse
factorization pivot-ratio proxy (`max |d| / min |d|`) only. It is not a true
matrix condition-number policy, does not set a CI gate, and does not promote the
sparse path beyond the named generated-grid observation set.

The `DEC-053` promotion packet records a true 2-norm condition number for each
bounded observation by running a deterministic dense symmetric eigenvalue
routine on the reduced dense matrix. That true-condition evidence closes the R4
residual for the named observation set only; it does not replace future release
conditioning policy.

Fixture unit metadata declares the calculation basis for reproducibility only. The harness does not define a project conversion catalog, convert units, supply protected benchmark values, or promote fixture results to release-quality performance evidence.

Sparse-vs-dense parity assertions in the tests cite the `DEC-026` analytic-class relative seed (1.0e-9) scaled by the dense reference solution magnitude; the harness itself records parity deltas without asserting thresholds.

## Verification

The unit tests cover deterministic repeat-run records, per-repeat observation rows, invented suite-runner records, suite summary counts (including sparse and value-storage aggregates), provenance rejection, invalid settings, nonzero-count metrics, conditioning observations, conditioning diagnostics, dense and sparse solve-failure diagnostic recording (including located sparse singular pivots), sparse-vs-dense parity on chain and grid fixtures (small and larger generated banded models), sparse repeat determinism, grid fixture validation, and residual calculation.

## K6: kernel memory and runtime observations (T3 D1 revision 5a.2 §4.8)

K6 adds kernel-level observations of the product's linear entry beside the
legacy DEC-023/050/053 harness, which is unchanged.

**Scope.**
- The sealed kernel models are R1's RF-LARGE families and the DEC-053 nine. There are 24 RF-LARGE cases: CHAIN, TREE and CONT, at 10, 100, 1,000 and 10,000 members, axis-aligned and rotated by Q3.
- They are generated in `src/k6/models.rs`, keeping R1's node and member order.
- Each model's canonical bytes (`k6-model v1`) are hashed in `observations/k6/models_sha256.txt`, and an independent Python generator in `runner/k6_runner.py` reproduces every hash.
- The section is formed from explicit products and `PI` only.

**The observation binary.** `k6_observe` runs one model in one mode per process, in one of four modes:
- `sparse`: `SparseAssemblyEvidence` in `SparseInteractive`;
- `dense`: `DenseScrutiny`;
- `lane-id`: the identity-order DEC-050/053 lane;
- `lane-lu`: the dense LU DEC-050/053 lane.

It runs the staged kernel sequence at public-API boundaries, in the adapter's order: assembly, evidence, ledger, reduce, geometry, [densify], prepare, factor, [witness], finish and recovery. It then runs the adapter's two entries end to end. The tests pin the staged result, bit for bit, to those entries.

It prints JSONL (`k6-observe-v1`), one object per line:
- `start`;
- `counts`;
- `stage_begin` and `stage`, for each stage of each repeat;
- `outcome`;
- `parity`;
- `summary`.

The `refusal` kind replaces the repeats when the binary refuses.

**The allocator.** A counting, capped global allocator lives only in the binary. It records the peak requested bytes per stage, in two models:
- in place, which carries the cap;
- move, where a growing `realloc` holds the old and the new block together.

A refused allocation writes a marker and aborts.

**Refusals by name.** The binary refuses these before any n² allocation, and exits 3:
- every n² mode (`dense`, `lane-lu`) on a model of 10,000 members or more;
- `lane-id` on CONT at 10,000 members;
- any run whose admission estimate exceeds half the heap cap.

**The runner.** `runner/k6_runner.py`, standard library only:
- runs the binary under `/usr/bin/time`, with `-l` on macOS and `-v` on Linux;
- on macOS, polls the binary's (not the wrapper's) RSS every 100 ms with `ps -o rss=` and kills the process group above the cap (`killed_by_rss_watchdog`);
- on Linux, sets `RLIMIT_AS` before exec;
- classifies each process, aggregates the median and minimum per stage over the repeats, and writes the records and the packet;
- has `--plan`, which lists the schedule and each run's admission without starting any process.

**Boundary.** These are observations only. No time or memory threshold is asserted in any test or record.
- The only claims are the observed log-log growth fits of peak memory against members, and the actual-to-estimate ratios for F1b's two estimates.
- The DEC-053 parity basis (1e-9 relative) is asserted in the tests where both modes publish Passed. Where a mode is Sensitive, the delta is recorded and not asserted.

**Running it.**
- `cargo build --release --bin k6_observe`, then `k6_observe --model <id> --mode <mode> --heap-cap-bytes <n>`.
- `k6_observe --list-models`, `--emit-model` and `--counts-only` inspect the models.
- `python3 runner/k6_runner.py --plan --counts observations/k6/counts.jsonl` shows the schedule. `--run --tier <T>` runs a tier, one process at a time, on a quiet host.
- `--packet --records <dir>` writes the compact packet. `observations/k6/k6_packet.json` is the packet of K6's recorded runs (Mac only), and `observations/k6/SHA256SUMS` lists the folder's hashes.
- The runner's tests run with `python3 -m unittest test_k6_runner` from `runner/`, and on the DEC-025 pytest surface through `tests/test_performance_harness_runner.py`.
