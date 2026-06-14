---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-AGGSTATUS-001
timestamp: 2026-06-14T15:20:00-0600
completed: 2026-06-14T16:10:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-AGGSTATUS-001 — drive the rule-check aggregate into the solve envelope / result assembly (C4 non-GUI residual)

## Tranche and authority basis

- Tranche: the completion-plan **C4** residual **"Remaining scope (non-GUI):
  driving `aggregate_status` into the solve envelope / `result_export`"** — the
  next unblocked item on the R3/Phase C dependency spine after C3's named
  residuals closed (`TP-C3-LIBREFPICKER-001`). Human-approved to take next
  ("continue to take the C4 aggregate_status item next"). The coupled *future
  additive `acceptability_relation` / solver-result-selector schema members* are
  explicitly out of scope here (a separate, likely ratification-gated item).
- Problem: the `core/rules/rule_check_runner` orchestrator computes a worst-of
  `aggregate_status` (`RULE_INPUTS_INCOMPLETE` / `USER_RULE_CHECKED` /
  `USER_RULE_FAILED`), but that aggregate lived only in the GUI run panel's
  ephemeral result. The non-GUI **result assembly** never incorporated it: the
  `core/runner/headless` bridge `run_preview_in_memory` — the one concrete
  non-GUI place that composes a result's `analysis_status` from a solve —
  **hardcoded the rule-check half to `RuleInputsIncomplete`** (it keyed off the
  product-physics envelope's `status.rule_check`, which a plain solve always
  leaves at `RULE_INPUTS_INCOMPLETE` because the solve runs no user rule checks).
  So a programmatic run that *did* execute rule checks could not report
  `USER_RULE_CHECKED` / `USER_RULE_FAILED` in its result envelope.
- Blockers: none. No human decision gates this; the rule-check status vocabulary
  is frozen (the three automatic statuses).

## Design decision (bounded; single crate; no schema/ruling)

The vocabulary already lines up — the `rule_check_runner` aggregate serializes to
the same three strings the result surfaces use, and the `result_export`
`AnalysisStatus` enum (DEL-08-04) **already** carries `UserRuleChecked` /
`UserRuleFailed` / `RuleInputsIncomplete`. The gap was purely the **assembly**:
nothing drove the aggregate into the envelope. So this is a wiring change in the
single owning crate (`core/runner/headless`, DEL-10-05), no schema/vocabulary
change anywhere:

1. New pure, public `analysis_status_for_rule_check(&str) -> Option<AnalysisStatus>`
   mapping the three rule-check vocabulary strings to the runner analysis-status
   vocabulary; `None` for anything else (no silent coercion — CONTRACT
   no-silent-defaults).
2. `run_preview_in_memory` now delegates to a new
   `run_preview_in_memory_with_rule_check(request, preview_request,
   rule_check_aggregate: Option<&str>)` (the old signature is preserved exactly →
   backward compatible; `None` reproduces the prior behavior). When a recognized
   aggregate is supplied it is written into the carried `MechanicsEnvelope`'s
   `status.rule_check` **before** that envelope's checksum binds (so the
   hash-bound envelope honestly carries the rule-check outcome), and the runner
   `analysis_status` is then derived from the envelope (single source of truth).
   An unrecognized non-`None` aggregate is a **blocking diagnostic**
   (`HEADLESS_RUNNER_RULE_CHECK_STATUS_INVALID`), never silently dropped, and the
   envelope is left conservatively at `RULE_INPUTS_INCOMPLETE` (no false pass).

Decoupling: the bridge takes the aggregate as its vocabulary `&str` (e.g.
`RuleCheckStatus::as_str()` output), so `core/runner/headless` gains **no new
crate dependency** on `rule_check_runner`. The worst-of aggregation already lives
in the runner; this consumes the final aggregate only.

## Changes (one file: `core/runner/headless/src/lib.rs`)

- Added `pub fn analysis_status_for_rule_check`.
- Added `pub fn run_preview_in_memory_with_rule_check`; `run_preview_in_memory`
  delegates with `None`.
