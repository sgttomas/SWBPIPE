# BROWSER_REPAIR — TASK return to ROOT (HELP_HUMAN)

- Role: Type 2 TASK, dispatched by HELP_HUMAN (ROOT). No further delegation. No Git operations.
- Candidate: `5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295` (PR905 head `e65001ad` + record/lockfile repairs),
  plus the uncommitted working-tree repair described here (`_run_records/repair.patch`).
- Paths below are relative to `projects/chirality-piping/` unless they start with `.github/`.
- Governing texts read: root `AGENTS.md`, `projects/chirality-piping/AGENTS.md`, and the TASK role
  text supplied in the brief (sha256 `1a13a5b0…c8fb7`). No other role or workflow bodies were loaded.

## Result

| Check | Result |
|---|---|
| Acceptance 1: the 18 originally failing cases (gui-workflow-validation:107 ×2, r2-smoke R2 ×2, r2-smoke R3 ×2, ui-foundation:677 ×12 variants, desktop project) | **18/18 pass** (run 6, final bytes, 0 retries) |
| Acceptance 2: complete `ui-foundation`, `gui-workflow-validation`, `r2-smoke` + `linear-authoring` + `result-compatibility`, both projects (120 tests) | **119 pass, 1 fail** — compact `ui-foundation.spec.ts:38 [preflight]`, load-sensitive; **passes on isolated rerun** (run 7); not in the changed area |
| linear-authoring.spec.ts:60 (added to boundary by ROOT) | **2/2 pass** (runs 5 and 6) |
| Extra: dist lane `ui-foundation-dist` + `result-compatibility-dist` (50 tests, not in hosted CI; DEC-025 sweep lane) | 44 pass, 6 fail — **all 6 pre-existing**: identical failure with the base 5b1c `styles.css` (A/B control, run 10) |
| Ad hoc e2e type check of the touched specs (`tsc`, strict) | 0 errors; `npm run build` (`tsc -b && vite build`) exit 0 |

## Failure groups — root cause, repair, preservation

### A. Sourced status chip (`gui-workflow-validation.spec.ts:107` ×2; also `linear-authoring.spec.ts:60` ×2)

- Root cause. The chip for a failed job carries a source (`src/features/workspace/shellLayout.ts:414`,
  `source: "Solve job state"`). `ShellStatusBar.tsx:96` renders `title="Solve job state: failed"`;
  the popover (`ShellStatusBar.tsx:105-108`) renders the source span, `<code>failed</code>` and the domain.
  Unit contract: `App.shell.test.tsx:217`, `shellLayout.test.ts:387`. The helper
  `e2e/workspace-driver.ts` `expectStatusChip` checked `title === token` and `code === token`.
  gui-workflow-validation passed the whole title as the token (the code check failed). linear-authoring
  passed `failed` (the title check failed). ROOT's diagnosis is confirmed.
- Repair. `expectStatusChip(page, testId, token, face, source?)`. With `source`, it checks
  `title === "${source}: ${token}"`, `data-status-token === token`, `<code> === token`, the popover's
  accessible name `"${face}: ${source}"`, and its first span `=== source`. The no-source path is
  byte-for-byte the previous check. Both callers pass `("failed", "Solver · Not solved", "Solve job state")`.
- Preservation. This is a strengthening: every earlier assertion still holds, plus the source is
  pinned in the tooltip, the popover text and its name. Remaining callers (`MODEL_INCOMPLETE`:
  linear-authoring:68, :85; r2-smoke:956) are unsourced and unchanged. No e2e spec references
  `needs_recompute` or "Derived numerical qualification".
