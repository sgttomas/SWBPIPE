**The current desktop path does not provide baseline-versus-changed-design comparison.** It provides editable project persistence, one saved result/run carrier, historical reopening, and a **single-run load-basis comparison**. Separate two-state/two-run comparison engines exist, but I found no desktop wiring to them.

Basis: committed `fd195cf4287e84572a12183169478a6f7ddf6a92`; no dirty B4 bytes used. Actual execution: **TASK Type2, gpt-6-astra / low**, read-only; no tests, builds, UI or mutations.

### Reachable capability

| Capability | Source evidence | Meaning |
|---|---|---|
| Solve and view comparison | `src/features/workspace/resultsSessionState.ts:78–81`; `src/App.tsx:608` | Comparison is derived from the single current solved result and its analysis-run envelope. |
| Compare load case with combination | `src/services/previewService.ts:272–395` | Defaults are `load:L-100` and `combination:C-OPER-ALT`. Both sides use **the same model-state reference and run ID**; kind is explicitly `single_run_load_basis_review`. |
| Read comparison deltas | `src/features/comparison/ComparisonPanel.tsx:8–80` | Shows reference/target bases, matching diagnostics and the four largest mapped deltas. There is no baseline/run picker here. |
| Save project and run | `src/services/projectService.ts:571–611`; `src-tauri/src/lib.rs:986–1040` | Save supplies one model, mechanics result and analysis-run object. Native persistence upserts by project ID, replacing the stored payload; this is not an immutable named-state repository. |
| Reopen saved result | `src/features/workspace/workspaceSession.ts:1760–1797` | Restores a Historical context, clears Current result/run and current-session manifest. A new solve is required for current report-package readiness. |
| Edit, Undo/Redo, solve again | `workspaceSession.ts:1387–1421` | History operations invalidate computed result/run state. Undo is not a retained baseline-run comparison mechanism. |
| Report state/run/comparison sections | `src/features/report/stateComparisonHandoffSections.ts:197–224` | Consumes one model/result/run and the existing preview comparison. Section names do not establish two-alternative comparison. |

The current comparison uses explicit source-result references and equal source units; it does not apply engineering tolerances (`previewService.ts:295–328,378–394`). A user-created model lacking those default load-basis IDs may yield no relevant pairs. This is narrower than arbitrary user-selected load-case comparison, and much narrower than design-alternative comparison.

Saving before an edit does preserve a disk snapshot temporarily, until another save overwrites that project ID. The inspected path does not preserve an immutable baseline while storing a changed alternative beside it. I did not establish a reachable named-state, run-list, Save As/copy or two-project comparison workflow.

### Existing reusable pieces, presently unwired

- `core/comparison/model_state/engine.py:64–93`: `compare_model_states(left_state, right_state, mappings, settings)` accepts state records plus explicit entity payloads and stable identities.
- `core/comparison/analysis_run/engine.py:165–196`: `compare_analysis_runs` accepts separate runs/results, explicit mappings, optional tolerance profile, unit conversions and settings.
- `core/analysis_runs/records.py:73–103`: constructs schema-oriented run envelopes with model-state references and result/input-manifest bindings.
- `tests/test_model_state_comparison.py` and `tests/test_analysis_run_comparison.py` exercise those engines directly.
- Desktop historical tests distinguish retained raw evidence, missing manifests, inconsistent identities and hash failures (`HistoricalRunContext.test.tsx:163–196`).

No desktop or desktop-tool references to `compare_model_states` or `compare_analysis_runs` were found. These are available implementation assets, not evidence of an installed user capability. I did not run their tests.

Copying a project alone would not solve comparison identity: project/model identity, state identity, run bindings, entity correspondence and load-basis correspondence must remain explicit. Reusing IDs across unrelated copies must not imply equivalent engineering objects.

### Smallest honest next proposal

**Recommended: one named baseline and one current alternative within the same project lineage.**

A bounded feature would:

1. Capture an immutable baseline model snapshot and its exact result/run/input-manifest evidence.
2. Keep ordinary editing and solving on the working alternative.
3. Compare explicit baseline/current model and run bases, initially matching unchanged typed entity identities and explicitly selected corresponding load bases.
4. Show added/removed/unmatched entities and incompatible results instead of inventing mappings or tolerances.
5. Save/reopen both comparison inputs without rebinding historical results to the current model.

