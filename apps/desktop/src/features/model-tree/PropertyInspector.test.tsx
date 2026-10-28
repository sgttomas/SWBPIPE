import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import type { ComponentProps } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent, EntityRef, OperationOutcome, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { entityKey } from "../workspace/selectionState";
import { PropertyInspector } from "./PropertyInspector";
import { ECCENTRIC_OFFSET_CONVENTION } from "../component-creation/eccentricOffsetConvention";

afterEach(cleanup);

/** Invented model: every entity and value is synthetic test input. */
function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "mm", force: "N", pressure: "MPa", temperature: "C", angle: "rad" };
  model.nodes = [
    { id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "invented node" },
    { id: "node:B", label: "Node B", position: { x: 1000, y: 0, z: 0 }, provenance: "invented node" }
  ];
  model.pipe_segments = [
    { id: "pipe:AB", label: "Pipe AB", from: "node:A", to: "node:B", material: "material:X", provenance: "invented pipe",
      section: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 5, unit: "mm" } } },
    { id: "pipe:shared", label: "Shared pipe", from: "node:A", to: "node:B", material: "material:X", provenance: "invented pipe",
      section_ref: "section:S-1", section: {} }
  ];
  model.supports = [{ id: "support:A", label: "Support A", node: "node:A", restraints: ["UX", "UY"], provenance: "invented support" }];
  model.materials = [{ id: "material:X", label: "Invented material", elastic_modulus: { value: 200000, unit: "MPa" }, provenance: "invented material" }];
  model.sections = [{ id: "section:S-1", name: "Shared", section_type: "pipe", properties: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 5, unit: "mm" } }, provenance: "invented section" }];
  return model;
}

type Props = ComponentProps<typeof PropertyInspector>;

function mount(selection: EntityRef, extra: Partial<Props> = {}, model = inventedModel()) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const props: Props = { model, selection, onQueueIntent: queue, ...extra };
  const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} /></DisplayUnitsProvider>);
  const rerender = (next: Partial<Props>) =>
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} {...next} /></DisplayUnitsProvider>);
  return { model, queue, rerender };
}

const byId = (testId: string) => screen.getByTestId(testId);
const type = (testId: string, value: string) => fireEvent.change(byId(testId), { target: { value } });
const lastIntent = (queue: ReturnType<typeof mount>["queue"]) => queue.mock.calls.at(-1)![0];
const catalogSettled = () => waitFor(() => expect(byId("property-unit-catalog-status")).toHaveTextContent("browser preview uses model metadata; no fallback catalog"));
const BROWSER_UNIT_STATUS = "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview";

function startTask() {
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(byId("inspector-start-task"));
}

function outcomeFor(intent: EditorOperationIntent, overrides: Partial<OperationOutcome> = {}): OperationOutcome {
  return {
    mode: "validate_only",
    operation_id: intent.operation_id,
    change_id: intent.change.change_id,
    change_kind: intent.change.change_kind,
    target_object_type: intent.target.object_type,
    target_ref: intent.target.ref,
    validation: { schema_validation: "passed", reference_validation: "passed", unit_validation: "not_required", before_state_validation: "matched", diff_preview_status: "generated", application_status: "validated" },
    diff_preview: [{ entity_ref: intent.target.ref, object_type: intent.target.object_type, field_path: intent.change.field_path, before: intent.change.before, after: intent.change.after, unit: intent.change.unit, dimension: intent.change.dimension, change_kind: intent.change.change_kind }],
    diagnostics: [],
    ...overrides
  } as unknown as OperationOutcome;
}