- linear-authoring second stale assertion (hidden behind line 232): after save/open, line 247
  expected a Historical saved run with `HISTORICAL_INPUT_MANIFEST_MISSING`/`MODEL_INCOMPLETE`.
  On main the browser Run produced a MODEL_INCOMPLETE result; under PR905 it produces none
  (`state=failed`, `result_rows=0`), so nothing is saved as a run. c3-viewport-visibility.spec.ts:301
  asserts the same `toHaveCount(0)` after save/open. The repair asserts the Results empty state
  (same text as gui-workflow-validation.spec.ts:307) and `historical-run-context` count 0. The model-hash
  and deformation checks are kept. The saved-run reopen path remains covered by
  `src/App.test.tsx:9808`, `:14504` and `src/features/results/HistoricalRunContext.test.tsx:130`, `:173`.
  Post-refusal `viewport-deformation-summary "not started; result rows=0"` is verified against
  `src/features/viewport/PipeViewport.tsx:4183-4187` (the `!result` branch).

### B. R2 browser smoke (`r2-smoke.spec.ts` R2 test, formerly :338, ×2) — three stale assertions, not one

1. `solve-job-unit-policy` `"N*m/rad,N/m"`. `SolvePanel.tsx:300` takes result units from the session
   `result` (App.tsx:894), which a refusal leaves null. `SolvePanel.tsx:353-356` then renders
   `results=none`. Repair: assert `results=none` (with `rows=0` and `conversion=false` kept).
2. `design-workspace-units` `"N*m/rad,N/m"` (**ROOT's diagnosis was wrong on this line**: it cannot be
   kept). `DesignWorkspacePanel.tsx:297,316-320` read `result`, which `App.tsx:631` wires to
   `currentSolvedResult`. The bundled reference never becomes Current, so the observed value is
   `results=none`. Repair: assert `model=angle=rad,force=N,length=m` and `results=none` (with
   `comparison=none` and `conversion=false` kept).
3. `report-redaction-blocked` "Raw report DOM suppressed…" and `report-export-link` href. `ReportPanel.tsx:96`
   assembles a packet only with a Current result (App.tsx:691). Otherwise `:267-270` renders the
   "Run the bounded preview mechanics path to assemble a report packet…" precondition and no export
   link. That is PR905's own unit contract, `App.test.tsx:9662-9664`. Repair: assert that text and
   `report-export-link` count 0 (`report-packet-body` count 0 kept).
- Preservation of the spring-unit intent. (a) New pure oracle in "recorded reference sources retain…":
  for both recorded sources, the rows with N*m/rad or N/m units are exactly the five C-150/SH-140 rows. Each
  equals its user-entered model value and unit, unconverted (`fixtures/product_preview/invented_preview_model.json`
  component C-150 `modifiers.*_stiffness_user_value`, SH-140 `hanger.stiffness.value`). (b) New live-UI
  witness where those units now legitimately appear: the inspected reference's
  `result-unit-policy` ("Source result units", `ResultsPanel.tsx:239`) must contain `N*m/rad, N/m`.
  Redaction suppression of an assembled packet stays covered by the pure `ReportPanel` renders at
  `App.test.tsx:9666-9669` and `:10906`. Report-lint counts (static public-surface inventory) pass unchanged.

### C. R3 guided flow (`r2-smoke.spec.ts` R3 test, formerly :1680, ×2)

- Root cause. `RULE_INPUTS_INCOMPLETE` was the old fixture result's `status.rule_check`
  (`fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json:58`). With no result,
  `MissingDataBlockingPanel.tsx:141` (fed the session `result`, App.tsx:1506) uses the model's
  `not_performed_user_rule_inputs_missing` (`invented_preview_model.json:35`). ROOT's diagnosis is confirmed.
- Repair. Assert the current model's rule standing in Issues: `missing-data-status-separation`
  contains `rule_check=${referenceModel.analysis_status.rule_check}`, `missing-data-summary`
  contains `rule_blocked=true`, `missing-data-warning-rule-check-required-inputs` contains `RULE_CHECK_BLOCKING`,
  and `diagnostic-RULE_INPUTS_MISSING` is visible. This mirrors gui-workflow-validation.spec.ts:156-167. The Analyze-page
  `RULE_INPUTS_INCOMPLETE` readiness assertion at the top of the test is unchanged.

### D. `ui-foundation.spec.ts:677` at 1280x800 (4 variants) — real layout defect, fixed in CSS

