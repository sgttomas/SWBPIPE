import { describe, expect, it } from "vitest";
import { healthStanding, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import { L1, keyModel, keyOutcome, keyOutcomes, keyResult, keyRows, keyRun } from "../../test-support/keyResultsFixtures";
import {
  KEY_CONTACT_NONE,
  KEY_RATIO_ABSENT,
  KEY_RATIO_ENVELOPE_ONLY,
  KEY_RATIO_NOT_PLACED,
  KEY_RATIO_NOT_THIS_RESULT,
  KEY_RESULTS_AUTO_OPEN_START,
  KEY_RESULTS_HISTORICAL,
  KEY_RESULTS_KEYS,
  KEY_RESULTS_NOT_SOLVED,
  KEY_RESULTS_NO_RESULT,
  KEY_SUPPORTS_NONE,
  buildKeyResults,
  keyRatioIncompleteText,
  keyRatioNoOutcomesText,
  keyRatioNotRunText,
  keyResultsAutoOpen,
  keyResultsKeyAction,
  type KeyResultsActionId
} from "./keyResults";
import { SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE } from "./supportLoadSummary";

// A6, the key-results panel's contents (DEL-07-05 Design `key-results-panel.md` §2 to §4). All data
// are invented (`keyResultsFixtures`); no value has engineering meaning.

const PASSING = healthStanding({ eligible: true });

describe("key results: ratio categories come only from the user's rule-check run", () => {
  it("shows an absent pack as absent: no category, no ratio and no zero", () => {
    const built = buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING });
    expect(built.ratio).toMatchObject({ state: "absent", note: KEY_RATIO_ABSENT, aggregate: null, incompleteNote: null });
    expect(built.ratio.categories).toEqual([]);
  });

  it("says a referenced pack has not been run on this result, and lists nothing from it", () => {
    const built = buildKeyResults({ model: keyModel({ rulePackRefs: 2 }), result: keyResult(), standing: PASSING });
    expect(built.ratio).toMatchObject({ state: "not_run", note: keyRatioNotRunText(2) });
    expect(built.ratio.categories).toEqual([]);
    expect(keyRatioNotRunText(1)).toContain("1 rule pack,");
  });

  it("lists each category of the run with its counts, the first case the contour can draw, and the run's incompleteness", () => {
    const built = buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING, ruleCheckRun: keyRun() });
    expect(built.ratio.state).toBe("used");
    expect(built.ratio.aggregate).toEqual({ token: "RULE_INPUTS_INCOMPLETE", text: "Rule pack · Rule inputs incomplete" });
    expect(built.ratio.incompleteNote).toBe(keyRatioIncompleteText(1, 4));
    expect(built.ratio.declaredWithoutOutcomes).toEqual(["Invented unused (cat_none)"]);
    const [a, b, r] = built.ratio.categories;
    expect(a).toMatchObject({
      quantityId: "rule-ratio:rule:cat_a", label: "Invented A (cat_a)", outcomeCount: 2, incompleteCount: 1, withheldCount: 0,
      caseKey: "load_case:load:L-1", caseLabel: "Invented one (load:L-1)", unavailableReason: null
    });
    expect(b).toMatchObject({ quantityId: "rule-ratio:rule:cat_b", caseKey: null, unavailableReason: KEY_RATIO_NOT_PLACED });
    expect(r).toMatchObject({ quantityId: "rule-ratio:rule:cat_r", caseKey: null, unavailableReason: KEY_RATIO_ENVELOPE_ONLY });
    expect(built.ratio.categories).toHaveLength(3);
  });

  it("offers a category whose outcomes are all incomplete, counted as incomplete, never as a ratio", () => {
    const run = keyRun([keyOutcome("chk_a", "cat_a", L1, "RULE_INPUTS_INCOMPLETE", { in_p1: undefined })]);
    const built = buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING, ruleCheckRun: run });
    expect(built.ratio.incompleteNote).toBe(keyRatioIncompleteText(1, 1));
    expect(built.ratio.categories).toEqual([expect.objectContaining({ outcomeCount: 1, incompleteCount: 1, caseKey: "load_case:load:L-1" })]);
  });

  it("counts withheld outcomes under a sensitive case", () => {
    const standing = healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } });
    const built = buildKeyResults({ model: keyModel(), result: keyResult(), standing, ruleCheckRun: keyRun() });
    expect(built.ratio.categories[0]).toMatchObject({ withheldCount: 1 });
    expect(built.standingText).toBe(standing.health!.headline);
  });

  it("reports a run of another result, and a run with no outcomes, without categories", () => {
    const other = keyRun([keyOutcome("chk_a", "cat_a", L1, "USER_RULE_CHECKED", { in_p1: "s:not-in-this-result" })]);
    expect(buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING, ruleCheckRun: other }).ratio)
      .toMatchObject({ state: "not_this_result", note: KEY_RATIO_NOT_THIS_RESULT, categories: [] });
    const empty = buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING, ruleCheckRun: keyRun([]) }).ratio;
    expect(empty).toMatchObject({ state: "no_outcomes", note: keyRatioNoOutcomesText("invented_pack"), categories: [] });
    expect(empty.aggregate?.token).toBe("USER_RULE_CHECKED");
  });
});

