# R2 Exit-Chain Verification — TP-INTEGRATED-VERIFY-002 (2026-06-12)

**Epistemic status:** independent verification record (assessment, non-governing). A separate WORKING_ITEMS session re-ran the R2-chain test surfaces and audited the landed A9–A12/A8 work against the PRD §22.3 exit criterion verbatim, after the executing session stopped at the human stage-advancement gate. This is the derivative verification snapshot the completion plan's Phase A exit-evidence row calls for (TP-INTEGRATED-VERIFY successor). No lifecycle, release, professional, certification, or code-compliance claim. Only the human project authority can advance the target stage.

## Verdict

**The R2 exit criterion — "User can create, solve, and report a small piping model without editing raw files" (PRD §22.3) — is demonstrated in substance, across two complementary verified surfaces, with one named seam not yet exercised in a single packaged run.**

| Criterion element | Evidence | Status |
|---|---|---|
| Create from blank, no raw file edits | A9 `New blank` + A10 create set (support/material/section) + A11 deletion coverage; Playwright journey authors the full A12 script (2 nodes, material, section, pipe run, support, load case, primitive load, combination) through visible GUI controls only | **Verified** (re-run this session) |
| Solve the authored model | A12 Tauri regression applies the script via `apply_model_operation` and solves via `run_preview_mechanics`; saved-project regression proves save → reopen → solve; browser mode honestly refuses edited-model solves (`BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`) | **Verified** (re-run this session) |
| Report the authored model | A12/saved-project regressions render the hash-bound A7 HTML via `render_calculation_report`; browser mode honestly refuses (`REPORT-RENDERER-DESKTOP-ONLY`) | **Verified** (re-run this session) |
| All of the above in one packaged-runtime GUI pass | Packaged binary builds and boots (below); GUI journey inside the packaged webview not yet exercised | **Open gap** (F-4) |

## Independently re-run evidence (this session, 2026-06-12 ~08:00 MDT)

| Surface | Result |
|---|---|
| Tauri backend suite (`cargo test`, src-tauri) | **32/32**, including `r2_from_blank_rehearsal_authors_solves_and_renders_report` and `r2_from_blank_saved_project_opens_solves_and_renders_report` |
| `core/product_physics` | **25/25**, including the B2 mixed-unit equivalence witness (MPa/mm/kPa ≡ SI fixture) and incompatible-unit blocking |
| `core/units` | **13/13**, including the schema↔crate dimension-vocabulary parity test |
| Playwright e2e (real Chrome, wasm engine rebuilt first) | **2/2**, including the from-blank GUI journey (every receipt `route=local_wasm_engine`, `professional_approval=false`) |
| Packaged desktop binary | `tauri build --debug --no-bundle` built in 1m05s; binary launched, registered a Foreground window with WebKit renderer/networking processes, zero bytes of error output; terminated cleanly |
| Five-surface DEC-025 sweep | run at this verification session's closeout HEAD; summary committed alongside (see `validation/evidence/sweeps/`) |

## Findings

- **F-1 — Silent load-category coercion at the product-physics boundary.** `parse_category` (core/product_physics/src/lib.rs) maps operation-authored `concentrated_force`/`concentrated_moment` → `Occasional` and `distributed_force` → `Weight` with no in-product disclosure. Mechanically labeling-level today (application method comes from the load target), but the coercion assigns stress-practice-meaningful classification the user did not choose, and category becomes load-bearing when combination-basis semantics and Phase D arrive. Recommended repair: emit a named advisory diagnostic per mapped load. **CLOSED same-session** by `TP-R2VERIFY-FIX-001` (`LOAD_CATEGORY_PREVIEW_MAPPED` warning diagnostic; see completion log).
- **F-2 — DEC-025 sweep git-state block can read false-clean.** Sweep summaries from the 2026-06-12 executing session record `working_tree_dirty: false` for a window in which an untracked file demonstrably existed in the worktree; `_capture()` in `tools/release/run_evidence_sweep.py` additionally converts any git invocation failure into a clean-looking empty result. The five surface results are unaffected; the commit-binding metadata is. Recommended repair: record capture failure explicitly and never default to clean. **CLOSED same-session** by `TP-R2VERIFY-FIX-002` (`status_capture_failed` member, `-gitunverified` filenames, nonzero exit on unbound summaries, schema_version 2; see completion log).
- **F-3 — Export-schema compatibility stance unrecorded.** `TP-UNITS-B2-EXPORTDISCLOSURE-001` made `unit_system_disclosure` a required member of the strict PCF/MBF/stress-neutral package schemas without an explicit version/compatibility note. Acceptable for unreleased, regenerated-on-demand derived packages; PKG-17's contract conventions suggest recording the stance at the next PKG-17-touching tranche. Routed as a residual, not repaired here.
- **F-4 — Packaged-runtime GUI journey remains unexercised.** Blockers recorded per the H4 evidence-posture exception: `tauri-driver` (the Tauri WebDriver harness) does not support macOS, so the Playwright/WebDriver journey cannot be extended in-repo to the packaged webview; interactive GUI driving from this verification session was attempted and halted at the desktop-automation permission gate (approval dialog timed out). What was proven instead: the packaged binary builds and boots with the embedded production frontend (TP-MAC-140). The gap stays in the A5/A8 rows; it qualifies the R2 claim but does not contradict it — every element of the criterion is proven on at least one verified surface.

## Disposition

This snapshot, the SMOKE TP-MAC-140 entry, and the DEL-00-08 run record together form the evidence packet input to the R2 stage-advancement decision (D-14, `execution/_Coordination/_DECISIONS/D-14_r2_stage_advancement.md`). The human project authority rules; agents do not advance the stage.