- Root cause (measured, `_run_records/geometry_probe/`). The bundled reference nests `ResultsPanel` in a second
  `section.panel` (`HistoricalRunContext.tsx:348`). With both rails open at 1280x800, the filter row is
  280px wide. The one-row grid `minmax(0,1fr) auto 32px` (old `styles.css` rule) gave the label 87.7px,
  and its inner grid `16px auto minmax(0,1fr)` gave the input track **0px**. The 18px input (padding and
  border only) overflowed under the nowrap summary, which owned its centre (`inputCenterOwner:
  result-filter-summary`). At 1440x920 the input was 87.8px.
  Screenshots: `controls-before-1280x800.png`.
- Repair (`src/styles.css`, now from line 2595). `.result-filter-row` is split from the shared grid rule and becomes
  `display:flex; flex-wrap:wrap; gap:8px`. The label gets `flex: 1 1 240px; min-width:0`, and the clear button gets
  `flex: 0 0 32px; margin-left:auto`. `.result-page-row` keeps its identical grid declarations. There is no DOM change,
  so DOM and focus order are unchanged.
- Measured after: 1280x800 input 192.1px, centre owned. 1440x920 input 280.1px. The row wraps to 72px
  height, with no horizontal overflow (scrollWidth equals clientWidth). The wide 1024x768 row (856px) keeps
  identical coordinates, and its controls screenshot is **byte-identical** before and after
  (sha256 `e374d538…a6450539`).
- `expectCenterUnobscured` and all geometry criteria are untouched.

## Other failures observed (full reporting)

1. `ui-foundation.spec.ts:38 [preflight]` compact, run 6: `viewport-measurement-readout` was empty.
   This was the first test of the fresh compact worker, with load average ≈6-7 on 4 CPUs and a concurrent full
   suite from another worktree. It passed on isolated rerun (run 7, 10.2s). It covers keyboard canvas measurement
   only (no Results page), so it is outside the changed area. ROOT's unrepaired full discovery also passed it.
2. Dist lane (`playwright.dist.config.ts`, 1280x900), 6 failures, **pre-existing**. Each fails identically with
   the base 5b1c `styles.css` (run 10). The spec imports only unchanged `workspace-driver` helpers.
   - `ui-foundation-dist.spec.ts:657` "production appearance … 1024x768" ×4 (light/dark × density):
     `ui-foundation-workflows.ts:352` focus outline width of `workspace-select` is 0 (expected ≥2).
   - `ui-foundation-dist.spec.ts:1128` "populated Results title … contrast" ×2: at :1166,
     `command-selection-readout` expected `""` (the `innerText` captured at :1135) but received
     "Selected support: support:NL-140; 0 queued".
   - Not diagnosed further (outside the brief). The Chromium difference (see Environment) is a possible factor for
     the focus-outline case. Hosted CI does not run this lane, but the DEC-025 evidence sweep does.

## Open issues for ROOT

- Pre-existing, unprotected sibling defect: `.result-page-row` in the same narrow column. At 1280x800 the page
  summary gets 10.2px and at 1440x920 98.2px (scrollWidth 364 vs 280 at 1280). The "Showing 1 to 50 of 830…"
  text sits hidden under the Previous button (`controls-*-1280x800.png`, `pagerow-before.json`). This is not
  repaired, to keep the brief's scope. The same wrap pattern would fix it, but it changes the 1440 layout and needs its
  own check run.
- The 6 dist-lane failures above block a clean DEC-025 sweep independently of this repair.
- Local Chromium is 1194 (141.0.7390.37). The pinned `@playwright/test` 1.60.0 expects chromium 1223
  (148.0.7778.96), as used by CI.
- This repair needs fresh-context independent review and CI on the committed revision (ROOT owns Git).
  Vitest was not run: there is no `src` logic change (CSS and e2e only). `tsc -b` ran inside `npm run build` and exited 0.

## Files changed (`git diff --stat`, base `5b1ccd356`)

