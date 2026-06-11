# OpenPipeStress — Repo & Application Assessment (2026-06-11)

**Epistemic status:** assessment (`PROPOSAL`-grade, non-governing). Statements about repo content and test runs are `FACT` from this session's direct reads and command runs; judgments are labeled as assessment. Nothing here is a release, professional, certification, or code-compliance claim, and nothing here changes lifecycle state.

**Baseline:** [ASSESSMENT_2026-06-10_repo_and_app_state.md](ASSESSMENT_2026-06-10_repo_and_app_state.md) (≈34 hours earlier). This assessment covers the delta plus a fresh verification pass: all root governance docs read end to end, decomposition/coordination/PKG-00 inspected, all four test surfaces independently re-run, and the new edit-loop code reviewed.

---

## 1. What changed in 34 hours: the R2 edit loop became real

Since the 2026-06-10 assessment, **28 commits landed 27 bounded tranches** (SMOKE ledger TP-MAC-82 → TP-MAC-108), all through the Application Integration And Issuance Loop, each with run records fanned into the owning deliverables, a SMOKE entry, completion-log narrative, and a compressed plan row. The composition:

| Plan item | Tranches landed | What exists now |
|---|---|---|
| A1 apply-operation seam | 1 | `core/model_operations/operation_applier` crate (4,885 lines, 34 tests); `validate_model_operation` / `apply_model_operation` Tauri commands (bridge now **14 commands**, 2 on the mutating path) |
| A2 model-document persistence | 1 | DEC-019 implemented: per-document semver + transform chain, migrate-in-memory-on-open / persist-on-save, store v9 evidence ledger ([model_document_migration.rs](../apps/desktop/src-tauri/src/model_document_migration.rs)) |
| A3 viewport editing UX | 7 | selection→tree/inspector binding, inline validate-only feedback, explicit node form, session undo/redo, explicit straight-pipe form, canvas click node drafting (raycast to `y=0` plane), viewport endpoint picking |
| A4 load case manager | 13 | right-rail manager: primitive magnitude/metadata edits; creation of empty load cases, all six primitive categories (concentrated force/moment, distributed, pressure, thermal, imposed displacement), combinations, combination terms; term factor/basis edits; term deletion |
| A5 solve-from-edited-model | 2 | edited-model solve guard (`MODEL_INCOMPLETE` + explicit diagnostic in browser fixture mode; Tauri path solves supplied payloads bound to edited `project.id`); **persisted round-trip regression**: edit → save to SQLite → reload → solve → result row provably changes |
| A6 results visualization | 2 | review-only normalized displacement-magnitude overlay (explicitly labeled non-physical scale, `vector_direction=TBD`); result family filters (displacement/reaction/force/moment/stress with counts) |
| A8 GUI test harness | 1 | Playwright harness + root `npm run test:e2e:desktop`; one R2 smoke spec covering shell, boundaries, viewport canvas activity, solve, result detail, overlay, report export |

Two human decisions were prepared and ruled same-day through the loop's escalation mechanism (D-01 → `DEC-018` SI-canonical units with dual display catalog; D-08 → `DEC-019` per-document semver migration), recorded in [SOFTWARE_DECOMP.md](../execution/_Decomposition/SOFTWARE_DECOMP.md) §12. The D-10 packet (report rendering target) is drafted, evidence-pinned, and **AWAITING_RULING**.

This is the assessment's headline: the gap called "largest" on 2026-06-10 — *no intent→operation→persisted-model loop* — **is closed as a mechanism**. A user can now select, create (nodes, pipes, load cases, primitives, combinations), edit, undo/redo, save, reload, and solve the edited model through structured operations with deterministic validation, diff preview, audit receipts, and hash evidence — and the persisted-edit-changes-the-result chain is regression-tested at the Tauri backend.

## 2. Verified test and build evidence (this session, 2026-06-11)

| Surface | Result | vs 2026-06-10 |
|---|---|---|
| Python (`python3 -m pytest -q tests`) | **342 passed**, 5.1s | +2 |
| Rust (`check_release_readiness.py --profile cargo --execute`) | **444 passed / 0 failed across 25 crate manifests** | +22 |
| Desktop Vitest (`npm test --workspace apps/desktop`) | **58 passed** (5 files) | +45 (was 13) |
| Desktop Playwright (`npm run test:e2e:desktop`) | **1/1 passed** (4.2s, real Chrome, dev server) | new surface |
| Desktop production build | passes (vite 1.58s; one >500 kB chunk warning) | unchanged |

