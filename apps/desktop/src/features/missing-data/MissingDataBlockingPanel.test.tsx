// R9: the Issues total counts the blocking missing-data entries; the drawer's Missing data panel
// shows that same number in its heading and lists those entries, and keeps the entries that do
// not block under one closed disclosure that says how many and that they are not counted.
import { fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import { buildMissingDataBlockingPacket, countMissingDataBlockers, countRuleInputsMissing, MissingDataBlockingPanel, RULE_INPUT_WARNING_CLASS } from "./MissingDataBlockingPanel";

it("heads the panel with the count the Issues total takes, lists the blocking entries, and keeps the rest apart, closed", async () => {
  const model = await loadPreviewModel();
  const packet = buildMissingDataBlockingPacket({ model, result: null });
  const blocking = packet.warnings.filter((warning) => warning.blocks_mechanics_solve || warning.blocks_rule_check);
  const others = packet.warnings.filter((warning) => !warning.blocks_mechanics_solve && !warning.blocks_rule_check);
  // The fixture has both kinds, so the test reads both.
  expect(blocking.length).toBeGreaterThan(0);
  expect(others.length).toBeGreaterThan(0);
  expect(countMissingDataBlockers({ model, result: null })).toBe(blocking.length);

  render(<MissingDataBlockingPanel model={model} result={null} />);
  expect(screen.getByTestId("missing-data-blocking-count")).toHaveTextContent(String(blocking.length));
  const list = screen.getByTestId("missing-data-warnings");
  const apart = screen.getByTestId("missing-data-not-blocking") as HTMLDetailsElement;
  // The blocking entries are listed outside the disclosure.
  const listed = [...list.querySelectorAll('[data-testid^="missing-data-warning-"]')].filter((entry) => !apart.contains(entry));
  expect(listed).toHaveLength(blocking.length);
  expect(apart.open).toBe(false);
  expect(screen.getByTestId("missing-data-not-blocking-toggle")).toHaveTextContent(
    `${others.length} more ${others.length === 1 ? "entry does" : "entries do"} not block; not counted in Issues`
  );
  fireEvent.click(screen.getByTestId("missing-data-not-blocking-toggle"));
  expect(apart.open).toBe(true);
  expect(within(apart).getAllByTestId(/^missing-data-warning-/)).toHaveLength(others.length);
});

it("counts as rule inputs missing only the rule-check entries, not a missing mechanics input that also blocks the rule check", async () => {
  const base = await loadPreviewModel();
  // Both statuses at once: the mechanics inputs and the rule inputs are incomplete.
  const model = { ...base, analysis_status: { ...base.analysis_status, mechanics: "MODEL_INCOMPLETE", rule_check: "RULE_INPUTS_INCOMPLETE" } };
  const packet = buildMissingDataBlockingPacket({ model, result: null });
  const mechanics = packet.warnings.find((warning) => warning.warning_id === "solve-required-physical-inputs");
  // The mechanics entry blocks the rule check too, so a count of every rule-check blocker takes it.
  expect(mechanics?.blocks_rule_check).toBe(true);
  const ruleInputs = packet.warnings.filter((warning) => warning.warning_class === RULE_INPUT_WARNING_CLASS);
  expect(ruleInputs.map((warning) => warning.warning_id)).toContain("rule-check-required-inputs");
  expect(countRuleInputsMissing({ model, result: null })).toBe(ruleInputs.length);
  expect(countRuleInputsMissing({ model, result: null })).toBe(packet.summary.rule_check_blocking_count - 1);

  // The entries the count counts are the ones the panel marks with their class, for the line's control to reach.
  render(<MissingDataBlockingPanel model={model} result={null} />);
  const marked = screen.getByTestId("missing-data-panel").querySelectorAll(`[data-warning-class="${RULE_INPUT_WARNING_CLASS}"]`);
  expect(marked).toHaveLength(ruleInputs.length);
  expect(screen.getByTestId("missing-data-warning-solve-required-physical-inputs")).toHaveAttribute("data-warning-class", "SOLVE_BLOCKING");
});