```
 .../desktop/e2e/gui-workflow-validation.spec.ts    |  3 +-
 .../apps/desktop/e2e/linear-authoring.spec.ts      | 11 +++---
 .../apps/desktop/e2e/r2-smoke.spec.ts              | 41 +++++++++++++++++++---
 .../apps/desktop/e2e/workspace-driver.ts           | 19 ++++++++--
 .../chirality-piping/apps/desktop/src/styles.css   | 22 ++++++++----
 5 files changed, 77 insertions(+), 19 deletions(-)
```

| File | sha256 after | sha256 base |
|---|---|---|
| `apps/desktop/e2e/workspace-driver.ts` | `67648908d7871431478294c3bc44e7d3e010b714719a4aa6b90212639c05917e` | `49275148…` |
| `apps/desktop/e2e/gui-workflow-validation.spec.ts` | `0ddae8fa66a3ee685ecb06048aeeafda6ed78dce5e0ade5c40f504584605e73b` | `9915292d…` |
| `apps/desktop/e2e/r2-smoke.spec.ts` | `bf654b42c77cc80bfa9033ffbe47a6f1d2d4d3f80e5e7e0afe63da786ff4a7e0` | `408a839a…` |
| `apps/desktop/e2e/linear-authoring.spec.ts` | `768bcd279c65b9c787e6086dd85bfc1785593da48dc6470d35327542355cf8b6` | `77ebb93d…` |
| `apps/desktop/src/styles.css` | `ba1dc91c2b252c9522f11599e56e3f713e149f17b5a581ad83d81ee81cc0a60d` | `9565a390…` |

`linear-authoring.spec.ts` is outside the original boundary. It was first proven failing by run 3 and then added
to the boundary by ROOT. The full diff is `_run_records/repair.patch` (sha256 in `_run_records/file-hashes.txt`).
No other file was written except this evidence directory. `LOCK_IMPACT_REVIEW/` was not touched.

## Commands and runs (raw logs in `_run_records/`)

Environment (`environment.txt`): Node v24.21.0, `@playwright/test` 1.60.0, Chromium 1194,
`PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=<chromium-1194>`, `CI=true`, `PLAYWRIGHT_WORKERS=1`.
Source lane from `apps/desktop`:
`../../node_modules/.bin/playwright test --project=chromium-desktop --project=chromium-compact --workers=1 --timeout=180000 --reporter=list <specs>`.

| Log | Scope | Result |
|---|---|---|
| `run2-targeted.log` | the 18 cases after A, C, D and the first B edit | 16 pass; R2 ×2 fail at `design-workspace-units` (stale #2) |
| `run3-r2-linear-desktop.log` | R2 + linear-authoring, desktop | R2 fails at report block (stale #3); linear fails at :232 (Group A) |
| `run4-r2-linear-desktop.log` | same after B#3 and linear :232 | R2 pass; linear fails at :247 (Historical) |
| `run5-r2-linear-both.log` | R2 + linear, both projects | 4/4 pass |
| `run6-full-specs.log`, `.json`, `run6-summary.txt`, `run6-targeted-cases.txt` | 5 full spec files, both projects, final bytes | 119 pass / 1 fail (preflight compact) |
| `run7-preflight-compact-isolated.log` | `ui-foundation.spec.ts:38`, compact | pass |
| `run8-dist-build.log`, `run9-dist-specs.log` | `npm run build`; 2 dist specs | build ok; 44 pass / 6 fail |
| `run10-dist-ab-base-css.log` | A/B: base `styles.css`, rebuilt dist, the 6 dist failures | the same 6 fail (pre-existing) |
| `run11-dist-rebuild-repaired.log` | dist rebuilt with repaired CSS (restored sha verified) | ok |
| `geometry_probe/` | standalone Playwright probes (`measure.cjs`, `shot.cjs`, `pagerow.cjs`), JSON output and PNGs, before and after | see D |
| `tsc-e2e.txt`, `tsconfig.e2e.adhoc.json` | ad hoc strict type check of the touched e2e files | 0 errors |

ROOT's pre-repair reproduction (10 failures) is in ROOT's scratchpad `repro/run1.log` (outside the repo).
