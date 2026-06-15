---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-ACCEPTREL-001
timestamp: 2026-06-14T19:45:00-0600
completed: 2026-06-14T20:25:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-ACCEPTREL-001 — additive `acceptability_relation` on a CheckDefinition (C4 dependency-spine tail)

## Tranche and authority basis

- Tranche: completion-plan **C4** named remaining scope (non-GUI) — "the future
  additive `acceptability_relation` / solver-result-selector schema members
  (additive schema work, likely ratification-gated)". This slice lands the
  **`acceptability_relation`** half; the `solver-result-selector` member stays a
  named follow-up.
- Selection: the earliest unblocked item on the R3/Phase C dependency spine. C1
  is landed; C2/C3/C4 *named* residuals are closed; the two R3-exit blocking
  residuals (F-4 packaged human journey; A3 authoring-journey usability) are
  human-gated and not agent-closable. This is the one genuinely-unblocked
  current-stage spine item: an additive, backward-compatible schema member built
  as a PROPOSAL following the established additive-member precedent
  (`library_value_ref` added by `TP-C3C4-LIBREF-001`, ratified by `DEC-038`).
- Problem: rule checks could only ever express `≤`. The runner hard-coded
  `ComparisonOperator::LessThanOrEqual` and the outcome label
  `"less_than_or_equal"`; a pack could not state that a check passes when the
  computed quantity is `≥` / `<` / `>` its limit (e.g. a minimum-thickness or
  margin check). The frozen grammar (DEC-022) already supports all six
  comparison operators and the `rule_pack_document` crate already has a
  string↔operator codec — only the runner's top-level acceptability comparison
  was fixed.

## Design decision (additive schema member — awaiting human ratification)

`CheckDefinition` is `additionalProperties: false`, so naming the relation
requires a new member. Added an **optional, additive** `acceptability_relation`
(string enum of the four ordering relations: `less_than`, `less_than_or_equal`,
`greater_than`, `greater_than_or_equal`) to `CheckDefinition` in
`schemas/rule_pack.schema.yaml`. It is **labelled PROPOSAL** and surfaced here
for human ratification (companion to `DEC-038`); ratification will move the
rule-pack `schema_version` 0.3.0 → 0.4.0 under the `DEC-033` additive-minor
policy (the bump rides ratification, exactly as the 0.2.0→0.3.0 bump rode
`DEC-038` for `library_value_ref` — so the draft template still declares
`0.3.0` for now).

Decisions inside the slice, all surfaced (no silent choices):

- **Direction**: the check passes when `computed_value <relation> limit_value`
  (computed is the formula result, limit is the value-slot). This matches the
  historical `computed <= limit`.
- **Absent → `less_than_or_equal`**: the sole documented default, preserving the
  behaviour of every pack authored before the member existed (the demo and all
  existing fixtures). It is a backward-compatibility default for a *missing*
  member, not a silent fill of a blank a user left in a new pack — the editor
  and draft template author the member explicitly.
- **Unrecognized token blocks**: an explicit `acceptability_relation` that is not
  one of the four ordering relations is never coerced to `≤`; the runner emits a
  blocking `RULE_EVALUATOR_ERROR` finding and the check blocks at
  `RULE_INPUTS_INCOMPLETE` (no-silent-defaults, CONTRACT).
- **Ordering relations only**: floating-point equality/inequality acceptance
  (`equal`/`not_equal`) is a deliberate non-goal of this member (fragile on
  floats; a boolean-predicate formula can express equality directly). The
  grammar retains all six operators for use inside formula expressions.

## Changes

### Schema — `schemas/rule_pack.schema.yaml`
- `CheckDefinition.properties.acceptability_relation` (optional; four-ordering-
  relation enum) with a description of the direction, the absent→default, the
  grammar mapping, and the equality non-goal. Not added to `required`
  (backward compatible).

### Runner — `core/rules/rule_check_runner/src/lib.rs`
- `resolve_acceptability_relation(check) -> Result<ComparisonOperator, String>`
  (absent/empty → `LessThanOrEqual`; the four ordering tokens → their operator;
  any other explicit token → `Err(token)`) and
  `acceptability_relation_label(&ComparisonOperator) -> &'static str`.
- Quantity-vs-limit branch: resolve the relation up front; on `Err` push a
  blocking `RULE_EVALUATOR_ERROR` evaluator finding + `evaluator_error`
  diagnostic and return `RULE_INPUTS_INCOMPLETE` with relation label `"none"`.
  Otherwise use the resolved operator in the synthesized `Compare` and report
  the resolved label on the outcome (the limit-missing / limit-bad blocked
  returns now also report the resolved label rather than the hard-coded `≤`).
