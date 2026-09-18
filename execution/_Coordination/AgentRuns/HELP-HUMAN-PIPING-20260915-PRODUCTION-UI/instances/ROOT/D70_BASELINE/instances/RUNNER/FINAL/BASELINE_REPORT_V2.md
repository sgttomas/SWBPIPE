# D70 final baseline handoff

The owner-narrowed objective was not achieved: N1000.2 is valid and complete with all original timing targets met, but no N10000 attempt completed validly. The five-attempt ceiling is exhausted. This report supplies the performance handoff evidence; it does not grant product performance acceptance, redesign acceptance, independent usability, or resource/native qualification.

| Slot | Actual disposition | Preserved limit |
|---|---|---|
|1000.1|Invalid/incomplete|Original boundary drift; original analysis retained|
|1000.2|Valid/complete, original targets PASS|One run, external LG60|
|1000.3|Invalid/incomplete|Point3 boundary failure|
|1000.4|Owner-interrupted|Partial evidence; no completed run result|
|1000.5|Waived/unattempted|No synthetic claim|
|10000.1|Invalid/incomplete|Point58 typed capture binding failure|
|10000.2|Invalid/incomplete|Point58 typed capture binding failure|
|10000.3|Invalid/incomplete|Point3 camera drift; owner reports manual interaction|
|10000.4|Invalid/incomplete|Point58 typed capture binding failure|
|10000.5|Invalid/incomplete|Point58 typed capture binding failure|

Nine actual attempts: one valid complete, seven invalid incomplete, one interrupted. One waived slot; zero remaining eligible slots. ACCOUNTING.json binds actual claims, terminal records, last budget and owner waiver. No false success-close was created. The owner attribution for10000.3 is reported, not independently instrumented, and is not extended to the typed mismatch in other attempts. The established point58 mismatch is expected node:UIF-08786 versus observed pipe:UIF-08786 with visual cue passing; a separate accepted recorded-input calculation identifies a numerical tie-instability mechanism, summarized below.

N1000.2 original metrics in milliseconds: assignment261.6960021; pointp9546.6990014; boxp9547.0730015; filterp9543.6950017; centerline orbitp9516.668; ActualOD orbitp9516.668. Targets remain2000/100/200/200/16.7/33.3 respectively. Its prescribed200points,20boxes,20filters and both orbit windows were completed, with settled owned RAF count0. Failed10000 runs retain assignment values642.417–654.143ms, but no full point/box/filter/orbit populations; missing whole populations are unavailable, never passes. Their settled status is FAIL with null count, not measured nonzero.

The valid1000.2 run used method6ea1ed09 and externalLG60. All10000 attempts used method1e0ab8a4 and owner-authorized internal120. Product8468a33c and build800ec179 are frozen. These are not same-display scaling comparisons. Fresh final helper checks verified clean instrumentdf1562e, all34 method members, source/build inventory, browser, fixture, oracle, exact internal120 display, memory and no remaining pinned browser/server.

Recorded boundary metadata (994 continuation snapshots) gives canvas794×557CSS, buffer1588×1114, browserDPR2 and effective cap2, within1440×920 browser viewport. Labels have budget80: OFF/count0 or ON/count1,2,3 at recorded boundaries. BOUNDARY_OBSERVATIONS.json retains each run/phase reference, labels, canvas and resource observations. These are boundary-only facts, not continuous proof of display/foreground/occlusion state or total labels across the model.

All eight accepted continuation offline extractor commands exited0 sequentially. Only1000.2 provides two orbit observations; others explicitly return UNAVAILABLE_NOT_ZERO. Interrupted1000.4 has a selected-slot disposition but no completed result; extractor success does not manufacture one. Original1000.1 offline analysis remains referenced unchanged.

For1000.2 the inclusive action+2s through+12s window includes every attributable ProxyMain::BeginMainFrame occurrence, not feedback-selected frames. RendererPID78726/TID61136922: each orbit has1800 occurrences,600 known durations,0 missing and1200 ambiguous/invalid phase occurrences. Known-duration p50/p95/max are1429/2168/2865µs (centerline) and1456/2225/2815µs (ActualOD); statistics apply to the600 known durations, not all1800. ORBIT_SUMMARY.json binds the full extractor output and raw references.

GPU-related context is independently identified atPID78720/TID61136826, not causally paired to frames. Each mode has1200 occurrences;828/826 known and372/374 ambiguous respectively. SwapBuffers has600 durations per mode: p50/p95/max370/522/728µs and368/539/630µs. EndAccessImages has228/226 known durations out of600:1/1/4µs and1/1/2µs, with372/374 unavailable. These are Chromium trace spans, not hardware GPU time. Chromium-reported presentation remains a separate original metric (orbitp9516.668ms for both); presentation is not scanout. No pooling, retiming, overhead subtraction or nearest-time cross-process joins were performed.

Startup, heap and RSS/process-memory measures are unavailable. Loading retains the original assignment metric plus separately recorded host preparation, not a native startup measure. Resource snapshots and cleanup are boundary diagnostics; earlier native/full-resource evidence retains its original attribution and supplies no new broad qualification. Observer raw references do not prove uninstrumented true cost/headroom.

CANONICAL_EVIDENCE_MANIFEST.json references the original accepted manifest and enumerates continuation raw/partial evidence, controls, claims, terminals, receipts and offline derivatives:9347files,1404615112bytes. Raw canonical bytes remain in place, with no duplicate raw copies. FINAL_BINDINGS.json and OFFLINE_COMMANDS.json record final proofs and actual commands. All original verdicts and historical files remain unchanged. ROOT owns independent review, registered sweep, acceptance and Git integration; redesign and independent-usability holds remain open.

## Accepted supplemental cause calculation

ROOT-verified POINT58_CAUSE/DIAGNOSIS.md reproduces the pinned product versus independent oracle on32 recorded-input cases:28 disagree, including all8 delivered-coordinate cases. A normalized-miss roundoff gap of about1.739e-12 exceeds the1e-12 tie cutoff before node type priority; broadphase includes the node. Float32 pointer equality is confirmed but is not the sole cause. The proposed follow-on is a separately authorized stable equivalent sphere/capsule endpoint distance formulation; no repair or timed reclassification was applied. Camera reconstruction uses recorded authored camera fields because exact runtime quaternion/matrix bits were not stored, so this does not establish bit-for-bit historical internals. Attempt3 owner interference remains separate. Exact source report and return hashes are bound in RETURN_V2.json.
