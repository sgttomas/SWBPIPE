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
- `python3 -B runner/k6_runner.py --plan --counts observations/k6/counts.jsonl` shows the schedule. `--run --tier <T>` runs a tier, one process at a time, on a quiet host.
- `--packet --records <dir>` writes the compact packet. `observations/k6/k6_packet.json` is the packet of K6's recorded runs (Mac only), and `observations/k6/SHA256SUMS` lists the folder's hashes.
- The runner's tests run with `python3 -B -m unittest test_k6_runner` from `runner/`, and on the DEC-025 pytest surface through `tests/test_performance_harness_runner.py`.

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
  - `nonlinear-pf`: its pressure-free twin;
  - `baseline-one-case-ex`: not a criterion 9 profile. It is the headless baseline's one-case
    model: `linear-ex`'s rack with its operating case (weight, pressure, temperature) alone and no
    combination ([The headless baseline](#the-headless-baseline)).

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
    commit and tree, `source_dirty` and the toolchain. The build script runs on every build. The
    identity records the checkout's HEAD commit and tree and its reported Git state at build
    time; it does not establish the identity of every input used to build the executable.
    `source_dirty` is `true`,
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
  - A driver started by this supervisor receives `--supervised-by <pid>` and stops when that
    parent exits. A driver invoked without `--supervised-by` watches no supervisor, whatever its
    environment. An invocation supplying a stale or non-parent pid exits 125.
  - Its build identity says clean only when Git status is clean and no index flag hides a file;
    otherwise it says unknown or dirty.

  It promises nothing against someone who can replace the `git` executable, forge objects in
  the checkout, or write into the output folder's parent between its own operations in ways the
  exclusive-creation rule does not already refuse. The exclusion for changes to the output
  folder's files applies only to deliberate modification of their contents by another person. It
  does not exclude ordinary folder moves, accidental corruption or the tool's own operations.
  Consumers must verify recorded integrity information before relying on stored files. No
  reader here currently performs that verification; the manifest and verifying reader belong to
  the follow-up.

  <a id="what-it-does-not-yet-do"></a>Two things it does not yet do, which a reader of a run's records must know:

  - **The build identity does not establish that the executable was built from the named
    revision alone.** It says that tracked files match the head, that nothing outside Git's
    ignore rules is untracked and that no index flag hides a file. A file Git ignores can take
    part in the build: an ignored build script in a dependency crate was compiled and run with
    the identity still naming the commit as clean. Until the tool lists ignored files and marks
    the identity incomplete, a run's identity is not evidence about the named revision unless
    the build was made in a fresh checkout of that revision, and the tool does not check that.
  - **No reader verifies what it reads back.** The supervisor records a hash of the bytes it
    wrote for each run's stdout record only; the aggregate record, the metadata, the packet, the
    stderr lines and the launch record have no recorded hash, and neither `--packet` nor any
    other reader here recomputes one. A folder carried from one machine to another is not
    evidence by itself. Until a manifest and a verifying reader exist, note the SHA-256 of
    `packet.json` by hand where the run is made (`shasum -a 256 results/packet.json`).

  It keeps no result bytes: `--write-results` is refused under `--supervise`, by name, before
  anything is written and before any driver starts, because the driver writes a kept result by
  pathname. The recorded argv contains the driver arguments with absolute paths reduced to
  basenames and the `--supervised-by <pid>` pair omitted.

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
  link to a file elsewhere, is refused. An existing entry is never opened for writing. If a
  per-run stdout, stderr or launch-record name appears during the plan, that run is refused when
  its files are prepared, before its driver starts. A planted `.packet.json.partial` fails the
  next packet write. A planted `packet.json` entry is replaced by the packet rename, without
  opening its target. A failed packet write stops the supervisor with exit status 2; that costs
  the rest of the plan and no record already written. `records.jsonl` is created once
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

  The folder to carry, prepared on a development machine from a fresh checkout of the named
  revision:

  ```sh
  (cd core/solver/release_timing && cargo build --release --locked)
  python3 -B core/solver/performance_harness/runner/release_runner.py --generate --subset criterion --models FOLDER
  python3 -B core/solver/performance_harness/runner/release_runner.py --write-plan FOLDER/plan.json --subset criterion --models FOLDER
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

## The headless baseline

This baseline does not measure criterion 9's linear workload. That workload is the profile
`linear-ex` (seven cases and five combinations). The baseline is a timing that a second worker
can reproduce from the repository alone, on two exact-route models. Each is solved by the
driver in sparse mode, in a process of its own:

- W1, `baseline-one-case-ex` at 2,000 nodes: `linear-ex`'s rack with its operating case alone
  and no combination. The product solves every case and combination a document holds, so the
  only way to time one case is a model that holds one case.
- W2, the committed reference system, `fixtures/reference_system/reference_system.json`.

The runner's named plan `baseline` holds ten runs, W1, W2, W1, W2, ..., five of each. Their run
ids end `-r1` to `-r5`. `baseline-with-rack` holds the same ten, then one run of `linear-ex` at
2,000 nodes. That run is the default plan's row, with the same expectation,
`exceeds_base_machine_budget`. It states criterion 9's own linear workload beside the baseline
on the same revision, and it is never a timing. A named plan fixes its mode, its limits, its
window and its quiet-host wait. `--plan --subset baseline` prints them, and the runner refuses
any flag that would change them, `--generator` included.

The window is the quiet-host wait, every run's timeout and a planning allowance of 30 s per run
for the supervisor's own work between runs: hashing the model, the host check, the launch and
the records. The allowance is a planning figure, not an established bound on that work, and
nothing enforces it. So the window fits every run at its full timeout provided that work stays
within the allowance. Where it does not, the supervisor cuts a later run's timeout to what
remains, or skips the run, and the run's record says so: `timeout_capped_by_window` true, or
`skipped_window_budget`. `--baseline-set` (step 8) then reports the set INCOMPLETE BY THE WINDOW.
This is a scope decision, not a necessity. A bounded window could instead decline to start a
run whose full timeout no longer fits; both supervisors are kept as they are by choice.

Whoever measures states their thresholds in their own measurement contract; this section sets
none. The steps below are the procedure. The revision to measure, the acceptable values and what
follows from a verdict belong to that contract.

On a shared host, run the build through the host's cargo wrapper and the supervised run through
its exclusive wrapper. A refusal by either is a host condition, not a result.

1. **A controlled build.** Make a fresh clone of the revision in a new directory (not a worktree
   of a checkout that has been worked in) and a new, empty build directory `BUILD` outside it.
   From the clone's root, build with an environment that holds only these four names. On a
   shared host, put the host's cargo wrapper where `cargo` is:

   ```sh
   env -i HOME="$HOME" PATH="$HOME/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin" \
       CARGO_TARGET_DIR=BUILD CARGO_BUILD_JOBS=4 \
       cargo build --release --locked --manifest-path core/solver/release_timing/Cargo.toml
   ```

   So no `RUSTFLAGS` of any spelling, compiler wrapper, linker or SDK variable, or
   `CARGO_PROFILE_` variable can reach the build. Record in `build-facts.txt`, before the build
   and again after it (the two must agree):

   - the Git state: `git rev-parse HEAD` is the revision; `git diff --quiet HEAD` exits 0; and
     `git -c core.fsmonitor=false status --short --ignored --untracked-files=all` prints nothing.
     A command that fails is not an empty answer.
   - Cargo configuration, which fails closed. Check `.cargo/config` and `.cargo/config.toml` in
     the clone, in every parent directory up to the root, and in `$HOME/.cargo/`. List each file
     that exists with its path and SHA-256. Any such file makes the build unfit unless its whole
     text is recorded and it holds none of these:
     - a `[build]`, `[target]`, `[profile]`, `[env]`, `[patch]`, `[source]` or `[unstable]`
       table;
     - an `include`;
     - a `rustflags`, `linker`, `rustc`, `rustc-wrapper` or `rustc-workspace-wrapper` key.
   - the toolchain, each as its command prints it: `rustup show active-toolchain` (in the clone),
     `rustc -Vv`, `cargo -V`, `xcode-select -p`, `xcrun --show-sdk-version`,
     `xcrun --show-sdk-path`, `cc --version`, `ld -v`, `sw_vers` and `uname -m`;
   - the SHA-256 of `core/solver/release_timing/Cargo.lock`;
   - the names, never the values, in the build's environment:
     `env -i HOME="$HOME" PATH=... CARGO_TARGET_DIR=... CARGO_BUILD_JOBS=4 env | cut -d= -f1 | sort`
     prints exactly `CARGO_BUILD_JOBS`, `CARGO_TARGET_DIR`, `HOME` and `PATH`.

   After the build, the executable is `BUILD/release/release_timing`, and the commands below
   name it by that path. `BUILD/release/release_timing --build-identity` must name the revision
   with `source_dirty: false` and `build_profile: "release"`. Record the executable's SHA-256
   (`shasum -a 256 BUILD/release/release_timing`). Two such builds from two clones are expected
   to differ in SHA-256, because the clone's path is compiled in. Compare their toolchain facts
   and the product's output bytes instead. Read the first item of
   [What it does not yet do](#what-it-does-not-yet-do): this step is what stands in for it.
2. **Produce the models:**

   ```sh
   python3 -B core/solver/performance_harness/runner/release_runner.py --generate --subset baseline --models FOLDER
   ```

   W1 is written by the repository's rack generator, the same bytes as
   `python3 -B core/solver/performance_harness/rack/rack_model.py --profile baseline-one-case-ex --nodes 2000 -o FILE`.
   `--generate` compares its SHA-256 with the line `baseline-one-case-ex  2000` of
   `observations/rack/models_sha256.txt`, which `tests/test_performance_rack_model.py` holds
   against the generator. W2 is copied only after
   `python3 -B fixtures/reference_system/generate_reference_system.py --check` exits 0, which shows
   the committed file is what its generator writes. No other file states its hash: it is the
   SHA-256 of the committed file at that revision (`shasum -a 256 fixtures/reference_system/reference_system.json`).
   `FOLDER/manifest.json` records both hashes. If W1's hash differs from its line, or W2's check
   fails, `--generate` names the model (for W1 also the expected and the found hash), leaves it
   out of the manifest and exits 1. The plan cannot then be written.
3. **Write the plan.** Each run then carries its model's SHA-256 from the manifest. First every
   model of the plan is compared again, both its manifest record and the file itself in
   `FOLDER`: W1 with its line of `observations/rack/models_sha256.txt`, W2 with the SHA-256 of
   the committed file at this checkout. If a model is missing or differs, nothing is written and
   the runner exits 2. So a
   models folder made any other way, or at another revision, cannot become a named plan.
   `--plan --models FOLDER` and `--run --models FOLDER` make the same comparison.

   ```sh
   python3 -B core/solver/performance_harness/runner/release_runner.py --write-plan plan.json --subset baseline --models FOLDER
   ```

4. **Check the host before the run**, and record each command with what it printed in
   `host-before.txt`:
   - `pgrep -x cargo` and `pgrep -x rustc`, each of which should print nothing. The supervisor's
     own quiet test looks for cargo and evidence sweeps, not for rustc.
   - `sysctl -n kern.memorystatus_level`, which must print a number. The supervisor's own test
     accepts an unreadable level.
   - `sysctl -n vm.loadavg`.
   - `df -g` of the volume the run writes to.
5. **Start the process log** before the supervised run, and stop it after the run:

   ```sh
   while :; do printf 'T %s\n' "$(date -u +%FT%TZ)"; ps -Ao pcpu=,pid=,comm= -r | head -12; sleep 5; done > host-processes.log
   ```

6. **Run the plan**, under the host's exclusive wrapper on a shared host, into a folder that does
   not yet exist:

   ```sh
   BUILD/release/release_timing --supervise plan.json --models FOLDER --out NEW_FOLDER
   ```

7. **Read the set's verdict**, into the file the bundle keeps:

   ```sh
   python3 -B core/solver/performance_harness/runner/release_runner.py --baseline-set NEW_FOLDER --plan-file plan.json --subset baseline > baseline-set.json
   ```

   It reads exactly one supervised folder, writes nothing there and prints a JSON report. First
   it holds `plan.json` to the named plan at this checkout: every top-level field (the schema,
   the window, the quiet-host wait), and every field of each run except two. The run's
   `expected_model_sha256` must instead be the repository's own. Its `model_stats` is the
   generator's description and is not compared. A run is a completed timing only if all of
   these hold:
   - its `classification` is `ok` and `as_expected` is true;
   - `driver.route` is `EX`;
   - the plan's `expected_model_sha256`, the record's `model_sha256` and
     `driver.document.sha256` are one value;
   - `driver.seconds.entry` and `launch.rss.exit_lifetime_max_phys_footprint_bytes` are numbers;
   - `driver.output_sha256`, `driver.rows` and `driver.output_bytes` are present, and equal
     those of the workload's other completed runs.

   For each workload the report gives its completed timings, with the median, minimum and
   maximum of `driver.seconds.entry`. That is the product's solve entry: the `entry` member of
   `seconds` in the driver's line, which is the `release-timing-v2` object in
   `NEW_FOLDER/<run id>.jsonl`, repeated in `packet.json` as `stage_seconds`. The verdict and its
   exit status are one of these:
   - **COMPLETE (0)**: every run of each workload is a completed timing, and no run was skipped
     for the window or timed out under a capped timeout. A run whose timeout was capped but
     which completed is unaffected.
   - **INCOMPLETE BY THE WINDOW (3)**: one or more runs were `skipped_window_budget` or timed out
     with `timeout_capped_by_window` true, and there is no finding. The report names those runs.
   - **FINDINGS (1)**: anything else that keeps a run from being a completed timing: a wrong,
     missing or disagreeing output, a memory breach, a timeout at the full timeout, a refusal, an
     error, a model mismatch, or a missing or null figure. Where a finding and a window problem
     both occur, the verdict is FINDINGS and both are listed. A finding is never absorbed into a
     window problem.

   A run that timed out, was killed for memory (`exceeds_base_machine_budget`,
   `killed_by_rss_cap`), was refused, ended in an error or was skipped is never a timing.

   A run the plan does not expect to solve (the `linear-ex` run of `baseline-with-rack`) is never
   a timing either, but it must have run:
   - Each of these is a finding: no record; no launch (`model_missing`, `model_mismatch`, or an
     error before any driver started); or a model run on other bytes than the plan's.
   - A skip for the window is a window problem.
   - Whatever the driver itself then did (a kill, a timeout, an entry error, a result) is
     reported as it is under `not_timings`, with its classification, detail, last stage and
     `as_expected`. It does not decide the verdict.

   `host_not_quiet` lists runs whose own host check was not quiet or read no memory level. It is
   for the measurement contract's rules and does not change the verdict.
8. **The bundle.** Put these in one folder:
   - `out/`, the folder the supervisor wrote. Each run that was launched has three files there:
     `<run id>.jsonl` (the driver's stdout line, empty where it printed none), `<run id>.stderr.txt`
     (its stage lines) and `<run id>.record.json` (the launch record). A run that was never
     launched has none: one skipped for the window, or one whose model was missing or
     mismatched.
   - `plan.json` and `models/manifest.json` (the models themselves can be produced again by
     step 2);
   - `build-facts.txt`, `host-before.txt`, `host-processes.log` and `baseline-set.json`;
   - on a shared host, the exclusive wrapper's own lines for the run;
   - a copy of the measurement contract used.

   Once the supervisor has exited and the process log has stopped, inventory the bundle from its
   root:

   ```sh
   find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 shasum -a 256 > SHA256SUMS
   shasum -a 256 SHA256SUMS
   ```

   Nothing in the bundle is written after that. The second command prints the root digest. State
   it outside the bundle, with the SHA-256 of the contract's copy, in the message that reports the
   run; a later reader takes both from there.
9. **Read a bundle back.** No reader here verifies stored records (the second item of
   [What it does not yet do](#what-it-does-not-yet-do)). Whoever reads a bundle does the
   following, in order, from its root, and stops on the first difference. The hashes show that
   the files are the ones inventoried; they do not show that a record is true. The checks after
   them cover what decides time, memory and classification.

   1. Integrity and membership. `shasum -c` alone does not see an added file, so the third
      command compares the bundle's files with the list's names:

      ```sh
      shasum -a 256 SHA256SUMS          # equals the stated root digest
      shasum -a 256 -c SHA256SUMS       # every line OK
      diff <(find . -type f ! -name SHA256SUMS | LC_ALL=C sort) <(sed 's/^[0-9a-f]*  //' SHA256SUMS)   # prints nothing
      shasum -a 256 contract.md         # equals the stated SHA-256 of the contract
      ```

   2. Items 2 to 6 below are checked by one command, with `REVISION` the full commit measured.
      It prints each difference and exits 1 if there is any:
      - 2. What is consumed is present: the bundle's files, one aggregate record for each run of
        `plan.json`, the three files of each launched run, and no file of a run never launched.
      - 3. For every run, these agree:
        - the aggregate's `launch` with `<run id>.record.json`, in every key the aggregate
          carries;
        - `<run id>.jsonl` with `launch.stdout_sha256`, where that is not null;
        - `last_stage` and `last_stage_memory` with the last stage line of the stderr file, both
          null where there is none;
        - `model_sha256`, where the run read its model, with the plan's and the manifest's.
      - 4. For a run with a driver line, the stdout line equals the aggregate's `driver`. For an
        `ok` run, each stage line's seconds also equal `driver.seconds`, and
        `driver.seconds.total` is the sum of the seven stages to 1e-6 s (there is no total stage
        line).
      - 5. A run with no driver line (a kill, a timeout, a skip) has an empty or absent stdout
        file and a null `driver`. That is not a corrupt bundle; such a run is read for its
        classification, detail and last stage only.
      - 6. `packet.json` lists each run once, with the aggregate's values, and `metadata.json`
        names `REVISION`.

      ```sh
      python3 -B - REVISION <<'EOF'
      import hashlib, json, os, sys
      revision = sys.argv[1]
      problems = []
      def differs(what):
          problems.append(what)
          print('DIFFERS:', what)
      def sha256(path):
          return hashlib.sha256(open(path, 'rb').read()).hexdigest()
      def stage_lines(run_id):
          path = 'out/%s.stderr.txt' % run_id
          prefix = 'release-timing-stage '
          if not os.path.exists(path):
              return []
          return [json.loads(line[len(prefix):]) for line in open(path) if line.startswith(prefix)]
      # 2. What is consumed is present.
      for name in ('out/metadata.json', 'out/records.jsonl', 'out/packet.json', 'plan.json', 'models/manifest.json',
                   'build-facts.txt', 'host-before.txt', 'host-processes.log', 'baseline-set.json', 'contract.md'):
          if not os.path.isfile(name):
              differs('%s is missing' % name)
      plan = json.load(open('plan.json'))
      manifest = json.load(open('models/manifest.json'))['models']
      records = [json.loads(line) for line in open('out/records.jsonl') if line.strip()]
      aggregate = {r['run_id']: r for r in records}
      if sorted(aggregate) != sorted(r['run_id'] for r in plan['runs']) or len(aggregate) != len(records):
          differs('the aggregate does not hold one record for each run of plan.json')
      for p in plan['runs']:
          run_id, r = p['run_id'], aggregate.get(p['run_id'])
          if r is None:
              continue
          files = ['out/%s%s' % (run_id, suffix) for suffix in ('.jsonl', '.stderr.txt', '.record.json')]
          if 'launch' in r:   # launched: its three files
              for path in files:
                  if not os.path.isfile(path):
                      differs('%s is missing' % path)
          else:               # never launched (skipped, model missing or mismatched): no files of its own
              for path in files:
                  if os.path.exists(path):
                      differs('%s exists for a run that was not launched' % path)
          # 3. Every run.
          if 'launch' in r and os.path.isfile(files[2]):
              record = json.load(open(files[2]))
              for key, value in r['launch'].items():
                  if record.get(key) != value:
                      differs('%s: launch.%s differs from %s' % (run_id, key, files[2]))
              if r['launch'].get('stdout_sha256') is not None and (
                      not os.path.isfile(files[0]) or sha256(files[0]) != r['launch']['stdout_sha256']):
                  differs('%s: the stdout file is not launch.stdout_sha256' % run_id)
          lines = stage_lines(run_id)
          last = lines[-1] if lines else None
          stage = {k: last.get(k) for k in ('repeat', 'stage', 'seconds')} if last else None
          memory = {k: last.get(k) for k in ('ru_maxrss_bytes', 'phys_footprint_bytes',
                                             'lifetime_max_phys_footprint_bytes')} if last else None
          if r.get('last_stage') != stage or r.get('last_stage_memory') != memory:
              differs('%s: last_stage or last_stage_memory differs from its stderr file' % run_id)
          if 'model_sha256' in r and not (r['model_sha256'] == p.get('expected_model_sha256')
                                          == (manifest.get(p['model']) or {}).get('sha256')):
              differs('%s: model_sha256 is not the plan\'s and the manifest\'s' % run_id)
          # 4. A run with a driver line; 5. a run without one.
          stdout = open(files[0]).read() if os.path.isfile(files[0]) else ''
          if r.get('driver') is not None:
              lines_out = [line for line in stdout.splitlines() if line.strip()]
              if len(lines_out) != 1 or json.loads(lines_out[0]) != r['driver']:
                  differs('%s: the stdout line is not the aggregate\'s driver' % run_id)
              if r['classification'] == 'ok':
                  seconds = r['driver']['seconds']
                  for line in lines:
                      if seconds.get(line['stage']) != line['seconds']:
                          differs('%s: stage %s seconds differ from driver.seconds' % (run_id, line['stage']))
                  stages = ('read', 'parse', 'request', 'entry', 'serialize', 'release_envelope', 'parse_back')
                  if abs(seconds['total'] - sum(seconds[s] for s in stages)) > 1e-6:
                      differs('%s: driver.seconds.total is not the sum of the seven stages' % run_id)
          elif r['classification'] == 'ok' or stdout.strip():
              differs('%s: no driver in the aggregate but classification ok or a stdout line' % run_id)
      # 6. The packet and the metadata.
      packet = json.load(open('out/packet.json'))
      for row in packet['runs'] + packet['not_run']:
          r = aggregate.get(row['run_id']) or {}
          driver = r.get('driver') or {}
          pairs = {k: r.get(k) for k in ('order', 'classification', 'classification_detail', 'as_expected', 'model',
                                         'model_sha256', 'model_bytes', 'expect', 'profile', 'nodes', 'mode', 'task')}
          pairs.update(route=driver.get('route'), status=driver.get('status'), stage_seconds=driver.get('seconds'),
                       rows=driver.get('rows'), output_bytes=driver.get('output_bytes'),
                       output_sha256=driver.get('output_sha256'))
          for key, value in pairs.items():
              if key in row and row[key] != value:
                  differs('%s: packet.%s differs from the aggregate' % (row['run_id'], key))
      if sorted(row['run_id'] for row in packet['runs'] + packet['not_run']) != sorted(aggregate):
          differs('the packet does not list each run of the aggregate once')
      if json.load(open('out/metadata.json')).get('source_commit') != revision:
          differs('metadata.json does not name the revision')
      print('read-back: %d difference(s)' % len(problems))
      sys.exit(1 if problems else 0)
      EOF
      ```

A set that is INCOMPLETE BY THE WINDOW, or otherwise unusable, is kept as it is. A repeat runs
the whole named plan again, into a new folder. Completed timings are never combined across
attempts: `--baseline-set` reads one folder only.
