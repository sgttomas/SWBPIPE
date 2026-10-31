import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { MaterialConstitutiveForm, PressureAuthoringPanel, pressureModeText } from "./PressureAuthoringPanel";
import { pressureProfileTarget } from "./pressureContract";
import { MaterialTemperatureForm } from "../material-temperature/MaterialTemperatureForm";
import type { PreviewModel, EditorOperationIntent } from "../../types";
import fixture from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";
vi.mock("../../services/hashService", () => ({ canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)) }));
afterEach(cleanup);
// B7b: the fixture is the current exact-contract document (0.8.0, 3.0.0/exact_pressure_v3
// with an explicit euler_bernoulli selection); a pressure-free document is the same 0.8.0
// without the contract. T4-U8 (D-6/O3): new authoring writes Timoshenko by default.
const model = fixture as PreviewModel;
const V3_EB = { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" as const };
const V3 = { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "timoshenko_annulus_energy_v1" as const };
const pressureFree = () => { const free = structuredClone(model); delete free.pressure_contract; return free; };
const withContract = (contract: unknown) => { const current = structuredClone(model); current.pressure_contract = contract as PreviewModel["pressure_contract"]; return current; };
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
/** Select the profile, optionally a shear model, queue it and return the queued after. */
async function queuedProfile(current: PreviewModel, shear?: string) {
  const queue = vi.fn(); render(<PressureAuthoringPanel model={current} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  change("Pressure profile", "exact_pressure_v3");
  if (shear) change("Shear model", shear);
  fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const after = JSON.parse((queue.mock.calls[0][0] as EditorOperationIntent).change.after);
  cleanup();
  return after;
}
it("requires explicit profile choice and queues only the profile projection", async () => {
  const legacy = pressureFree();
  const queue = vi.fn(); render(<PressureAuthoringPanel model={legacy} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  expect((screen.getByRole("button", { name: "Queue pressure profile" }) as HTMLButtonElement).disabled).toBe(true);
  change("Pressure profile", "exact_pressure_v3"); fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent;
  expect(intent.target).toEqual({ object_type: "Model", ref: model.project.id });
  expect(JSON.parse(intent.change.after)).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
  expect(legacy.load_cases).toEqual(model.load_cases);
});
it("shows the retired legacy label and re-authors it to the exact profile (U3)", async () => {
  const labelled = structuredClone(model); labelled.pressure_contract = { version: "1.0.0", mode: "legacy_pressure_v1" };
  const queue = vi.fn(); render(<PressureAuthoringPanel model={labelled} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  expect(screen.getByText(/pressure mode: 1\.0\.0\/legacy_pressure_v1 \(retired; solves are refused until the model is re-authored to exact_pressure_v3\)/)).toBeInTheDocument();
  change("Pressure profile", "exact_pressure_v3"); fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent;
  expect(JSON.parse(intent.change.before)).toEqual({ schema_version: "0.8.0", pressure_contract: { version: "1.0.0", mode: "legacy_pressure_v1" } });
  expect(JSON.parse(intent.change.after)).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
});
it("shows a model without a pressure contract as pressure-free (U3)", () => {
  const free = pressureFree();
  render(<PressureAuthoringPanel model={free} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={vi.fn()} />);
  expect(screen.getByText(/pressure mode: none \(pressure-free namespace; legacy pressure primitives are refused\)/)).toBeInTheDocument();
  expect(screen.queryByText(/pressure mode: legacy\b/)).toBeNull();
});
it("says when a model without a contract still carries legacy pressure primitives, and when a contract is unsupported (U3, RV127 N-5)", () => {
  const carrying = pressureFree();
  carrying.load_cases[0].primitive_loads = [...(carrying.load_cases[0].primitive_loads ?? []),
    { id: "load:legacy-p", category: "pressure", target: { type: "element", pipe: carrying.pipe_segments[0].id }, direction: "global_x", dimension: "pressure", magnitude: { value: 0, unit: "Pa" }, provenance: "legacy" }];
  render(<PressureAuthoringPanel model={carrying} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={vi.fn()} />);
  expect(screen.getByText(/pressure mode: none \(pressure-free namespace; this model carries 1 legacy pressure primitive, which is refused: remove it or re-author the model to exact_pressure_v3\)/)).toBeInTheDocument();
  cleanup();
  const unsupported = structuredClone(model); unsupported.pressure_contract = { version: "1.0.1", mode: "legacy_pressure_v1" };
  render(<PressureAuthoringPanel model={unsupported} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={vi.fn()} />);
  expect(screen.getByText(/pressure mode: 1\.0\.1\/legacy_pressure_v1 \(unsupported; solves are refused/)).toBeInTheDocument();
  expect(screen.queryByText(/\(retired;/)).toBeNull();
});
it("offers only exact_pressure_v3 and re-authors a retired v2 contract explicitly (T4-U2; B7b: v2 no longer solves)", async () => {
  const v2 = structuredClone(model); v2.pressure_contract = { version: "2.0.0", mode: "exact_straight_pressure_v2" } as never;
  const queue = vi.fn(); render(<PressureAuthoringPanel model={v2} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  expect(screen.getByText(/pressure mode: 2\.0\.0\/exact_straight_pressure_v2 \(unsupported; solves are refused until the model is re-authored to exact_pressure_v3\)/)).toBeInTheDocument();
  const options = [...(screen.getByLabelText("Pressure profile") as HTMLSelectElement).options].map(o => o.value).filter(Boolean);
  expect(options).toEqual(["exact_pressure_v3"]);
  change("Pressure profile", "exact_pressure_v3"); fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(JSON.parse(queue.mock.calls[0][0].change.before)).toEqual({ schema_version: "0.8.0", pressure_contract: { version: "2.0.0", mode: "exact_straight_pressure_v2" } });
  expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
});
it("shows a declared 3.0.0/exact_pressure_v3 contract with its families, the signed-pressure hint and the tangency rule (T4-U2)", () => {
  const v3 = structuredClone(model); v3.pressure_contract = V3;
  render(<PressureAuthoringPanel model={v3} selection={{ type: "load", id: model.load_cases[0].id }} loadCaseId={model.load_cases[0].id} onQueueIntent={vi.fn()} />);
  // T4-U8: the mode text also names the document's straight-member shear model and
  // states that realized arcs are Euler–Bernoulli under either model.
  expect(screen.getByText(/pressure mode: exact_pressure_v3 \(straight members, realized bends and linear restraints or springs; other families are refused by name; straight members use Timoshenko \(energy-matched annular shear factor\), and realized arcs are Euler–Bernoulli under either shear model; pressure-2 results are read with their standing, and only the result JSON and stress-neutral exports, the rule check, the rendered report and the report package admit a numerically eligible result\)/)).toBeInTheDocument();
  expect(screen.queryByText(/solves are refused/)).toBeNull();
  expect(screen.getByText(/a direction change above 1e-3 rad at a bend's end is refused as a mitre/)).toBeInTheDocument();
  expect(screen.getByText(/signed \(a negative value is a net external differential; external-pressure stability and collapse are not assessed\)/)).toBeInTheDocument();
  cleanup();
  // A document not on the current contract (here the retired v2 label) keeps the non-negative hint.
  const v2 = structuredClone(model); v2.pressure_contract = { version: "2.0.0", mode: "exact_straight_pressure_v2" } as never;
  render(<PressureAuthoringPanel model={v2} selection={{ type: "load", id: model.load_cases[0].id }} loadCaseId={model.load_cases[0].id} onQueueIntent={vi.fn()} />);
  expect(screen.getByText(/Expected dimension: pressure; p >= 0\./)).toBeInTheDocument();
});
it("writes the Timoshenko default and Euler–Bernoulli only as an explicit choice (T4-U8, D-6/O3)", async () => {
  const free = pressureFree();
  render(<PressureAuthoringPanel model={free} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={vi.fn()} />);
  const selector = screen.getByLabelText("Shear model") as HTMLSelectElement;
  expect([...selector.options].map(o => [o.value, o.textContent])).toEqual([
    ["timoshenko_annulus_energy_v1", "Timoshenko (energy-matched annular shear factor)"],
    ["euler_bernoulli", "Euler–Bernoulli"],
  ]);
  expect(selector.value).toBe("timoshenko_annulus_energy_v1");
  expect(screen.getByText("The shear model applies to straight members; realized arcs are Euler–Bernoulli under either model.")).toBeInTheDocument();
  cleanup();
  // The default write, without touching the selector.
  expect(await queuedProfile(free)).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
  // The explicit Euler–Bernoulli write.
  expect(await queuedProfile(free, "euler_bernoulli")).toEqual({ schema_version: "0.8.0", pressure_contract: V3_EB });
  // Activation writes only the profile: no allowance is supplied for a section missing one.
  const missing = pressureFree(); delete missing.pipe_segments[0].section.corrosion_allowance;
  render(<PressureAuthoringPanel model={missing} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={vi.fn()} />);
  change("Pressure profile", "exact_pressure_v3"); change("Shear model", "euler_bernoulli");
  expect(screen.getByRole("button", { name: "Queue pressure profile" })).toBeDisabled();
  expect(missing.pipe_segments[0].section.corrosion_allowance).toBeUndefined();
});
it("preserves a document's shear selection on re-queue unless the user changes it (T4-U8)", async () => {
  // Euler–Bernoulli stays Euler–Bernoulli, and the selector shows it.
  const eb = withContract(V3_EB);
  render(<PressureAuthoringPanel model={eb} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={vi.fn()} />);
  expect((screen.getByLabelText("Shear model") as HTMLSelectElement).value).toBe("euler_bernoulli");
  expect(screen.getByText(/straight members use Euler–Bernoulli, and realized arcs are Euler–Bernoulli under either shear model/)).toBeInTheDocument();
  cleanup();
  expect(await queuedProfile(eb)).toEqual({ schema_version: "0.8.0", pressure_contract: V3_EB });
  expect(eb.pressure_contract).toEqual(V3_EB);
  // Timoshenko stays Timoshenko.
  expect(await queuedProfile(withContract(V3))).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
  // The user's explicit change is written, in both directions.
  expect(await queuedProfile(eb, "timoshenko_annulus_energy_v1")).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
  expect(await queuedProfile(withContract(V3), "euler_bernoulli")).toEqual({ schema_version: "0.8.0", pressure_contract: V3_EB });
  // No admitted selection (absent or unknown) is shown as such; the queued profile then
  // carries the default, which the before/after review shows explicitly.
  for (const contract of [{ version: "3.0.0", mode: "exact_pressure_v3" }, { ...V3, shear_model: "cowper" }]) {
    const current = withContract(contract);
    expect(pressureModeText(current)).toContain("no admitted shear model is selected, so solves are refused until one is");
    expect(pressureProfileTarget(current)).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
    expect(await queuedProfile(current)).toEqual({ schema_version: "0.8.0", pressure_contract: V3 });
  }
  // The target itself: the explicit choice, else the document's selection, else the default.
  expect(pressureProfileTarget(eb).pressure_contract).toEqual(V3_EB);
  expect(pressureProfileTarget(eb, "timoshenko_annulus_energy_v1").pressure_contract).toEqual(V3);
  expect(pressureProfileTarget(pressureFree()).pressure_contract).toEqual(V3);
  expect(pressureProfileTarget(pressureFree(), "euler_bernoulli").pressure_contract).toEqual(V3_EB);
});
it("labels realized-bend members with k and geometry-only bend spans as needing realization (T4-U2)", () => {
  const v3 = structuredClone(model); v3.pressure_contract = V3;
  const first = v3.pipe_segments[0];
  const second = { ...structuredClone(first), id: "pipe:second", label: "Second" };
  v3.pipe_segments.push(second);
  v3.components = [
    { id: "component:realized", label: "Realized", kind: "bend", node: first.from, provenance: "invented", geometry: { bend_pipe_ref: first.id }, modifiers: { flexibility_factor_user_value: { value: 2, unit: "none" } }, mechanics_interface: { solver_consumption: "curved_bend_macro_element" } },
    { id: "component:chord", label: "Chord", kind: "bend", node: second.from, provenance: "invented", geometry: { bend_pipe_ref: second.id } },
  ];
  render(<PressureAuthoringPanel model={v3} selection={{ type: "load", id: model.load_cases[0].id }} loadCaseId={model.load_cases[0].id} onQueueIntent={vi.fn()} />);
  expect(screen.getAllByText(/realized bend component:realized, k = 2/).length).toBeGreaterThan(0);
  expect(screen.getAllByText(/geometry-only bend span component:chord: realize this bend first/).length).toBeGreaterThan(0);
});
it("retains regions and ordered terminal declarations through an edit", async () => {
  const queue = vi.fn(); render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: model.load_cases[0].id }} loadCaseId={model.load_cases[0].id} onQueueIntent={queue} />);
  change("Region 1 terminal 2 closure transfer", "separately_supported_or_compensated");
  fireEvent.click(screen.getByRole("button", { name: "Queue pressure regions" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent; const after = JSON.parse(intent.change.after);
  expect(after[0].pressure).toEqual({ value: 2000, unit: "kPa" });
  expect(after[0].terminals.map((t: { node_ref: string }) => t.node_ref)).toEqual(model.load_cases[0].pressure_regions![0].terminals.map(t => t.node_ref));
  expect(after[0].terminals[1].closure_transfer).toBe("separately_supported_or_compensated");
  expect(after[0].provenance).toBe(model.load_cases[0].pressure_regions![0].provenance);
});
it("queues explicit empty regions for an undeclared case without changing its loads", async () => {
  const input = structuredClone(model); delete input.load_cases[1].pressure_regions;
  const queue = vi.fn(); render(<PressureAuthoringPanel model={input} selection={{ type: "load", id: input.load_cases[1].id }} loadCaseId={input.load_cases[1].id} onQueueIntent={queue} />);
  fireEvent.click(screen.getByRole("button", { name: "Queue explicit empty pressure regions" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(queue.mock.calls[0][0].change.before).toBe("not_present"); expect(queue.mock.calls[0][0].change.after).toBe("[]");
  expect(input.load_cases[1].primitive_loads).toEqual(model.load_cases[1].primitive_loads);
});
it("preserves explicit connector membership on 0.8.0 and never offers it on old documents", async () => {
  const current = structuredClone(model); current.schema_version = "0.8.0"; current.pressure_contract = V3;
  current.load_cases[0].pressure_regions![0].member_connector_ids = ["component:J"];
  const queue = vi.fn();
  render(<PressureAuthoringPanel model={current} selection={{ type: "load", id: current.load_cases[0].id }} loadCaseId={current.load_cases[0].id} onQueueIntent={queue} />);
  expect(screen.getByLabelText("Region 1 member connectors")).toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "Queue pressure regions" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(JSON.parse(queue.mock.calls[0][0].change.after)[0].member_connector_ids).toEqual(["component:J"]);
  cleanup();
  // B7b: an old (retired-version) document is never offered it.
  const old = structuredClone(model); old.schema_version = "0.4.0";
  render(<PressureAuthoringPanel model={old} selection={{ type: "load", id: model.load_cases[0].id }} loadCaseId={model.load_cases[0].id} onQueueIntent={queue} />);
  expect(screen.queryByLabelText("Region 1 member connectors")).toBeNull();
});
it("allows explicit negative nu and does not synthesize G", async () => {
  const queue = vi.fn(); render(<MaterialConstitutiveForm model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={queue} />);
  change("Poisson ratio nu value", "-0.2"); fireEvent.click(screen.getByRole("button", { name: "Queue constitutive properties" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const after = JSON.parse(queue.mock.calls[0][0].change.after); expect(after.poisson_ratio).toEqual({ value: -0.2, unit: "1" }); expect(after.shear_modulus).toBeUndefined();
});
it("preserves explicitly authored temperature-point nu", async () => {
  const input = structuredClone(model); input.materials![0].temperature_points = [{ id: "point:a", elastic_modulus: { value: 200, unit: "GPa" }, poisson_ratio: { value: -0.2, unit: "1" }, provenance: "synthetic" }];
  const queue = vi.fn(); render(<MaterialTemperatureForm model={input} selection={{ type: "material", id: input.materials![0].id }} onQueueIntent={queue} />);
  fireEvent.click(screen.getByRole("button", { name: "Queue temperature points" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(JSON.parse(queue.mock.calls[0][0].change.after)[0].poisson_ratio).toEqual({ value: -0.2, unit: "1" });
});
/** G5 (DEL-07-03): the E/nu create form's optional material-level thermal expansion coefficient. */
function enterExactMaterial() {
  change("Exact material ID", "material:new"); change("Exact material label", "New");
  change("Constitutive basis", "homogeneous_isotropic_E_nu_v1");
  change("Elastic modulus E value", "200"); change("Elastic modulus E unit", "GPa");
  change("Poisson ratio nu value", "0.3"); change("Poisson ratio nu unit", "1");
  change("Constitutive provenance", "synthetic");
}
it("creates an E/nu material with an entered thermal expansion coefficient", async () => {
  const queue = vi.fn(); render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  enterExactMaterial();
  change("Thermal expansion coefficient value", "1.2e-5"); change("Thermal expansion coefficient unit", "1/K");
  fireEvent.click(screen.getByRole("button", { name: "Queue exact material" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  const intent = queue.mock.calls[0][0] as EditorOperationIntent;
  expect(intent.change.change_kind).toBe("create_material");
  expect(JSON.parse(intent.change.after).thermal_expansion_coefficient).toEqual({ value: 1.2e-5, unit: "1/K" });
});
it("leaves the coefficient absent when blank and refuses a half-entered one", async () => {
  const queue = vi.fn(); render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: model.load_cases[0].id }} onQueueIntent={queue} />);
  enterExactMaterial();
  change("Thermal expansion coefficient value", "1.2e-5"); change("Thermal expansion coefficient value", "");
  fireEvent.click(screen.getByRole("button", { name: "Queue exact material" })); await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  expect(Object.hasOwn(JSON.parse(queue.mock.calls[0][0].change.after), "thermal_expansion_coefficient")).toBe(false);
  change("Exact material ID", "material:second"); change("Thermal expansion coefficient value", "1.2e-5");
  fireEvent.click(screen.getByRole("button", { name: "Queue exact material" }));
  expect(await screen.findByText(/thermal expansion coefficient unit is required/i)).toBeTruthy();
  expect(queue).toHaveBeenCalledOnce();
});
