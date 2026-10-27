import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { loadBundledMechanicsReference } from "../../services/previewService";
import { numericalHealthView, readNumericalHealth, type ResultOverlayStanding } from "../results/numericalHealth";
import { numericalResultStanding } from "../results/resultStanding";
import type { MechanicsResult, PreviewModel } from "../../types";
import { PipeViewport } from "./PipeViewport";

// The result contour's controls and legend in the viewport (A4; DEL-07-05 Design
// `result-contour.md` §4 to §7), over the bundled invented reference with the session's own
// standing path. jsdom has no WebGL, so the sleeves are covered by `resultContourMesh.test.ts`;
// here the legend and the numbers layer show what the viewport derived and wired.

function sessionStanding(model: PreviewModel, result: MechanicsResult): ResultOverlayStanding {
  const standing = numericalResultStanding(result, model);
  return {
    eligible: standing.eligible,
    health: numericalHealthView(readNumericalHealth(result, standing.findings), { eligible: standing.eligible, verdictHold: false })
  };
}

async function mountColour(standing?: (own: ResultOverlayStanding) => ResultOverlayStanding) {
  const { model, source } = await loadBundledMechanicsReference();
  const own = sessionStanding(model, source);
  const onQueueIntent = vi.fn();
  render(
    <DisplayUnitsProvider>
      <PipeViewport model={model} result={source} resultStanding={standing ? standing(own) : own} selection={{ type: "project", id: model.project.id }} onSelect={vi.fn()} onQueueIntent={onQueueIntent} />
    </DisplayUnitsProvider>
  );
  fireEvent.click(screen.getByTestId("toggle-viewport-result-colour"));
  return { model, source, onQueueIntent };
}

const legend = () => screen.getByTestId("viewport-result-colour-legend");

