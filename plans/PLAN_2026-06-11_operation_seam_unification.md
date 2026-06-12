# OpenPipeStress — Operation-Seam Unification Plan

**Date:** 2026-06-11
**Epistemic status:** PROPOSAL (non-governing). This plan proposes the resolution of the duplicated editor-operation engine identified in [ASSESSMENT_2026-06-11_repo_and_app_state.md](ASSESSMENT_2026-06-11_repo_and_app_state.md) §5.1/§5.3 and the follow-on review discussion with the human project authority. It changes no lifecycle state and creates no release, professional, certification, or code-compliance claim. Every item below is a candidate tranche to be proposed, approved, executed, and evidenced through the Application Integration And Issuance Loop in [_COORDINATION.md](../execution/_Coordination/_COORDINATION.md). Humans decide all gates.

**Relationship to the completion plan:** [PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md) remains the active selection instrument. This plan occupies the loop's *residual hardening* lane next, and supplies the recorded rationale the loop requires for selecting hardening ahead of new current-stage scope (§8 below). Landed rows here follow the same maintenance rule: compress to one line, move narrative to [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md).

**Plan acceptance (RULED 2026-06-11):** the human project authority accepted this plan as the active hardening-lane selection and **authorized the executing agent to add this plan's pointer to the Active Surface list in [_COORDINATION.md](../execution/_Coordination/_COORDINATION.md)** (one-line edit alongside the completion-plan pointer, to be removed when this plan closes). The handoff agent performs that edit as its first coordination action, then executes T1→T4 in order. The §5 freeze rule is in force as of acceptance.

**PLAN CLOSED 2026-06-11:** T1–T4 all LANDED (see §3 rows); §4 exit criteria met; §5 freeze rule lifted at the T4 landing; the Active Surface pointer was removed per the acceptance authorization above. This file remains as history; narrative lives in [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md). Deferred items roll forward per §9 into the next development-plan revision.

---

## 1. Problem and basis (FACT unless labeled)

The structured editor-operation seam — the enforcement point for no-silent-defaults, unit/dimension checks, before-state integrity, and professional-boundary receipts — is implemented twice:

- **Authoritative engine:** [core/model_operations/operation_applier/src/lib.rs](../core/model_operations/operation_applier/src/lib.rs) (Rust, ~4,885 lines, 34 tests), invoked by the `validate_model_operation` / `apply_model_operation` Tauri commands.
- **Browser mirror:** [apps/desktop/src/services/operationService.ts](../apps/desktop/src/services/operationService.ts) (~2,280 lines, 17 tests), used whenever no Tauri backend exists — Vite dev mode, all jsdom/Vitest edit-loop tests, the Playwright e2e harness, and the agent in-browser smoke evidence that closed the 27 tranches of 2026-06-10/11.

The mirror was a deliberate, code-level choice (its header names the Rust crate as "the contract reference"), but: (a) no executable contract enforces alignment — each suite tests only its own engine; (b) no ADR records the decision — the ADR surface DEL-00-01 anticipated (`docs/architecture/adr/index.md`, `template.md`) does not exist in the worktree; (c) no governance document mandates the mirror (verified across docs/, SOFTWARE_DECOMP, the PKG-00 packets, and the coordination record).

**Why resolve now.** Two scheduled work fronts multiply the duplication: the R2 from-scratch authoring set (~4–6 new operation kinds: support/material creation, entity deletion, blank-project path) and Phase B unit-aware fields (conversion/tolerance logic per `DEC-018` — numerically sensitive code that must never be hand-maintained twice). Failure mode if unresolved: environment-dependent validation outcomes and environment-dependent canonical model hashes — an invisible fracture of the project's central no-silent-defaults and hash-evidence guarantees.

**Resolution direction (per human input, 2026-06-11):** dual maintenance is not an acceptable steady state. The TypeScript engine is to be **deleted** — by replacement, not amputation: the Rust crate, compiled to WebAssembly, becomes the sole engine in every environment. What remains in TypeScript is only what must be: React UI, intent builders, and a thin routing adapter. The browser evidence loop (Vitest, Playwright, agent smoke) is preserved throughout and afterward exercises the shipped engine rather than a copy.

**Alternatives considered and rejected** (record in the ADR): keep dual + contract test permanently (safe but inefficient; Phase B compounds it); delete browser-mode editing outright (guts the GUI test layer and the agent verification loop; forces all evidence onto packaged-native automation that does not exist and is weak on macOS); local Rust dev-sidecar over HTTP (adds a process and network surface against the local-first/no-daemon posture).

## 2. Human gate — **RULED**

| ID | Decision | State | Gates |
|---|---|---|---|
| D-13 | **Operation-seam engine unification:** adopt a wasm32 build of `operation_applier` as the sole browser-mode engine; retire the TypeScript mirror's validation/apply logic; stand up the DEL-00-01 ADR surface and record this as its first entry | **RULED 2026-06-11 — accepted as proposed** by the human project authority (ruling captured in this session against this plan as the packet) | None remaining. T1–T4 are all unblocked; T2 transcribes the ruling into the formal record surfaces before T3/T4 execute |

