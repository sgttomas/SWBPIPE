---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-APPAGG-001
timestamp: 2026-06-14T18:05:00-0600
completed: 2026-06-14T18:55:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-APPAGG-001 — wire the GUI rule-check aggregate into the app-held analysis-run envelope (C4 app-side residual)

## Tranche and authority basis

- Tranche: the completion-plan **C4** residual **"app-side wiring of the GUI run
  aggregate into an app-held/exported envelope"** — the earliest unblocked item
  on the R3/Phase C dependency spine after the non-GUI assembly residual closed
  (`TP-C4-AGGSTATUS-001`, which itself named this as "a separate GUI-coupled
  tranche; out of scope for this non-GUI residual"). Pre-approved as a
  completion-plan unblocked item per the session entry prompt. No human decision
  gates it; the rule-check status vocabulary is frozen (the three automatic
  statuses).
- Problem: the GUI worst-of rule-check `aggregate_status`
  (`RULE_INPUTS_INCOMPLETE` / `USER_RULE_CHECKED` / `USER_RULE_FAILED`) produced
  by the C4 run panel lived **only** in that panel's ephemeral state. The
  app-held analysis-run envelope (DEL-14-02, built by
  `previewService.buildAnalysisRunPreview`) composed its `analysis_status` from
  `result.status.rule_check`, which a plain solve always leaves at
  `RULE_INPUTS_INCOMPLETE` (the solve runs no user rule checks). So after a user
  ran rule checks in the GUI and got `USER_RULE_CHECKED` / `USER_RULE_FAILED`,
  the app-held envelope — and its downstream consumers `ResultExportPanel` and
  `ReportPanel`, which both read `run.analysis_status` — still reported
  `RULE_INPUTS_INCOMPLETE`. This is the exact app-side analog of the headless
  envelope gap `TP-C4-AGGSTATUS-001` closed.
- The coupled **future additive `acceptability_relation` / solver-result-selector
  schema members** are explicitly out of scope here (a separate, likely
  ratification-gated item).

## Design decision (bounded; frontend only; no schema/ruling)

The vocabulary already lines up — the panel aggregate and the envelope
`analysis_status` use the same three frozen rule-check strings, and
`analysis_status` is already a `string[]` that carries them. The gap was purely
the **app-side assembly**: nothing lifted the aggregate out of the panel into the
app-held envelope. So this is a frontend wiring change, no schema/vocabulary
change anywhere:

1. `previewService.appliedRuleCheckStatus(solveRuleCheck, ruleCheckAggregate?)`
   — new pure, exported helper (mirrors the headless
   `analysis_status_for_rule_check`): a recognized aggregate supersedes the solve
   envelope's `rule_check`; an absent or unrecognized aggregate falls back to the
   solve envelope's own `rule_check` — **no silent coercion, no false pass**
   (CONTRACT no-silent-defaults).
2. `buildAnalysisRunPreview(result, ruleCheckAggregate?)` gains an optional
   second argument (omitting it reproduces the prior behavior byte-for-byte). A
   recognized aggregate is composed into the analysis-run **record's own** status
   — the status bound by the `analysis_run_record` hash and surfaced in
   `analysis_status`. The embedded `result_envelope` hash still binds the **raw
   solve** (`canonicalJson(result)`, unchanged), so the hash-bound solve envelope
   is **never mutated**. (Two hash scopes, each honest about what it binds: the
   solve envelope as solved; the analysis-run record as "this solve + the
   rule-check run executed against it" — exactly the `TP-C4-AGGSTATUS-001`
   pattern, where the aggregate is driven into the record before its checksum
   binds.)
3. `RuleCheckRunPanel` gains an optional `onAggregateChange(aggregate | null)`
   callback (backward compatible). It fires with the worst-of aggregate on a
   successful desktop run, and with `null` whenever there is no current run
   outcome (new pack loaded/pasted, browser-only unavailable seam, or a run
   error).
4. `App` holds the lifted aggregate, and `handleRuleCheckAggregate` rebuilds the
   app-held `analysisRun` from `(result, aggregate)`. The aggregate is reset
   wherever the analysis-run is cleared/rebuilt (fresh solve, model edit,
   computed-state clear, blank-create, project open) so it never goes stale
   relative to the current solve.

## Changes (frontend only)

- `apps/desktop/src/services/previewService.ts`: added `appliedRuleCheckStatus`
  and the frozen `RULE_CHECK_RUN_STATUSES` set; `buildAnalysisRunPreview` takes
  the optional aggregate and composes `recordStatus` into the
  `analysis_run_record` hash and `analysis_status`; the `result_envelope` hash is
  untouched.
- `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`: optional
  `onAggregateChange` prop; fired on run success / cleared on new pack, browser
  seam, and run error.
- `apps/desktop/src/App.tsx`: `ruleCheckAggregate` state (idempotence-guarded),
  `handleRuleCheckAggregate` (rebuilds `analysisRun`; reverts on a hashing
  failure rather than surfacing a false outcome), prop wiring, and aggregate
  resets at every analysis-run clear/rebuild/restore site.

No change to `core/rules/rule_check_runner` (DEL-06-02) — it already emits the
aggregate — or to `core/reporting/result_export` (DEL-08-04) — its
`analysis_status` already carries the vocabulary and is fed from
`run.analysis_status`. The blocked-before-solve path and every existing
boundary/validation rule are unchanged.

## Evidence

- Desktop Vitest: previewService gains 6 tests (`appliedRuleCheckStatus`
  recognized/fallback/no-false-pass; `buildAnalysisRunPreview` default,
  recognized-aggregate-drives-status-with-byte-stable-result-envelope-hash,
  USER_RULE_CHECKED, unrecognized-aggregate-reproduces-no-aggregate-envelope);
  RuleCheckRunPanel gains 2 tests (lifts the aggregate on a desktop run; clears
  on new pack and on the desktop-only browser seam). Full desktop Vitest **365**
  (+8 over the 357 baseline). `tsc -b` clean.
- The key correctness assertion: with a recognized aggregate, the
  `result_envelope`-scope hash is **byte-identical** to the no-aggregate envelope
  (the raw solve is never mutated) while the `analysis_run_record`-scope hash and
  `analysis_status` reflect the rule-check outcome; an unrecognized aggregate
  reproduces the no-aggregate envelope exactly (no false pass).
- Five-surface DEC-025 sweep PASS — see the committed sweep summary.

### UI evidence posture (H4 amendment)
No new Playwright e2e assertion. The end-to-end aggregate→envelope flow requires
the desktop Tauri backend: running rule checks (`run_rule_checks`) is
desktop-only, and in browser preview a rule-check run returns the explicit
unavailable seam (so it produces **no** aggregate, and the app-held envelope
stays solve-only — unchanged from before this tranche). The browser Playwright
harness therefore structurally cannot exercise the flow, and the browser-visible
behavior (the desktop-only rule-check seam) is unchanged. Both composition halves
are instead covered by Vitest: the panel lift (RuleCheckRunPanel desktop-mode
mocked-`invoke` test) and the envelope threading (`buildAnalysisRunPreview` pure
test). This is the same posture the prior non-browser-reachable seam used
(`TP-C4-AGGSTATUS-001`). The App-level connective tissue (the handler rebuilding
`analysisRun` from result+aggregate) is simple prop+handler wiring between two
unit-covered endpoints; it is not separately automated because the App test
harness runs in browser-fixture mode (no Tauri backend → rule checks
unavailable).

## Residuals and hand-offs

- Remaining C4 (non-GUI), unchanged: the **future additive
  `acceptability_relation` / solver-result-selector schema members** — additive
  schema work, likely ratification-gated (DEC-038-style), not started here.
- `ReportPanel` still shows the **solve envelope's** `result.status.rule_check`
  on its "Rule check" line (correct for that line — it describes the solve). A
  follow-up could additionally surface the app-held analysis-run rule-check
  status; left as a small optional hand-off, not a gap.
- Pre-existing test flake (not introduced here): `App.test.tsx > "round trips
  review-only proposal operations through local save and open"` is the heaviest
  full-App round-trip test and straddles the 10s global `testTimeout` under
  full-suite CPU load (measured 8.3s with headroom in isolation / at a 30s
  timeout; ~10.2–10.6s under load). Proven independent of this tranche: it passes
  on the clean tree and with this tranche's changes when not CPU-starved. Flagged
  as a hardening candidate (raise this one test's bound or split it); **not**
  altered as part of this C4 wiring tranche.

## Boundary compliance

Frontend/local-only; pure status composition + in-memory envelope build; no
network/daemon/telemetry/filesystem beyond the existing local seams.
Status-vocabulary-only — only the three automatic rule-check statuses are
produced, and the conservative fallback prevents any false pass;
`HUMAN_REVIEW_REQUIRED` is still always present in the analysis-run status set; no
compliance/certification/sealing/authentication/approval/code-compliance or
professional-acceptance claim. The raw solve envelope's hash is never mutated.
Deliverables stay `CHECKING`. Git/test evidence is source-control hygiene only.

## Open decisions awaiting human ruling

- None blocks the current spine. Register items D-06, D-10b, D-04b, D-05b, D-07b,
  D-11, D-12 remain `NOT_PREPARED` future-phase decisions (Phase E / R5 /
  governance). No decision packet is currently `AWAITING_RULING`.
