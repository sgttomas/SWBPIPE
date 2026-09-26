# RETURN — T1_WP3_NATIVE_INPUTS

- **Role:** TASK (Type 2). I report to the T1 load-state WORKING_ITEMS manager.
- **Brief:** `TASK_BRIEFS/T1_WP3_NATIVE_INPUTS.md` (updated at `1b95d0b66`), read with `_T1_COMMON.md` (Wave 2). I also read the WP2 desktop readers RETURN and `T1_WAVE1_RULINGS.md` §12 and §14.
- **Manager rulings during execution:**
  - C1 widened my write set to `App.tsx` (the control) and `features/workspace/workspaceSession.ts` (the handler).
  - C2 set the e2e route (blank 0.4.0; no fake backend).
  - C3 accepted the validate-only pre-check and the labels.
  - These are now §15 of the rulings (commit `ae0fac8c6`).
- **Checkout:** load-state worktree, branch `codex/piping-load-states-20260925`.
  - My base is `1b95d0b66`. HEAD later moved to `ae0fac8c6`, which is docs only.
  - I made no Git writes.
- **Paths:** relative to `projects/chirality-piping/`. Run records are in `_run_records/`, with machine paths replaced by `<SCRATCH>`, `<REPO>` and `<SIBLING_WORKTREE>`.
- **Status:** execution is complete. This is not acceptance.
  - Every check passes, and all 14 mutants are killed.
  - Native behaviour is not claimed. No e2e shows a 0.4.0 solve: the browser has no solver backend (§3.3). Native solve and the block's display in the app remain owner's-Mac witness 4. Witnesses 2–8 (`T1_PLAN.md` §6) stay outstanding.

## 1. Files changed

WP2 bytes are unchanged, including every protected one: `types.ts`, `loadReferenceEvidence.ts`, `loadReferenceSourceEvidence.ts`, `numericalResultQuality.ts`, `LoadReferenceOutputGate.tsx` and `loadReferenceOutputAvailability.ts` (`git diff --quiet 1b95d0b66` is clean on all six), and their tests. In `SolvePanel.tsx` the solve-job download gate is unchanged; I added one line that mounts the block. In `projectService.ts` I only added a new function, and `buildBlankLocalModelDocument` is byte-identical.

| File | sha256 before (`1b95d0b66`) | sha256 after |
|---|---|---|
| `apps/desktop/src/App.tsx` | `0f6f778f7d4d64620c3bea166111e2750dca472e3687c9eb35ba129ff59c5125` | `2960da41f6bebda32df814e9aeca3b4670dcaf9d99b3903f83188a5cd96738db` |
| `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` | `68b49ca96314cba671bf63be30142a0ef07ccf1cbf78604a1f1e46258a41110f` | `a27afa7b0e7e72a3cbf75672fbee81d29a210c9eb0cfd938f08195c243e0a980` |
| `apps/desktop/src/features/material-temperature/MaterialTemperatureForm.tsx` | `e666eb7098246855e56ca30ac4d5b665f9b8fdbd85dd634dafc85d502e8bad00` | `8867b10e4101dc167dfa4e07d53c9784e1ac4540bbc8a46a5a5f9213ea542634` |
| `apps/desktop/src/features/solve/SolvePanel.tsx` | `73bba1ca4a29763655b2dd2e4f31a72f3f1e2450822f3bb29d7c686b397947f9` | `69879ebde5a5842eae780f507d1bbd4c011cd41ae319d6f891a6b303e4841cb7` |
| `apps/desktop/src/features/workspace/workspaceSession.ts` | `d7cad0f732a3ab642435016fd723b49172cc33f4ea9a3a3e74de9524c14a6734` | `b1e59fa4c4a2409a4694cda56662db365fb302bb0e26296b9b50096cf62204f8` |
| `apps/desktop/src/services/projectService.ts` | `7d9b644c78dedeec954a14ba934a46505a26e83ef8a4426a13beb425584a5293` | `4b639ab7c46623ca53eedfcf267edba99f7eed365423984215c1806200e41f40` |
| `apps/desktop/src/features/load-cases/loadStateAuthoring.tsx` | (new) | `eb5b46490481c0fbf8b980714b8d9ccdabcf605f90b64f040d5386cffd04db03` |
| `apps/desktop/src/features/load-cases/LoadReferenceStateInputs.tsx` | (new) | `b9a1fe749e3a5ef5403f5935abf92816acfe9293206e064d32da61492edd2847` |
| `apps/desktop/src/features/load-cases/LoadReferenceStateInputs.test.tsx` | (new) | `9b9b53220794e65772a516435920015cd2e58dcfaa3d40734a2a210453a432c7` |
| `apps/desktop/src/features/material-temperature/ExpansionLawsEditor.tsx` | (new) | `0aeb9acd9430cd8b1a6ca06982bb8c570cea9b5c866c117592f5591271c9561e` |
| `apps/desktop/src/features/solve/LoadReferenceStatesBlock.tsx` | (new) | `878680cd49ac0857bb0cfee57d112337128a331ad22bed2c8aa8c78fe525db30` |
| `apps/desktop/src/features/solve/LoadReferenceStatesBlock.test.tsx` | (new) | `a32082882695d36bb32eb9416edc3103607c308cfd404833fdae1213af89577a` |
| `apps/desktop/src/features/solve/LoadReferenceStatesBlock.joinedGuard.test.tsx` | (new) | `609e08309180ed60940df97528d53fa50f0099d45d06aa1e4a223d20479d2576` |
| `apps/desktop/src/services/blankLoadStateModel.test.ts` | (new) | `8647fc112c02e83a49b3925b8574fe49c8a423a11ccf427bc04d3b5eeb8974fc` |
| `apps/desktop/e2e/load-state-inputs.spec.ts` | (new) | `f26da2139e9008086301a379d8bd4422805ebd5ee7c03ff8df757ff45f09dc95` |

