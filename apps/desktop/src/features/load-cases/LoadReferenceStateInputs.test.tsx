import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { LoadReferenceStateInputs } from "./LoadReferenceStateInputs";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";
import { MaterialTemperatureForm } from "../material-temperature/MaterialTemperatureForm";
import { LOAD_STATE_NEEDS_040, NOT_PRESENT, toPayload } from "./loadStateAuthoring";
import { applyModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import connected from "../../../../../fixtures/product_preview/load_reference/connected.request.json";
import exactPressure from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";

// All values are invented test inputs (see the fixture provenance
// `invented_load_reference_fixture_not_library_data`). The operation engine is
// the real wasm build of core/model_operations/operation_applier.
afterEach(cleanup);
const base = () => structuredClone(connected.model) as unknown as PreviewModel;
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const click = (name: string) => fireEvent.click(screen.getByRole("button", { name }));
const cold = (model: PreviewModel) => model.load_cases.find(c => c.id === "case:cold")!;

function renderInputs(model: PreviewModel, caseId = "case:cold") {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const view = render(<LoadReferenceStateInputs model={model} selection={{ type: "load", id: caseId }} onQueueIntent={queue} loadCase={model.load_cases.find(c => c.id === caseId) ?? null} />);
  return { queue, view };
}

async function queued(queue: ReturnType<typeof vi.fn>): Promise<EditorOperationIntent> {
  await waitFor(() => expect(queue).toHaveBeenCalledOnce(), { timeout: 20_000 });
  return queue.mock.calls[0][0] as EditorOperationIntent;
}

/** Applies the queued intent through the same engine Review/Apply uses. */
async function applied(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  expect(outcome.validation.application_status).toBe("applied_to_session_model");
  return outcome.applied_model as PreviewModel;
}

function controlValues(region: HTMLElement): string[] {
  return Array.from(region.querySelectorAll("input, select")).map(el => (el as HTMLInputElement | HTMLSelectElement).value);
}

describe("pre-0.4 gating (D3: no upgrade operation)", () => {
  it("offers no load/reference-state field on a pre-0.4 model, only the one line", () => {
    for (const version of ["0.1.0", "0.2.0", "0.3.0", "0.4.1"]) {
      const model = structuredClone(exactPressure) as unknown as PreviewModel;
      model.schema_version = version;
      const { view } = renderInputs(model, model.load_cases[0].id);
      expect(screen.getByTestId("load-state-needs-040")).toHaveTextContent(LOAD_STATE_NEEDS_040);
      expect(screen.queryByTestId("load-state-inputs")).toBeNull();
      expect(screen.queryByLabelText("Analysis state contract")).toBeNull();
      expect(screen.queryByRole("button", { name: "Queue reference configurations" })).toBeNull();
      view.unmount();
      const materials = render(<MaterialTemperatureForm model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={vi.fn()} />);
      expect(screen.getByTestId("expansion-laws-needs-040")).toHaveTextContent(LOAD_STATE_NEEDS_040);
      expect(screen.queryByTestId("expansion-laws-editor")).toBeNull();
      expect(screen.queryByRole("button", { name: "Queue expansion laws" })).toBeNull();
      materials.unmount();
    }
  });

  it("mounts the fields in the Load Case Manager on 0.4.0 and the one line on the 0.3.0 model", () => {
    const model = base();
    const first = render(<LoadCaseManagerPanel model={model} selection={{ type: "load", id: "case:hot" }} onQueueIntent={vi.fn()} onSelect={vi.fn()} />);
    expect(within(screen.getByTestId("load-case-manager")).getByTestId("load-state-inputs")).toBeInTheDocument();
    expect(screen.getByTestId("load-state-analysis-state")).toHaveTextContent("Analysis state — case:hot");
    first.unmount();
    const legacy = structuredClone(exactPressure) as unknown as PreviewModel;
    render(<LoadCaseManagerPanel model={legacy} selection={{ type: "load", id: legacy.load_cases[0].id }} onQueueIntent={vi.fn()} onSelect={vi.fn()} />);
    expect(within(screen.getByTestId("load-case-manager")).getByTestId("load-state-needs-040")).toHaveTextContent(LOAD_STATE_NEEDS_040);
    expect(screen.queryByTestId("load-state-inputs")).toBeNull();
  });
});

describe("operation submission, one field group at a time", () => {
  it("reference configuration: member fit is submitted as a whole-record Model/reference_configurations replacement", async () => {
    const model = base();
    const before = structuredClone(model);
    const { queue } = renderInputs(model);
    change("Reference configuration 1 member 2 fit", "fit_strain");
    change("Reference configuration 1 member 2 fit strain value", "0.0001");
    change("Reference configuration 1 member 2 fit strain unit", "1");
    click("Queue reference configurations");
    const intent = await queued(queue);
    expect(intent.target).toEqual({ object_type: "Model", ref: model.project.id });
    expect(intent.change.change_kind).toBe("set_field");
    expect(intent.change.field_path).toBe("reference_configurations");
    expect([intent.change.unit, intent.change.dimension]).toEqual(["none", "dimensionless"]);
    expect(intent.change.before).toBe(await canonicalJsonString(model.reference_configurations));
    const expected = structuredClone(model.reference_configurations!);
    expected[0].member_references[1].fit = { kind: "fit_strain", strain: { value: 0.0001, unit: "1" } };
    expect(JSON.parse(intent.change.after)).toEqual(expected);
    expect(screen.getByTestId("load-state-reference-notice")).toHaveTextContent("checked by the operation engine and queued for Review/Apply");
    expect(model).toEqual(before); // no client-side mutation
    const next = await applied(model, intent);
    expect(next.reference_configurations).toEqual(expected);
    expect(next.schema_version).toBe("0.4.0");
  });

  it("analysis state: support motion, element thermal state, material selection and a load-source factor", async () => {
    const model = base();
    const before = structuredClone(model);
    const { queue } = renderInputs(model);
    // Support motion on the second support.
    click("Add Support state 2 boundary motion");
    change("Support state 2 motion 1 DOF", "UY");
    change("Support state 2 motion 1 displacement value", "2");
    change("Support state 2 motion 1 displacement unit", "mm");
    change("Support state 2 motion 1 meaning", "absolute_reference_displacement");
    // Element thermal state: explicit constant-alpha interval.
    change("Element state 2 thermal state", "constant_alpha_interval");
    change("Element state 2 expansion coefficient value", "0.000012");
    change("Element state 2 expansion coefficient unit", "1/K");
    change("Element state 2 temperature change value", "30");
    change("Element state 2 temperature change unit", "K");
    change("Element state 2 coefficient meaning", "engineering_interval");
    change("Element state 2 thermal provenance", "invented_wp3_test_input");
    // Material selection by temperature interpolation.
    change("Element state 1 material selection", "temperature_interpolation");
    change("Element state 1 selection temperature value", "50");
    change("Element state 1 selection temperature unit", "degC");
    change("Element state 1 interpolation", "piecewise_linear");
    change("Element state 1 extrapolation", "forbidden");
    // Load source factor.
    change("Load source 1 factor (dimensionless)", "2");
    click("Queue analysis state");
    const intent = await queued(queue);
    expect(intent.target).toEqual({ object_type: "Load", ref: "case:cold" });
    expect(intent.change.change_kind).toBe("update_load");
    expect(intent.change.field_path).toBe("analysis_state");
    expect(intent.change.before).toBe(await canonicalJsonString(cold(model).analysis_state));
    const expected = structuredClone(cold(model).analysis_state!);
    expected.support_states[1].boundary_motion = [{ dof: "UY", value: { value: 2, unit: "mm" }, meaning: "absolute_reference_displacement" }];
    expected.element_states[1].thermal_state = { kind: "constant_alpha_interval", coefficient: { value: 0.000012, unit: "1/K" }, temperature_change: { value: 30, unit: "K" }, coefficient_meaning: "engineering_interval", provenance: "invented_wp3_test_input" };
    expected.element_states[0].material_selection = { kind: "temperature_interpolation", material_ref: "material:shared", temperature: { value: 50, unit: "degC" }, interpolation: "piecewise_linear", extrapolation: "forbidden" };
    expected.load_sources[0].factor = 2;
    expect(JSON.parse(intent.change.after)).toEqual(expected);
    expect(model).toEqual(before);
    const next = await applied(model, intent);
    expect(cold(next).analysis_state).toEqual(expected);
    expect(next.load_cases.find(c => c.id === "case:hot")).toEqual(model.load_cases.find(c => c.id === "case:hot"));
  });

  it("expansion laws: a user-entered dilation table is submitted as a Material/expansion_laws replacement", async () => {
    const model = base();
    const material = model.materials![0];
    const queue = vi.fn();
    render(<MaterialTemperatureForm model={model} selection={{ type: "material", id: material.id }} onQueueIntent={queue} />);
    click("Add expansion law");
    change("Expansion law 2 definition", "engineering_dilation");
    change("Expansion law 2 ID", "law:dilation-invented");
    change("Expansion law 2 datum temperature value", "20");
    change("Expansion law 2 datum temperature unit", "degC");
    change("Expansion law 2 data", "table");
    change("Expansion law 2 interpolation", "linear_dilation");
    click("Add point to expansion law 2");
    click("Add point to expansion law 2");
    change("Expansion law 2 point 1 temperature value", "20");
    change("Expansion law 2 point 1 temperature unit", "degC");
    change("Expansion law 2 point 1 dilation value", "0");
    change("Expansion law 2 point 1 dilation unit", "1");
    change("Expansion law 2 point 2 temperature value", "150");
    change("Expansion law 2 point 2 temperature unit", "degC");
    change("Expansion law 2 point 2 dilation value", "0.0015");
    change("Expansion law 2 point 2 dilation unit", "1");
    change("Expansion law 2 provenance", "invented_wp3_test_input");
    click("Queue expansion laws");
    const intent = await queued(queue);
    expect(intent.target).toEqual({ object_type: "Material", ref: material.id });
    expect(intent.change.change_kind).toBe("set_field");
    expect(intent.change.field_path).toBe("expansion_laws");
    const expected = [...structuredClone(material.expansion_laws!), {
      definition: "engineering_dilation", id: "law:dilation-invented", datum_temperature: { value: 20, unit: "degC" },
      data: { kind: "table", interpolation: "linear_dilation", points: [
        { temperature: { value: 20, unit: "degC" }, dilation: { value: 0, unit: "1" } },
        { temperature: { value: 150, unit: "degC" }, dilation: { value: 0.0015, unit: "1" } }
      ] }, provenance: "invented_wp3_test_input"
    }];
    expect(JSON.parse(intent.change.after)).toEqual(expected);
    const next = await applied(model, intent);
    expect(next.materials![0].expansion_laws).toEqual(expected);
  });

  it("removal uses the explicit not_present value and restores absence", async () => {
    const model = base();
    const { queue } = renderInputs(model, "case:hot");
    click("Queue removal of analysis state");
    const intent = await queued(queue);
    expect(intent.change.after).toBe(NOT_PRESENT);
    const next = await applied(model, intent);
    expect(Object.hasOwn(next.load_cases.find(c => c.id === "case:hot")!, "analysis_state")).toBe(false);
  });
});

describe("engine refusals are shown as targeted diagnostics and the model stays editable", () => {
  async function refusal(testId: string, code: string) {
    await waitFor(() => expect(screen.getByTestId(`${testId}-diagnostic-${code}`)).toBeInTheDocument(), { timeout: 20_000 });
    expect(screen.getByTestId(`${testId}-error`)).toHaveTextContent("refused by the operation engine; nothing was queued");
  }

  it("unknown field: the closed typed boundary refuses and nothing is queued", async () => {
    const model = base();
    (model.reference_configurations![0] as unknown as Record<string, unknown>).unreviewed_extra = "invented";
    const before = JSON.stringify(model);
    const { queue } = renderInputs(model);
    click("Queue reference configurations");
    await refusal("load-state-reference", "OP-LOAD-STATE-PAYLOAD-INVALID");
    expect(screen.getByTestId("load-state-reference-diagnostic-OP-LOAD-STATE-PAYLOAD-INVALID")).toHaveTextContent("unknown field `unreviewed_extra`");
    expect(queue).not.toHaveBeenCalled();
    expect(JSON.stringify(model)).toBe(before);
    // Still editable: the draft is kept and the fields remain enabled.
    expect(screen.getByLabelText("Reference configuration 1 ID")).toBeEnabled();
    expect(screen.getByLabelText("Reference configuration 1 ID")).toHaveValue("reference:installed");
  });

  it("dangling reference: a load source naming a primitive the case no longer stores", async () => {
    const model = base();
    const target = cold(model) as unknown as { primitive_loads: Array<{ id: string }> };
    target.primitive_loads = target.primitive_loads.filter(p => p.id !== "load:transverse:case:cold");
    const { queue } = renderInputs(model);
    click("Queue analysis state");
    await refusal("load-state-analysis", "OP-LOAD-STATE-REFERENCE-UNRESOLVED");
    expect(screen.getByTestId("load-state-analysis-diagnostic-OP-LOAD-STATE-REFERENCE-UNRESOLVED")).toHaveTextContent("load:transverse:case:cold");
    expect(queue).not.toHaveBeenCalled();
    expect(screen.getByLabelText("Analysis state provenance")).toBeEnabled();
  });

  it("explicit null inside the record is refused, not treated as absence", async () => {
    const model = base();
    (cold(model).analysis_state as unknown as Record<string, unknown>).load_sources = null;
    const { queue } = renderInputs(model);
    click("Queue analysis state");
    await refusal("load-state-analysis", "OP-LOAD-STATE-PAYLOAD-INVALID");
    expect(queue).not.toHaveBeenCalled();
  });

  it("inbound reference: removing laws that element states still name is refused", async () => {
    const model = base();
    const queue = vi.fn();
    render(<MaterialTemperatureForm model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={queue} />);
    click("Queue removal of expansion laws");
    await refusal("expansion-laws", "OP-LOAD-STATE-INBOUND-REFERENCE");
    expect(queue).not.toHaveBeenCalled();
    expect(screen.getByLabelText("Expansion law 1 ID")).toBeEnabled();
  });
});

describe("no defaults: empty means absent, units are explicit", () => {
  it("a case without analysis_state starts with every field empty and adds empty rows", () => {
    const model = base();
    delete cold(model).analysis_state;
    renderInputs(model);
    const region = screen.getByTestId("load-state-analysis-state");
    expect(region).toHaveTextContent("analysis_state is absent for this case.");
    expect(controlValues(region).every(v => v === "")).toBe(true);
    click("Add element state");
    click("Add support state");
    click("Add load source");
    click("Add Support state 1 boundary motion");
    change("Element state 1 material selection", "exact_point");
    change("Element state 1 thermal state", "constant_alpha_interval");
    const values = controlValues(region);
    expect(values.filter(v => v !== "")).toEqual(["exact_point", "constant_alpha_interval"]);
    for (const label of ["Element state 1 operating temperature unit", "Element state 1 expansion coefficient unit", "Element state 1 temperature change unit", "Support state 1 motion 1 displacement unit"])
      expect(screen.getByLabelText(label)).toHaveValue("");
  });

  it("new reference configurations and expansion laws start empty", () => {
    const model = base();
    delete model.reference_configurations;
    delete model.materials![0].expansion_laws;
    renderInputs(model);
    const region = screen.getByTestId("load-state-reference-configurations");
    expect(region).toHaveTextContent("reference_configurations is absent.");
    click("Add reference configuration");
    click("Add member to reference configuration 1");
    expect(controlValues(region).every(v => v === "")).toBe(true);
    cleanup();
    render(<MaterialTemperatureForm model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={vi.fn()} />);
    const laws = screen.getByTestId("expansion-laws-editor");
    expect(laws).toHaveTextContent("expansion_laws is absent for this material.");
    click("Add expansion law");
    expect(controlValues(laws).every(v => v === "")).toBe(true);
  });

  it("an empty draft is sent with absent keys and the engine refuses it; nothing is filled in", async () => {
    const model = base();
    delete cold(model).analysis_state;
    const { queue } = renderInputs(model);
    click("Queue analysis state");
    await waitFor(() => expect(screen.getByTestId("load-state-analysis-diagnostic-OP-LOAD-STATE-PAYLOAD-INVALID")).toHaveTextContent("missing field `contract`"), { timeout: 20_000 });
    expect(queue).not.toHaveBeenCalled();
  });

  it("the payload conversion keeps absence and refuses non-numbers", () => {
    expect(toPayload({ a: { value: "", unit: "" }, b: { value: "1.5", unit: "mm" }, c: "", factor: "" })).toEqual({ b: { value: 1.5, unit: "mm" }, c: "" });
    expect(toPayload({ value: "2" })).toEqual({ value: 2 });
    expect(toPayload([{ source_ref: "s", factor: "0.5" }])).toEqual([{ source_ref: "s", factor: 0.5 }]);
    expect(() => toPayload({ q: { value: "abc", unit: "mm" } })).toThrow("must be a finite number");
    expect(() => toPayload({ factor: "x" })).toThrow("must be a finite number");
  });
});
