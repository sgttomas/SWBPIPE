# Independent review: PR905 browser repair, records and graph checkpoint

- Role: Type 2 TASK, fresh-context non-author reviewer, dispatched by HELP_HUMAN (ROOT). No delegation. No Git write operations.
- Governing texts read: root `AGENTS.md`, `projects/chirality-piping/AGENTS.md`, and the TASK role text in the brief (sha256 `1a13a5b0…c8fb7`). No other role or workflow body was loaded.
- Reviewed diffs:
  - `5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295..8e216efbf2066a7d7a9e083dbcaadf07924b5a01`. This is the worktree HEAD and was checked out clean.
  - ROOT's scope addition `8e216efbf..347e214fe265b13e6d24f847f163821f0c833018`, read with `git show`/`git diff` only.
- Not re-reviewed, per the brief: the e650→5b1c lockfile repair and the record/preimage repairs. The LOCK_IMPACT_REVIEW record was checked only for record accuracy.
- Paths are relative to `projects/chirality-piping/` unless they start with `.github/` or a repository-root file name. Raw outputs are in [_run_records/](_run_records/). Machine paths appear only there.

## Verdict: FINDINGS (none blocking)

The product and test changes are correct and well targeted. They also cover the 347e214fe delta.
- Each new expectation matches the application source.
- The sourced-chip check is stronger than before.
- The CSS repair fixes the measured defect without changing the wide layout, DOM order or focus order.
- Eight mutation checks each fail at the new assertion they target. Committed bytes were restored and verified.

The findings are three should-fix items and several notes:
- Two should-fix items are in the work-graph checkpoint. They should be repaired before the final readiness review relies on it.
- One should-fix item is a remaining coverage gap. The repair's "spring-unit intent retained" wording overstates what is covered.

## Findings

### S1 (should-fix): the graph's native-input claim is false for the actual candidate

- Location: `execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md:344`: "later candidates may claim only 1,091 byte-identical inputs plus that out-of-closure lock". Also `:351`: "cite the lock review for the one changed lock".
- Evidence ([native-inputs-vs-candidates.jsonl](_run_records/native-inputs-vs-candidates.jsonl)). The 1,092 recorded inputs of the 8b982aa7 executable compare as follows:
  - 1,092 byte-identical at `e650`.
  - 1,091 at `5b1c`.
  - **1,086 at `8e216efbf`**: `report_package/Cargo.lock`, `apps/desktop/src/styles.css`, and four `apps/desktop/e2e/*` files differ.
  - **1,085 at `347e214fe`**: the dist spec also differs.
- Why the claim is wrong. The graph copies LOCK_IMPACT_REVIEW's wording for "5b1c or any later head". That wording was conditioned by its carry-forward 2, and this candidate breaks the condition. The same section's "Native reuse" bullet (`:350`) contradicts it.
- Failure scenario:
  - The final readiness review or the PR description repeats "1,091 of 1,092 byte-identical" for a head where 1,085 are. That is a false empirical claim.
  - Or `reconcile_coverage.py` fails on seven paths while the graph tells the reviewer to expect one.
- Repair. Restate the claim per revision:
  - `5b1c`: 1,091/1,092.
  - `8e216efbf`/`347e214fe`: 1,086 or 1,085/1,092.
  - Classify every path. The lock is outside the closure (LOCK_IMPACT_REVIEW). `styles.css` is inside the compiled closure, because it is bundled into `dist`, which Tauri embeds. The e2e specs are recorded inputs but are not bundled (`tauri.conf.json` `frontendDist: ../dist`, and no `src` import of `e2e`).
  - Make the readiness gate account for all of these paths, not "the one changed lock".

### S2 (should-fix): no native-reuse disposition or gate for the in-closure CSS change

- Location: `WORK_GRAPH.md:350-351`.
- Two records require a decision at this revision:
  - LOCK_IMPACT_REVIEW carry-forward 2: "If `styles.css` … changes in the next frozen revision, the compiled native closure changes. The native-reuse question must then be reassessed at that revision".
  - The session handoff: the impact check comes "before deciding what must repeat".
- What the graph does and does not do:
  - It records the facts correctly: the closure differs, the witnesses stay bound to 8b982aa7, the CSS is evidenced in Chromium only, and no WebKit or native re-witness is claimed.
  - It does not record a disposition, and "Remaining PR905 gates" lists no native item.
