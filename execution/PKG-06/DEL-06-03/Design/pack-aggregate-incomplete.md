# Pack-level aggregate when a check is incomplete (FR-RULE-003)

**Status:** decided, not yet implemented. **Basis:** `origin/main` at
`b9d3111321` (2026-10-24). Standard claim fence applies (claims taxonomy per
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

## Where things stand

`core/rules/rule_check_runner/src/lib.rs` aggregates worst-of, with
`USER_RULE_FAILED` ranked above `RULE_INPUTS_INCOMPLETE` (`severity_rank`). A
pack with one failed and one incomplete check therefore headlines
`USER_RULE_FAILED`. It never headlines `USER_RULE_CHECKED` while a check is
incomplete, but the failed headline reads as a pack-level fail. The desktop
shows the aggregate in `RuleCheckRunPanel.tsx` ("Aggregate status").

## Change

The runner's aggregate is `RULE_INPUTS_INCOMPLETE` whenever any check is
incomplete. The run result carries the failed, incomplete and checked counts,
so the headline can state them. Every surface that shows the aggregate uses
the headline above. A pack with no incomplete check is unchanged: failed if any
check failed, otherwise checked.

The implementation, its tests (failed and incomplete, incomplete only, all
checked) and the as-built record follow in the implementing change.