**Ruling record (2026-06-11, human project authority):** *Accept as proposed — adopt the wasm32 build of `operation_applier` as the sole browser-mode engine; retire the TypeScript mirror's validation/apply logic at T4; stand up the DEL-00-01 ADR surface and record this decision as its first accepted entry.* The alternatives in §1 (permanent dual engines + contract test; deleting browser-mode editing; dev sidecar; deferral until T1 evidence) were presented and declined.

**Transcription obligations (T2, bookkeeping authorized by the ruling above — not a new decision):**
1. Write the ADR with status **accepted**, citing this ruling and date.
2. Append the decision to [SOFTWARE_DECOMP.md](../execution/_Decomposition/SOFTWARE_DECOMP.md) §12 as `DEC-020`, following the `DEC-018`/`DEC-019` pattern ("Human project authority ruling on 2026-06-11 accepting `plans/PLAN_2026-06-11_operation_seam_unification.md` §2 as proposed").
3. Append the D-13 row to [_DECISIONS/_REGISTER.md](../execution/_Coordination/_DECISIONS/_REGISTER.md) directly in state **RULED**, with this plan as the packet pointer and `DEC-020` as the ruling record.

## 3. Tranche plan

### T1 — `TP-SEAM-CORPUS-001`: cross-engine contract corpus (+ boundary-hygiene rider)

**LANDED 2026-06-11** — `TP-SEAM-CORPUS-001`: 44-case cross-engine corpus + both runners landed (`cargo test` 36/36; Vitest 105/105; Playwright 1/1 unchanged; desktop build green); coverage floor enforced programmatically in both runners; the TS engine reproduced all 44 Rust-reference outcomes with zero alignment fixes (`operationService.ts` unchanged); rider landed (explicit `LOCAL-FEA-RESULT-SUMMARY-REF-MISSING` finding replaces the fixture-ID fallback). Residual: corpus README `review_status` pending human review. Evidence: `apps/desktop/SMOKE.md` TP-MAC-109; DEL-16-02 `_run_records/WORKING_ITEMS_RUN_2026-06-11_seam_contract_corpus_fanin.md` (fan-out: DEL-16-03, DEL-00-08; rider: DEL-10-03); narrative in [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md).

### T2 — `TP-SEAM-DECISION-001`: ADR surface + ruling transcription (ruling already in hand — see §2)

**LANDED 2026-06-11** — `TP-SEAM-DECISION-001`: ADR surface stood up (`docs/architecture/adr/` — template, index, ADR-0001 status accepted) after the gate check confirmed the surface was unbuilt by deferral, not resolved elsewhere; `DEC-020` appended to SOFTWARE_DECOMP §12; D-13 register row appended directly in state RULED; cross-consistency of ADR ↔ DEC-020 ↔ D-13 ↔ this plan's §2 verified; no code changes. No residuals. Evidence: DEL-00-01 `_run_records/TASK_RUN_2026-06-11_1408.md`; narrative in [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md).

### T3 — `TP-SEAM-WASM-001`: wasm build enablement (proceeds after T2 transcription)

**LANDED 2026-06-11** — `TP-SEAM-WASM-001`: corpus-through-wasm spike succeeded (§6 stop rule did not fire; no TS fallback written); three-way native↔wasm↔TS corpus parity green including canonical-hash equality (`cargo test` 36/36; Vitest 151/151 with 46 wasm-lane tests; desktop build green, index chunk at baseline); pinned toolchain verified (wasm-bindgen 0.2.123); `BUILD_AND_RELEASE.md` §3 baseline paragraph added; loading-shim fixes were artifact-location/typing only. No residuals. Evidence: DEL-00-02 `_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_completion.md` (fan-out: DEL-00-08, DEL-16-02); narrative in [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md).

### T4 — `TP-SEAM-SWAP-001`: route browser mode through wasm; delete the TS engine (proceeds after T2 transcription)

**LANDED 2026-06-11** — `TP-SEAM-SWAP-001`: `operationService.ts` is a 112-line thin routing adapter (Tauri `invoke` else wasm; ≤ ~250 target met); TS engine + private tables deleted, 17 engine tests superseded by corpus + 6 adapter tests (net −3,114/+200 lines); browser route receipt now `local_wasm_engine`; apply panel reports engine status (`operation-engine-status`); corpus continues as native↔wasm parity (adapter lane + direct lane); Playwright extended with engine-ready wait + real-browser apply route assertion; full surfaces green (cargo sweep exit 0 across 25 manifests; pytest 342/342; Vitest 140/140; Playwright 1/1; production build, index chunk at baseline). §5 freeze rule lifted at this landing. No residuals. Evidence: `apps/desktop/SMOKE.md` TP-MAC-110; DEL-16-02 `_run_records/WORKING_ITEMS_RUN_2026-06-11_t4_wasm_engine_swap.md` (fan-out: DEL-16-03, DEL-07-02, DEL-00-08); narrative in [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md).

## 4. Exit criteria

