import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useEffect, useState } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { chooseVirtualMultiTarget, chooseVirtualTarget } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";

// Core unit-dimension validation is mocked so these cases exercise the panel's
// own drafting, readiness and intent-building; LoadCaseManagerPanel.units.test.tsx
// covers the validation request lifecycle itself.
const convert = vi.hoisted(() => vi.fn());
vi.mock("../../services/displayQuantityService", () => ({ convertDisplayQuantities: convert }));
beforeEach(() => convert.mockResolvedValue([{ id: "primitive-unit", status: "converted", value: 1, unit: "N" }]));
afterEach(() => { cleanup(); convert.mockReset(); });

/** Invented two-case model: every value is synthetic test input. */
function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "mm", force: "kN", moment: "kN*m", temperature: "C", angle: "rad" };
  model.nodes = [
    { id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "invented" },
    { id: "node:B", label: "Node B", position: { x: 1, y: 0, z: 0 }, provenance: "invented" }
  ];
  model.pipe_segments = [{ id: "pipe:AB", label: "Pipe AB", from: "node:A", to: "node:B", section: {}, material: "material:X", provenance: "invented" }];
  model.supports = [{ id: "support:A", label: "Support A", node: "node:A", restraints: ["UX"], provenance: "invented" }];
  model.load_cases = [
    { id: "load:dead", label: "Dead", kind: "sustained", status: "draft", provenance: "invented case", primitive_loads: [{
      id: "load:existing", category: "concentrated_force", target: { type: "node", node: "node:A" },
      direction: "global_y", magnitude: { value: 12, unit: "kN" }, dimension: "force", provenance: "invented load"
    }] },
    { id: "load:hot", label: "", kind: "thermal", status: "reviewed", provenance: "invented case", primitive_loads: [] }
  ];
  model.combinations = [
    { id: "combo:ops", label: "Operating", basis: "mechanics", terms: [{ load_case: "load:dead", factor: 1 }], provenance: "invented" },
    { id: "combo:legacy", label: "Legacy", basis: "custom_basis", terms: [], provenance: "invented" },
    { id: "combo:range", label: "Range", basis: "range_envelope", terms: [], provenance: "invented" }
  ];
  return model;
}

/** The panel as the workspace mounts it: a row chosen through onSelect becomes the selection. */
function Harness({ model, selection, queue, select }: {
  model: PreviewModel;
  selection: EntityRef;
  queue: (intent: EditorOperationIntent) => void;
  select: (ref: EntityRef) => void;
}) {
  const [current, setCurrent] = useState(selection);
  useEffect(() => setCurrent(selection), [selection]);
  return <LoadCaseManagerPanel model={model} onQueueIntent={queue} onSelect={(ref) => { select(ref); setCurrent(ref); }} selection={current} />;
}

function mount(model: PreviewModel = inventedModel(), selection: EntityRef = { type: "project", id: model.project.id }) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const select = vi.fn<(ref: EntityRef) => void>();
  const view = render(<Harness model={model} queue={queue} select={select} selection={selection} />);
  const rerender = (next: PreviewModel, nextSelection: EntityRef = selection) =>
    view.rerender(<Harness model={next} queue={queue} select={select} selection={nextSelection} />);
  return { model, queue, select, rerender };
}

const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });
const lastIntent = (queue: ReturnType<typeof mount>["queue"]) => queue.mock.calls.at(-1)![0];
const optionValues = (testId: string) => within(byId(testId)).getAllByRole("option").map((option) => (option as HTMLOptionElement).value);
/** A virtual target picker's shown value. */
const pickerValue = (testId: string) => byId(testId).querySelector(".virtual-target-picker-current");
const open = (testId: string) => fireEvent.click(byId(testId));
const row = (testId: string) => byId(testId).closest("tr")!;

