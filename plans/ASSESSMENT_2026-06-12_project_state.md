# OpenPipeStress — Project State Assessment (2026-06-12)

**Epistemic status:** assessment (`PROPOSAL`-grade, non-governing). Statements about repo content, registers, and committed evidence are `FACT` from this session's direct reads; judgments are labeled as assessment. Nothing here is a release, professional, certification, or code-compliance claim, and nothing here changes lifecycle state.

**Method note (differs from the prior two assessments):** this assessment was taken at ~00:20 MDT while a development session was **actively working in the same worktree** (mid-tranche on A10 — see §5). Per the F-4 operational note (do not run the cargo sweep and desktop suites concurrently), no test surfaces were independently re-run. Evidence basis instead: the committed DEC-025 sweep at HEAD, run records, the SMOKE ledger, plans/registers, and direct read-only checks (status tool, decision log, `core/units`, `.gitignore`, working tree). Snapshot HEAD: `833f0c4b8` (clean at sweep time; in sync with `origin/main`).

**Baseline:** [ASSESSMENT_2026-06-11_repo_and_app_state.md](ASSESSMENT_2026-06-11_repo_and_app_state.md) (≈18 hours earlier).

---

## 1. What changed in 18 hours: hardening sprint, eight rulings, and the start of from-scratch authoring

Since the 2026-06-11 assessment, roughly **13 tranches landed and 8 human decisions were ruled** (DEC-020..028, with DEC-026 revising DEC-024 same-day), all through the coordination loop with run records, SMOKE entries (TP-MAC-109..116), and completion-log narrative:

