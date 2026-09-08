# NATIVE launch brief amendment — load-case draft initialization variance

**Status:** `SEALED_AND_RELAYED_DURING_EXECUTION`
**Amends:** `LAUNCH_BRIEF_V1.md`, SHA-256 `7d3aaeeade894dd662d3ca793984b1d25a34754a6ec329d52294d3367a510bb2`
**Follows:** `LAUNCH_BRIEF_AMENDMENT_V1.md`, SHA-256 `afc30a71ec2d39eca7b5868f4e706fa761c622bf5973f2b0794d5fa5e0c1b61b`

In the native load form, the sole case selector visibly shows `load:UI-A` while **Queue** remains disabled. Root records this as an apparent existing form draft-initialization issue and permits this UI-only operational workaround: create a distinct second synthetic case through the native UI, record its exact generated identity, select it, switch the Case field back to the required `load:UI-A`, apply the exact `load:UI-A-FY = 350 N` load, and delete the temporary case through the native UI before solving. Do not use source changes, direct store manipulation, hooks, or out-of-band state changes.

NATIVE must preserve the final fixture's required IDs, values, quantities, units, provenance, and hashes; record every workaround action and the observed disabled state; and report this limitation explicitly. The result must not be described as a clean-path pass. All original native evidence, persistence, solve, undo/redo, visual, cleanup, and downstream acceptance gates remain unchanged. This operational adaptation makes no engineering, API, ownership, source-scope, or acceptance decision.