Three other files in the live worktree differ from HEAD. They are the manager's review fix-ups (F1–F3, confirmed by the manager), not mine, and none of my evidence uses them:
- `OperationLedgerPanel.tsx`
- `loadReferenceOutputRefusal.test.tsx`
- `core/runner/headless/src/load_reference_route_tests.rs`

## 2. What each change does

### 2.1 Inputs (item 1)

**`features/load-cases/loadStateAuthoring.tsx` (new)** holds the shared pieces.
- `isLoadStateModel` and `LoadStateNeeds040` provide the gating. On any `schema_version` other than `0.4.0` the fields are not offered; one line is shown instead: "Load/reference state needs a 0.4.0 model." There is no upgrade operation.
- `useLoadStateQueue` submits each edit as one whole-record replacement, through the typed operations in `load_state_authoring.rs`:
  - `Model/reference_configurations` with `set_field`;
  - `Material/expansion_laws` with `set_field`;
  - `Load/analysis_state` with `update_load`;
  - unit `none`, dimension `dimensionless`, and the before-value canonicalised by the engine (`canonicalJsonString`), or `not_present` when the key is absent.
- Submission order (C3):
  1. `validateModelOperation` runs with the current `computeModelHash`. This is the same wasm/IPC engine the queue uses, in validate-only mode.
  2. If the engine reports a blocking diagnostic, nothing is queued. Each diagnostic (code, severity, message) is shown inside that field group, and the draft and the model stay as they were, still editable.
  3. Otherwise the intent goes to the normal queue (`onQueueIntent`), where Review/Apply, Undo/Redo and the existing batch engine handle it.
  - There is no client-side model mutation and no second validator.
- Removal sends the operation's explicit `not_present`.
- `toPayload` turns entered text in quantity slots (`{value, unit}`) and in `factor` into JSON numbers.
  - A blank entry is an absent key.
  - Text that is not a finite number is reported, because it cannot be a JSON number. This is input conversion, not validation.
  - Every other key, including records the fields do not edit, passes through as authored.
- Fields have no default, placeholder quantity, library value or code value.
  - Every select starts at "Not provided".
  - Every unit is typed explicitly.
  - A discriminant change keeps only the keys of the new variant.

