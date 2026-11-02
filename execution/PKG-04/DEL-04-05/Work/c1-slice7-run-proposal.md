# C1 slice 7 run proposal

Serves RT-U4 / criterion 9 and the C1 plan's slice 7. This is an offline-prepared
proposal, not a window grant, measurement, acceptance or memory-fit result.

**Execution status:** both owner-authorized 500-node attempts have failed and
closed. The initial attempt at `1e1453a29` stopped on uninitialized host telemetry;
the repaired attempt at clean `eb77d0067bf9f09e81916cccb4ec487ba196ccfb`
passed startup telemetry and timed out waiting for first paint during native
reader validation. Both approvals are consumed. **No retry, B1000 or further
native interval is authorized.** This document retains their selected identities,
original recipes and partial-evidence limits; it is not a runnable grant.

## Repaired A500 result and closure (2026-11-02)

The owner explicitly approved one repaired attempt on clean `eb77d0067`, using
the unchanged bound `app-p3` executable and maintained 500-node input, with fresh
`evidence/n0500-refresh1`. Exact source, script/plan/model/Wasm/dist/binary hashes
were reverified. The original failed packet's hashes are unchanged. Python was
recorded as `/Applications/Xcode.app/Contents/Developer/usr/bin/python3`, 3.9.6;
its reported monotonic implementation is `mach_absolute_time()`.

The normal wrapper recorded WAIT at **01:58:28 UTC**, acquisition at
**01:58:29 UTC**, and `END-EXCLUSIVE rc=1` at **02:03:32 UTC**. The elapsed value
from those one-second timestamps is **303 s**; the supervisor window was
**303,399 ms**. This is below the 900 s interval cap, with no extension or build
inside it. The separate startup host attempt was fresh (sent 01:58:29.210,
sampled 01:58:29.246, received 01:58:29.249 UTC), available 74%, pressure normal.
Every streaming host sample remained known-normal, with available level 69–74%.
Thus the initial-refresh correction worked in this attempt; the gate still failed.

The App shell became ready, opened the selected model, started sparse solve and
received the worker's published result. The solve timeline records `job_done`
**73,378 ms** after `solve_requested`, **918,638,964 result bytes**, worker
lifetime-maximum footprint **3,485,617,728 B** and peak grant **6,375,342,080 B**.
Native `reader_verdict_start` was the next mark. No `reader_verdict_end` or first
paint arrived before the configured **300,000 ms** first-paint timeout (observed
wait 300,651 ms). The classifier is `hang` for that deadline; this does not prove
the reader could never finish. No threshold, input, result or standing was changed.

The table/navigation/filter/CSV, Record/Save, native reopen/re-save, History and
normal result-retirement sequence was **not reached**. The scalar allocation
probe was also not reached: there is no direct reader-working or parse-allocation
peak and no health-working upper bound for this run. The UI cancellation attempt
reported requested=true and success_claimed=false. That is not a completed
cancellation/reap receipt. The App then exited with code 0 via the harness's
failure report, and the runner exited 1; actual process closure was checked
separately. The last native pool report still held one result, no worker, and
8,446,497,008 reserved bytes. Normal lease retirement was not demonstrated before
App exit, and must not be inferred from later process disappearance.

Partial physical observations on this Apple M5 Max/18-CPU, 128 GiB host,
Darwin 25.6.0:

| Observation | Bytes | Interpretation |
| --- | ---: | --- |
| App sampled footprint maximum | 8,377,883,872 | Direct per-PID sampled value; not reader-only or parse-only |
| App kernel lifetime maximum | 8,617,729,272 | Direct kernel high-water observation; includes all App phases |
| Worker kernel lifetime maximum | 3,485,617,728 | Also reported by the completed worker |
| WebContent sampled footprint maximum | 824,968,320 | One new baseline-attributed WebContent; parentage remains inferred |
| Aligned sampled App + worker + WebContent maximum | 8,852,152,792 | Sequential PID readings at one sample, not an atomic concurrent peak; worker absence is inferred zero after `job_done` |

