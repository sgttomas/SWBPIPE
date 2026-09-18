# D-72 — performance acceptance criteria for the redesigned SWB Piping Designer interface

Status: **PROPOSAL / AWAITING_RULING**, 2026-09-18. Prepared by HELP_HUMAN Agent 0 (successor ROOT) of the run [`HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`](../AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/). No criterion here takes effect, and none may be used for acceptance, until the owner rules. Its precedent and form are the [D-69](D-69_ui_foundation_performance_disposition.md) and [D-71](D-71_interface_program_governed_text_decisions.md) packets: decision and recommendation, evidence and consequence, why this is an owner decision, on-ruling mechanism, bound evidence. This is the proposal owed under [D-70](D-70_RULING_2026-09-17.md) effect 6: criteria "proposed with a score-independent rationale, frozen prospectively, applied identically to baseline and candidate, and ruled by the owner before use, consistent with V79's guidance".

Disclosure: the author has read the recorded results of the original D-70 cohort and of the successor demonstration ([reconciliation record](../AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/PIPING_HANDOFF_RECONCILIATION_2026-09-18.md)). Each rationale below is therefore written so that it can be checked without them: it rests on what the engineer does at the screen, on the geometry the design fixes, and on what the instrument can and cannot observe. Where a number cannot be justified that way yet, the packet says so and sets no number.

## Decision and recommendation

Six items, ruled separately. The owner may accept, amend, choose an alternative or defer each.

### Item 1 — keep the five ruled numeric limits and the settled-frame and resource obligations; add no CPU gate

**Recommendation.** The D-68 measures carry to the redesigned product with their numeric limits unchanged: model assignment to presented canvas and responsive tables ≤ 2,000 ms; point selection to presented feedback p95 ≤ 100 ms; box selection or filtering to presented feedback p95 ≤ 200 ms; orbit reported-presentation interval p95 ≤ 16.7 ms in centreline display and ≤ 33.3 ms in Actual OD display; a settled viewport with no scheduled animation frames; a stable owned-resource baseline after 20 model replacements and repeated unmount and remount.

**Rationale, independent of any score.** These are human-perception bounds, not properties of this product: about 100 ms is the limit under which a direct manipulation feels caused by the hand; about 200 ms is the limit for a compound response to a command before the engineer looks away; 60 and 30 presented frames per second are the ordinary bounds for smooth and for acceptable continuous motion; two seconds is the limit for a load before attention breaks. The redesign changes what is drawn, not who is looking. Nothing in the design argues for a looser bound, and a tighter one would claim a need the owner has not stated.

**Four quantities stay separate and only two are gates.** (a) Interaction latency, from a real pointer or keyboard stimulus to Chromium-reported presented feedback: gate. (b) Chromium-reported presentation intervals during orbit: gate. (c) Main-frame work from trace spans: recorded every run as a regression observation, with known-duration and ambiguous spans counted separately; no threshold, because short main-thread work does not establish timely visible feedback. (d) Available GPU-thread trace evidence: recorded as same-window context; no threshold, because these spans are not hardware GPU execution time and are not joined per frame. Presentation feedback is not physical scanout, and no observer overhead is subtracted from any quantity.

### Item 2 — declare the reference profile with the limits, and qualify on both of the owner's displays

