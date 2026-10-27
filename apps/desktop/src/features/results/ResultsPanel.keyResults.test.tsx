import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { keyResult } from "../../test-support/keyResultsFixtures";
import { ResultsPanel, type ResultsViewRequest } from "./ResultsPanel";

// A6's results-table request (DEL-07-05 Design `key-results-panel.md` §2): "Show contact and
// lift-off state" sets the table's text filter to the support-state rows of this run, with every
// other filter at its default, once per request. Invented data only.

const STATE_KIND = "nonlinear_support_active_set_state_code";

function mount(request: ResultsViewRequest | null) {
  const result = keyResult();
  const onSelectResult = vi.fn();
  const view = (next: ResultsViewRequest | null) => (
    <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={onSelectResult} viewRequest={next} />
  );
  const rendered = render(view(request));
  return { result, onSelectResult, rerender: (next: ResultsViewRequest | null) => rendered.rerender(view(next)) };
}

const filter = () => screen.getByRole("searchbox", { name: /filter/iu });
const rowIds = () => [...document.querySelectorAll("tr[data-testid^='result-row-']")].map((row) => row.getAttribute("data-testid")!.slice("result-row-".length));

describe("ResultsPanel view request", () => {
  it("shows only the support-state rows of every case, and no other row", () => {
    const { result, onSelectResult } = mount({ serial: 1, runId: "run:invented", filterText: STATE_KIND });
    expect(filter()).toHaveValue(STATE_KIND);
    const states = result.results.filter((row) => row.kind === STATE_KIND).map((row) => row.id);
    expect(rowIds().sort()).toEqual([...states].sort());
    expect(screen.getByTestId("results-navigator-case-position")).toHaveTextContent(/^All cases/u);
    expect(onSelectResult).not.toHaveBeenCalled();
  });

  it("applies a request once: the user's own filter then stands until the next request", () => {
    const { rerender } = mount({ serial: 1, runId: "run:invented", filterText: STATE_KIND });
    fireEvent.change(filter(), { target: { value: "" } });
    rerender({ serial: 1, runId: "run:invented", filterText: STATE_KIND });
    expect(filter()).toHaveValue("");
    rerender({ serial: 2, runId: "run:invented", filterText: STATE_KIND });
    expect(filter()).toHaveValue(STATE_KIND);
  });

  it("ignores a request for another run", () => {
    mount({ serial: 1, runId: "run:other", filterText: STATE_KIND });
    expect(filter()).toHaveValue("");
  });
});