The aligned sum has **21 unknown samples** (including incomplete process
attribution/readings); this count is not unknown host telemetry. GPU/networking
are retained separately in the raw packet, not added as independently timed
maxima. These observations do not establish the full gate, memory fit, a material
reader share, 500-node usable size, 1,000/2,000-node capability, or the 16 GiB base
machine criterion. SQLite `quick_check` returned `ok`, with zero project, run and
state rows; no persistence step was attempted.

Closure: the supervisor reaped runner group 22336 and verified all private-exe
PIDs gone, with no group errors. Independent control-only lookup returned `[]`;
`ps` found none of the owned wrapper/runner/App/worker PIDs or group, and no
A-owned lock/`exclusive.pending` remained. A later shared slot's unrelated
`lockf` holders are explicitly distinguished from this closed interval; they
were not signalled or changed. **No held exception.** Acquisition and actual
closure/failure were reported to Agent0 promptly. No B or second run was queued.

Preserve the new packet/private store/App copy for the owner and failure diagnosis
consumer, alongside the unchanged first packet:

- `scratch/rt-perf-c1-slice7/evidence/n0500-refresh1/perf-run-perf-vvmNnv-native-linear-ex-n0500.json`
- `scratch/rt-perf-c1-slice7/evidence/n0500-refresh1/perf-window-1793584709038.json`
- The same directory's raw sampler JSONL, aggregate report and `perf-vvmNnv/`.
- Adjacent `a500-refresh1-preflight.json`, `a500-refresh1-admission.json`, console,
  wrapper, summary and closure files. The closure records raw packet hashes and
  verification time; it does not rewrite raw evidence.

No new implementation, measurement or third-run proposal is selected by this
return. The observed outstanding issue is native reader validation failing to
reach first paint within the approved gate; source diagnosis is distinct from
another solve/profile or a timeout extension. #59 stays draft and unmerged.

## Offline diagnosis of the repaired first-paint timeout

**Supported conclusion:** the packet locates the timeout inside the awaited
native-registration sequence, but cannot identify its active subcall or split
CPU, parse, clone and drop costs. Its `trace.counts` is empty. No native result
text, native phase checkpoints, stacks or P3 metrics survived in this packet.
The last pool reservation total and physical samples cannot supply that missing
attribution. They remain the observed values above, not parse/reader metrics.

The exact path at selected `eb77d0067` is:

1. `previewService.ts:642` marks `reader_verdict_start`.
2. `readNativeVerdict(job,{})` sends the native verdict command. Despite its
   local variable name `header`, this calls `reader_dispatch_json`, not a
   route-only reader. The native path admits args, prescans held bytes without
   materializing them, acquires the parse lease, then deserializes the entire
   result via `serde_json::from_str<Value>`.
3. `validateNativeReaders` sends a separate route-specific validation request
   with the captured model or invocation. It performs another held-result
   prescan, parse and validation.
4. `readNativeStanding` dispatches again with the current binding, with another
   held-result parse. Its exact-input cache differs from the header request.
5. `readNativeSummary` prescans once, calls the health text API (which parses
   the held source), then separately parses it again to build summary/planes.
   The first health source tree is dropped before the summary source is parsed.
6. Only after these awaits and envelope refusal handling does the session mark
   `reader_verdict_end`. An envelope-registration refusal may repeat standing
   and summary without binding. Later handle construction requests standing
   again, but the exact-input cache reuses the successful standing request.

Four bounded regressions in the maintained `nativeResult.connect.test.ts`
freeze header, registration, standing and summary separately on the existing
small pressure replay, using committed Rust responses. Every pending case gives
exactly the same final `reader_verdict_start` mark as A500. The normal completed
sequence issues three verdict requests plus one summary; there is no fourth
verdict command at handle construction. **All 11 connection tests pass.** These
are mocked/replay control-flow checks, not measurements or reproduction of the
919 MB native workload.

