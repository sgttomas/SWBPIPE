import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { healthStanding } from "../../test-support/resultOverlayFixtures";
import { L1, keyModel, keyOutcome, keyResult, keyRun } from "../../test-support/keyResultsFixtures";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import type { ResultOverlayStanding } from "./numericalHealth";
import { KEY_RESULTS_CANVAS_HIDDEN, KeyResultsPanel } from "./KeyResultsPanel";
import {
  KEY_CONTACT_NONE,
  KEY_RATIO_ABSENT,
  KEY_RATIO_NOT_PLACED,
  KEY_RESULTS_KEYS,
  KEY_RESULTS_NO_RESULT,
  keyRatioIncompleteText,
  keyRatioNotRunText,
  type KeyResultsActionId
} from "./keyResults";

// A6, the key-results panel (DEL-07-05 Design `key-results-panel.md` §2, §4, §5, §7): each action
// calls exactly its one setter with the ids the view already offers, by button and by key alike;
// absent and incomplete packs read as such; and the panel changes no model. All data are invented.

const PASSING = healthStanding({ eligible: true });
const CALLBACKS = ["onShowRatioContour", "onShowDeflectedShape", "onShowSupport", "onShowContactStatus", "onOpenTable", "onBackToEditing", "onOpenRuleChecks", "onClose", "onEverySolveChange"] as const;
type Spies = Record<(typeof CALLBACKS)[number], ReturnType<typeof vi.fn<(...args: unknown[]) => void>>>;

function renderPanel(options: {
  model?: PreviewModel;
  result?: MechanicsResult | null;
  standing?: ResultOverlayStanding | null;
  run?: SessionRuleCheckRun | null;
  canvasHidden?: boolean;
} = {}) {
  const spies = Object.fromEntries(CALLBACKS.map((name) => [name, vi.fn<(...args: unknown[]) => void>()])) as Spies;
  const model = options.model ?? keyModel();
  const before = JSON.stringify(model);
  render(
    <KeyResultsPanel
      model={model}
      result={options.result === undefined ? keyResult() : options.result}
      standing={options.standing === undefined ? PASSING : options.standing}
      ruleCheckRun={options.run ?? null}
      canvasHidden={options.canvasHidden}
      everySolve={false}
      {...spies}
    />
  );
  return { spies, model, before };
}

const panel = () => screen.getByTestId("key-results-panel");
const button = (action: KeyResultsActionId) => screen.getByTestId(`key-results-${action}`);
const status = () => screen.getByTestId("key-results-status");

/** The one callback that was called, with its arguments; every other spy stays silent. */
function onlyCall(spies: Spies): [string, unknown[]] {
  const called = CALLBACKS.filter((name) => spies[name].mock.calls.length > 0);
  expect(called).toHaveLength(1);
  expect(spies[called[0]].mock.calls).toHaveLength(1);
  return [called[0], spies[called[0]].mock.calls[0]];
}

const EXPECTED: Record<KeyResultsActionId, [string, unknown[]]> = {
  ratio: ["onShowRatioContour", ["rule-ratio:rule:cat_a", "load_case:load:L-1"]],
  deflection: ["onShowDeflectedShape", ["load_case:load:L-1"]],
  support: ["onShowSupport", ["support:S-A"]],
  contact: ["onShowContactStatus", []],
  "results-table": ["onOpenTable", ["results"]],
  "stress-ratio-table": ["onOpenTable", ["stress-ratio"]],
  "displacement-extremes": ["onOpenTable", ["displacement-extremes"]],
  edit: ["onBackToEditing", []]
};