- Why this matters. The layout change reaches the packaged default window. `tauri.conf.json` and `playwright.config.ts` give the default as 1440x920.
  - At that size with both rails open, the reference row (368px) now wraps.
  - A Current Results row is 26px wider (about 394px). It sits 1.7px above the width at which the clear button drops alone to a second line ([band probe](_run_records/band-probe-committed.summary.txt)).
  - WebKit font metrics can put it on either side.
- Failure scenario. PR905 merges on the listed gates. The 8b982aa7 witnesses are then implicitly reused for a different executable, or no decision is recorded at all. Either way, the native Results filter layout is unobserved in the product host.
- Repair. Add a gate, or record an explicit ROOT disposition, choosing one of:
  - (a) Reuse the 8b982aa7 witnesses because the CSS change affects only `.result-filter-row` layout, which none of those witnesses observed. The native filter-row layout then remains an explicitly unwitnessed item for B6/UI.
  - (b) A native re-witness of Results filtering at 1440x920 with both rails open.

### S3 (should-fix): a Current result's unit composition in Solve and Design workspace is no longer tested anywhere

- Location: `apps/desktop/e2e/r2-smoke.spec.ts:681-684` and `:747-751`. These replaced the last `"N*m/rad,N/m"` checks on `solve-job-unit-policy` and `design-workspace-units`.
- Why the replacement was needed. Under PR905 the browser has no Current result, so the old checks could not hold. That part is correct.
- What the new checks cover:
  - The new `results=none` checks duplicate `src/App.test.tsx:1236` and `:3218`, which cover the empty path.
  - The "preservation" named in the return (the pure fixture oracle and the reference's `result-unit-policy`) covers the fixture bytes and `ResultsPanel`. It does not cover `SolvePanel` or `DesignWorkspacePanel`.
- What is now unchecked:
  - The non-empty path is `SolvePanel.tsx:300` (`result_units` from a Current result) and `DesignWorkspacePanel.tsx:297` / `:316-320`.
  - It had unit coverage on main at `App.test.tsx:10172` and `:11446-11471`. Earlier PR905 commits `0a438a686`/`22452ecd1` removed that coverage. This delta removes the last automated remnant.
  - `SolvePanel.test.tsx` has no unit assertions.
- Mutation m3 shows what the non-empty composition looks like: `results=MPa,N,N*m,N*m/rad,N/m,…`. Nothing now asserts it for a Current result.
- Failure scenario. A regression makes these two panels drop or convert spring units for a genuine Current result, for example by mapping `item.unit` through a display conversion. No test fails.
- Repair. Choose one:
  - (a) Add pure render tests in `App.test.tsx`, in the style of the existing `ReportPanel` pure render at `:9666`. Render `SolvePanel` and `DesignWorkspacePanel` with `result` set to the recorded source, and assert that `results=` contains `N*m/rad,N/m` and that `conversion=false`.
  - (b) Record the gap explicitly and stop describing the spring-unit intent as retained. The affected wording is in BROWSER_REPAIR `RETURN.md` §B and `WORK_GRAPH.md:347`.

### Notes

- **N1.** `apps/desktop/e2e/linear-authoring.spec.ts:248-249` says the saved-run reopen path, "HISTORICAL_INPUT_MANIFEST_MISSING and the recorded status", stays covered by `App.test.tsx` and `HistoricalRunContext.test.tsx`.
  - The manifest finding is covered there (`App.test.tsx:9808`, `:14504`; `HistoricalRunContext.test.tsx:130`, `:173`).
  - The recorded status (`HistoricalRunContext.tsx:345`, `mechanics=…`) is asserted only in `src/App.shell.test.tsx:847`.
  - Repair: cite that file too.
- **N2.** "12 real failures in five spec files" (`ROOT_DISCOVERY/RETURN.md:13`, `WORK_GRAPH.md:345`). The raw outcomes show four spec files: `r2-smoke` 4, `ui-foundation` 4, `gui-workflow-validation` 2, `linear-authoring` 2. A fifth appears only if the excluded ENOSPC artifact is counted. The same sentence says "four causes" and then lists three bullets. Repair: "four spec files", and match the cause count to the bullets.
- **N3.** `ROOT_DISCOVERY/RETURN.md:11` says the self-check gave "Exit 0". The retained log has no exit status. Its severities are INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 124, and none are in Piping (WARN is 81 `docs`, 43 `_DomainEngines`). The earlier shallow-clone COMMIT_NOT_FOUND run is not retained. Repair: record the exit code, or reword the claim to what the log shows.
- **N4.** `WORK_GRAPH.md:349` says the six dist failures are "under separate baseline diagnosis". At `8e216efbf` no linked record supports this. At `347e214fe`, DIST_LANE exists but the graph was not updated. The B6 row (`:56`) also carries neither the `.result-page-row` defect nor the focus-outline observation. Repair:
  - Link DIST_LANE and state the outcome: two failures repaired; four pre-existing on main and still open.
  - Add both observations to B6's row or its follow-up list.
- **N5.** Two issues in BROWSER_REPAIR `RETURN.md`:
  - Line 6 says paths are relative to `projects/chirality-piping/`. Many cited paths (`src/…`, `e2e/…`, `App.test.tsx:…`) are relative to `apps/desktop/`.
  - Its statement that the DEC-025 sweep runs the dist lane holds only for the host-mode sweep. The CI-bound sweep binds surface 4 to the hosted run and runs no dist lane (`tools/release/run_evidence_sweep.py:205-229`). DIST_LANE states this correctly.
- **N6.** The CSS has an intermediate wrap state. For filter-row widths of about 392–432px, the input and summary share line 1 and the clear button sits alone at the right of line 2. In the probe this is the reference row at 1500–1536px viewports with both rails open.
  - It works: the input is 161–187px wide (before the change it was about row width minus 280, so about 121–141px), there is no overflow, and Tab goes from the input to the clear button.
  - In the probe, typing a filter never changed the line structure. The summary narrows by 6.1px, so a toggle is possible only within about 6px of a threshold.
  - No repair required. It is relevant to S2.
- **N7.** The DIST_LANE raw logs carry no command, revision, timestamp or browser version. The RETURN's "same host/Chromium 1194" and the candidate identity rest on file names only. Repair: add a header or an `environment.txt`.
- **N8.** The load-state link at `WORK_GRAPH.md:352` points to the mutable branch. The text names `404cd9c6b`, which is the current branch head. Prefer an immutable commit link.

## Judgement per change

| Change | Judgement |
|---|---|
| `e2e/workspace-driver.ts` `expectStatusChip(…, source?)` | **Stronger.** The unsourced path is byte-identical in behaviour. The sourced path adds four checks: title `source: token`, `data-status-token`, the popover accessible name `face: source`, and the first popover span. Each matches `ShellStatusBar.tsx:96,103-108` and `shellLayout.ts:414`. Mutation m2 (source dropped) fails at the title. Mutation m2b (popover source span dropped) fails at `workspace-driver.ts:291`. Remaining callers are unsourced `MODEL_INCOMPLETE` chips (`chipFor`, `shellLayout.ts:373-378`, carries no source). |
| `gui-workflow-validation.spec.ts:103-104`, `linear-authoring.spec.ts:232-233` | Correct token (`failed`) and source. |
| `linear-authoring.spec.ts:244-252` | Correct. A refusal leaves `result` null, so nothing is saved as a run. The Results empty text (`ResultsPanel.tsx:179`) and `historical-run-context` count 0 would fail if a saved run were fabricated. Model hash and deformation checks are kept. Citation: see N1. |
| `r2-smoke.spec.ts:88-100` (pure spring oracle) | Correct against `invented_preview_model.json` C-150 `modifiers.*_stiffness_user_value` and SH-140 `hanger.stiffness.value`, for both sources. Mutation m6b (SH-140 converted to kN/m) fails at `:94`. |
| `r2-smoke.spec.ts:681-684` (`results=none`) | Matches `SolvePanel.tsx:300,353-356` and `App.tsx:894` (session `result`). See S3. |
| `r2-smoke.spec.ts:720-721` (`N*m/rad, N/m` in the reference's source units) | Correct against the `ResultsPanel.tsx:336-352` order and `, ` join. Mutation m6a (display conversion) fails at `:721`. |
| `r2-smoke.spec.ts:747-751` (design workspace) | Matches `DesignWorkspacePanel.tsx:297,316-320` and `App.tsx:631` (`currentSolvedResult`). Mutation m3 (reference leaks into this panel) fails at `:749`. See S3. |
| `r2-smoke.spec.ts:754-763` (report) | Matches `ReportPanel.tsx:96,267-270` and `App.tsx:691`. The pure redaction renders cited in the return are real (`App.test.tsx:9666-9669`, `:10906`). Mutation m4 (reference leaks into the report) fails at `:758` with "Raw report DOM suppressed … (832 redacted values)". |
| `r2-smoke.spec.ts:1751-1759` (R3 Issues) | Matches `MissingDataBlockingPanel.tsx:137-143` (model status when `result` is null), the fixture's `not_performed_user_rule_inputs_missing` and diagnostic `RULE_INPUTS_MISSING`, and `App.tsx:1037` (session `result`). It is more specific than the old single token. Mutation m5 (reference leaks into Issues) fails at `:1754` with `rule_check=RULE_INPUTS_INCOMPLETE; … mechanics_qualified=true`. |
| `src/styles.css:2595-2660` | See the CSS checks below. |
| `ui-foundation-dist.spec.ts:1135-1139` (347e214fe) | **Stronger.** The readout is inside the closed `<details className="command-context">` (`PipeViewport.tsx:2463-2480`, readout at `:2474`). There, `innerText` returned `""` while `toHaveText` compares `textContent`, so the old baseline could never pass. The new code first asserts that the readout names `Selected support: support:NL-140`, then captures the full `textContent`, which includes `0 queued`. The later comparison fails if selecting a reference row changes the current selection, because that row's entity is asserted not to be the target. `src` is unchanged in 8e216efbf..347e214fe. |

### CSS checks

- **Correct fix.** Mutation m1 (whole `styles.css` reverted to 5b1c) fails `ui-foundation.spec.ts:677` light comfortable 1280x800 with "center is covered by result-filter-summary".
- **Wide layout unchanged.** At one line, the label's flex-grow absorbs all free space, so the clear button's `margin-left:auto` resolves to 0. The 1024x768 before and after screenshots are byte-identical (both `e374d538…a6450539`, re-hashed).
- **`.result-page-row` unchanged.** The same four declarations (grid, the same columns, `align-items`, 8px gap) now sit in one rule, at the position of the old column rule. No other rule sets these properties; the only other matches are colour rules at `:2630-2636` and `:3575-3578`. The only consumers of either class are `ResultsPanel.tsx:264,290`.
- **Theme rules unchanged.** The rules at `:3575-3578` (`.app-shell .result-filter-row label span`, `> span`, `.result-page-row > span`) select the same unchanged DOM. The change touches layout properties only. Dark and light variants of `:677` pass.
- **No reordering.** There is no `order` and no reverse direction. The probe records DOM order `[input, clear]` and Tab from the input to `clear-result-filter` at every width.
- **No horizontal overflow.** Row `scrollWidth` equals `clientWidth` and the document has no x-overflow at all 12 probed widths from 1280 to 1920.

## Records and graph: what else was checked

- **BROWSER_REPAIR.**
  - Its five "after" hashes equal the committed blobs, and `repair.patch` equals the committed product diff apart from index lines.
  - Its runs agree with its raw logs:
    - run5: 4/4.
    - run6: 119/1; the one failure is compact `:38` at `ui-foundation-workflows.ts:822`.
    - run7: isolated pass.
    - run9 and run10: the same six dist failures with and without the base CSS; run10 kept the e2e edits, and the dist specs import none of the changed helpers.
    - run11: rebuild.
  - The geometry JSON matches its figures: 1280 input 18→192.1px, 1440 input 87.8→280.1px, row height 32→72px.
  - Every `file:line` it cites was opened and matches, apart from the N1 and N5 corrections.
- **ROOT_DISCOVERY.**
  - The 18-case repro (10 fail), the main baseline (18 pass), the full suite (466/20/13) and the cohort rerun all match their logs.
  - Checked against GitHub for run 36139859999 on `e65001ad`:
    - Accessibility and shard 1 succeeded.
    - Shards 2 and 3 were cancelled.
    - Shard 4 failed with exactly the five named cases.
    - The hosted Issues text shows `rule_check=not_performed_user_rule_inputs_missing`, which is what the new R3 assertion expects.
- **LOCK_IMPACT_REVIEW.** Checked for record accuracy only: links, and no machine paths outside `_run_records/`. Its carry-forward 2 is what S1 and S2 rest on.
- **DIST_LANE (347e214fe).**
  - The main log shows four `:637` focus-outline failures and 45 passes, including both contrast cases.
  - On main, the contrast test ran the in-browser fixture solve and did not read the selection readout.
  - The candidate log shows the two contrast cases passing.
  - The file hashes `888a63de…` → `aeea81e7…` match.
- **Links and paths.** Every relative link in the four RETURN files and in the new graph section resolves. The external handoff `94a1cbb16e…/SESSION_HANDOFF_2026-09-25/HANDOFF.md` exists on `codex/piping-pipes-20260924`, and the load-state branch head is `404cd9c6b`. No machine-absolute path appears in any changed file outside `_run_records/`; the root-prefixed agent handles in the graph (such as `native_pan_manager`) are pre-existing agent names on unchanged lines.
- **Graph content.**
  - The graph keeps "Original finding closure remains 4 of 38 (M04/M09/M24/M35)", the native-reuse limits and the pending gates.
  - The COR-LOADSTATE row and the load-state bullet are consistent with commits `5511596af` (CP1 review: no blocking finding, three should-fix), `3293c319a` and `404cd9c6b` (CP2 WIP; review pending; no finding closes).
  - The M10/M16/M29 rows still say that production remains open.
- **Hosted CI (context).** On `8e216efbf`, Piping Desktop E2E (36173380877), governance-harness, Harness Pre-merge and pec-tests all succeeded. On `347e214fe`, runs were in progress at review time.

## Commands and results (raw outputs in `_run_records/`)

Environment ([environment.txt](_run_records/environment.txt)):
- HEAD `8e216efbf`, clean tree.
- Node v24.21.0, `@playwright/test` 1.60.0, Chromium 1194 (141.0.7390.37; CI uses 1223).
- `CI=true`, `PLAYWRIGHT_WORKERS=1`, port 5174, load average 2.8–4.5.

| Record | Command (from `apps/desktop`) | Result |
|---|---|---|
| [r1-targeted-committed.log](_run_records/r1-targeted-committed.log) | `playwright test --project=chromium-desktop --project=chromium-compact --workers=1 --timeout=180000 e2e/gui-workflow-validation.spec.ts:108 e2e/linear-authoring.spec.ts:60 e2e/r2-smoke.spec.ts:66 e2e/r2-smoke.spec.ts:351 e2e/r2-smoke.spec.ts:1703 e2e/ui-foundation.spec.ts:677` | **22/22 pass** (10.3 min) |
| [tsc-e2e-committed.log](_run_records/tsc-e2e-committed.log) | `tsc -p tsconfig.e2e.review.json` over the four touched e2e files | rc=0 |
| [mutations-summary.log](_run_records/mutations-summary.log), `mut-*.log` | [mutations.sh](_run_records/mutations.sh) through [mutate.py](_run_records/mutate.py) (chromium-desktop); see the mutation table below | **8/8 caught** at the targeted assertion; every file restored to its HEAD blob; `git diff --quiet HEAD` clean |
| [band-probe-committed.jsonl](_run_records/band-probe-committed.jsonl), [summary](_run_records/band-probe-committed.summary.txt) | `node band_probe.cjs 900 1280 … 1920` against a dev server on 5174 (stopped afterwards) | Layout states as in N6; no overflow; Tab order unchanged |
| [native-inputs-vs-candidates.jsonl](_run_records/native-inputs-vs-candidates.jsonl) | `python3 native_inputs_vs_candidates.py e65001ad5 5b1ccd356 8e216efbf 347e214fe` from the repository root | 1,092 / 1,091 / 1,086 / 1,085 identical (S1) |

Mutations run by `mutations.sh`:

| Mutation | Change | Caught at |
|---|---|---|
| m1 | whole `styles.css` reverted to 5b1c | `ui-foundation.spec.ts:677` |
| m2 | chip source dropped | `gui-workflow-validation.spec.ts` title check |
| m2b | popover source span dropped | `workspace-driver.ts:291` |
| m3 | reference leaks into Design workspace | `r2-smoke.spec.ts:749` |
| m4 | reference leaks into Report | `r2-smoke.spec.ts:758` |
| m5 | reference leaks into Issues | `r2-smoke.spec.ts:1754` |
| m6a | Results unit display converted | `r2-smoke.spec.ts:721` |
| m6b | fixture spring row converted | `r2-smoke.spec.ts:94` |

Not run:
- the complete five-spec source suite or the dist lane (BROWSER_REPAIR run6/run9 and hosted CI cover them);
- vitest, since there is no `src` logic change;
- the 347e214fe spec, because the brief allowed only `git show`/`git diff` for that delta.

Nothing native was run.

Standard claim fence applies.
