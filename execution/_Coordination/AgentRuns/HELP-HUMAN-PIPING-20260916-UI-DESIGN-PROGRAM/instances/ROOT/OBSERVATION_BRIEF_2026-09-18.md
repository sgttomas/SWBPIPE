# Observation brief — what can be observed on the product as it exists, and what cannot yet

Status: brief from the design program (successor ROOT) to the piping session, prepared 2026-09-18 for delivery through the owner. It bounds the separate overlay and deformation observation run that [D-70](../../../../_DECISIONS/D-70_RULING_2026-09-17.md) effect 4 provides for, using the classification in [RESEARCH-G](../RESEARCH/G_rendering_workload_classification.md) (read at `451c5f595e0488a0d6064d9b0f972c5e8fc1d09d`, 50 rows: 10 observable now, 8 needing a bounded harness adaptation, 32 needing implementation first). Its precedent and form are the [rendering brief](RENDERING_BRIEF_2026-09-18.md): the ask, the basis, the boundary. It is a request, not an instruction; the piping session owns its instrument, its schedule and whether to run this at all. **It blocks nothing**: not the completed baseline handoff, not redesign progress, not the D-72 ruling.

## 1. Why the rendering brief's §4 table cannot be run as written

Six of its eight observations name a subject the product does not draw: result colour of any kind, an edge-line pass on model geometry, proposal ghosts, a deformation scale factor or animation, and fixed-screen-size glyphs. Measuring the nearest existing behaviour would characterise an absence. Those six are withdrawn from this run and listed in §4 as future observations. The rendering brief stands as the statement of what the redesign needs; this brief replaces only its §4 as the ask for a run now.

## 2. Observations requested now

Same host, pinned browser, production build, frozen fixtures at 1,000 and 10,000 pipes and boundary recording as the successor demonstration. One run per size is enough; this is characterization, with no target and no pass or fail. Real pointer and keyboard stimuli wherever responsiveness is the subject. Substantial builds and tests stay clear of timed collection.

| # | Observation | State today | Smallest adaptation (from RESEARCH-G) | Record |
|---|---|---|---|---|
| O-1 | Labels recomputed during orbit; no two plates overlap at rest | Frame-time half already measured in both orbit windows; non-overlap is enforced by the product but not witnessed | A label-placement query reading each plate's rectangle at the settled boundary | Reported-presentation interval p95 as today; rendered count against budget; the overlap check's result |
| O-2 | A held selection of five hundred elements during orbit | Large selections supported; today's cue recolours the instance and draws an overlay, it is not a halo | A box sample whose frozen oracle yields 500 typed references; the existing orbit window with that selection retained | Interval p95; renderer calls and triangles at the window boundaries; worded as the cost of today's cue |
| O-3 | Row selection to canvas highlight | Exists; only an untimed reset in the instrument | A timed action class activating the real tree row for a frozen sample of typed references, with the point lane's causal witness | Latency p95, stimulus to presented feedback |
| O-4 | Picking a node by its label plate | Exists; the point population runs labels off | A small labels-on sample with expectations from the plates' own entity keys, real pointer click; the frozen labels-off population untouched | Pick result against expectation; latency |
| O-5 | Hide and Show All on a large set | Exists with functional coverage; no benchmark action | A named visibility action with typed references | Latency to presented; hidden count as shown |
| O-6 | The routing draft ghost following the pointer, and commit | Exists for one draft; no benchmark action | A named route-draft setup through the real command and picker, real pointer movement; optionally a commit action | Interval p95 while moving; latency from Enter to the committed model generation |
| O-7 | Orbit over the static deformed overlay | The overlay draws, but no instrument route reaches it, and whether the in-product preview solves the benchmark fixtures is not established from source | A named deformation setup through the real mechanics-preview control. If the preview cannot solve the fixtures, record that and stop; do not load the frozen overlay fixtures by a side route the product does not have | Interval p95 in the existing window; it is the static overlay, not animation |

O-1 to O-4 are the core. O-5 to O-7 are worth having and may be dropped if an adaptation proves larger than it looks; a dropped row is reported as not observed, with the reason.

## 3. Unavailable measurements, stated now so they are not expected

Not recorded by the qualified lane today and not requested here: JavaScript heap, process RSS and any memory delta (recorded as unavailable by the instrument itself); native cold start; time from run completion to first coloured paint (no result colour and no run in the lane); physical scanout; hardware GPU execution time (trace spans are same-window context only); continuous display monitoring (boundaries are snapshots). Device pixel ratio 1 and the redesign's canvas sizes of 603 × 828 and 1000 × 828 are outside the instrument's frozen boundary profile, which admits device pixel ratio 2 at 1440 × 920 only; a second frozen profile is an instrument change for the later qualification under D-72, not for this run.

## 4. Future observations, after implementation exists

Each waits for its subject to be implemented, and none is a prerequisite for implementing it: result colour with edge lines and labels at Budget (rendering brief §4 row 1); per-node interpolated colour, where the source reading already says the one-segment instanced path cannot vary colour along an element, so this is a design-and-architecture question before it is a measurement (row 2); deformation animation and the cost of changing the factor (row 3); two hundred ghosted proposal elements and time to accept (row 4); theme switch with results shown, which also needs a token-driven canvas palette since today's switch repaints little of the canvas (row 7); one-pixel edge line and fixed-size glyphs at both canvas sizes and both device pixel ratios (row 8); isolate as dimming, which differs semantically from today's isolate-as-hide. These feed D-72 item 6's supplementary proposal.

## 5. What this brief does not ask

No product change, no tolerance, oracle or target change, no change to the frozen point, box, filter or orbit populations, no rerun of the baseline cohort or the demonstration, and no claim. The observation report states what was measured under the standard claim fence; the original failures and the successor demonstration keep their attribution.

## Withdrawn as a request, 2026-09-18 (appended; the text above is unchanged)

The owner asked in session what this brief was for, noted that the piping session is on hold while the design program completes its work, and asked whether anything in it must be done by that session. ROOT's answer, recorded here: the brief existed because D-70 effect 4 provides for a separate observation run and because the benchmark instrument belongs to the piping session; its adaptations are changes to product test code, which the design program's design-only assignment does not cover. But nothing in it has to be done now, by anyone. None of its seven observations would change a design decision: the facts that do (what the product draws and does not, where isolate, selection and theme differ from the design) came from reading the source, and the measurements that matter are those of the redesigned product under D-72. Most of what it would measure, today's selection cue and the 80-label cap, is replaced by the redesign. **ROOT therefore withdraws the request. The notice was not relayed and needs no relay.** The classification in RESEARCH-G and the lists in §3 and §4 remain useful to whoever writes the implementation and qualification briefs; the row for the static deformed overlay (O-7) records an open question for that work, whether the in-product preview can solve the benchmark fixtures.


Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
