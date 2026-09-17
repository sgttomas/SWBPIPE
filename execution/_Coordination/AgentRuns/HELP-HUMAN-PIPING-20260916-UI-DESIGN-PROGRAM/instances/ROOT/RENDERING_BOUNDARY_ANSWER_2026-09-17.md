I fast-forwarded my worktree to main so I'm now looking at the merged foundation rather than the pre-#789 shell, and I read the D-69 ruling, the follow-on brief, the V79 diagnosis, and the cohort scores.

## Where the piping session actually got to

PR #789 landed the "professional modelling workspace foundation" under your D-69 Option A ruling: functional acceptance, with the 1,000/10,000-pipe performance qualification left open. The renderer in that PR is a genuine rebuild: one persistent WebGL context, spatially chunked instanced meshes for pipes, nodes, supports and components, render-on-invalidation with a settled-frame guarantee, an ownership ledger for disposal and context loss, a typed model index, a virtualized tree, and an Actual OD mode. That is sound engine work.

The performance status is better than "has not passed" suggests, and the open question is narrower than it sounds:

| Fixture | Valid runs | Assignment | Point p95 | Orbit p95 | Status |
|---|---|---|---|---|---|
| 1,000 pipes | 5 of 5 | 221 to 232 ms | 29 to 48 ms | 8.3 to 16.7 ms | pass |
| 10,000 pipes | 2 of 5 (one invalid evidence, two never produced) | 580 ms | 58 ms | 16.7 ms | incomplete |

Targets are 2,000 / 100 / 16.7 ms. The separately recorded 33.33 ms orbit failures were taken apart by the V79 diagnosis: main-frame render work at 10,000 pipes is about 3.3 ms p95 and GPU work under 1 ms, while the reported interval comes from Chromium's estimated future display time on a 60 Hz external monitor snapping phase by one period. That is the "frame-timing measurement question": the acceptance metric is currently measuring the monitor's presentation estimator, not the product. The 16.668 ms passes sit exactly on the 16.7 target because a 60 Hz display cannot report anything better, so the metric can flip from pass to fail with zero product change.

## Does the redesign replace the rendering architecture?

No. The engine layer stays: Three.js, instanced chunking, the invalidation scheduler, the resource ledger, the model index, picking and box selection. What the redesign will replace is the presentation layer above it and the chrome around it: real outside diameter as the default rather than schematic tubes, bends and fittings as geometry, distinct support glyphs, load and result overlays, deformed shapes, an annotation and dimension layer, a label strategy that is not 80 DOM chips, and the theme, materials and lighting. It will also give the canvas far more pixels. Every one of those changes the render workload, so the current qualification is a baseline for the engine, not an acceptance for the redesigned product. The follow-on brief already says as much. One caveat for later: a results experience with thousands of annotations may justify a text-rendering or WebGPU extension, but that is an addition decided in the design program's rendering brief, not a replacement decided now.

## What should continue in the piping session before redesign implementation

1. **Make the measurement decision now, as an owner decision, not another diagnostic.** My recommendation: gate on what the product controls, meaning main-frame render work per frame plus the settled-frame and resource obligations, and carry Chromium-reported presentation as a disclosed observation with the display profile stated. The engineering rationale is independent of the score: the product cannot control the compositor's display-time predictor, users run 60 Hz and 120 Hz displays, and the redesign needs a regression instrument that does not flip on monitor phase. If you prefer to keep the reported-presentation metric, then the reference profile must name the display explicitly and a 60 Hz external monitor is the wrong reference. Either way, decide; V79 has already answered the physics.
2. **Run exactly one complete cohort** on the merged product with the settled method. The gap is the three missing 10,000-pipe runs. This produces the engine baseline that every redesign build will be compared against.
3. **Turn the harness into a reusable instrument.** Drive it through named view commands and typed entity identities rather than panel placement, so it survives the layout change. Keep the 1,000/10,000 fixtures frozen. Add the workloads the redesign will introduce as observations only: Actual OD as default, labels on, result overlay and deformed shape on. That tells us the headroom before design commits to it.
4. **Continue non-UI engineering** in its own write scopes: pressure runtime, connectors, sparse. None of it collides with design work.
5. **Do not** start any further UI tranche, refactor the viewport or shell for performance, run more compositor diagnostics, or touch the results and diagnostics panels. Those are all redesign territory now.

The stopping point should be bounded: one measurement decision plus one cohort, time-boxed to a single loop session. If the cohort still cannot complete, carry the limitation forward as a documented open item and start redesign implementation anyway. Design work continues in parallel regardless, and the design program owes the piping session a rendering brief so step 3 measures the right things.

## What #789 changes in my first report

The foundation fixes real mechanics: ordered multi-select and box select, hide and isolate, fit commands, measurement, a virtualized tree with a resize handle, Properties and Task tabs, light and dark themes, two densities, and Actual OD. The dark theme is competent. But the five diagnoses stand. The menu bar, the new command band, the command bar, the viewport toolbar and the dock are five command systems. At 1440x920 with both rails open and no dock, the canvas gets roughly a third of the window. The results, diagnostics and review panels were not in the PR, so the machine-string walls, the seeded five issues on an empty model, the "Invented" entity names and the truncated label chips are all still there, and the tree now shows a raw diagnostic code as a node. The foundation is a good engine under the same instrument panel.

I'm ready for your answers to the seven questions whenever you are.