For an admitted ordinary pressure result, `reader_verdict::dispatch` calls
`standing`. That computes its token using `numerical_use_standing_with_context`
-> `for_source` -> full pressure evidence validation **before** the empty-basis
return, and then the pressure branch validates evidence again to obtain findings.
Thus the successful pressure sequence above invokes full pressure evidence checks
**2 + 1 + 2 = 5 times**, independently of the five sequential result parses
(three verdicts plus health plus summary). This is a source-path count, not an
assertion that A reached all five. The selected model has exact combinations;
the packet does not carry its returned route, so the pressure-route work count
is stated conditionally rather than presented as observed call progress.

Inside a pressure evidence pass with combinations,
`exact_combination_evidence::direct_view` first does `source.clone()`, then removes
`exact_combinations` and combination rows. This definitely allocates a second
full `Value` tree before discarding those members; it is not just another copy
of the raw UTF-8 buffer. The primitive check then builds borrowed ID/evidence
indexes, and the combination check rederives expected rows/evidence and retains
expected combination rows across records. It scans rows per operand/case;
its support-magnitude reconstruction performs a linear row search for each
support/component. Model binding also has member/section searches. These are
concrete allocation/repeated-work and potential scaling sites, **not a proved
CPU bottleneck for this packet**. Value destruction and allocator-retained
blocks can contribute to later CPU/physical readings as well.

The existing `exact_combination_contract` corpus-admission test passes under the
normal slot wrapper on its five maintained small cases (735–1,828 rows), with
and without models. No expected rows, numeric comparisons or authority changed.
No core source was modified, and no allocation/timing benchmark, native App,
worker solve, large-result parse or new measurement layer was run.

**Smallest missing observation:** a bounded entry/exit checkpoint for the
existing header/registration/standing/summary awaits, and within the pending
native command, prescan/admission, parse, validate and source-drop boundaries.
A checkpoint must be durable before the long phase starts; an end-only reply
cannot identify a timeout. This is a proposal for narrowly extending existing
smoke/timeline evidence, not a request to run the App. Replay tests can validate
its ordering without the large input. No CPU/allocation value should be invented
from the phase marker.

**Smallest supported product repair plan, conditional on the identified phase:**

- For the redundant initial dispatch, expose a route/binding-only reading using
  the existing downgrade/identity/receipt-shape guards. It must have a separate
  return type with no standing/eligibility; registration and model-bound standing
  still run their full checks before the session grants Current. Merely deleting
  validation from the existing dispatch API would change its contract and is
  not proposed.
- For combination projection, clone only the retained primitive members/rows
  rather than cloning the full source and then deleting combination material.
  Pin projected data equality and existing first-refusal/numeric outcomes on the
  maintained corpus before considering a borrowed-view refactor or indexes.
  Do not remove evidence checks, change their order or tune any tolerance.
- A broader single-parse registration operation could remove later full parses,
  but it is not the first patch: it changes lease lifetimes and the connection
  contract. Reusing parsed trees globally without a custody-bound lease/cache
  decision is not justified by this packet.

Ownership: native verdict/summary, session connection and smoke boundaries are
C1 surfaces. The combination projection and shared reader/standing APIs are
`core/reporting/result_export` (DEL-05-02/DEL-04-07 numerical reading) and the
operation-applier bridge; coordinate exact edits through Agent0 with RT-A3 and
other consumers before shared writes. Any product change needs independent
numerical/permission scrutiny and byte/refusal parity as applicable. No new
handler, window, input, memory budget/reserve, timeout enlargement, authenticity
or numerical-eligibility contract is selected by this diagnostic return.

## A500 execution and closure (not performance evidence)

All approved source/script/plan/model/Wasm/dist/binary identities passed the
19:34:13 UTC offline preflight, and the output directory was fresh. The normal
exclusive wrapper recorded acquisition on 2026-11-01 at **19:34:58 UTC** and
`END-EXCLUSIVE rc=1` at **19:34:59 UTC** (one-second timestamp precision).
The supervisor's internal window was 19:34:58.740–19:34:59.287 UTC, **547 ms**;
that is not a subsecond measurement of the full lock-hold duration.

