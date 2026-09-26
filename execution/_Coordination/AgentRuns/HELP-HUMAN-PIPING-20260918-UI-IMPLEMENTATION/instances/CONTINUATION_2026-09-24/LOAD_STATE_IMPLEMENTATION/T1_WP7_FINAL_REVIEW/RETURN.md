# RETURN — T1_WP7_FINAL_REVIEW (fresh-context, non-author final review)

- **Role:** TASK (Type 2), non-author reviewer. I wrote none of the reviewed bytes and delegated nothing. I report to the T1 load-state WORKING_ITEMS manager.
- **Brief:** `TASK_BRIEFS/T1_WP7_FINAL_REVIEW.md`, read at the candidate with `_T1_COMMON.md` (including Wave 2), the root `AGENTS.md` and `agents/AGENT_TASK.md`.
- **Candidate:** `f3270ea79` on `codex/piping-load-states-20260925`. The review started on `6f42b9be528a52322fcd65b554f93e2e9033462e`; the candidate then moved to `f3270ea79` (see "Candidate move" below). The verdict is against `f3270ea79`.
  - Review diff: `git diff 82b43f9bd..f3270ea79 -- projects/chirality-piping` (1021 files).
  - The merge base `82b43f9bd` equals `git merge-base 6f42b9be5 origin/main`.
  - origin/main (`51f096e48`) is 2 commits past the base, outside the piping product.
- **Where I worked:** a scratch `git archive` of the candidate. I made no Git writes and changed no product bytes. The live worktree was not modified.
- **Paths:** relative to WORKING_ROOT (`projects/chirality-piping/`). `LSI` is the load-state implementation folder. Line numbers are at `6f42b9be5`. They are identical at `f3270ea79` for every file cited, because only one test file changed.

## Candidate move: `6f42b9be5` to `f3270ea79`

- **What changed:** `git diff 6f42b9be5 f3270ea79` touches one file, `core/reporting/result_export/tests/load_reference_source_contract.rs`. It has two hunks, both inside `source_text` and `request_text`.
- **What the change does:** the `joined_raw!` and `joined_request!` macros, which built paths with `concat!`, are removed. Each match arm now calls `include_str!` with a literal path.
- **Check:** I expanded the old macros and compared the result with the new literals by script. Both versions have 15 arms (10 raw, 5 request). The (arm key, fixture file) mapping is identical, and no other line changed. The test inputs and assertions are therefore unchanged.
- **Verdict on the change:** clean. It raises no finding.

## Verdict: **FINDINGS** (nothing BLOCKING)

I found no path that publishes a wrong number as Current, eligible or exported without a diagnostic. The findings are one SHOULD-FIX, which is a records obligation for Checkpoint 7, and NOTEs.

## Findings

