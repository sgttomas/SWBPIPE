import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { chooseVirtualTarget } from "../../test-support/workspaceTestControls";
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

function mount(model: PreviewModel = inventedModel(), selection: EntityRef = { type: "project", id: model.project.id }) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const select = vi.fn<(ref: EntityRef) => void>();
  const view = render(<LoadCaseManagerPanel model={model} onQueueIntent={queue} onSelect={select} selection={selection} />);
  const rerender = (next: PreviewModel, nextSelection: EntityRef = selection) =>
    view.rerender(<LoadCaseManagerPanel model={next} onQueueIntent={queue} onSelect={select} selection={nextSelection} />);
  return { model, queue, select, rerender };
}

const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });
const lastIntent = (queue: ReturnType<typeof mount>["queue"]) => queue.mock.calls.at(-1)![0];
const optionValues = (testId: string) => within(byId(testId)).getAllByRole("option").map((option) => (option as HTMLOptionElement).value);

describe("LoadCaseManagerPanel rendering states", () => {
  it("summarises an empty model and hides combination editors", () => {
    const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
    model.load_cases = [{ id: "load:only", label: "Only", kind: "sustained", status: "draft", provenance: "invented" }];
    mount(model);
    expect(byId("load-case-manager-summary")).toHaveTextContent("1 load cases; 0 primitive loads; 0 combinations;");
    expect(byId("load-case-manager-empty")).toHaveTextContent("No primitive loads are available in the current model.");
    for (const absent of ["load-case-manager-combinations", "load-manager-create-combination-term-heading", "load-manager-selected-combination", "load-manager-selected-combination-term", "load-manager-selected-primitive"]) {
      expect(screen.queryByTestId(absent)).not.toBeInTheDocument();
    }
    expect(byId("load-manager-selected-case")).toHaveTextContent("load:only");
    // A model with no nodes leaves the new primitive target unset.
    expect(byId("load-manager-create-primitive-heading")).toHaveTextContent("target=node:TBD");
  });

  it("lists cases, primitives and combinations with display fallbacks", () => {
    mount();
    expect(byId("load-case-manager-summary")).toHaveTextContent("2 load cases; 1 primitive loads; 3 combinations;");
    // An empty label shows the case id.
    expect(within(byId("load-manager-case-load:hot")).getByText("load:hot", { selector: "strong" })).toBeInTheDocument();
    expect(byId("load-manager-case-load:dead")).toHaveTextContent("load:dead; sustained; draft; primitives=1");
    expect(byId("load-manager-primitive-load:existing")).toHaveTextContent("concentrated_force; 12 kN");
    expect(byId("load-manager-primitive-load:existing")).toHaveTextContent("load:existing; node:node:A; global_y; dimension=force");
    expect(byId("load-manager-primitive-load:existing")).toHaveAttribute("aria-pressed", "true");
    expect(byId("load-manager-combination-select-combo:ops")).toHaveTextContent("combo:ops; basis=mechanics; terms=load:dead x 1");
    expect(byId("load-manager-combination-select-combo:legacy")).toHaveTextContent("combo:legacy; basis=custom_basis; terms=");
    expect(byId("load-manager-combination-select-combo:range")).toHaveTextContent("combo:range; basis=range_envelope; mode=TBD; operands=TBD");
    expect(byId("load-manager-combination-term-combo:ops-0")).toHaveTextContent("load:dead x 1");
  });

  it("marks the selected case row active and reports row clicks through onSelect", () => {
    const { select } = mount(inventedModel(), { type: "load", id: "load:hot" });
    expect(byId("load-manager-case-load:hot")).toHaveClass("active");
    expect(byId("load-manager-case-load:dead")).not.toHaveClass("active");
    expect(byId("load-manager-selected-case")).toHaveTextContent("load:hot");
    fireEvent.click(byId("load-manager-case-load:dead"));
    expect(select).toHaveBeenLastCalledWith({ type: "load", id: "load:dead" });
    fireEvent.click(byId("load-manager-primitive-load:existing"));
    expect(select).toHaveBeenLastCalledWith({ type: "load", id: "load:dead" });
    fireEvent.click(byId("load-manager-combination-select-combo:legacy"));
    expect(select).toHaveBeenLastCalledWith({ type: "combination", id: "combo:legacy" });
    fireEvent.click(byId("load-manager-combination-term-combo:ops-0"));
    expect(select).toHaveBeenLastCalledWith({ type: "combination", id: "combo:ops" });
  });
});