describe("key results: cases, supports and support states", () => {
  it("offers the viewport's cases, load cases first, and no range envelope", () => {
    const built = buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING });
    expect(built.cases.map((item) => item.key)).toEqual(["load_case:load:L-1", "load_case:load:L-2"]);
  });

  it("lists the anchors and equipment connections, in model order, and no other support", () => {
    const built = buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING });
    expect(built.supports.items).toEqual([
      { supportId: "support:S-A", label: "Anchor A", text: "Anchor A (anchor)" },
      { supportId: "support:S-C", label: "Nozzle C", text: "Nozzle C (equipment connection)" }
    ]);
    const plain = keyModel();
    plain.supports = plain.supports.map((support) => ({ ...support, family: "guide", boundary_association: undefined }));
    expect(buildKeyResults({ model: plain, result: keyResult(), standing: PASSING }).supports).toEqual({ reason: KEY_SUPPORTS_NONE, items: [] });
  });

  it("lists a support by its declared anchor family, never by its restraints", () => {
    const declared = keyModel();
    declared.supports = declared.supports.map((support) => ({ ...support, family: "anchor", restraints: ["UY"], boundary_association: undefined }));
    expect(buildKeyResults({ model: declared, result: keyResult(), standing: PASSING }).supports.items.map((item) => item.text))
      .toEqual(declared.supports.map((support) => `${support.label} (anchor)`));
    const undeclared = keyModel();
    undeclared.supports = undeclared.supports.map(({ family: _family, ...support }) => ({ ...support, restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], boundary_association: undefined }));
    expect(buildKeyResults({ model: undeclared, result: keyResult(), standing: PASSING }).supports).toEqual({ reason: KEY_SUPPORTS_NONE, items: [] });
    expect(KEY_SUPPORTS_NONE).toBe("No support is declared as an anchor or equipment connection.");
  });

  it("counts the support-state rows and their cases, and none when the result states none", () => {
    expect(buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING }).contact).toEqual({ rowCount: 2, caseCount: 2 });
    const linear = keyResult(keyRows().filter((item) => item.kind !== "nonlinear_support_active_set_state_code"));
    expect(buildKeyResults({ model: keyModel(), result: linear, standing: PASSING }).contact).toEqual({ rowCount: 0, caseCount: 0 });
    expect(KEY_CONTACT_NONE).toContain("no nonlinear support state");
  });

  it("states the result's standing as the results panel words it", () => {
    expect(buildKeyResults({ model: keyModel(), result: keyResult(), standing: { eligible: false, health: null } }).standingText).toBe(SUPPORT_LOAD_WITHHELD_NOT_ELIGIBLE);
    expect(buildKeyResults({ model: keyModel(), result: keyResult(), standing: { eligible: true, health: null } }).standingText).toBeNull();
  });

  it("offers no view without a result, for a reopened run, or when mechanics are not solved", () => {
    expect(buildKeyResults({ model: keyModel(), result: null, standing: null })).toMatchObject({ state: "no_result", reason: KEY_RESULTS_NO_RESULT, cases: [] });
    expect(buildKeyResults({ model: keyModel(), result: keyResult(), standing: PASSING, historical: true })).toMatchObject({ state: "historical", reason: KEY_RESULTS_HISTORICAL });
    const failed = { ...overlayResult([row("d:A", "displacement_magnitude", "node:A", 1, "mm")]), status: { mechanics: "MODEL_INCOMPLETE", rule_check: "not_run", professional_acceptance: "not_claimed" } } as never;
    expect(buildKeyResults({ model: keyModel(), result: failed, standing: PASSING })).toMatchObject({ state: "not_solved", reason: KEY_RESULTS_NOT_SOLVED, cases: [] });
  });

  it("is deterministic and frozen, and leaves its inputs unchanged", () => {
    const model = keyModel();
    const result = keyResult();
    const run = keyRun(keyOutcomes());
    const before = JSON.stringify([model, result, run]);
    const first = buildKeyResults({ model, result, standing: PASSING, ruleCheckRun: run });
    expect(buildKeyResults({ model, result, standing: PASSING, ruleCheckRun: run })).toEqual(first);
    expect(Object.isFrozen(first)).toBe(true);
    expect(JSON.stringify([model, result, run])).toBe(before);
  });
});

