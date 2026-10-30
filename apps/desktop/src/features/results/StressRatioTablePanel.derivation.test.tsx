// A3 render cost (DEL-07-05 Design `stress-ratio-table.md` §8): the panel derives its table once per
// (model, result, standing, ruleCheckRun) identity. Control changes (category, mode, threshold),
// the selected row and the test-id prefix reuse it. Invented data only (`resultOverlayFixtures`).
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const derivations = vi.hoisted(() => ({ count: 0 }));
vi.mock("./stressRatioTable", async (original) => {
  const actual = await original<typeof import("./stressRatioTable")>();
  return {
    ...actual,
    buildStressRatioTable: (input: Parameters<typeof actual.buildStressRatioTable>[0]) => {
      derivations.count += 1;
      return actual.buildStressRatioTable(input);
    }
  };
});
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import { StressRatioTablePanel } from "./StressRatioTablePanel";

afterEach(cleanup);

it("derives once per input identity and not on control changes", () => {
  derivations.count = 0;
  const model = overlayModel();
  const result = overlayResult([
    row("a", "element_local_axial_normal_stress", "pipe:P-1", 1, "MPa"),
    row("t", "element_local_torsional_shear_stress", "pipe:P-1", 2, "MPa")
  ]);
  const standing = healthStanding({ eligible: true });
  const view = render(<StressRatioTablePanel model={model} result={result} standing={standing} />);
  expect(derivations.count).toBe(1);
  fireEvent.change(screen.getByTestId("stress-ratio-group"), { target: { value: "stress:element_local_torsional_shear_stress" } });
  fireEvent.change(screen.getByTestId("stress-ratio-threshold"), { target: { value: "1.5" } });
  view.rerender(<StressRatioTablePanel model={model} result={result} standing={standing} selectedResultId="t" />);
  view.rerender(<StressRatioTablePanel model={model} result={result} standing={standing} selectedResultId="t" testIdPrefix="stress-ratio" />);
  expect(screen.getByTestId("stress-ratio-row-t").getAttribute("aria-current")).toBe("true");
  expect(derivations.count).toBe(1);
  view.rerender(<StressRatioTablePanel model={model} result={overlayResult([row("a", "element_local_axial_normal_stress", "pipe:P-1", 3, "MPa")])} standing={standing} />);
  expect(derivations.count).toBe(2);
  view.rerender(<StressRatioTablePanel model={overlayModel()} result={result} standing={standing} />);
  expect(derivations.count).toBe(3);
  view.rerender(<StressRatioTablePanel model={overlayModel()} result={result} standing={healthStanding({ eligible: true })} />);
  expect(derivations.count).toBe(4);
});