describe("LoadCaseManagerPanel rendering states", () => {
  it("summarises an empty model and hides combination editors", () => {
    const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
    model.load_cases = [{ id: "load:only", label: "Only", kind: "sustained", status: "draft", provenance: "invented" }];
    mount(model);
    expect(byId("load-case-manager-summary")).toHaveTextContent("1 load cases; 0 primitive loads; 0 combinations");
    expect(byId("load-case-manager-empty")).toHaveTextContent("No loads in this case.");
    for (const absent of ["load-case-manager-combinations", "load-manager-add-term", "load-manager-selected-combination", "load-manager-selected-combination-term", "load-manager-selected-primitive"]) {
      expect(screen.queryByTestId(absent)).not.toBeInTheDocument();
    }
    expect(byId("load-manager-selected-case")).toHaveTextContent("Only");
    // A model with no nodes leaves the new primitive target unset.
    open("load-manager-add-load");
    expect(pickerValue("load-manager-create-primitive-node")).toHaveTextContent("No target selected");
  });

  it("lists cases, primitives and combinations with display fallbacks", () => {
    mount();
    expect(byId("load-case-manager-summary")).toHaveTextContent("2 load cases; 1 primitive loads; 3 combinations");
    // An empty label shows the case id.
    expect(within(byId("load-manager-case-load:hot")).getByText("load:hot", { selector: "strong" })).toBeInTheDocument();
    expect(byId("load-manager-case-load:hot")).toHaveTextContent("thermal · 0 loads");
    expect(byId("load-manager-case-load:dead")).toHaveTextContent("Deadsustained · 1 load");
    // The selected case's loads, one row each: type, applied to, direction, value, unit.
    expect([...row("load-manager-primitive-load:existing").cells].map((cell) => cell.textContent)).toEqual(["Force", "A", "Global Y", "12", "kN"]);
    // A load's edit opens only when its row is chosen.
    expect(byId("load-manager-primitive-load:existing")).toHaveAttribute("aria-expanded", "false");
    expect(screen.queryByTestId("load-manager-selected-primitive")).toBeNull();
    expect(byId("load-manager-combination-select-combo:ops")).toHaveTextContent("OperatingAlgebraic sumdead × 1");
    expect(byId("load-manager-combination-select-combo:legacy")).toHaveTextContent("Legacycustom_basisno terms");
    expect(byId("load-manager-combination-select-combo:range")).toHaveTextContent("RangeRange envelopemode not set; no operands");
    open("load-manager-combination-select-combo:ops");
    expect([...row("load-manager-combination-term-combo:ops-0").cells].map((cell) => cell.textContent)).toEqual(["dead", "1"]);
  });

  it("marks the selected case row active and reports row clicks through onSelect", () => {
    const { select } = mount(inventedModel(), { type: "load", id: "load:hot" });
    expect(byId("load-manager-case-load:hot")).toHaveClass("active");
    // The selected row is marked by more than colour: aria-current, and a bar drawn for it.
    expect(byId("load-manager-case-load:hot")).toHaveAttribute("aria-current", "true");
    expect(byId("load-manager-case-load:dead")).not.toHaveClass("active");
    expect(byId("load-manager-case-load:dead")).not.toHaveAttribute("aria-current");
    expect(byId("load-manager-selected-case")).toHaveTextContent("load:hot");
    fireEvent.click(byId("load-manager-case-load:dead"));
    expect(select).toHaveBeenLastCalledWith({ type: "load", id: "load:dead" });
    expect(byId("load-manager-case-load:dead")).toHaveAttribute("aria-current", "true");
    fireEvent.click(byId("load-manager-primitive-load:existing"));
    expect(select).toHaveBeenLastCalledWith({ type: "load", id: "load:dead" });
    fireEvent.click(byId("load-manager-combination-select-combo:legacy"));
    expect(select).toHaveBeenLastCalledWith({ type: "combination", id: "combo:legacy" });
    expect(byId("load-manager-combination-select-combo:legacy")).toHaveAttribute("aria-current", "true");
    expect(byId("load-manager-case-load:dead")).not.toHaveAttribute("aria-current");
    fireEvent.click(byId("load-manager-combination-select-combo:ops"));
    fireEvent.click(byId("load-manager-combination-term-combo:ops-0"));
    expect(select).toHaveBeenLastCalledWith({ type: "combination", id: "combo:ops" });
  });
});

describe("LoadCaseManagerPanel load-case creation", () => {
  it("proposes the next free id and queues a trimmed empty load-case shell", () => {
    const model = inventedModel();
    model.load_cases.push({ id: "load:L-300", label: "Taken", kind: "sustained", status: "draft", provenance: "invented" });
    const { queue } = mount(model);
    open("load-manager-new-case");
    expect(byId("load-manager-create-load-label")).toHaveFocus();
    expect(byId("load-manager-create-load-id")).toHaveValue("load:L-301");
    type("load-manager-create-load-id", "  load:new case  ");
    type("load-manager-create-load-label", "  New case ");
    fireEvent.click(byId("queue-create-load-case-intent"));
    expect(queue).toHaveBeenCalledOnce();
    // Queue closes the form.
    expect(byId("load-manager-create-load-case-form")).not.toHaveAttribute("open");
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      // A character outside [A-Za-z0-9_.:-] is escaped reversibly (space -> ~20), so distinct ids never share an operation id.
      operation_id: "op:load-manager-create-load:new~20case",
      operation_kind: "create",
      target: { object_type: "Load", ref: "load:new case" },
      change: { change_kind: "create_load_case", field_path: "load_cases", before: "not_present", unit: "none", dimension: "dimensionless" },
      validation: { unit_validation: "not_required_dimensionless" },
      audit_boundary: { direct_model_mutation_allowed: false, requires_user_acceptance: true }
    });
    expect(JSON.parse(intent.change.after)).toEqual({
      id: "load:new case", label: "New case", kind: "primitive_user_load", status: "draft", provenance: "user_entered_local_preview", primitive_loads: []
    });
  });

  it("refuses blank fields and existing ids with a reason", () => {
    mount();
    open("load-manager-new-case");
    const button = byId("queue-create-load-case-intent");
    expect(button).toBeEnabled();
    for (const field of ["label", "kind", "status", "provenance"]) {
      const testId = `load-manager-create-load-${field}`;
      const before = (byId(testId) as HTMLInputElement).value;
      type(testId, "   ");
      expect(button).toBeDisabled();
      expect(byId("load-manager-create-load-preview")).toHaveTextContent("complete id/label/kind/status/provenance to queue an empty load case");
      type(testId, before);
    }
    type("load-manager-create-load-id", " load:dead ");
    expect(button).toBeDisabled();
    expect(byId("load-manager-create-load-preview")).toHaveTextContent("id=load:dead already exists; no load case queued");
  });

  it("re-proposes the draft id when a model update takes it, as the combination draft does", () => {
    const { model, rerender } = mount();
    open("load-manager-new-case");
    expect(byId("load-manager-create-load-id")).toHaveValue("load:L-300");
    const next = structuredClone(model);
    next.load_cases.push({ id: "load:L-300", label: "Applied", kind: "sustained", status: "draft", provenance: "invented" });
    rerender(next);
    expect(byId("load-manager-create-load-id")).toHaveValue("load:L-301");
    expect(byId("queue-create-load-case-intent")).toBeEnabled();
  });
});

