import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference, loadPreviewModel } from "../../services/previewService";
import { numericalHealthView, readNumericalHealth, type ResultOverlayStanding } from "../results/numericalHealth";
import { numericalResultStanding } from "../results/resultStanding";
import type { EntityRef, MechanicsResult, PreviewModel } from "../../types";
import { PipeViewport } from "./PipeViewport";

// The viewport's result overlays (OD-5): the toggles, the legend and its standing lines, over the
// bundled invented reference with the session's own standing path. jsdom has no WebGL, so the
// drawn colours and arrows are covered by the unit tests of their tables and meshes and by
// `e2e/result-overlays.spec.ts`.

function sessionStanding(model: PreviewModel, result: MechanicsResult): ResultOverlayStanding {
  const standing = numericalResultStanding(result, model);
  return {
    eligible: standing.eligible,
    health: numericalHealthView(readNumericalHealth(result, standing.findings), { eligible: standing.eligible, verdictHold: false })
  };
}

function mount(model: PreviewModel, result: MechanicsResult | null, standing: ResultOverlayStanding | null, selection: EntityRef = { type: "project", id: model.project.id }) {
  return render(
    <DisplayUnitsProvider>
      <PipeViewport model={model} result={result} resultStanding={standing} selection={selection} onSelect={vi.fn()} />
    </DisplayUnitsProvider>
  );
}

describe("PipeViewport result overlays", () => {
  it("offers no overlay toggle without a result", async () => {
    const model = await loadPreviewModel();
    mount(model, null, null);
    expect(screen.queryByTestId("toggle-viewport-result-colour")).toBeNull();
    expect(screen.queryByTestId("toggle-viewport-reactions")).toBeNull();
    expect(screen.queryByTestId("viewport-result-legend")).toBeNull();
  });

  it("is off by default, and shows the colour legend with quantity, unit, case, ticks, range and no-value count when turned on", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const standing = sessionStanding(model, source);
    expect(standing.eligible).toBe(true);
    mount(model, source, standing);
    const toggle = screen.getByTestId("toggle-viewport-result-colour");
    expect(toggle).toHaveAttribute("aria-pressed", "false");
    expect(screen.queryByTestId("viewport-result-legend")).toBeNull();

    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-pressed", "true");
    const legend = screen.getByTestId("viewport-result-colour-legend");
    expect(legend).toHaveAttribute("data-state", "available");
    expect(screen.getByTestId("viewport-result-colour-title")).toHaveTextContent("Displacement magnitude [mm] ·");
    expect(screen.getByTestId("viewport-result-colour-title")).toHaveTextContent("(load:L-100)");
    for (let index = 0; index < 5; index += 1) expect(screen.getByTestId(`viewport-result-colour-tick-${index}`)).toHaveTextContent("mm");
    expect(screen.getByTestId("viewport-result-colour-tick-0")).toHaveTextContent(/^0\s?mm$/u);
    expect(screen.getByTestId("viewport-result-colour-range")).toHaveTextContent(/min .* mm · max .* mm/u);
    expect(screen.getByTestId("viewport-result-colour-no-value")).toHaveTextContent(`No value: 0 of ${model.pipe_segments.length} members`);
    expect(screen.getByTestId("viewport-overlay-standing")).toHaveTextContent("Numerical health: checks passed");

    fireEvent.change(screen.getByTestId("viewport-overlay-quantity"), { target: { value: "normal-stress-maximum" } });
    expect(screen.getByTestId("viewport-result-colour-title")).toHaveTextContent("Normal-stress maximum [Pa]");
    fireEvent.change(screen.getByTestId("viewport-overlay-case"), { target: { value: "load_case:load:L-200" } });
    expect(screen.getByTestId("viewport-result-colour-title")).toHaveTextContent("(load:L-200)");

    fireEvent.click(toggle);
    expect(screen.queryByTestId("viewport-result-legend")).toBeNull();
  });

  it("colours nothing the numerical standing withholds, and says so in the product's words", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const own = sessionStanding(model, source);
    mount(model, source, { eligible: false, health: own.health });
    fireEvent.click(screen.getByTestId("toggle-viewport-result-colour"));
    expect(screen.getByTestId("viewport-result-colour-legend")).toHaveAttribute("data-state", "held");
    expect(screen.getByTestId("viewport-result-colour-reason")).toHaveTextContent("No result colour on the current model");
    expect(screen.getByTestId("viewport-result-colour-no-value")).toHaveTextContent(`No value: ${model.pipe_segments.length} of ${model.pipe_segments.length} members`);
    expect(screen.queryByTestId("viewport-result-colour-scale")).toBeNull();
  });

  it("states that numerical health is unavailable when the session has none, and colours nothing", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    mount(model, source, null);
    fireEvent.click(screen.getByTestId("toggle-viewport-result-colour"));
    expect(screen.getByTestId("viewport-result-colour-legend")).toHaveAttribute("data-state", "held");
    expect(screen.getByTestId("viewport-overlay-standing")).toHaveTextContent("Numerical health is not available for this result.");
  });

  it("summarises the reaction arrows, counts the withheld attribution and reads out a selected support", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    mount(model, source, sessionStanding(model, source), { type: "support", id: "support:S-100" });
    fireEvent.click(screen.getByTestId("toggle-viewport-reactions"));
    const legend = screen.getByTestId("viewport-reaction-legend");
    expect(legend).toHaveAttribute("data-state", "available");
    expect(screen.getByTestId("viewport-reaction-summary")).toHaveTextContent(/Force arrows \d+; largest \|F\| .* N/u);
    expect(screen.getByTestId("viewport-reaction-withheld")).toHaveTextContent("Ambiguous attribution is withheld: 1 support with no arrow (support:CE-120, CONSTANT_EFFORT_NOT_CONSUMED)");
    const focus = within(legend).getByTestId("viewport-reaction-focus");
    expect(focus).toHaveTextContent("support:S-100");
    const fz = source.results.find((row) => row.id === "result:support-action:10:load:L-100:13:support:S-100:Fz")!;
    expect(fz.value).toBeLessThan(0);
    // W-15 (DEL-03-05 Design §6): C-130's scale redistributes the demo's support actions (was −116.6 N unscaled).
    expect(within(focus).getByTestId("viewport-reaction-focus-force")).toHaveTextContent(/Fz \u2212120\s?N/u);
    // Colour stays off: the reaction legend alone.
    expect(screen.queryByTestId("viewport-result-colour-legend")).toBeNull();
  });

  it("is view state only: toggling overlays changes neither the model nor the result", async () => {
    const { model, source } = await loadBundledMechanicsReference();
    const modelBefore = structuredClone(model);
    const resultBefore = structuredClone(source);
    mount(model, source, sessionStanding(model, source));
    for (const id of ["toggle-viewport-result-colour", "toggle-viewport-reactions"]) {
      fireEvent.click(screen.getByTestId(id));
      fireEvent.click(screen.getByTestId(id));
      fireEvent.click(screen.getByTestId(id));
    }
    fireEvent.change(screen.getByTestId("viewport-overlay-quantity"), { target: { value: "bending-moment-z" } });
    expect(model).toEqual(modelBefore);
    expect(source).toEqual(resultBefore);
  });
});
