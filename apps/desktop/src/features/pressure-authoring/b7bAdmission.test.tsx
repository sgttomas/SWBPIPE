import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { PressureAuthoringPanel, pressureModeText } from "./PressureAuthoringPanel";
import { pressureProfileTarget } from "./pressureContract";
import { selectedPropertyRows } from "../model-workspace/modelView";
import { evaluateModelDocumentLocal } from "../../services/projectService";
import type { PreviewModel, EditorOperationIntent } from "../../types";
import fixture from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";

vi.mock("../../services/hashService", () => ({
  canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value))
}));
afterEach(cleanup);
function model(): PreviewModel {
  const model = structuredClone(fixture) as PreviewModel;
  model.schema_version = "0.5.0";
  delete model.pressure_contract;
  for (const pipe of model.pipe_segments) {
    pipe.section.corrosion_allowance = { value: 0, unit: "m" };
  }
  return model;
}

it("B7b preserves authored null presence in the profile display and before projection", async () => {
  const current = model();
  current.pressure_contract = null as unknown as PreviewModel["pressure_contract"];
  expect(pressureModeText(current)).toContain("invalid authored pressure contract");
  const queue = vi.fn();
  render(<PressureAuthoringPanel model={current} selection={{ type: "project", id: current.project.id }} onQueueIntent={queue} />);
  fireEvent.change(screen.getByLabelText("Pressure profile"), { target: { value: "exact_pressure_v3" } });
  fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent;
  expect(JSON.parse(intent.change.before)).toEqual({ schema_version: "0.5.0", pressure_contract: null });
  expect(JSON.parse(intent.change.after)).toEqual(pressureProfileTarget(current));
  expect(current.pressure_contract).toBeNull();
});

it("B7b never supplies an absent user allowance during pressure activation", () => {
  const current = model();
  delete current.pipe_segments[0].section.corrosion_allowance;
  const before = structuredClone(current);
  const queue = vi.fn();
  render(<PressureAuthoringPanel model={current} selection={{ type: "project", id: current.project.id }} onQueueIntent={queue} />);
  fireEvent.change(screen.getByLabelText("Pressure profile"), { target: { value: "exact_pressure_v3" } });
  expect(screen.getByRole("button", { name: "Queue pressure profile" })).toBeDisabled();
  expect(screen.getByText(/Enter an explicit corrosion allowance/)).toBeInTheDocument();
  expect(queue).not.toHaveBeenCalled();
  expect(current).toEqual(before);
  const rows = selectedPropertyRows(current, { type: "pipe", id: current.pipe_segments[0].id });
  expect(rows.some(([label]) => label === "Corrosion allowance")).toBe(false);
});

it("B7b saves incomplete current document bytes by version and refuses all retired versions", () => {
  const current = model();
  current.pressure_contract = null as unknown as PreviewModel["pressure_contract"];
  delete current.pipe_segments[0].section.corrosion_allowance;
  const before = structuredClone(current);
  expect(evaluateModelDocumentLocal(current).status).toBe("current");
  expect(current).toEqual(before);
  for (const version of ["0.1.0", "0.2.0", "0.3.0", "0.4.0"]) {
    const retired = structuredClone(current);
    retired.schema_version = version;
    expect(evaluateModelDocumentLocal(retired).status).toBe("retired_schema");
  }
});
