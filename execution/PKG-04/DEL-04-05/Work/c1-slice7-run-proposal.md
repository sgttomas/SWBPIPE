# C1 slice 7 run proposal

Serves RT-U4 / criterion 9 and the C1 plan's slice 7. This is an offline-prepared
proposal, not a window grant, measurement, acceptance or memory-fit result.

**Execution status:** the release App, two model documents and baseline P2 plan
are prepared. P1's runner safeguards are implemented offline and await independent
scrutiny; P2 and P3 remain executable prerequisites below. Do not acquire an
exclusive interval or launch the App until those are
ready and the narrow run authority has been granted. Preparation has launched no
App, solve worker, model solve, memory sampler, screenshot or LLM session.

## Candidate and prepared identities

The runtime basis is `a37d1ef1f535155e8ea3f0e330bb7cd7a056bb71`, built clean.
Its only changes from independently cleared
`55fd8f4fccb334e91fe2929e4f93ba0aa3165b1b` are two Work documents. Runtime,
tests, schemas, core and permissions are identical. This proposal and
`apps/desktop/native-smoke/perf-plan-c1.json` and the 500/1,000-node extensions of
`rackModelOpen.test.ts` are later preparation changes, not part of that binary.
Record their submitting revision separately. Any later
runtime, frontend, reader, worker, driver, sampler or plan change must be named
and hashed in the run record; do not silently call it the a37d candidate.

The maintained `perf.mjs --build` completed under `t3_slot.sh`, without launching
its output. Binding: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app/perf-binding.json`.

| Component | Prepared identity |
| --- | --- |
| App | Mach-O arm64, 48,353,808 bytes; SHA-256 `94a2cc83f390163c15c6a3e60e5ed4befbe0a2c68612ca2beaaf1115f520ba19` |
| App path | `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app/openpipestress-desktop` |
| Worker | The exact same executable, `--swbpipe-solve-worker`, launched by `CurrentExeLauncher` with an empty environment; record its PID/argv and the product's reported lifetime footprint/grant |
| Frontend | `dist_digest` = `sha256:37882871bcee062a482c72c236f8a5356115b6d3bae9f0dbd7b8e24868c18422`, embedded by the custom protocol |
| Operation Wasm bytes | SHA-256 `6f584adcbc7b5233c8418075aa23f15fe33d68999042e30261984f34dedb1a3a` |
| Self-weight Wasm bytes | SHA-256 `5adff8809a205605654b1998d2f0bef405aae7462604b880d5f0647a0ab4a5ce` |
| Wasm source stamps | operation `14ab23c574227e35d6cc5c94e5f7a1dde7d8343d8e4eebfdbbd3c35c624cd54a`; self-weight `04c15ac0dded329c0e873985a34b3f73c68fdeab27331cbe02e7925a9539c938` (source stamps, not artifact hashes) |

Build flags are `--release --locked --offline --features native-smoke,tauri/custom-protocol`.
The smoke window is 1440×920 with background throttling disabled. This binary
contains the injection hook and must not be distributed. These runs use native
mode, not injection. Node v24.18.0 and `node:sqlite` are available.

Prepared external harness identities (SHA-256, recheck before execution):

- `perf.mjs`: `9e79ae82f10f7ff84de7ecb732209fca870b6fd691c8225209f66906a1d11714`.
- `perf-driver.js`: `95894a5d6a672ad30f1c5dcff15bcf1149fb7254021837fb32828fe21832807c`.
- `perf-sampler.py`: `99bc61de966068e1e1acf550219a3dd8df4e35e82bc7723fe0c4d69412f69295`.
- `rack/rack_model.py`: `9bbc6b9b1f6f78c11233da9906bf1346d4322a16bfe1b5c5e2069c7914456559`.

Those are the original prepared a37d harness identities. The later P1 preparation
changes `perf.mjs`, `perf-sampler.py` and their tests; it changes neither the bound
App/worker/frontend nor the driver. Bind its submitting revision and re-hash those
external scripts before execution. The old identities do not certify the new
supervision path, and green CI at 867e51b does not cover that later code.

## Inputs and reproducible commands

Generate only the maintained invented `linear-ex` racks. They carry model 0.9.0,
seven primitive cases and five subtraction combinations. Keep the generator's
pending-family ledger: these are its current interim racks, not a claim that all
downstream mechanics, nonlinear EX, or criterion 9 is delivered.

| Model | Nodes / members | File bytes | File SHA-256 | JCS SHA-256 (the solve-proof comparison) |
| --- | --- | --- | --- | --- |
| `linear-ex-n0500.json` | 500 / 499 | 1,828,475 | `42567114a27415b7b042bb9a91d49c8dc640d534953616e5b7efb66b2a2025eb` | `0d45797d8aacada487ef418ce0a776239f6fd9801ed315c1a0f6e1ca2e25466c` |
| `linear-ex-n1000.json` | 1,000 / 999 | 3,649,735 | `fc33acd8f38c2275e5432d4a8b5f1e5aaeb4407532ae83a1c7f484ceb01bb892` | `299aedb4b44b81e9916c4b85650d53d9e96c4c1c728bde8789ed3dddb6f2f312` |

Offline reproduction, from this checkout. Generation, build and plan validation
were executed; the App build was bound at clean a37d before these preparation edits:

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
RT_C1_TARGET=/Users/ryan/dev/chirality-t3/targets/rt-perf-c1
mkdir -p "$RT_C1_KIT/models" "$RT_C1_KIT/app"
/Users/ryan/dev/chirality-t3/tools/t3_slot.sh python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 500 --output "$RT_C1_KIT/models/linear-ex-n0500.json"
/Users/ryan/dev/chirality-t3/tools/t3_slot.sh python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 1000 --output "$RT_C1_KIT/models/linear-ex-n1000.json"
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh node apps/desktop/native-smoke/perf.mjs --build --app-dir "$RT_C1_KIT/app" --target-dir "$RT_C1_TARGET"
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-1000
```

