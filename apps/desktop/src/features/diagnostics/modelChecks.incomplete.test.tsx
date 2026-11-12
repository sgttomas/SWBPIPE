// A check whose library-reference collection fails keeps the engine's answer
// (DEL-07-07 REQ-008: the engine's answer is kept when a later group fails): the
// solve group's blocking finding stands, and the library group reads as not
// completed. The invented model is the bundled one with pipe P-100's material emptied.
import { render, within } from "@testing-library/react";
import { expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { readSolveGate } from "../workspace/solveGates";
import { ModelChecksPanel } from "./ModelChecksPanel";
import { modelChecksNotCompleted, runModelChecks } from "./modelChecks";

it("keeps the engine's findings when the library-reference collection throws, and marks only the library group not completed", async () => {
  const model = JSON.parse(JSON.stringify(await loadPreviewModel())) as PreviewModel;
  model.pipe_segments.find((pipe) => pipe.id === "pipe:P-100")!.material = "";
  const run = await runModelChecks(model, () => { throw new Error("invented library store failure"); });
  expect(run.solve).toEqual({ kind: "checked" });
  expect(run.findings.filter((item) => item.group === "solve").map((item) => [item.diagnostic.code, item.diagnostic.severity])).toContainEqual(["MATERIAL_INPUT_MISSING", "blocking"]);
  expect(run.findings.some((item) => item.group === "library")).toBe(false);
  expect(run.libraryNotCompleted).toBe("invented library store failure");
  expect(readSolveGate(model, run).kind).toBe("withheld");

  const view = render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
  expect(view.getByTestId("model-checks-library-not-completed")).toHaveTextContent("The library-reference findings could not be collected (invented library store failure); that group is not complete.");
  const library = view.container.querySelector('[data-group="library"]') as HTMLElement;
  expect(within(library).getByText("not completed")).toBeInTheDocument();
  expect(view.queryByTestId("model-checks-solve-unchecked")).toBeNull();
});

it("says a check that started and failed did not complete, not that it did not run", async () => {
  const model = await loadPreviewModel();
  const view = render(<ModelChecksPanel model={model} run={modelChecksNotCompleted(model, new Error("invented failure"))} onRun={() => {}} onSelectEntity={() => {}} />);
  expect(view.getByTestId("model-checks-solve-unchecked")).toHaveTextContent("The model checks did not complete: invented failure");
  expect(view.getByTestId("model-checks-solve-unchecked")).not.toHaveTextContent("did not run");
});