describe("LoadCaseManagerPanel load-case metadata and deletion", () => {
  it("orders metadata choices and queues only a changed value", () => {
    const { queue } = mount();
    expect(optionValues("load-manager-metadata-value")).toEqual(["draft", "preview_only", "reviewed", "TBD"]);
    expect(byId("queue-load-metadata-intent")).toBeDisabled();
    expect(byId("load-manager-metadata-preview")).toHaveTextContent("current=draft; no changed status queued");
    type("load-manager-metadata-value", "reviewed");
    type("load-manager-metadata-rationale", "   ");
    fireEvent.click(byId("queue-load-metadata-intent"));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-load:dead-status",
      operation_kind: "modify",
      target: { object_type: "Load", ref: "load:dead" },
      change: { change_kind: "update_load", field_label: "Load case status", field_path: "status", before: "draft", after: "reviewed" },
      // A blank rationale falls back to a generated one.
      rationale: expect.stringMatching(/^load case metadata edit intent for project:blank-local-/)
    });
  });

  it("switching the field resets the proposal to the current kind", () => {
    mount();
    type("load-manager-metadata-value", "reviewed");
    type("load-manager-metadata-field", "kind");
    expect(byId("load-manager-metadata-value")).toHaveValue("sustained");
    // Model document 0.9.0 §2.3: "operating" is offered among the kind suggestions.
    expect(optionValues("load-manager-metadata-value")).toEqual(["sustained", "operating", "primitive_user_load", "thermal", "TBD"]);
    expect(byId("queue-load-metadata-intent")).toBeDisabled();
    expect(byId("load-manager-metadata-preview")).toHaveTextContent("current=sustained; no changed kind queued");
  });

  it("queues a whole-case delete with the engine before-display and requires a rationale", () => {
    const { queue } = mount();
    type("load-manager-load-case-delete-rationale", "  ");
    expect(byId("queue-delete-load-case-intent")).toBeDisabled();
    expect(byId("load-manager-load-case-delete-preview")).toHaveTextContent("select a load case and provide rationale to queue deletion");
    type("load-manager-load-case-delete-rationale", "  invented removal  ");
    fireEvent.click(byId("queue-delete-load-case-intent"));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-load:dead-delete",
      operation_kind: "delete",
      change: { change_kind: "delete_load_case", field_label: "Dead load case", field_path: "load_cases", before: "load:dead; Dead; sustained; draft; primitives=1", after: "not_present" },
      rationale: "invented removal"
    });
  });

  it("labels the delete of an unlabelled case by its id while keeping its empty label in the before-value", () => {
    const { queue } = mount(inventedModel(), { type: "load", id: "load:hot" });
    fireEvent.click(byId("queue-delete-load-case-intent"));
    expect(lastIntent(queue).change).toMatchObject({ field_label: "load:hot load case", before: "load:hot; ; thermal; reviewed; primitives=0" });
  });
});

