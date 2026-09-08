import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "./PropertyInspector";
import { ModelTree } from "./ModelTree";

afterEach(cleanup);

function inspect(model: PreviewModel, selection: EntityRef) {
  const queued: EditorOperationIntent[] = [];
  render(<DisplayUnitsProvider initialPreference="SI">
    <PropertyInspector model={model} selection={selection} onQueueIntent={(intent) => queued.push(intent)} />
  </DisplayUnitsProvider>);
  return queued;
}

function chooseField(path: string) {
  fireEvent.change(screen.getByTestId("editor-intent-field"), { target: { value: path } });
}

describe("typed property inspection", () => {
  it("keeps Queue and Validate and forwards the exact displayed force intent once to inline Apply", async () => {
    const model = await loadPreviewModel();
    const load = model.load_cases[0];
    load.id = "load:UI-A";
    load.label = "Invented Phase A force";
    load.provenance = "synthetic_ui_acceptance_input";
    load.primitive_loads = [{
      id: "load:UI-A-FY",
      category: "concentrated_force",
      target_ref: "node:UI-A-110",
      direction: "global_y",
      magnitude: { value: 350, unit: "N" },
      dimension: "force",
      provenance: "synthetic_ui_acceptance_input"
    }];
    const queue = vi.fn();
    const validate = vi.fn();
    const apply = vi.fn();
    render(<DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector model={model} selection={{ type: "load", id: load.id }} onQueueIntent={queue} onValidateIntent={validate} onApplyIntent={apply} />
    </DisplayUnitsProvider>);
    chooseField("primitive_loads.0.magnitude.value");
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "500" } });
    expect(screen.getByRole("button", { name: "Queue change" })).toBeEnabled();
    expect(screen.getByRole("button", { name: "Validate" })).toBeEnabled();
    const applyButton = screen.getByTestId("apply-editor-intent-inline");
    expect(applyButton).toBeEnabled();
    fireEvent.click(applyButton);
    expect(apply).toHaveBeenCalledTimes(1);
    const intent = apply.mock.calls[0][0] as EditorOperationIntent;
    expect(intent.target.ref).toBe("load:UI-A");
    expect(intent.change).toMatchObject({
      change_kind: "update_load",
      field_path: "primitive_loads.0.magnitude.value",
      before: "350",
      after: JSON.stringify({ value: 500, unit: "N" }),
      unit: "N",
      dimension: "force"
    });
    expect(intent.rationale).toBe("user_entered_preview_change");
    expect(queue).not.toHaveBeenCalled();
    expect(validate).not.toHaveBeenCalled();
  });

  it("disables inline Apply for unchanged, incomplete, or busy intents", async () => {
    const model = await loadPreviewModel();
    const load = model.load_cases[0];
    const props = { model, selection: { type: "load" as const, id: load.id }, onQueueIntent: vi.fn(), onApplyIntent: vi.fn() };
    const view = render(<PropertyInspector {...props} />);
    chooseField("primitive_loads.0.magnitude.value");
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "" } });
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "500" } });
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeEnabled();
    view.rerender(<PropertyInspector {...props} operationBusy />);
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
  });

  it("keeps user rule fields visible and forwards the exact selected rule value to Apply", async () => {
    const model = await loadPreviewModel();
    const component = model.components.find((item) => item.id === "component:C-110")!;
    const original = String(component.modifiers?.sif_user_value?.value);
    const apply = vi.fn();
    render(<DisplayUnitsProvider initialPreference="SI">
      <PropertyInspector
        model={model}
        selection={{ type: "component", id: component.id }}
        onQueueIntent={vi.fn()}
        onApplyIntent={apply}
      />
    </DisplayUnitsProvider>);
    expect(screen.getByLabelText("Property inspector")).toHaveTextContent("user_rule_pack_inputs_only");
    chooseField("modifiers.sif_user_value.value");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue(original);
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "1.2" } });
    fireEvent.click(screen.getByTestId("apply-editor-intent-inline"));
    expect(apply).toHaveBeenCalledTimes(1);
    expect(apply.mock.calls[0][0]).toMatchObject({
      target: { object_type: "Component", ref: component.id },
      change: {
        change_kind: "set_field",
        field_path: "modifiers.sif_user_value.value",
        before: original,
        after: "1.2",
        unit: "none",
        dimension: "dimensionless"
      }
    });
    expect(String(component.modifiers?.sif_user_value?.value)).toBe(original);
  });

  it("keeps creation forms collapsed, preserves their drafts and opens selected support configuration", async () => {
    const model = await loadPreviewModel();
    const props = { model, onQueueIntent: vi.fn() };
    const view = render(<PropertyInspector {...props} selection={{ type: "project", id: model.project.id }} />);

    for (const kind of ["section", "material", "support", "component"]) {
      expect(screen.getByTestId(`create-${kind}-intent-panel`)).not.toBeVisible();
    }
    expect(screen.getByLabelText("Support configuration")).not.toBeVisible();

    const summary = screen.getByText("New section", { selector: "summary" });
    fireEvent.click(summary);
    const name = screen.getByTestId("create-section-name");
    expect(name).toBeVisible();
    fireEvent.change(name, { target: { value: "Explicit draft section" } });
    fireEvent.click(summary);
    expect(name).not.toBeVisible();
    fireEvent.click(summary);
    expect(name).toBeVisible();
    expect(name).toHaveValue("Explicit draft section");

    model.supports[0].provenance = "";
    view.rerender(<PropertyInspector {...props} selection={{ type: "support", id: model.supports[0].id }} />);
    expect(screen.getByLabelText("Support configuration")).toBeVisible();
    expect(screen.getByTestId("inspector-required-flags")).toBeVisible();
    expect(screen.getByTestId("editor-intent-panel")).toBeVisible();
  });

  it("names the selected property and keeps rationale accessible without empty-status clutter", async () => {
    const model = await loadPreviewModel();
    inspect(model, { type: "pipe", id: model.pipe_segments[0].id });
    expect(screen.getByRole("heading", { name: "Edit name" })).toBeVisible();
    expect(screen.getByRole("textbox", { name: "New name" })).toBeVisible();
    expect(screen.queryByTestId("inspector-required-flags")).not.toBeInTheDocument();
    const rationale = screen.getByTestId("editor-intent-rationale");
    expect(rationale).not.toBeVisible();
    expect(rationale).toHaveValue("user_entered_preview_change");
    fireEvent.click(screen.getByText("Operation details", { selector: "summary" }));
    expect(rationale).toBeVisible();
    chooseField("material");
    expect(screen.getByRole("textbox", { name: "New material" })).toBeVisible();
  });

  it("keeps labels and references as text under a numeric display-unit preference", async () => {
    const model = await loadPreviewModel();
    const pipe = model.pipe_segments[0];
    pipe.label = "00123";
    const queued = inspect(model, { type: "pipe", id: pipe.id });
    expect(screen.getByTestId("inspector-text-value")).toHaveTextContent("00123");
    expect(screen.queryByTestId("inspector-dual-unit-display")).not.toBeInTheDocument();
    expect(screen.queryByText(/quantity is not a finite numeric value/)).not.toBeInTheDocument();

    chooseField("material");
    expect(screen.getByTestId("inspector-text-value")).toHaveTextContent(pipe.material);
    expect(screen.queryByTestId("inspector-dual-unit-display")).not.toBeInTheDocument();
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "material:explicit-reference" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue change" }));
    expect(queued[0].change.after).toBe("material:explicit-reference");
    expect(queued[0].change.unit).toBe("none");
    expect(queued[0].validation.unit_validation).toBe("not_required_dimensionless");
    expect(pipe.material).not.toBe("material:explicit-reference");
  });

  it("keeps enum tokens as text and dimensionless numeric slots as quantities", async () => {
    const model = await loadPreviewModel();
    inspect(model, { type: "load", id: model.load_cases[0].id });
    chooseField("kind");
    expect(screen.getByTestId("inspector-text-value")).toHaveTextContent(model.load_cases[0].kind);
    expect(screen.queryByTestId("inspector-dual-unit-display")).not.toBeInTheDocument();
    chooseField("equivalent_static.seismic.g_factor_x.value");
    expect(screen.getByTestId("inspector-dual-unit-display")).toHaveTextContent("TBD");
    expect(screen.queryByTestId("inspector-text-value")).not.toBeInTheDocument();
  });

  it("keeps invalid quantity diagnostics visible and queues entered unit-bearing values unchanged", async () => {
    const model = await loadPreviewModel();
    const node = model.nodes[0];
    node.position.x = "TBD" as unknown as number;
    const queued = inspect(model, { type: "node", id: node.id });
    chooseField("position.x");
    const current = screen.getByTestId("inspector-dual-unit-display");
    expect(within(current).getByRole("status")).toHaveTextContent("quantity is not a finite numeric value");
    expect(within(current).getByRole("status")).toBeVisible();
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "12.5" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue change" }));
    expect(JSON.parse(queued[0].change.after)).toEqual({ value: 12.5, unit: model.project.units.length });
    expect(queued[0].audit_boundary.direct_model_mutation_allowed).toBe(false);
    expect(node.position.x).toBe("TBD");
  });
});

describe("model browser", () => {
  it("retains stable identity selection and ID filtering with concise group names", async () => {
    const model = await loadPreviewModel();
    const pipe = model.pipe_segments[0];
    const onSelect = vi.fn();
    render(<ModelTree model={model} selection={{ type: "project", id: model.project.id }} onSelect={onSelect} />);
    expect(screen.getByRole("heading", { name: "Pipes" })).toBeInTheDocument();
    fireEvent.change(screen.getByRole("searchbox"), { target: { value: pipe.id } });
    const row = screen.getByTestId(`tree-row-${pipe.id}`);
    expect(row).toHaveAttribute("title", `${pipe.label} (${pipe.id})`);
    fireEvent.click(row);
    expect(onSelect).toHaveBeenCalledWith({ type: "pipe", id: pipe.id });
  });
});