describe("LoadCaseManagerPanel load-case creation", () => {
  it("proposes the next free id and queues a trimmed empty load-case shell", () => {
    const model = inventedModel();
    model.load_cases.push({ id: "load:L-300", label: "Taken", kind: "sustained", status: "draft", provenance: "invented" });
    const { queue } = mount(model);
    expect(byId("load-manager-create-load-id")).toHaveValue("load:L-301");
    type("load-manager-create-load-id", "  load:new case  ");
    type("load-manager-create-load-label", "  New case ");
    fireEvent.click(byId("queue-create-load-case-intent"));
    expect(queue).toHaveBeenCalledOnce();
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:load-manager-create-load:new-case",
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
    expect(byId("load-manager-selected-case")).toHaveTextContent("field=status; current=draft; path=status");
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
    expect(optionValues("load-manager-metadata-value")).toEqual(["sustained", "primitive_user_load", "thermal", "TBD"]);
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
    expect(byId("load-manager-create-primitive-heading")).toHaveTextContent("load:dead-F300");
    expect(byId("load-manager-create-primitive-heading")).toHaveTextContent("concentrated_force; target=node:A; unit=kN");
    expect(byId("load-manager-create-primitive-direction")).toHaveValue("global_y");
    expect(optionValues("load-manager-create-primitive-unit")).toEqual(["kN"]);
    await waitFor(() => expect(byId("queue-create-primitive-intent")).toBeEnabled());
    fireEvent.click(byId("queue-create-primitive-intent"));
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
    expect(byId("load-manager-create-primitive-heading")).toHaveTextContent("thermal; target=pipe:AB; unit=degC");

    type("load-manager-create-primitive-category", "concentrated_moment");
    expect(byId("load-manager-create-primitive-node")).toBeInTheDocument();
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-M300");
    // W-17 §2.4: the moment default is the moment key, not kN*mm composed from force and length.
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("kN*m");

    type("load-manager-create-primitive-category", "distributed_force");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:dead-D300");
    expect(byId("load-manager-create-primitive-unit")).toHaveValue("kN/mm");
  });

  it("re-derives the id when the case changes and skips ids already in use", async () => {
    const model = inventedModel();
    model.load_cases[1].primitive_loads = [{ id: "load:hot-F300", category: "concentrated_force", target: { type: "node", node: "node:B" }, direction: "global_x", magnitude: { value: 1, unit: "kN" }, dimension: "force", provenance: "invented" }];
    mount(model);
    chooseVirtualTarget("load-manager-create-primitive-load-case", "load:hot");
    expect(byId("load-manager-create-primitive-id")).toHaveValue("load:hot-F301");
  });

  it("re-proposes the draft id when a model update takes it, as the load-case draft does", async () => {
    const { model, rerender } = mount();
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
    expect(byId("load-manager-selected-primitive")).toHaveTextContent("load:existing");
    expect(byId("load-manager-selected-primitive")).toHaveTextContent("load:dead; path=primitive_loads.0.magnitude.value");
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
    expect(byId("load-manager-primitive-load:sparse")).toHaveTextContent("concentrated_force; TBD TBD");
    expect(byId("load-manager-primitive-load:sparse")).toHaveTextContent("load:sparse; target:TBD; TBD; dimension=TBD");
    expect(byId("load-manager-edit-preview")).toHaveTextContent("current=TBD TBD; no changed magnitude queued");
  });

  it("falls back to the empty state when a model update removes the selected primitive", () => {
    const { model, rerender } = mount();
    expect(byId("load-manager-selected-primitive")).toBeInTheDocument();
    const next = structuredClone(model);
    next.load_cases[0].primitive_loads = [];
    rerender(next);
    expect(screen.queryByTestId("load-manager-selected-primitive")).not.toBeInTheDocument();
    expect(byId("load-case-manager-empty")).toBeInTheDocument();
  });
});