describe("LoadCaseManagerPanel primitive-load creation", () => {
  it("defaults a concentrated force on the first node of the first case", async () => {
    const { queue } = mount();
    open("load-manager-add-load");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-F300");
    expect(byId("load-manager-create-primitive-category")).toHaveValue("concentrated_force");
    expect(pickerValue("load-manager-create-primitive-node")).toHaveTextContent("Node A (node:A)");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("kN");
    expect(byId("load-manager-create-primitive-direction")).toHaveValue("global_y");
    expect(optionValues("load-manager-create-primitive-unit")).toEqual(["kN"]);
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    expect(byId("load-manager-add-load-form")).not.toHaveAttribute("open");
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:load-manager-load:dead-load:dead-F300-primitive",
      target: { object_type: "Load", ref: "load:dead" },
      change: { change_kind: "create_primitive_load", field_label: "Concentrated force primitive load", field_path: "primitive_loads", unit: "kN", dimension: "force" },
      validation: { unit_validation: "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview" }
    });
    expect(JSON.parse(intent.change.after)).toEqual({
      id: "load:dead-F300", category: "concentrated_force", target: { type: "node", node: "node:A" }, direction: "global_y",
      magnitude: { value: 250, unit: "kN" }, dimension: "force", provenance: "user_entered_local_preview"
    });
  });

  it("re-derives target, direction, id and unit when the category or DOF changes", async () => {
    const { queue } = mount();
    open("load-manager-add-load");
    type("load-manager-create-primitive-category", "imposed_displacement");
    expect(byId("load-manager-create-primitive-support")).toBeInTheDocument();
    expect(screen.queryByTestId("load-manager-create-primitive-node")).not.toBeInTheDocument();
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-I300");
    expect(byId("load-manager-create-primitive-direction")).toHaveValue("UZ");
    expect(optionValues("load-manager-create-primitive-direction")).toEqual(["UX", "UY", "UZ", "RX", "RY", "RZ"]);
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("mm");
    type("load-manager-create-primitive-direction", "RX");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("rad");
    await waitFor(() => expect(convert).toHaveBeenLastCalledWith([expect.objectContaining({ from_unit: "rad", dimension_id: "rotation" })]));
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
    const intent = lastIntent(queue);
    expect(intent.change).toMatchObject({ field_label: "Imposed displacement primitive load", unit: "rad", dimension: "rotation" });
    expect(JSON.parse(intent.change.after)).toMatchObject({ target: { type: "support", support: "support:A", dof: "RX" }, direction: "RX", magnitude: { value: 250, unit: "rad" } });

    type("load-manager-create-primitive-category", "thermal");
    expect(byId("load-manager-create-primitive-pipe")).toBeInTheDocument();
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-T300");
    expect(byId("load-manager-create-primitive-direction")).toHaveValue("global_z");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("degC");
    expect(pickerValue("load-manager-create-primitive-pipe")).toHaveTextContent("Pipe AB (pipe:AB)");

    type("load-manager-create-primitive-category", "concentrated_moment");
    expect(byId("load-manager-create-primitive-node")).toBeInTheDocument();
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-M300");
    // the moment default is the moment key, not kN*mm composed from force and length.
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("kN*m");

    type("load-manager-create-primitive-category", "distributed_force");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-D300");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("kN/mm");
  });

  it("re-derives the id when the case changes and skips ids already in use", async () => {
    const model = inventedModel();
    model.load_cases[1].primitive_loads = [{ id: "load:hot-F300", category: "concentrated_force", target: { type: "node", node: "node:B" }, direction: "global_x", magnitude: { value: 1, unit: "kN" }, dimension: "force", provenance: "invented" }];
    mount(model);
    open("load-manager-add-load");
    chooseVirtualTarget("load-manager-create-primitive-load-case", "load:hot");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:hot-F301");
  });

  it("re-proposes the draft id when a model update takes it, as the load-case draft does", async () => {
    const { model, rerender } = mount();
    open("load-manager-add-load");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-F300");
    const next = structuredClone(model);
    next.load_cases[0].primitive_loads!.push({
      id: "load:dead-F300", category: "concentrated_force", target: { type: "node", node: "node:B" },
      direction: "global_x", magnitude: { value: 1, unit: "kN" }, dimension: "force", provenance: "invented"
    });
    rerender(next);
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-F301");
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    // A free id the user entered is kept across a model update.
    type("load-manager-create-primitive-id", "load:dead-custom");
    rerender(structuredClone(next));
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-custom");
  });

  it("refuses zero, non-numeric, duplicate and incomplete primitive drafts", async () => {
    mount();
    open("load-manager-add-load");
    const button = byId("queue-create-primitive-intent");
    await waitFor(() => expect(button).toBeEnabled());
    const help = "complete case/id/target/direction/nonzero magnitude/provenance to queue a primitive load";
    for (const magnitude of ["0", "-0", "abc", "1e", "", "Infinity", "0x10"]) {
      type("load-manager-create-primitive-magnitude", magnitude);
      expect(button).toBeDisabled();
      expect(byId("load-manager-create-primitive-preview")).toHaveTextContent(help);
    }
    for (const magnitude of ["-2.5", ".5", "5.", "1e3", " 7 "]) {
      type("load-manager-create-primitive-magnitude", magnitude);
      expect(button).toBeEnabled();
    }
    type("load-manager-create-primitive-provenance", " ");
    expect(button).toBeDisabled();
    type("load-manager-create-primitive-provenance", "invented");
    type("load-manager-create-primitive-id", " load:existing ");
    expect(button).toBeDisabled();
    expect(byId("load-manager-create-primitive-preview")).toHaveTextContent("id=load:existing already exists; no primitive load queued");
  });

  it("refuses a primitive whose project unit is undeclared", async () => {
    const model = inventedModel();
    model.project.units = { length: "mm" };
    mount(model);
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("TBD");
    await waitFor(() => expect(convert).toHaveBeenCalled());
    await waitFor(() => expect(byId("load-manager-create-primitive-preview")).toHaveTextContent("complete case/id/target"));
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
  });

  it("reports a rejected unit-validation request and keeps the queue closed", async () => {
    convert.mockReset();
    convert.mockRejectedValue(new Error("invented bridge failure"));
    mount();
    await waitFor(() => expect(byId("load-manager-create-primitive-preview")).toHaveTextContent("Unit validation unavailable: Error: invented bridge failure"));
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
  });

  it("treats a reply without the requested result as unavailable", async () => {
    convert.mockReset();
    convert.mockResolvedValue([{ id: "some-other-request", status: "converted", value: 1, unit: "N" }]);
    mount();
    await waitFor(() => expect(byId("load-manager-create-primitive-preview")).toHaveTextContent(/^Unit validation unavailable$/));
    expect(byId("queue-create-primitive-intent")).toBeDisabled();
  });
});

