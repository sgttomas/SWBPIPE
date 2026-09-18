# Frozen independent D-70 expectations

Disposition: READY_FOR_IMPLEMENTATION; no policy conflict found. Read-only source assessment at fc21b00d0b4233349c7fa8df433d6a82e9f7678e, before instrument edits. Product remains 8468a33c86adb622b25e98f98b0eaf28c7e9fa0e. This derivative expectation packet consumes D-70 and the supplied approved execution contract; it is neither decomposition truth nor performance acceptance. TASK child of /root, native delegated execution, requested gpt-6-astra/low, no delegation. The request supplies model/reasoning attribution; there is no independent runtime model introspection in this assessment. Paths/non-delegation are instruction/config asserted.

Scope: PKG-07 / DEL-07-01, SOW-020 / OBJ-006; DEL-07-02 observation and DEL-07-06 SOW-036 Remaining propagation only. Applicable OPS-K-AUTH-1/2 and OPS-K-REPORT-1 require truthful, hash-bound evidence and limitations; no engineering, release, usability, minimum-hardware or full-resource claim. No tests, builds, browser, cohort or Git mutation performed.

## Immutable behavioral expectations

1. Qualification is the default. Explicit characterization permits a valid original-target miss to continue, preserving original FAIL_TARGETS and scores. Unknown/conflicting mode inputs fail closed. Characterization must not masquerade as focused/assignment-only diagnostic or historical baseline proxy mode. Separate evidence validity, collection completeness, target outcome and attempt disposition in both per-run and cohort records.
2. Hard abort: invalid/missing required causal evidence; source/build/browser/fixture/method/profile drift; incomplete trace/loss/overflow; invalid metadata; stale or ambiguous required identity; cleanup failure; settled owned rAF not exactly zero; existing action/trace/per-run timeout. A target-only miss is not an abort. No catch-all conversion of thrown errors into tolerated misses. Existing scorer remains byte-unchanged.
3. One plan: five fresh 1000-pipe sessions followed by five fresh 10000-pipe sessions, unique run/session IDs, exact order, 200 points, 20 boxes, 20 filters, both orbit modes per session, 2000 ms warmup + 10000 ms measured. No retries, replacements, sample filtering or second cohort. Pause may resume only unattempted runs with identical bindings; interrupted timed run remains incomplete and ends attempt.
4. Preserve points labels OFF and restore ON for orbit phases. This is explicit fixture-manifest.json point_hit_protocol policy, controller lines 599 and 732, and the approved contract's per-phase requirement. D-70 both-mode labels-on means both orbit modes; globally forcing labels on would alter the frozen point workload. Capture actual per-phase label enabled/rendered count rather than asserting intent. Do not silently change box/filter inherited state.
5. Use existing real controls and typed identities for setup/query. All measured interactions retain real pointer/keyboard input, frozen independent hit/box/tree oracles and camera/layout. Actual OD retains cold first-conversion precondition and conversion state, not an opportunistic warm conversion.
6. Reference profile is M5 Max / 128 GiB, pinned Chromium 153.0.8010.36 revision @507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c, viewport 1440x920, DPR cap 2 and existing external 60 Hz profile. Verify before collection; unavailable profile is a blocker. No profile substitution, CPU gate, new category/observer, timing subtraction, compositor investigation or source repair.
7. Snapshot metadata outside measured windows at initial presentation and scenario boundaries. Bind every sample group to snapshot identity/hash and run identity. Required values: CSS and drawing-buffer dimensions, browser and effective DPR, labels enabled/rendered count, display/geometry mode and conversion state, camera, theme/density/panels, and source/build/browser/fixture/method identities. Finite positive dimensions/DPR, safe nonnegative label counts and valid typed state are mandatory; explicit unavailable is acceptable only for report fields permitted unavailable, not required evidence. Boundary snapshots never prove continuous display/foreground/occlusion monitoring.
8. Offline main-frame statistics use all attributable events in frozen orbit windows, independently of the original selected presentation endpoints. Record window definition, PID/TID, event names, full population and missing/ambiguous counts. Preserve original score populations, ties, integer timestamps and source event occurrence identity. Main-frame work, available GPU-related spans, input latency and Chromium-reported presentation are separate; GPU spans are not hardware time and presentation is not scanout. Unavailable/ambiguous optional attribution is an explicit limitation, not fabricated zero or nearest-time join.
9. Preserve raw trace bytes once canonically plus hashes and references. Use lossless numeric handling for trace IDs/timestamps, never parse/re-serialize traces through ordinary JS Number JSON as canonical evidence. Report incomplete raw capture honestly. Existing 256 MiB cap, 30 s trace completion/IO deadlines and 20 min run timeout remain; no overall clock cap. Startup/loading/heap/process memory/resources are separately labeled, including unavailable and historical applicability.

## Concrete source traps and required tests