Rebuilding at a later head changes the binding's source identity; keep the
prepared a37d copy or explicitly replace its binding. The two `--plan` commands
have passed. The new plan's default selects only 500; it has no 2,000-node run.
Do not use the old `perf-plan.json` default window, which includes 2,000.

**Conditional execution recipes, not executed:** after prerequisites P1–P3 and
the narrow grant, each command takes its own interval. Set the two variables
above in the calling shell. Do not nest `t3_slot.sh` inside the exclusive wrapper.

```sh
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_KIT/app" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500 --budget-s 840 --out "$RT_C1_KIT/evidence/n0500"
```

Only after the 500-node gate passes, release/reacquire through the same queue:

```sh
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_KIT/app" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-1000 --budget-s 840 --out "$RT_C1_KIT/evidence/n1000"
```

These commands execute the baseline open/solve/Record/Save driver through the P1
supervisor (independent review pending).
They become the complete gate only after the existing runner/driver and the
reader-attribution carrier meet P1–P3. Rebind changed external scripts and any
changed App before using them. No assumed flags or nonexistent profiling command
is presented as executable here.

## Interactions and usable-size gate

One native solve per fresh private App launch/throwaway store; no injected
result, C2 full copy, worker-budget increase or pool reconfiguration. First verify
the solve proof matches the model JCS hash, sparse mode, native result custody,
and nonzero displayed rows. Record every refusal verbatim; do not alter input,
result or standing to make a run progress.

The baseline driver already opens the document, chooses sparse, runs
`analyze.run`, waits for `first_paint`, records a named run and saves the project.
P2 adds the following bounded probes to that same maintained driver:

1. Wait for actual result rows as well as the two-animation-frame timeline mark.
   Select a row, next/previous entity and case, next/previous page, and one filter
   followed by clearing it. Log selected published row IDs, page positions and
   action-to-settled-DOM times. A visible no-match result is distinct from a typed
   refusal. Check the result detail resolves and controls continue responding.
2. Stream one displayed group's CSV through its existing button into the private
   Downloads directory. Check its receipt, native byte length and complete row
   count/CSV structure outside the webview; hold no full row array in the driver.
   Existing byte-parity tests remain the format oracle, not a claim that a live
   file has already been checked.
3. Record Run and Save Project through the actual UI. Inspect SQLite only after
   the App has closed. A named SQLite-size refusal or model-only save is an
   honest outcome, but does not pass this round-trip usability gate. The combined
   SQLite row limit remains; C1 never promised its removal.
4. When a result was saved, use `file.open-local` in the same App session. Require
   historical origin, integrity findings as shown, native paged rows, navigation,
   and a native re-save with preserved evidence. Check the recorded history run
   via the existing History check (read/check/dispose). A restored run must never
   acquire fresh solve/Current authority. Unexpected Replace prompts are cancelled
   and reported, never automatically confirmed.
5. Release held results through normal close/reset and observe pool counters
   returning to their starting retained/worker/custody counts. Do not equate lower
   RSS with lease release or force JS GC to improve the result.