describe("LoadCaseManagerPanel existing primitive editing", () => {
  it("queues a changed magnitude with the stored unit and dimension", () => {
    const { queue } = mount();
    open("load-manager-primitive-load:existing");
    expect(byId("load-manager-primitive-load:existing")).toHaveAttribute("aria-expanded", "true");
    expect(byId("load-manager-selected-primitive")).toHaveTextContent("load:existing");
    expect(byId("queue-load-magnitude-intent")).toBeDisabled();
    expect(byId("load-manager-edit-preview")).toHaveTextContent("current=12 kN; no changed magnitude queued");
    type("load-manager-magnitude-value", " 15.5 ");
    type("load-manager-rationale", " invented edit ");
    fireEvent.click(byId("queue-load-magnitude-intent"));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-load:dead-load:existing-magnitude",
      target: { object_type: "Load", ref: "load:dead" },
      change: { field_label: "concentrated_force primitive magnitude", field_path: "primitive_loads.0.magnitude.value", before: "12", after: JSON.stringify({ value: 15.5, unit: "kN" }), unit: "kN", dimension: "force" },
      rationale: "invented edit"
    });
  });

  it("refuses a non-numeric, blank or zero magnitude, as the create form does", () => {
    const { queue } = mount();
    open("load-manager-primitive-load:existing");
    for (const magnitude of ["abc", "1e", "", "   ", "Infinity", "0x10", "0", "-0"]) {
      type("load-manager-magnitude-value", magnitude);
      expect(byId("queue-load-magnitude-intent")).toBeDisabled();
      expect(byId("load-manager-edit-preview")).toHaveTextContent("current=12 kN; enter a finite nonzero magnitude; no changed magnitude queued");
    }
    type("load-manager-magnitude-value", "-2.5");
    fireEvent.click(byId("queue-load-magnitude-intent"));
    expect(lastIntent(queue).change.after).toBe(JSON.stringify({ value: -2.5, unit: "kN" }));
  });

  it("queues an indexed primitive delete with the full before-display", () => {
    const { queue } = mount();
    open("load-manager-primitive-load:existing");
    type("load-manager-primitive-delete-rationale", "");
    expect(byId("queue-delete-primitive-load-intent")).toBeDisabled();
    expect(byId("load-manager-primitive-delete-preview")).toHaveTextContent("select a primitive load and provide rationale to queue deletion");
    type("load-manager-primitive-delete-rationale", "invented delete");
    fireEvent.click(byId("queue-delete-primitive-load-intent"));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-load:dead-load:existing-delete",
      operation_kind: "delete",
      change: { change_kind: "delete_primitive_load", field_path: "primitive_loads.0", before: "load:existing; concentrated_force; node:node:A; global_y; 12 kN; force", after: "not_present", unit: "kN", dimension: "force" }
    });
  });

  it("shows TBD for a primitive without magnitude, target or dimension", () => {
    const model = inventedModel();
    model.load_cases[0].primitive_loads = [{ id: "load:sparse", category: "concentrated_force" }];
    mount(model);
    expect([...row("load-manager-primitive-load:sparse").cells].map((cell) => cell.textContent)).toEqual(["Force", "not set", "TBD", "TBD", "TBD"]);
    open("load-manager-primitive-load:sparse");
    expect(byId("load-manager-edit-preview")).toHaveTextContent("current=TBD TBD; no changed magnitude queued");
  });

  it("falls back to the empty state when a model update removes the selected primitive", () => {
    const { model, rerender } = mount();
    open("load-manager-primitive-load:existing");
    expect(byId("load-manager-selected-primitive")).toBeInTheDocument();
    const next = structuredClone(model);
    next.load_cases[0].primitive_loads = [];
    rerender(next);
    expect(screen.queryByTestId("load-manager-selected-primitive")).not.toBeInTheDocument();
    expect(byId("load-case-manager-empty")).toBeInTheDocument();
  });
});

