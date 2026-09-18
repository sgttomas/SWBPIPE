# Point 58 numerical root cause — recorded-input calculation

**Confirmed immediate mechanism: product picking splits an effectively tied node/pipe hit by floating-point normalized-miss error larger than its 1e-12 tie tolerance.** The independent oracle keeps the three hits tied and applies node-before-pipe priority. Actual pinned product picking reproduces the recorded pipe without changing the workload or expected oracle. Pointer float32 rounding is confirmed but is not the sole cause.

## Calculation and results

`compare.cjs` executes the actual pinned exported `buildModelIndex`, `pointPickPrimitives`, `pickPointPrimitive` and independent `expectedHitForProbe`. TypeScript is transpiled in memory for these local calculations; no product build, tests, browser or source modification. Product source revision verified 8468a33c86adb622b25e98f98b0eaf28c7e9fa0e; instrument revision df1562e06c90653a96252e19f5730b19b8d4f66c. Model bytes hash dcdcac0ff2031050c3de2c4cdef23a052ec68f2f663acc2ab53e7dab648f8b85, matching recorded run binding.

Each of four failed attempts (1,2,4,5) supplies before/after camera, canvas and intended/delivered pointer. A 2×4 matrix uses before/after camera and intended/delivered/X-only rounded/Y-only rounded points: 32 calculations total. Actual exported product function and an instrumented exact-source copy agree on every winner. The copy changes only the returned diagnostics to expose candidates/groups; it does not change arithmetic or selection. Oracle diagnostic copy similarly adds returned intermediates. Source hashes and replacement operations are preserved in script/matrix.

Exactly 28/32 product cases disagree with the oracle; four after-camera/intended-point cases agree. All eight delivered-point cases disagree. All four attempts give the same pattern:

| Reconstructed camera | Point | Product | Independent oracle |
|---|---|---|---|
| Before | Intended, delivered, X-only or Y-only | pipe:UIF-08786 | node:UIF-08786 |
| After | Intended | node:UIF-08786 | node:UIF-08786 |
| After | Delivered, X-only or Y-only | pipe:UIF-08786 | node:UIF-08786 |

Both intended coordinates round exactly to delivered values under `Math.fround`: X 839.3801013839062 → 839.380126953125; Y 657.5079070039999 → 657.5079345703125. This proves float32 representability/rounding equality, not which browser layer performed it. In after-camera reconstruction changing either single coordinate from intended to delivered is sufficient to flip node to pipe. In before-camera reconstruction no coordinate change is needed: the product already chooses pipe. Therefore “the browser rounded the click” alone is an incomplete root-cause explanation, and pre-rounding oracle inputs does not resolve the product/oracle mismatch.

## Decisive numerical boundary

Representative before-camera, delivered-point case (identical across all four attempts):

| Candidate | Product entry distance | Product normalized miss | In product miss group? |
|---|---:|---:|---|
| pipe:UIF-08786 | 16.70697752083452 | 0.8971722704697218 | Yes |
| pipe:UIF-08787 | 16.70697752083452 | 0.8971722704714609 | No |
| node:UIF-08786 | 16.70697752083452 | 0.8971722704714609 | No |

The approximately 1.7391e-12 miss difference exceeds the 1e-12 grouping threshold, so the node never reaches type-priority comparison. Independent oracle gives all three normalized miss 0.8971722704825292, entries differing only ~4.75e-13 (well inside relative front tolerance), and all three enter its miss group: node wins. Product broad-phase includes node and both front-group pipes; this is not missing-node broad-phase culling. Full front/miss groups, relevant candidate values and broad-phase count 20601 are in matrix.json.

Product `viewportSelection.ts:741–798` computes sphere miss via subtraction `toCenter.lengthSq() - along*along`, while capsule miss uses Three.js `ray.distanceSqToSegment`. These algebraically related distance paths accumulate different rounding/cancellation. At `:598–605` front hits are grouped by relative 1e-9 entry tolerance then absolute 1e-12 miss tolerance; `:846–863` node priority applies only after that filter. The independent oracle `point-hit-oracle.mjs:342–347` computes misses through explicit closest-point/vector-distance paths; `:366–373` applies the same semantic tie policy. The decisive divergence is at normalized-miss grouping, not at the final priority table, timing metric, pixel cue assertion or reset setup.

## Confidence, limitations and recommended correction

High confidence in a reproduced numerical tie-instability mechanism sufficient to explain the observed typed mismatch. Delivered-coordinate cases reproduce pipe with both recorded camera reconstructions. Saved diagnostics provide authored position/target/up/FOV rather than exact runtime quaternion/matrix elements; reconstruction uses the pinned Three.js PerspectiveCamera/lookAt and subtracts recorded local origin. It cannot prove bit-for-bit historical camera internals. No claim that the browser's internal float conversion or exact historical rounding path has been uniquely identified. These limits do not erase the deterministic exported-product mismatch on recorded inputs.

No repair has been implemented. Recommended bounded follow-on is a **product picking numerical-stability correction**, separately authorized outside this baseline tranche: compute geometrically equivalent sphere/capsule endpoint miss distances consistently with a stable closest-point distance formulation, preserving front/tie/type semantics and current tolerances. First add a regression from this recorded model/camera/point matrix demonstrating endpoint-sharing node/pipe tie consistency across intended and delivered coordinates; independently verify the analytic tie, not merely force this desired identity. Do not just enlarge tolerance, change coordinates/camera, replace the expected node with observed pipe, disable typed/marker checks or rewrite the oracle. A specific corrected implementation remains unproved until that focused repair is performed and reviewed.

What was done: preserved failures, inspected exact identity/cue evidence, then executed this deterministic actual-product-versus-independent-oracle matrix after collection. What was not done: no product/harness repair, new timed run, extra attempt, scoring change or relabeling. All five N10000 attempts remain consumed and invalid; attempt3 owner interference is separate. The repeated point58 issue is product picking numerical behavior relative to the independent tie contract, not evidence of poor timing performance.

TASK /root/d70_expectations, parent /root, caller-configured gpt-6-astra/low; no independent model introspection; scope/non-delegation instruction/config asserted. Single bounded calculation complete. F-PIP-2 / DEC-081 claims fence applies.