- Success path: `mechanics` is now `mut`; the optional aggregate is driven into
  `mechanics.status.rule_check` before checksum/`analysis_status` composition;
  the rule-check `analysis_status` push is derived via the new helper (covering
  all three statuses, not just `RuleInputsIncomplete`); the success-path
  `RunnerResult.diagnostics` now carries any unrecognized-aggregate blocking
  diagnostic instead of being unconditionally empty.

No change to `core/rules/rule_check_runner` (DEL-06-02) or
`core/reporting/result_export` (DEL-08-04): the runner already emits the
aggregate; the export enum already carries the vocabulary. The blocked-before-
solve path and every existing boundary/validation rule are unchanged.

## Evidence

- `cargo test` (`core/runner/headless`): **16 pass** (+5 over the 11 baseline):
  `analysis_status_for_rule_check_maps_the_runner_vocabulary` (3 mapped + 3
  non-mapped incl. empty);
  `preview_bridge_drives_user_rule_failed_into_envelope_and_analysis_status`
  (envelope `rule_check == USER_RULE_FAILED`; `analysis_status` gains
  `UserRuleFailed`, drops `RuleInputsIncomplete`; result still validates);
  `…_drives_user_rule_checked_into_analysis_status`;
  `preview_bridge_incomplete_aggregate_equals_the_no_aggregate_default`
  (explicit `RULE_INPUTS_INCOMPLETE` ≡ `None`); and
  `preview_bridge_blocks_unrecognized_rule_check_aggregate` (blocking diagnostic
  emitted, envelope left conservative). The existing 11 tests (incl.
  `preview_bridge_executes_product_physics_with_deterministic_refs`) re-pass
  unchanged.
- `cargo fmt --check` (`core/runner/headless`): clean.
- Five-surface DEC-025 sweep: see the committed sweep summary (the cargo crate
  sweep re-runs all crates; pytest, desktop Vitest+wasm, Playwright e2e ×2, and
  the production build are unaffected — the change is isolated to the
  `core/runner/headless` crate, which is not consumed by the frontend/wasm/pytest
  build graphs).

### UI evidence posture (H4 amendment)
No SMOKE / Playwright / Vitest entry: this is a non-GUI core crate
(`core/runner/headless`, DEL-10-05) that is **not consumed by the desktop app**
(the Tauri app depends on `product_physics` and `rule_check_runner`, not the
headless runner). There is no user-visible desktop behavior to smoke, so the
evidence is the Rust unit tests + the cargo sweep — the same posture the prior
crate-only tranches used (e.g. `TP-UNITS-B1-CATALOG-001`, `TP-D03-SPARSE-001`).

## Residuals and hand-offs

- The C4 non-GUI residual "driving `aggregate_status` into the solve envelope /
  result_export" is now **closed at the assembly point**: the non-GUI bridge can
  drive a rule-check aggregate into the solve envelope and the runner
  `analysis_status` that references the result-export envelope.
- Remaining C4 scope (unchanged): the **future additive `acceptability_relation`
  / solver-result-selector schema members** — additive schema work, likely
  ratification-gated (DEC-038-style), not started here.
- The live Tauri app still surfaces the aggregate only in the GUI run panel
  (the C4 GUI slice). Wiring the aggregate into an app-held/exported envelope
  would be a separate GUI-coupled tranche; out of scope for this non-GUI residual.

## Boundary compliance

Single core crate; pure mapping + in-memory bridge; no network/daemon/telemetry/
filesystem (the crate's own invariants, preserved). Status-vocabulary-only — only
the three automatic rule-check statuses are produced; no
compliance/certification/sealing/authentication/approval/code-compliance or
professional-acceptance claim, and `HUMAN_REVIEW_REQUIRED` preservation is
unchanged. Deliverables stay `CHECKING`. Git/test evidence is source-control
hygiene only.

## Open decisions awaiting human ruling

- None blocks the current spine. Register items D-06, D-10b, D-04b, D-05b, D-07b,
  D-11, D-12 remain `NOT_PREPARED` future-phase decisions (Phase E / R5 /
  governance). No decision packet is currently `AWAITING_RULING`.
