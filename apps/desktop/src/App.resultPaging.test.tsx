import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useState } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "./App";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { loadBundledMechanicsReference } from "./services/previewService";
import type { MechanicsResult } from "./types";

// The result page is the user's position in one run's rows. It survives every
// render of that run, including the first moments after the rows appear, and
// starts again at page 1 only when a different run is shown.
//
// A click on Next can arrive after the rows are committed but before React runs
// the panel's passive effects (a busy main thread; the dead-control audit hit
// this under load). The page reset on a run change must not be such an effect,
// or it lands after the click and silently undoes it.

const PAGE_ONE = "Showing 1 to 50 of 745 matching results; page 1 of 15";
const PAGE_TWO = "Showing 51 to 100 of 745 matching results; page 2 of 15";

function openWorkspaceSection(section: "solve"): HTMLElement {
  fireEvent.click(screen.getByTestId("menu-view"));
  fireEvent.click(screen.getByTestId(`menu-item-view.section.${section}`));
  return screen.getByTestId(`workspace-section-${section}`);
}

/** Clicks Next in the microtask after the commit that first shows it enabled:
 * after the DOM is written and before any passive effect of that commit runs. */
function clickNextWhenItAppears(): { clicked: () => boolean; stop: () => void } {
  let clicked = false;
  const observer = new MutationObserver(() => {
    if (clicked) return;
    const next = document.querySelector<HTMLButtonElement>("[data-testid='next-result-page']");
    if (!next || next.disabled) return;
    clicked = true;
    observer.disconnect();
    fireEvent.click(next);
  });
  observer.observe(document.body, { childList: true, subtree: true, attributes: true });
  return { clicked: () => clicked, stop: () => observer.disconnect() };
}

/** Lets every scheduled React task, passive effects included, run. */
async function settle(): Promise<void> {
  for (let round = 0; round < 5; round += 1) {
    await new Promise((resolve) => setTimeout(resolve, 20));
  }
}

function withRunId(result: MechanicsResult, runId: string): MechanicsResult {
  return { ...structuredClone(result), run_id: runId };
}

let deliver: (result: MechanicsResult | null) => void = () => undefined;
function DeliveredResults({ initial }: { initial: MechanicsResult | null }) {
  const [result, setResult] = useState(initial);
  deliver = setResult;
  return <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => undefined} />;
}

/** Delivers a result the way a resolved load does: outside act, so React
 * schedules the render and its passive effects on its own. */
function deliverOutsideAct(result: MechanicsResult | null) {
  const environment = globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean };
  const previous = environment.IS_REACT_ACT_ENVIRONMENT;
  environment.IS_REACT_ACT_ENVIRONMENT = false;
  try {
    deliver(result);
  } finally {
    environment.IS_REACT_ACT_ENVIRONMENT = previous;
  }
}

describe("result paging keeps the user's page for the run shown", () => {
  afterEach(() => {
    cleanup();
  });

  it("keeps page 2 when Next is clicked as the bundled reference's rows appear", async () => {
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    const solve = openWorkspaceSection("solve");
    const watcher = clickNextWhenItAppears();
    try {
      fireEvent.click(within(solve).getByRole("button", { name: "Inspect bundled reference" }));
      await waitFor(() => expect(screen.getByTestId("historical-run-context")).toHaveTextContent("Bundled reference — not a solve for the current model"), { timeout: 30_000 });
      await waitFor(() => expect(watcher.clicked()).toBe(true));
    } finally {
      watcher.stop();
    }
    await settle();
    const reference = screen.getByTestId("historical-run-context");
    expect(within(reference).getByTestId("result-page-summary")).toHaveTextContent(PAGE_TWO);
  }, 60_000);

  it("keeps the page across a re-delivery of the same run and resets it for a different run", async () => {
    const { source } = await loadBundledMechanicsReference();
    render(<DeliveredResults initial={null} />);

    // First delivery: Next clicked before the delivery's passive effects run.
    const watcher = clickNextWhenItAppears();
    try {
      deliverOutsideAct(withRunId(source, "run:paging-a"));
      await waitFor(() => expect(watcher.clicked()).toBe(true));
    } finally {
      watcher.stop();
    }
    await settle();
    expect(screen.getByTestId("result-page-summary")).toHaveTextContent(PAGE_TWO);

    // The same run delivered again as a new object (a reload or re-read) keeps the page.
    act(() => deliver(withRunId(source, "run:paging-a")));
    await settle();
    expect(screen.getByTestId("result-page-summary")).toHaveTextContent(PAGE_TWO);

    // A different run starts at page 1, already in the render that first shows it.
    const summaries: string[] = [];
    const observer = new MutationObserver(() => {
      const summary = document.querySelector("[data-testid='result-page-summary']")?.textContent;
      if (summary) summaries.push(summary);
    });
    observer.observe(document.body, { childList: true, subtree: true, characterData: true });
    try {
      deliverOutsideAct(withRunId(source, "run:paging-b"));
      await waitFor(() => expect(screen.getByTestId("result-page-summary")).toHaveTextContent(PAGE_ONE));
      await settle();
    } finally {
      observer.disconnect();
    }
    expect(screen.getByTestId("result-page-summary")).toHaveTextContent(PAGE_ONE);
    expect(summaries.every((summary) => summary.includes("page 1 of 15"))).toBe(true);
  });
});