describe("key results: keys and the auto-open rule", () => {
  it("maps one letter to each action, either case, and nothing with Alt, Ctrl or Meta", () => {
    const letters = Object.values(KEY_RESULTS_KEYS);
    expect(new Set(letters).size).toBe(letters.length);
    for (const [action, key] of Object.entries(KEY_RESULTS_KEYS) as [KeyResultsActionId, string][]) {
      expect(keyResultsKeyAction({ key })).toBe(action);
      expect(keyResultsKeyAction({ key: key.toLowerCase() })).toBe(action);
      expect(keyResultsKeyAction({ key, altKey: true })).toBeNull();
      expect(keyResultsKeyAction({ key, ctrlKey: true })).toBeNull();
      expect(keyResultsKeyAction({ key, metaKey: true })).toBeNull();
    }
    expect(keyResultsKeyAction({ key: "Q" })).toBeNull();
    expect(keyResultsKeyAction({ key: "Enter" })).toBeNull();
  });

  it("opens for the session's first solved result only, unless every solve is chosen", () => {
    const first = keyResult();
    const second = keyResult();
    let step = keyResultsAutoOpen(KEY_RESULTS_AUTO_OPEN_START, { result: first, historical: false, everySolve: false });
    expect(step.open).toBe(true);
    // The same result again (a re-render) opens nothing.
    step = keyResultsAutoOpen(step.memory, { result: first, historical: false, everySolve: true });
    expect(step.open).toBe(false);
    step = keyResultsAutoOpen(step.memory, { result: null, historical: false, everySolve: false });
    expect(step.open).toBe(false);
    step = keyResultsAutoOpen(step.memory, { result: second, historical: false, everySolve: false });
    expect(step.open).toBe(false);
    step = keyResultsAutoOpen(step.memory, { result: keyResult(), historical: false, everySolve: true });
    expect(step.open).toBe(true);
  });

  it("opens for no reopened run, no cleared result and no unsolved result", () => {
    expect(keyResultsAutoOpen(KEY_RESULTS_AUTO_OPEN_START, { result: keyResult(), historical: true, everySolve: true }).open).toBe(false);
    expect(keyResultsAutoOpen(KEY_RESULTS_AUTO_OPEN_START, { result: null, historical: false, everySolve: true }).open).toBe(false);
    const unsolved = { ...keyResult(), status: { mechanics: "MODEL_INCOMPLETE", rule_check: "not_run", professional_acceptance: "not_claimed" } } as never;
    const step = keyResultsAutoOpen(KEY_RESULTS_AUTO_OPEN_START, { result: unsolved, historical: false, everySolve: false });
    expect(step).toEqual({ open: false, memory: { lastResult: unsolved, openedOnce: false } });
  });
});
