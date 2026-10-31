// G7/G8 (DEL-07-03 `Work/authoring-speed.md` §3; DEL-09-01 run sheet §3.8, §3.10, §3.11):
// the reference system's reference-configuration members, its three analysis states
// and its copied pressure regions, entered with the fill and copy controls, then
// queued and applied through the wasm operation engine, equal the fixture's.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeAll, expect, it, vi } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { computeModelHash } from "../../services/hashService";
import { applyModelOperation, warmupOperationEngine } from "../../services/operationService";
import { PressureAuthoringPanel } from "../pressure-authoring/PressureAuthoringPanel";
import { LoadReferenceStateInputs } from "./LoadReferenceStateInputs";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const fixture = JSON.parse(readFileSync(path.join(projectRoot, "fixtures/reference_system/reference_system.json"), "utf8")) as PreviewModel;
const P = "invented reference-system fixture input (DEL-09-01 Work/reference-system.md); not standard, supplier, commercial or project data";
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const click = (name: string) => fireEvent.click(screen.getByRole("button", { name }));
const caseOf = (model: PreviewModel, id: string) => model.load_cases.find(c => c.id === id)!;
afterEach(cleanup);
beforeAll(async () => {
  expect((await warmupOperationEngine()).state).toBe("ready");
});

async function queuedAndApplied(model: PreviewModel, queue: ReturnType<typeof vi.fn>): Promise<PreviewModel> {
  await waitFor(() => expect(queue).toHaveBeenCalledOnce(), { timeout: 20_000 });
  const outcome = await applyModelOperation(model, queue.mock.calls[0][0] as EditorOperationIntent, await computeModelHash(model));
  expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  cleanup();
  return outcome.applied_model as PreviewModel;
}
function renderState(model: PreviewModel, caseId: string) {
  const queue = vi.fn();
  render(<LoadReferenceStateInputs model={model} selection={{ type: "load", id: caseId }} onQueueIntent={queue} loadCase={caseOf(model, caseId)} />);
  return queue;
}
function fill(name: string, targets = "missing") {
  change(`${name} template`, "0");
  change(`${name} fill targets`, targets);
  click(`Fill ${name.toLowerCase()}s`);
}
/** The head of an analysis state, its first support state and its load sources. */
function enterCommon() {
  change("Analysis state contract", "openpipestress.load_reference_state/1.0.0");
  change("Reference configuration", "reference:RS-INSTALLED");
  click("Add support state");
  change("Support state 1 support", "support:RS-A");
  change("Support state 1 participation", "active_model_device");
  fill("Support state");
  change("Factor for every primitive (dimensionless)", "1");
  click("Add every primitive of this case");
  change("History", "independent_equilibrium");
  change("Analysis state provenance", P);
}

