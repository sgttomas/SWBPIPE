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

## K6b: W1 observations on the K6 harness (T3 K6b)

K6b adds a fifth mode, `w1a`, to `k6_observe`: K4's W1a kernel method (`FK/structural/retained/`) through FK's public `structural::retained_api`. Observation only, as K6.

- **The adapter** (`src/k6/w1/adapter.rs`) maps a K6 model to K4's `SourceParts`: R1's node order, member ids 1..m, every restraint a constraint at 0, one load per nonzero DOF (`k6:<DOF>`), one station per member at 0.5. Its K4SRC bytes equal those of an independent Python writer (`runner/k6b_sources.py`, `observations/k6b/sources.txt`).
- **The run:** per repeat, `w1_source` (the adapter) and `w1_solve` (one `solve_case`, with `CaseLimit` and `InvocationMeter` at `u64::MAX` unless `--case-limit`/`--invocation-limit` say otherwise). It prints the outcome, one `attempt` line per attempt (work by stage, own and shared, storage counts, and the charged work no stage records when a build stopped partway), and parity lines (W1's counts against each attempt's storage, the work closure, the stage totals, the budget). The stage totals equal the charged totals on completed builds (a stop in the stop rule included); on a build K4 stopped partway (`Failed(Stop(_))`) at most one side (own or shared) falls short, and the remainder is reported as unstaged. `--w1-prefixes` re-runs the call under the case work charged through each segment of the schedule (budget-truncated prefixes), so memory and time are observed per segment. `--dump-published` writes the rows (`k6b-rows v1`); `--emit-source` prints K4SRC.
- **The counts** gain W1's keys: K4's structural profile (RCM on the structural free–free pattern), the published rows, limbs per entry by precision, K4's storage bytes, and the W1 estimate (`estimate_adm_bytes_w1a`, with its terms), derived from the struct definitions (`src/k6/w1/counts.rs`). It bounds every modelled phase, each `Vec` growth under the allocator's move model: the builds, each solve with its fallback, each verification build and pass (with the shift), each decision and the end. A test recomputes every line of `observations/k6b/counts.jsonl` from the code.
- **The analysis** (`runner/k6b_analysis.py`, standard library): `--project` projects W1-T3 and W1-T4 from the 100-member runs, and `--packet --records <dir>` writes K6b's packet (K6's packet plus, per `w1a` process, the W1 figures, the attempts' work by stage and precision, the ratios to W1's estimate and the W1/binary64 time multiple). `observations/k6b/k6b_packet.json` is the packet of K6b's recorded runs (Mac only; W1-T4 at 10,000 members pre-KF3), and `observations/k6b/SHA256SUMS` lists the folder's hashes.
- **The tests** (`tests/k6b_*.rs`) check determinism, equality with K4's own `solve_case` and `solve_cases`, R1's unchanged predicate at 10 and 100 members (from `K4T/r1_large.txt`, its sha256 asserted), the counts, the work closure, the prefixes and the estimate. No test asserts a time or memory bound.

## Criterion 9: release timing of the product entry (RT-U4)

This part times the product's linear entry, as the desktop solve worker runs it, on invented
reference racks. DEL-04-05 owns the measured quantity and the base-machine method. It is observation only: no test or record asserts a time or
memory bound.

- **The racks** (`rack/rack_model.py`, standard library only) are deterministic invented pipe
  racks of an exact node count, built from 41-node bays. The profiles are:
  - `linear-ex`: the exact route, with the static case set and its subtractions;
  - `nonlinear-ex`: OPE with lift-off, friction and gap supports;
  - `nonlinear-pf`: its pressure-free twin.

  Only forms that main admits are emitted. `LEDGER` (`--ledger`) lists each target form that
  is pending, with its interim form and what it waits for. `observations/rack/models_sha256.txt`
  holds the scaling series' hashes; `--hashes` regenerates them. The model version is read from
  `model_format.rs`.
