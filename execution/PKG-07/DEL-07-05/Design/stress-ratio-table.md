# Sorted stress and ratio table

This design covers survey item A3: per stress category, the result locations sorted by the user's
rule ratio, or by stress magnitude where no rule-check run is held, with a threshold, a stress or
ratio toggle and a row that highlights its location. Paths are relative to
`projects/chirality-piping/`. The standard claim fence applies (F-PIP-2). A ratio here is
arithmetic on two values of the user's own rule-check outcome. Sorting and filtering are a view of
emitted rows and of the run's outcomes. Neither is a qualification, an acceptance or a code check.

## 1. Basis and scope

HELP_HUMAN adopted A3 under the owner's 2026-10-27 delegation (#1662). The CAEPIPE survey
(#1657, `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`) recommends it because the
first question after a solve is "where is it worst?", and the desktop answered it only by scanning
the per-row results table. It serves FR-SOL-008 (recovered stresses) and keeps FR-RULE-003
(incomplete checks stay visible as incomplete and are never sorted or filtered as passing). It
touches DEL-06-03 (the rule-check run). This deliverable owns the viewer slice. W-17's key-results
panel (A6) may embed it, so the derivation and the component have a stated contract (§8).

In scope: the derivation, the view rule (sort and threshold), the panel, its placement in the
Results section, and highlighting a location through the session's selection. Out of scope:

- **Export of the table** is item D2. The rows keep stored values, units, result-row ids and
  outcome identities (§3), so D2 can write them without a second derivation. D2 is an output
  surface under `apps/desktop/src/features/results/outputPolicy.ts`. This panel is a view, like
  the viewport overlays and A2's panel, and is not in `OUTPUT_SURFACES`.
- **Writing outcomes or ratios into a report or package** is W-18b's later slice. The panel
  persists, exports and sends nothing.
- **Allowables.** The product supplies none. A limit shown here is the limit the user's own rule
  pack reported for that outcome.

## 2. Data sources

| Source | Where the desktop holds it | Used for |
|---|---|---|
| Emitted stress rows | `MechanicsResult.results`, by `kind` (§2.1) | Stress groups, sorted by magnitude |
| Case labels | `supportLoadCases(model, result)` (A2) | The case column and the envelope flag |
| Numerical standing | `viewportResultStanding` in `App.tsx` (`{ eligible, health }`) | Withholding (§4) |
| The rule-check run | `ruleCheckReportRun` of the results session (`SessionRuleCheckRun`: the loaded pack and `RuleCheckRunResult`) | Rule groups (§2.2) |
| The pack's binding plan | `deriveRuleCheckBindingPlan(run.rulePackDocument)` (`ruleCheckService.ts`) | Category names, each check's solver inputs and their selectors |

### 2.1 Stress kinds

The product's result rows carry no sustained, expansion or occasional category. Those are the
user's own rule-pack categories (FR-RULE-001; `stress_categories` in the pack). Without a run, a
group is one emitted stress kind. `STRESS_KINDS` in `stressRatioTable.ts` lists, with a label, every
kind that a pinned semantic contract gives the `stress` family
(`fixtures/results/semantic_contract_v0_3_*.json`: preview-physics-2, pressure-2,
pressure-source-1 and the two retained routes; 17 kinds). A test reads those fixtures and fails if a
stress-family kind is missing from the list. Kinds absent from a result offer no group. The list is
explicit, as `RESULT_COLOUR_QUANTITIES` is, so the derivation needs no reader call per row.

A row's location is its `entity_ref` (pipe, component or element), its station
(`metadata.location`), its `metadata.component` (for example `sigma_plus`) and the section basis
named in `metadata.basis` (`nominal_section` or `reduced_section` on the exact route). A range-
envelope row (`metadata.basis: "explicit_user_range_envelope"`) is listed and labelled "range
envelope". It is one location's value selected from the envelope's operands.

### 2.2 Rule-check outcomes

A run holds one outcome per check and basis (DEL-06-01 Design `rule-pack-0.5.0.md` §2.7):
`check_id`, `category_ref` (the user's stress category), `basis_ref`, `status`
(`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED` or `USER_RULE_FAILED`), `computed_value`,
`limit_value`, `acceptability_relation`, `bound_inputs`, and the completeness and evaluator
findings. A rule group is one `category_ref`, labelled with the pack's category name, in the
pack's category order. Categories the outcomes name but the pack does not declare follow, in order
of appearance.

**Location of an outcome.** The check's solver inputs are its `required_input_refs` that are
`solver_result` inputs of the pack. For each, the bound input's `result_id` names the row the runner
bound. Where it is a row of the shown result, the location is that row's. Otherwise the location is
the input's selector (`solver_result_ref`: entity, station, component, section basis). A check with
several solver inputs lists each location. The highlight target is the first bound row, or else the
first selector's entity.

**The run belongs to this result.** App passes the run only while the panel's result is the
session's current solved result (`currentSolvedResult`), and the session retires the run on every
new solve, model edit and session change (`handleRuleCheckAggregate`, the `ruleCheckRunBasis` key).
As a further guard, if any bound solver `result_id` is not a row of the shown result, the run's
groups are not built, and the panel says "The rule-check run does not belong to this result; its
outcomes are not shown."

Nothing else is needed. The panel does not touch the solver, the rule-check runner, the model
document schema or `product_physics`.

## 3. Rows

Every value is a stored value with its stored unit, unchanged.

- **Stress row:** the case, the location, the value and unit, the result-row id, and its standing.
- **Outcome row:** the outcome identity (`ruleCheckOutcomeIdentity`), the check, the case, the
  status, the relation, the computed value and the limit (each with its unit), the ratio or the
  reason none is formed, the locations, the highlight target, the incomplete reason and its standing.

### 3.1 The ratio

The ratio is `computed_value ÷ limit_value` of the outcome, formed only when:

1. the relation is an upper bound, `less_than` or `less_than_or_equal` (the runner's default);
2. both values are present and finite;
3. both are stated in one unit (`unit_ref`) and dimension; values are not converted;
4. the limit is greater than zero.

Otherwise the row says why no ratio is formed: a lower-bound relation (`greater_than`,
`greater_than_or_equal`), a predicate check (`formula_predicate`), a missing value, units that
differ, or a limit that is not positive. For an upper-bound relation and a positive limit, the
ratio exceeds 1 exactly when the computed value exceeds the limit, so it ranks outcomes in the
direction of the user's rule. The status shown is always the runner's own. The ratio does not
re-derive it. A rounded ratio of 1 can sit beside either status.

### 3.2 Incomplete reason

An incomplete outcome states: the missing inputs (`bound_inputs` not supplied), then its
completeness and evaluator findings as "CODE (subject): message", or else its diagnostic codes,
or else "The run states no reason."

## 4. Numerical standing and withheld rows

Rows are admitted only by the viewport overlays' rule, `overlayMayShowRow(standing, row)` in
`numericalHealth.ts`, reused and not restated. An outcome is admitted by its `basis_ref`.

- A withheld stress row carries no value in the view. The view counts withheld rows per case and
  states each case once with its reason, A2's texts ("Withheld: numerical health {state}." and the
  others; `caseWithheldReason` in `supportLoadSummary.ts`).
- A withheld outcome row carries no value and no status. It states its check, its case and the
  reason. It is pinned (§5) and never sorted or hidden.
- An incomplete outcome is shown as incomplete whatever its standing. If its case is also withheld,
  its values are not shown.

The retained routes have no numerical health view, so all of their rows are withheld.

## 5. View rule: sort, threshold and toggle

`stressRatioView(group, { mode, threshold })` is pure. It splits a group's rows into three parts,
shown in this order:

1. **Pinned.** Incomplete outcomes, then withheld outcomes, each in run order, with their reasons.
   They are never sorted, never compared with the threshold and never hidden. For a stress group,
   the withheld cases are stated here.
2. **Ranked.** Rows with a sort key, sorted by the key, largest first.
3. **Not ranked.** Admitted outcomes with no key (no ratio formed in ratio mode, no computed value
   in stress mode), in run order. They are never hidden.

**Keys.** Ratio mode: the ratio. Stress mode for a rule group: the magnitude of the computed value.
Stress group: the magnitude of the stored value; the cell shows the signed value.

**Ties.** Equal keys put a failed outcome before a checked one, and then keep source order (run
order for outcomes, result-row order for stress rows). 0 and -0 tie.

**Units.** A key compares only within one stored unit. If a group's keyed rows have more than one
unit in stress mode, the ranked part keeps source order, the view says "Units differ (…); not
sorted and no threshold is applied", and the threshold is disabled. Ratios are unitless.

**Threshold.** The user enters a number. A row is shown when its key is at or above it (≥). In
stress mode the number is in the display unit (MPa for a value stored in Pa), and it is converted to
the stored unit by a decimal shift of the entered text (1 MPa = 10^6 Pa), then compared with the
stored value. The threshold hides only ranked stress rows and ranked outcomes the rule check reports
as `USER_RULE_CHECKED`. It never hides an incomplete, withheld, failed or not-ranked row. Text that
is not a number applies no threshold, and the view says so. The view counts the rows hidden by the
threshold, and the panel states it.

**Toggle.** A rule group offers ratio and stress. Ratio mode shows the ratio and the relation.
Stress mode shows the computed value and the limit. A stress group offers stress only, and the
ratio choice is disabled with its reason: "No rule-check run in this session" or "Emitted stress:
no rule outcome". Each mode keeps its own threshold text.

## 6. Display rule

Values are shown under the report's display rule, `displayQuantityParts` and `displayQuantity` in
`features/report/reportQuantityDisplay.ts` (#1655): six significant digits, half away from zero,
ECMAScript layout, Pa shown as MPa. That covers stresses, computed values, limits and ratios (a
ratio is printed as a number with no unit). Cells, titles and attributes follow it, and no text or
attribute of the panel shows a value otherwise. A title gives the displayed number and unit and the
result-row id or outcome identity. Where a column's shown values share one unit, the header carries
it. The stored values stay in the derivation's rows for D2.

## 7. Placement and interaction

`StressRatioTablePanel` (`features/results/StressRatioTablePanel.tsx`) sits in the Results section
below A2's panel and is hidden while a reopened historical run is shown, as A2's is.

- **Controls.** A category select (rule categories, then emitted stresses, as option groups), a
  "Sort and show by" radio group (Ratio, Stress), and a "Show at or above" number field with its
  unit.
- **Highlight.** Each row's location is a button named "Highlight {location}, {case}". Activating it (click, Enter or Space) calls
  `onSelectLocation(target)`. App maps a result-row id to `handleSelectResult` (the results table's
  own path: the review target and the entity selection) and an entity reference to
  `resolveEntitySelection` and `handleSelectEntity`. The row whose result-row id is the session's
  selected result is marked (`aria-selected`, `.selected-result-row`).
- **Length.** The ranked part shows 50 rows, and "Show 50 more" extends it. Pinned and not-ranked
  rows are always shown.
- **Accessibility.** The table is labelled "{group}: sorted by {key}". The key column header has
  `aria-sort="descending"` when sorted. Column and row headers are `th` with scope. A polite live
  line states "Showing n of m rows; k below the threshold hidden." An empty cell reads "none" to a
  screen reader.
- **Context lines.** The numerical health header and output sentence when present, the run line
  (pack id and outcome count, or that no run is held), the key's definition, and the boundary
  sentence.

## 8. Composition contract (A6)

```ts
// stressRatioTable.ts
export function buildStressRatioTable(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;  // App's viewportResultStanding
  ruleCheckRun?: SessionRuleCheckRun | null;           // only a run of this result
}): StressRatioTable;
// StressRatioTable = { state: "available" | "unavailable"; reason; runNote; groups: StressRatioGroup[] }
// StressRatioGroup = { id, source: "rule_category" | "emitted_stress", label, modes, rows, withheldCases }
export function stressRatioView(group: StressRatioGroup, options: { mode: StressRatioMode; threshold: string }): StressRatioView;
// StressRatioView = { mode, keyUnit, sorted, unitNote, threshold, pinned, ranked, unranked, hiddenCount, shownCount, totalCount }

// StressRatioTablePanel.tsx
export function StressRatioTablePanel(props: {
  model: PreviewModel; result: MechanicsResult | null; standing: ResultOverlayStanding | null | undefined;
  ruleCheckRun?: SessionRuleCheckRun | null;
  selectedResultId?: string | null;
  onSelectLocation?: (target: StressRatioTarget) => void;  // { resultId, entityRef }
  testIdPrefix?: string;                                   // default "stress-ratio"
}): JSX.Element;
```

A6 passes the inputs it already has. A key-results headline can call `buildStressRatioTable` and
`stressRatioView(group, { mode, threshold: "" })` and read `pinned` and `ranked[0]` per group
without rendering the panel; it must show the pinned incomplete rows beside any "worst" row. Every
output is frozen and deterministic for its inputs. The panel derives once per identity of
(`model`, `result`, `standing`, `ruleCheckRun`) and applies the view per control change.

## 9. Claims fence

The panel lists emitted stresses and the user's rule-check outcomes, sorted and filtered. A ratio
is the user's computed value divided by the user's limit. The status is the rule check's own
report, and the panel adds no pass or fail. Incomplete outcomes are never shown, sorted or hidden as
passing. The panel supplies no allowable, records or implies no acceptance, issuance or compliance,
and writes nothing.

## 10. Tests

- `apps/desktop/src/features/results/stressRatioTable.test.ts`: the stress-kind list against the
  semantic-contract fixtures; stress groups sorted by magnitude with signed values, ties in row order
  (and 0 against -0); envelope rows labelled; mixed units unsorted with the threshold disabled; the
  MPa threshold against Pa rows by decimal shift; withheld rows under mixed standing (a sensitive
  case, ineligible standing, no health view); rule groups by category in pack order; the ratio and
  each reason none is formed; incomplete outcomes pinned under every threshold, mode and sort;
  withheld outcomes pinned with no status; failed outcomes never hidden; multiple bases per check;
  tie order (failed before checked, then run order); locations from bound rows and from selectors;
  the run-mismatch guard; no run gives stress groups only; unavailable states.
- `apps/desktop/src/features/results/StressRatioTablePanel.test.tsx`: the display rule in cells,
  titles and headers, with no stored value at full precision in the markup; the toggle and its
  disabled reason; the threshold and the hidden count; pinned incomplete rows with their reasons
  under a threshold above every ratio; highlight by click and keyboard; the selected-row mark;
  "Show 50 more"; a custom test-id prefix; one derivation per input identity.
- Native smoke (`apps/desktop/native-smoke/driver.js`, `run.mjs`): structure only, no values, and
  the committed `expected.json` is unchanged. After each solve, with no run held, the table lists
  emitted stress kinds in stress mode with ratio disabled. After the bundled invented pack's rule
  checks on the demo and the exact route (every outcome `RULE_INPUTS_INCOMPLETE`), it opens on the
  pack's category in ratio mode with every row pinned as incomplete and none ranked.

## 11. Remaining items

- **D2: export of the table**, an output surface under `outputPolicy.ts`.
- **Writing outcomes and ratios into a report** (W-18b).
- **A case filter** and stepping by case (A1's navigator may supply it).
- **Display units.** The panel could follow the session's display-unit preference, as A2 could.
- **Lower-bound ratios.** A lower-bound relation could rank by limit ÷ computed. That is a
  choice of meaning, so no ratio is formed today.