it("enters the reference system's load/reference state with fills and copies", async () => {
  // The run sheet's model at §3.8: geometry, supports, self-weight cases and RG regions of
  // RS-SUS only (entered once, §3.10); no reference members past the first, no states.
  let model = structuredClone(fixture);
  model.reference_configurations![0].member_references = model.reference_configurations![0].member_references.slice(0, 1);
  for (const loadCase of model.load_cases) {
    delete (loadCase as { analysis_state?: unknown }).analysis_state;
    if (loadCase.id !== "load:RS-SUS") delete loadCase.pressure_regions;
  }

  // §3.8: one member typed, the other 25 filled.
  let queue = renderState(model, "load:RS-SUS");
  fill("Reference configuration 1 member");
  click("Queue reference configurations");
  model = await queuedAndApplied(model, queue);
  expect(model.reference_configurations).toEqual(fixture.reference_configurations);

  // §3.11, RS-SUS: one element state typed, filled to every pipe.
  queue = renderState(model, "load:RS-SUS");
  click("Add element state");
  change("Element state 1 pipe", "pipe:RS-P010");
  change("Element state 1 material selection", "exact_point");
  change("Element state 1 material", "material:RS-M1");
  change("Element state 1 material point", "point:RS-M1-T020");
  change("Element state 1 thermal state", "unchanged_reference");
  change("Element state 1 thermal provenance", `${P}; no temperature in this case`);
  fill("Element state");
  const p180 = model.pipe_segments.findIndex(p => p.id === "pipe:RS-P180") + 1;
  const replacedSpan = `${P}; replaced span RS-P180: zero eigenstrain (spec §5.1)`;
  expect(screen.getByLabelText(`Element state ${p180} pipe`)).toHaveValue("pipe:RS-P180");
  change(`Element state ${p180} thermal provenance`, replacedSpan);
  enterCommon();
  click("Queue analysis state");
  model = await queuedAndApplied(model, queue);
  expect(caseOf(model, "load:RS-SUS").analysis_state).toEqual(caseOf(fixture, "load:RS-SUS").analysis_state);

  // RS-SAM: copied from RS-SUS; its own primitives and RS-C's motions are added.
  queue = renderState(model, "load:RS-SAM");
  change("Copy analysis state from case", "load:RS-SUS");
  click("Replace this draft's analysis state with the copy");
  expect(screen.getByText(/28 load sources name primitives this case does not hold/)).toBeTruthy();
  change("Factor for every primitive (dimensionless)", "1");
  click("Add every primitive of this case");
  click("Add Support state 3 boundary motion");
  click("Add Support state 3 boundary motion");
  for (const [m, dof, value] of [[1, "UX", "15"], [2, "UY", "-10"]] as const) {
    change(`Support state 3 motion ${m} DOF`, dof);
    change(`Support state 3 motion ${m} displacement value`, value);
    change(`Support state 3 motion ${m} displacement unit`, "mm");
    change(`Support state 3 motion ${m} meaning`, "absolute_reference_displacement");
  }
  click("Queue analysis state");
  model = await queuedAndApplied(model, queue);
  expect(caseOf(model, "load:RS-SAM").analysis_state).toEqual(caseOf(fixture, "load:RS-SAM").analysis_state);

  // RS-OPE: one operating element state typed and filled; the replaced span RS-P180 edited.
  queue = renderState(model, "load:RS-OPE");
  click("Add element state");
  change("Element state 1 pipe", "pipe:RS-P010");
  change("Element state 1 operating temperature value", "210"); change("Element state 1 operating temperature unit", "degC");
  change("Element state 1 material selection", "temperature_interpolation");
  change("Element state 1 material", "material:RS-M1");
  change("Element state 1 selection temperature value", "210"); change("Element state 1 selection temperature unit", "degC");
  change("Element state 1 interpolation", "piecewise_linear");
  change("Element state 1 extrapolation", "forbidden");
  change("Element state 1 thermal state", "free_length_state");
  change("Element state 1 expansion law", "law:RS-M1-SECANT");
  fill("Element state");
  change(`Element state ${p180} operating temperature value`, ""); change(`Element state ${p180} operating temperature unit`, "");
  change(`Element state ${p180} thermal state`, "unchanged_reference");
  change(`Element state ${p180} thermal provenance`, replacedSpan);
  enterCommon();
  click("Queue analysis state");
  model = await queuedAndApplied(model, queue);
  expect(caseOf(model, "load:RS-OPE").analysis_state).toEqual(caseOf(fixture, "load:RS-OPE").analysis_state);

  // §3.10: RS-SUS's five regions copied to RS-OPE and RS-SAM with their own ids.
  for (const caseId of ["load:RS-OPE", "load:RS-SAM"]) {
    queue = vi.fn();
    render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: caseId }} loadCaseId={caseId} onQueueIntent={queue} />);
    const regions = screen.getByRole("region", { name: "Case pressure regions" });
    fireEvent.change(within(regions).getByLabelText("Copy pressure regions from case"), { target: { value: "load:RS-SUS" } });
    fireEvent.click(within(regions).getByRole("button", { name: "Replace this draft's pressure regions with the copy" }));
    fireEvent.click(within(regions).getByRole("button", { name: "Queue pressure regions" }));
    model = await queuedAndApplied(model, queue);
    expect(caseOf(model, caseId).pressure_regions).toEqual(caseOf(fixture, caseId).pressure_regions);
  }
  // Nothing else in the document moved.
  expect(model).toEqual(fixture);
}, 120_000);