- **The driver** (`core/solver/release_timing/`, bin `release_timing`, its own crate and lock) mirrors the
  worker's ordinary solve: read, parse, request wrapper, entry, serialize, release of the
  envelope, then `parse_back`, and nothing else in the process. `parse_back` is a cheaper
  stand-in for the worker's read-back of its result text, and a lower bound on it: one streamed
  parse that keeps nothing. The worker's read-back is the attestation digest (`d_S`,
  `result_text_digest.rs`, in the desktop crate, which the driver cannot link): it materialises
  each leaf value up to 256 KiB (a result below that is materialised whole), collects and sorts
  the members of larger objects, decodes escaped keys, validates the values and serialises them
  canonically. The driver reproduces none of that, so it does not reproduce the worker's
  read-back peak. It uses serde_json features equal to the desktop's.
  - `--model-checks` is its own invocation, in a process of its own: the desktop's on-demand
    model checks (`check_preview_model`, a menu command run in its own check worker), as that
    worker runs them: read, parse, `model_checks` (the public entry `model_checks_owned`) and
    serialize. A solve run never runs them.
  - It prints one `release-timing-v2` JSON line (`invocation` `solve` or `model_checks`) with
    seconds, `ru_maxrss` and phys_footprint per stage; a solve adds rows and bytes per kind, case
    and row class, and diagnostic counts, and the model checks a count of what the report shows.
    After each stage it also writes a `release-timing-stage {json}` progress line to stderr, so a
    killed run's record shows the last stage it finished and that stage's memory readings.
  - Its `limits` list says what it does not reproduce: the attestation digests and the worker's
    read-back peak, the worker's allocator, IPC and the webview.
  - Its tests pin the written bytes to the product entry's own serialization, and the model
    checks' report to the entry's own report.
  - `release_timing --build-identity` prints the identity embedded at build time: the source
    commit and tree, `source_dirty` and the toolchain. The build script runs on every build, so
    the identity is that of the tree the binary was linked from. `source_dirty` is `true`,
    `false`, or `"unknown"` where git cannot establish it (no git, no repository, no commit), never
    `false` then; the Python runner records its own checkout the same way. The identity says
    clean only when the status is clean and no index entry is flagged assume-unchanged or
    skip-worktree (`git ls-files -v`), since either flag hides a file's changes from the status;
    a clean status with such a flag is `"unknown"`.

  How to read the figures: a solve's stage seconds and memory readings are the driver's, in one
  process. `driver_peak_footprint_bytes` is the driver's lifetime-maximum
  phys_footprint after `parse_back`; it stands where the worker's reported peak stands but is not
  a measured worker peak. The driver's memory at serialisation and after is a lower bound on the
  worker's. A model-checks run's figures are its own process's, never part of a solve's.
- **The runner** (`runner/release_runner.py`, standard library; it reuses `k6_runner`'s
  launch, watchdog and metadata) does four things:
  - `--plan` prints the schedule;
  - `--generate` writes the models and a manifest;
  - `--run` runs one process at a time under `/usr/bin/time -l` and a timeout. Every 100 ms a
    footprint watchdog reads the driver's phys_footprint and lifetime-maximum phys_footprint
    (macOS `proc_pid_rusage`). It kills the run above the 12 GiB base-machine budget, which is
    how the product enforces the budget (`worker_footprint_exceeded`), and the run is classified
    `exceeds_base_machine_budget`. The `ps` RSS watchdog stays as an emergency cap at 16 GiB,
    because RSS runs well above the footprint. `--attribute` (and the `-attr` runs) take a
    `sample` stack snapshot every 2 s, and whenever the footprint has grown by 512 MiB. Each
    snapshot records the top call paths, reduced to product frames. Its timings are not
    measurements. A run id starting `model-checks-` (subset `model-checks`; `criterion` includes
    the 2,000-node ones) runs the driver's `--model-checks` invocation;
  - `--packet` writes the compact packet.

  Each record names the footprint readings its classification compared with the budget
  (`classification_readings`: `used`, and `at_exit`, the runner's reading of the driver at its
  exit, or why there is none). The runner's reading at exit is `/usr/bin/time`'s peak memory
  footprint, which a killed run does not have: `/usr/bin/time` is killed with the driver's
  process group.

  `runner/release_window.sh` is the single exclusive-window entry, run under the host's
  exclusive wrapper; its default `window` subset fits 15 minutes, as do `window2` (linear-ex
  1000 timed, then its attribution twin) and `scaling` (both exact-route profiles at 250, 500
  and 1,000 nodes). `scaling-2000` adds the linear rack at 2,000 nodes and needs a longer
  window (`RELEASE_WINDOW_BUDGET_S`). Result bytes are kept only with `--write-results`.