The App process was launched (PID 7456). The supervisor stopped its runner with
`host_headroom_unknown` after two early samples. Both had null available/pressure
values and a null host refresh timestamp. No UI-ready, model-open, solve,
reader-profile or completed-run evidence exists. The receipt is explicitly
`evidence_complete:false`; these execution facts establish no usable size,
component memory peak, host memory shortage or memory fit.

The wrapper exited and the supervisor verified the runner group and private
executable PIDs gone, with no group errors. Control-only lookup, `ps` and lock
inspection at 19:36:24.968 UTC found no owned processes, no lock holders and no
`exclusive.pending`. Later shared-slot reuse by other jobs is not an A-held
resource. There is no held exception. App Agent0 was notified by Agent0, and no B
was queued. The failed packet/private copy is preserved byte-for-byte for the
owner and initial-refresh diagnosis/review consumer:

- `scratch/rt-perf-c1-slice7/evidence/n0500/perf-window-1793561698740.json`
- `scratch/rt-perf-c1-slice7/evidence/n0500/perf-samples-perf-SaUjMu-native-linear-ex-n0500.jsonl`
- `scratch/rt-perf-c1-slice7/evidence/n0500/perf-SaUjMu/`
- Adjacent preflight, wrapper and closure records; `a500-closure.json` records
  the preserved packet hashes and distinguishes A's closure from later lock use.

## Confirmed initial-refresh cause and bounded offline correction

The sampler initialized `host_at=0.0` and queried only when
`monotonic()-host_at >= 1`. A process-relative monotonic clock near zero therefore
emitted the untouched initial host object until one second elapsed. A failed
sysctl attempt would have assigned a timestamp, so the observed null timestamps
identify the unattempted-refresh path. The deterministic old-source regression
starts at **0.015 s** and reproduces a first query at **1.015 s**, after the
stream guard could stop on the initial unknowns. No native clock or sampler was
re-run to obtain that diagnosis.

The minimal initialization repair uses `None` for "no attempt yet" and reads
before the first emitted sample, independent of clock origin. One shared bounded
host-reading function retains actual values or unknowns; failed/stalled reads
never manufacture normal values. The streaming stop policy is unchanged: two
genuinely fresh unknown attempts stop; repeated cached sample ticks do not count
as new attempts. The pure AST tests execute only extracted functions with fake
process/time/sysctl/signal services; no module import, libproc, real sampler mode
or host probe runs. They cover the first valid sample near zero, cached ticks,
two fresh failed readings and bounded stalled readings.

The original entry contract requires fresh known-normal host state before App
launch. The old runner started the App before sampling and did not enforce that
ordering. A separate `--host-status` control in the existing sampler now performs
one fresh attempt (0.5 s sysctl timeout, no PID memory counters). The runner awaits
it immediately before App launch, with a 1.5 s interpreter timeout/SIGKILL and
4 KiB output cap, all under the existing independent supervisor. It admits only
normal pressure, at least 30% available and a timestamp from that request (1 ms
rounding tolerance); failed, stalled, malformed, unknown or stale telemetry
refuses before App launch. There is no entry retry. Streaming still tolerates
one genuine unknown refresh after admission. Native pool preflight, caps,
reserves and the 840/810 s budgets are unchanged.

That newly enforced startup ordering is a safeguard change, so it requires one
focused independent review under the existing honest-local-mistake threat model.
It does not reopen the three closed P1 teardown rounds or change their controls.
The fake-child timeout test and all **41 Node tests** pass offline; no new App,
worker, solve, sampler mode or exclusive interval has run. Exact-head CI and
focused scrutiny remain required before returning this repair candidate.

## Candidate and prepared identities