Report each action's latency and completion, not a new accepted usability time
threshold. Passing this sequence establishes use of those C1 paths at the
observed size on the recorded host. First paint, component tests or a small
footprint alone do not. Comparisons/derived JSON and other bounded C2 consumers
remain explicitly excluded. Two sizes on the development host establish neither
2,000-node usability nor the 16 GiB base-machine criterion.

## Memory and attribution

Use the existing sampler at 200 ms: `proc_pid_rusage` resident size,
`phys_footprint` and kernel lifetime maximum for the App PID, workers running the
private executable, and new WebKit PIDs relative to the prelaunch baseline.
Keep the timeline/progress/store report produced by `perf.mjs`. Record actual
host CPU/RAM/OS from that report; the development host is not the base machine.

- Per-PID readings and kernel lifetime maxima are direct observations. A short
  process or unreadable PID can be missed; absence is unknown, not zero. Match
  worker PID/argv and its product-reported lifetime maximum with the sampler.
- New WebContent is attributed by executable path and launch baseline, not proven
  parentage: WebKit launches from launchd. More than one candidate, replacement
  WebContent, an unrelated WebKit launch or missing readings makes attribution
  ambiguous. Do not choose the smallest PID footprint or progress to 1,000 on
  that evidence.
- Compute `max_t(app + worker + WebContent)` from the same timeline rows, with
  missing values marked unknown. Those PID reads occur sequentially in one
  polling interval: this is an aligned sampled sum, not an atomic concurrent
  maximum. Never add separately timed per-process maxima and call it concurrent
  peak. GPU/networking peaks are reported separately; the compact timeline lacks
  their synchronous sum. Shared pages, compression and unobserved transients
  prevent a proof of total base-machine fit from these sums.
- The existing `reader_verdict_start/end` marks cover registration and can contain
  several native reads; `job_done` separates producer completion from delivery
  work. They do not isolate the reader's own heap from parsing. Prescan M is an
  admission bound, not measured resident memory. Subtracting it from App footprint
  would not measure reader working peak.
- P3 must separately report actual parser allocation peak and reader-only peak
  (requested bytes and existing allocation-envelope bytes), with model/input
  parsing already held outside the reader-only count. Cover dispatch and every
  registration reader, hashed row-ID sets/evidence indexes, and the summary and
  semantics-batch working passes reached by these interactions. Reuse the
  maintained `ingress::counting`/allocator test facility and the same Value reader
  APIs behind `operation_applier`; no core/pricing edit or generic profiling layer.
  The existing counter is per-thread and covers Rust allocator requests; identify
  any off-thread or native-library allocation it does not count.
  Identify its exact test binary/source and matched producer inputs. An isolated
  probe establishes component allocations, not the App's concurrent physical
  peak; it must not replace the live P2 run. If the large input or probe is absent,
  this part of the gate is incomplete rather than inferred from small fixtures.

Keep held result bytes, parse M/reservations, counted working heap, response
buffers, App physical peak and WebContent physical peak as separate columns.
SQLite cache/journal/FTS, allocator cache, thread stacks and Tauri delivery remain
physical-process residuals unless directly attributed. Report absolute working
peak and its context; do not adopt a new materiality threshold. A material reader
share needs its bound/lease decision before leaving draft, per the C1 plan.

## Resource limits, cancellation and cleanup

Two conditional intervals maximum: A at 500, B at 1,000 only after A passes.
Each holds all four host slots for at most **900 seconds including teardown**;
the runner work budget is 840, with no builds inside either interval. Queue/drain
time precedes acquisition and is reported separately from UTC acquisition/release
and actual held duration. Preserve reservation order. No automatic second window
after failure or timeout, and no 2,000-node attempt even if both pass: that needs
a separate decision after the observed evidence and reader-residual disposition.

Entry: memguard running, exclusive locks acquired, host pressure normal and
known, `kern.memorystatus_level >= 30`, T3 resident use below 52 GiB, and fresh
native pool status with no prior retained/queued/running jobs, its unchanged
12 GiB invocation budget, and known headroom at least that budget. Do not raise
pool/budget or bypass a named admission refusal. Product admissions keep their
existing fresh-sample/normal-pressure/resampling rules; there is no external
retry of a refused solve.

P1 must monitor host/pool status and owned process samples. Stop on pressure
warning/critical/unknown, host available level below 30, unknown attribution,
typed resource refusal, process crash/replacement, a hung required interaction,
or an aligned App+worker+WebContent footprint sum above 12 GiB. That sum is a
run-only protective stopping criterion, not a new product budget or a proof of
16 GiB fit. Any owned PID RSS above 16 GiB is an emergency stop. The host's
22%-available memguard remains enabled as the last backstop.

