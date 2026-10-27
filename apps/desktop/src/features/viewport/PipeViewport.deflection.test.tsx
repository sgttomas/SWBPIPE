import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { PipeViewport } from "./PipeViewport";
import { DeflectedShapeControls } from "./DeflectedShapeControls";
import { buildDeformationOverlay } from "./deformationOverlay";

// The deflected-shape controls (A5) over the bundled invented reference. jsdom has no WebGL, so
// the drawn shape is covered by `deflectedShape.test.ts` (positions, phase and the ghost's dim
// batches); this file covers the controls, their keyboard use, the on-canvas factor and the
// readout, and that none of it changes the model or the result.

function mount(model: PreviewModel, result: MechanicsResult | null) {
  return render(
    <DisplayUnitsProvider>
      <PipeViewport model={model} result={result} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} />
    </DisplayUnitsProvider>
  );
}

const byId = (id: string) => screen.getByTestId(id);

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

describe("PipeViewport deflected-shape controls", () => {
  it("offers none without a result", async () => {
    const { model } = await loadBundledMechanicsReference();
    mount(model, null);
    expect(screen.queryByTestId("viewport-deflection-controls")).toBeNull();
    expect(screen.queryByTestId("viewport-deflection-label")).toBeNull();
  });

  it("starts on the normalized governing drawing, with the factor and animation unavailable", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    mount(model, source);
    expect(byId("viewport-deformation-status").querySelector("summary")).toHaveTextContent("Deformation · normalized");
    expect(byId("viewport-deflection-label")).toHaveTextContent("Deformed · normalized, not to scale · governing case per node");
    expect(byId("viewport-deflection-case")).toHaveValue("");
    expect(byId("viewport-deflection-factor")).toBeDisabled();
    expect(byId("toggle-viewport-deflection-animation")).toBeDisabled();
    expect(byId("viewport-deflection-phase")).toBeDisabled();
    expect(byId("viewport-deflection-reason")).toHaveTextContent("choose one case to set a factor");
  });

  it("sets the factor for one case from the keyboard and states it on the canvas", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const modelBefore = structuredClone(model);
    const resultBefore = structuredClone(source);
    mount(model, source);

    const caseSelect = byId("viewport-deflection-case");
    caseSelect.focus();
    expect(document.activeElement).toBe(caseSelect);
    fireEvent.change(caseSelect, { target: { value: "load_case:load:L-100" } });

    const field = byId("viewport-deflection-factor") as HTMLInputElement;
    expect(field).toBeEnabled();
    const defaultFactor = Number(field.value);
    expect(defaultFactor).toBeGreaterThan(0);
    expect(byId("viewport-deflection-label")).toHaveTextContent(`Deformed ×${field.value} (default) · `);
    expect(byId("viewport-deflection-label")).toHaveTextContent("(load:L-100)");
    expect(byId("viewport-deformation-status").querySelector("summary")).toHaveTextContent(`Deformation · ×${field.value}`);
    expect(byId("viewport-deformation-boundary")).toHaveTextContent(`factor=${field.value}`);
    expect(byId("viewport-deformation-boundary")).toHaveTextContent("case=load_case:load:L-100");
    expect(byId("viewport-deflection-factor-default")).toBeDisabled();

    field.focus();
    fireEvent.change(field, { target: { value: "200" } });
    fireEvent.keyDown(field, { key: "Enter" });
    expect(byId("viewport-deflection-label")).toHaveTextContent("Deformed ×200 · ");
    expect(byId("viewport-deformation-boundary")).toHaveTextContent("factor=200");

    fireEvent.change(field, { target: { value: "-3" } });
    fireEvent.keyDown(field, { key: "Enter" });
    expect(screen.getByRole("alert")).toHaveTextContent("Enter a positive number.");
    expect(byId("viewport-deflection-label")).toHaveTextContent("Deformed ×200 · ");
    fireEvent.keyDown(field, { key: "Escape" });
    expect(field).toHaveValue("200");
    expect(screen.queryByRole("alert")).toBeNull();

    fireEvent.click(byId("viewport-deflection-factor-up"));
    expect(field).toHaveValue("500");
    fireEvent.click(byId("viewport-deflection-factor-down"));
    fireEvent.click(byId("viewport-deflection-factor-down"));
    expect(field).toHaveValue("100");
    fireEvent.click(byId("viewport-deflection-factor-default"));
    expect(Number(field.value)).toBe(defaultFactor);

    // The readout is the emitted maximum, whatever the factor.
    expect(byId("viewport-deformation-summary")).toHaveTextContent(/max=/u);
    fireEvent.change(caseSelect, { target: { value: "" } });
    expect(byId("viewport-deflection-label")).toHaveTextContent("normalized");
    expect(model).toEqual(modelBefore);
    expect(source).toEqual(resultBefore);
  });

  it("toggles the undeformed ghost, steps the phase and animates only while asked", async () => {
    vi.useFakeTimers();
    const { model, source } = await loadBundledMechanicsReference();
    mount(model, source);
    const ghost = byId("toggle-viewport-undeformed-ghost");
    expect(ghost).toHaveAttribute("aria-pressed", "false");
    fireEvent.click(ghost);
    expect(ghost).toHaveAttribute("aria-pressed", "true");

    fireEvent.change(byId("viewport-deflection-case"), { target: { value: "load_case:load:L-200" } });
    const phase = byId("viewport-deflection-phase");
    expect(phase).toHaveValue("24");
    const stated = Number((byId("viewport-deflection-factor") as HTMLInputElement).value);
    fireEvent.change(phase, { target: { value: "6" } });
    expect(phase).toHaveValue("6");
    expect(phase).toHaveAttribute("aria-valuetext", "25% of the deflection");
    // Review r1 (#1667): the canvas and the summary state the magnification drawn at this phase.
    expect(byId("viewport-deflection-label")).toHaveTextContent(`Deformed ×${stated / 4}, 25% of ×${stated} (default) · `);
    expect(byId("viewport-deformation-status").querySelector("summary")).toHaveTextContent(`Deformation · ×${stated / 4}`);
    expect(byId("viewport-deformation-boundary")).toHaveTextContent("phase=0.25");

    const animate = byId("toggle-viewport-deflection-animation");
    fireEvent.click(animate);
    expect(animate).toHaveAttribute("aria-pressed", "true");
    expect(animate).toHaveTextContent("Stop animation");
    const seen = new Set<string>();
    for (let step = 0; step < 24; step += 1) {
      act(() => { vi.advanceTimersByTime(100); });
      seen.add((phase as HTMLInputElement).value);
    }
    expect(seen.size).toBeGreaterThan(1);
    fireEvent.click(animate);
    expect(animate).toHaveAttribute("aria-pressed", "false");
    expect(phase).toHaveValue("24");
    const settled = (phase as HTMLInputElement).value;
    act(() => { vi.advanceTimersByTime(1000); });
    expect(phase).toHaveValue(settled);
  });

  it("follows a reduced-motion preference: the phase steps by hand and nothing animates", async () => {
    vi.stubGlobal("matchMedia", (query: string) => ({
      matches: query.includes("prefers-reduced-motion"), media: query,
      addEventListener: () => {}, removeEventListener: () => {}
    }));
    const { model, source } = await loadBundledMechanicsReference();
    mount(model, source);
    fireEvent.change(byId("viewport-deflection-case"), { target: { value: "load_case:load:L-100" } });
    expect(byId("toggle-viewport-deflection-animation")).toBeDisabled();
    expect(byId("viewport-deflection-phase")).toBeEnabled();
  });
});

