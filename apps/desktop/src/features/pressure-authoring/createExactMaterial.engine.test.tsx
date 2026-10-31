// G5 (DEL-07-03, DEL-09-01 run sheet §3.2): the E/nu create form reproduces the
// reference system's material RS-M1, including its material-level thermal
// expansion coefficient, through the operation engine (the wasm export apply uses).
// G5b: the coefficient is added to an existing material afterwards, to the same record.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, expect, it, vi } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { computeModelHash } from "../../services/hashService";
import { applyModelOperation, warmupOperationEngine } from "../../services/operationService";
import { buildBlankLoadStateModelDocument } from "../../services/projectService";
import { MaterialConstitutiveForm, PressureAuthoringPanel } from "./PressureAuthoringPanel";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const reference = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/reference_system/reference_system.json"), "utf8")) as PreviewModel;
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
afterEach(cleanup);
beforeAll(async () => {
  expect((await warmupOperationEngine()).state).toBe("ready");
});

it("creates RS-M1 with its thermal expansion coefficient exactly as the reference system records it", async () => {
  const expected = structuredClone(reference.materials![0]) as Record<string, unknown>;
  delete expected.temperature_points; delete expected.expansion_laws;
  const model = buildBlankLoadStateModelDocument(new Date("2026-10-31T00:00:00Z"));
  const queue = vi.fn(); render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: "" }} onQueueIntent={queue} />);
  change("Exact material ID", "material:RS-M1"); change("Exact material label", "RS-M1 invented carbon-steel-like material");
  change("Constitutive basis", "homogeneous_isotropic_E_nu_v1");
  change("Elastic modulus E value", "200"); change("Elastic modulus E unit", "GPa");
  change("Poisson ratio nu value", "0.3"); change("Poisson ratio nu unit", "1");
  change("Constitutive provenance", String(expected.provenance));
  change("Thermal expansion coefficient value", "1.2e-5"); change("Thermal expansion coefficient unit", "1/K");
  fireEvent.click(screen.getByRole("button", { name: "Queue exact material" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const applied = await applyModelOperation(model, queue.mock.calls[0][0] as EditorOperationIntent, await computeModelHash(model));
  expect(applied.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  expect((applied.applied_model as PreviewModel).materials).toEqual([expected]);
});

it("adds RS-M1's coefficient to the existing material afterwards, to the same record (G5b)", async () => {
  const expected = structuredClone(reference.materials![0]) as Record<string, unknown>;
  delete expected.temperature_points; delete expected.expansion_laws;
  const { thermal_expansion_coefficient: alpha, ...withoutAlpha } = expected;
  let model = buildBlankLoadStateModelDocument(new Date("2026-10-31T00:00:00Z"));
  model = { ...model, materials: [withoutAlpha as NonNullable<PreviewModel["materials"]>[number]] };
  const queue = vi.fn(); render(<MaterialConstitutiveForm model={model} selection={{ type: "material", id: "material:RS-M1" }} onQueueIntent={queue} />);
  change("Thermal expansion coefficient value", "1.2e-5"); change("Thermal expansion coefficient unit", "1/K");
  fireEvent.click(screen.getByRole("button", { name: "Queue constitutive properties" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const applied = await applyModelOperation(model, queue.mock.calls[0][0] as EditorOperationIntent, await computeModelHash(model));
  expect(applied.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  expect((applied.applied_model as PreviewModel).materials).toEqual([{ ...withoutAlpha, thermal_expansion_coefficient: alpha }]);
});
