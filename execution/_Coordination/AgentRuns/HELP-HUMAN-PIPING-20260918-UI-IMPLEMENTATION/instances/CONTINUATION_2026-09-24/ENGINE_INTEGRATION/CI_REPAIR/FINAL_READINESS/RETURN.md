# PR905 final readiness review

**Verdict: READY, conditional on a clean DEC-025 sweep at `6d9c0915f1c46563bc123fc7196109f835d6708a` bound to dispatch run 36190876950. See §5 for what the sweep must show, and S4 for the merging-revision condition.** No blocking finding. There are four should-fix items and seven notes. None of them weakens a protected check, and none requires a product or test byte change before merge.

Unless stated otherwise, paths are repository-relative. `CI_REPAIR/` means `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/CI_REPAIR/`. `_run_records/` means `CI_REPAIR/FINAL_READINESS/_run_records/`.

Reviewer: a fresh-context Type 2 TASK dispatched by HELP_HUMAN (ROOT) with the owner's approval. I implemented none of the reviewed changes and delegated nothing. I read `AGENTS.md`, `projects/chirality-piping/AGENTS.md` and `agents/AGENT_TASK.md` (the brief's sha256 `1a13a5b0…c8fb7`). I made no Git write, no browser run and no native run. My only writes are in this directory.

## 1. Candidate and review coverage

- **Head and base.** PR905 head `6d9c0915f` has base and live main `6dac281c6`, both confirmed through the GitHub API ([github-state.json](_run_records/github-state.json)). The PR is draft, and GitHub reports `mergeable_state: clean`.
- **Coverage chain for every mergeable change since `e65001ad5`.**
  - `e650`, 322 maintained paths: `readiness_precheck`.
  - `e650..5b1ccd356`, only the four leaf `Cargo.lock` files outside records: `LOCK_IMPACT_REVIEW`.
  - `5b1ccd356..347e214fe`: `INDEPENDENT_REVIEW`.
  - `347e214fe..6d9c0915f`: **this review**. It contains two non-merge commits, `50b9dd5af` and `203183724`, and two merges of origin/main.
  - I checked that the first-parent chain has no other commit.
- **Merges.** For each merge, `git merge-tree --write-tree <p1> <p2>` reproduces the committed tree exactly:
  - `ae5e333af` gives tree `afdebdfa…`;
  - `6d9c0915f` gives tree `818e3024…`.

  Neither merge changes any file on both sides of its merge base. So neither merge resolved a conflict or edited anything, and PR905's own delta since `347e214fe` is exactly the two commits.
- **Earlier independent-review findings.** All of them are closed in `50b9dd5af`, and I checked each against the bytes:
  - S1 and S2: the graph now states the native-input counts and the native-reuse decision.
  - S3: `CURRENT_UNIT_TESTS`.
  - N1: the `linear-authoring.spec.ts:248-249` comment now also cites `App.shell.test.tsx`. I confirmed that `App.test.tsx` and `HistoricalRunContext.test.tsx` carry `HISTORICAL_INPUT_MANIFEST_MISSING`.
  - N2 and N3: ROOT_DISCOVERY now says "four spec files and three causes" and qualifies the exit status.
  - N4: DIST_LANE is linked, and the two carries appear in the B6 row.
  - N7: `DIST_LANE/_run_records/commands.txt`.
  - N8: the link is now an immutable commit link.

## 2. Product/test delta review (`50b9dd5af`, `203183724`)

The standard: each change must preserve or strengthen what the original check protected, with nothing vacuous or skipped, and each new expectation must be correct against source. **All changes meet it.** The per-file judgements follow.

