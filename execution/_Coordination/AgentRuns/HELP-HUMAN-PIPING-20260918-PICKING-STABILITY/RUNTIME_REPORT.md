# Landed picking repair — fresh 10,000-pipe demonstration

**Complete, valid, original timing targets met.** PR [#794](https://github.com/sgttomas/chirality/pull/794) merged as `362dcffc0f66d52c58689a58f268891461db0346` after both actual-head CI checks passed. One separately bound owner-requested N10000 run on that product completed on 2026-09-18; all 200 point selections, including the formerly failing point 58, matched the frozen expectations. No retry or replacement occurred.

This is a derivative runtime report and successor handoff to the accepted implementation in `HANDOFF_STATE.md`. It leaves the original D-70 cohort, historical failures and original local handoff intact. This successful single demonstration does not establish five-run qualification, redesigned-product acceptance, release readiness or professional reliance. Standard F-PIP-2/DEC-081 fence applies.

## Scores and collection

| Original metric | Measured (ms) | Unchanged target (ms) |
|---|---:|---:|
| Model assignment to usable presentation | 640.786 | 2000 |
| Point-selection p95 | 42.036 | 100 |
| Box-selection p95 | 44.296 | 200 |
| Tree-filter p95 | 43.092 | 200 |
| Centerline orbit reported-presentation interval p95 | 8.335 | 16.7 |
| Actual OD orbit reported-presentation interval p95 | 8.335 | 33.3 |

Both-size untimed smoke passed first. Timed collection retained all 243 segments: one assignment, 200 point selections, 20 directional box selections, 20 filters and two orbit windows, each with the frozen two-second warm-up and ten-second measurement. Each orbit has 1201 scored presentation gaps. The final disposition is `COMPLETE_SINGLE_DEMONSTRATION`, `VALID`, `PASS_METRIC_ACCEPTANCE`; process exit 0. No validity or target failure was reported. Settled diagnostics observed zero owned pending animation frames. Post-run browser/server cleanup, display checks, clean product/instrument revisions and source/build bindings passed.

## Environment and comparison limits

Product and instrument are distinct clean checkouts at the merge above; the source aggregate, production manifest, 38-file method manifest, immutable fixture/oracle, pinned Chromium 153.0.8010.36 and profile hashes are separately bound in `_run_records/RUNTIME_EVIDENCE.json`. The original oracle, sampled actions and scoring targets were unchanged. The new launch route expressly identifies a fresh demonstration instead of reusing exhausted D-70 slots.

Reference host: Apple M5 Max, 128 GiB, internal Color LCD 120 Hz, no mirroring, approved by the owner. Browser viewport 1440×920; browser and effective rendering DPR 2. Canvas 794×557 CSS pixels and 1588×1114 drawing-buffer pixels at all 243 stopped boundaries. Light appearance, Comfortable density, visible 280 px tree and 340 px inspector rails. Assignment stopped with labels enabled and one rendered label; points, boxes and filters had labels disabled with zero rendered. Both orbits had labels enabled, two rendered at ready and three at stopped; Actual OD conversion was available in its orbit, centerline conversion not requested. Camera and panel details are in the referenced boundary records. These are boundary observations, not continuous monitoring.

The old successful N1000 baseline used the external LG 60 Hz profile and older product/method, so this report makes no same-profile scaling or speedup comparison. One N10000 run demonstrates this workload on this instrumented host, not minimum hardware requirements, solver capacity or uninstrumented headroom.

## Work and presentation observations

Offline extraction uses all attributable `ProxyMain::BeginMainFrame` occurrences in the frozen orbit windows on renderer PID 72574/thread 61869944. Centerline has 3601 occurrences: 1200 known duration spans, 2401 unknown-phase/ambiguous-duration entries; Actual OD has 3600: 1200 known spans, 2400 unknown-phase entries. Known-span p95 is 2160 µs and 2004 µs respectively; no missing/ambiguous span is counted as zero. These trace spans are not complete frame cost and introduce no CPU acceptance threshold.

GPU-related events are explicitly unbound same-window GPU-thread context (PID 72568/thread 61869837), not causal per-frame joins or hardware execution time. Chromium presentation feedback is not physical scanout. No observer-overhead subtraction was applied. The unchanged scorer's instrumented original-target result remains separate from these observations.

Native startup, JavaScript heap and process RSS were not measured. Loading retains the original assignment metric and preparation records. Renderer resource counts are boundary diagnostics only; this run does not repeat or supersede the separately attributed resource-lifecycle or native qualification. No further compositor investigation or UI repair is selected here.

## Custody and successor handoff

`_run_records/RUNTIME_EVIDENCE.json` points to the sole raw copy and the complete raw manifest, terminal, independent runner return, profile, method, build and offline observations. Prior source review and DEC-025/local checks retain their original candidate attribution; PR #794 supplies actual-head CI/merge evidence. This metadata-only follow-up receives complete independent review and affected local/CI checks; it does not repeat unchanged product runtime suites.

Root accepts the landed bounded repair and complete valid demonstration. Remaining closeout is evidence publication under standing authority, recorded by its owning PR. The immediate picking-repair/runtime assignment is complete; no additional benchmark attempt is required. Under D-70, redesign implementation need not wait on this session. The design program retains its own implementation readiness/approval, prospective performance criteria, independent-usability holds, and later rendering-brief-driven overlay/deformation observation. Pressure runtime, connector mechanics and sparse work remain separate. Formal DAG, dependencies, decomposition, stage, lifecycle and release status do not change.
