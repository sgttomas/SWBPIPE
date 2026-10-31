# Criterion 9: what is measured, and how

**Basis:** `docs/RELEASE_TARGET_v1.0.md` §4, exit criterion 9 (RT-U4): on a 2,000-node rack
model, on the 16 GiB base machine, measured by the DEL-04-05 harness, linear static ≤ 10 s
and nonlinear operating ≤ 60 s *(provisional)*. Main at `7e061cf2e`, model document 0.8.0.

This is a working reading under the provisional values (Agent 0, 2026-10-31). The values and
this reading become final only on the owner's acceptance after the RT-U2 walkthrough. Nothing
here sets a threshold, asserts one in a test or declares the criterion met (DEL-04-05:3). A
target left unmet for a model class is recorded in the release limitations list for the owner,
not relaxed.

## 1. What the clock measures

The clock runs from Analyze → Run in the desktop until the result is verified and first
painted. The engine time and each stage are recorded separately, so a shortfall points at
its stage:

| Stage | Where | Measured by |
|---|---|---|
| Model read and parse, request wrapper | solve worker | `release_timing` (`read`, `parse`, `request`) |
| Solve and publication | `product_physics::run_linear_static_preview_value_with_mode` | `release_timing` (`entry`) |
| Serialization of the result | solve worker | `release_timing` (`serialize`) |
| Result digest parse-back | solve worker (`result_digest`) | `release_timing` (`parse_back`) |
| Worker protocol, memory-pool leases, IPC take | desktop parent | desktop measurement (§6) |
| Decode, `JSON.parse`, reader verdict (wasm), state | webview | desktop measurement (§6) |
| First paint | webview | desktop measurement (§6) |

"Linear static" is every case and combination of the `linear-ex` rack document. "Nonlinear
operating" is the OPE case of the `nonlinear-ex` rack.

## 2. The reference racks

`core/solver/performance_harness/rack/rack_model.py` generates the racks deterministically
for any node count, from invented data only. Their sha256 for the scaling series (250, 500,
1,000, 1,500 and 2,000 nodes) is in `observations/rack/models_sha256.txt`. Every run records
its model's sha256.

- **Bay.** One bay is 41 nodes:
  - a 16-node S270 header at 1.5 m with rests, guides and a line stop;
  - a four-bend realized expansion loop;
  - an S170 branch with a flanged valve, rests and an end anchor.

  A 2,000-node rack has 48 bays, 1,999 members, 192 bends, 144 rigid components and 97
  pressure regions per case.
- **`linear-ex`.** Exact pressure route. Cases SUS, OPE, HYD (interim), WIN, EQX, EQY and EQZ,
  with the subtractions EXP, dWIN, dEQX, dEQY and dEQZ.
- **`nonlinear-ex`.** OPE, with lift-off, friction-shoe and gap supports.
- **`nonlinear-pf`.** `nonlinear-ex`'s interim twin on the pressure-free route, kept until EX
  admits nonlinear supports.
- **Interim forms.** The generator emits only what main admits. Each target family or case
  form that main refuses is in the generator's `LEDGER` with its interim form, the refusal
  observed and what it waits for:
  - the tee, the reducer and the EX nonlinear supports wait for RT-A3 increments 1, 2 and 4;
  - the lift-off friction shoe waits for S7;
  - SAM waits for #1678;
  - HYD's test condition and the SRSS and absolute-sum combinations wait for 0.9.0 S1;
  - nonlinear solves above about 1,760 nodes wait for RT-A3 stage P.

  Until increment 4, `nonlinear-ex` solves OPE linearly and is not a nonlinear measurement.
- **Field shapes.** These mirror the reference system (DEL-09-01 `Work/reference-system.md`):
  the same material values and section sizes, the `RK-` id prefix and no reference-system
  id. No rack value is a reference for an expected result.
- **Desktop file shape.** The documents also carry what the desktop's Open Model Document
  requires and the solve does not read:
  - a `data_boundary` and an empty `diagnostics` list;
  - each node's, pipe's, support's and component's `RK-` tag as its label;
  - load-case `kind` and `status`;
  - empty `terms` on each subtraction.

  The rows and bytes `release_timing` publishes are unchanged by them. Hashes recorded before
  these members were added (the windows of §5) are those of the same racks without them.