**`projects/chirality-piping/tests/test_stress_neutral_physics_source.py:183-186`.** Correct.
- The old code called `rehash_packet` outside `pytest.raises`. For an unknown method, `_csv_policy` raises `SN-CSV-PROFILE-MISMATCH` (`core/handoff/stress_neutral/package_v0_3.py:86-99`), so the old test crashed before it checked anything.
- The new version asserts two refusals: the materializer's specific refusal, and the validator's `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`.
- The validator's refusal comes from its first statement, `_transport_contract` (`:567`, `:41-53`), before any hash is compared. So the refusal is independent of hash consistency. That was the original intent of "relabel rejected even when re-hashed", and the new version preserves it.
- The schema assertion is unchanged.
- Verified: `-k method_namespace` gives 2 passed ([log](_run_records/pytest-stress-neutral-method-namespace.log)).

**`projects/chirality-piping/tests/test_headless_runner_contract.py`, with `CI_REPAIR/HEADLESS_RUNNER_TESTS/RETURN.md`.** Correct. The diagnosis in that RETURN (2 legacy-pressure failures plus 6 solver-identity failures) matches source:
- `wire.rs:313-318` and `:523-528` require exact name, version and build_ref equality with the linked component.
- `core/product_physics/Cargo.toml` is at `0.2.0`.
- The witnesses carry `0.1.0`.

The identity binding still protects the contract:
- `linked_solver_identity` reads the same `CARGO_PKG_NAME`/`VERSION` that `solver_component_name()`/`_version()` embed (`core/product_physics/src/lib.rs:11084-11092`).
- The every-verb solve test cross-checks it against the producer identity the binary actually emits.
- `bind_solver_identity` rewrites only the three identity fields. The other `0.1.0` strings in the witness are the rule-pack `version` and schema versions, which the runner does not compare.
- The new two-case control puts `0.1.0` on only the audit manifest, then on only the envelopes. It requires `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH` with the side-specific message, no package and no file. It also asserts that the linked version is not `0.1.0`, so it cannot become vacuous after the witnesses are regenerated.

The failure-case parametrization still reaches its named diagnostics. `MISSING` returns early. If the binding had erased the `BINDING_MISMATCH` or `PACKAGE_BLOCKED` difference, those tests would fail rather than pass.

The legacy-pressure refusal controls derive their expected `affected_refs` from the input model, not from output. The success paths use the unchanged re-authored `exact_pressure_authoring_model.json`, and the constants are independent oracles.

Verified: 6 focused tests passed, covering every-verb, both legacy controls, zip-exact and both identity-control cases ([log](_run_records/pytest-headless-focused.log)).

**`apps/desktop/src/features/workspace/currentResultUnitPolicy.test.tsx` (new).** Correct and non-vacuous. Current is reached only through `useWorkspaceSession().results.handleRun()`. The replay refuses any request other than the exact recorded one. The three recorded mutations fail it: the SolvePanel unit filter, the DesignWorkspace unit filter and the gate closed. The negative control is included. See S1 for the fixture's provenance.

**The five `VITEST_REGRESSIONS` files.** Each is correct.
- **`HandoffPanel.test.tsx` and `HeadlessRunnerPanel.test.tsx`.**
  - `precisionCarrier` is byte-for-byte the existing `precision()` protocol control in `result-export/resultExportAdapter.test.ts:26-37`.
  - A non-vacuity count was added: witnesses equal declared rows, and there is more than 0 of them.
  - A separate legacy-refusal test asserts `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED` and that the carrier is unchanged.
- **`RuleCheckRunPanel.test.tsx`.**
  - Desktop runs now use a basis that is registered through the production `runPreviewMechanics` route (the precision_connected_ui pair).
  - The assertions are stronger: the backend receives exactly that envelope and model, bindings are forwarded, and the first call is `run_rule_checks`.
  - A refusal control is added: a stub and a byte-identical clone both give `RULE_NATIVE_INVOCATION_REQUIRED`, with zero backend calls.
  - The old-pack test, which passed vacuously on HEAD, now proves that its request reached the backend.