**`features/load-cases/LoadReferenceStateInputs.tsx` (new)** is mounted in `LoadCaseManagerPanel` beside the pressure authoring (one line added there).
- Reference configurations: ID, label, geometry, and provenance. Each member has its pipe, its basis (temperature reference with installation temperature, or direct strain), its fit (none, natural length change, or fit strain) and its provenance.
- The selected case's `analysis_state`:
  - contract, reference configuration, history and provenance;
  - element states: pipe, optional operating temperature, material selection (explicit base properties / exact point / temperature interpolation), and thermal state (unchanged reference / explicit interval strain / constant-alpha interval / free-length state with the material's law);
  - support states: support, participation (active or inactive), and `boundary_motion` rows (DOF, displacement with unit, meaning);
  - `load_sources`: the case's stored primitive and its factor.
- Lists show "absent" or "N entries". A list is created by "Add", made absent by "Mark … absent", and becomes an explicit empty list when its last row is removed.

**`features/material-temperature/ExpansionLawsEditor.tsx` (new)** is mounted in `MaterialTemperatureForm` (one line). It edits `Material.expansion_laws`:
- definition (the four closed definitions), ID, datum temperature, data (constant coefficient for the secant only, or a table with its interpolation and points), and provenance;
- the same gating line appears on a pre-0.4 model.

**Retained but not editable here.** These keys pass through unchanged and are named in the form as "retained as authored":
- the components of `locked_equivalent_support`, `base_motion`, `device_reference`, `analysis_basis_override`, `mass_state_ref`, and predecessor position sources.

The brief's required groups are all editable.

### 2.2 Blank 0.4.0 model (item 2, C1)

- **`projectService.ts`:** new `buildBlankLoadStateModelDocument`. It produces the blank document with `schema_version` "0.4.0", `pressure_contract` `{version "2.0.0", mode "exact_straight_pressure_v2"}` and the project name "Blank Local Model 0.4.0". Everything else equals the existing blank, and all collections are empty. There is no `reference_configurations` key and no engineering content.
- **`workspaceSession.ts`:** the old `handleCreateBlankProject` body became `createBlankProjectFrom(builder, message)`.
  - `handleCreateBlankProject` calls it with `buildBlankLocalModelDocument` and the unchanged message.
  - The new `handleCreateBlankLoadStateProject` calls it with the 0.4.0 builder.
  - No native IPC change was needed: `create_local_project` stores the model it is given.
- **`App.tsx`:** a "Blank 0.4.0 model" button (`data-testid="new-blank-040"`) beside "New blank".
  - Its accessible name deliberately does not contain "New blank". Existing tests find New blank with `/New blank/i` and `/^New blank$/`, and those tests pass unchanged.

### 2.3 Read-only resolved-state block (item 3)

**`features/solve/LoadReferenceStatesBlock.tsx` (new)** is mounted in `SolvePanel` (one line), after the solve-job audit.
- **Route.** The route comes from WP2's `sourceContract`. The block renders only for `load_reference` and `load_reference_source`.
- **Admission.** Values appear only after the WP2 reader for the route admits the result.
  - For `load-reference-1`, `validateLoadReferenceEvidence(result)` must not throw.
  - For `load-reference-source-1`, `loadReferenceSourceStanding` must be exactly `LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`. If the result has not been validated yet, the block runs `validateLoadReferenceSourceEvidence` and reads the registration again.
  - A refusal shows the reader's code and no values.
- **Standing.** Standing comes from WP2's `numericalResultStanding(result, model)`, shown with its findings.
  - Per case, the label is "integrity checked" only when the route is ordinary, the WP2 standing is eligible, and the case's published `solve_quality` is `checks_passed`.
  - Otherwise the label is "needs recompute — not Current".
  - Joined results are always "needs recompute — not Current": the route label says so ("in T1"), and there is an explicit route guard as well as the standing check. A sensitive result is never labelled integrity checked. The word Current never appears as a positive label.
- **Display.** Plain tables of published values only, shown with `String(value)`; nothing is computed or converted:
  - element states: pipe, material, selection, operating, selection and installation temperatures (K), thermal definition, law, thermal strain, fit, fit strain, total eigenstrain, E (Pa), nu;
  - support motions: support, node, DOF, value, unit, meaning;
  - sources: each contribution with its factor and published value, plus the excluded sources.
- **No output.** The block contains no link, button, input or download. Nothing it does writes to a file, package, handoff or request. SolvePanel's `LoadReferenceOutputGate` around the solve-job JSON is unchanged, and the tests assert it for every load/reference result.

## 3. Checks

### 3.1 Commands

All runs used Node 24 from `apps/desktop`, on a scratch `git archive` copy of `1b95d0b66` with my files overlaid. It differs from the live tree only by the manager's F-fix test file.

| Check | Command | Result |
|---|---|---|
| Desktop vitest, baseline (`1b95d0b66`) | `npx vitest run` | 129 files, 2772 tests. 20 failed. **13** were my scratch setup error: the self-weight wasm asset was missing from the copy. Those 3 files passed 21/21 on a rerun once it was copied. The other **7** are the 30 s / 600 s timeout class WP2 reported: `App.deadControls` (600 s), 3 × `App.test`, `physicsSourceIntegration`, and 2 × `sourceBlockRecovery` multicase. Load average was 6–8. |
| Desktop vitest, this change, final run | `npx vitest run` | **133 files, 2812 tests, all passed** (+4 files, +40 tests). Run on a quieter host (load about 1–3). |
| Desktop vitest, this change, earlier run | `npx vitest run` | 132 files, 2811 tests, before the guard test was added. 7 failed: 6 were 30 s timeouts (`App.test` render, 5 × `physicsSourceIntegration`), and 1 (`App.test` native-menu `toggle-tree` aria-expanded) came straight after the timed-out App render in the same file. Rerun in isolation, both files passed 242/242. The final run above passed with nothing skipped and no timeout raised. |
| New tests | `npx vitest run <file>` | `LoadReferenceStateInputs.test.tsx` 14; `LoadReferenceStatesBlock.test.tsx` 20; `LoadReferenceStatesBlock.joinedGuard.test.tsx` 1; `blankLoadStateModel.test.ts` 5. All passed. |
| Type check and build | `npm run build --workspace apps/desktop` | passed (`tsc -b && vite build`). The only warning is vite's existing chunk-size warning. |
| Chromium Playwright | `npx playwright test e2e/load-state-inputs.spec.ts e2e/linear-authoring.spec.ts e2e/result-compatibility.spec.ts` | **6/6 passed**, both projects (desktop and compact), on the scratch copy. The new spec took 53–55 s against the unchanged 120 s budget. |

Playwright environment: `PLAYWRIGHT_BROWSERS_PATH=<PW_BROWSERS>`, `CI=1`, and the repo's existing `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` override pointing at the installed chromium-1194 (the pinned revision is 1223). I did not run `playwright install`.

Logs: `_run_records/desktop_checks.log` and `_run_records/e2e_checks.log`.

### 3.2 What the tests cover

**Operation submission, per field group.** Every group runs through the real wasm engine: each queued intent is applied with `applyModelOperation`.
- Reference configuration (member fit → fit strain).
- Analysis state: support `boundary_motion`, a constant-alpha element thermal state, a temperature-interpolation material selection, and a load-source factor.
- Expansion laws (a user-entered dilation table).
- Removal by `not_present`.
- Each test asserts the target, kind, path, canonical before-value and exact after-value. It also asserts that the source model object is unchanged, which shows there is no client mutation.

**Refusal display.** Each case asserts the diagnostic test id, the error text, that nothing was queued, that the field stays enabled and that the draft is kept.
- An unknown field gives `OP-LOAD-STATE-PAYLOAD-INVALID`.
- A dangling load source gives `OP-LOAD-STATE-REFERENCE-UNRESOLVED`.
- An explicit null gives `OP-LOAD-STATE-PAYLOAD-INVALID`.
- An inbound-reference removal gives `OP-LOAD-STATE-INBOUND-REFERENCE`.
- A wrong version gives `OP-LOAD-STATE-SCHEMA-VERSION-INVALID`, on the pre-0.4 blank in `blankLoadStateModel.test.ts`.

**No defaults.**
- Every control of an absent `analysis_state`, of new rows, of a new configuration and of a new law starts empty, and every unit starts empty.
- An empty draft goes to the engine with its keys absent and is refused with "missing field `contract`".
- `toPayload` keeps absence and refuses non-numbers.

**Pre-0.4 gating.** Versions 0.1.0, 0.2.0, 0.3.0 and 0.4.1 show only the line, both in the inputs and in the materials form. The Load Case Manager shows the fields on 0.4.0 and the line on the 0.3.0 fixture.

**The resolved-state block, on the committed raws, in both modes.**
- Ordinary `connected` and `pressure`: "integrity checked", and every table cell equals the published value.
- All five joined sets (`eigen_motion`, `fields`, `mixed`, `n05`, `n06`): always "needs recompute — not Current", including the `mixed` case whose published quality is `checks_passed`. Nothing is shown before the reader admits the result.
- The committed sensitive ordinary raw (`core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-*`): "needs recompute — not Current", with finding `NUMERICAL_INTEGRITY_NOT_QUALIFIED`.
- Tampered ordinary and joined publications show the refusal and no values.
- The block has no output controls, and the solve-job gate's refusal is present.

**Blank 0.4.0 model.**
- The existing builder's RFC 8785 sha256 equals the value computed on the base tree, `da4399892d3e025ccf81eb8f2f50c0f339a22bc78f36aa86ebcf34b0e6054f10` (`desktop_checks.log`).
- The existing path's message and name are still asserted by the unchanged `App.test` "creates a blank local model document…" and by the session test.
- The 0.4.0 builder's content is as described in §2.2. It persists and reopens byte-for-byte as `current`.

### 3.3 Chromium e2e (`e2e/load-state-inputs.spec.ts`, the C2 flow)

1. The bundled pre-0.4 fixture shows only the gating line.
2. "Blank 0.4.0 model" creates the document.
3. Existing authoring then reaches a straight model: two nodes, the toolkit material, a pipe between the existing nodes, a support, a load case and a concentrated force. The existing authoring worked unchanged on the 0.4.0 blank; none of it needed a workaround.
4. Through the plain fields: a reference configuration (queued, then applied), then the analysis state:
   - material selection: explicit base properties;
   - element thermal state: constant-alpha interval;
   - support `boundary_motion`: UX 1 mm;
   - load source: factor 1.
   The engine checks it and it is queued for Review/Apply.
5. Apply changes the model hash. Undo restores the pre-apply hash, and the field shows `analysis_state` absent. Redo restores the post-apply hash, and the field shows it applied. The byte witness is the RFC 8785 model hash from the project-validation export, the same witness `linear-authoring.spec.ts` uses.
6. Solve in sparse and in dense each gives the existing browser refusal `BROWSER_SOLVE_BACKEND_REQUIRED…`, and no resolved-state block appears. The model hash is unchanged.

**No e2e shows a 0.4.0 solve.**

### 3.4 One engine refusal, captured verbatim (C3)

Route `local_wasm_engine`, saved in `_run_records/engine_refusal_capture.json`:
- `OP-LOAD-STATE-PAYLOAD-INVALID` (blocking): "reference_configurations refused at the product's closed typed boundary: unknown field `unreviewed_extra`, expected one of `id`, `label`, `geometry_ref`, `member_references`, `provenance` at line 1 column 695"
- `OP-LOAD-STATE-SCHEMA-VERSION-INVALID` (blocking; 0.3.0 model): "analysis_state can be authored only on a model whose schema_version is already 0.4.0; no operation changes the version"

## 4. Mutation evidence

- Script: `_run_records/run_mutants.py`, run on the scratch copy. Each mutant is one exact replacement, restored afterwards, and I checked by `cmp` that the sources were restored.
- Results: `_run_records/mutation_results.json` and `mutants.log`.
- **14 mutants, 14 killed.**

| Group | Mutant | Result |
|---|---|---|
| Pre-0.4 gating | G1: load-case gating removed | killed (2 tests) |
| | G2: expansion-law gating removed | killed (1) |
| | G3: version predicate always true | killed (2) |
| Not-Current labelling | L1: joined route label loses "needs recompute — not Current" | killed (10) |
| | L2: case label taken from solve quality only, ignoring route and standing (joined case labelled integrity checked) | killed (4) |
| | L3: sensitive accepted as integrity checked | killed (2) |
| | L4: joined route guard removed | killed (1) |
| | L5: values shown without reader admission | killed (2) |
| Operation submission | S1: reference-configuration submission removed | killed (2) |
| | S2: analysis-state submission removed | killed (4) |
| | S3: expansion-law submission removed | killed (1) |
| | S4: queue hand-off removed | killed (4) |
| | S5: engine pre-check bypassed | killed (5) |
| | S6: removal sends a JSON string instead of `not_present` | killed (2) |

On a first run L4 survived. WP2 standing is never eligible for a joined result, so the extra route guard is redundant against the real standing. I added `LoadReferenceStatesBlock.joinedGuard.test.tsx`, which stubs only the standing function as "eligible" and asserts that joined cases still read needs recompute. That tests the T1 rule as defence in depth, and on the rerun L4 is killed.

## 5. Findings, not done, and open items

1. **An existing Load Case Manager defect, not in WP2 code.** The panel throws when a load case has no `label`, `kind` or `status`: `proposedMetadataValue.trim()` of `undefined` at `LoadCaseManagerPanel.tsx:219`. The committed 0.4.0 request models' cases have none of these fields.
   - Cases created through the desktop always carry them, so the e2e is unaffected.
   - I did not change that behaviour. The mount test adds invented metadata and says so.
2. **An existing SolvePanel assumption.** `SolvePanel` spreads `model.diagnostics`. The two ordinary request models (`load_reference/connected|pressure.request.json`) have no `diagnostics` array. The block test supplies an empty one and says so. Desktop-created models always carry the array.
3. **The toolkit "create material" writes an E/G material, also on 0.4.0.** The engine accepts it. Whether a 0.4.0 case can use it is left to the product's solve-time diagnostics, as ruling §5 Q4 says. The e2e uses an explicit-base-properties selection, which the operation only resolves by reference.
4. **Not editable in the fields** (retained as authored): locked-support components, `base_motion`, `device_reference`, `analysis_basis_override`, `mass_state_ref`, and predecessor position sources. The brief's required groups are covered.
5. **Open.** Native behaviour and the owner's-Mac witnesses 2–8 remain outstanding. No e2e shows a 0.4.0 solve.
6. **WP2 defects.** I found none in WP2 code.

## 6. Symlinks and scratch

- **Symlinks I created in the worktree** (both still present at return; see below):
  - `node_modules` → `<SIBLING_WORKTREE>/projects/chirality-piping/node_modules`
  - `apps/desktop/node_modules` → `<SIBLING_WORKTREE>/projects/chirality-piping/apps/desktop/node_modules`
- **Symlinks in my scratch copy:** the same two links. They were deleted with the scratch copy.
- **Scratch.** The scratch copy (base plus my change, with its `dist/`, Playwright output and logs) is deleted. Disk use stayed above 8 GB free.
- **The worktree links.** The manager asked me not to remove the worktree links while the manager is using them. I tell the manager at return that both links are mine and still present, and ask the manager to remove them when finished, or to tell me to.

---

# Addendum A — repair loop after review (manager messages and ROOT points, 2026-09-26)

Scope: the manager asked me to fix §5 findings 1 and 2. ROOT added three points to that. Then, after the manager's option (a) decision, ROOT confirmed it with condition 1 as a hard gate and added a stored-state harm check. My write set did not change. I made no Git writes.

The container restarted in the middle of the loop. The worktree and my scratch copy both survived intact; I checked this with `diff -r` against the worktree. The Chromium run killed by the restart (exit 137) was run again, and so was the full vitest.

## A1. Files (sha256 before = HEAD `24e45a3e5`, which equals `583f5d9fa` outside `execution/`)

This table replaces §1 for every file listed in it. Files not listed here are unchanged from §1.

| File | sha256 before | sha256 after |
|---|---|---|
| `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` | `68b49ca96314cba671bf63be30142a0ef07ccf1cbf78604a1f1e46258a41110f` | `103e6bf815e51a1a4616cecae32b50dae921279bf4728d9b32a9efdebb93a801` |
| `apps/desktop/src/features/solve/SolvePanel.tsx` | `73bba1ca4a29763655b2dd2e4f31a72f3f1e2450822f3bb29d7c686b397947f9` | `ed21ab8dfe747d3628b585fb6b6ca94a107dd56fa8192b27a0daab7b0f1aad3a` |
| `apps/desktop/src/features/load-cases/LoadReferenceStateInputs.test.tsx` | (new) | `ebd00fcfbd63a396438afeab00ddb24e3b7709e26c51f593726aa45fb86c8184` |
| `apps/desktop/src/features/load-cases/loadStateFixtures.test.tsx` | (new) | `2eeedf5b5d34138f54eef118d6c0d0033b464a86a30239ebf0f868c8ed2197ba` |
| `apps/desktop/src/features/solve/LoadReferenceStatesBlock.test.tsx` | (new) | `3a5c602e62f8f2623276add1ec98d790dee77de6c2daf9215e5404cf227aade6` |

Unchanged since §1:
- `App.tsx` `2960da41…738db`
- `MaterialTemperatureForm.tsx` `8867b10e…42634`
- `workspaceSession.ts` `b1e59fa4…204f8`
- `projectService.ts` `4b639ab7…41f40`
- `loadStateAuthoring.tsx`, `LoadReferenceStateInputs.tsx`, `ExpansionLawsEditor.tsx`, `LoadReferenceStatesBlock.tsx`, `LoadReferenceStatesBlock.joinedGuard.test.tsx`, `blankLoadStateModel.test.ts`, `e2e/load-state-inputs.spec.ts`

## A2. The two panel guards (findings 1 and 2)

**LoadCaseManagerPanel.** These are display-only changes. Nothing is written to the model.
- `loadCaseMetadataValue` returns `""` for an absent or non-text `label`, `kind` or `status`.
- The case row shows an absent label as the case id (`loadCaseLabelDisplay`, as ROOT allowed). It shows an absent kind or status as "not set", and the metadata heading shows `current=not set`.
- The delete intent's before-value mirrors the engine's `load_case_delete_display`, which reads an absent text field as `TBD`. The helper is `engineMetadataText`, used for the comparison string only.
  - Without this, deleting such a case was refused with `OP-STALE-BEFORE-VALUE`. Probe: `case:hot; undefined; undefined; undefined; primitives=2` against `case:hot; TBD; TBD; TBD; primitives=2`.
- A metadata edit on an absent field is refused by the engine (`OP-FIELD-NOT-PRESENT`). That is unchanged: the panel adds no key.

**SolvePanel.** An absent `model.diagnostics` counts as none, at both reads (the readiness summary and the job packet). The key is never added.

**Tests.** `loadStateFixtures.test.tsx` covers the following. The earlier test workarounds are removed: the invented case metadata in `LoadReferenceStateInputs.test.tsx`, and the supplied `diagnostics` in `LoadReferenceStatesBlock.test.tsx`. Both now render the committed models unmodified.
- The committed `load_reference/connected`, `load_reference/pressure` and `load_reference_source/eigen_motion` request models, unmodified, are rendered in LoadCaseManagerPanel and SolvePanel for every case.
  - Absent metadata shows as the id or "not set".
  - After rendering, the document bytes are unchanged, and the presence of `diagnostics`, `label`, `kind` and `status` is exactly as committed.
- The fields show the authored values (reference members, fit, points, laws, motions, sources and factors) of connected and eigen_motion, and the expansion laws of pressure.
- SolvePanel shows the resolved-state block on the unmodified connected model, which has no `diagnostics`.
- The delete intent for a case without metadata matches the engine's before-value.

**Mutants.** They were run on the scratch copy, and each guard was reverted separately. Results are in `_run_records/mutants.log` and `mutation_results.json`.

| Mutant | Result |
|---|---|
| A1: label/kind/status guard reverted (`return loadCase[field]`) | killed (4 tests) |
| A2: `model.diagnostics` guard reverted in the readiness summary | killed (11) |
| A3: `model.diagnostics` guard reverted in the job packet | killed (11) |
| A4: delete before-value no longer mirrors the engine's `TBD` | killed (1) |

The original 14 mutants (G1–G3, L1–L5, S1–S6) were re-run on the rebased copy, and all were killed. **Total: 18 of 18 killed.**

## A3. Committed 0.4.0 fixture opened at session level (ROOT point 2, option (a))

**Route: vitest integration, not e2e.** The browser lane has no file import. Its only open route is the in-memory browser snapshot (`createLocalProject`, then `openLocalProject`), and a Playwright page cannot put a committed file into that snapshot without a test hook.

So the test uses the same real route inside the workspace session:
1. `createLocalProject(committed model)`
2. the session's own `handleOpenProject()`
3. `LoadCaseManagerPanel` and `SolvePanel` mounted on the session model.

**eigen_motion (desktop-shaped).**
- It opens byte-exact, and the 0.4.0 inputs show the authored values.
- An unrelated edit through the fields (the analysis-state provenance) is queued, then applied with `handleApplyIntent`, which changes the model.
- `handleUndoSessionModelEdit` then restores the document byte-for-byte (`JSON.stringify` equality).

**connected and pressure** are proven at panel level (A2). Why they are not opened in the session is explained in A5.

## A4. Pass-through list (ROOT point 3)

Proven on eigen_motion at session level. Each key below was added with an invented value to case `case:join`:
- the fields name each one as "Retained as authored (not edited here)";
- an edit elsewhere in the same record is applied (support motion 1.0 → 0.75 mm and a load-source factor of 2.5);
- each retained value is then identical in RFC 8785 canonical bytes, and Undo restores the whole document byte-for-byte.

The canonical comparison is needed because the wasm operation route returns objects with sorted keys (WP2 RETURN §6.3); values are unchanged.

| Authored 0.4.0 key the fields cannot edit | Where | Routing |
|---|---|---|
| `analysis_basis_override` | element state | UI-SUCCESSOR (editing) |
| `mass_state_ref` | element state | UI-SUCCESSOR (editing); the product refuses it (M21 seam) |
| `base_motion` | support state | UI-SUCCESSOR (editing); T5 (support semantics) |
| `device_reference` (`force_at_reference` / `unloaded_reference`) | support state | UI-SUCCESSOR (editing); T5 (support semantics) |
| `participation.locked_equivalent_support.components` | support state | UI-SUCCESSOR (editing); T5 (support semantics) |
| `position_source` `predecessor_value` (and `entered`) inside locked components | support state | UI-SUCCESSOR (editing); T5 (support semantics) |

- I found no other authored 0.4.0 key without a field. The check was a walk of the closed DTOs in `core/product_physics/src/case_state/input.rs`.
- `label` on a reference configuration, `operating_temperature`, every thermal and selection variant, `boundary_motion`, all four law definitions and both data kinds do have fields.

## A5. Product-shaped documents: base characterization, harm check, routing

**Base characterization (hard gate): passed. The crash predates T1.** The scratch was a clean `git archive` of `583f5d9fa`, before my files were overlaid. Record: `_run_records/addendum_base_characterization.log`.

| Model | Route at base | Result |
|---|---|---|
| `core/product_physics/tests/fixtures/exact_pressure_connected_request.json` (0.3.0, no `diagnostics` or `data_boundary`) | native open route: `open_local_project` returns it, then the session's `handleOpenProject` | **`TypeError: Cannot read properties of undefined (reading 'forEach')` at `buildModelIndex (src/features/workspace/modelIndex.ts:259:21)`**. This is the same read and line that connected and pressure hit with my change. |
| same 0.3.0 model | `buildModelIndex` directly | same error, `modelIndex.ts:259:21` |
| same 0.3.0 model | browser `createLocalProject` | refused before the session: `newer_than_supported` (the unchanged browser 0.3.0 divergence, §14) |
| `fixtures/product_preview/invented_dec092_temperature_g_request.json` (0.1.0, product-shaped, also has no `components`) | browser open route, then the session | `items is not iterable` at `duplicateIds (modelIndex.ts:360)`: an earlier unguarded read in the same index |

**Stored-state harm check: the crash does not write to or corrupt persisted state; it only fails to render.**

The walk of the open route:
- `createLocalProject` writes the snapshot once. That is the test's setup, before the open.
- `handleOpenProject` runs:
  1. `openLocalProject`, a read: in the browser it clones the snapshot; natively it is the `open_local_project` read.
  2. `buildHistoricalRunContext`, a read.
  3. Then only session state setters: `commitModel`, selection, undo/redo, queues, results, summary and messages.
  4. Then `computeModelHash` and `computeProjectEnvelopeHash`, which are pure.
- The throw happens during React's render of the new model: `useModelSessionState`, then `modelIndexFor`, then `buildModelIndex:259`.
- The only persistence writes in the session are:
  - `createLocalProject`, in `handleCreateProject` and `createBlankProjectFrom`;
  - `saveLocalProject`, in `handleSaveProject`;
  - `saveReportPackage`, in `handleSaveReportPackage`.
- Each of these runs only from a user command. None runs on open, before the throw or after it. There is no autosave.
- No ledger is appended and no operation is applied on open: undo, redo and queues are only cleared in session memory.

The test is `loadStateFixtures.test.tsx`, "product-shaped documents: the workspace crash writes no stored state", run on connected and on pressure:
- it opens the model through that route and catches the `TypeError` at `modelIndex.ts:259`;
- afterwards the stored envelope is byte-identical: model bytes, `mechanics_result` null, `analysis_run` null, `editor_intents`, and `model_migration_ledger`;
- the local project listing, including its timestamps, is identical;
- the source model is unchanged.

**Routing line.** Product-shaped model documents (no `data_boundary`/`diagnostics`/`sections`/labels) crash the workspace for every version → **UI-SUCCESSOR (desktop import tolerance)**.

Unguarded reads found (`apps/desktop/src`, excluding tests; none is in T1's write set):
- `model.diagnostics`:
  - `features/workspace/modelIndex.ts:259` (the crash site)
  - `App.tsx:1541`
  - `features/native-package/NativePackagePanel.tsx:208`
  - `features/workspace/solveJobAudit.ts:85` (`model?.diagnostics.length`), `:123`, `:361`
  - `features/design-workspace/DesignWorkspacePanel.tsx:196`
  - `features/diagnostics/DiagnosticsPanel.tsx:23`
- `model.data_boundary.*`:
  - `App.tsx:1562`
  - `features/review-geometry/ReviewGeometryPanel.tsx:269`
  - `features/caepipe-external/CaepipeExternalHarnessPanel.tsx:264`
  - `features/report-lint/ReportLintPanel.tsx:839`
  - `features/caepipe-mbf/CaepipeMbfExportPanel.tsx:270`
  - `features/pcf-export/PcfExportPanel.tsx:320`
  - `features/export-adapter-sdk/ExportAdapterSdkPanel.tsx:303`
- Collections assumed present: `features/workspace/modelIndex.ts:147–150` (`duplicateIds(model.nodes | pipe_segments | supports | components)`); the `:360` crash for a model without `components`.
- Labels are read as text in `ModelTree`, `PropertyInspector`, `PipeViewport`, `routeDraft.ts`, `componentIntent.ts`, `hangerSelection.ts`, `renderableReportInput.ts` and `SelfWeightPlanPanel`. These were found by a `\.label\.trim|\.kind\.trim|\.status\.trim|loadCase\.label` search and not probed individually. The two T1 panels are fixed.

**Witness 2 wording.** "Open a headlessly authored, desktop-shaped 0.4.0 model; inputs shown."

## A6. Checks on the rebased scratch copy

The scratch is a `git archive` of `583f5d9fa`, identical to HEAD `24e45a3e5` outside `execution/`, with this change overlaid. Records: `_run_records/addendum_checks.log`.

| Check | Result |
|---|---|
| Full desktop vitest, run A (quiet host, load 0.4) | 134 files, 2822 tests: 1 failed and 2821 passed. The failure was in `App.test.tsx`, "qualifies only a matching captured native unit replay…": `findByTestId("historical-run-context")` did not find the reference panel within its default wait (5.2 s test, not a timeout). This file and test were not changed by me. The route is physics-1 replay, where the resolved-state block renders nothing. Isolated reruns passed 3 of 3. |
| Full desktop vitest, run B (before the restart) | **134 files, 2822 tests, all passed** |
| Full desktop vitest, run C (after the restart) | **134 files, 2822 tests, all passed** |
| `npm run build --workspace apps/desktop` | passed; the only warning is the existing chunk-size warning |
| Chromium e2e: `load-state-inputs`, `linear-authoring`, `result-compatibility`, both projects | **6/6 passed**, on a rerun after the restart killed the first attempt |
| Mutants | 18 of 18 killed (A2) |

Run A's single failure is reported as observed. It did not recur in two full runs and three isolated runs, and I cannot attribute it to this change. It goes with the other `App.test.tsx` timing items to WP7's quiet-host sweep.

## A7. Symlinks and scratch

- The two worktree links (`node_modules` and `apps/desktop/node_modules`, pointing at `<SIBLING_WORKTREE>`) are removed at return.
  - The manager had asked me to leave them for integration.
  - ROOT's later restart instruction says to remove them, and I followed the later instruction.
  - The manager can recreate them with the same two `ln -s` commands.
- My scratch copy, with its own two links, is deleted at return.

## Manager integration note

- **Scope.** The six changed files match the §15 write set: one mount line or handler each, plus the builder in `projectService.ts`. `SolvePanel.tsx` keeps its `LoadReferenceOutputGate`. The new components contain no download, link or IPC output. No WP2 bytes changed.
- **Gates from ROOT and the manager.** The base characterization reproduced the crash on a product-shaped 0.3.0 model at `modelIndex.ts:259`, so T1 did not introduce it. The harm check shows the crash writes and corrupts nothing. The crash is therefore routed to UI-SUCCESSOR, as desktop import tolerance.
- **Manager checks in the worktree.**
  - WP3's test files, the load-case, solve and material folders, and the refusal test: 8 files, 119 tests passed.
  - `npm run build`: passed.
- **Open for WP7.** The full suite, e2e and timing run in the WP7 quiet-host sweep on the frozen candidate. That includes the one non-timeout `App.test` wait failure WP3 saw once, which passed 3 of 3 in isolation.