- **Model version changes.** The generator reads the model document version from
  `model_format.rs`. When the version moves, `tests/test_performance_rack_model.py` fails and
  names the re-check, and the hashes are regenerated with `--hashes`.

## 3. Meeting "the 16 GiB base machine" on another host

The development host is not the base machine. Its CPU and memory are recorded in every
packet. Each condition is met, or not, as follows.

- **Memory, by the product's own cap.**
  - On a 16 GiB machine the desktop's memory pool is 12 GiB, and one solve's worker may use
    at most the default per-invocation budget of 12 GiB (`memory_pool.rs`,
    `recommended_pool_bytes`, `DEFAULT_INVOCATION_BUDGET_BYTES`). The desktop measures the
    worker's lifetime-maximum phys_footprint and refuses a result above the budget.
  - The runner reads the driver's live and lifetime-maximum phys_footprint every 100 ms
    (`proc_pid_rusage`). It kills a run whose footprint passes 12 GiB, as the desktop would,
    and classifies it, or any completed run whose lifetime maximum exceeds 12 GiB, as
    `exceeds_base_machine_budget`, that is, not met on the base machine. An RSS watchdog at
    16 GiB, the base machine's physical memory, is an emergency stop only. RSS ran about 1.4×
    the footprint in the first window, so RSS is not used to judge the budget.
  - The desktop measurement adds the result's pool leases (R while retained, 2R once taken)
    and the webview's own copies, which share the same 16 GiB.
- **Memory: limits.** A run within the cap here can still page or compress on a 16 GiB
  machine running other applications. Memory pressure, compression and swap are not
  reproduced, and neither is the WKWebView's footprint until the desktop measurement exists.