- **`physicsSourceIntegration.test.tsx` and `sourceBlockRecovery.test.ts`.**
  - Both now use the permitted session boundary, and the import guard is untouched.
  - The direct-IPC registration premise is retained.
  - A session solve in the other mode receiving bytes recorded under the first mode fails with `SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED`. This is stronger than the old manifest-setter probe.
  - The session's own `handleRuleCheckAggregate` revision replaces the direct `setAnalysisRun`.
  - FE02's `-0` / `+0` split is kept at the session boundary.
  - The removed "result and manifest without an analysis record" check is a family-agnostic gate clause (`resultsSessionState.ts:56`, `analysisRun?.schema_version === "0.3.0"`). It is still covered at `resultsSessionState.test.ts:40-43`.

**ROOT's `resultsSessionState.test.ts:117-147`.** Correct.
- Flipping the manifest mode withdraws Current, and restoring it reinstates Current. This holds for both physics-source and source-blocks.
- This restores the protection the HEAD `badManifest` probe gave, and adds the source-blocks family that never had one.
- See N2 for the masking, and S2 for the evidence.
- Verified with the new unit-policy file: 2 files, 11 tests passed ([log](_run_records/vitest-focused-two-files.log)).

**The `linear-authoring.spec.ts` comment, the records and the graph.** They are accurate against the bytes, apart from S2 and S3.

## 3. Merge integration and D-GOV-45

- **No PR905 file lies in an archived path.**
  - The PR905 run folder `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION` is not archived. Its tree is untouched by the second merge, and no binary in it is archived.
  - None of the 3,502 files PR905 adds or modifies lies inside one of the 54 archived Piping run folders or matches one of the 229 archived files.
- **References to archived paths.** The only PR905 file that names an archived run is a historical provenance hash list: `…/SOLVER_MANAGER/NUMERICAL_INTEGRITY/INTEGRATION_REVIEW/SUCCESSOR/_run_records/AFFINE_BASIS.json`, which names `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/…`. Under D-GOV-45 §3 that resolves as preserved history, and no check depends on it.
- **Missing local paths predate the merge.** Maintained files still name some local record paths that do not exist, for example in `validation/qualification/capability_inventory.json`, `validation/qualification/fixtures/first_static/PROVENANCE.json` and `apps/desktop/src/test/nativeMechanicsReplay.ts`. They were already absent at `ae5e333af`, before the archive merge. They are provenance strings for records kept on other branches or relocated, not runtime reads, and hosted CI passed at head. The archive did not break them. The full pytest run in sweep surface 2 is the definitive runtime check.
- **Leak scan.** `tools/validation/validate_run_record_leaks.py --base 6dac281c6 --head 6d9c0915f` scanned 3,170 changed run-record files and found 0 possible credentials; exit 0 ([log](_run_records/leak-scan-pr-delta.log)). The reviewed delta alone gives 111 files, 0 findings and exit 0 ([log](_run_records/leak-scan-review-delta.log)).
- **Files over 5 MB: WARN only, non-blocking under D-GOV-45 §5.** All four come from earlier PR905 commits. The largest file in the reviewed delta is 1.5 MB. The four files, all under `…/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/` unless noted:
  - `CI_REPAIR/BROWSER_REPAIR/_run_records/run6-full-specs.json`, 5.1 MB
  - `FINAL_SUPPORT_REVIEW/CANDIDATE.patch`, 5.9 MB
  - `PHYSICS_READER_JOIN/HEADLESS_PRECISION/straight-full.document.json`, 6.1 MB
  - `…/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/N05_CONTAINMENT_01/HEADLESS_OUTPUTS/straight-full.document.json`, 5.9 MB
- **What main brought into Piping.** Only `tools/ci/e2e_plan.py` (the plan summary cap) and its test, plus notices. Neither overlaps PR905.

## 4. Readiness against the project `AGENTS.md`

**Independent review.** Every mergeable change is covered (§1). With this review, `347e214fe..6d9c0915f` is covered. There is one limit: my evidence for ROOT's claimed mutation result is source inspection only (S2).