describe("KeyResultsPanel actions", () => {
  for (const action of Object.keys(EXPECTED) as KeyResultsActionId[]) {
    it(`${action}: the button and the key ${KEY_RESULTS_KEYS[action]} each call only ${EXPECTED[action][0]}, with the same arguments`, () => {
      const byButton = renderPanel({ run: keyRun() });
      fireEvent.click(button(action));
      expect(onlyCall(byButton.spies)).toEqual(EXPECTED[action]);
      const buttonStatus = status().textContent;
      cleanup();

      const byKey = renderPanel({ run: keyRun() });
      button("results-table").focus();
      fireEvent.keyDown(document.activeElement!, { key: KEY_RESULTS_KEYS[action].toLowerCase() });
      expect(onlyCall(byKey.spies)).toEqual(EXPECTED[action]);
      expect(status().textContent).toBe(buttonStatus);
      expect(JSON.stringify(byKey.model)).toBe(byKey.before);
    });
  }

  it("acts on the chosen category, case and support, and keeps focus on the button pressed", () => {
    const { spies } = renderPanel({ run: keyRun() });
    fireEvent.change(screen.getByTestId("key-results-deflection-case"), { target: { value: "load_case:load:L-2" } });
    fireEvent.change(screen.getByTestId("key-results-support-choice"), { target: { value: "support:S-C" } });
    expect(spies.onShowDeflectedShape).not.toHaveBeenCalled();
    button("deflection").focus();
    fireEvent.click(button("deflection"));
    expect(spies.onShowDeflectedShape).toHaveBeenCalledWith("load_case:load:L-2");
    expect(document.activeElement).toBe(button("deflection"));
    expect(status()).toHaveTextContent("Deflected shape: Invented two (load:L-2).");
    fireEvent.keyDown(document.activeElement!, { key: "S" });
    expect(spies.onShowSupport).toHaveBeenCalledWith("support:S-C");
    expect(status()).toHaveTextContent("Support load summary: Nozzle C, in Results.");
  });

  it("leaves keys to the selects and the checkbox, and to Alt, Ctrl and Meta chords", () => {
    const { spies } = renderPanel({ run: keyRun() });
    fireEvent.keyDown(screen.getByTestId("key-results-deflection-case"), { key: "d" });
    fireEvent.keyDown(screen.getByTestId("key-results-every-solve"), { key: "r" });
    fireEvent.keyDown(button("ratio"), { key: "r", altKey: true });
    fireEvent.keyDown(button("ratio"), { key: "r", metaKey: true });
    fireEvent.keyDown(button("ratio"), { key: "r", ctrlKey: true });
    for (const name of CALLBACKS) expect(spies[name]).not.toHaveBeenCalled();
  });

  it("closes on Escape and from the close button, and offers each action's key to assistive technology", () => {
    const { spies } = renderPanel();
    for (const [action, key] of Object.entries(KEY_RESULTS_KEYS)) expect(button(action as KeyResultsActionId)).toHaveAttribute("aria-keyshortcuts", key);
    expect(panel()).toHaveAttribute("role", "region");
    expect(panel()).toHaveAccessibleName("Key results");
    fireEvent.keyDown(button("edit"), { key: "Escape" });
    expect(spies.onClose).toHaveBeenCalledTimes(1);
    fireEvent.click(screen.getByRole("button", { name: "Close key results" }));
    expect(spies.onClose).toHaveBeenCalledTimes(2);
  });

  it("says when the canvas the view draws on is hidden", () => {
    renderPanel({ canvasHidden: true });
    fireEvent.click(button("deflection"));
    expect(status()).toHaveTextContent(KEY_RESULTS_CANVAS_HIDDEN);
  });
});