1. One operation engine: every environment (packaged Tauri, dev browser, jsdom, Playwright) validates and applies through `operation_applier` (native or wasm); the TS engine logic no longer exists in the tree.
2. Contract corpus checked in with provenance, executing in `cargo test` and `npm test`, asserting semantic and canonical-hash parity native↔wasm.
3. `D-13` RULED (→ `DEC-020`); ADR surface exists with the unification ADR as a recorded, accepted entry.
4. `LocalFeaHandoffPanel` fixture fallback replaced by an explicit finding.
5. All test surfaces green at closeout (pytest unaffected; cargo; Vitest; Playwright; production build), SMOKE/run-record/completion-log evidence recorded, tranches committed and pushed per the loop.

## 5. Interim guardrails (from plan acceptance until T4 lands — **lifted 2026-06-11 at the T4 landing**)

- **Freeze rule:** no new operation kinds or field rules enter the TS engine. If an urgent authoring slice must land mid-window, it ships Tauri-only with an explicit browser block (named diagnostic, e.g. `BROWSER_OPERATION_BACKEND_REQUIRED`, following the existing edited-model solve-guard pattern) — an honest finding, never a silent divergence.
- All standing boundary prohibitions hold: invented fixtures only with provenance; no protected standards content; no network/daemon/cloud surfaces (wasm runs in-process; local-first intact); no compliance/certification/approval language; deferred fields stay deferred (the corpus encodes blocked-stays-blocked); semantics are preserved — this plan changes *where* validation runs, never *what* it accepts.

## 6. Risks and stop rules

| Risk | Handling |
|---|---|
| wasm loading friction in Vitest/jsdom | T3 spikes it first; stop-and-escalate rule — no TS engine may be reintroduced as a workaround |
| Numeric/hash parity native vs wasm | Corpus asserts canonical-hash equality; same `serde_json` crate both targets; IEEE f64 semantics identical under wasm |
| Toolchain availability/drift | Prerequisites documented and pinned in the ADR; scripts fail explicitly when tooling is absent |
| Bundle size / startup | Lazy engine init; measure against the existing ~577 kB index chunk; record in T4 evidence |
| Mid-window authoring pressure | §5 freeze rule + Tauri-only escape hatch; R2 authoring set is sequenced after T4 by design (§7) |
| Playwright timing on wasm init | Engine-ready testid; spec waits on it |

## 7. Sequencing against the completion plan

- **Waits for T4:** the R2 from-scratch authoring set (support/material creation, deletions, blank-project path) and Phase B item B2 (unit-aware I/O) — so their operation kinds and unit logic are written once, in one engine.
- **May run in parallel (disjoint write scopes):** B1 (`core/units` catalog/conversion crate — crate-side only), A7 report rendering once `D-10` is ruled, and decision-packet preparation (`D-02`, `D-03`, `D-04`).
- Estimated scale: 4 bounded tranches, one human ruling. (`ASSUMPTION`: granularity comparable to the June app tranches.)

## 8. Loop-selection rationale (recorded here per the coordination rule)

The loop requires a recorded reason when hardening is selected ahead of unblocked current-stage scope. Reason: the next current-stage scope (R2 authoring set) itself expands the duplicated surface ~50% (4–6 new operation kinds × two engines); landing T1–T4 first makes that scope cheaper, single-engine, and verified by evidence that exercises the shipped engine. The corpus also upgrades the probative value of every existing browser-based test retroactively.

## 9. Assessment items folded in vs deferred

**Folded into this plan:** dual-engine drift risk (assessment §5.1 → T1/T3/T4); missing-ADR governance gap (review discussion → T2, including standing up the DEL-00-01 ADR surface); `LocalFeaHandoffPanel` fixture fallback (§5.3 → T1 rider).

**Explicitly deferred to the revised development plan** (per the human direction that remaining findings roll forward after this work):

1. R2 from-scratch authoring enumeration as explicit plan rows (create_support, material creation, entity deletion, blank-project path) — post-T4.
2. Evidence-posture amendments to the coordination record: Playwright-spec extension as default UI evidence (live browser smoke as exception); unit-test floor for new components; template-batch rule for homogeneous slices; decision-prep ahead of need (batch `D-02`/`D-03`/`D-04`).
3. Completion-plan hygiene: compress A3/A4/A6 rows and FR-007/FR-013 cells per the plan's own rule; recalibrate §5 sizing (~2× finer granularity observed).
4. Unit-test backfill and factoring for `LoadCaseManagerPanel`, `PipeViewport`, `PropertyInspector` (beyond what T4 touches).
5. Packaged-Tauri smoke over a saved edited project; CI browser provisioning policy.
6. Phase B B1–B3 execution; `D-10` ruling → A7 report rendering; remaining decision packets `D-05`–`D-07`, `D-09`, `D-11`, `D-12`.
7. Housekeeping: commit or revert the uncommitted [init/init-prompt.md](../init/init-prompt.md) loop-driver edit (human git action).

**Acceptance status:** plan accepted and D-13 ruled by the human project authority on 2026-06-11 (see header and §2). The executing agent's first coordination action is the authorized Active Surface pointer edit in [_COORDINATION.md](../execution/_Coordination/_COORDINATION.md); execution then proceeds T1 → T2 (transcription) → T3 → T4 under the loop's normal evidence and git-closeout discipline.
