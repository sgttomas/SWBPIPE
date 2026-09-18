# D70 baseline characterization — incomplete attempt

The sole authorized cohort ended with an invalid-evidence abort in the first 1,000-pipe run. One run was attempted, none completed validly, and nine were unattempted. No retry, replacement, second cohort, source repair or new diagnostic occurred. This derivative report completes the D70 incomplete-attempt handoff; it does not establish performance acceptance.

## Attempt and original outcomes

| Planned run | Disposition |
|---|---|
| 1,000 / 1 | Attempted; FAIL_INVALID_EVIDENCE; abort remaining |
| 1,000 / 2 | Unattempted |
| 1,000 / 3 | Unattempted |
| 1,000 / 4 | Unattempted |
| 1,000 / 5 | Unattempted |
| 10,000 / 1 | Unattempted |
| 10,000 / 2 | Unattempted |
| 10,000 / 3 | Unattempted |
| 10,000 / 4 | Unattempted |
| 10,000 / 5 | Unattempted |

Playwright exited1 with1failed/9didnotrun. Cohort scorer remains `FAIL_COHORT`; collection is `INVALID_OR_UNAVAILABLE / INCOMPLETE_ATTEMPT / ABORTED`, with `timedAttemptEnded=true`. The first point segment failed at stop-persist and work with `boundary profile/model/binding drift`. The failed stopped boundary snapshot was not accepted. This report does not identify an exact drifting field or infer a physical monitor change. External disk/profile/browser bindings passed before and after; live reference-profile records also remain preserved.

| Original metric | Retained value | Original target |
|---|---:|---:|
| Assignment |271.3380018234255ms upper bound (270.9499978065489ms lower)|2000ms|
| Point p95 |unavailable|100ms|
| Box p95 |unavailable|200ms|
| Filter p95 |unavailable|200ms|
| Centerline orbit p95 |unavailable|16.7ms|
| Actual OD orbit p95 |unavailable|33.3ms|

Assignment alone is marked PASS_QUALIFIED_CAUSAL_EVIDENCE by the preserved evidence; its below-target value does not rescue the invalid run. The original scorer has no targetFailures because required populations were invalid/incomplete, not because every target passed. Points/boxes/filters contain no scored samples; no orbit population was reached. Full validity-failure strings are preserved in RETURN.json and canonical result.json.

## Binding and metadata

Product `/Users/ryan/.codex/worktrees/8728/chirality-rendering-product-8468a33c/projects/chirality-piping` remains clean at8468a33c86adb622b25e98f98b0eaf28c7e9fa0e. Instrument revision36e56f2d93f763296b09520fe9f7c8481dd6fd54 is separate. CommandsV4 and launcher hashes remain04bd20156fefb44163d4d9143a50b2c966d9a1eca73fa7e734c4aa268d986d08 and29c9817d417b7a128a8cc86a27886cb11f146b017e264dc1290f2df3959b1e19. Exact environment paths/hashes are in RETURN.json and COHORT_V1_LAUNCH.json.

| Binding | SHA256 |
|---|---|
| UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256 | `800ec17958e47de299e39e3da1942dbbef0093eeed3c5c4311b5e9bcfa66567f` |
| UI_FOUNDATION_METHOD_MANIFEST_SHA256 | `c36048c44d14acdceb7b15a898c96e38a473e96adb985a07a9694496324426b1` |
| UI_FOUNDATION_MANIFEST_SHA256 | `6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739` |
| PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256 | `bfe18f25f912e28e567164f823efbcbf0a87c5292bd66566b6cce7861086d789` |
| UI_FOUNDATION_REFERENCE_PROFILE_SHA256 | `117aac9e5792d3274cbe5e5d710612bce374fb091dc32d28d4dc2190326b5922` |
| UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256 | `0b8a0b5239f10547c72907e32160db262288216ed1fb4818551782dc89b05b17` |

Host: AppleM5Max/128GiB, macOS26.6.2; pinned Chrome153.0.8010.36; external LG ULTRAFINE60Hz. Window1440x920, browser/effective DPR2; saved initial and point-ready canvas794x557CSS and1588x1114buffer pixels. Initial labelsON/rendered1; point-ready labelsOFF/rendered0. Saved boundaries include camera, schematic geometry/conversion state, light theme, comfortable density and panel dimensions. No orbit-mode metadata exists because orbit was not reached. Boundary snapshots do not prove continuous foreground/display/occlusion state.

## Cleanup, raw custody and unavailable populations

Restoration reports PASS_FULL_RESTORE, zero untouched pending native handles, removed pointer/input listeners, no restoration errors and zero overflow; prior run errors remain recorded. This is distinct from settled-frame validity: the final settled test is FAIL with ownedRafCount=null, so settled-zero was not established. After the process, no pinned browser remained and port5176 was clear.

Assignment and first-point raw traces were retained once canonically; trace-finalization/completeness records and every file hash are in CANONICAL_EVIDENCE_MANIFEST.json and RETURN.json. Raw bytes were not rewritten. The offline processor ran only after timedAttemptEnded and process cleanup, exited0, and returned no orbit observations plus20 UNAVAILABLE_NOT_ZERO records across the planned two modes × ten runs. Thus there is no valid orbit window/PID/TID population to summarize for main-frame work, GPU-related spans, input latency or presentation. Event presence in partial assignment/point traces is not a substitute orbit population; no further attribution diagnostic was performed. GPU spans are not hardware time and Chromium presentation is not scanout.

Startup, heap and process memory were not measured. Loading retains the original assignment metric and separately labeled host preparation; resource data is boundary diagnostics only. Earlier native/resource evidence retains its historical source/browser/applicability and supplies no new full resource qualification here.

Canonical cohort root: `/Users/ryan/.codex/task-runtime-cache/chirality-d70-baseline-20260917/instances/VERIFY/cohort/D70-8468a33c-20260917-CHARACTERIZATION-01`. Manifest records46files/4908678bytes. Prior failed untimed preparations and successful camera/oracle/smoke03 remain separately preserved and contribute zero cohort samples.

## Handoff

Report and canonical manifest are complete for the one incomplete attempt. ROOT owns evidence review, acceptance, Remaining propagation and Git closeout. D68 original target and resource/settled obligations remain visible for the redesigned product. No additional runtime or repair follows from this report. Standard F-PIP-2/DEC-081 claim fence applies.