- **The self-supervising driver** (`release_timing --supervise`, macOS) is the runner's `--run`
  and `--packet` inside the driver binary, for a machine on which nothing may be installed: a
  stock Mac has no Python or git. It reads a plan file, runs each run by re-executing itself as
  the driver in its own session and process group, and watches that child from outside as the
  runner does. Every 100 ms one `proc_pid_rusage` call reads the phys_footprint, its lifetime
  maximum and the resident size. The whole group is killed with SIGKILL above the footprint
  budget, above the 16 GiB resident cap or at the timeout. It classifies each run with the
  runner's rules (`src/classify.rs` ports `classify` line for line) and writes the same records
  and packet, with the same schema names and keys. Where the runner's source is absent
  (`/usr/bin/time`, `ps`), the key is null and a named sibling carries this supervisor's reading:
  the child's final lifetime-maximum footprint, read at exit before reaping, and `wait4`'s
  `ru_maxrss`. The same rules; the driver also has the reading at exit. It reads it on a killed
  run too, where the runner's `/usr/bin/time` was killed with the group, and counts it as the
  stricter observation, so the two do not class every run alike: an emergency RSS-cap kill whose
  sampled footprint was within the budget and whose reading at exit is above it is
  `exceeds_base_machine_budget` here and `killed_by_rss_cap` in the runner. Each record's
  `classification_readings` names the readings its classification used.

  What the self-supervisor promises about its records and its drivers:

  - It writes only entries it created exclusively in the output folder it opened, and reports
    only what it wrote. No plan content and no state of that folder, before or during a run,
    makes it write elsewhere or record a hash that is not of the bytes it wrote. No two records
    it writes can be the same file, by name, by case or by link.
  - A driver it started does not outlive it; a driver it did not start is not affected by it.
  - Its build identity says clean only when Git status is clean and no index flag hides a file;
    otherwise it says unknown or dirty.

  It promises nothing against someone who can replace the `git` executable, forge objects in
  the checkout, write into the output folder's parent between its own operations in ways the
  exclusive-creation rule does not already refuse, or write into the output folder's own entries
  during or after a run: it answers for the bytes it wrote, not for what another writer does to
  a file afterwards. Its build identity does not see files Git ignores. An identity of "clean"
  says that tracked files match the head, that no file outside Git's ignore rules is untracked,
  and that no index flag hides a file; it does not say that no ignored file took part in the
  build. Where a measurement will be cited for exit criterion 9, the build is made in a fresh
  checkout of the head.

  It keeps no result bytes: `--write-results` is refused under `--supervise`, by name, before
  anything is written and before any driver starts, because the driver writes a kept result by
  pathname. The recorded argv is the driver's own arguments, without the supervisor's process
  id. A `.packet.json.partial` that someone plants in the folder during a run makes the next
  packet write fail and the supervisor stop with exit status 2; that costs the rest of the plan
  and no record already written.

  How it keeps them. Run ids are bare names (`[A-Za-z0-9][A-Za-z0-9._-]{0,127}`), and both
  supervisors refuse at plan load, before anything is written, any other id, two ids that are
  the same ignoring case (a case-insensitive volume, the macOS default, opens them as one file),
  an id that is, ignoring case, a name the supervisor writes, and any plan whose ids would make
  two names it writes the same ignoring case (the id `records` would make its stdout record
  `records.jsonl`). The output folder is opened once, as a directory and without following a
  link, and every record is created relative to it, so a later replacement of its path cannot
  redirect a record. Each run needs a new folder: a folder that already holds any name the run
  would write is refused before anything is written and before any driver starts, with a
  message to give `--out` a new folder. Every record is created as a new file (`O_CREAT |
  O_EXCL`), so an existing entry of that name, whether a file, a FIFO, a symbolic link or a hard
  link to a file elsewhere, is never opened for writing; one that appears during the run is
  refused by name as that run's error, before its driver starts. `records.jsonl` is created once
  and written through that one descriptor. `packet.json` is written under a new name and renamed
  into place, which replaces the folder's entry and leaves any other link of the old file as it
  was. The summary printed at the end is the packet just written, never a second read of the
  folder's path. A write to a regular file can still block on a stalled file system: each group
  of record writes is bounded (120 s), and past that the supervisor stops with exit status 4 and
  a message that the records are incomplete (no driver is running then: the run's driver has
  already been reaped). A failed write of a run's stdout record makes the run `error`
  (`stdout_record_write_failed`), with `stdout_sha256` null and `stdout_record_error` saying why:
  no hash of bytes that were not written is recorded. From a driver's start on, any failure
  kills and reaps it. SIGINT, SIGTERM, SIGHUP, SIGQUIT and the other signals that end a process
  without a fault kill and reap the running driver before the supervisor exits (128 + the
  signal). For a death no handler sees (SIGKILL, a fault), the supervisor passes the driver it
  starts its own process id as an argument, and that driver watches it and stops when it is
  gone; a driver started without that argument (directly, or by the Python runner) watches
  nothing, whatever its environment. On macOS the driver exits 125 when the kernel event reports
  its supervisor's exit; on Linux the kernel sends SIGKILL when its parent exits. `--packet` (the
  runner) and the supervisor's packet refuse, by name, a record whose driver line is not
  `release-timing-v2`: "record ID: its driver line has schema S, not release-timing-v2; an
  earlier driver's record is not repackaged".

  A difference from the Python runner: it has no such watch. A Python-supervised run whose runner
  is killed (SIGKILL, or any death that skips its handlers) can leave a driver running, in its
  own session. Find it by its process name and the model path in its arguments, for example
  `pgrep -fl 'release_timing .*linear-ex-n2000.json'`, and stop it with `kill`. The Python
  runner opens record paths by name and does not pin its output directory; replacing that path
  can redirect its record reads and writes. With `--write-results` it passes the driver a
  pathname rather than a directory descriptor, so kept result bytes have the same limitation.

  Host facts come from `sysctl`, and the cargo
  and sweep checks from the process table. The source commit, tree, dirty flag and toolchain are
  embedded at build time (`--build-identity` above). It takes no stack snapshots, so a plan with
  `-attr` runs is refused. `tests/test_release_timing_supervise.py` runs both supervisors on the
  same plan and models, on cases away from that boundary, and compares their classifications,
  outputs and keys; the boundary itself is tested in the crate's `tests/supervise.rs`.

  The folder to carry, prepared on a development machine from a clean checkout:

  ```sh
  (cd core/solver/release_timing && cargo build --release --locked)
  python3 core/solver/performance_harness/runner/release_runner.py --generate --subset criterion --models FOLDER
  python3 core/solver/performance_harness/runner/release_runner.py --write-plan FOLDER/plan.json --subset criterion --models FOLDER
  cp "${CARGO_TARGET_DIR:-core/solver/release_timing/target}/release/release_timing" FOLDER/
  ```

  `FOLDER` then holds the binary `release_timing`, `plan.json` (`release-timing-plan-v2`: the
  runs, each with its task, model name and sha256, the window budget and the quiet-host wait) and
  the plan's rack models with their
  `manifest.json`. Any subset without `-attr` runs can be written, and `--budget-s` and
  `--quiet-wait-s` set the window and the wait. On the measured Mac, put the folder in the home
  folder (not Desktop or Documents, which ask for permission) and run, in Terminal:

  ```sh
  cd FOLDER && ./release_timing --supervise plan.json --out results
  ```

  It writes only inside `results/`, which must be a new folder (one that already holds any of
  the names below is refused before anything starts): `metadata.json`, `records.jsonl` (one line
  as each run ends), `packet.json` (replaced after each run), and per run its stdout line (`.jsonl`), its
  stderr progress lines (`.stderr.txt`) and the launch record (`.record.json`). It keeps no
  result bytes; `--models DIR` reads the models from another folder. Ctrl-C
  kills the running driver before the supervisor exits. Nothing is installed, and no tool
  outside the binary is run. A copy that macOS has quarantined (one from AirDrop or a download)
  may be refused at first start; copy the folder from a drive instead. Bring back
  `results/`; `packet.json` is the record.