**Hosted CI on the actual head. Verified through the API; I did not take it on ROOT's word.**
- PR run 36190875867 (`pull_request`) and dispatch run 36190876950 (`workflow_dispatch`, selection "Deliberate manual full integration milestone") are both `completed/success` on `6d9c0915f`, attempt 1.
- Across the 24 check runs on the head there are 0 failures. Shards 1–4, the numerical cargo suite, source-mode E2E, `harness`, `Harness pre-merge` and `pec` all succeeded.
- The skips are by design:
  - the accessibility barrier is skipped in full mode (`.github/workflows/piping-desktop-e2e.yml:78`);
  - the App and PEC jobs are skipped by path selection.
- **Surface 4 must bind 36190876950, not the PR run.** Under `projects/chirality-piping/docs/CI_STRATEGY.md` ("Exact coverage…"), PR-event full coverage omits compact duplicates and is not a DEC-093 surface-4 binding. That is adequate, provided the sweep's binding is exactly this run (§5).

**Native reuse.** The decision is sound and honestly bounded.
- The compiled and bundled difference from the witnessed `8b982aa7` executable is only `apps/desktop/src/styles.css`, 15+/7−. It is scoped to `.result-filter-row`; `.result-page-row` has an equivalent declaration split.
- The leaf `report_package/Cargo.lock` sits outside the src-tauri resolve, per LOCK_IMPACT_REVIEW.
- All other differing inputs are unit-test or e2e sources. The desktop build (`tsc -b && vite build`, with `noEmit`) type-checks them but emits and bundles nothing from them.
- The unwitnessed native WebKit layout of the wrapped row is explicitly carried to B6, and no native re-witness is claimed.

**Native-input accounting at the actual head.** This is the count the graph requires ([script](_run_records/native_input_reconcile.py), [log](_run_records/native-input-reconcile.log), [membership](_run_records/native-input-reconcile-membership.log)). The recorded set is the `git ls-files` roots from `JOINED_ENGINE_NATIVE/_run_records/build.py:9`.

At `6d9c0915f`, **1,079 of the 1,092 recorded inputs are byte-identical, 13 differ and 1 is added:**

| Class | Paths | Enters compiled/bundled closure? |
|---|---|---|
| Bundled stylesheet | `apps/desktop/src/styles.css` | yes (covered by the ROOT reuse decision and the B6 carry) |
| Leaf lock | `core/reporting/report_package/Cargo.lock` | no (outside the native resolve, per LOCK_IMPACT_REVIEW) |
| e2e sources (5) | `apps/desktop/e2e/{gui-workflow-validation,linear-authoring,r2-smoke,ui-foundation-dist}.spec.ts`, `workspace-driver.ts` | no |
| unit tests changed (6) | `HandoffPanel`, `HeadlessRunnerPanel`, `physicsSourceIntegration`, `sourceBlockRecovery`, `RuleCheckRunPanel`, `resultsSessionState` `*.test.*` | no (type-checked only) |
| unit test added (1) | `apps/desktop/src/features/workspace/currentResultUnitPolicy.test.tsx` | no (type-checked only) |

The graph's earlier per-revision counts reproduce exactly: 1,092 at `e650`, 1,091 at `5b1c`, 1,086 at `8e21` and 1,085 at `347e`. Between `347e214fe` and the head, only test files were added to the recorded roots. The native manifest was not edited.

**Overclaim check.**
- The graph keeps "4 of 38 (M04/M09/M24/M35)" and "no solver finding … newly closed". Its PR905 section makes no engineering-acceptance, lifecycle or release claim.
- The PR text says "makes no release or engineering-acceptance claim". It does not claim a closure count, and it lists the remaining work.
- There is no overclaim. See N4 for a limits-wording improvement.

## 5. What the DEC-025 sweep must show