describe("KeyResultsPanel: absent, incomplete and unavailable", () => {
  it("shows an absent pack as absent: no category, no ratio, and the action says why", () => {
    const { spies } = renderPanel();
    expect(screen.getByTestId("key-results-ratio-note")).toHaveTextContent(KEY_RATIO_ABSENT);
    expect(screen.getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "absent");
    expect(screen.queryByTestId("key-results-ratio-category")).toBeNull();
    expect(button("ratio")).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByTestId("key-results-ratio-reason")).toHaveTextContent(KEY_RATIO_ABSENT);
    fireEvent.click(button("ratio"));
    fireEvent.keyDown(button("ratio"), { key: "R" });
    expect(spies.onShowRatioContour).not.toHaveBeenCalled();
    expect(status()).toHaveTextContent(KEY_RATIO_ABSENT);
    // No ratio, zero or pass is stated anywhere for the absent pack.
    const ratioGroup = within(screen.getByRole("group", { name: "Ratio contours" }));
    expect(ratioGroup.queryByText(/\b0\b|pass/iu)).toBeNull();
  });

  it("offers the rule-check route when the model references a pack that has not been run", () => {
    const { spies } = renderPanel({ model: keyModel({ rulePackRefs: 1 }) });
    expect(screen.getByTestId("key-results-ratio-note")).toHaveTextContent(keyRatioNotRunText(1));
    fireEvent.click(screen.getByTestId("key-results-rule-checks"));
    expect(onlyCall(spies)).toEqual(["onOpenRuleChecks", []]);
  });

  it("shows an incomplete pack as incomplete: the run's status, the count, the declared category without outcomes and each category's incomplete count", () => {
    renderPanel({ run: keyRun() });
    expect(screen.getByTestId("key-results-ratio-aggregate")).toHaveTextContent("Rule pack · Rule inputs incomplete");
    expect(screen.getByTestId("key-results-ratio-aggregate")).toHaveAttribute("data-status-token", "RULE_INPUTS_INCOMPLETE");
    expect(screen.getByTestId("key-results-ratio-incomplete")).toHaveTextContent(keyRatioIncompleteText(1, 4));
    expect(screen.getByTestId("key-results-ratio-declared-without-outcomes")).toHaveTextContent("Invented unused (cat_none)");
    const options = within(screen.getByTestId("key-results-ratio-category")).getAllByRole("option").map((option) => option.textContent);
    expect(options).toEqual(["Invented A (cat_a): 2 outcomes, 1 incomplete", "Invented B (cat_b): 1 outcome", "Invented range (cat_r): 1 outcome, 1 withheld"]);
  });

  it("offers a category whose outcomes are all incomplete, and draws its legend through the contour, not a value here", () => {
    const { spies } = renderPanel({ run: keyRun([keyOutcome("chk_a", "cat_a", L1, "RULE_INPUTS_INCOMPLETE", { in_p1: undefined })]) });
    expect(screen.getByTestId("key-results-ratio-incomplete")).toHaveTextContent(keyRatioIncompleteText(1, 1));
    fireEvent.click(button("ratio"));
    expect(spies.onShowRatioContour).toHaveBeenCalledWith("rule-ratio:rule:cat_a", "load_case:load:L-1");
  });

  it("says why a category the contour cannot place is not drawn", () => {
    const { spies } = renderPanel({ run: keyRun() });
    fireEvent.change(screen.getByTestId("key-results-ratio-category"), { target: { value: "rule-ratio:rule:cat_b" } });
    expect(button("ratio")).toHaveAttribute("aria-disabled", "true");
    fireEvent.keyDown(button("ratio"), { key: "r" });
    expect(spies.onShowRatioContour).not.toHaveBeenCalled();
    expect(status()).toHaveTextContent(KEY_RATIO_NOT_PLACED);
  });

  it("says a result without support-state rows has none", () => {
    renderPanel({ result: keyResult(keyResult().results.filter((row) => row.kind !== "nonlinear_support_active_set_state_code")) });
    expect(button("contact")).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByTestId("key-results-contact-reason")).toHaveTextContent(KEY_CONTACT_NONE);
  });

  it("without a result offers no view, but still the tables and editing", () => {
    const { spies } = renderPanel({ result: null, standing: null });
    expect(screen.getByTestId("key-results-reason")).toHaveTextContent(KEY_RESULTS_NO_RESULT);
    for (const action of ["ratio", "deflection", "support", "contact"] as const) {
      expect(button(action)).toHaveAttribute("aria-disabled", "true");
      fireEvent.click(button(action));
    }
    expect(spies.onShowRatioContour).not.toHaveBeenCalled();
    expect(spies.onShowDeflectedShape).not.toHaveBeenCalled();
    expect(spies.onShowSupport).not.toHaveBeenCalled();
    expect(spies.onShowContactStatus).not.toHaveBeenCalled();
    fireEvent.click(button("results-table"));
    expect(spies.onOpenTable).toHaveBeenCalledWith("results");
  });

  it("states a withheld result's standing and leaves the withholding to the views", () => {
    const { spies } = renderPanel({ standing: { eligible: false, health: null } });
    expect(screen.getByTestId("key-results-standing")).toHaveTextContent("Withheld: the result's numerical standing does not admit it.");
    fireEvent.click(button("deflection"));
    expect(spies.onShowDeflectedShape).toHaveBeenCalledWith("load_case:load:L-1");
  });
});