describe("LoadCaseManagerPanel combination creation", () => {
  it("queues an explicitly entered mechanics combination and refuses a non-finite factor or existing id", () => {
    const { queue } = mount();
    open("load-manager-new-combination");
    expect(byId("load-manager-create-combination-label")).toHaveFocus();
    expect(byId("load-manager-create-combination-id")).toHaveValue("combination:C-300");
    // Nothing is proposed: no first case, no factor 1.
    expect(byId("load-manager-create-combination-basis")).toHaveValue("mechanics");
    expect(pickerValue("load-manager-create-combination-load-case")).toHaveTextContent("No target selected");
    expect(byId("load-manager-create-combination-factor")).toHaveValue("");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("term 1: not chosen: its load case");
    chooseVirtualTarget("load-manager-create-combination-load-case", "load:dead");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("term 1: not entered: its factor");
    for (const factor of ["1e", "", "abc"]) {
      type("load-manager-create-combination-factor", factor);
      expect(byId("queue-create-combination-intent")).toBeDisabled();
      expect(byId("load-manager-create-combination-preview")).toHaveTextContent("complete id/label/load case/finite factor/provenance/rationale to queue a mechanics combination");
    }
    // A zero factor is a valid explicit term.
    type("load-manager-create-combination-factor", "0");
    expect(byId("queue-create-combination-intent")).toBeEnabled();
    type("load-manager-create-combination-factor", "-1.5");
    fireEvent.click(byId("queue-create-combination-intent"));
    expect(byId("load-manager-create-combination-form")).not.toHaveAttribute("open");
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:load-manager-create-combination:C-300",
      target: { object_type: "Combination", ref: "combination:C-300" },
      change: { change_kind: "create_combination", source_note: "explicit user-entered mechanics combination; no code/rule combination default inferred" },
      rationale: "user_entered_combination_create_preview_change"
    });
    expect(JSON.parse(intent.change.after)).toEqual({
      id: "combination:C-300", label: "User mechanics combination", basis: "mechanics", provenance: "user_entered_local_preview",
      terms: [{ load_case: "load:dead", factor: -1.5 }]
    });
    type("load-manager-create-combination-id", "combo:ops");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("id=combo:ops already exists; no combination queued");
  });

  it("requires distinct minuend and subtrahend for a subtraction", () => {
    const { queue } = mount();
    open("load-manager-new-combination");
    type("load-manager-create-combination-basis", "result_state_subtraction");
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("Minuend: not chosen. Subtrahend: not chosen.");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("not chosen: the minuend");
    chooseVirtualTarget("load-manager-create-combination-minuend", "load:dead");
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("not chosen: the subtrahend");
    chooseVirtualTarget("load-manager-create-combination-subtrahend", "load:hot");
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("Minuend: load:dead resolves to the load case. Subtrahend: load:hot resolves to the load case.");
    fireEvent.click(byId("queue-create-combination-intent"));
    expect(JSON.parse(lastIntent(queue).change.after)).toEqual({
      id: "combination:C-300", label: "User mechanics combination", basis: "result_state_subtraction", provenance: "user_entered_local_preview",
      minuend_id: "load:dead", subtrahend_id: "load:hot", terms: []
    });
    chooseVirtualTarget("load-manager-create-combination-subtrahend", "load:dead");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("select distinct minuend/subtrahend load cases");
  });

  it("offers no default subtrahend for a single-case model", () => {
    const model = inventedModel();
    model.load_cases = [model.load_cases[0]];
    mount(model);
    open("load-manager-new-combination");
    type("load-manager-create-combination-basis", "result_state_subtraction");
    chooseVirtualTarget("load-manager-create-combination-minuend", "load:dead");
    expect(pickerValue("load-manager-create-combination-subtrahend")).toHaveTextContent("No target selected");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
  });

  it("queues a range envelope with its operands and mode", () => {
    const { queue } = mount();
    open("load-manager-new-combination");
    type("load-manager-create-combination-basis", "range_envelope");
    expect(byId("load-manager-create-combination-mode")).toHaveValue("");
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("Operands: none chosen.");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("not chosen: any operand");
    chooseVirtualMultiTarget("load-manager-create-combination-operands", "load:dead");
    expect(byId("load-manager-create-combination-preview")).toHaveTextContent("not chosen: the mode");
    type("load-manager-create-combination-mode", "min");
    expect(byId("load-manager-create-combination-resolved")).toHaveTextContent("Operands: load:dead resolves to the load case.");
    fireEvent.click(byId("queue-create-combination-intent"));
    expect(JSON.parse(lastIntent(queue).change.after)).toEqual({
      id: "combination:C-300", label: "User mechanics combination", basis: "range_envelope", provenance: "user_entered_local_preview",
      operand_ids: ["load:dead"], mode: "min", terms: []
    });
  });

  it("re-proposes the draft id when a model update takes it", () => {
    const { model, rerender } = mount();
    open("load-manager-new-combination");
    const next = structuredClone(model);
    next.combinations!.push({ id: "combination:C-300", label: "Applied", basis: "mechanics", terms: [], provenance: "invented" });
    rerender(next);
    expect(byId("load-manager-create-combination-id")).toHaveValue("combination:C-301");
    chooseVirtualTarget("load-manager-create-combination-load-case", "load:dead");
    type("load-manager-create-combination-factor", "1");
    expect(byId("queue-create-combination-intent")).toBeEnabled();
  });
});