describe("deflected-shape controls without a default factor (review r1, #1667)", () => {
  it("say why a zero-extent model has no default and still take a factor from the keyboard", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const deformation = buildDeformationOverlay(model, source, {
      basisKey: "load_case:load:L-100", scale: { mode: "factor", factor: null }, modelLengthUnit: "m", modelExtent: 0
    });
    const onFactorChange = vi.fn();
    render(
      <DeflectedShapeControls cases={[]} caseKey="load_case:load:L-100" onCaseChange={vi.fn()} deformation={deformation}
        onFactorChange={onFactorChange} ghost={false} onGhostChange={vi.fn()} playing={false} onPlayingChange={vi.fn()}
        phase={1} onPhaseChange={vi.fn()} reducedMotion={false} />
    );
    expect(byId("viewport-deflection-reason")).toHaveTextContent("no positive extent (for example a single node)");
    const field = byId("viewport-deflection-factor");
    expect(field).toBeEnabled();
    expect(byId("viewport-deflection-factor-up")).toBeDisabled();
    fireEvent.change(field, { target: { value: "20" } });
    fireEvent.keyDown(field, { key: "Enter" });
    expect(onFactorChange).toHaveBeenCalledWith(20);
  });
});

describe("the popover explanation at a partial phase (review r2, #1667)", () => {
  it("states the drawing at phase zero, not the stated factor", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    mount(model, source);
    fireEvent.change(byId("viewport-deflection-case"), { target: { value: "load_case:load:L-100" } });
    const field = byId("viewport-deflection-factor");
    fireEvent.change(field, { target: { value: "100" } });
    fireEvent.keyDown(field, { key: "Enter" });
    fireEvent.change(byId("viewport-deflection-phase"), { target: { value: "0" } });
    expect(byId("viewport-deflection-label")).toHaveTextContent("Deformed ×0, 0% of ×100 · ");
    expect(byId("viewport-deformation-status").querySelector("summary")).toHaveTextContent("Deformation · ×0");
    expect(byId("viewport-deflection-reason")).not.toHaveTextContent("Displacements are drawn at ×100");
    expect(byId("viewport-deflection-reason")).toHaveTextContent("Displacements are drawn at ×0 their physical length in model units, 0% of the stated ×100.");
  });
});