- full-cohort-controller.ts:829 currently throws for every non-PASS_METRIC_ACCEPTANCE; ui-foundation-performance.benchmark.ts afterAll also throws on any non-PASS_COHORT_METRICS. Both need mode-aware validity/completeness handling, while strict behavior stays unchanged. A serial Playwright failure skips later tests; only genuine hard failures should trigger it in characterization. Test a valid target-miss run followed by another run, not merely a helper predicate.
- performance-targets.ts:144–198 separates validityFailures from targetFailures; settled-zero is validity at 192–193. The cohort score itself is FAIL_COHORT for any target miss (231–232); preserve that score and independently derive collection disposition from ten valid, complete outcomes.
- performanceExpectation (controller 328–395) rechecks sample counts, oracle bytes, exact method inventory, bundle source/dist bytes and browser. It resolves method files relative to process.cwd() (project root), while product files resolve against candidateSourceRoot. Instrument checkout and product checkout must be intentionally distinct; never update product source inventory to current instrument HEAD to make hashes pass.
- candidate-server-response.ts: source stage preliminary is technically accepted alongside final. The actual attempt must use independently frozen final product source/build metadata, not rely on that permissive validator as proof of final readiness. Require build/source provenance back to product SHA, complete source/dist inventory, executable hash, response headers and before/after identity checks. A manifest's own aggregate does not prove omitted files were inventoried.
- benchmark configured pipe/run arrays can differ from its independently constructed ten-run candidatePlan. Explicit characterization must reject reordering, duplicate/subset/extra/non-finite settings before runtime; do not merely discover missing samples in afterAll.
- Metadata must distinguish reported browser DPR from effective canvas DPR/drawing buffer. uiDiagnostics.ts exposes canvas and viewport.labels (enabled/renderedCount/budget); PipeViewport.tsx sets renderedCount zero when disabled. readCandidateDiagnostics only proves attachment/freeze, not semantic completeness; add focused invalid-field/drift tests.
- Cleanup must still run after action failure and persist failures; do not let cleanup exceptions discard earlier canonical trace/stop records. Recheck bindings at exit even after misses. Capture failures before result.json exists require explicit run disposition in final attempt report.
- Offline tests should include duplicate timestamps, huge numeric IDs, missing duration versus zero duration, extra attributable main-thread events not joined to selected endpoints, wrong PID/TID, ambiguous attribution, incomplete trace, empty populations and stable original-score bytes. No actual browser needed for these tests.

## Exact method inventory and prerequisites

Existing requiredMethodFiles has 28 entries (below), all relative to apps/desktop/e2e/ui-foundation/. Freeze updated exact inventory including every new characterization/metadata/offline attribution helper and focused test/config directly or transitively used; update inventory enforcement and manifest together. Do not omit helpers merely because not imported by the browser driver. No unreviewed bytes may be added after freeze.

- `benchmark-harness.ts`
- `candidate-server-response.ts`
- `chromium-compositor-trace.ts`
- `lossless-trace-json.mjs`
- `lossless-trace-json.d.mts`
- `causal-presentation-extractor.mjs`
- `causal-presentation-extractor.d.mts`
- `causal-presentation-preflight.benchmark.ts`
- `verify-causal-presentation-extractor.mjs`
- `playwright.causal-presentation-preflight.config.ts`
- `playwright.candidate-causal-presentation-preflight.config.ts`
- `fixture-manifest.json`
- `causal-method-contract.ts`
- `causal-phase-journal.ts`
- `causal-method-contract.spec.ts`
- `playwright.causal-method-contract.config.ts`
- `tsconfig.causal-method.json`
- `performance-targets.ts`
- `performance-targets.spec.ts`
- `ui-foundation-performance.benchmark.ts`
- `playwright.candidate-performance.config.ts`
- `full-cohort-controller.ts`
- `full-cohort-controller.spec.ts`
- `playwright.performance.config.ts`
- `serve-bound-candidate-output.mjs`
- `freeze-candidate-point-oracle.mjs`
- `point-hit-oracle.mjs`
- `box-selection-oracle.mjs`

Before the runtime lease: accepted independent expectations; instrument readiness/test evidence; one untimed both-size/action-family smoke; frozen camera/oracles; fresh complete-diff PASS review; phase activation/approved contract/context bindings; exact final method manifest and SHA; final external product bundle manifest and SHA with source/dist inventories; unchanged fixture-manifest and all its files including samples, point oracles, control/observer bindings; pinned Chromium executable and hash; fresh explicit cohort ID and ten-run plan; clean isolated checkout and source/build provenance; external 60 Hz/host profile verification record; exclusive runtime lease. Record literal file paths, hashes, sizes where required, source revision, actual invocation/config/environment, agent parentage/settings/scopes, and before/after state. Reusing V79's old source/build/method/browser or two trace populations cannot qualify this attempt.

Required environment bindings from candidate-server-response and performanceExpectation: UI_FOUNDATION_MANIFEST_SHA256, UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST, UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256, UI_FOUNDATION_CANDIDATE_SOURCE_ROOT, UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT, UI_FOUNDATION_CANDIDATE_SOURCE_STAGE=final, UI_FOUNDATION_METHOD_MANIFEST_PATH, UI_FOUNDATION_METHOD_MANIFEST_SHA256, PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH, PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256, evidence root and cohort ID. Explicit characterization choice must appear in durable invocation and outputs.

## Handoff

Implementation may begin against these frozen expectations. ROOT still owns runtime authorization, validation/fan-in and governed closeout. No runtime readiness is claimed. Closure of this child is read-only expectations complete; baseline closure is completed characterization or exact incomplete-attempt report, never implied performance pass. Remaining work: implementation, smoke/review, single exclusive attempt, offline analysis/report and final evidence review. Original targets and D-68 obligation remain visible for redesigned product; D-70 resolves policy without reopening it. Standard F-PIP-2 / DEC-081 claims fence applies.