- Module doc-comment and the `CheckOutcome.acceptability_relation` field comment
  updated to describe the four relations and the default.

### GUI editor — `apps/desktop/src/features/rule-packs/CheckDefinitionsEditor.tsx`
- Exported `ACCEPTABILITY_RELATIONS` vocabulary (the four ordering relations,
  copied verbatim from the schema). `defaultCheckDefinition` now authors
  `acceptability_relation: "less_than_or_equal"`. A new `EnumSelect`
  (`rule-pack-check-acceptability-relation`) lets the user choose the relation;
  out-of-vocabulary stored tokens surface as `(current) …` (no silent snap),
  the same no-silent-default posture as the other check selectors.

### Draft template — `apps/desktop/src/services/rulePackService.ts`
- `buildDraftRulePackDocument()` check authors `acceptability_relation:
  "less_than_or_equal"` (schema_version stays `0.3.0` until ratification).

## Evidence

- `cargo test -p open_pipe_stress_rule_check_runner` (from the crate dir):
  **11 unit + 4 new integration + 3 demo integration = 18 pass.** New
  `tests/acceptability_relation_run.rs`: `>=` flips the demo pass/fail both ways;
  `<` is strict at the boundary (ratio 1.0 fails `< 1.0`); explicit `<=`
  reproduces the absent default; unrecognized `between` blocks with the evaluator
  finding (no silent `<=`). The 3 `invented_demo_run` tests are unchanged →
  backward compatibility witnessed.
- `cargo fmt --check` (runner crate): clean (after `cargo fmt`).
- `python3 -m pytest tests/test_rule_pack_schema.py`: **5 pass** (schema valid
  with the additive member; demo/example fixtures conform — they omit it).
- `npm test --workspace apps/desktop` (Vitest): **367 pass** (+2 net:
  CheckDefinitionsEditor "authors the relation through the selector" and "shows
  the less_than_or_equal default for a pack with no relation, without mutating
  it"; the default-shape and vocabulary-equality tests extended;
  RuleCheckRunPanel outcome now asserts `relation=less_than_or_equal` renders).
- `npm run build --workspace apps/desktop`: clean (1653 modules).
- `npx playwright test -g "rule-pack manager"`: **2/2** (chromium-desktop +
  chromium-compact). The r2-smoke spec now asserts the relation selector shows
  the `less_than_or_equal` default and that choosing `greater_than_or_equal`
  rewrites the canonical document JSON — the changed behaviour exercised in a
  real browser (H4 evidence posture).
- Five-surface DEC-025 sweep: committed sweep summary at the tranche HEAD.

## Residuals and hand-offs

- **Schema ratification** — `acceptability_relation` is a PROPOSAL additive
  member awaiting a human `DEC` (companion to `DEC-038`); ratification bumps the
  rule-pack `schema_version` 0.3.0 → 0.4.0 (`DEC-033` additive-minor) and the
  draft template's declared version with it.
- **`solver-result-selector`** — the other half of the C4 non-GUI remaining
  scope (a schema selector tying a `solver_result` input to a solved result row,
  retiring the caller-supplied binding) is still a named follow-up.
- **Equality acceptance** — `equal`/`not_equal` as a top-level acceptability
  relation is intentionally out of scope; revisit only on a human request.

## Boundary compliance

Local-only (pure in-memory runner + invented test fixtures; no
network/daemon/telemetry/private-data writes). Status-vocabulary-only
(`RULE_INPUTS_INCOMPLETE` / `USER_RULE_CHECKED` / `USER_RULE_FAILED`); no
compliance/certification/sealing/authentication/approval/code-compliance or
professional-acceptance claim. Deliverables stay `CHECKING`. Git/test evidence
is source-control hygiene only, not lifecycle issuance.

## Open decisions awaiting human ruling

- **Schema ratification** of the additive `acceptability_relation` member
  (PROPOSAL; companion to `DEC-038`).
- Pre-existing, unrelated: **D-06**, **D-10b**, **D-04b**, **D-05b**, **D-07b**,
  **D-11**, **D-12** remain `NOT_PREPARED` in the decision register. The two
  R3-exit blocking residuals (F-4 packaged human journey; A3 authoring-journey
  usability) remain human-gated.