describe("PropertyInspector views", () => {
  it("edits a valve's centre of gravity as an explicit vector in global X/Y/Z from its node (W-15)", async () => {
    const model = inventedModel();
    model.components = [{ id: "component:V", label: "Valve", kind: "valve", node: "node:B", provenance: "invented valve",
      geometry: { rigid_pipe_ref: "pipe:AB", weight: { value: 420, unit: "N" }, center_of_gravity: { x: 120, y: 20, z: 0, unit: "mm" } } }];
    const { queue } = mount({ type: "component", id: "component:V" }, { onApplyIntent: vi.fn(async () => true) }, model);
    await catalogSettled();
    startTask();
    changeFormControl(byId("editor-intent-field"), { target: { value: "geometry.center_of_gravity" } });
    expect(byId("inspector-text-value")).toHaveTextContent("120, 20, 0 mm");
    expect(byId("inspector-vector-convention")).toHaveTextContent("global X/Y/Z from the component's node");
    expect(byId("editor-intent-value")).toHaveValue("120, 20, 0");
    expect(byId("queue-editor-intent")).toBeDisabled();
    type("editor-intent-value", "-150, 35.5, -10");
    expect(byId("apply-editor-intent-inline")).toBeEnabled();
    fireEvent.click(byId("queue-editor-intent"));
    expect(lastIntent(queue).change).toMatchObject({
      field_path: "geometry.center_of_gravity", before: "120, 20, 0 mm",
      after: JSON.stringify({ x: -150, y: 35.5, z: -10, unit: "mm" }), unit: "mm", dimension: "length"
    });
    // Fewer than three components stay as entered; the operation service refuses them.
    type("editor-intent-value", "1, 2");
    expect(byId("apply-editor-intent-inline")).toBeDisabled();
    // The scale's source and the consumption mode are editable beside the weight.
    for (const [path, label] of [["modifiers.source_reference", "Stiffness scale source"], ["mechanics_interface.solver_consumption", "Solver consumption"], ["geometry.weight.value", "Weight"]]) {
      changeFormControl(byId("editor-intent-field"), { target: { value: path } });
      expect(screen.getByRole("heading", { name: `Edit ${label.toLowerCase()}` })).toBeInTheDocument();
    }
    type("editor-intent-value", "mechanics_geometry_only");
    changeFormControl(byId("editor-intent-field"), { target: { value: "mechanics_interface.solver_consumption" } });
    type("editor-intent-value", "mechanics_geometry_only");
    fireEvent.click(byId("queue-editor-intent"));
    expect(lastIntent(queue).change).toMatchObject({ field_path: "mechanics_interface.solver_consumption", before: "TBD", after: "mechanics_geometry_only" });
  });

  it("edits a reducer's eccentric offset as a vector under its own convention, and offers it on reducers only (W-15 slice c)", async () => {
    const model = inventedModel();
    model.components = [{ id: "component:R", label: "Reducer", kind: "reducer", node: "node:A", provenance: "invented reducer",
      geometry: { rigid_pipe_ref: "pipe:AB", eccentric_offset: { x: 0, y: -50, z: 0, unit: "mm" } } }];
    const { queue } = mount({ type: "component", id: "component:R" }, {}, model);
    await catalogSettled();
    startTask();
    changeFormControl(byId("editor-intent-field"), { target: { value: "geometry.eccentric_offset" } });
    expect(screen.getByRole("heading", { name: "Edit eccentric offset" })).toBeInTheDocument();
    expect(byId("inspector-text-value")).toHaveTextContent("0, -50, 0 mm");
    expect(byId("inspector-vector-convention")).toHaveTextContent(ECCENTRIC_OFFSET_CONVENTION);
    type("editor-intent-value", "0, -47, 0");
    fireEvent.click(byId("queue-editor-intent"));
    expect(lastIntent(queue).change).toMatchObject({
      field_path: "geometry.eccentric_offset", before: "0, -50, 0 mm",
      after: JSON.stringify({ x: 0, y: -47, z: 0, unit: "mm" }), unit: "mm", dimension: "length"
    });
    cleanup();
    const valveModel = inventedModel();
    valveModel.components = [{ id: "component:V", label: "Valve", kind: "valve", node: "node:A", provenance: "invented valve", geometry: { rigid_pipe_ref: "pipe:AB" } }];
    mount({ type: "component", id: "component:V" }, {}, valveModel);
    await catalogSettled();
    startTask();
    changeFormControl(byId("editor-intent-field"), { target: { value: "geometry.center_of_gravity" } });
    expect(screen.getByRole("heading", { name: "Edit center of gravity" })).toBeInTheDocument();
    expect(() => changeFormControl(byId("editor-intent-field"), { target: { value: "geometry.eccentric_offset" } }))
      .toThrow(/no option for geometry.eccentric_offset/);
  });

  it("authors an absent centre of gravity from three entered components and a unit", async () => {
    const model = inventedModel();
    model.components = [{ id: "component:V", label: "Valve", kind: "valve", node: "node:B", provenance: "invented valve", geometry: { rigid_pipe_ref: "pipe:AB" } }];
    const { queue } = mount({ type: "component", id: "component:V" }, {}, model);
    await catalogSettled();
    startTask();
    changeFormControl(byId("editor-intent-field"), { target: { value: "geometry.center_of_gravity" } });
    expect(byId("editor-intent-value")).toHaveValue("");
    expect(byId("queue-editor-intent")).toBeDisabled();
    type("editor-intent-value", "0 0 250");
    fireEvent.click(byId("queue-editor-intent"));
    expect(lastIntent(queue).change).toMatchObject({ before: "TBD", after: JSON.stringify({ x: 0, y: 0, z: 250, unit: "mm" }), unit: "mm" });
  });

  it("names the typed target and switches between Properties and Task", () => {
    mount({ type: "node", id: "node:A" });
    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("Node A — node: node:A");
    expect(screen.getByRole("tab", { name: "Properties" })).toHaveAttribute("aria-selected", "true");
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    expect(screen.getByRole("tab", { name: "Task" })).toHaveAttribute("aria-selected", "true");
    expect(byId("inspector-task-empty")).toHaveTextContent("Current selection: node: node:A");
    expect(screen.queryByTestId("task-action-footer")).not.toBeInTheDocument();
    fireEvent.click(byId("inspector-start-task"));
    expect(byId("inspector-frozen-task-target")).toHaveTextContent("Draft target: node: node:A");
    expect(screen.getByRole("heading", { name: "Edit name" })).toBeInTheDocument();
    expect(byId("inspector-text-value")).toHaveTextContent("Node A");
  });

  it("Cancel discards the draft and returns to the empty Task view", () => {
    const { queue } = mount({ type: "node", id: "node:A" });
    startTask();
    type("editor-intent-value", "Renamed node");
    fireEvent.click(byId("cancel-editor-intent"));
    expect(byId("inspector-task-empty")).toBeInTheDocument();
    fireEvent.click(byId("inspector-start-task"));
    expect(byId("editor-intent-value")).toHaveValue("Node A");
    expect(byId("queue-editor-intent")).toBeDisabled();
    expect(queue).not.toHaveBeenCalled();
  });

  it("shows aggregate read-only inspection for a multi-selection", () => {
    const first = { type: "node", id: "node:A" } as const;
    const refs: EntityRef[] = [first, { type: "node", id: "node:B" }, { type: "pipe", id: "pipe:AB" }];
    mount(first, { selectionState: { orderedKeys: refs.map(entityKey), primaryKey: entityKey(first), rangeAnchorKey: null, focusKey: null, preparationEpoch: 0 } });
    expect(screen.getByRole("heading", { level: 2 })).toHaveTextContent("3 selected items");
    const aggregate = byId("aggregate-property-inspector");
    expect(within(aggregate).getByRole("button", { name: "Start property task" })).toBeDisabled();
    expect(aggregate).toHaveTextContent("node2");
    expect(aggregate).toHaveTextContent("pipe1");
    expect(aggregate).toHaveTextContent("Primarynode: node:A");
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    expect(byId("inspector-start-task")).toBeDisabled();
    expect(byId("inspector-start-task")).toHaveAttribute("aria-describedby", "multi-selection-task-reason");
    expect(screen.getByText(/Reduce the selection to one typed entity before starting a task/)).toBeInTheDocument();
  });
});