describe("PipeViewport result contour", () => {
  it("offers labelled, keyboard-reachable controls, with one value per member as the default", async () => {
    await mountColour();
    const resolution = screen.getByRole("combobox", { name: "Result colour resolution" });
    const display = screen.getByRole("combobox", { name: "Result overlay display" });
    const thresholdOn = screen.getByRole("checkbox", { name: "Only above" });
    const threshold = screen.getByRole("textbox", { name: "Threshold, in the legend's unit" });
    for (const control of [resolution, display, thresholdOn]) {
      control.focus();
      expect(document.activeElement).toBe(control);
    }
    expect(threshold).toBeDisabled();
    expect(resolution).toHaveValue("member");
    expect(display).toHaveValue("colour");
    expect(legend()).toHaveAttribute("data-resolution", "member");
    expect(screen.queryByTestId("viewport-result-contour-summary")).toBeNull();
    expect(screen.getByTestId("viewport-overlay-threshold-unit")).toHaveTextContent("mm");
  });

  it("along members, draws every member from its stated stations and says so", async () => {
    const { model } = await mountColour();
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    expect(legend()).toHaveAttribute("data-resolution", "station");
    expect(screen.getByTestId("viewport-result-contour-summary")).toHaveTextContent(`Along members: ${model.pipe_segments.length} members from stated stations`);
    // Displacement magnitude is unsigned: the blend of magnitudes is disclosed and no sign is inferred.
    expect(screen.getByTestId("viewport-result-contour-boundary")).toHaveTextContent(
      "a straight-line graphical blend of adjacent stated magnitudes, which does not represent a signed result or its zero crossing"
    );
    expect(screen.getByTestId("viewport-result-contour-sign-rule")).toHaveTextContent("no sign is inferred");
    fireEvent.change(screen.getByTestId("viewport-overlay-quantity"), { target: { value: "normal-stress-extreme" } });
    expect(screen.getByTestId("viewport-result-contour-summary")).toHaveTextContent(`Along members: ${model.pipe_segments.length} members from stated stations`);
    expect(screen.getByTestId("viewport-result-contour-sign-rule")).toHaveTextContent("no sign is inferred");
    // A signed quantity states its rule.
    fireEvent.change(screen.getByTestId("viewport-overlay-quantity"), { target: { value: "bending-moment-z" } });
    expect(screen.getByTestId("viewport-result-contour-sign-rule")).toHaveTextContent("This quantity is signed: between adjacent stations of opposite sign stated under one sign convention the tube has the no-value colour.");
    expect(screen.getByTestId("viewport-result-contour-boundary")).toHaveTextContent("does not represent a signed result or its zero crossing");
  });

  it("applies a threshold typed in the legend's unit, and states it as a display filter, not a limit", async () => {
    await mountColour();
    fireEvent.click(screen.getByTestId("viewport-overlay-threshold-on"));
    const input = screen.getByTestId("viewport-overlay-threshold");
    expect(input).toBeEnabled();
    fireEvent.change(input, { target: { value: "1.5" } });
    expect(legend()).toHaveAttribute("data-threshold", "1.5");
    expect(screen.getByTestId("viewport-result-threshold-legend")).toHaveTextContent(/Coloured above 1\.5\s?mm; at or below it, the plain pipe colour/u);
    expect(legend()).toHaveTextContent("It is not a limit or an allowable.");
    fireEvent.change(input, { target: { value: "abc" } });
    expect(legend()).toHaveAttribute("data-threshold", "");
    expect(screen.getByTestId("viewport-overlay-threshold-invalid")).toHaveTextContent("Not a number; no threshold is applied.");
    expect(screen.queryByTestId("viewport-result-threshold-legend")).toBeNull();
  });

  it("shows numbers in place of colours: stated values only, largest first, and fewer under a threshold", async () => {
    await mountColour();
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "numbers" } });
    expect(legend()).toHaveAttribute("data-display", "numbers");
    const layer = screen.getByTestId("viewport-result-values");
    const all = within(layer).getAllByTestId(/^viewport-result-value-\d+$/u);
    expect(all.length).toBeGreaterThan(0);
    expect(screen.getByTestId("viewport-result-numbers-summary")).toHaveTextContent(`Numbers in place of colours: ${all.length} of ${all.length} stated values selected for labelling, largest first; a selected number outside the view or overlapping another is hidden`);
    const values = all.map((label) => Number(/: (\S+) mm/u.exec(label.getAttribute("title") ?? "")![1]));
    expect(values).toEqual([...values].sort((a, b) => b - a));
    for (const label of all) expect(label).toHaveTextContent(/mm$/u);

    fireEvent.click(screen.getByTestId("viewport-overlay-threshold-on"));
    fireEvent.change(screen.getByTestId("viewport-overlay-threshold"), { target: { value: "1" } });
    const above = within(screen.getByTestId("viewport-result-values")).queryAllByTestId(/^viewport-result-value-\d+$/u);
    expect(above.length).toBe(values.filter((value) => value > 1).length);
    expect(above.length).toBeLessThan(all.length);
    expect(screen.getByTestId("viewport-result-threshold-legend")).toHaveTextContent(/^Numbers above 1\s?mm$/u);

    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "colour" } });
    expect(screen.queryByTestId("viewport-result-values")).toBeNull();
  });

  it("shows no contour, no numbers and no colour where the numerical standing withholds the result", async () => {
    await mountColour((own) => ({ eligible: false, health: own.health }));
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "numbers" } });
    expect(legend()).toHaveAttribute("data-state", "held");
    expect(screen.getByTestId("viewport-result-colour-reason")).toHaveTextContent("No result colour on the current model");
    expect(screen.queryByTestId("viewport-result-contour-summary")).toBeNull();
    expect(screen.queryByTestId("viewport-result-values")).toBeNull();
    expect(screen.queryByTestId("viewport-result-colour-scale")).toBeNull();
  });

  it("is view state only: no model or result change and nothing queued for the model or its undo", async () => {
    const { model, source, onQueueIntent } = await mountColour();
    const modelBefore = structuredClone(model);
    const resultBefore = structuredClone(source);
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "station" } });
    fireEvent.click(screen.getByTestId("viewport-overlay-threshold-on"));
    fireEvent.change(screen.getByTestId("viewport-overlay-threshold"), { target: { value: "2" } });
    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "numbers" } });
    fireEvent.change(screen.getByTestId("viewport-overlay-display"), { target: { value: "colour" } });
    fireEvent.change(screen.getByTestId("viewport-overlay-resolution"), { target: { value: "member" } });
    expect(model).toEqual(modelBefore);
    expect(source).toEqual(resultBefore);
    expect(onQueueIntent).not.toHaveBeenCalled();
  });
});
