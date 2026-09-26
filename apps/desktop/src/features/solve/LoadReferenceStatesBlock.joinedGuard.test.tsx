import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { MechanicsResult, PreviewModel } from "../../types";
import { initialSolveJob } from "../workspace/solveJobAudit";

// Defence in depth for T1's declared rule that a joined load-reference-source-1
// result is never Current: even if the standing function reported it eligible,
// the block still labels every joined case as needing recompute. The standing
// function is stubbed only here; every other test uses WP2's real standing.
vi.mock("../results/numericalResultQuality", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../results/numericalResultQuality")>();
  return { ...actual, numericalResultStanding: () => ({ contract: "load_reference_source", status: "integrity_checked", eligible: true, findings: [] }) };
});
const { SolvePanel } = await import("./SolvePanel");
const { INTEGRITY_CHECKED, NEEDS_RECOMPUTE_NOT_CURRENT } = await import("./LoadReferenceStatesBlock");
import request from "../../../../../fixtures/product_preview/load_reference_source/mixed.request.json";
import raw from "../../../../../fixtures/product_preview/load_reference_source/mixed-sparse_interactive.raw.json";

afterEach(cleanup);

it("labels joined cases needs recompute even against an eligible standing report", async () => {
  const model = structuredClone(request.model) as unknown as PreviewModel;
  const result = structuredClone(raw) as unknown as MechanicsResult;
  render(<SolvePanel analysisRun={null} model={model} result={result} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive" onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
  await waitFor(() => expect(screen.getByTestId("load-reference-state-standing-case:ordinary-pressure")).toBeInTheDocument(), { timeout: 20_000 });
  for (const id of ["case", "case:ordinary-pressure"])
    expect(screen.getByTestId(`load-reference-state-standing-${id}`)).toHaveTextContent(`Numerical standing: ${NEEDS_RECOMPUTE_NOT_CURRENT}`);
  expect(screen.getByTestId("load-reference-states-block")).not.toHaveTextContent(INTEGRITY_CHECKED);
});