- **Time: no projection is applied.** The solve runs on one thread (user time ≈ real time in
  RT-A3's window), so the host's single-thread speed governs it. Times from this host are
  indications for the base machine, not measurements of it. Criterion 9's time condition can
  be declared met only from a run on a 16 GiB base machine, which is the owner's to arrange.
  A stated projection, for example a ratio of single-thread benchmarks, would be an estimate
  for the owner to accept or refuse.

## 4. How a measurement is taken

1. **Prepare.**
   - Build the driver with `t3_cargo.sh build --release` in
     `core/solver/release_timing`.
   - Generate the models with
     `release_runner.py --generate --models <dir>`.
2. **Run.** Run `release_window.sh <models> <out> <release_timing> [subset]` as ONE
   `t3_exclusive.sh` invocation. Window rules:
   - it takes all four slots;
   - it lasts no more than 15 minutes; the `window` subset fits, and later runs that would
     overrun are skipped and logged;
   - Agent 0 is told its duration first.

   One process runs at a time, under `/usr/bin/time -l`, with a 100 ms footprint watchdog at
   the 12 GiB budget, a 16 GiB RSS emergency stop and a timeout. The driver prints a progress
   line after each stage, so a killed run records the stage it reached. Attribution runs
   (`--attribute`) also take stack snapshots every 2 s and on each 512 MiB of growth, to tie
   memory growth to code. Their timings are not measurements.
3. **Record.** The packet (`packet.json`) records:
   - the host, the build and the model sha256;
   - stage seconds and the peak RSS and footprint;
   - rows, bytes and classification;
   - the conditions, including concurrent activity.

   Result bytes are not kept unless asked: a 2,000-node result is gigabytes.

## 5. Observations so far

### 5.1 First exclusive window

The first window ran on 2026-10-31 from 10:07:38 to 10:15:23Z under `t3_exclusive.sh`, on the
development host (Apple M5 Max, 18 cores, 128 GiB). `release_timing` was built from #31
merged with main `b33a7d358`, in sparse mode. No cargo jobs were running, but the host was not
quiet by K6's test: memorystatus was 76 against ≥ 80, and the load averages were
6.6/15.3/19.9. CPU time was within 2% of wall time.

| Rack | Nodes | Entry (s) | Total (s) | Rows | Result | Peak footprint | Peak RSS |
|---|---|---|---|---|---|---|---|
| linear-ex | 250 | 22.1 | 23.2 | 471,818 | 0.46 GB | 2.63 GB | 3.65 GB |
| linear-ex | 500 | 63.6 | 65.9 | 946,742 | 0.92 GB | 5.28 GB | 7.31 GB |
| linear-ex | 1,000 | — | killed at 211 s (RSS watchdog, then 12 GiB) | — | — | not read | 12.7 GB |
| nonlinear-pf | 250 | 16.6 | 16.7 | 21,960 | 0.02 GB | 0.34 GB | 0.43 GB |
| nonlinear-pf | 500 | 83.8 | 83.9 | 43,940 | 0.03 GB | 1.15 GB | 1.50 GB |
| nonlinear-pf | 2,000 | — | 0.1, refused (`NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED`) | — | — | 0.08 GB | |

- **Entry time.** linear-ex grows as about n^1.5.
- **Stage peaks at 500 nodes.** The footprint was 3.33 GB after the entry and 3.33 GB after
  serialization, then 5.28 GB after the parse-back. The parse-back mirrors the worker's
  attestation digest (`result_digest`).
- **The 1,000-node run.** RSS was 1.6 GB at 193 s, then jumped to 12.7 GB by 211 s. The
  watchdog then judged the run by RSS. That judgement is replaced by the footprint watchdog
  above, and the run is repeated with attribution.

### 5.2 Second exclusive window: 1,000 nodes, with attribution

The second window ran on 2026-10-31 from 10:53:24 to 11:02:22Z under `t3_exclusive.sh`, on the
same host, with `release_timing` at #31 `82daa118b` (the footprint watchdog). No cargo jobs
were running; memorystatus was 76 and the 1-minute load about 5.

- **Timed run (linear-ex, 1,000 nodes): ok.**
  - It published 1,895,142 rows, a 1.84 GB result.
  - Stage seconds: entry 235.2, serialize 1.2, parse-back 3.5; total 240.4.
  - Lifetime-maximum footprint: 6.42 GB after the entry, 10.57 GB after the parse-back. That
    is under the 12 GiB budget, by 2.3 GB.
  - Peak RSS was 14.7 GB, so the earlier RSS watchdog's kill at 1,000 nodes was not the
    product's.
- **Attribution twin** (stack snapshots; its timing is not a measurement).
  - 0–216 s: the seven cases. The footprint grows from 0.17 to 1.39 GB. Most time is in
    `u4b_rows::published` (RT-A3 increment 1b); each case also re-forms and refactors the
    sparse system.
  - 216–222 s: `exact_combinations` build and append take the footprint from 1.39 to
    4.14 GB, with a transient lifetime maximum of about 6.4 GB. The causes are the combined
    rows, the per-row combination evidence (16% of the envelope's bytes on a 128-node rack)
    and per-operand key indexes.
  - Then serialization adds the 1.84 GB text, and the attestation parse-back adds about
    4.2 GB.
- **What this means for 2,000 nodes.** Extrapolated, the peak is about 21 GB today, and
  about 13 GB without the parse-back. That is still over the budget, so criterion 9's linear
  memory needs the result-volume design as well.

### 5.3 Dry runs

These dry runs are not evidence. They were taken on the development host on one shared slot,
not in an exclusive window, with `release_timing` at `7e061cf2e`, sparse mode.

| Rack | Nodes | Entry (s) | Rows | Result bytes | Peak footprint |
|---|---|---|---|---|---|
| linear-ex | 250 | 26.0 | 471,818 | 0.46 GB | 2.7 GB |
| linear-ex | 500 | 73.5 | 946,742 | 0.92 GB | 5.6 GB |
| nonlinear-pf | 500 | 97.0 | 43,940 | 0.03 GB | 1.2 GB |
| nonlinear-pf | 1,000 | 1,465.9 | 87,948 | 0.07 GB | 4.2 GB |

- **The linear rack's result volume dominates.**
  - It publishes about 1,890 rows per member across its 12 case and combination sets, and
    93% of them are the five station rows.
  - Footprint grows at about 5.9 kB per published row.
  - Extrapolated, a 1,000-node rack approaches the 12 GiB budget, and a 2,000-node rack
    (about 3.8 M rows, 3.7 GB of JSON) exceeds it.
  - Entry time grows faster than linearly; RT-A3's increment 1b removes the quadratic row
    lookup.
- **The nonlinear twin's time** is in the nonlinear loop's dense Cholesky, refactored on
  every sliding-friction iteration (`factor_structural_cholesky` under
  `solve_iteration_with_sliding_friction_evidence`). It grew about 15× from 500 to 1,000
  nodes, with two outer iterations in each run. That is RT-A3 stage P's scope.

RT-A3's exclusive window (2026-10-31 01:39–01:50Z) measured its own scratch rack with 8 cases:

- EX linear: 71 s at 1,000 nodes and 346 s at 2,000 nodes, with 2.05 M rows and 1.51 GB;
- PF nonlinear: 149 s and 11.2 GB at 1,500 nodes;
- 2,000 nodes refused by the dense-memory guard.

## 6. Desktop measurement

The desktop part of the clock (§1's last three rows) is measured on the app itself. Its pieces
are in `apps/desktop`:

- **Timeline** (`src/services/solveTimeline.ts`). It records observations only and changes
  no behaviour. The latest solve's marks are taken at `performance.now()`:
  - `solve_requested`, `job_started`, then `job_done` (with `result_bytes`,
    `worker_peak_footprint_bytes` and `worker_peak_grant_bytes`);
  - `take_start`, `take_end` (bytes), `decode_end`, `parse_end`;
  - `reader_verdict_start` and `reader_verdict_end` (registered or not, with the reason code);
  - `analysis_run_built`, `result_set`, `result_committed` (the layout effect), and
    `first_paint` (two animation frames later).

  Every pass of `checked_json_text`, `native_fingerprint`, `reader_written_text` and each wasm
  reader call is counted and timed per stage. A pass counts as full-envelope when it reads at
  least half the result's text. The first failure is recorded with its stage, its leading
  code and its error class.

  The timeline holds timings, counts and codes only: no model data, path, identifier or
  message text. It is read through its own frozen, read-only global,
  `__openPipeStressSolveTimelineV1.readCurrent()`. That global is installed under the very
  condition that installs `__openPipeStressUiDiagnosticsV1` (`uiDiagnostics.ts`), so it is
  present exactly where that one is. The diagnostics global and its snapshot are unchanged.
- **Result injection** (`src-tauri/src/native_smoke_inject.rs`), compiled only under the
  `native-smoke` feature. When `SWBPIPE_NATIVE_SMOKE_INJECT_RESULT` names a file, a started job
  skips its worker and publishes the file's bytes. Everything after that is the product's own
  path: the result lease, the attestation (the worker's `d_M` and `d_S` functions),
  publication, poll, take and IPC. What differs from a solve:
  - no worker runs, so `worker_peak_footprint_bytes` stays unset;
  - `d_M`, `d_S` and their parsed values are computed in the app process before `job_done`.

  The file must be `release_timing --write` output from the very document the app opens.
  `native_smoke_reach_scan_tests.rs` fails if the variable or the module is named outside the
  gate. A product release build (no feature) does not contain the variable's name; the check
  is in the PR record.
- **Driver and runner.**
  - `native-smoke/perf-driver.js` opens the model, runs Analyze → Run (sparse) and waits for
    first paint or a failure. It then attempts Record Run… and Save Local Project and records
    each outcome verbatim.
  - `native-smoke/perf.mjs` builds a release app with the feature (`--build`). It runs each
    planned run (one model, one mode, one launch, a throwaway HOME) and samples the app, its
    workers and the WebKit processes started for it every 200 ms (`perf-sampler.py`,
    `proc_pid_rusage`). It then inspects the SQLite store, classifies the run and writes
    `perf-report-<UTC>.json`. The classes are `ok`, `refused_with_message`, `crash` (with the
    process), `hang`, `misleading_message` and `half_written_save`.
- **Racks the desktop opens.** The generator writes the desktop's model-file shape (§2):
  `data_boundary`, `diagnostics`, labels, load-case `kind` and `status`, and subtraction
  `terms`.
  - `apps/desktop/src/features/model-file/rackModelOpen.test.ts` passes each profile through
    Open Model Document's own check (`checkNativeOutcome`).
  - At 82 nodes, `release_timing` publishes the same rows (153,178) and bytes (147,491,921)
    with these members as without them.
  - The window below ran on documents carrying the same members, added by an interim
    adapter that this change retires.

### 6.1 Desktop window

The window ran on 2026-10-31 from 12:43:58 to 12:56:24Z, as one `t3_exclusive.sh` invocation
(746 s, budget 880 s). It ran on the development host, an Apple M5 Max with 18 cores and
128 GiB. **That host is not the 16 GiB base machine.** On a 16 GiB machine, the footprints
below would page or be killed long before these times.

The app was a release build with the `native-smoke` feature, from P2 at `6c2f02d81` (main
`b33a7d358`). Each run solved `linear-ex` racks in sparse mode, with a timeout per run.
Footprints are the 200 ms sampler's peaks. Result sizes are the `release_timing --write`
files; "app" is the app process, not its worker.

| Run | Result | Outcome | Last stage seen | Run (s) | WebContent peak | App peak | Worker peak |
|---|---|---|---|---|---|---|---|
| injected, 250 nodes | 0.46 GB | hang | `job_started`, then no progress | killed at 130 | 20.9 GB | 2.4 GB | — |
| injected, 500 nodes | 0.92 GB | crash (WebContent) | `take_start` | killed at 130 | 36.3 GB, gone at 36 s | 4.8 GB | — |
| injected, 1,000 nodes | 1.84 GB | crash (WebContent) | `take_start` | killed at 130 | 38.4 GB, gone at 43 s | 9.6 GB | — |
| injected, 2,000 nodes | 3.68 GB | refused: `PREVIEW-SOLVE-RESULT-UTF8-INVALID` | `take_end` (27.5 s) | 33.5 | 15.5 GB | 19.1 GB | — |
| native, 250 nodes | about 0.46 GB | hang | `job_started` | killed at 150 | 19.3 GB | 1.1 GB | 1.7 GB, ended at 25 s |
| native, 500 nodes | about 0.92 GB | crash (WebContent) | `job_started` | killed at 170 | 35.0 GB, gone at 127 s | 2.1 GB | 3.4 GB, ended at 72 s |

- **Where the webview stops.** In every run below 2,000 nodes, the driver's own progress
  stopped while the webview handled the result. The main thread was busy in the take, decode,
  parse or reader stages, so it could neither report nor paint. WebContent grew to 19 to 38 GB.
  At 500 and 1,000 nodes its process ended, and a fresh, idle WebContent replaced it. No run
  reached `reader_verdict_end`, so none reached first paint, Record Run or Save Project. No
  store was written.
- **The 2,000-node run.**
  - The 3.68 GB result was published.
  - It was taken in 1.2 s: `take_end` at 27.5 s, after the app computed `d_M` and `d_S`. The
    app's peak was 19.1 GB, 16.7 GB of it after `job_done`.
  - It was then refused at decode as "not UTF-8". The bytes are `release_timing`'s own UTF-8
    output. The likely cause is the webview's limit on one decoded string (JavaScriptCore's
    string length), so the message probably misstates the cause. That is an inference, not
    checked here.
  - Save Local Project then saved the model without a result, which is correct, and Record
    Run was unavailable (`RUN-RECORD-NO-RUN`).
- **An earlier refusal.** At 82 nodes, the earlier functional run reached the reader. There
  the verdict took about 70 s and refused with `SOURCE_PHYSICS_RHS_DIRECTION`, because seven
  pressure groups publish `direction_component_magnitude` = 1.0000000000000002 and the reader
  requires values ≤ 1. Every `linear-ex` rack meets that refusal before first paint, whatever
  the webview's limits.

What this means for criterion 9: on today's main, the desktop cannot show a `linear-ex` result
of 0.46 GB or more. The time and the memory are in the webview's handling of one whole-envelope
result: the take, decode, `JSON.parse`, the fingerprint and serialization passes, and the wasm
reader. They are not in the solve. Meeting the criterion needs the result-volume design (§5.2),
not only a faster solve.