On stop, try `analyze.cancel` for an active solve; allow at most 5 seconds for
its cancellation/reap receipt. A completed native reader may not be cancellable;
do not wait indefinitely. Terminate the runner/App/helper group, and identify
product workers separately: `solve_worker.rs` deliberately gives them their own
process groups. Freeze the current App PID before enumerating PIDs whose executable
path is this window's unique private App copy; stop those worker groups/PIDs
before removing that copy. Never infer worker termination from App exit alone.
Leave the runner's parent-death guard runnable during that freeze.
The deadline must begin teardown by 840 seconds and finish/release
the exclusive wrapper by 900, even if store inspection hangs. Do not kill a
global process name or an ambiguously attributed WebKit process. Stop the owned
sampler/caffeinate, close the App, and check no owned worker remains. Unexpected
remaining processes or ambiguous WebKit teardown are reported and prohibit B.

After all writers exit, retain the compact JSON/progress/PID evidence, verify
SQLite `quick_check`, recorded IDs, stored lengths and integrity findings against
the reported outcomes, and check CSV receipt/file cleanup. A killed writer is
checked as an interrupted transaction, not assumed to have committed. Remove
only this run's private HOME/store/downloads and temporary App copy after those
checks. On failure keep only the named private artifacts needed to diagnose it;
report their path/size and remove them when that consumer finishes. Keep the
bound App/lane cache while this proposed window still needs them. No broad host
cleanup utility or other lane's files. Report cleanup and lock-release failures
truthfully; do not extend the interval to hide them.

## Missing executable prerequisites and smallest grant

P1 — **Round-1 repairs implemented offline; recheck pending.** The first independent
review at `3faf9ea12` blocked supervisor-death cleanup and forced-stop evidence
loss (F1/F2); both new in-model kinds were reported to Agent 0. The parent repaired
them and the smaller failure-handling notes without an implementation delegate.
This is no risk acceptance or native-window grant. `perf.mjs --run` now
forks the runner into its own process group under an independent supervising
event loop. The App/helpers inherit that group; synchronous SQLite inspection
can block only the runner. The supervisor begins stopping at the work deadline
(maximum 840 s), escalates after 5 s, bounds a missing exit receipt by a further
5 s, bounds worker discovery by 1.5 s, checks disappearance for up to 2 s and
bounds its asynchronous report/cleanup by 10 s. These limits leave teardown
margin within 900 s; actual held duration and cleanup are still run evidence.
The child receives a smaller run budget (810 s for the proposed 840 s window),
reserving 30 s for post-run work before the supervisor's deadline. That reserve
does not guarantee a multi-GB SQLite check completes; an unfinished check remains
partial evidence, not a successful save verification.

The child arms IPC-disconnect self-group termination and a same-module guard
thread before starting the App. The thread checks reparenting every 100 ms even
when the runner's main thread is blocked in SQLite. SIGHUP uses the supervisor's
bounded stop path. Normal handover removes the disconnect handler and unreferences
the guard without keeping the runner alive; it still protects delayed helper/I/O
exit. Abnormal supervisor loss supplies no normal window receipt: recovery uses
the durable partial carriers and verifies process closure before another window.

The product worker's own process-group behavior is preserved. Before final
cleanup the supervisor freezes the current App PID, not its guard thread, and uses the sampler's
control-only `--list-owned` mode (no memory counters) to find PIDs/groups by the
unique private executable path. It kills only that closed set plus its own runner
group, verifies disappearance, and reports failed/unknown lookup as unverified
cleanup. WebKit candidates are never targeted by name or PID. Private scratch
is removed asynchronously only after successful outcome and process verification;
failures retain its named path. `--keep` retains stores and removes the private
App copy after verification. There is no automatic next run after a failure.

The existing sampler additionally supplies host available-percent and
`kern.memorystatus_vm_pressure_level` readings (at most once per second: 1 normal,
2 warning, 4 critical, other values unknown). Elevated pressure, stale samples,
low headroom, an aligned sum above
12 GiB, PID RSS above 16 GiB, and ambiguous/replacement WebContent stop the window.
One transient unknown PID reading is retained but tolerated; two consecutive
unknowns for that PID stop the run. A vanished PID is removed from the pending
set. Unknown host status needs two distinct 1 Hz readings, not two cached 200 ms
samples; known elevated pressure or a known exceeded cap still stops immediately.
These are run controls, not changes to product pool/budget admission. Native pool
pressure/preflight and UI cancellation probes still belong to P2. A supervisor
forced stop does not claim a successful UI cancellation or transaction commit.