describe("LoadCaseManagerPanel combination editing", () => {
  it("keeps a stored non-closed basis visible and queues a basis change", () => {
    const { queue } = mount(inventedModel(), { type: "combination", id: "combo:legacy" });
    expect(byId("load-manager-selected-combination")).toHaveTextContent("Legacy");
    expect(within(byId("load-manager-combination-basis-value")).getByRole("option", { name: "custom_basis (stored)" })).toBeInTheDocument();
    expect(byId("queue-combination-basis-intent")).toBeDisabled();
    expect(byId("load-manager-combination-basis-preview")).toHaveTextContent("current=custom_basis; no changed combination basis queued");
    type("load-manager-combination-basis-value", "mechanics");
    fireEvent.click(byId("queue-combination-basis-intent"));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-combo:legacy-basis",
      target: { object_type: "Combination", ref: "combo:legacy" },
      change: { field_label: "Legacy combination basis", field_path: "basis", before: "custom_basis", after: "mechanics" }
    });
  });

  it("queues a whole-combination delete with the engine before-display", async () => {
    const { queue } = mount(inventedModel(), { type: "combination", id: "combo:legacy" });
    // The delete is checked by the operation engine before it is queued.
    fireEvent.click(byId("queue-delete-combination-intent"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(lastIntent(queue).change).toMatchObject({ change_kind: "delete_combination", field_label: "Legacy combination", before: "combo:legacy; Legacy; basis=custom_basis; terms=none", after: "not_present" });
    cleanup();
    const second = mount(inventedModel(), { type: "combination", id: "combo:ops" });
    type("load-manager-combination-entity-delete-rationale", " ");
    expect(byId("queue-delete-combination-intent")).toBeDisabled();
    expect(byId("load-manager-combination-entity-delete-preview")).toHaveTextContent("select a combination and provide rationale to queue deletion");
    type("load-manager-combination-entity-delete-rationale", "invented");
    fireEvent.click(byId("queue-delete-combination-intent"));
    await waitFor(() => expect(second.queue).toHaveBeenCalledOnce());
    expect(lastIntent(second.queue).change.before).toBe("combo:ops; Operating; basis=mechanics; terms=load:dead x 1");
  });

  it("edits and deletes an indexed combination term", async () => {
    const { queue } = mount();
    open("load-manager-combination-select-combo:ops");
    open("load-manager-combination-term-combo:ops-0");
    expect(byId("load-manager-combination-term-combo:ops-0")).toHaveAttribute("aria-expanded", "true");
    expect(byId("load-manager-selected-combination-term")).toBeInTheDocument();
    expect(byId("queue-combination-factor-intent")).toBeDisabled();
    type("load-manager-combination-factor-value", "  ");
    expect(byId("queue-combination-factor-intent")).toBeDisabled();
    expect(byId("load-manager-combination-factor-preview")).toHaveTextContent("current=1; no changed combination factor queued");
    type("load-manager-combination-factor-value", "1.5");
    fireEvent.click(byId("queue-combination-factor-intent"));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-combo:ops-term-0-factor",
      change: { field_label: "load:dead combination factor", field_path: "terms.0.factor", before: "1", after: "1.5", unit: "none", dimension: "dimensionless" }
    });
    fireEvent.click(byId("queue-delete-combination-term-intent"));
    await waitFor(() => expect(queue).toHaveBeenCalledTimes(2));
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-combo:ops-term-0-delete",
      operation_kind: "delete",
      change: { change_kind: "delete_combination_term", field_path: "terms.0", before: "load:dead x 1", after: "not_present" }
    });
  });

  it("refuses a non-numeric existing-term factor, as the create forms do", () => {
    const { queue } = mount();
    open("load-manager-combination-select-combo:ops");
    open("load-manager-combination-term-combo:ops-0");
    for (const factor of ["abc", "1e", "Infinity", "0x10"]) {
      type("load-manager-combination-factor-value", factor);
      expect(byId("queue-combination-factor-intent")).toBeDisabled();
      expect(byId("load-manager-combination-factor-preview")).toHaveTextContent("current=1; enter a finite factor; no changed combination factor queued");
    }
    // A zero factor is a valid explicit term.
    type("load-manager-combination-factor-value", "0");
    fireEvent.click(byId("queue-combination-factor-intent"));
    expect(lastIntent(queue).change.after).toBe("0");
  });

  it("drafts a new term only from an explicitly chosen case and entered factor", () => {
    const { queue } = mount();
    open("load-manager-combination-select-combo:ops");
    open("load-manager-add-term");
    expect(pickerValue("load-manager-create-combination-term-combination")).toHaveTextContent("Operating (combo:ops)");
    // No prefill: neither a case nor a factor is proposed.
    expect(pickerValue("load-manager-create-combination-term-load-case")).toHaveTextContent("No target selected");
    expect(byId("load-manager-create-combination-term-factor")).toHaveValue("");
    expect(byId("queue-create-combination-term-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-term-preview")).toHaveTextContent("not chosen: the load case the new term names");
    chooseVirtualTarget("load-manager-create-combination-term-load-case", "load:hot");
    expect(byId("queue-create-combination-term-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-term-preview")).toHaveTextContent("not entered: the factor");
    type("load-manager-create-combination-term-factor", "1");
    fireEvent.click(byId("queue-create-combination-term-intent"));
    expect(byId("load-manager-add-term-form")).not.toHaveAttribute("open");
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:load-manager-combo:ops-term-1-create",
      target: { object_type: "Combination", ref: "combo:ops" },
      change: { change_kind: "create_combination_term", field_path: "terms", before: "not_present", after: JSON.stringify({ load_case: "load:hot", factor: 1 }) }
    });
    // A new target combination clears the chosen case; the entered factor stays as typed.
    open("load-manager-add-term");
    chooseVirtualTarget("load-manager-create-combination-term-combination", "combo:legacy");
    expect(pickerValue("load-manager-create-combination-term-load-case")).toHaveTextContent("No target selected");
    expect(byId("load-manager-create-combination-term-factor")).toHaveValue("1");
    chooseVirtualTarget("load-manager-create-combination-term-load-case", "load:dead");
    type("load-manager-create-combination-term-factor", "x");
    expect(byId("queue-create-combination-term-intent")).toBeDisabled();
    expect(byId("load-manager-create-combination-term-preview")).toHaveTextContent("select combination/load case and finite factor to queue a combination term");
    type("load-manager-create-combination-term-factor", "2");
    type("load-manager-create-combination-term-rationale", " ");
    expect(byId("queue-create-combination-term-intent")).toBeDisabled();
  });

  it("retargets the term draft when a combination row is selected", () => {
    mount();
    fireEvent.click(byId("load-manager-combination-select-combo:range"));
    open("load-manager-add-term");
    expect(pickerValue("load-manager-create-combination-term-combination")).toHaveTextContent("Range (combo:range)");
    expect(pickerValue("load-manager-create-combination-term-load-case")).toHaveTextContent("No target selected");
  });
});