describe("PropertyInspector task intents", () => {
  it("queues a deep-frozen copy of the displayed intent", () => {
    const { queue } = mount({ type: "node", id: "node:A" });
    startTask();
    type("editor-intent-value", "  Renamed node  ");
    type("editor-intent-rationale", "  ");
    fireEvent.click(byId("queue-editor-intent"));
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:editor-intent-node-node:A-label",
      target: { object_type: "Node", ref: "node:A" },
      change: { change_id: "change:node-node:A-label", change_kind: "set_field", field_path: "label", before: "Node A", after: "Renamed node", unit: "none", dimension: "dimensionless", source_note: "node label only" },
      validation: { unit_validation: "not_required_dimensionless" },
      rationale: expect.stringMatching(/^preview edit intent for project:blank-local-/)
    });
    expect(Object.isFrozen(intent)).toBe(true);
    expect(Object.isFrozen(intent.change)).toBe(true);
    expect(Object.isFrozen(intent.professional_boundary)).toBe(true);
  });

  it("captures an entered unit for a free-text mass unit and gates Apply on a finite value", async () => {
    const { queue } = mount({ type: "pipe", id: "pipe:AB" }, { onApplyIntent: vi.fn(async () => true), onValidateIntent: vi.fn() });
    await catalogSettled();
    startTask();
    changeFormControl(byId("editor-intent-field"), { target: { value: "section.material_density.value" } });
    expect(byId("editor-intent-unit").tagName).toBe("INPUT");
    expect(byId("editor-intent-unit")).toHaveValue("");
    // A unit alone is a change: Add and Review open, Apply stays closed for the TBD value.
    type("editor-intent-unit", "kg/m3");
    expect(byId("queue-editor-intent")).toBeEnabled();
    expect(byId("validate-editor-intent-inline")).toBeEnabled();
    expect(byId("apply-editor-intent-inline")).toBeDisabled();
    type("editor-intent-value", "7850");
    expect(byId("apply-editor-intent-inline")).toBeEnabled();
    fireEvent.click(byId("queue-editor-intent"));
    expect(lastIntent(queue).change).toMatchObject({
      field_path: "section.material_density.value", before: "TBD", after: JSON.stringify({ value: 7850, unit: "kg/m3" }), unit: "kg/m3", dimension: "density",
      source_note: "unit metadata required; entered unit captured explicitly"
    });
    expect(lastIntent(queue).validation.unit_validation).toBe(BROWSER_UNIT_STATUS);
    type("editor-intent-value", "seven");
    fireEvent.click(byId("queue-editor-intent"));
    expect(lastIntent(queue).change.after).toBe(JSON.stringify({ value: "seven", unit: "kg/m3" }));
    expect(byId("apply-editor-intent-inline")).toBeDisabled();
  });

  it("offers only the stored unit for a quantity when no catalog is available", async () => {
    mount({ type: "material", id: "material:X" });
    await catalogSettled();
    startTask();
    changeFormControl(byId("editor-intent-field"), { target: { value: "elastic_modulus.value" } });
    expect(screen.getByRole("heading", { name: "Edit elastic modulus" })).toBeInTheDocument();
    expect(screen.getByText("New elastic modulus (MPa)")).toBeInTheDocument();
    expect(byId("editor-intent-unit")).toHaveAttribute("role", "combobox");
    fireEvent.click(byId("editor-intent-unit"));
    const list = document.getElementById(byId("editor-intent-unit").getAttribute("aria-controls")!)!;
    expect(Array.from(list.querySelectorAll('[role="option"]')).map((option) => (option as HTMLElement).dataset.value)).toEqual(["MPa"]);
  });

  it("forwards the displayed intent to Validate and keeps the draft when Apply declines or fails", async () => {
    const validate = vi.fn();
    const apply = vi.fn<(intent: EditorOperationIntent) => Promise<boolean>>().mockResolvedValueOnce(false).mockRejectedValueOnce(new Error("invented failure"));
    const { queue } = mount({ type: "node", id: "node:A" }, { onValidateIntent: validate, onApplyIntent: apply });
    startTask();
    type("editor-intent-value", "Renamed node");
    fireEvent.click(byId("validate-editor-intent-inline"));
    expect(validate).toHaveBeenCalledOnce();
    expect(validate.mock.calls[0][0]).toMatchObject({ change: { before: "Node A", after: "Renamed node" } });
    for (let attempt = 0; attempt < 2; attempt += 1) {
      await act(async () => { fireEvent.click(byId("apply-editor-intent-inline")); });
      expect(byId("inspector-frozen-task-target")).toHaveTextContent("node: node:A");
      expect(byId("editor-intent-value")).toHaveValue("Renamed node");
    }
    expect(apply).toHaveBeenCalledTimes(2);
    expect(apply.mock.calls[1][0]).toEqual(validate.mock.calls[0][0]);
    expect(queue).not.toHaveBeenCalled();
  });

  it("disables Review without a validate handler and Apply without an apply handler", () => {
    mount({ type: "node", id: "node:A" });
    startTask();
    type("editor-intent-value", "Renamed node");
    expect(byId("queue-editor-intent")).toBeEnabled();
    expect(byId("validate-editor-intent-inline")).toBeDisabled();
    expect(byId("apply-editor-intent-inline")).toBeDisabled();
  });

  it("lists queued intents in the operation details", () => {
    const { rerender } = mount({ type: "node", id: "node:A" });
    startTask();
    expect(byId("editor-intent-queue-empty")).toHaveTextContent("No changes queued.");
    const queued = { ...outcomeIntent(), queue_id: "queue:invented-1" } as EditorOperationIntent;
    rerender({ queuedIntents: [queued] });
    expect(byId("queued-queue:invented-1")).toHaveTextContent("queue:invented-1");
    expect(byId("queued-queue:invented-1")).toHaveTextContent("label: Node A to Queued label; status: not_applied; no accepted model change");
  });
});