Raw sampler JSONL is exclusively created outside disposable HOME/scratch and
flushed before the line is sent to its consumer. Each completed run is atomically
recorded before the next App starts; its ok HOME is then removed unless `--keep`.
Thus prior stores do not accumulate for a multi-run window. Samples/records survive
normal scratch removal. The supervisor keeps the latest/triggering compact sample
and completed-record paths, labels forced-stop evidence incomplete, and records
inspection-start/completion separately. Failed/unknown inspection is not a commit
receipt. Transient group-probe EPERM is polled as still live within the existing
2 s limit; persistent uncertainty fails verification.

Twenty-five maintained tests pass under `t3_slot.sh`, including only tiny fake
Node processes for blocked inspection, workers in separate groups, supervisor
death with a stopped fake App, normal guard handover, SIGHUP, repeated interruption
and failed/stalled lookup. Added regressions check preserved records/raw samples,
cleanup containment and transient host/PID readings. The production sampler writer
was exercised only as an isolated pure AST function with synthetic data; its module,
ctypes, sampler modes and sysctls were not executed. No native App, worker solve, sampler mode or
memory measurement was launched. The own-executable discovery is source-reviewed
preparation until separately authorized live evidence exists. Scope for the
required safeguard review is the existing honest-local-mistake threat model:
finite teardown, closed owned-PID/path and scratch containment, honest failure
reporting and no unrelated-process kill. Hostile concurrent path/PID substitution
remains outside the existing kit's threat model; this is not risk acceptance.

P2 — **The maintained driver needs the C1 interaction probes.** Its current
steps stop at first paint/Record/Save. Prepare/test the bounded DOM/navigation,
CSV, native reopen/re-save/history and lease-counter probes above. It must report
the actual outcomes and timely refusal; the old classifier's `ok` alone does not
establish this larger usability gate. No new product commands are needed.

P3 — **Reader/parse attribution is not executable yet.** The phase-separated
large-input probe described above is absent; no reader working-peak value exists
on this candidate. Prepare and bind that narrow probe using the maintained
allocator facility, and give its exact command/input recipe before requesting
execution. Its allocation/timing run also belongs inside an authorized exclusive
interval, never disguised as an ordinary concurrent test. If it cannot fit with
the App sequence inside A/B, return a bounded additional-interval proposal;
do not silently add one or extend either interval. These prerequisites are
offline implementation/verification work, not a request for broader permissions.

Once P1–P3 are verified, exact-head CI is clear and identities are reconciled,
the smallest owner/resource grant is: launch this named native-smoke App on only
the two invented racks, automate the specified local UI/save/export/reopen
interactions in throwaway stores, read the owned process footprint/PID/pool and
DOM/timeline/SQLite/CSV evidence, and acquire A then conditional B, each <=15 min,
under `t3_exclusive.sh` with announced acquisition/release. The existing local
test data/storage boundaries apply. **No screenshot, video or screen recording
is needed:** the gate is response/DOM/data/memory behavior, not visual-layout
acceptance. Request no embedded LLM, provider authentication, credential/keychain,
network model, supplier input, native release/signing or general capture grant.

Both generated models have measured static container depth **7**. The committed
v0.3 builder corpus's records have maximum depth **14**, well below the parser's
127-container boundary. These are static JSON checks, not a native reopening
measurement. The selected racks add array entries, not a new nesting form; they
do not exercise the forged/deep stored-record edge. Actual generated record/header
depth and native reopen success still belong in the run evidence. Do not turn
this source/corpus assessment into acceptance or an assertion that every stored
record can reopen.

The maintained `rackModelOpen.test.ts` now also passes both exact-size generated
documents through File > Open Model Document's `checkNativeOutcome` shape/Wasm
validation: **6 tests pass**. The generator subprocess needed an explicit 8 MiB
stdout buffer for the 1.8/3.6 MB documents; the initial run hit Node's default
`ENOBUFS`, corrected in that test transport only. No product limit or expected
result changed. This is source-mode validation; it launches neither the App nor
a solve and establishes no first-paint, usable-size, depth-safe actual reopen or
memory-fit result. Both plan subsets validate offline; the deliverables check
reports 0 errors. No additional reviewer or broad suite replay was started for
this preparation.
