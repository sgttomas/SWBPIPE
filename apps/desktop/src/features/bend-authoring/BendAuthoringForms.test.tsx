import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../../../../fixtures/model_operations/pressure_v3_l_line_authoring.json";
import { BendAuthoringForms, GEOMETRY_ONLY_BANNER, SIF_HINT } from "./BendAuthoringForms";
import { ConnectorDraftFields } from "../component-creation/ConnectorDraftFields";
import { buildCreateComponentIntent, CONNECTOR_CONTRACT_NOTICE, defaultComponentDraft, isComponentDraftValid, type ComponentDraft } from "../component-creation/componentIntent";
import { applyModelOperation, validateModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import type { EditorOperationIntent, PreviewModel } from "../../types";

afterEach(cleanup);
const authored = fixture.authored_model as unknown as PreviewModel;
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
/** The authored L line with its bend returned to a geometry-only span bend. */
function spanBend(): PreviewModel {
  const model = structuredClone(authored);
  model.components[0].mechanics_interface = { solver_consumption: "mechanics_geometry_only" };
  delete model.components[0].modifiers;
  for (const loadCase of model.load_cases) loadCase.pressure_regions = [];
  return model;
}
async function blocking(model: PreviewModel, intent: EditorOperationIntent) {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  return { codes: outcome.diagnostics.filter(d => d.severity === "blocking").map(d => d.code), model: outcome.applied_model as PreviewModel | null };
}

describe("bend realization (T4-U2)", () => {
  it("shows the geometry-only banner on v3, asks for the user's k and realizes the span bend", async () => {
    const model = spanBend();
    const queue = vi.fn();
    render(<BendAuthoringForms model={model} selection={{ type: "component", id: "component:bend-1" }} onQueueIntent={queue} />);
    expect(screen.getByTestId("bend-geometry-only-banner")).toHaveTextContent(GEOMETRY_ONLY_BANNER);
    expect((screen.getByLabelText("Bend flexibility factor k") as HTMLInputElement).value).toBe("");
    fireEvent.click(screen.getByRole("button", { name: "Queue realize bend" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Enter the bend's flexibility factor k");
    expect(queue).not.toHaveBeenCalled();
    change("Bend flexibility factor k", "2"); change("Source of k", "invented user value");
    fireEvent.click(screen.getByRole("button", { name: "Queue realize bend" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0] as EditorOperationIntent;
    expect([intent.target, intent.change.field_path, intent.author_type]).toEqual([{ object_type: "Component", ref: "component:bend-1" }, "bend_realization", "user"]);
    expect(JSON.parse(intent.change.after)).toEqual({ solver_consumption: "curved_bend_macro_element", flexibility_factor_user_value: { value: 2, unit: "none" }, source_reference: "invented user value" });
    const applied = await blocking(model, intent);
    expect(applied.codes).toEqual([]);
    expect(applied.model!.components[0].mechanics_interface?.solver_consumption).toBe("curved_bend_macro_element");
    cleanup();

    const back = vi.fn();
    render(<BendAuthoringForms model={applied.model!} selection={{ type: "component", id: "component:bend-1" }} onQueueIntent={back} />);
    expect(screen.queryByTestId("bend-geometry-only-banner")).toBeNull();
    expect(screen.getByTestId("bend-realization-state")).toHaveTextContent("Realized curved bend over pipe:BEND: k = 2 (invented user value)");
    fireEvent.click(screen.getByRole("button", { name: "Queue return to geometry-only" }));
    await waitFor(() => expect(back).toHaveBeenCalledOnce());
    expect(JSON.parse(back.mock.calls[0][0].change.after)).toEqual({ solver_consumption: "mechanics_geometry_only" });
    expect((await blocking(applied.model!, back.mock.calls[0][0])).codes).toEqual([]);
  });

  it("takes the directional SIF pair with its source, both or neither (T4-U4b)", async () => {
    const model = spanBend();
    const queue = vi.fn();
    render(<BendAuthoringForms model={model} selection={{ type: "component", id: "component:bend-1" }} onQueueIntent={queue} />);
    expect(screen.getByText(SIF_HINT)).toBeInTheDocument();
    change("Bend flexibility factor k", "2"); change("Source of k", "invented user values");
    change("Bend in-plane SIF i_ip", "2.3");
    fireEvent.click(screen.getByRole("button", { name: "Queue realize bend" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Enter both directional SIFs");
    expect(queue).not.toHaveBeenCalled();
    change("Bend out-of-plane SIF i_op", "1.7");
    fireEvent.click(screen.getByRole("button", { name: "Queue realize bend" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0] as EditorOperationIntent;
    expect(JSON.parse(intent.change.after)).toEqual({ solver_consumption: "curved_bend_macro_element", flexibility_factor_user_value: { value: 2, unit: "none" },
      source_reference: "invented user values", sif_in_plane_user_value: { value: 2.3, unit: "1" }, sif_out_of_plane_user_value: { value: 1.7, unit: "1" } });
    const applied = await blocking(model, intent);
    expect(applied.codes).toEqual([]);
    expect(applied.model!.components[0].modifiers?.sif_out_of_plane_user_value).toEqual({ value: 1.7, unit: "1" });
    cleanup();
    // Realized with the pair: the state names it, and returning to geometry-only is refused by the engine.
    const back = vi.fn();
    render(<BendAuthoringForms model={applied.model!} selection={{ type: "component", id: "component:bend-1" }} onQueueIntent={back} />);
    expect(screen.getByTestId("bend-realization-state")).toHaveTextContent("Directional SIFs i_ip = 2.3, i_op = 1.7");
    fireEvent.click(screen.getByRole("button", { name: "Queue return to geometry-only" }));
    await waitFor(() => expect(back).toHaveBeenCalledOnce());
    expect((await blocking(applied.model!, back.mock.calls[0][0])).codes).toContain("OP-BEND-DIRECTIONAL-SIF-NOT-REALIZED-ARC");
  });

  it("names a corner marker and offers insertion only at a node joining exactly two pipes", () => {
    const model = spanBend();
    delete model.components[0].geometry!.bend_pipe_ref;
    render(<BendAuthoringForms model={model} selection={{ type: "component", id: "component:bend-1" }} onQueueIntent={vi.fn()} />);
    expect(screen.getByTestId("bend-realization-state")).toHaveTextContent("it marks a corner. Select its node and use Insert bend at corner");
    cleanup();
    render(<BendAuthoringForms model={authored} selection={{ type: "node", id: "node:B" }} onQueueIntent={vi.fn()} />);
    expect(screen.getByTestId("insert-bend-at-corner-form")).toBeInTheDocument();
    cleanup();
    render(<BendAuthoringForms model={authored} selection={{ type: "node", id: "node:A" }} onQueueIntent={vi.fn()} />);
    expect(screen.queryByTestId("insert-bend-at-corner-form")).toBeNull();
  });

  it("refuses an agent-authored k in the engine", async () => {
    const model = spanBend();
    const queue = vi.fn();
    render(<BendAuthoringForms model={model} selection={{ type: "component", id: "component:bend-1" }} onQueueIntent={queue} />);
    change("Bend flexibility factor k", "2"); change("Source of k", "invented");
    fireEvent.click(screen.getByRole("button", { name: "Queue realize bend" }));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const agent = { ...queue.mock.calls[0][0], author_type: "agent" } as EditorOperationIntent;
    expect((await blocking(model, agent)).codes).toContain("OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED");
  });
});

describe("the objective connector form (T4-U2)", () => {
  const connector: Partial<ComponentDraft> = {
    connectorAxes: "1 0 0; 0 1 0; 0 0 1", endINodeAxes: "1 0 0; 0 1 0; 0 0 1", endJNodeAxes: "1 0 0; 0 1 0; 0 0 1",
    endIOffset: "0 0 0", endJOffset: "0 0 0", offsetUnit: "m", qRefTranslation: "0 0 0", qRefTranslationUnit: "m",
    qRefRotation: "0 0 0", qRefRotationUnit: "rad", referenceState: "stress_free", installedTemperature: "20", installedTemperatureUnit: "degC",
    translationScale: "0.5", translationScaleUnit: "m",
    workMatrixUpperTriangle: "800000 0 0 0 0 0 225000 0 0 0 0 225000 0 0 0 620000 0 0 480000 0 480000",
    stiffnessSourceReference: "invented", measurementRestraints: "invented", basisTransformReference: "none", validityStatement: "invented analytical control",
    installedGeometry: "authored model geometry", hardwareKind: "untied", pressureModel: "unpressurized", provenance: "invented",
  };
  it("builds an objective connector replacing the selected span on a v3 document, which the engine accepts", async () => {
    const model = structuredClone(authored);
    for (const loadCase of model.load_cases) { loadCase.pressure_regions = []; loadCase.primitive_loads = []; }
    const draft = { ...defaultComponentDraft(model, { type: "node", id: "node:A" }, [], "expansion_joint"), primaryPipeRef: "pipe:S1", ...connector } as ComponentDraft;
    expect(isComponentDraftValid(model, draft)).toBe(true);
    expect(isComponentDraftValid(model, { ...draft, workMatrixUpperTriangle: "1 2 3" })).toBe(false);
    expect(isComponentDraftValid(model, { ...draft, connectorAxes: "1 0 0; 0 1 0" })).toBe(false);
    const intent = buildCreateComponentIntent({ draft, model, sourceRef: "test", sourceRole: "gui_editor", unitValidation: "test" });
    const payload = JSON.parse(intent.change.after);
    expect(payload.geometry).toBeUndefined();
    expect(payload.objective_connector.topology).toEqual({ type: "replaces_span", span_ref: "pipe:S1" });
    expect([payload.objective_connector.end_i.node_ref, payload.objective_connector.end_j.node_ref]).toEqual(["node:A", "node:B"]);
    expect(payload.objective_connector.stiffness.upper_triangle).toHaveLength(21);
    const outcome = await validateModelOperation(model, intent, await computeModelHash(model));
    expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  });

  it("is not offered on a pressure-free document", () => {
    const free = structuredClone(fixture.corner_document as unknown as PreviewModel);
    const draft = { ...defaultComponentDraft(free, { type: "node", id: "node:A" }, [], "expansion_joint"), primaryPipeRef: "pipe:S1", ...connector } as ComponentDraft;
    expect(isComponentDraftValid(free, draft)).toBe(false);
    render(<ConnectorDraftFields model={free} draft={draft} prefix="create" TextInput={() => <input />} update={vi.fn()} />);
    expect(screen.getByTestId("create-component-connector-unavailable")).toHaveTextContent(CONNECTOR_CONTRACT_NOTICE);
  });
  it("authors the effective area only when the model is 0.9.0", () => {
    const model = structuredClone(authored); model.schema_version = "0.9.0";
    const draft = { ...defaultComponentDraft(model, { type: "node", id: "node:A" }, [], "expansion_joint"), primaryPipeRef: "pipe:S1", ...connector,
      pressureModel: "constant_effective_area_reference_v1", effectiveArea: "0.004", effectiveAreaUnit: "m^2", pressureBasis: "internal_differential_zero_external_v1", pressureSourceReference: "invented explicit area" } as ComponentDraft;
    expect(isComponentDraftValid(model, draft)).toBe(true);
    expect(isComponentDraftValid({ ...model, schema_version: "0.3.0" }, draft)).toBe(false);
    expect(isComponentDraftValid(model, { ...draft, effectiveArea: "" })).toBe(false);
    const intent = buildCreateComponentIntent({ draft, model, sourceRef: "test", sourceRole: "gui_editor", unitValidation: "test" });
    expect(JSON.parse(intent.change.after).objective_connector.pressure_model).toEqual({ kind: "constant_effective_area_reference_v1", effective_area: { value: 0.004, unit: "m^2" }, pressure_basis: "internal_differential_zero_external_v1", source_reference: "invented explicit area" });
  });

  // T4-U5b (DEL-04-07 Design u4b-u5b-implementation.md §3.1, §5.2): the rod table.
  const rodRow = (id: string, y: string) => ({ id, offsetI: `-0.05 ${y} 0`, offsetJ: `0.05 ${y} 0`, offsetUnit: "m", stiffness: "10000", stiffnessUnit: "kN/m",
    referenceExtension: "-0.025", referenceExtensionUnit: "mm", sourceReference: "invented rod" });
  it("offers explicit tie rods on 0.9.0 with a rod table, and builds the hardware the engine accepts", async () => {
    const model = structuredClone(authored); model.schema_version = "0.9.0";
    for (const loadCase of model.load_cases) { loadCase.pressure_regions = []; loadCase.primitive_loads = []; }
    const rods = [rodRow("rod:a", "0.1"), rodRow("rod:b", "-0.1")];
    const draft = { ...defaultComponentDraft(model, { type: "node", id: "node:A" }, [], "expansion_joint"), primaryPipeRef: "pipe:S1", ...connector,
      hardwareKind: "explicit_axial_rods_v1", tieRods: JSON.stringify(rods) } as ComponentDraft;
    expect(isComponentDraftValid(model, draft)).toBe(true);
    expect(isComponentDraftValid({ ...model, schema_version: "0.5.0" }, draft)).toBe(false);
    for (const broken of [
      { tieRods: "" },
      { tieRods: JSON.stringify([rodRow("rod:a", "0.1"), rodRow("rod:a", "-0.1")]) },
      { tieRods: JSON.stringify([{ ...rodRow("rod:a", "0.1"), sourceReference: " " }]) },
      { tieRods: JSON.stringify([{ ...rodRow("rod:a", "0.1"), stiffness: "0" }]) },
      { tieRods: JSON.stringify([{ ...rodRow("rod:a", "0.1"), offsetJ: "0.05 0.1" }]) },
      { hardwareKind: "ideal_inextensible" },
    ]) {
      expect(isComponentDraftValid(model, { ...draft, ...broken })).toBe(false);
    }
    const intent = buildCreateComponentIntent({ draft, model, sourceRef: "test", sourceRole: "gui_editor", unitValidation: "test" });
    const hardware = JSON.parse(intent.change.after).objective_connector.hardware;
    expect(hardware.kind).toBe("explicit_axial_rods_v1");
    expect(hardware.rods[0]).toEqual({
      id: "rod:a", law: "bilateral_linear",
      offset_i: { x: { value: -0.05, unit: "m" }, y: { value: 0.1, unit: "m" }, z: { value: 0, unit: "m" } },
      offset_j: { x: { value: 0.05, unit: "m" }, y: { value: 0.1, unit: "m" }, z: { value: 0, unit: "m" } },
      stiffness: { value: 10000, unit: "kN/m" }, reference_extension: { value: -0.025, unit: "mm" }, source_reference: "invented rod"
    });
    const outcome = await validateModelOperation(model, intent, await computeModelHash(model));
    expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
    // The engine refuses what the form cannot express, by the solver's names.
    const ideal = JSON.parse(intent.change.after);
    ideal.objective_connector.hardware.rods[0].law = "ideal_inextensible";
    const refused = await validateModelOperation(model, { ...intent, change: { ...intent.change, after: JSON.stringify(ideal) } }, await computeModelHash(model));
    expect(refused.diagnostics.map(d => d.message).join(" ")).toContain("JOINT_HARDWARE_LAW_UNSUPPORTED");
  });
  it("renders the rod table, adds and removes rows, and offers rods only on 0.9.0", () => {
    const model = structuredClone(authored); model.schema_version = "0.9.0";
    const update = vi.fn();
    const draft = { ...defaultComponentDraft(model, { type: "node", id: "node:A" }, [], "expansion_joint"), primaryPipeRef: "pipe:S1", ...connector,
      hardwareKind: "explicit_axial_rods_v1", tieRods: JSON.stringify([rodRow("rod:a", "0.1")]) } as ComponentDraft;
    const TextInput = ({ label, testId, value, onChange }: { label: string; testId: string; value: string; onChange: (value: string) => void }) =>
      <input aria-label={label} data-testid={testId} value={value} onChange={(event) => onChange(event.target.value)} />;
    render(<ConnectorDraftFields model={model} draft={draft} prefix="create" TextInput={TextInput} update={update} />);
    const hardware = screen.getByTestId("create-component-connector-hardware") as HTMLSelectElement;
    expect([...hardware.options].map(option => option.value)).toEqual(["", "untied", "explicit_axial_rods_v1"]);
    expect((screen.getByTestId("create-component-connector-tie-rod-0-id") as HTMLInputElement).value).toBe("rod:a");
    fireEvent.change(screen.getByTestId("create-component-connector-tie-rod-0-stiffness"), { target: { value: "20000" } });
    expect(JSON.parse(update.mock.calls.at(-1)![1])[0].stiffness).toBe("20000");
    fireEvent.click(screen.getByTestId("create-component-connector-tie-rod-add"));
    expect(JSON.parse(update.mock.calls.at(-1)![1])).toHaveLength(2);
    fireEvent.click(screen.getByTestId("create-component-connector-tie-rod-0-remove"));
    expect(update.mock.calls.at(-1)).toEqual(["tieRods", ""]);
    cleanup();
    const older = { ...model, schema_version: "0.5.0" } as PreviewModel;
    render(<ConnectorDraftFields model={older} draft={{ ...draft, hardwareKind: "" }} prefix="create" TextInput={TextInput} update={update} />);
    const options = [...(screen.getByTestId("create-component-connector-hardware") as HTMLSelectElement).options].map(option => option.value);
    expect(options).toEqual(["", "untied"]);
  });
});