Scale (this session's count basis: non-generated source files): Rust ≈45k lines across 26 crates; TypeScript ≈32k lines in the desktop app (+~8k in one day); Python ≈47k lines including tools and tests. Lifecycle state unchanged: **100 deliverables `CHECKING`, 1 `ISSUED`** (DEL-01-01), confirmed via `list_deliverable_status.py --dag DAG-006`.

## 3. Governance and process: working as designed

- **Docs/decomposition/coordination coherence holds.** The full docs set (DIRECTIVE, CONTRACT, TYPES, SPEC, PRD, INTENT, IP/professional boundaries, validation strategy, build/release, agentic workflow) remains internally consistent and consistent with the code I inspected. The one stale-doc item flagged on 2026-06-10 ([BUILD_AND_RELEASE.md](../docs/BUILD_AND_RELEASE.md) §3) was fixed the same evening.
- **The coordination loop's full cycle is now demonstrated**, including its rarest leg: decision-blocked item → packet → human ruling → DEC record → implementation landed under the ruling (A2 under DEC-019). Evidence hygiene (run records fanned to owning deliverables, SMOKE ledger, completion log) was maintained across all 27 tranches.
- **Boundary discipline held under high tranche velocity.** Independent code review this session found: all mutations route through the structured-operation seam (no bypasses found); no silent engineering defaults (empty input becomes explicit `TBD`, unit mismatches block with remediation text, deferred fields block with named plan items); all professional/compliance claim fields hard-`false`; no new protected-content red flags. One exception noted in §5.
- **Working tree:** only [init/init-prompt.md](../init/init-prompt.md) is modified (uncommitted): the loop driver was loosened from stop-at-first-ruling to "stop when **any further** progress requires a human ruling … commit and push **whenever** you complete a tranche" — i.e., the multi-tranche autonomous session mode that produced this delta. Worth committing or reverting deliberately rather than leaving dirty.

## 4. Where the application actually stands against R2

R2 exit (PRD §22.3): *"User can create, solve, and report a small piping model without editing raw files."* The loop mechanism exists; the remaining distance is now precisely enumerable:

1. **From-scratch authoring is not yet possible — only extend-and-edit of the loaded invented model.** The seam supports exactly 10 operation kinds (`set_field`, `update_load`, `update_support`, `create_node`, `connect_pipe_run`, `create_load_case`, `create_primitive_load`, `create_combination`, `create_combination_term`, `delete_combination_term`). There is **no `create_support` (a from-scratch model can't be restrained), no material/section creation (pipes must reference an existing material), and no deletion of nodes/pipes/load cases/combinations** (only combination terms). `create_local_project` seeds from the current session model, not a blank document. Today's honest capability statement: a user can author *additions and edits to* a solvable model, not author a small model from nothing.
2. **A7 report rendering is the only Phase A item with zero code, and it is human-blocked.** The app's "report" today is a JSON technical-preview packet export ([ReportPanel.tsx](../apps/desktop/src/features/report/ReportPanel.tsx)); envelopes/sections/linter crates are ready but nothing renders a reviewable document. **D-10 is the single highest-leverage human action available** — everything else in Phase A is unblocked residual work.
3. **Phase B has not started despite D-01 being ruled.** `core/units/` still contains only a README. Every authoring surface landed this cycle hardcodes the single-unit preview posture (project units `m`, `N`, `Pa`, `degC`, `rad` validated literally). That was the sanctioned interim posture, but each new A3/A4 surface adds UI that B2 must later retrofit — the cost of deferring B1/B2 grows with every authoring tranche.
4. **A5/A8 residuals:** no packaged-Tauri (vs dev-server) GUI smoke over a saved edited project; the Playwright surface is one happy-path spec with hardcoded fixture IDs — meaningful, but not yet the "authored create→edit→solve→report journey" automation the plan targets, and no error-path coverage.
5. **Beyond R2, the 2026-06-10 gap list is unchanged**: solver component depth (no bend/branch/expansion-joint elements; no assembled nonlinear iteration), headless CLI binary, release machinery, and the nine unprepared decision packets (D-02..D-07, D-09, D-11, D-12).

## 5. New findings from this cycle's code (not in the prior assessment)

1. **Dual-engine drift risk (highest structural risk).** The operation seam is implemented twice: browser-local TypeScript mirror ([operationService.ts](../apps/desktop/src/services/operationService.ts), 2,280 lines) and the Rust applier (4,885 lines). Field rules, dimension vocabularies, restraint tokens, and the 10 operation kinds are hand-duplicated; there is **no cross-engine contract test** (apply the same intent in both, diff outcomes). Each new field/operation kind is a two-place edit. Phase B (unit-aware fields everywhere) multiplies this surface; divergence here would corrupt the no-silent-defaults guarantee precisely where users trust it.
2. **Component accretion is outpacing unit tests.** [LoadCaseManagerPanel.tsx](../apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx) reached 2,051 lines / ~29 state hooks / 13 near-identical intent-builder functions with **zero unit tests** (covered only via App-level tests and the single e2e path); [PipeViewport.tsx](../apps/desktop/src/features/viewport/PipeViewport.tsx) (1,193 lines) and PropertyInspector are likewise untested at unit level. Vitest growth (13→58) concentrated in App-level and service tests.
3. **One boundary-hygiene blemish:** [LocalFeaHandoffPanel.tsx](../apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.tsx) falls back to fixture entity IDs (`pipe:P-120`, `node:N-140`) when result summary fields are missing — a silent-invented-reference seam that contradicts the otherwise uniform explicit-finding posture. Small, but exactly the class of defect this project's invariants exist to prevent; cheap regression-repair tranche.
4. **Plan-document hygiene is degrading under its own maintenance rule.** The plan (§ "Plan maintenance") requires landed detail to compress to one line and move to the log — the log half happened, but the A3/A4/A5/A6 rows *also* accreted per-sub-slice narrative (the A4 cell is ~700 words). The FR map rows (FR-007, FR-013) have the same growth. The plan is becoming a history despite the rule designed to prevent that.
5. **Phase A sizing calibration was ~2× off.** §5 estimated Phase A at "~8–14 bounded tranches"; 27 have landed with A7 plus residuals across A3/A4/A5/A8 still open. Not a problem in itself — slicing went finer than the estimate, which the plan explicitly labeled an `ASSUMPTION` — but remaining-phase estimates (B ~3–5, C ~4–6, D ~8–12, E ~6–10) should be re-read at this granularity, especially Phase D.

## 6. Recommendation: update the plan — surgically, not structurally

**Yes, update [PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md), as targeted maintenance.** The plan's architecture (phases, decision register, FR map, spine ordering, completion-log mechanism) remains correct and is demonstrably steering selection well — no replan is warranted. The specific updates worth making:

1. **Enforce the plan's own compression rule.** Collapse the A3/A4/A5/A6 row sub-slice narratives to the one-line `LANDED` form (the detail already exists in the completion log) and prune the duplicated sub-slice enumerations from FR-007/FR-013 rows. Restore the plan to a scannable selection instrument.
2. **Make the R2 from-scratch authoring set explicit.** Replace the catch-all "broader editor coverage" with named remaining items, because they — not more load-manager breadth — are now the binding gap on the R2 exit criterion: `create_support` (+ delete), material creation (+ section if modeled as records), node/pipe/load-case/combination deletion, and a blank-model or minimal-template new-project path. Suggest adding these as an enumerated A3 remainder or a new "A9 — R2 exit checklist" row tied verbatim to PRD §22.3.
3. **Add a hardening row for the dual-engine seam** (fits the loop's "residual hardening" lane): a cross-engine contract test that feeds identical intents to `operationService.ts` and `operation_applier` and diffs outcomes, plus unit tests for the load-case manager and viewport before Phase B widens the field surface. Cheap now, expensive after B2.
4. **Record the D-10 bottleneck status in the plan's decision register row** (it already says AWAITING_RULING — add that it is the *only* Phase A blocker and the highest-leverage pending human action). Every session summary should continue surfacing it, per the loop.
5. **Start Phase B explicitly.** D-01 is ruled; B1 (unit catalog + conversion crate in `core/units`) is unblocked and parallel-lane safe. Sequencing B1/B2 soon caps the retrofit cost that every additional single-unit authoring surface accrues. The plan already permits this; the update is to mark B1 as *selected next* alongside the remaining A items rather than implicitly queued behind them.
6. **Minor:** add the `LocalFeaHandoffPanel` fixture-fallback fix as a regression-repair item; recalibrate the §5 tranche-count assumption (~2× finer granularity observed); and commit or revert the uncommitted [init/init-prompt.md](../init/init-prompt.md) loop-driver edit so the live driver text is version-controlled.

## 7. Bottom line

Thirty-four hours ago this repo was a disciplined contract-and-mechanics foundation with a fixture-driven preview in front of it. Today the preview is an *editing* application: the product hinge the prior assessment called out — intent→operation→persisted-model→solve — exists, is exercised end to end across four green test surfaces (342 pytest / 444 cargo / 58 Vitest / 1 Playwright e2e), and was built without a single observed breach of the project's IP, no-silent-defaults, or professional-boundary invariants, including through two same-day human decision gates. What it is still not: a tool in which an engineer can start from nothing (no support/material authoring, no deletion), read a rendered calculation report (blocked on the D-10 ruling), or work in any unit system other than the single labeled preview set (Phase B unstarted). The plan that produced this delta is healthy and should be kept — updated surgically per §6 — and the fastest path to the R2 exit now runs through three things: the D-10 ruling, the explicit from-scratch authoring set, and starting B1 before the single-unit posture gets more expensive to retire.
