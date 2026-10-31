import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { JointPressureModelEditor } from "./JointPressureModelEditor";
import fixture from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";
import type { PreviewModel } from "../../types";
vi.mock("../../services/hashService", () => ({ canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)) }));
afterEach(cleanup);
function model(version: string): PreviewModel {
  return { ...structuredClone(fixture), schema_version: version, pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3" },
    components: [{ id: "component:J", label: "Invented joint", kind: "expansion_joint", node: "node:A", provenance: "invented", objective_connector: { version: "1.0.0", pressure_model: { kind: "unpressurized" } } }] } as unknown as PreviewModel;
}
it("queues an explicit area model through the common edit seam on 0.9.0 only", async () => {
  const queue = vi.fn();
  render(<JointPressureModelEditor model={model("0.9.0")} selection={{ type: "component", id: "component:J" }} onQueueIntent={queue} />);
  const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
  change("Joint pressure model kind", "constant_effective_area_reference_v1");
  change("Joint effective area value", "0.004"); change("Joint effective area unit", "m^2");
  change("Joint pressure basis", "internal_differential_zero_external_v1"); change("Joint area source reference", "invented explicit area");
  fireEvent.click(screen.getByRole("button", { name: "Queue joint pressure model" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0];
  expect(intent.change.field_path).toBe("objective_connector.pressure_model");
  expect(JSON.parse(intent.change.after)).toEqual({ kind: "constant_effective_area_reference_v1", effective_area: { value: 0.004, unit: "m^2" }, pressure_basis: "internal_differential_zero_external_v1", source_reference: "invented explicit area" });
  cleanup();
  render(<JointPressureModelEditor model={model("0.4.0")} selection={{ type: "component", id: "component:J" }} onQueueIntent={queue} />);
  expect(screen.queryByRole("button", { name: "Queue joint pressure model" })).toBeNull();
});
