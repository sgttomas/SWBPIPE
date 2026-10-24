# Pack-level aggregate when a check is incomplete (FR-RULE-003)

**Status:** implemented (W-19). **Basis:** `origin/main` at `e893344c19`
(2026-10-24). Standard claim fence applies (claims taxonomy per
DEC-081).

## Requirement and reading

PRD FR-RULE-003 (Must): "Software refuses to report pass/fail when required
rule-pack values are missing."

The owner accepted this reading (2026-10-23: "I accept your other
recommendations for the matters you needed from me"): the pack-level aggregate
must not headline pass or fail while any check is incomplete.
- **Headline:** "INCOMPLETE — n check(s) failed; m check(s) blocked", with the
  real counts.
- **Failures** stay prominent: a failed check is never hidden behind the
  incomplete headline.
- **Per-check results** stay listed, each with its own status.

## Before

`core/rules/rule_check_runner/src/lib.rs` aggregated worst-of, with
`USER_RULE_FAILED` ranked above `RULE_INPUTS_INCOMPLETE`. A pack with one
failed and one incomplete check therefore headlined `USER_RULE_FAILED`.

## As built

- **Runner.** `RuleCheckStatus::aggregate_rank` ranks incomplete (2) above
  failed (1) above checked (0). The pack aggregate is `RULE_INPUTS_INCOMPLETE`
  whenever any check is incomplete; otherwise failed if any check failed,
  otherwise checked. `RuleCheckRunResult::status_counts()` gives the failed,
  incomplete and checked counts. The result JSON and its schema
  (`schemas/rule_check_run_result.schema.json`) keep the same members and
  status vocabulary; only the precedence changed, and the schema description
  states it.
- **Desktop.** `RuleCheckRunPanel.tsx` headlines "INCOMPLETE — n check(s)
  failed; m check(s) blocked (RULE_INPUTS_INCOMPLETE)" and sets `data-status`
  to `RULE_INPUTS_INCOMPLETE` whenever any check is incomplete. A result whose
  checks include a blocked check is given the aggregate `RULE_INPUTS_INCOMPLETE`
  once, on arrival, whatever aggregate it carried. The headline, the run status
  line, the aggregate lifted to the session (and so the app-held analysis-run
  record) and the held run then agree. Each check stays listed with its own
  status, and the run status line gives all three counts.
- **Rendered report.** The "User-Rule Check Outcomes" line reads "Aggregate
  status: INCOMPLETE — n check(s) failed; m check(s) blocked · User-rule-check
  authority · Rule inputs incomplete (RULE_INPUTS_INCOMPLETE)", counted from
  the recorded checks, so a run recorded before this change is headlined the
  same way. The per-check table is unchanged. The analysis-status banner (HTML
  and PDF, `status_banner_text`) follows the same rule: where the recorded
  checks include a blocked check, a recorded `user_rule_failed` or
  `user_rule_checked` is shown as `rule_inputs_incomplete (recorded: …)`,
  keeping the recorded status beside it.
- **Unchanged.** A pack with no incomplete check headlines as before. The
  analysis-status mapping (`core/runner/headless`) and the app-held aggregate
  carry the runner's token, which is now incomplete in the mixed case.

## Tests

- Runner (`rule_check_runner` unit tests): failed and incomplete gives
  incomplete with counts 1/1/1; incomplete only; all checked; failed with none
  incomplete. `tests/point_path_non_finite_run.rs` now expects incomplete for
  its blocked-plus-failed pack.
- Renderer (`report_renderer/tests/render.rs`): the headline for a blocked
  check; a failed and a blocked check under a recorded failed aggregate,
  including the banner for a recorded `user_rule_failed` disclosure; all
  checked. PDF (`pdf_emitter/tests/emit.rs`): the banner correction, with a
  control without rule checks.
- Desktop (`RuleCheckRunPanel.test.tsx`, "FR-RULE-003 pack headline"): failed
  and blocked; a failed aggregate with a blocked check, including the status
  line, the lifted aggregate and the held run; blocked only; all passed.