| Work front | What landed |
|---|---|
| **Operation-seam unification** ([plan](PLAN_2026-06-11_operation_seam_unification.md), **CLOSED**) | D-13 ruled (`DEC-020`); T1 cross-engine contract corpus (44 cases, TS mirror needed zero alignment fixes); T2 ADR surface stood up (ADR-0001); T3 wasm enablement; T4 swap — **TS engine deleted** (−3,114 lines; `operationService.ts` is a 112-line adapter; every environment now validates/applies through `operation_applier`, native or wasm). Independent post-closure verification accepted in substance; its F-1 (Playwright budget) and F-2 (NUL bytes) findings repaired same day (`TP-SEAM-FIX-001`) |
| **Merge gate** (`TP-SWEEP-001`, per `DEC-025`) | Five-surface local evidence sweep (`tools/release/run_evidence_sweep.py`) codified as the commit-bound pre-push gate; F-4 atomic wasm build rider; named in `_COORDINATION.md` step 9. Five sweep summaries recorded since, all pass |
| **A7 report rendering** (`TP-APP-R2-REPORTRENDER-001`, per `DEC-021`) | Deterministic hash-bound single-file HTML via `core/reporting/report_renderer`; 15th Tauri command; three-point protected-content lint gating; labeled non-hash-bound print view. The item the prior assessment called "the only Phase A item with zero code" is landed |
| **Hash truth** (H1 `TP-H1-HASHUNIFY-001`, H5 `TP-H5-JCSRENDER-001`) | Frontend hashing single-sourced through the wasm engine; then `core/serialization/canonical_json` made canonical JSON true RFC 8785 (ryu ECMAScript number rendering, UTF-16 key sort) — backend hashes byte-equal across all lanes, H1's measured transport exclusions retired, DEC-010 "JCS-compatible" now literally true |
| **C1 grammar freeze** (`TP-C1-GRAMMAR-001`, per `DEC-022`) | Expression grammar v1.0.0: typed AST extended to the PRD §12.3 set, 18-relation dimension-product table, `grammar_version` bound into the JCS-hashed rule-pack checksum, 69-case blessed conformance corpus. Three labeled ASSUMPTIONs await human review |
| **D7 first slice** (`TP-D03-SPARSE-001`, per `DEC-023`) | In-repo `sparse_direct` crate (deterministic RCM ordering, skyline LDLᵀ, zero new dependencies) + diagnostics + performance-harness lane. **Not yet adopted by the live solve path** (integration plan in the run record) |
| **A9 blank-project path** (`TP-APP-R2-BLANK-001`, 2026-06-12) | Explicit `New blank` local authoring target: no fixture entities, visible `MODEL_INCOMPLETE`/`BLANK_PROJECT_AUTHORING_TARGET` diagnostics, Tauri blank solve pinned to `MODEL_INCOMPLETE` with named missing-input diagnostics |
| **Process** | H4 evidence-posture amendment applied to `_COORDINATION.md` (Playwright-spec extension as default UI evidence; Vitest floor for new components); plan hygiene executed (completion plan back to 221 scannable lines; A9–A12 and H1–H5 rows added); evidence-sweep dirty-path parsing fix; init prompt committed (closing the prior assessment's dirty-file note) |

Every recommendation in the 2026-06-11 assessment's §6 has been acted on **except one**: §6.5 "start Phase B." `core/units/` still contains only a README (verified directly this session). See §6.1.

## 2. Evidence state at HEAD

The committed DEC-025 sweep at `d9fad1315` (2026-06-12T06:17Z, clean tree) reports **all five surfaces pass**: cargo crate sweep (exit 0 across 26 manifests), pytest, desktop Vitest (wasm engine built first), Playwright e2e, desktop production build — overall pass in 36s. Latest per-tranche counts from the A9/H5 run entries: desktop Vitest **174/174** (was 58 at the prior assessment), pytest **353** (was 342), Tauri Rust 29/29, Playwright 1/1, production build green. Cargo aggregate was last independently counted at 446 across 25 manifests (06-11 verification session); two crates have been added since (`canonical_json` 8 tests, `sparse_direct` 18). Counts here are quoted from committed evidence, not re-run (method note above).

Lifecycle state unchanged and re-verified via `list_deliverable_status.py --dag DAG-006`: **101 deliverables — 100 `CHECKING`, 1 `ISSUED`** (DEL-01-01). DAG-006 remains the approved active graph authority per `_DAG/_LATEST.md`.

## 3. Decision state: the register is effectively drained for current-stage work

11 of the 13 top-level `D-XX` rows are **RULED** (D-01..D-05, D-07..D-10, D-13, plus D-04's same-day DEC-026 revision). **Nothing is AWAITING_RULING.** The remaining queue is deliberately phase-timed, all `NOT_PREPARED`:

- **D-06** (release matrix/installers/signing) — Phase E packaging; pairs with D-05b.
- **D-11** (issuance waves for the 100 `CHECKING` deliverables) — human-paced Phase F.
- **D-12** (FR-024/FR-025 disposition) — at the R5 gate.
- Follow-ups D-02b, D-04b, D-05b, D-07b, D-10b — each anchored to a named later lead-up.

Human dispositions pending outside the register (non-blocking, surfaced per the loop): F-3 contract-corpus fixture `review_status` (from the seam verification); C1's three labeled ASSUMPTIONs (dimension-product table contents, derived unit-ref convention, Select branch-compatibility rule); H5's vocabulary-only residual label sites. The CAEPIPE plan's G1/G2 gates (§4) become register rows only when that work is selected.

This is a notable inversion of the 06-10 position, when nine unprepared packets were called the cheapest acceleration available. The decision pipeline ran ahead of implementation, as designed; nothing on the current spine is human-blocked. (Assessment: the two-day burst of 11 rulings was the project's highest-leverage interval so far.)

## 4. Plans inventory and their states

| Plan | State |
|---|---|
| [PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md) | **Active selection instrument.** Phase A: A1, A2, A7, A9 landed; A3/A4/A5/A6/A8 partially landed with enumerated residuals; **A10 in flight now** (§5); A11/A12 open. Phase B: **unstarted**. Phase C: C1 landed; C2–C4 open (C2/C3 need A3's authoring shell). Phase D: D7 first slice landed crate-side; D1–D6/D8 open. Phase E: decision prerequisites now mostly ruled (D-05/07/09); D-06 outstanding; no E items started. Phase F: human-paced |
| [PLAN_2026-06-11_operation_seam_unification.md](PLAN_2026-06-11_operation_seam_unification.md) | **CLOSED 2026-06-11.** T1–T4 landed, exit criteria met, independent verification accepted in substance, findings repaired. Deferred items rolled into the completion plan's hardening lane |
| [PLAN_2026-06-12_caepipe_external_oracle_feedback_loop.md](PLAN_2026-06-12_caepipe_external_oracle_feedback_loop.md) | **Drafted (r2), not started.** Verified: no `SCA-005` in SOFTWARE_DECOMP, no `validation/oracle_cases/` ignore rule yet. Its own sequencing holds T1 (SCA-005 prep) / T2 (contracts+guards) in a parallel governance lane and gates implementation (T3–T6) on the Phase A exit test — correctly, in my assessment, since the loop consumes the spine A8 proves |
| Hardening lane | H1, H4, H5 landed. **Open: H2** (DEC-019 evaluation unification through the wasm channel — `projectService.ts` `evaluateModelDocumentLocal` is now the largest remaining TS mirror of Rust logic) **and H3** (unit-test backfill for `LoadCaseManagerPanel`/`PipeViewport`/`PropertyInspector`) |

## 5. The live session (observed directly)

At assessment time the development session had just pushed A9's evidence commit (`833f0c4b8`, 00:18) and begun the next tranche: four files modified in the working tree — `operation_applier/src/lib.rs`, its contract-corpus test, `operationContractCorpus.test.ts`, and `types.ts` — already containing `create_support`/`create_material`/`create_section` references. That is the **A10 from-scratch entity creation** signature, which is exactly the next item on the plan's spine after A9. Selection discipline is holding under autonomous operation; `main` is being pushed at each closeout.

Interaction hazard (recorded, not fixed, per the external-noise rule): this assessment file itself enters the shared worktree untracked and will appear in the live session's next `git status` and sweep `dirty_paths`. Precedent exists for shared-worktree trouble (the 06-11 "shared-worktree incident" during H1 forced a mid-tranche collision-defense commit). The live session should treat this file as out-of-tranche noise; a `git add -A` closeout would silently sweep it into an unrelated tranche commit.

## 6. Findings and risks (assessment)

1. **Phase B deferral is now the project's most-repeated unexecuted recommendation, and its cost compounds with every authoring tranche.** D-01 was ruled 2026-06-10; the 06-11 assessment recommended starting B1; since then ~13 more tranches landed — several adding exactly the single-unit input surfaces (load-manager forms, A9 blank path, now A10 material/section forms) that B2 must retrofit with unit-aware display/entry. The seam plan's stated reason to defer B2 (write unit logic once, in one engine) was satisfied when T4 closed on 06-11. B1 is crate-side, parallel-lane safe, and human-ruled. Every additional A-tranche before B1/B2 widens the retrofit. This is the one place where the otherwise excellent selection discipline keeps choosing visible product motion over a ruled, unblocked foundation item.
2. **The R2 exit now has a short, concrete critical path: A10 (in flight) → A11 deletion coverage → A12 rehearsal → A8 automation.** After A10, a user can in principle author a solvable model from blank (nodes → pipes → supports → material/section → loads → combination). A11 matters for authoring honesty (today only `delete_combination_term` exists); A12/A8 convert the capability into the scripted exit evidence the phase requires. Watch item: the e2e surface is still **one Playwright spec**; A8's authored create→solve→report journey is now the R2 exit-evidence critical path, and the H4 amendment makes spec extension the default evidence for each UI tranche — enforcement should show up as specs, not only SMOKE entries.
3. **`sparse_direct` is landed but dormant** — the live solve path still runs dense. Dormant-correct code drifts; the run record holds the integration plan. Sequencing it behind Phase D scale work is defensible; leaving it unadopted past Phase D entry would not be.
4. **Hash/evidence integrity materially improved** (H1+H5): one canonical-JSON implementation, byte-equal hashes across native/wasm/headless lanes, corpus floors enforced programmatically. The remaining mirror of this defect class is H2's migration-evaluation copy (~60 lines, conservative failure mode) — cheap to close with the next persistence-adjacent tranche.
5. **Process health is strong.** The loop's full cycle (packet → ruling → DEC transcription → implementation → sweep-gated push) ran 8 times in one day without a recorded boundary breach; the plan was re-compressed per its own maintenance rule (221 lines); the sweep gate has bound every push since it landed; the completion log carries the narrative. The known weak spots are evidence-count opacity in the sweep (exit codes only — per-surface counts live in run records; D-04b tooling will eventually firm this up) and the shared-worktree concurrency hazard (§5), which is procedural, not technical.
6. **Stage position vs the coordination record:** the Working Desktop Application Standard's current target (PRD §22.3 R2 exit) is not yet met — correct per plan — but is plausibly within a handful of tranches. When A12 evidence exists, the coordination record's staged standard requires a human-reviewed stage advance; preparing that review packet alongside A12 would avoid a stall at the gate.

## 7. Recommended near-term order (PROPOSAL)

1. Let A10 land (in flight), then **B1** (`core/units` catalog/conversion crate per DEC-018) before or in parallel with A11 — crate-side TASK lane, disjoint write scope from the app tranches; B2 unit-aware I/O immediately after the A11/A12 authoring surfaces stabilize, so the retrofit is done once.
2. A11 → A12 → A8 journey automation as the R2 exit chain, with the stage-advance review packet prepared from A12's evidence.
3. Fold H2 into the first persistence-adjacent tranche; schedule H3 backfill before Phase C widens the panel surface.
4. Decision lane when the human has bandwidth: the F-3 corpus disposition and C1's three ASSUMPTIONs are five-minute reviews; D-06 preparation belongs at the Phase E lead-up, not now.
5. CAEPIPE oracle plan: keep T1/T2 parked in the governance lane until the human elects to spend a ruling on SCA-005; its implementation gating on A8 is right and needs no change.

## 8. Bottom line

Eighteen hours ago the assessment said the product hinge existed but an engineer could not start from nothing, read a rendered report, or trust that the browser and backend enforced the same rules. Today the report renders hash-bound (A7), one Rust engine enforces the rules everywhere with byte-equal canonical hashes (seam plan closed, H1/H5), the blank-project path exists (A9), and from-scratch entity creation is being implemented in a live session as this is written (A10). Eight human rulings cleared the entire current-stage decision queue; nothing on the spine is human-blocked; the five-surface sweep gates every push. The two things this assessment would change: start the ruled-and-waiting units engine (B1) before more single-unit UI accrues retrofit debt, and treat the A8 journey automation as the R2 exit's evidence backbone rather than its trailing item. The trajectory from technical preview to the R2 "create, solve, report without editing raw files" milestone is now measured in tranches, not phases.