**Recommendation.** Reference host Apple M5 Max, 128 GiB. Pinned Chromium, the version bound per run. Production build. Device pixel ratio 2 as the gate, with one run per size at device pixel ratio 1 recorded as an observation (the design's one-pixel edge line and fixed-size glyphs are specified for both). Light appearance and Comfortable density as the gate; one Dark run per size as an observation, since the themes are peers and differ only in token values. Display: the five-run cohort is collected on the host's internal display in its 120 Hz mode, and the same frozen method is run once per size on the external display in its 60 Hz mode as a declared observation whose result is published beside the cohort whatever it is.

**Rationale.** A limit and the display it is read on are one statement (D-70, second correction). The internal display is the one display every unit of the reference host has; an external monitor is whatever the office supplies. That makes the internal display the reproducible reference, a reason that holds whatever any run scored. The internal profile entered the record because the external display was unavailable before any 10,000-pipe launch ([owner direction](../AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/OWNER_DIRECTION_INTERNAL120_20260917.md)), not to obtain a result. The 60 Hz observation is kept because V79 cautions against treating 60 Hz as unsuitable, and because a 16.7 ms limit read on a 16.67 ms refresh has no margin: Chromium's future-display estimate alone can move an interval across it. Publishing that run, rather than dropping the display, keeps the limitation visible instead of deciding it silently.

**Instrument consequence.** The instrument's boundary validator today admits device pixel ratio 2 at 1440 × 920 only (RESEARCH-G §3.2). The 1440 × 900 window, the two canvas configurations of item 3 and the device pixel ratio 1 observation each need a second frozen boundary profile, bound before the cohort and applied to baseline and candidate alike.

**Alternatives.** B: gate on both displays. C: gate on 60 Hz only, as D-68 first ran. Under B or C the owner should also rule whether a p95 interval equal to the refresh period is the intended concept at 60 Hz, or a missed-refresh ratio is; this packet does not propose that change.

### Item 3 — freeze the redesigned product's geometry and label populations

**Recommendation.** Window 1440 × 900 (the design's reference size; D-68 used 1440 × 920 for the old shell). Two canvas configurations, both measured: Both view, canvas 603 × 828 CSS pixels beside the 737-wide table with the agent strip; Model view, canvas 1000 × 828 with the drawer closed. Drawing-buffer dimensions, device pixel ratio and rendered label count are recorded at every segment boundary, as D-70 effect 1 requires. Label populations, from the design system's budget of one label per 3,600 square pixels of canvas: Off (0 rendered) for selection and filter segments, as today; Budget for assignment and orbit segments, which caps at 138 labels in Both view and 230 in Model view. The rendered count at each boundary is reported beside the cap; a count far under the cap is stated, not hidden.

**Rationale.** The sizes are the design's, fixed in design system V1.1 §0 before any measurement of them exists. Budget is the design's default label mode (mock-review decision 13), so measuring orbit with the old cap of 80 would measure a product nobody will use. The redesigned canvas draws 499,284 or 828,000 CSS pixels against the current shell's 442,258, so fill cost differs; that is declared here rather than normalised away.

### Item 4 — workloads: a shared core applied to baseline and candidate, and redesigned-only workloads with perception limits

**Recommendation.**

*Core (baseline and candidate, identical method).* The frozen 1,000- and 10,000-pipe fixtures, oracles, sampled actions and scoring of the D-70 instrument: one assignment, 200 point selections, 20 box selections, 20 filters, two orbit windows of two seconds' warm-up and ten seconds' measurement. Real pointer and keyboard stimuli; named setup and query actions; typed entity identities. The candidate runs it in both canvas configurations of item 3. The baseline is the product at the PR #794 merge (`362dcffc0f66d52c58689a58f268891461db0346`), run fresh under the same frozen method, profile and cohort size in its own shell; the one recorded demonstration is a comparison basis and is not reused as a cohort member. Where the redesign replaces the tree filter with a table filter, the segment is named by what it does ("filter to presented rows") and the substitution is declared in the method before freezing.

*Redesigned-only (candidate; no baseline counterpart exists).* Limits are the item 1 perception bounds applied to the design's own interactions: row selection to canvas highlight, and canvas pick to row selection, p95 ≤ 100 ms (rendering brief §3: "within one frame" is the design intent; 100 ms is the gate); view switch (Table, Model, Both) and stage switch with camera, selection, label mode and legend intact, p95 ≤ 200 ms to presented; split drag and drawer resize, reported-presentation interval p95 ≤ 33.3 ms while dragging; orbit with result colour, edge lines and labels at Budget, the item 1 orbit limits.

**Rationale.** "Applied identically to baseline and candidate" can only mean the workloads both products have. For interactions that exist only in the redesign there is no baseline to compare, so their limits come from the same perception bounds and not from a measurement of either product.

### Item 5 — cohort and pass rule

**Recommendation.** Five fresh production-build runs per size per canvas configuration, on a frozen candidate, with no substantial build, test or other runtime activity on the host during timed collection. Every run must be valid and complete under the instrument's own validity rules; every gated quantity must meet its limit in every run. Failed and invalid runs are retained and reported; no replacement sample, no retry beyond a budget frozen with the method, no tolerance or oracle change to obtain a result, no rounding of a limit. The picking repair's maintained regression tests pass on the candidate as a precondition.

**Rationale.** This is D-68's cohort and D-70's evidence discipline unchanged. "Every run" rather than a pooled percentile is the reading that does not let one good run hide a bad one.

### Item 6 — what is not frozen now, and why

**Recommendation.** No numeric criterion is proposed yet for: deformation animation (frame cadence over ten seconds, cost of changing the scale factor); two hundred ghosted proposal elements and time to accept; selection halo on five hundred elements; theme switch with results shown; time from run completion to first coloured paint; per-node interpolated result colour; memory (JavaScript heap, process RSS, memory delta under overlays) and cold start. For the first five the behaviours are proposed, not implemented, or only partly implemented ([RESEARCH-G classification](../AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/RESEARCH/G_rendering_workload_classification.md)); freezing a number now would either be arbitrary or wait on measuring something that does not exist. Memory and cold start have no probe in the current instrument and were reported unavailable by both piping reports. These are **observations first**: the bounded observation run on supported behaviour ([observation brief](../AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/OBSERVATION_BRIEF_2026-09-18.md)) and, later, observation of the implemented redesign inform a supplementary proposal, which the owner rules before it is used. Until then they are reported and do not gate.

**This is not circular.** Implementation of those features does not wait for their measurement; items 1 to 5 can be frozen and ruled without them; the supplementary criteria are frozen after the features exist and before the qualification cohort runs.

## Evidence and practical consequence

The basis is the two published piping records, kept apart: the original D-70 cohort (incomplete at 10,000 pipes; historical verdict unchanged) and the one successor demonstration on the repaired product (complete, valid, original limits met; not a five-run qualification and not acceptance of the redesign). Neither is evidence about the redesigned product. If items 1 to 5 are ruled as proposed, an implementation tranche has a fixed target before it starts, the piping session's instrument needs only named adaptations (canvas configurations, Budget label mode, row-to-canvas stimuli), and the baseline cohort can be collected at any quiet time. The cost is two canvas configurations at two sizes, twenty candidate runs and ten baseline runs, plus the declared observations.

Not covered here and still open: the independent-usability holds PDU-045 and PDU-046; WCAG 2.2 AA criteria for touched controls (D-68, unchanged); the redesign's functional acceptance, which belongs to the implementation-ready handoff. No minimum-hardware claim, solver-capacity claim, release or lifecycle promotion follows from any of this.

## Why this is an owner decision

D-70 effect 6 reserves the ruling to the owner before use. The active workplan's Step 2 and `DEC-087` reserve new normative and acceptance criteria to the owner. V79 requires any change of metric or profile to be owner-authorized, reasoned independently of score, frozen prospectively and applied fairly. Agent 0 can recommend and prepare; it cannot adopt.

## On-ruling mechanism

The owner rules item by item, in chat or on this packet. Agent 0 appends the ruling verbatim with its SHA-256 as this packet's Human Ruling section, writes `D-72_RULING_<date>.md` on the D-70 ruling record's form, and updates the register row. The ruled criteria are then carried into the implementation-ready handoff as its performance verification requirement and notified to the piping session through the owner. Freezing happens at the ruling: a later change is a new owner act with its own rationale, made before the cohort it applies to. Silence leaves no redesigned-product criteria in force, and the D-68 obligation open and unqualified.

## Bound evidence

- [D-70 ruling](D-70_RULING_2026-09-17.md); [D-68 ruling](D-68_RULING_2026-09-15.md) and the D-68 plan §4 (`../AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/APPROVED_PLAN.md`, SHA-256 `077371d84e0067315c4661cac6f0e199f3501ca1efce801fb3d31a0d37f8ca73`).
- V79 guidance: `../AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/GPU_ORBIT_BOUNDARY_DIAGNOSIS_V79/REPORT.md`, SHA-256 `ace3939d620489882e29e8fd10097135c3425e1d6b52aa2aa6e46fdd4e1561ca`.
- Original baseline: `../AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md`, SHA-256 `56aa2f08de5ed0b5935d507c22d34d6aa22cda22d24e6b7227cadb4f64caffb7`.
- Successor demonstration: `../AgentRuns/HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/RUNTIME_REPORT.md`, SHA-256 `e8acb49bce189f15ddf6958e16ffdc34f8ddc502de51df2065f0502c18dc4a40`; `../AgentRuns/HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/_run_records/RUNTIME_EVIDENCE.json`, `ac2892d5f3b9f06a4f47db5a39390cf1fe6a7bd3f9ebc2440a338a5ab43e9ba7`.
- Design geometry and label budget: design system V1.1 §0 and §6 (`34ba3f1fadacbd1c3854e7bb654098b5525e25b9734b4106dfdd39fc94f0cf1f`); rendering brief §1, §3, §4.

## Human Ruling, 2026-09-18 (appended; the packet above is unchanged)

Ruled in part by Ryan Tufts in session on 2026-09-18, in the same message that ruled D-71; the complete message is reproduced once, with custody and hash, in the [D-71 ruling record](D-71_RULING_2026-09-18.md) (SHA-256 of the extracted bytes `711ca05ef7cb9bcaf6b7f0e6812ad002be58d8f9793aa68a08d0f19bceaf7944`). Its D-72 lines, verbatim, spelling the owner's:

> Item 1: Recommendation accepted.
> Item 2: I want this app to work on people's laptops or when connected to an external monitor.  I don't understand all the technical implications here but I also don't want performative governance or testing with no real value.  I am willing to accept your recommendation after considering this.
> Item 3: Same again.   want this app to work on people's laptops or when connected to an external monitor.  I don't understand all the technical implications here but I also don't want performative governance or testing with no real value.  I am willing to accept your recommendation after considering this.
> Item 4: Recommendation accepted, we need not pin ourselves to previous test results or methods.  Those are just evidence to know what was the case before changes are made.
> Item 5: I'm not convinced this testing is necessary.  I don't know that for certain, it just seems like overkill.  I need more of a rationale to rule on this.
> Item 6: Recommendation accepted.

Disposition: items 1, 2, 3, 4 and 6 accepted as recommended; **item 5 not ruled**. The owner's stated intent governs how the accepted items are read: the product must work on a laptop's own display and on an external monitor, and testing must earn its keep. The [ruling record](D-72_RULING_2026-09-18.md) states the effect. The supplement below answers the owner's request for a rationale on item 5 and, taking the owner's intent at its word, proposes to make the whole plan lighter.

## Supplement, 2026-09-18 — item 5's rationale, and a lighter plan for the owner's ruling

**What the testing is for, in one sentence.** The redesign adds work to every frame (more labels, edge lines, result colour, halos, a table beside the canvas), and the only way to know that a 10,000-pipe model still feels immediate afterwards is to drive the real interface with real pointer input and time it; the instrument that does this already exists, runs unattended, and costs machine time, not the owner's.

**What five runs buy, and when they buy nothing.** Within one run, the p95 over 200 selections already covers variation between actions. Repeating the run covers variation between runs: thermal state, garbage collection, whatever else the machine was doing. That matters only when a result is near its limit. If selection feedback takes 45 ms against a 100 ms limit, a second run cannot change the conclusion and four more are ceremony. If it takes 95 ms, one run proves nothing and repeats are the whole point. The packet's item 5 treated both cases alike, and the fresh five-run baseline cohort on the old interface measures a product nobody will use again. The owner's objection is right about both.

**S-1, replacing item 5 — repeat only when it is close.** One run per fixture size (1,000 and 10,000 pipes) per canvas configuration on the laptop's display: four runs. A run whose every gated quantity is at or under 80 % of its limit settles that configuration. If any gated quantity lands between 80 % and 100 % of its limit, that configuration is run twice more and all three runs must meet every limit. Any quantity over its limit is a failure: the product is fixed and the changed product is tested as a new candidate. A run the instrument itself marks invalid is repeated and the invalid run is kept on record. The 80 % line is fixed now, before any run of the redesigned product exists; no tolerance, oracle or limit is changed to obtain a result. The picking regression tests pass first.

**S-2 — no fresh baseline cohort.** The recorded demonstration on the repaired product is the before-picture, with its recorded conditions. This follows the owner's word on item 4: earlier results are evidence of what was the case before the change, not something to pin to. It sets aside, by owner act, D-70 effect 6's phrase "applied identically to baseline and candidate" so far as it would require re-measuring the old interface; the criteria themselves still come from perception bounds and not from either product's scores.

**S-3, refining items 2 and 3 for "laptops and external monitors".** The external-monitor check becomes real rather than decorative: one run per size on an external monitor at that monitor's own resolution and pixel ratio, in the larger canvas configuration, with the latency and assignment limits applied as gates there too, since those limits do not depend on the display. The orbit interval on that monitor is reported, and a result over its limit opens an investigation into whether product work or the browser's display-time estimate caused it, because on a 60 Hz monitor the 16.7 ms limit equals the refresh period and leaves no margin for the estimate. The separate Dark-theme run and the separate device-pixel-ratio-1 run on the laptop display are dropped: the themes differ only in colour values, and the external monitor already exercises a second pixel ratio where one exists.

**Cost if S-1 to S-3 are ruled:** six unattended runs in the ordinary case (four on the laptop display, two on the external monitor), against thirty in the packet as first written, plus repeats only where a result is close.

**Alternatives.** Keep item 5 as written (five runs, every run passes). Or go further and accept a single run per configuration with no repeat rule, which is cheaper still but leaves a near-limit pass indistinguishable from luck.

The owner may rule S-1, S-2 and S-3 separately. Until then item 5 is unruled, and items 2 and 3 stand as accepted.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