This is more than adding a picker to `ComparisonPanel`: it needs a retained snapshot carrier, immutable identities, comparison wiring and persistence. It can reuse the existing comparison contracts without starting a general States browser, arbitrary cross-project mapping editor or R7 agent workflow. Whether it satisfies all adopted §23/R3 requirements still needs explicit scope disposition.

**Smaller interim alternative:** retain separate exported baseline and alternative artifacts and review them manually. That could support a truthful early-preview journey, but it does **not** satisfy in-product model/run comparison or silently retire PRD §23’s requirement.

Suggested discussion wording:

> “Create and solve a model, review/apply a change, and solve again. The proposed next capability will retain a named baseline and compare its model and run with the changed alternative. Today’s Comparison panel reviews load-basis results within one run.”

The full “preserve baseline → changed alternative → compare” publication promise should remain proposed until its bounded scope is accepted and implemented.

### Limits and custody

No installed-package behavior was witnessed. Absence claims are limited to the inspected desktop/controller/service/native wiring, not every repository artifact. Current PRD amendments, broader States/Comparison scope and activation remain owner decisions.

Instructions matched the previously read basis. Brief hash verified:
`451f93b27742e5bc081a24e1df3e25fdea69b7b47b64badee222e4e6f93832f4`.

Consulted hashes below use prefix `P/ = projects/chirality-piping/`; desktop paths use `A/ = P/apps/desktop/`.

```text
522464840a56bcf44949b7b3490eb2304b704e315864374387d4ef28599d35c8 A/src/App.tsx
32d38da55dd71a2f6a6a8342251a77e46c335de9a89db70b737160fc85e05ee5 A/src/features/comparison/ComparisonPanel.tsx
cc6e1ce35ff4b5deeb415c0735fec148c06b1d60ae46edbf91d5374b047bcb41 A/src/features/workspace/resultsSessionState.ts
68feb8dc93c7d3c836fe45ab7e75611a7ebc61480b73770f2d918c54bbd67e3a A/src/features/workspace/workspaceSession.ts
9047e26a578d92f5dc986e1018b7945649b1d9cda662b131e53c4ca633ae10d7 A/src/services/previewService.ts
da38305ab02c69be95cdae4e79c9546568038716d597cc72385883938cc1363d A/src/services/projectService.ts
6ea1c8f46a32edc2c5d69cb6f9a94ea64d53e18e3ede324ef5d4e8e9fd56c880 A/src-tauri/src/lib.rs
197c62814de74f629e5d6d8e2a91d4b759f2aa0b885eda9da3bc44f08b601c8d A/src/features/report/stateComparisonHandoffSections.ts
6d5a762e6d7e1b6238eb3eb8873e83f88904af6986a84b52f13049f4061f94c9 A/src/features/results/HistoricalRunContext.test.tsx
64220403f6b0cfc81e7ecbdbad9a82ec73527644a30aa722592614b08bf067a2 P/core/comparison/model_state/engine.py
14aada4049fde08c6f68a99a844f84e73b16fd8b6e4bb2a91cb7e4c8f63b7f2c P/core/comparison/analysis_run/engine.py
709f3a16f01b617084dd8d522081138826cae5d390fa0fc56dec23cb6c1a6bb0 P/core/analysis_runs/records.py
6efee096f066965be8ad91d8ede961b3df8a050ca7c5743104c839f88e88594d P/tests/test_model_state_comparison.py
460079325e00dcb6424f5458ade02e35d2f5415d8c6d048bc069ba010f4f0b72 P/tests/test_analysis_run_comparison.py
```

Live continuation discussion/context files:

```text
e25e12686a857d5e0108beb0fa77c3e28fb7e64e79919504d10e339508ac43cc MVP_PUBLISHING_DISCUSSION.md
0a488f8093501ebc002d72e10cf1a260fb389acf71d2beba30fa91c29a6f7238 _run_records/MVP_BASIS_INVENTORY_RETURN.md
```

Root/Piping/TASK/loop instruction hashes remain the four previously reported unchanged hashes.