The runtime basis is `a37d1ef1f535155e8ea3f0e330bb7cd7a056bb71`, built clean.
Its only changes from independently cleared
`55fd8f4fccb334e91fe2929e4f93ba0aa3165b1b` are two Work documents. Runtime,
tests, schemas, core and permissions are identical. This proposal and
`apps/desktop/native-smoke/perf-plan-c1.json` and the 500/1,000-node extensions of
`rackModelOpen.test.ts` are later preparation changes, not part of that binary.
Their submitting revisions and external hashes are recorded below. Any later
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
changes `perf.mjs`, `perf-sampler.py` and their tests. P2 also changes the
external driver and plan. Neither changes the bound App/worker/frontend. Bind its submitting revision and re-hash those
external scripts before execution. The old identities do not certify the new
supervision path, and green CI at 867e51b does not cover that later code.

## Later instrumented executable (prepared offline)

The P2 probe source is commit `e989364ec`; P3 is
`284c71f48165fac5ec34cf0cf8ba5eb20b74113f`. The latter was built from a clean tree through the
same maintained `--build` recipe, with no launch. The source delta from the
reviewed runtime is explicit: native-smoke reader instrumentation, inactive
allocator hooks under that feature, external driver/plan/reporting and tests,
and the narrowly parsed TLS scan form. It changes no core, pricing, schema,
ordinary product handler or frontend source. The current frontend/dist and
both Wasm identities equal the original a37d binding above. Later Work-only
binding reconciliation does not change this runtime/test identity.

- Binding: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-p3/perf-binding.json`.
- App/worker: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-p3/openpipestress-desktop`,
  50,985,168 bytes, SHA-256 `288811f283e2e979fa17a452d478f378a5d85d717662fb4751d9dfbb94c48dcf`.
- Worker is the same new executable with `--swbpipe-solve-worker`; inactive
  counter checks add instrumentation overhead even outside P3. Neither App
  binary has been launched.

Current external source identities (SHA-256; recorded in each run and
inspection checkpoint as well as the final report, with current source commit
and dirty flag, separately from the native App binding):

| File | Digest |
| --- | --- |
| `perf.mjs` | `42ee95e79221cd46f7a24deb314fc8ffa9304c322f27c7dcb4e7e8a46ec41a13` |
| `perf-driver.js` | `92e06d6b38c25341d86381fd51fcfebccaa0ba832e39a1f5fc71df7e459dfa84` |
| `perf-sampler.py` | `846f5493cc9cc309140cced0b6ff40ec9b92468db10a4f63e85fe654f9e03d3b` |
| `perf-plan-c1.json` | `663a0affddcdd40323e11f3f4a89e1a10e961ab34e7e1f301d89825f5fbec2cc` |