describe("LoadCaseManagerPanel combination creation", () => {
  it("queues the default mechanics combination and refuses a non-finite factor or existing id", () => {
    const { queue } = mount();
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("combination:C-300");
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("basis=mechanics; term=load:dead x 1");
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
    type("load-manager-create-combination-basis", "result_state_subtraction");
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("minuend=load:dead; subtrahend=load:hot");
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
    type("load-manager-create-combination-basis", "result_state_subtraction");
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("minuend=load:dead; subtrahend=TBD");
    expect(byId("queue-create-combination-intent")).toBeDisabled();
  });

  it("queues a range envelope with its operands and mode", () => {
    const { queue } = mount();
    type("load-manager-create-combination-basis", "range_envelope");
    type("load-manager-create-combination-mode", "min");
    expect(byId("load-manager-create-combination-heading")).toHaveTextContent("mode=min; operands=load:dead");
    fireEvent.click(byId("queue-create-combination-intent"));
    expect(JSON.parse(lastIntent(queue).change.after)).toEqual({
      id: "combination:C-300", label: "User mechanics combination", basis: "range_envelope", provenance: "user_entered_local_preview",
      operand_ids: ["load:dead"], mode: "min", terms: []
    });
  });

  it("re-proposes the draft id when a model update takes it", () => {
    const { model, rerender } = mount();
    const next = structuredClone(model);
    next.combinations!.push({ id: "combination:C-300", label: "Applied", basis: "mechanics", terms: [], provenance: "invented" });
    rerender(next);
    expect(byId("load-manager-create-combination-id")).toHaveValue("combination:C-301");
    expect(byId("queue-create-combination-intent")).toBeEnabled();
  });
});

describe("LoadCaseManagerPanel combination editing", () => {
  it("keeps a stored non-closed basis visible and queues a basis change", () => {
    const { queue } = mount(inventedModel(), { type: "combination", id: "combo:legacy" });
    expect(byId("load-manager-selected-combination")).toHaveTextContent("combo:legacy");
    expect(byId("load-manager-selected-combination")).toHaveTextContent("field=basis; current=custom_basis; path=basis");
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

  it("queues a whole-combination delete with the engine before-display", () => {
    const { queue } = mount(inventedModel(), { type: "combination", id: "combo:legacy" });
    fireEvent.click(byId("queue-delete-combination-intent"));
    expect(lastIntent(queue).change).toMatchObject({ change_kind: "delete_combination", field_label: "Legacy combination", before: "combo:legacy; Legacy; basis=custom_basis; terms=none", after: "not_present" });
    cleanup();
    const second = mount(inventedModel(), { type: "combination", id: "combo:ops" });
    type("load-manager-combination-entity-delete-rationale", " ");
    expect(byId("queue-delete-combination-intent")).toBeDisabled();
    expect(byId("load-manager-combination-entity-delete-preview")).toHaveTextContent("select a combination and provide rationale to queue deletion");
    type("load-manager-combination-entity-delete-rationale", "invented");
    fireEvent.click(byId("queue-delete-combination-intent"));
    expect(lastIntent(second.queue).change.before).toBe("combo:ops; Operating; basis=mechanics; terms=load:dead x 1");
  });

  it("edits and deletes an indexed combination term", () => {
    const { queue } = mount();
    expect(byId("load-manager-selected-combination-term")).toHaveTextContent("combo:ops");
    expect(byId("load-manager-selected-combination-term")).toHaveTextContent("term=load:dead; path=terms.0.factor");
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
    expect(lastIntent(queue)).toMatchObject({
      operation_id: "op:load-manager-combo:ops-term-0-delete",
      operation_kind: "delete",
      change: { change_kind: "delete_combination_term", field_path: "terms.0", before: "load:dead x 1", after: "not_present" }
    });
  });

  it("refuses a non-numeric existing-term factor, as the create forms do", () => {
    const { queue } = mount();
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

  it("drafts a new term against a case the combination does not yet use", () => {
    const { queue } = mount();
    expect(byId("load-manager-create-combination-term-heading")).toHaveTextContent("combo:ops");
    expect(byId("load-manager-create-combination-term-heading")).toHaveTextContent("new term; load=load:hot; factor=1");
    fireEvent.click(byId("queue-create-combination-term-intent"));
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:load-manager-combo:ops-term-1-create",
      target: { object_type: "Combination", ref: "combo:ops" },
      change: { change_kind: "create_combination_term", field_path: "terms", before: "not_present", after: JSON.stringify({ load_case: "load:hot", factor: 1 }) }
    });
    chooseVirtualTarget("load-manager-create-combination-term-combination", "combo:legacy");
    expect(byId("load-manager-create-combination-term-heading")).toHaveTextContent("new term; load=load:dead; factor=1");
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
    expect(byId("load-manager-create-combination-term-heading")).toHaveTextContent("combo:range");
    expect(byId("load-manager-create-combination-term-heading")).toHaveTextContent("load=load:dead");
  });
});
