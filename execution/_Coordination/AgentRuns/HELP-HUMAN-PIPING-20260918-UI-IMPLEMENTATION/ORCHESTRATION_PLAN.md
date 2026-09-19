# SWBPIPE interface implementation — Agent 0 plan

Status: the plan the owner approved on 2026-09-18 through the host's plan-approval step, reproduced from ROOT's plan file without change below this paragraph. `{P}` is `{WORKING_ROOT}`. Authority and the owner's words: `instances/ROOT/ACTIVATION_2026-09-18.md`. Departures from this plan are recorded in `HANDOFF_STATE.md` with their reason.

## Context

The design program is complete and merged (PRs #796 to #799): design system V1.3, UX specification V1.2 with a 313-row operations map, eighteen frames, and an implementation handoff. On 2026-09-18 the owner authorized this program to implement (not hand off), kept the piping loop on hold, and confirmed SCA-010 as narrowed. The owner also ruled, in planning:

- **Order:** visible rename (A1), then shell and canvas (B), then the identity layer (A2).
- **Hard identifiers kept:** `openpipestress_jcs_ijson_v1`, `openpipestress_result_semantics_v0_2`, the `.opsproj` extension and `openpipestress-projects.sqlite3` stay. Recorded as a named exception to DEC-101 (iv).
- **Root lint:** the owner directs the edit of `tools/validation/validate_claims_language.py` and its test in Tranche A1.

The product today: `apps/desktop/src/App.tsx` is one 4,367-line component with 68 `useState`, no store, one 3,904-line stylesheet with no colour tokens, no table component, and a 4,532-line `PipeViewport.tsx` with 41 hard-coded colours. 145 of the map's 313 rows need no gap; the first tranches stay inside those plus the interface and rendering gaps (G-16, G-27 to G-34).

`{P}` = `projects/chirality-piping`. Design basis: `{P}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` (its ten constraints and §6 verification bind every brief).

## Step 0 — open the implementation run (ROOT, no delegation)

- New run `{P}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/`: activation record with the owner's authorization and the three planning rulings verbatim and hashed (stored-transcript custody); `ORCHESTRATION_PLAN.md`; `WORK_GRAPH.json` in the piping shape (`base`, per node `role, model, effort, status, briefSha256, returnSha256, candidate`); `briefs/_INDEX.md`; `HANDOFF_STATE.md`.
- Close the design run's frontier with a pointer to the new run. Record the hard-identifier exception as a note against DEC-101 for the owner's next decision record (ROOT proposes the text; it is the owner's ruling already).
- Bind deliverables: PKG-07 `DEL-07-02` (tree and inspector), `DEL-07-09` (palette contract) and the viewport deliverable; each tranche appends `_run_records/` and `_STATUS.md` History lines.
- Probe once, cheaply: can a child spawned by the Agent tool itself spawn a child? The answer selects the delegation shape below. Probe that Playwright runs on this host (`npm run build:wasm:desktop`, one spec), since the DEC-025 sweep's surface 4 needs host capability.

## Delegation and model strategy

**Shape.** ROOT (Type 0, this session, Fable) holds the owner's rulings, seals every brief, integrates, and owns Git. Per project `AGENTS.md`, any fan-out is the multi-agent path: sealed brief per child, returns retained by hash, siblings never message each other.

- **Default: flat.** ROOT dispatches Type 2 children directly. It is permitted (`AGENTS.md`: HELP_HUMAN may dispatch bounded Type 2 work), it keeps one integrator, and it avoids a manager that re-derives context ROOT already holds.
- **Type 1 WORKING_ITEMS only for Tranche B's two lanes** (shell lane, canvas lane), and only if the probe shows nested spawning works. Each manager owns its lane's slicing, dispatches its own Type 2 implementers, and returns an integrated branch. If nesting does not work, ROOT runs both lanes flat, sequentially within a lane and in parallel across lanes using worktree isolation.
- **One writer per path at a time.** Lanes have disjoint write scopes: shell lane `apps/desktop/src/**` except `features/viewport/**`; canvas lane `features/viewport/**` and `features/workspace/modelIndex.ts`. `App.tsx` and `styles.css` belong to the shell lane; the canvas lane requests changes there through ROOT.

**Models by kind of work.**

| Work | Role | Model | Why |
|---|---|---|---|
| Orchestration, briefs, integration, owner-facing judgment | Type 0 | Fable | holds the rulings and design intent |
| Lane managers for Tranche B (if nesting works) | Type 1 WORKING_ITEMS | Fable | slicing a 4,000-line component is design judgment |
| Hard implementation: state extraction from `App.tsx`, the table component, persistence dual-accept, the canvas passes (halo, dimming, tokens, Budget labels), the root lint | Type 2 TASK | Fable | highest defect cost, touches result integrity and picking |
| Mechanical implementation: string replacement across ~150 files, PRD's 23 replacements, test-assertion moves, doc renames, fixture regeneration scripts | Type 2 TASK | Sonnet | bulk edits against an exact inventory; cheap to re-run; checked by lint, tests and review |
| Read-only inventories and searches | Explore | Haiku or Sonnet | locating, not judging |
| **Independent code review of 100 % of each frozen diff** | Type 2 TASK, `software-code-review`, read-only, fresh context | **Opus** | a different model from the implementer, so a shared blind spot is less likely; objective checking is what it did well in REVIEW-01 to 04 |
| Design-fidelity review (does the build match the specification and frames, by screenshot) | Type 2 TASK, read-only | Fable, fresh context, never the implementer | needs design judgment; briefed against the specification and the owner's rulings, never against ROOT's account |
| Benchmark runner for D-72 | Type 2 TASK | Sonnet | follows the README's literal commands; builds kept clear of timed runs |

Every return records the model that actually ran. A reviewer's PASS with no actionable finding is required before the sweep and before push; findings go back to the owning implementer by message, then the same reviewer backchecks.

## Tranche A1 — the visible rename and the rulings (one PR, ~150 files)

Executes DEC-100, DEC-101 (i to iii), DEC-102, DEC-103 item 9 (copy only), DEC-105, SCA-010. DEC-104 (the Checked mark) moves to Tranche B with the table it lives in. Needs a tranche manifest because the root lint is on the instruction surface.

Four children, disjoint scopes, then one integration:

1. **Lint and registry (Fable).** `tools/validation/validate_claims_language.py` (`REGISTERED_TEXTS` lines 59 to 86, the `MISSING_MATURITY_BANNER` anchor 103 to 107), `tools/validation/test_validate_claims_language.py`, `{P}/docs/claims_registry.md` (retire `BS-MATURITY`, retire `BS-ACCEPT` for product surfaces, `BS-IP` takes SWBPIPE, add DEC-102's eight-row display table to §2). Must land in the same commit as child 2's `App.tsx:3282` change.
2. **Product text (Fable for the constrained parts, Sonnet for the bulk).** Remove the maturity sentence (`App.tsx:3282`, `BuildReadinessPanel.tsx:75`, `ExportReviewPanel.tsx:658`); remove the acceptance sentence's 53 placements in 21 files and move the 15 test assertions; replace `readableWorkspaceStatus` (`App.tsx:3766`) and the two `formatStatus` copies with one label function reading DEC-102's table, token still reachable in place; move the seven pinned Vitest assertions. **Constrained:** `core/rules/rule_check_runner/src/lib.rs:78` `PROFESSIONAL_BOUNDARY_NOTICE` is a schema `const` (`schemas/rule_check_run_result.schema.json`) inside emitted documents. It is a data contract, not a product surface; leave it and record why, unless review disagrees.
3. **Name and packaging (Sonnet).** `tauri.conf.json` productName, identifier and title; `index.html`; the macOS menu title (`src-tauri/src/lib.rs:3724`); "Untitled … Project"; report titles; the ten `source_name` strings; `BuildReadinessPanel` literals; `tools/release/package_release_artifact.py`; the tests that pin them. New bundle identifier recorded; the Apple App ID stays the owner's act and blocks signed builds only. CAEPIPE removed from rendered copy and `docs/user_guide/index.md:65,273,274`; the compatibility flag no longer rendered; identifiers untouched until A2.
4. **SCA-010 (Sonnet).** `docs/PRD.md` 23 replacements with line 15 excepted, header per A004, `docs/report_notice_template.md` lines 16, 41, 68, `core/reporting/report_renderer/src/lib.rs:719` in the same commit, `_ScopeChange/_LATEST.md`.

Frozen and never touched: `execution/**` history, `validation/evidence/**`, `plans/**`, `docs/_history/**`, `docs/_ScopeChange/**`. `docs/validation_manual/cases/**` takes the name at its next amendment, not here.

## Tranche B — shell and canvas over today's operations (a series of PRs)

Each slice leaves the product working and the e2e specs green. Constraint 6 applies: a control whose gap is outside the tranche is absent or disabled with its reason.

**Shell lane**

- **B1 Tokens.** Import `cssFrom` from the design system's `tools/gen.mjs` into a build step that writes `apps/desktop/src/tokens.css`; do not copy the MOCKS output. Map the 26 existing `--ui-*` variables onto tokens; theme attribute stays `data-theme`.
- **B2 State extraction, no behaviour change.** Pull `AppSession`'s model, results, operations and chrome state into modules beside `features/workspace/selectionState.ts`, keeping every generation gate (`SolveRunGenerationGate`, `RuleRevisionGenerationGate`, the `stillCurrent` idiom) and `commitModelAfterSolveInvalidation` / `clearComputedModelState` byte-for-byte in behaviour. `App.test.tsx` (18,071 lines) is the safety net and must pass unchanged. Opus review focuses here.
- **B3 Shell.** Title bar, stage rail, Table / Model / Both per stage, status bar with chips from DEC-102, toolbar with Undo, Redo, Inspector toggle, units selector (G-28, G-33). Dock sections stay mounted so panel state survives, as today.
- **B4 Table component** on `VirtualList.tsx`: sort as view, filters, row expansion, footer with selection group and the "Apply" edit chip, gutter, keyboard model, copy and CSV. Every cell edit goes through `applyModelOperation`; add the operation-equivalence test (constraint 1). DEC-104's Checked mark lands here in interface state.
- **B5 Docked inspector and slide-over**, reusing `PropertyInspector.tsx` internals.
- **B6 Results header, run menu, Historical band; issues drawer with Filter; palette entity search** (single palette surface per DEC-094, `ToolkitPalette.tsx`). Stale standing is not drawn (G-11 is outside the tranche); add the presentation test that a Historical record gains no overlay, chip or readiness cue.
- **B7 Review page chrome** (three chips, comment stream filter row) over `ReportPanel.tsx`.

**Canvas lane** (foundation of constraint 5 retained; picking tests run first on every slice)

- **C1** Canvas colours from `canvas.*` tokens with live repaint in both themes (`viewportResource.ts:292` today repaints only background, gizmo and selection); parse the two `rgba` tokens.
- **C2** Selection as an overlay pass over the element's own colour, replacing `applyInstancedSelection`'s destructive recolour (G-31). Hover halos need per-move picking, which is new per-frame work: build behind a probe and keep only if D-72's orbit and point limits hold.
- **C3** Isolate as dimming with a stated pick rule and a total hidden count (G-30).
- **C4** Budget labels replacing the cap of 80 at `viewportSelection.ts:610`, control All / Budget / Off; replace the O(n²) overlap test if the probe at 138 and 230 plates needs it. The instrument pins 80 in `fixture-manifest.json` and `characterization-commands.ts:53`; bind a second frozen boundary profile (1440 × 900, canvases 603 × 828 and 1000 × 828), never edit tolerances or oracles.
- **C5** Real OD as the resting state, HUD as one group of ten with Fit first, fitted-camera state (G-32, session state until G-17).
- **C6** `canvas.edgeAlt`: feasibility probe only; fallback is one edge colour.
- **D-72 qualification** after the canvas lane: four gated runs plus six observations, by the Sonnet runner, builds kept clear of timed runs. The recorded demonstration is the comparison basis.

## Tranche A2 — the identity layer (after B; its own plan at that time)

Schema `$id` host to `swbpipe.com` (49 files), 65 document kinds, crate and binary names (39 `Cargo.toml`, ~225 referencing files), CAEPIPE-bearing identifiers, 122 fixtures of which 95 need regeneration by their producers, not by replacement. Dual-accept at the four read gates (`core/project_persistence/service.py:193`, `core/product_physics/src/lib.rs:843`, `core/product_preview/service.py:249`, `operation_applier/src/geometry_operations.rs:659`) with read-compatibility tests on the existing saved-document fixtures. The four hard identifiers stay. Touches `core/**`, so independent review and the full sweep are mandatory.

## Per-PR verification (every slice)

1. Registered checks by path rule (`{P}/software-workflow.json`): `npm run test:desktop`, `npm run build:desktop`; `piping-pytest` and `evidence-sweep` when `core/**` or `tests/**` change; `harness-self-check` always (it runs the claims lint as GEN-13).
2. Picking regression tests (`viewportSelection.test.ts`, the shared-endpoint fixture) before any canvas change is accepted.
3. Fresh read-only Opus `software-code-review` over the complete frozen diff: PASS with no actionable finding. For B and C slices, also the Fable design-fidelity review by screenshot against the named frames.
4. DEC-025 five-surface sweep (`python3 tools/release/run_evidence_sweep.py --execute`), Playwright surfaces on the host, bound to the clean candidate commit; `CLOSEOUT_CHECKS.json` with tested and reviewed SHAs.
5. Repository validators (path anchors, claims language, tranche manifest), CI `piping-desktop-e2e` on the final revision, then merge by merge commit on a `codex/swbpipe-…-<date>` branch under the standing Git authorization.
6. ROOT opens the built app and looks at it before reporting a slice as done.

## What returns to the owner

The Apple App ID; typed-interface and engine gaps per gap (G-11 Stale runs and G-17 persisted view state first); anything a reviewer finds that changes a ruling; the open copy items from the handoff's §7 when first seen in the running product. WCAG 2.2 AA is checked per touched control; PDU-045 and PDU-046 remain holds.

## Amendment 1 (2026-09-19) — the control layer first

**Authority.** The owner's direction of 2026-09-19, recorded with ROOT's recommendation that it refers to and ROOT's reading in `instances/ROOT/OWNER_DIRECTION_2026-09-19_CONTROL_LAYER_FIRST.md` (SHA-256 `bc41f82680cfcb69a193ee3820cb0861f36fbe872fae970df1be1729f9a23268`). The plan above stays as the record of what the owner approved on 2026-09-18; where this amendment and the plan differ, the amendment governs.

**What it changes.** Inside Tranche B: what a slice builds, which reviews gate it, where the design-system work and the token adoption sit, and when D-72 is qualified. It changes no ruling of the owner's (D-68, D-71, D-72, the decision rows), no constraint of the implementation handoff, no tolerance, oracle or limit, and not the order of tranches: A1 is merged, B continues, A2 follows B with its own plan.

**Why.** The handoff's constraints that protect results are all in the control layer: the one typed-operation route, the tables and the canvas as projections of the canonical model, run standing, the generation gates. After four merged pull requests one slice of that layer had landed, while the visual thread (tokens, the canvas colours, a design-system amendment in its third round of contrast findings) had run ahead of it. The open questions are in the appearance, and they are better answered once, on the product's real controls, as one package for the owner.

### The sequence from here

1. **Shell lane, control layer:** B2F (four handler repairs, failing test first), then B3, B4, B5, B6, B7, as the brief gives them. Each slice builds structure and behaviour on the tokens already in the product. **Layout follows the frames from the start**: regions, order, what is docked, which controls a surface has. Colour, contrast, washes and fine spacing are not tuned. Sealed as the lane's addendum 3.
2. **Canvas lane**, in parallel: **the first pull request finishes as sealed** (slice C1's first part with review findings F1 to F3, the guidance probe T1, the edge line C1E, the stylesheet rows C1b), because slices C2 to C5 stack on its material and palette code, the colours cannot reach `main` without the edge line, and D-72 measures orbit with the line on. Then C2, C3, C4, C5, which are mechanisms with behaviour. C6 and the canvas's appearance questions leave the lane's order. Sealed as the lane's addendum 4.
3. **No third stream.** ROOT runs no work of its own beside the two lanes except the independent reviews. Design system V1.4 is parked with its branch (ROOT's decision DS5-D3, open to the owner) and the token adoption slice waits with it; nothing in the control layer needs either.
4. **The closing visual pass**, after both lanes' control work:
   - the design system's open items, settled on the product's real controls and put to the owner as one package: correction 2 of V1.4 (sealed, not executed), the pressed-row finding, DS5-D1, DS5-D2, the text contrast target, and from the canvas the casing's look, the derived silhouette shades, the roles the design does not name, the deformed shape's ink with §6.8's deformation view, and `canvas.edgeAlt` (C6);
   - the design-system amendment reopened from its parked branch, with a fresh independent review;
   - the token adoption slice (`drafts/B1B-TOKENS13.DRAFT.md`, to be re-sealed) and the stylesheet rows that follow from it;
   - one design-fidelity review by screenshot over every surface against the eighteen frames (`drafts/FIDELITY-REVIEW.TEMPLATE.md`);
   - the contrast criteria of D-68 checked on every control the slices listed as touched.
5. **D-72 qualification**, last in Tranche B, because it must measure what ships. The second profile's freeze still goes to the owner as one package before any timed run.
6. **Tranche A2**, with its own plan. Its write scope (`core/**`, schemas, fixtures, crate names) is disjoint from the closing visual pass; whether the two may overlap in time is the owner's to say when A2 is planned.

### Gates per slice (replaces step 3 of "Per-PR verification")

- A fresh read-only Opus `software-code-review` over the complete frozen diff: PASS with no actionable finding. For a slice that draws a surface, ROOT's review brief carries a structural checklist drawn from the specification and the named frames, and the reviewer checks it from the code and the tests.
- The Fable design-fidelity review by screenshot runs twice and not per slice: on B3's candidate before it merges, for structure only, and at the closing visual pass over every surface.
- D-68 per slice: name, role, state, keyboard operation, focus order, a visible focus indicator, target size, for every control touched. The contrast criteria (1.4.3, 1.4.11) are checked at the closing visual pass; each slice return and pull request lists the controls it touched as **contrast not yet checked**.
- Unchanged: the registered checks, the picking tests first on every canvas change, the DEC-025 sweep on the clean candidate, the validators, CI on the final revision, a merge commit, and ROOT's own look at the built app before a slice is reported done.

### Delegation and models (amends the table above)

- **Managers launch a child in the foreground when its return is their next input.** The lane briefs said "background"; on this host a background child's completion goes to ROOT when its manager has stopped, and ROOT became the carrier of every such return. ROOT probed the alternative on 2026-09-19T05:14Z (`instances/ROOT/PROBE_NESTED_FOREGROUND_2026-09-19.md`): a nested foreground child's reply reached its parent directly, in the same turn, and nothing came to ROOT. Both lanes' addenda replace the clause. The relay protocol (`tools/relay.py`) remains for background children. Not probed: a foreground child that runs for a long time.
- **Who launches what, unchanged:** each manager seals and launches its own Type 2 children; ROOT launches the managers and every independent review, so that a review stays independent of the manager's account of the work; Type 2 never delegates; siblings never message each other.
- **Agent contexts do not outlive the session that launched them.** A manager, a child or a reviewer can be resumed by message only from that session. Lane records are therefore written so that a new manager can start from them, and each lane's latest addendum gives the reading order. "The same reviewer backchecks" holds inside a session; across sessions, the next complete review carries the earlier review's return and re-reads its findings' dispositions.
- **Models.** Fable: ROOT, the two managers, hard implementation, the two fidelity reviews and the author of the reopened design-system amendment. Opus: every independent code review and the design-system amendment's review. Sonnet: mechanical work against an exact inventory, which now includes the token adoption slice and the closing pass's stylesheet rows once structure is stable, the contrast measurement script's runs, and the D-72 runner. Haiku or Sonnet `Explore`: read-only inventories.

### What ROOT takes from the first day, for the record

ROOT put the tokens slice and the canvas colour slice first and let a design-system amendment grow a review loop that the shell's next slice was told to wait on. ROOT also wrote "background" into the managers' briefs. Both were ROOT's choices and neither was the owner's.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
