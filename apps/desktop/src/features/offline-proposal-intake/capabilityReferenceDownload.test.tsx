// The capability and schema reference download is the capability list and the operation schema
// (DEL-07-08 REQ-014): the same bytes whatever project is open, also after the person has opened the
// checked example, which stays on screen for the open model. Invented content only.
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
  await waitFor(() => expect(screen.getByTestId("capability-reference-json")).toHaveTextContent("reconnect_pipe_run"));
  const link = screen.getByRole("link", { name: "Download capability and schema reference" });
  expect(link).toHaveAttribute("download", "piping-operation-capabilities.json");
  const href = link.getAttribute("href") ?? "";
  expect(href.startsWith("data:application/json;charset=utf-8,")).toBe(true);
  const result = { bytes: decodeURIComponent(href.slice(href.indexOf(",") + 1)), shown: screen.getByTestId("capability-reference-json").textContent ?? "" };
  cleanup();
  return result;
}

describe("capability and schema reference download", () => {
  it("is the same bytes for different open projects, after the checked example is opened, which stays on screen", async () => {
    clearReconnectExampleCache();
    const first = reconnectModel();
    const second = otherModel();
    const a = await downloadAfterOpening(first);
    const b = await downloadAfterOpening(second);
    expect(a.bytes).toBe(b.bytes);
    expect(a.bytes).toBe(JSON.stringify(capabilityReference, null, 2));
    for (const download of [a.bytes, b.bytes]) {
      const parsed = JSON.parse(download);
      expect(Object.keys(parsed).sort()).toEqual(["capabilities", "operation_schema", "provider_status"]);
      for (const projectText of ["project:w10-session", "project:invented-other", "Invented other P", "W-10 session"]) {
        expect(download).not.toContain(projectText);
      }
    }
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