1. The sweep's `commit_hash` equals `6d9c0915f1c46563bc123fc7196109f835d6708a`, and it runs on a clean checkout. This review's untracked directory lives in the ROOT engine worktree, not in the sweep worktree.
2. All five surfaces pass fail-fast, with no failed, skipped or unavailable surface:
   - **cargo crate sweep**;
   - **python pytest**: the full piping suite, including the repaired `test_stress_neutral_physics_source.py` and all 22 `test_headless_runner_contract.py` tests. The known `test_qualification_gate` `input_closed` race counts as a failure if it recurs. Record it and do not waive it;
   - **desktop vitest**: I expect **118 files and 2,059 tests** (the 2,057 run plus ROOT's two `resultsSessionState` cases, which that run predates), with 0 failed;
   - **desktop production build**;
   - **surface 4**, CI-bound.
3. Surface 4's `ci_binding` must pass `validate_ci_binding` (`projects/chirality-piping/tools/release/run_evidence_sweep.py:110-149`):
   - `workflow_path` is `.github/workflows/piping-desktop-e2e.yml`;
   - `run_id` is **36190876950** and `run_attempt` is 1;
   - `head_sha` equals the sweep commit;
   - `conclusion` is `success`;
   - `registered_e2e_specs_executed` is true;
   - `viewport_projects` is exactly the registered dual-viewport pair.
4. The raw summary and logs are retained next to the graph reference, as for PR901.

## 6. Findings

### Blocking

None.

### Should-fix (non-blocking; records or wording only, no product or test byte change required)

- **S1. The precision-1 fixture pair is a historical capture that the current product would refuse.**
  - Where: `apps/desktop/src/features/workspace/currentResultUnitPolicy.test.tsx:14-18`, `fixtures/product_preview/PRECISION_FIXTURES.md:3,16`, `fixtures/product_preview/precision_fixture_generation.json`.
  - What: the new test establishes Current from `invented_mechanics_result_precision_1_sparse.json`, described as "verbatim product stdout for the unchanged invented_preview_model.json".
  - Why that is out of date: the fixture was captured at `0a438a686`. The refusal `core/product_physics/src/pressure_runtime.rs:198-209` (`PRESSURE_MODEL_REAUTHOR_REQUIRED`) arrived later, at `22452ecd1`. The current product refuses a fresh solve of that model; the headless legacy control shows this. The generation record now differs from 13 of its 128 source inputs and does not list `pressure_runtime.rs` ([log](_run_records/precision-fixture-freshness.log)).
  - Scenario: a reader or a later regeneration treats the pair as currently producible. `PRECISION_FIXTURES.md` says "Source/code changes require regeneration", and doing that now would produce a refusal.
  - Why it does not block: the test weakens nothing. It is explicitly a unit transport replay and not native evidence, and it is the only recorded source with spring units (`N*m/rad`, `N/m`); the current connected pairs have none. The underlying staleness is also pre-existing PR905 content.
  - Repair: record in the graph (COR-RESULTS/DEL-14-02 follow-up) that the precision-1 pair predates the legacy-pressure refusal and is historical transport data only. In a later slice, add that sentence to the test header and `PRECISION_FIXTURES.md`, and supersede the generation record. Do not edit the fixture bytes.
- **S2. ROOT's mutation claim has no retained raw output.**
  - Where: `CI_REPAIR/ROOT_DISCOVERY/RETURN.md:39`.
  - What: the mode-control mutation result is stated without a log.
  - Evidence I have: I confirmed the claim by inspection only. `hasNativeMechanicsInvocation` compares `solverMode` (`apps/desktop/src/services/previewService.ts:135`). The clause alone is therefore masked, and removing the clause together with the mode argument (VITEST_REGRESSIONS `GAP.new`) defeats both mode checks.
  - Repair: retain the mutation log, or reword the claim as "by source inspection".
- **S3. The graph's vitest count predates the final candidate.**
  - Where: `WORK_GRAPH.md:356`.
  - What: the graph says "brings the full suite to 2057/2057". That run did not include ROOT's `resultsSessionState.test.ts` additions: the file's hash differs from the tested state, and the test file does not appear in the log. At `6d9c0915f`, no full desktop vitest has run outside the pending sweep.
  - Repair: the final reconcile cites the sweep's surface-3 count (expected 2,059) as the actual-head figure.
- **S4. The merging-revision condition.**
  - What: committing this RETURN, the sweep evidence or a graph or receipt update makes a new head after `6d9c0915f`. Required CI, review and the sweep's surface-4 binding must then be reconciled to the actual merging revision.
  - Repair, either of:
    - (a) Confirm that `git diff 6d9c0915f <new-head>` touches only `projects/chirality-piping/execution/**` record paths, with no maintained, test or native-input path. Obtain green hosted CI on the new head. State in the receipt why the `6d9c0915f` sweep still covers the unchanged product bytes, and cite the owner's merge policy for records-only follow-ons.
    - (b) Re-run the sweep with a new dispatch on the final head.

### Notes

- **N1.** Four run-record files over 5 MB WARN under D-GOV-45 and do not block (§3). Consider CI artifacts for similar evidence in future.
- **N2.** The source-blocks and physics-source mode clauses are individually masked by the mode check inside `hasNativeMechanicsInvocation`. The new controls protect the binding as a whole. That is exactly what the HEAD `badManifest` probe protected, so this is not a weakening. ROOT discloses it correctly.
- **N3.** `validation/witness/inputs/del1005_export_results_*_input.json` and their generator still carry `0.1.0`, and tests bind the identity at test time. After an authorized regeneration the binding becomes a no-op, and the explicit-`0.1.0` control stays valid. The `wire.rs` unit tests inject their own identity and do not exercise the real linked version.
- **N4.** The PR body's native claims are true for the `8b982aa7` executable. The final PR or receipt text should also carry these limits:
  - the native witnesses are bound to `8b982aa7`, reused under ROOT's decision;
  - the filter row is witnessed in Chromium only, with its WebKit layout carried to B6;
  - the four pre-existing dist focus-outline failures and `.result-page-row` go to B6;
  - the witness-input regeneration remains outstanding.
- **N5.** The legacy demo's richer topology (hangers, combination) is no longer on any CLI solve success path. This is disclosed in HEADLESS_RUNNER_TESTS §2. Re-authoring it is engineering authoring and is correctly out of scope.
- **N6.** The only integration surface where the D-GOV-45 archive could still matter at runtime is the full pytest and vitest suites at head, run by sweep surfaces 2 and 3. Hosted CI excludes `projects/*/execution/` from checkout, and it passed.
- **N7.** My focused vitest run may have written vitest's Git-ignored cache under `node_modules`. No tracked file changed; `git status` shows only this directory.

## 7. Commands (from `projects/chirality-piping` unless stated; outputs in `_run_records/`)

| Command | Result |
|---|---|
| `git merge-tree --write-tree <p1> <p2>` for both merges (repo root) | trees equal the committed trees |
| `python3 _run_records/native_input_reconcile.py <revs>` (repo root) | 1,079/1,092 identical and 1 added at head; earlier counts reproduced |
| `python3 tools/validation/validate_run_record_leaks.py --base 6dac281c6 --head 6d9c0915f` (repo root) | PASS, 4 WARN, exit 0 |
| `python3 _run_records/precision_fixture_freshness.py` | 13/128 recorded inputs differ |
| `CARGO_BUILD_JOBS=1 <dec025 venv python> -m pytest -q -p no:cacheprovider tests/test_stress_neutral_physics_source.py -k method_namespace -rA` | 2 passed |
| same interpreter (its path is in the raw logs), `tests/test_headless_runner_contract.py -k "unlinked_solver_identity or legacy_pressure or every_active_verb or zip_exact"` | 6 passed |
| `VITEST_MAX_WORKERS=1 npm test --workspace apps/desktop -- --run src/features/workspace/resultsSessionState.test.ts src/features/workspace/currentResultUnitPolicy.test.tsx` (Node v24.21.0) | 2 files, 11 tests passed |
| GitHub API: runs 36190875867 and 36190876950, PR 905, check runs, main | [github-state.json](_run_records/github-state.json) |

Compute was deliberately minimal while the concurrent sweep ran: no full suite, no build and no native run.