The original a37d App remains preserved in `app/`; execution recipes below
select `app-p3/`. P1 was independently clear at dee93a72 and its control block
is byte-identical. P2/P3 connected measurement/permissions scrutiny, including the TLS scan
delta, cleared `92983690e`; the external-script follow-ups below change no
App/worker/frontend/Wasm bytes. No successful allocation probe or native
interaction is claimed by compilation or these hashes.

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
were executed; the original App build was bound at clean a37d. The `app-p3` build recipe
selects the later instrumentation candidate and preserves that original copy:

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
RT_C1_TARGET=/Users/ryan/dev/chirality-t3/targets/rt-perf-c1
RT_C1_APP="$RT_C1_KIT/app-p3"
mkdir -p "$RT_C1_KIT/models" "$RT_C1_APP"
/Users/ryan/dev/chirality-t3/tools/t3_slot.sh python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 500 --output "$RT_C1_KIT/models/linear-ex-n0500.json"
/Users/ryan/dev/chirality-t3/tools/t3_slot.sh python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 1000 --output "$RT_C1_KIT/models/linear-ex-n1000.json"
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh node apps/desktop/native-smoke/perf.mjs --build --app-dir "$RT_C1_APP" --target-dir "$RT_C1_TARGET"
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-1000
```

The P3 command requires the later `app-p3` binding; the preserved a37d
App cannot execute it. Its submitting revision and artifact digest are recorded above. The two `--plan` commands
have passed. The new plan's default selects only 500; it has no 2,000-node run.
Do not use the old `perf-plan.json` default window, which includes 2,000.

## Repaired one-run recipe (approved, consumed and closed)

After focused review and exact-head CI, the owner approved **one 500-node
attempt only** using this repaired harness and the unchanged bound `app-p3` executable.
It executed and failed as recorded above. The following recipe is historical,
not permission to execute it again. Use the
existing variables above; do not build or nest a slot wrapper inside the window.
Reverify the selected clean source and every current external/binary/model hash.
The output below now contains the failed packet and must be preserved; the
original `n0500` packet is also immutable.

```sh
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_APP" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500 --budget-s 840 --out "$RT_C1_KIT/evidence/n0500-refresh1"
```

That one-run grant is consumed. There is no automatic retry or new window,
no B1000 even if a later separate attempt passes, and no 2,000-node scope. The
same no-capture/LLM/auth/network/release restrictions remain. Any further native
work requires a separately concrete proposal and fresh authorization.

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
  process or unreadable PID can be missed; absence is unknown, not zero. `c1_physical` reports the aligned sampled peak for the
  complete interval, before/during/after P3, with unknown-sample counts. An
  absent worker contributes zero only after the product `job_done` mark, and
  that zero is explicitly labelled inferred. Per-PID lifetime maxima in a
  phase are still lifetime observations, not phase-only maxima. Match
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
  The P3 command in `src-tauri/src/native_smoke.rs` is
  `native_smoke_profile_readers`: raw args `{jobId}`, with the existing ingress
  header. The driver invokes it once after fresh table/CSV probes and before
  persistence, within the same 810 s child budget and a 90 s profile wait.
  It has no standalone model/result arguments or export. It reads only this
  launch's plan model path, checks JSON.stringify bytes against the job's intake
  digest and checks held bytes against its publication digest. Model/invocation
  wrapper and solver mode match the production registration. Source, model and
  invocation parse allocations are counted separately. Dispatch and the route's
  actual registration reader run on held parsed Values; all rows pass through
  borrowed semantics batches of at most 1,000, reporting maxima rather than the
  sum of batch peaks. Summary includes its response/admission work, with the
  health-view map measured separately.

  Numerical health's Value API is private. Its text entry reports a directly
  counted combined parse/work peak and an **inferred upper bound** for working
  requested/envelope bytes: combined peak minus the separately measured live
  source tree. It does not subtract prescan M or physical memory, and it is not
  labelled an isolated health working measurement. It includes findings parsing
  and serialization, and can overstate health work because parser transients
  also contribute. No core API is changed to make the bound smaller.

  The command returns only bounded scalar metrics, phase epochs, lengths and
  digests (64 KiB cap); no standing, result, model, record, rows or capture.
  It rechecks custody after all phases and never registers or upgrades a handle.
  Pool leases for raw args, file/texts, each parse, normalization scratch and
  summary response precede their allocations under the existing capped pool.
  Reader working/health text allocations remain the existing unpriced residuals
  being observed, not newly claimed bounded allocations. All work is off the
  executor. Allocation accounting disarms on unwind and rejects nesting.

  The later App/worker includes inactive per-thread counter checks in allocation
  calls even outside the P3 phase. Record that instrumentation overhead; it is
  not byte-identical to the reviewed a37d runtime. Driver/native epoch markers
  separate P3 from ordinary pre-probe App/worker/WebContent samples. Later
  persistence/reopen samples can retain allocator caches created by P3; they
  cannot be described as the uninstrumented product's physical footprint.
  These component allocations do not replace the live P2 observations. If the large input or probe is absent,
  this part of the gate is incomplete rather than inferred from small fixtures.

Keep held result bytes, parse M/reservations, counted working heap, response
buffers, App physical peak and WebContent physical peak as separate columns.
SQLite cache/journal/FTS, allocator cache, thread stacks and Tauri delivery remain
physical-process residuals unless directly attributed. Report absolute working
peak and its context; do not adopt a new materiality threshold. A material reader
share needs its bound/lease decision before leaving draft, per the C1 plan.

## Resource limits, cancellation and cleanup

The original A/B approval is consumed/blocked as recorded above. The new
one-run proposal is 500 only and holds all four host slots for at most **900 seconds including teardown**;
the runner work budget is 840, with no builds inside either interval. Queue/drain
time precedes acquisition and is reported separately from UTC acquisition/release
and actual held duration. Preserve reservation order. No automatic second window
after failure or timeout, and no 2,000-node attempt: that needs
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

## Verification status and smallest grant

P1 — **Independent round 3 clear at `dee93a72`.** The first independent
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
The round-2 recheck cleared F1–F6 at `f0bf1c5bd` but found that self-stop hid a
runner error's cause (F7). The child now writes that cause synchronously to stderr
before self-group termination, with an optional bounded IPC stop reason; the new
fake-process regression checks the cause survives. This is a failure-reporting
repair within the existing threat model, with no additional native authority.
The final reviewer independently ran all 26 tests and cleared F7. Its mutation
also showed the small fixture can pass with asynchronous console output, so the
tests do not establish that synchronous writing is necessary. A full non-blocking
stderr pipe can still make this diagnostic best-effort; it is not an unconditional
durable cause in the window JSON. The optional IPC reason and incomplete/failed
receipt semantics are unchanged. No fourth P1 review is opened for these notes.

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

Twenty-six maintained tests pass under `t3_slot.sh`, including only tiny fake
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

P2 — **Implemented offline; connected review clear at `92983690e`.**
The C1 plan now enables bounded probes for all three navigator axes, next/previous
pages, row detail, no-match/clear filtering, streamed group CSV, Record Run,
native project reopen/re-save, recorded-run check/disposal, normal model-open
retirement and pool counts returning to their starting state. The trace rejects
whole-value/legacy persistence and budget-changing commands. It retains only
counts, bounded native headers, small receipts and metadata; no row page, full
result, record, model or CSV chunk is retained by the observer.

The input plan binds the two existing maintained rack hashes and node counts;
changed bytes, wrong node count or a size escalation is refused before App launch.
The solve proof must match the model JCS hash. A separate native-table-ready time
waits for settled rows and two animation frames after Results activation; the
older solve `first_paint` mark alone does not show that table. These are DOM/frame
observations, not pixel verification. Required interactions record latencies and
published IDs/positions, without a new accepted usability threshold.

Native persistence is observed through its actual DTOs (`run`/`message` for Record
Run, small result outcome for saves). Reopen and re-save must preserve producer
identity, handle/origin, envelope evidence and the complete findings list. A
model-only SQLite save or named refusal fails the round-trip usability probe;
it is not called a half-written result. CSV inspection streams with Python's
standard CSV reader outside the webview, checks BOM/preamble/width/receipt bytes
and compares data-row count with the native group's query total. Inspection
failure is unverified evidence, classified `refused_with_message` at step
`csv_inspection` with `needs_review`, not assumed file corruption. Python CSV
inspection uses isolated mode (`-I`).

The preflight refuses prior held jobs, changed budget or unknown/insufficient
headroom. Live monitoring uses native pool status, stops immediately on known
elevated pressure, after two unknown replies, or after an outstanding status
request exceeds 3 s on two consecutive timer ticks. Staleness starts at request
dispatch; a delayed webview tick before a request is not low-memory evidence. It attempts
bounded cancellation on failure without inventing a successful receipt. Terminal
UI outcomes remain verbatim; the separate low-level refusal trace is bounded
and marks truncation/omission. Transient admission retries remain observable,
not reclassified as terminal product failures.

The current thirty-eight Node tests pass under the slot wrapper. Added tests use DOM/IPC
emulation and the committed Rust response shapes, plus synthetic CSV files and
pool readings; they are source-mode evidence only. They do not establish actual
native interaction, cleanup, first table paint, memory fit or the complete P3
gate. The P1 control block is byte-identical to its reviewed `dee93a72` source;
P2 changes input validation, observation, UI probes and read-only file inspection.
No new product command or scope is needed for P2.

P3 — **Implemented and bound offline; connected review clear at `92983690e`.**
The existing native-smoke feature now supplies the read-only scalar command
above and the C1 plan enables it. The existing allocator facility is also
compiled under that feature, remaining inactive except on the probe thread.
Default product builds do not compile these hooks or the new command. A standard
TLS macro is now visible to the user-release scan: its one qualified invocation
is admitted only in ingress, its body must be parsed statics and every initializer
is scanned with the existing rules. No opaque macro exemption or handler-macro
re-pin is added; the ordinary product handler definition is unchanged.

The independent review at `92983690e65ba3292a3d3ea87183fb383b92272c`
found no blockers or new in-model failure kind. It independently checked hashes,
binding/leases, production DTOs, the health upper bound and TLS scan. Its own
checks: 36 Node tests; 69 targeted Rust tests across ingress, native-smoke and
reach scans (including the real-source scans); non-test smoke `cargo check`.
Inherited author checks remain the default product check, TypeScript/release
build, model-version 8, timeline/plan validation and graph 0 errors/one existing
cycle. No full local Rust/vitest run is claimed after P2/P3; exact-head hosted
verification remains required. No successful probe, allocation profile or native
App invocation has run.

Parent follow-ups to the nonblocking notes change only external scripts/tests
and Work text. The pool monitor tolerates delayed ticks and requires two stale
pending-request ticks. Before potentially blocking store/CSV inspection, the
runner atomically checkpoints the exited App's report, memory timeline, source
identities and inspection-pending state outside disposable scratch. Its class
is `refused_with_message` and `evidence_complete` is false; it is never announced
as a completed run. A committed final record supersedes/removes it. A deadline
while inspecting still fails the gate; the checkpoint supplies partial evidence,
not a commit receipt or permission to exceed the window. CSV inspection uses
isolated Python. All 38 Node tests pass, including delayed-page/pending-request
and unfinished-inspection regressions. P1's control block remains byte-identical;
no additional reviewer is requested for these contained note repairs.

Remaining interpretation limits, recorded rather than expanded:

- No successful profiling command has run. The maintained generator writes
  sorted keys; native Open reads those through serde, and the session adopts
  the opened document unchanged. Existing open/solve source-mode and JS-text
  writer tests support that input basis, but JCS equality alone would not
  establish exact intake key order. The first authorized 500-node run is the
  connecting-path canary. An intake-order refusal ends that gate without retry.
- `semantics_working_max` has maximum requested/priced/live byte values across
  all bounded batches; `elapsed_us` and allocation count are **sums**. Its rows
  are borrowed, so this is the per-row reader/output work compared with the
  existing 4 MiB working allowance, not total production command memory.
- `model_parse_bound_bytes` names the raw model file's admission bound. The
  measured model parse reads its JSON.stringify-normalized text. The raw bound
  covers that same model tree; it is not labelled an exact normalized-text size.
- The inferred health bound depends on the selected engine keeping its parsed
  source alive through health construction/serialization. That order was
  independently read at the bound source; no test pins its lifetime. A change
  there invalidates this inference until rechecked. Parser transients remain
  separately observed, not inferred from a partly built tree.
- The smoke-only non-test build warns about unused `counting::live_priced`;
  this does not affect the probe. The 30 s inspection reserve does not promise
  that large store/CSV checks finish; incomplete inspection fails the gate.

The live allocation/timing run belongs inside the authorized interval. If it
cannot fit with the App sequence inside A/B, return a bounded additional-interval
proposal; do not silently add one or extend either interval.

The original narrow launch/data grant was given and A consumed it; it supplies
no retry, B, capture or model-provider authority. After the focused repair review
and exact-head CI, the smallest new request is the one 500-node interval above
with the same local UI/data/read-only evidence scope. **No screenshot, video or
screen recording is needed.** There is no embedded LLM, provider authentication,
credential/keychain, network model, supplier, signing or release requirement.

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