/** A label edit intent shaped exactly as the inspector builds it for node:A. */
function outcomeIntent(after = "Queued label"): EditorOperationIntent {
  return {
    operation_id: "op:editor-intent-node-node:A-label",
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    target: { object_type: "Node", ref: "node:A" },
    change: { change_id: "change:node-node:A-label", change_kind: "set_field", field_label: "Label", field_path: "label", before: "Node A", after, unit: "none", dimension: "dimensionless", source_note: "node label only" },
    validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_required_dimensionless", diff_preview_status: "not_generated", application_status: "not_applied" },
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
    rationale: "user_entered_preview_change"
  } as EditorOperationIntent;
}

describe("PropertyInspector inline validation outcome", () => {
  function editedTask(outcome?: OperationOutcome) {
    const view = mount({ type: "node", id: "node:A" });
    startTask();
    type("editor-intent-value", "Renamed node");
    if (outcome) view.rerender({ operationOutcomes: { [outcome.operation_id]: outcome } });
    return view;
  }

  it("shows a matching outcome's status, diff and validate-only boundary", () => {
    editedTask(outcomeFor(outcomeIntent("Renamed node")));
    expect(byId("editor-intent-inline-validation-status")).toHaveTextContent("validate_only — application status: validated; schema: passed; units: not_required; prior state: matched");
    expect(byId("editor-intent-inline-validation-diff")).toHaveTextContent("diff: node:A label Node A to Renamed node [none]");
    expect(byId("editor-intent-inline-validation-boundary")).toHaveTextContent("op:editor-intent-node-node:A-label; validate-only; no accepted model mutation.");
  });

  it("ignores an outcome for an earlier value, another target type or another change kind", () => {
    const stale = outcomeFor(outcomeIntent("Older value"));
    for (const outcome of [stale, outcomeFor(outcomeIntent("Renamed node"), { target_object_type: "Element" }), outcomeFor(outcomeIntent("Renamed node"), { change_kind: "update_load" })]) {
      editedTask(outcome);
      expect(screen.queryByTestId("editor-intent-inline-validation")).not.toBeInTheDocument();
      expect(byId("editor-intent-inline-validation-empty")).toHaveTextContent("Validate to check this change before applying it.");
      cleanup();
    }
  });

  it("shows a blocked outcome without diff rows and its diagnostics, but not an unblocked empty diff", () => {
    const blocked = outcomeFor(outcomeIntent("Renamed node"), {
      diff_preview: [],
      validation: { schema_validation: "passed", reference_validation: "failed", unit_validation: "not_required", before_state_validation: "matched", diff_preview_status: "not_generated", application_status: "blocked" },
      diagnostics: [{ id: "diag:1", code: "OP-INVENTED-BLOCK", severity: "blocking", message: "invented refusal", remediation: "", affected_refs: [], source: "test" }]
    });
    editedTask(blocked);
    expect(byId("editor-intent-inline-validation-status")).toHaveTextContent("application status: blocked");
    expect(byId("editor-intent-inline-validation-diagnostic-OP-INVENTED-BLOCK")).toHaveTextContent("blocking: OP-INVENTED-BLOCK - invented refusal");
    expect(screen.queryByTestId("editor-intent-inline-validation-boundary")).not.toBeInTheDocument();
    cleanup();
    editedTask(outcomeFor(outcomeIntent("Renamed node"), { diff_preview: [] }));
    expect(byId("editor-intent-inline-validation-empty")).toBeInTheDocument();
  });
});

