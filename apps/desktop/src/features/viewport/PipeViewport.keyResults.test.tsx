import { act, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider } from "../display-units";
import { healthStanding } from "../../test-support/resultOverlayFixtures";
import { keyModel, keyResult, keyRun } from "../../test-support/keyResultsFixtures";
import { PipeViewport } from "./PipeViewport";
import type { ViewportViewCommand } from "./viewportSelection";

// A6's viewport commands (DEL-07-05 Design `key-results-panel.md` §2): "show-result-colour" sets
// exactly what the Result colour toggle and the legend's quantity and case selectors set, and
// "show-deflected-shape" what the deformation popover's case selector sets. Neither queues an
// operation. jsdom has no WebGL; the legend and the popover show the state. Invented data only.

function mount() {
  const model = keyModel();
  const result = keyResult();
  const command = { current: null as ((value: ViewportViewCommand) => void) | null };
  const onQueueIntent = vi.fn();
  const before = JSON.stringify([model, result]);
  render(
    <DisplayUnitsProvider>
      <PipeViewport
        model={model}
        result={result}
        resultStanding={healthStanding({ eligible: true })}
        ruleCheckRun={keyRun()}
        selection={{ type: "project", id: "project:invented" }}
        onSelect={vi.fn()}
        onQueueIntent={onQueueIntent}
        viewCommandRef={command}
      />
    </DisplayUnitsProvider>
  );
  return { command, onQueueIntent, unchanged: () => expect(JSON.stringify([model, result])).toBe(before) };
}

describe("PipeViewport key-results commands", () => {
  it("shows the rule ratio of the chosen category and case, as the toggle and the legend's selectors would", () => {
    const { command, onQueueIntent, unchanged } = mount();
    expect(screen.getByTestId("toggle-viewport-result-colour")).toHaveAttribute("aria-pressed", "false");
    expect(screen.queryByTestId("viewport-result-colour-legend")).toBeNull();
    act(() => command.current!({ type: "show-result-colour", quantityId: "rule-ratio:rule:cat_a", caseKey: "load_case:load:L-2" }));
    expect(screen.getByTestId("toggle-viewport-result-colour")).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByTestId("viewport-overlay-quantity")).toHaveValue("rule-ratio:rule:cat_a");
    expect(screen.getByTestId("viewport-overlay-case")).toHaveValue("load_case:load:L-2");
    // The L-2 outcome of the invented run is incomplete: it colours nothing, and the legend counts it.
    expect(screen.getByTestId("viewport-result-colour-legend")).toHaveAttribute("data-state", "unavailable");
    act(() => command.current!({ type: "show-result-colour", quantityId: "rule-ratio:rule:cat_a", caseKey: "load_case:load:L-1" }));
    expect(screen.getByTestId("viewport-overlay-case")).toHaveValue("load_case:load:L-1");
    expect(screen.getByTestId("viewport-result-colour-legend")).toHaveAttribute("data-state", "available");
    expect(screen.getByTestId("viewport-deflection-case")).toHaveValue("");
    expect(onQueueIntent).not.toHaveBeenCalled();
    unchanged();
  });

  it("chooses the deflected shape's case, as the popover's case selector would, and nothing else", () => {
    const { command, onQueueIntent, unchanged } = mount();
    expect(screen.getByTestId("viewport-deflection-case")).toHaveValue("");
    act(() => command.current!({ type: "show-deflected-shape", caseKey: "load_case:load:L-2" }));
    expect(screen.getByTestId("viewport-deflection-case")).toHaveValue("load_case:load:L-2");
    expect(screen.getByTestId("toggle-viewport-result-colour")).toHaveAttribute("aria-pressed", "false");
    expect(onQueueIntent).not.toHaveBeenCalled();
    unchanged();
  });
});
