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
| Worker protocol, memory-pool leases, IPC take | desktop parent | desktop measurement (P2, pending) |
| Decode, `JSON.parse`, reader verdict (wasm), state | webview | desktop measurement (P2, pending) |
| First paint | webview | desktop measurement (P2, pending) |

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
     `core/solver/performance_harness/release`.
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