describe("PropertyInspector creation forms", () => {
  it("validates and queues a new section, then reserves its id", async () => {
    const { queue } = mount({ type: "project", id: "project" });
    await catalogSettled();
    expect(byId("create-section-id")).toHaveValue("section:S-2");
    expect(byId("create-section-name")).toHaveValue("Section S-2");
    const button = byId("queue-create-section-intent");
    expect(button).toBeDisabled();
    type("create-section-od", "100");
    type("create-section-wall", "50");
    expect(button).toBeDisabled();
    type("create-section-wall", "-1");
    expect(button).toBeDisabled();
    type("create-section-wall", "6.5");
    type("create-section-id", "section:S-1");
    expect(button).toBeDisabled();
    type("create-section-id", " section:new ");
    expect(button).toBeEnabled();
    fireEvent.click(button);
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_id: "op:create-section-section:new",
      target: { object_type: "Section", ref: "section:new" },
      change: { change_kind: "create_section", field_path: "sections", before: "not_present", unit: "mm", dimension: "length" },
      validation: { unit_validation: `length=${BROWSER_UNIT_STATUS}` }
    });
    expect(JSON.parse(intent.change.after)).toEqual({
      id: "section:new", name: "Section S-2", section_type: "pipe",
      properties: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 6.5, unit: "mm" } }, provenance: "user_entered_local_preview"
    });
    // The draft resets to the next free default id; the queued custom id does not displace it.
    expect(byId("create-section-id")).toHaveValue("section:S-2");
    expect(byId("create-section-od")).toHaveValue("");
  });

  it("reserves the default section id once it is queued", () => {
    const { queue } = mount({ type: "project", id: "project" });
    type("create-section-od", "100");
    type("create-section-wall", "5");
    fireEvent.click(byId("queue-create-section-intent"));
    expect(lastIntent(queue).target.ref).toBe("section:S-2");
    expect(byId("create-section-id")).toHaveValue("section:S-3");
  });

  it("queues a material with optional thermal expansion and validates both units", async () => {
    const { queue } = mount({ type: "project", id: "project" });
    await catalogSettled();
    expect(byId("create-material-id")).toHaveValue("material:M-1");
    const button = byId("queue-create-material-intent");
    type("create-material-elastic", "200000");
    expect(button).toBeDisabled();
    type("create-material-shear", "0");
    expect(button).toBeDisabled();
    type("create-material-shear", "80000");
    expect(button).toBeEnabled();
    fireEvent.click(button);
    expect(lastIntent(queue).validation.unit_validation).toBe(`stress=${BROWSER_UNIT_STATUS}; thermal_expansion_coefficient=not_provided`);
    expect(JSON.parse(lastIntent(queue).change.after)).not.toHaveProperty("thermal_expansion_coefficient");
    expect(byId("create-material-id")).toHaveValue("material:M-2");
    type("create-material-elastic", "200000");
    type("create-material-shear", "80000");
    type("create-material-thermal", "abc");
    expect(button).toBeDisabled();
    type("create-material-thermal", "-0.000012");
    fireEvent.click(button);
    const intent = lastIntent(queue);
    expect(intent.target.ref).toBe("material:M-2");
    expect(intent.change).toMatchObject({ change_kind: "create_material", unit: "MPa", dimension: "stress" });
    expect(intent.validation.unit_validation).toBe(`stress=${BROWSER_UNIT_STATUS}; thermal_expansion_coefficient=${BROWSER_UNIT_STATUS}`);
    expect(JSON.parse(intent.change.after)).toMatchObject({ elastic_modulus: { value: 200000, unit: "MPa" }, shear_modulus: { value: 80000, unit: "MPa" }, thermal_expansion_coefficient: { value: -0.000012, unit: "1/C" } });
  });

  it("drafts a support on the selected node and requires restraints and positive stiffness", async () => {
    const { queue } = mount({ type: "node", id: "node:B" });
    await catalogSettled();
    expect(byId("create-support-id")).toHaveValue("support:S-1");
    const button = byId("queue-create-support-intent");
    expect(button).toBeEnabled();
    for (const restraint of ["UX", "UY", "UZ"]) {
      expect(byId(`create-support-restraint-${restraint}`)).toBeChecked();
      fireEvent.click(byId(`create-support-restraint-${restraint}`));
    }
    expect(button).toBeDisabled();
    fireEvent.click(byId("create-support-restraint-RZ"));
    expect(button).toBeEnabled();
    fireEvent.click(button);
    expect(lastIntent(queue).change).toMatchObject({ change_kind: "create_support", unit: "none", dimension: "dimensionless", source_note: "explicit user-entered support node and restraint tokens" });
    expect(JSON.parse(lastIntent(queue).change.after)).toEqual({ id: "support:S-1", label: "Support S-1", node: "node:B", restraints: ["RZ"], provenance: "user_entered_local_preview" });
    expect(byId("create-support-id")).toHaveValue("support:S-2");
    type("create-support-stiffness", "-5");
    expect(button).toBeDisabled();
    type("create-support-stiffness", "1500");
    fireEvent.click(button);
    expect(lastIntent(queue).change).toMatchObject({ unit: "N/mm", dimension: "linear_stiffness" });
    expect(lastIntent(queue).validation.unit_validation).toBe(`linear_stiffness=${BROWSER_UNIT_STATUS}`);
    expect(JSON.parse(lastIntent(queue).change.after).properties).toEqual({ linear_stiffness: { value: 1500, unit: "N/mm" } });
  });
});

