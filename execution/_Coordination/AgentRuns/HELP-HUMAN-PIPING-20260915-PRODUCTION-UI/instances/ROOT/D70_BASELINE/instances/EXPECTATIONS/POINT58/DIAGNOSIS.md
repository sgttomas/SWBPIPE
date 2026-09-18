# Point 58 saved-evidence diagnosis

**Verdict: reproducible typed product/oracle disagreement; no concrete harness setup/assertion defect or semantics-preserving correction established in this bounded pass.** High confidence in the immediate failure mechanism; insufficient evidence to assign underlying fault to product picking versus independent oracle/numerical input sensitivity. Do not weaken the guard or substitute the observed pipe into the expected oracle.

Both N10000.1 and .2 select `{type: pipe, id: pipe:UIF-08786}` after the real pointer action. Frozen candidate-runtime oracle and action expectation require `{type: node, id: node:UIF-08786}`. The workload anchor is a different entity, `{type: pipe, id: pipe:UIF-02124}` at authored `[9.75, 0.1378405, 0.2230525]`; anchor identity is not expected winner identity. The current guard correctly distinguishes these typed identities.

## Decisive retained evidence

For each attempt's point-selection-058 directory:

- `point-before-state.json`: reset selection is exclusive project:UIF-10000, labels disabled, schematic geometry, frozen camera, expected node:UIF-08786. Oracle reports 108 candidates, 3 front-group and 3 miss-group members; node wins the frozen tie ordering. Product projection crosscheck MATCH is only a projection check, not independent validation of product picking.
- `action-stopped.json`: real down/up captured at client `(839.380126953125, 657.5079345703125)`; intended client point `(839.3801013839062, 657.5079070039999)`. Both attempts have zero feedback markers, no observer errors/overflow, one CONTENT_NOT_READY rejection. Rejection details already show pipe:UIF-08786 in rendered selection, primary ref and inspector, with real pointer selection association. Thus this is not merely a screenshot arriving late.
- `point-stopped.json`: both orderedRefs and primaryRef are pipe:UIF-08786, inspector ref agrees. Before project selection becomes pointer pipe selection (action sequence 176→177, model generation 1). Render submission becomes 244 in .1 / 238 in .2. Camera sequence advances 121→122 with approximately 4e-15 coordinate differences; no evidence here proves that tiny change causes the mismatch.
- `pixel-witness.json`: BOTH have `exact=false`, `markerBound=false`, but `verdict.status=PASS_PAIRED_WINNER_CUE_TRANSITION`, changed=true, count=23, minimum=4. Visual transition succeeded. `highlight-after.png.json` reports geometry=true/noDrift=true and stable epoch identity. Saved 96×96 before/after crops visibly show a new white-rimmed selection cue; a cue near the expected node is not proof of the expected typed selection. The paired cue alone would falsely mask this disagreement.
- Returns record 57 completed points each, INVALID/INCOMPLETE, process exit 1, verified cleanup and INTERNAL120_EXACT_POSTCHECK_PASS. Neither failure is a valid timing-target miss. No box/filter/orbit completion follows.

## Source interpretation

Instrument `full-cohort-controller.ts:654–683` resets the project, turns labels off, applies frozen camera, projects the authored anchor, converts canvas-local to client coordinates and stimulates real `page.mouse.click`. Lines 691–696 independently require exact typed membership/primary/inspector identity, one qualified marker, and visual transition. Saved data falsifies the identity and marker requirements while satisfying visual transition; throwing is appropriate.

`point-hit-oracle.mjs:356–380` constructs a shifted-camera analytic ray, computes candidates, uses relative 1e-9 front grouping, 1e-12 normalized-miss grouping, then component/support/node/pipe priority. `src/features/viewport/viewportSelection.ts:570–609` uses product broad-phase membership followed by exact primitives and analogous front/miss grouping. Saved evidence does not include the product's candidate list, winning distances/misses or rejected node candidate. Consequently it cannot determine whether broad-phase selection, primitive math, tie sensitivity, coordinate transport or another product/oracle difference produced the observed pipe.

There is a small intended-versus-observed pointer difference (~0.00002557 CSS px X and ~0.00002757 CSS px Y), and the oracle reports a three-way near tie. This is a bounded possible sensitivity, not a demonstrated cause. No oracle recomputation, source execution, test or new probe was performed. Changing coordinates, camera, tie tolerances, expected identity, or suppressing marker/typed checks is not supported as a semantics-preserving correction by these observations.

## Disposition

No repair proposed. Another unchanged attempt is not supported as a likely resolution: two fresh attempts produced the same typed disagreement, cue verdict, pointer coordinates and failure location. This is not proof that every future run must fail, but timing noise alone has no demonstrated corrective mechanism. ROOT should treat the saved evidence as a concrete repeated product/oracle mismatch with unresolved underlying cause, retain both consumed failures, and decide remaining bounded scope; this packet authorizes neither expanded diagnosis nor another run. Slots 10000.3–5 remain unattempted under supplied brief.

Single read-only pass complete. TASK /root/d70_expectations, parent /root, caller-configured gpt-6-astra/low; model not independently introspected. Native scope/non-delegation instruction/config asserted. No source writes, tests, browser, build, raw traces, scoring change, new probe, repair, runtime or delegation. Product remains 8468a33c; method 1e0ab8a4e4a57753084d7ceca695b0b4b952aa047923cb48e3dbd3074bf22a78. F-PIP-2 / DEC-081 applies.
