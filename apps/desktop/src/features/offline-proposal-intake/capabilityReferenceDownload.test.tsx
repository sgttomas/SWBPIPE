// DEL-07-08 REQ-014 makes the capability list and the operation schema downloadable. Regression
// expectations of this test: the link holds the static reference, the same bytes whatever project
// is open and after the person has opened the checked example, and the display still includes the
// checked example for the open model. Invented content only.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import operationSchema from "../../../../../schemas/model_operation.schema.json";
import { reconnectModel } from "../../test-support/reconnectModel";
import type { PreviewModel } from "../../types";
import { clearReconnectExampleCache, operationExamples } from "../geometry-tools/reconnectExample";
import { toolkitCapabilities } from "../toolkit/capabilityCatalog";
import { OfflineProposalIntakePanel } from "./OfflineProposalIntakePanel";

afterEach(cleanup);

const capabilityReference = {
  capabilities: toolkitCapabilities,
  operation_schema: operationSchema,
  provider_status: "Imported proposals require explicit human review; live availability is shown by the native agent panel."
};
/** What the download must hold, taken before anything is rendered with the reference. */
const expectedBytes = JSON.stringify(capabilityReference, null, 2);

/** A second project, materially different: its own identity, coordinates and labels. */
function otherModel(): PreviewModel {
  const model = reconnectModel();
  model.project = { ...model.project, id: "project:invented-other", name: "Invented other project" };
  model.nodes = model.nodes.map((node, index) => ({ ...node, label: `Other ${index}`, position: { ...node.position, z: 0.5 } }));
  model.pipe_segments = model.pipe_segments.map((pipe) => ({ ...pipe, label: `Invented other ${pipe.label}` }));
  return model;
}

/** The bytes the download link holds, after the checked example has been opened for `model`. */
async function downloadAfterOpening(model: PreviewModel): Promise<{ bytes: string; shown: string }> {
  render(
    <OfflineProposalIntakePanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={() => undefined}
      capabilityReference={capabilityReference} loadOperationExamples={() => operationExamples(model)} />
  );
  const details = screen.getByText("Offline authoring reference").closest("details")!;
  details.open = true;
  fireEvent(details, new Event("toggle"));
  // Wait for the checked example itself: the static schema already holds the words `reconnect_pipe_run`.
  await waitFor(() => {
    const shown = JSON.parse(screen.getByTestId("capability-reference-json").textContent ?? "null");
    const batch = shown.operation_examples.reconnect_pipe_run.batch;
    expect(JSON.parse(batch.operations[0].change.before).label).toBe(model.pipe_segments[0].label);
  });
  const link = screen.getByRole("link", { name: "Download capability and schema reference" });
  expect(link).toHaveAttribute("download", "piping-operation-capabilities.json");
  const href = link.getAttribute("href") ?? "";
  expect(href.startsWith("data:application/json;charset=utf-8,")).toBe(true);
  const result = { bytes: decodeURIComponent(href.slice(href.indexOf(",") + 1)), shown: screen.getByTestId("capability-reference-json").textContent ?? "" };
  cleanup();
  return result;
}

describe("capability and schema reference download", () => {
  it("holds the static reference for different open projects after the checked example is opened, and the example stays on screen", async () => {
    clearReconnectExampleCache();
    const first = reconnectModel();
    const second = otherModel();
    const a = await downloadAfterOpening(first);
    const b = await downloadAfterOpening(second);
    expect(a.bytes).toBe(expectedBytes);
    expect(b.bytes).toBe(expectedBytes);
    // The checked example is shown on screen for the open model.
    for (const [shown, model] of [[a.shown, first], [b.shown, second]] as const) {
      const reference = JSON.parse(shown);
      const example = reference.operation_examples.reconnect_pipe_run;
      expect(example.batch.operations[0].target.ref).toBe("pipe:P");
      expect(JSON.parse(example.batch.operations[0].change.before).label).toBe(model.pipe_segments[0].label);
    }
    expect(b.shown).toContain("Invented other P");
  });
});