describe("PropertyInspector delete intents and context", () => {
  it.each([
    [{ type: "node", id: "node:A" }, "delete-node-intent-panel", "queue-delete-node-intent", { object_type: "Node", change_kind: "delete_node", field_path: "nodes", before: "Node A; x=0; y=0; z=0" }],
    [{ type: "pipe", id: "pipe:AB" }, "delete-pipe-intent-panel", "queue-delete-pipe-intent", { object_type: "Element", change_kind: "delete_pipe_run", field_path: "pipe_segments", before: "Pipe AB; node:A->node:B; material=material:X" }],
    [{ type: "support", id: "support:A" }, "delete-support-intent-panel", "queue-delete-support-intent", { object_type: "Support", change_kind: "delete_support", field_path: "supports", before: "Support A" }]
  ] as const)("offers only the matching delete for %o", (selection, panel, button, expected) => {
    const { queue } = mount(selection);
    for (const other of ["delete-node-intent-panel", "delete-pipe-intent-panel", "delete-support-intent-panel"]) {
      if (other === panel) expect(byId(other)).toBeInTheDocument();
      else expect(screen.queryByTestId(other)).not.toBeInTheDocument();
    }
    fireEvent.click(byId(button));
    const intent = lastIntent(queue);
    expect(intent).toMatchObject({
      operation_kind: "delete",
      target: { object_type: expected.object_type, ref: selection.id },
      change: { change_kind: expected.change_kind, field_path: expected.field_path, before: expected.before, after: "not_present" },
      audit_boundary: { direct_model_mutation_allowed: false }
    });
  });

  it("offers no delete for a material or the project", () => {
    for (const selection of [{ type: "material", id: "material:X" }, { type: "project", id: "project" }] as const) {
      mount(selection);
      for (const panel of ["delete-node-intent-panel", "delete-pipe-intent-panel", "delete-support-intent-panel"]) {
        expect(screen.queryByTestId(panel)).not.toBeInTheDocument();
      }
      cleanup();
    }
  });

  it("shows source rows, the unit catalog posture and missing-provenance flags", async () => {
    const model = inventedModel();
    Object.assign(model.nodes[1], { provenance: "", source_name: "Invented survey", review_status: "unreviewed" });
    mount({ type: "node", id: "node:A" }, {}, model);
    await catalogSettled();
    expect(byId("inspector-provenance-block")).toHaveTextContent("Sourceinvented node");
    expect(byId("property-unit-basis-summary")).toHaveTextContent("mm, model metadata; MPa, model metadata; N/mm, model metadata; 1/C, model metadata");
    expect(screen.queryByTestId("inspector-required-flags")).not.toBeInTheDocument();
    cleanup();
    mount({ type: "node", id: "node:B" }, {}, model);
    expect(byId("inspector-provenance-block")).toHaveTextContent("source nameInvented survey");
    expect(byId("inspector-provenance-block")).toHaveTextContent("review statusunreviewed");
    expect(byId("inspector-required-flags")).toHaveTextContent("⚑ provenance");
    cleanup();
    mount({ type: "project", id: "project" }, {}, model);
    expect(byId("inspector-provenance-block")).toHaveTextContent("No provenance fields on this selected entity.");
  });

  it("reports the pipe section basis and incomplete insulation mass input", () => {
    const model = inventedModel();
    model.pipe_segments[0].section.insulation_thickness = { value: 50, unit: "mm" };
    mount({ type: "pipe", id: "pipe:AB" }, {}, model);
    expect(byId("pipe-section-basis")).toHaveTextContent("Local pipe section: diameter and wall belong to this pipe.");
    expect(byId("mass-input-completeness")).toHaveTextContent("Insulation mass is incomplete");
    cleanup();
    mount({ type: "pipe", id: "pipe:shared" }, {}, model);
    expect(byId("pipe-section-basis")).toHaveTextContent("Shared section: section:S-1. Edit diameter and wall in Sections.");
    expect(screen.queryByTestId("mass-input-completeness")).not.toBeInTheDocument();
    // A shared-section pipe does not offer its own diameter or wall for editing.
    startTask();
    fireEvent.click(byId("editor-intent-field"));
    const list = document.getElementById(byId("editor-intent-field").getAttribute("aria-controls")!)!;
    const fields = Array.from(list.querySelectorAll('[role="option"]')).map((option) => (option as HTMLElement).dataset.value);
    expect(fields).not.toContain("section.outside_diameter.value");
    expect(fields).not.toContain("section.wall_thickness.value");
    expect(fields).toContain("section.mill_tolerance.value");
  });
});
