import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { EntityRef, MechanicsResult, PreviewModel } from "../../types";
import { ResultsPanel } from "../results/ResultsPanel";
import { PipeViewport } from "./PipeViewport";

// A7: the element-axis toggle (off, local, global) and its readout, and the results table's
// frame column and frame filter, which show each row in the frame it is published in and never
// re-project a value. jsdom has no WebGL; the triads' geometry is covered by elementAxes.test.ts.

function mount(model: PreviewModel, result: MechanicsResult | null, selection: EntityRef) {
  return render(
    <DisplayUnitsProvider>
      <PipeViewport model={model} result={result} selection={selection} onSelect={vi.fn()} />
    </DisplayUnitsProvider>
  );
}

describe("element axes in the viewport", () => {
  it("is off by default and switches between local and global from the keyboard", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const pipe = model.pipe_segments.find((candidate) => candidate.id === "pipe:P-130")!;
    expect(pipe.y_reference).toBeDefined();
    const modelBefore = structuredClone(model);
    const view = mount(model, source, { type: "pipe", id: pipe.id });
    const group = screen.getByRole("group", { name: "Element axes" });
    const off = within(group).getByTestId("viewport-element-axes-off");
    const local = within(group).getByTestId("viewport-element-axes-local");
    const global = within(group).getByTestId("viewport-element-axes-global");
    expect(off).toHaveAttribute("aria-pressed", "true");
    expect(screen.queryByTestId("viewport-element-axes-status")).toBeNull();

    local.focus();
    expect(document.activeElement).toBe(local);
    fireEvent.click(local);
    expect(local).toHaveAttribute("aria-pressed", "true");
    const status = screen.getByTestId("viewport-element-axes-status");
    expect(status).toHaveTextContent(`Local axes: ${model.pipe_segments.length} members`);
    expect(status).toHaveAttribute("title", expect.stringContaining("y from the member's y_reference, z = x × y"));
    // The selected member's axes as unit vectors in global X, Y, Z.
    expect(screen.getByTestId("viewport-element-axes-selected").textContent).toMatch(/pipe:P-130 x \(.+\) y \(.+\) z \(.+\)/u);

    fireEvent.click(global);
    expect(global).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByTestId("viewport-element-axes-status")).toHaveTextContent("Global axes:");
    expect(screen.getByTestId("viewport-element-axes-selected")).toHaveTextContent("x (1, 0, 0) y (0, 1, 0) z (0, 0, 1)");

    fireEvent.click(off);
    expect(screen.queryByTestId("viewport-element-axes-status")).toBeNull();
    view.unmount();
    expect(model).toEqual(modelBefore);
  });

  it("says why a member has no local frame", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const target = model.pipe_segments[0];
    const edited = { ...model, pipe_segments: model.pipe_segments.map((pipe) => pipe.id === target.id ? { ...pipe, y_reference: undefined } : pipe) };
    mount(edited, source, { type: "pipe", id: target.id });
    fireEvent.click(screen.getByTestId("viewport-element-axes-local"));
    expect(screen.getByTestId("viewport-element-axes-status")).toHaveTextContent("1 without a local frame");
    expect(screen.getByTestId("viewport-element-axes-selected")).toHaveTextContent("no y_reference entered; no default orientation is applied");
  });
});

describe("the results table states each row's frame", () => {
  it("labels every row with its published frame and filters to the local or the global rows", async () => {
    const { source } = await loadBundledMechanicsReference();
    const before = structuredClone(source);
    render(<ResultsPanel result={source} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);

    const filter = screen.getByTestId("result-frame-filter");
    const all = within(filter).getByTestId("result-frame-all");
    const local = within(filter).getByTestId("result-frame-element_local");
    const global = within(filter).getByTestId("result-frame-global");
    expect(all).toHaveAttribute("aria-pressed", "true");
    const localCount = source.results.filter((row) => row.metadata?.coordinate_system === "element_local").length;
    const globalCount = source.results.filter((row) => row.metadata?.coordinate_system === "global").length;
    expect(local).toHaveTextContent(`Element local${localCount}`);
    expect(global).toHaveTextContent(`Global${globalCount}`);

    const globalEnd = source.results.find((row) => row.kind === "element_global_end_force_v1")!;
    const localEnd = source.results.find((row) => row.kind === "element_local_axial_force" && row.metadata?.location === "end_i")!;
    fireEvent.change(screen.getByTestId("result-page-size"), { target: { value: "200" } });

    global.focus();
    fireEvent.click(global);
    expect(global).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByTestId("result-filter-summary")).toHaveTextContent(`${globalCount} of ${source.results.length}`);
    expect(screen.getByTestId(`result-row-frame-${globalEnd.id}`)).toHaveTextContent("Global");
    expect(screen.queryByTestId(`result-row-${localEnd.id}`)).toBeNull();
    // The value shown is the published one.
    expect(screen.getByTestId(`result-row-dual-${globalEnd.id}`)).toHaveTextContent(`Entered: ${globalEnd.value} ${globalEnd.unit}`);

    fireEvent.click(local);
    expect(screen.getByTestId("result-filter-summary")).toHaveTextContent(`${localCount} of ${source.results.length}`);
    expect(screen.getByTestId(`result-row-frame-${localEnd.id}`)).toHaveTextContent("Element local");
    expect(screen.queryByTestId(`result-row-${globalEnd.id}`)).toBeNull();

    fireEvent.click(all);
    expect(screen.getByTestId("result-filter-summary")).toHaveTextContent(`${source.results.length} of ${source.results.length}`);
    expect(source).toEqual(before);
  });
});
