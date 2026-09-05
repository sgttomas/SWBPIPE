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