| # | Severity | Location | Concrete failure scenario | Suggested repair |
|---|---|---|---|---|
| R1 | SHOULD-FIX (records, Checkpoint 7) | `LSI/T1_WAVE1_RULINGS.md` §7, §11–§14, §16, §17; `CHECKPOINT_6.md` (no T3/T5/T6/UI-SUCCESSOR list) | **What is required.** Ruling §12 says: "The next checkpoint lists this as routed open work naming T6." The brief (item 9) asks for complete T3/T4/T5/T6/UI-SUCCESSOR routing. **What exists.** `CHECKPOINT_6.md` predates §12 and names only T4. The other routings are scattered across the rulings, the WP2/WP3/WP4 returns and code comments. **The risk.** No consolidated routing record exists at the candidate, so an item can be dropped at closure. | Checkpoint 7 carries one routing table. Items found at the candidate: **T3:** joined (`load-reference-source-1`) numerical eligibility and reader re-derivation (§7); the headless joined binding/canonical-document route (§11). **T4:** the M07 joint-element refusal, which applies on the 0.4.0 route (`lib.rs:1534`, `_T1_COMMON` Wave 2). **T5:** support semantics for `base_motion`, `device_reference`, locked-equivalent components and predecessor position sources (§17, WP3 RETURN A4). **T6:** desktop output of both identities (every §12 surface); solver-mode custody on receipt-less identities (§13); the vacuous `build_result_export_document` pattern at `core/runner/headless/src/result_envelope_binding.rs:674` (§16 F1). **UI-SUCCESSOR:** the browser 0.3.0 divergence (§14); product-shaped document import tolerance, with the unguarded-read list in WP3 RETURN A5 (§17); editing of the pass-through fields (§17); layout and visual work (T1_PLAN). **ROOT:** the language gap where the Python stress-neutral packager accepts both identities (§12). |
| N1 | NOTE (records) | `LSI/T1_WP3_NATIVE_INPUTS/RETURN.md:132`; `LSI/T1_WP3_NATIVE_INPUTS/_run_records/e2e_checks.log:1` | Both files name a machine path (`PLAYWRIGHT_BROWSERS_PATH=<PW_BROWSERS>`). `_T1_COMMON` says to keep absolute machine paths out of every file written. No other T1-era record (`T1_*`, `CHECKPOINT_4-6`, rulings) has one. The machine paths in the CP1–CP3-era `_run_records` follow the repository's run-record convention (1697 run-record files on main carry them). The `<AGENT_HANDLE_PATH>/...` strings in the CP1-era briefs are agent handles, not paths. | Replace the value with a placeholder (`<PW_BROWSERS>`), as the same return already does for `<SCRATCH>`/`<REPO>`. |
| N2 | NOTE (records) | `LSI/CHECKPOINT_6.md` "Checks" table | The cargo, pytest and vitest counts (lib 331, result_export 91, headless 69, operation_applier 194, Python 1231, vitest 334) have no retained log in `_run_records/session5/`. Only `merge_regen_compare.log` (34/34, verified) and `merge_vp_static_rerun.txt` (507/507 both modes, verified) are retained. The claims cannot be checked against a record. | Checkpoint 7 retains the sweep logs, with `/proc/loadavg` as §16 F4 requires, so every count it states has a log. |
| N3 | NOTE (tests) | `apps/desktop/src/features/results/loadReferenceOutputRefusal.test.tsx:146,170-177` (manager's F2 repair) | **What was asked.** The F2 repair adds a proposal case, as the wave-2 reviewer requested. **What it does.** It uses the fixed fixture `invented_agent_proposal.json`, not a proposal derived from the result. The native `sample_agent_proposal` route (built from the result) is still proven only by code reading. **What it still proves.** Invariance under every AnalysisRun value, with a proposal present. That is what §14 needs. By reading, the proposal carries only an id. | Optional: add one case whose proposal names a result row or diagnostic id taken from the fixture raw, and perturb the raw. |
| N4 | NOTE (UX) | `apps/desktop/src/features/load-cases/loadStateAuthoring.tsx:41` (`toPayload` quantity heuristic) | A partly filled `boundary_motion` row, with only its displacement entered, is `{value:{value,unit}}`. Every key is `value`, so `toPayload` takes the row as a quantity. It reports "… value must be a finite number." instead of letting the engine name the missing `dof`/`meaning`. Nothing is sent and nothing is defaulted, so this is only a misleading message. `Number()` also accepts `0x`/`0b`/`0o` numerals, as the existing `parseQuantities` does. | Optional: treat an object as a quantity only when it has a `unit` key, or a scalar `value`. |
| N5 | NOTE (tests) | `tests/test_load_reference_headless_artifacts.py:47-51` | The file skips unless `HEADLESS_LOAD_REFERENCE_OUTPUT_DIR`/`…_SOURCE_OUTPUT_DIR` are set. An ordinary pytest sweep therefore runs none of its 15 tests. WP4 recorded them with the variables set (407 passed, 0 skipped). | Checkpoint 7 states whether the sweep set the variables, or cites the WP4 run. |

## Per item: what I verified and how

### 1. Product semantics (0.4.0 route, `core/product_physics`)

I read the whole `lib.rs` diff and `case_state/resolve.rs`, the thermal composition in `case_state/thermal.rs:696-743`, and the boundary preparation. These are the silent-wrong checks.

**Resolved states.**
- `resolve_case` returns `None` unless every pipe resolves, and the caller blocks (`lib.rs:1538-1551`; `resolve.rs:1090-1095`).
- Missing references, duplicates, request materials, explicit nulls, a case-wide modulus basis, legacy thermal primitives in the ledger, mass states, inactive or locked participation, base motion and device reference are all blocking (`resolve.rs:66-405`).
- A 0.4.0 case never consumes the case-wide base pair (`lib.rs:1526-1532`).
- Each case rebuilds stiffness from its own member pairs (`lib.rs:1648-1683`).
- A member with no pair, or a nonzero eigenstrain with no section, is blocking and is never dropped (`lib.rs:5093-5100`, `2202-2226`).

**Support motion.**
- Every restrained DOF is prescribed: zero unless the resolved case supplies a value (`lib.rs:2293-2306`).
- A motion on a DOF that is not a rigid restrained DOF is refused, in the resolver (`resolve.rs:1027-1039`) and again at solve (`lib.rs:2307-2331`).
- Two rigid supports on one DOF are refused by `RepeatedRestrainedDof` (`core/solver/linear_supports/src/lib.rs:641-647`), so no overwrite in `prescribed.insert` can go silent.
- The checked solve receives the nonzero prescribed map (`lib.rs:3965`). The frame kernel audits the coupling and prescribed compatibility (`core/solver/frame_kernel/src/structural.rs:271,481-492,705`).
- The complete `u` includes the prescribed values (`lib.rs:2513-2516`).
- Reactions are `K u − f` on the unreduced system (`lib.rs:2693-2702`).
- The legacy DEC050/053 observation lanes consume `K_ff u_f = f_f − K_fc g_c` (`lib.rs:2342-2358`, `2527-2533`, `3968-3976`).
- A 0.4.0 document bypasses the pre-0.4 `imposed_displacement` mapping. A support-targeted primitive cannot deserialize (`LoadTargetInput` has only node/element, `lib.rs:663-666`), so D3 holds.

**Eigenstrain enters once.**
- Thermal and fit compose as `f + t + f·t` (`thermal.rs:720-724`).
- The eigenload is `E_member·A·ε` from the resolved pair, and it replaces `build_thermal_element_loads`. It does not add to it (`lib.rs:2202-2227`, `2010-2029`).
- Thermal primitives cannot also enter: they are refused in the ledger (`resolve.rs:377-385`).
- Recovery removes the same `thermal_loads` once (`lib.rs:8721-8738`, the recovery input at `2376-2380`).
- The ledger applies only declared primitives, scaled by explicit nonzero finite factors (`resolve.rs:1063-1078`, `353-375`).
- On the exact route, `equivalent_static`, components, nonlinear or constant-effort supports and combinations are refused (`pressure_runtime.rs:162-199,224-230`), so no load or element outside the ledger enters.

**Material selection:** per-member evidence comes from the same pair that built stiffness (`lib.rs:3448-3454`, `2033-2042`).

**Joined route and receipt.**
- The identity becomes joined only when a case is source-selected on a 0.4.0 exact model (`lib.rs:1931-1932,1976-1981`).
- A finalized receipt is required; otherwise the typed entry returns `SOURCE_BLOCKS_FINALIZATION_FAILED` (`lib.rs:1424-1428`).

**SF-1 fallback** (`lib.rs:1432-1467`, `2388-2456`, `3495-3504`, `1990-1998`).
- Triggers: a selected case whose finalization fails (`3495-3503`), or an invocation receipt that fails (`1990-1997`). Either one records `load_state_join_failure`.
- The invocation is then republished from a clone of the request, with `load_state_join_withheld` set. The same ledger continues (`withholding_load_state_join`, `835-847`).
- During the rerun every successful attempt is declined with an info diagnostic, so no case is selected. The envelope is `load-reference-1` with no receipt, and the `SOURCE_BLOCKS_FINALIZATION_FAILED` check at `1424-1428` cannot fire.
- A budget-exhausted source attempt in the rerun is itself an info-level "unavailable", never blocking.
- No recursion: the fallback runs only when `withheld` is `None` at top level.
- Pre-0.4 is unchanged: `fallback_request` requires `is_load_state`.
- The fallback cannot publish a sensitive case as eligible: sensitive cases keep their ordinary non-`checks_passed` quality, and T0R standing withholds them.
- Covered by `source_receipt/load_state_fallback_tests.rs` (441 lines) and the CP4 backcheck. The reviewer mutants (below) confirm that the fallback, the prescribed write-back and the observation coupling are each test-pinned.

**Not found.** No path publishes a changed number without a blocking or info diagnostic.

### 2. Identity and standing

- **Every 0.4.0 envelope carries a T1 identity.** `mechanics_producer_for_model`, `formulation_basis_for_model` and `blocked_envelope` route every 0.4.0 envelope, blocked or solved, to `load-reference-1` or its profile (`lib.rs:954-970,11330-11345`). `solver_blocked` goes through `blocked_envelope` (`lib.rs:11385-11397`). Test: `case_state/tests.rs:337` (three blocked shapes, both modes).
- **Pre-0.4 envelopes never carry one.** Pre-0.4 documents that carry 0.4 keys block with `LOAD_STATE_CONTRACT_VERSION_MISMATCH` and keep their own identity (`resolve.rs:83-89`; ruling §11).
- **The fresh set is identical in all three languages:**
  - Rust `semantic_contract.rs` `FRESH_IDENTITIES`;
  - Python `compatibility.py` `FRESH_CONTRACT_IDS`;
  - TS `knownSemanticLimitations.ts` `FRESH_SEMANTIC_CONTRACT_IDS`.
  - Each holds the same six identities.
- **The standing order holds** (validation, then T0R's reason, then the joined early return):
  - Rust `numerical_use_standing_with_context`: `standing_reason`, then the LRS early return (`semantic_contract.rs` hunk at `+467`).
  - Python `numerical_use_standing`: `_source_contract` (unsupported on refusal), then the fresh/`_standing_reason` check, then the LRS early return.
  - TS: `load_reference_source` returns `eligible:false` with the reader's registered finding, so a refused envelope keeps its reader code (`numericalResultQuality.ts:95-102`, `loadReferenceSourceEvidence.ts:103-110`). TS has no "unsupported" status vocabulary, and that predates T1.
- **Joined results are never eligible** in any of the three languages, nor in the headless binding (`result_envelope_binding.rs:258-260`, asserted in the F1 test).

### 3. Readers in three languages

- I read the TS dispatch (`numericalResultQuality.ts`), the output-availability helper and the parity harness (`loadReferenceEvidence.parity.test.ts`). The harness runs the shared Rust/Python mutation corpora (`load_reference_mutations.json`, `load_reference_source_mutations.json`) and requires the same accept/refuse outcome in each language, plus Python's exact code. The one declared exception is the JS binary64 representation case.
- Python reader suites on my archive: `test_load_reference_readers.py` 239 passed, `test_load_reference_source_readers.py` 151 passed. The 2 skips are the env-gated outcome recorders.
- The wave-2 review's survivor analysis (8/8 reader, 10/24 ledger) is consistent with the code I read. Line anchors were spot-checked, not re-derived.
- After the sweep I ran the TS parity suite and the reader suites: 544/544 passed on `f3270ea79`.

### 4. Schemas and carriers

- `test_load_reference_schema.py` 527 passed and `test_load_reference_source_schema.py` 161 passed on my archive. They pin branch order (precision, physics, source-blocks, physics-source, preview, LR, LRS) by identity and require that old documents keep their outcome.
- The one non-append pointer change (ruling 1(3), AnalysisRun `contract_evidence` anyOf) is covered there.

### 5. Authoring

**Typed operations.** `operation_applier/src/load_state_authoring.rs` parses with the product's closed DTOs, checks only identity and references, and sets no default.

**Desktop inputs** (`loadStateAuthoring.tsx`, `LoadReferenceStateInputs.tsx`, `ExpansionLawsEditor.tsx`):
- Selects start at "Not provided", units start empty, and blanks become absent keys (`setMember`, `formSupport.ts:19-26`).
- `switchVariant` keeps only the new variant's keys.
- Nothing mutates the model.
- Every edit goes through `validateModelOperation` with the current `computeModelHash`, then `onQueueIntent` (`loadStateAuthoring.tsx:103-134`). Stale drafts are refused: epoch, model identity and queue identity are checked.

**Pre-0.4 gating:** `isLoadStateModel` equals 0.4.0 exactly (`loadStateAuthoring.tsx:21-28`).

**Pass-through:**
- `analysis_basis_override` and `mass_state_ref` (element), and `base_motion`, `device_reference` and locked components (support) are kept through `member()` edits and named as retained.
- A walk of `case_state/input.rs` found no other authored key without a field.

**Blank 0.4.0 model:**
- The existing blank builder is unchanged (`projectService.ts:482`).
- The new builder adds only `schema_version` and the exact pressure contract, and renames the project (`projectService.ts:536-551`).

**Undo/Redo byte exactness:** proven by WP3 (e2e hash and session-level `JSON.stringify` equality). I re-ran the session-level test (`loadStateFixtures.test.tsx`, in the 114/114 run); the e2e belongs to the manager's sweep.

### 6. Persistence and migration

- **Native:** `model_document_migration.rs` returns 0.4.0 as `current` with no migrated document. Its neighbours 0.4.1, 0.5.0, 0.3.1 and 1.0.0 stay refused. The native store round-trips model and result bytes exactly (the `src-tauri/src/lib.rs` test at the `+stored_json_column` hunk).
- **Browser:** `projectService.ts` `migrateModelDocumentLocal` mirrors the native behaviour.
- These are by reading only. src-tauri belongs to the manager's sweep.

### 7. Output refusals (§12, §14)

- I enumerated every `ControlledExportLink` user in `apps/desktop/src`. Every panel that takes a `MechanicsResult` or `AnalysisRunEnvelope` and emits a file is gated by `LoadReferenceOutputGate` or `refuseLoadReferenceOutput`:
  - SolvePanel, Handoff, HeadlessRunner, Report, RenderedReport (including `onRender`), RuleCheck, ResultExport (including `buildCurrentResultExport`, `deriveResultDocument` and `validateResultDocument`), StressNeutral (build, validate and table path), PCF, CAEPIPE MBF and external, Native package, Local FEA, Export review, Export adapter SDK, Adapter framework, External prover, Missing data, Design workspace, Report lint.
- The only other native output, `save_local_result_json`, is reached only through `ControlledExportLink`.
- The §14 group (c) panels read only identifiers:
  - review geometry: `result.run_id`, `run.run_id` and `run.model_state_ref` (`ReviewGeometryPanel.tsx:143-169`);
  - diff preview and review ledger: the run identifiers.
- The §14 reclassification is therefore sound.
- Since `8b49f38d0`, the Python validation path (`tools/validation/qualification_load_reference*.py`, `core/handoff/stress_neutral/package_v0_3.py`) has not changed (`git log`).

### 8. Tests

- **The manager's F1 repair (`5cf56e3de`, `load_reference_route_tests.rs:557-583`) is sound.** The forged `QualifiedPreviewEvidence` recomputes every digest, including the reparsed-receipt variant. The assertion requires `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`, which `build_result_export_document_with_evidence` can return only after every digest check (`result_envelope_binding.rs:242-260`). A digest slip would surface as `SOLVED_SOURCE_BINDING_MISMATCH` and fail the test, so the test cannot be vacuous.
- **The F3 repair is sound.** The origin passes the model, hash and producer gates, and `deriveResultDocument` reaches its own `refuseLoadReferenceOutput` (`resultExportAdapter.ts:51-58`).
- **The F2 repair is sound**, with the limit in N3.
- **The §14 reclassification (`64711fd94`)** only exports the three builders and removes their gates. Invariance tests: `loadReferenceOutputRefusal.test.tsx:157-178`.
- **No `.only`, skip or `#[ignore]` added, and no timeout raised**, in any product or test file of the diff. The only skips are env-gated recorders or artifact consumers (N5). The e2e keeps the 120 s budget (`playwright.config.ts:51`).
- **No reference, golden or oracle edits.** I found none other than the ones the rulings grant (§1(1), §6).

### 9. Records

- T1-era records have one machine path (N1).
- Checkpoint claims were spot-checked against logs (N2): the regen and VP-STATIC claims match.
- The open-work routing is scattered (R1). A consolidated list is proposed in R1.

## Rulings §11–§17 against their enforcement

| Ruling | Enforcement found |
|---|---|
| §11 | Enforced by the WP4 route tests: no joined document, exact unavailability text, receipt bound to the invocation, `needs_recompute` in both readers. The F1 repair adds the forged-proof refusal. Pre-0.4 identity controls per `resolve.rs:83-89`. |
| §12 | Enforced by the shared refusal (`loadReferenceOutputAvailability.ts`) on every result-data output surface (item 7), with the not-yet-available wording. The Python path is untouched. Routing to T6: see R1. |
| §13 | Pin retained and annotated. Routed to T6 (R1). |
| §14 | Group (c) enforced (item 7). Survivor acceptance is recorded in §16. The browser 0.3.0 item is routed. Timeouts belong to the manager's sweep. |
| §15 | C1: blank path byte-identical, no IPC change. C2: e2e route in `e2e/load-state-inputs.spec.ts`. C3: validate-only pre-check with the model hash, labels "integrity checked" / "needs recompute — not Current". The integrity-checked label requires ordinary route, WP2 eligibility and per-case `checks_passed` (`LoadReferenceStatesBlock.tsx:66-76`). Joined results can never be labelled integrity checked. |
| §16 | F1–F3 verified (item 8). F4 belongs to the manager's sweep. |
| §17 | Desktop-shaped scope; pass-through list verified against `input.rs`. Import tolerance and editing are routed (R1). |

## Checks run

| Check | Result |
|---|---|
| pytest, one file at a time, scratch archive (`_run_records/pytest_presweep.log`) | LR readers 239 passed, 1 skipped; LRS readers 151 passed, 1 skipped; LR schema 527 passed; LRS schema 161 passed |

| vitest, after "sweep finished", scratch archive of `f3270ea79` with the candidate-built wasm: WP3 and §14 files (`LoadReferenceStatesBlock`, the joined-guard test, `LoadReferenceStateInputs`, `loadStateFixtures`, `loadReferenceOutputRefusal`, `blankLoadStateModel`) | 6 files, **114/114 passed** |
| vitest: TS readers (parity over the shared Rust/Python corpora, readers, reader cases, joined ledger, `knownSemanticLimitations`) | 5 files, **544/544 passed** |
| cargo `result_export --test load_reference_source_contract` (the file changed at `f3270ea79`) | 5/5 passed |
| cargo `headless --lib load_reference` (includes the F1 forged-proof test) | 7/7 passed |
| cargo `product_physics --lib -- load_state case_state`; `--test load_reference_state_runtime` (baseline) | 60/60; 20/20 passed |

Logs: `_run_records/pytest_presweep.log`, `_run_records/postsweep_checks.log`, `_run_records/mutants.log`. Script: `_run_records/run_mutants.py`.

### Mutation evidence (reviewer mutants, `core/product_physics/src/lib.rs`)

Each mutant is one exact replacement. The source was restored and checked by sha256 afterwards (`4cc5ec22…`, equal to `git show f3270ea79:…/lib.rs`).

| Mutant | What it breaks | Result, with the killing tests |
|---|---|---|
| M-SF1-no-fallback | The SF-1 republication never triggers (`lib.rs:1451-1454`) | **killed** by 5 `load_state_fallback_tests`. Without the fallback the public entry returns `SOURCE_BLOCKS_FINALIZATION_FAILED` |
| M-PRESCRIBED-writeback-removed | The complete `u` omits the prescribed boundary values (`lib.rs:2513-2516`) | **killed** by 3: two `case_state::tests` prescribed-motion tests and `joined_rows_agree_with_the_ordinary_route_and_closed_forms` |
| M-OBSERVATION-uncoupled | The legacy observation lanes see `f_f` instead of `f_f − K_fc g_c` (`lib.rs:2344`) | **killed** by `dense_observation_lanes_observe_the_prescribed_boundary_system` |

3/3 killed.

## Limits

- **Full suites.** I ran no full suites and did not repeat the manager's sweeps (DEC-025, src-tauri, Chromium e2e, VP-STATIC), as the brief directs. The results the manager reports for `f3270ea79` are its own records; I did not re-verify them. Before "sweep finished" I used only code reading and single-file pytest runs; after it, targeted vitest files, crate-targeted cargo tests and 3 mutants.
- **Scope of the reading.** I read every product and test source change in the diff: the `lib.rs` diff in full; `case_state` resolve, input and thermal as far as they bear on silent-wrong exposure; the readers by structure and dispatch. For the per-check parity of the three reader implementations I rely on the parity harness run (544/544) and on the earlier reviews' mutant records, not on a line-by-line re-derivation. The 1021-file diff is mostly fixtures and records, which I sampled rather than read in full.
- **Native behaviour.** Not observed: no native app run, and owner's-Mac witnesses 2–8 are outstanding. Native persistence is reviewed by reading only.
- **Checkpoint 7.** It did not exist at the candidate. R1 and N2 are what it needs to carry.
- **Browser storage and the Undo/Redo byte witness.** Not re-run by me beyond the `loadStateFixtures` session-level test (which includes the Undo byte restore). The e2e is the manager's sweep.
- **Symlinks created and removed:** `node_modules` and `apps/desktop/node_modules` in my scratch archive, both pointing at the sibling engine worktree's `projects/chirality-piping/node_modules` and `apps/desktop/node_modules`. The first pair (on the `6f42b9be5` archive) was removed when that archive was replaced; the second pair (on the `f3270ea79` archive) was removed before return. No link was created in the live worktree.
- **Scratch.** The private cargo targets were deleted; free disk stayed above 8 GB (lowest 11 GB).
