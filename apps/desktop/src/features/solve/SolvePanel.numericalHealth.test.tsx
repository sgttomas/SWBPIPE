// DEL-07-05 numerical health §4: the Solve panel's diagnostics tile carries the header
// badge (with its text, not colour alone) for a held result, and none without one.
import { render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { numericalHealthView, readNumericalHealth } from "../results/numericalHealth";
import { numericalResultStanding } from "../results/resultStanding";
import { SolvePanel } from "./SolvePanel";
import type { MechanicsResult } from "../../types";

it("adds the header badge to the diagnostics tile", async () => {
  const model = await loadPreviewModel();
  const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json"), "utf8")) as MechanicsResult;
  const health = numericalHealthView(readNumericalHealth(result, numericalResultStanding(result).findings), { eligible: false, verdictHold: false });
  const props = { model, result, analysisRun: null, running: false, solveJob: initialSolveJob(),
    solverMode: "dense_scrutiny" as const, onRun() {}, onCancel() {}, onSolverModeChange() {} };
  const view = render(<SolvePanel {...props} numericalHealth={health} />);
  const badge = screen.getByTestId("solve-numerical-health-badge");
  expect(badge).toHaveTextContent("Numerical health: 1 sensitive");
  expect(badge).toHaveAttribute("data-badge", "red");
  expect(screen.getByTestId("readiness-diagnostics")).toContainElement(badge);
  view.rerender(<SolvePanel {...props} numericalHealth={null} />);
  expect(screen.queryByTestId("solve-numerical-health-badge")).toBeNull();
});