describe("LoadCaseManagerPanel layout", () => {
  it("shows what exists first and opens each creation form only on its command", () => {
    const model = inventedModel();
    model.load_cases[0].pressure_regions = [];
    model.load_cases[0].condition_use = { temperature: "T1" };
    mount(model);
    for (const form of ["load-manager-create-load-case-form", "load-manager-create-combination-form", "load-manager-seismic-template-form", "load-manager-add-load-form"]) {
      expect(byId(form)).not.toHaveAttribute("open");
    }
    expect(byId("load-manager-analysis-settings")).not.toHaveAttribute("open");
    // The editor names the entity it holds by its full id, for a journey that must know which case is chosen.
    expect(byId("load-manager-selected-case")).toHaveAttribute("data-entity-id", model.load_cases[0].id);
    // The selected case's sections, in order; a disclosure states what the case has and is open when it has content.
    const sections = within(byId("load-manager-selected-case")).getAllByTestId(/^load-case-section-/).map((section) => section.dataset.testid);
    expect(sections).toEqual(["load-case-section-case", "load-case-section-loads", "load-case-section-pressure", "load-case-section-wind", "load-case-section-seismic", "load-case-section-hydrostatic-test", "load-case-section-conditions", "load-case-section-reference-state"]);
    expect(byId("load-case-section-pressure")).toHaveTextContent("Pressure: none (declared empty)");
    expect(byId("load-case-section-wind")).toHaveTextContent("Wind: none");
    expect(byId("load-case-section-seismic")).toHaveTextContent("Seismic: none");
    expect(byId("load-case-section-hydrostatic-test")).toHaveTextContent("Hydrostatic test: off");
    expect(byId("load-case-section-hydrostatic-test")).not.toHaveAttribute("open");
    expect(byId("load-case-section-pressure")).not.toHaveAttribute("open");
    expect(byId("load-case-section-conditions")).toHaveTextContent("Conditions: temperature T1");
    expect(byId("load-case-section-conditions")).toHaveAttribute("open");
    // Identifier, status and provenance are in a closed "More" inside their form.
    open("load-manager-new-case");
    expect(byId("load-manager-new-case")).toHaveAttribute("aria-expanded", "true");
    expect(byId("load-manager-create-load-case-form")).toHaveAttribute("open");
    expect(byId("load-manager-create-load-more")).not.toHaveAttribute("open");
    expect(byId("load-manager-create-load-more")).toContainElement(byId("load-manager-create-load-id"));
    open("load-manager-create-load-cancel");
    expect(byId("load-manager-create-load-case-form")).not.toHaveAttribute("open");
  });

  it("offers the templates from one command and closes the template on Cancel", () => {
    mount();
    open("load-manager-templates");
    open("load-manager-template-seismic");
    expect(byId("load-manager-seismic-template-form")).toHaveAttribute("open");
    expect(screen.queryByTestId("load-manager-template-seismic")).toBeNull();
    open("load-manager-seismic-template-cancel");
    expect(byId("load-manager-seismic-template-form")).not.toHaveAttribute("open");
    open("load-manager-templates");
    open("load-manager-template-operating-difference");
    expect(byId("load-manager-create-combination-form")).toHaveAttribute("open");
    expect(byId("load-manager-create-combination-basis")).toHaveValue("result_state_subtraction");
    expect(byId("load-manager-operating-difference-toggle")).toBeChecked();
  });

  it("opens Add load with the selected case chosen", () => {
    const model = inventedModel();
    model.load_cases[1].primitive_loads = [];
    mount(model, { type: "load", id: "load:hot" });
    open("load-manager-add-load");
    expect(pickerValue("load-manager-create-primitive-load-case")).toHaveTextContent("(load:hot)");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:hot-F300");
  });
});
