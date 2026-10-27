import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { healthStanding, row } from "../../test-support/resultOverlayFixtures";
import { COMBINED, keyResult, keyRows } from "../../test-support/keyResultsFixtures";
import { SUPPORT_LOAD_WITHHELD_NO_STATE, supportLoadWithheldStateText } from "./supportLoadSummary";
import { SUPPORT_STATE_ROW_KIND, withheldSupportStateRows } from "./keyResults";
import { ResultsPanel, type ResultsViewRequest } from "./ResultsPanel";

// A6's results-table request (DEL-07-05 Design `key-results-panel.md` §2, §4): "Show contact and
// lift-off state" shows exactly the support-state rows of this run, with every other filter at its
// default, once per request, and marks each row the numerical standing withholds with the reason
// the A2 and A3 panels give. Invented data only.

/** A row whose id and metadata contain the support-state kind but whose kind is another one. */
const COLLIDING = row(`decoy:${SUPPORT_STATE_ROW_KIND}`, "displacement_magnitude", "node:A", 1, "mm", {
  metadata: { component: SUPPORT_STATE_ROW_KIND, coordinate_system: "global", location: "node", basis: "invented", sign_convention: SUPPORT_STATE_ROW_KIND }
});

function request(serial: number, overrides: Partial<ResultsViewRequest> = {}): ResultsViewRequest {
  return { serial, runId: "run:invented", kind: SUPPORT_STATE_ROW_KIND, withheld: {}, ...overrides };
}

function mount(next: ResultsViewRequest | null, rows = [...keyRows(), COLLIDING]) {
  const result = keyResult(rows);
  const onSelectResult = vi.fn();
  const view = (value: ResultsViewRequest | null) => (
    <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={onSelectResult} viewRequest={value} />
  );
  const rendered = render(view(next));
  return { result, onSelectResult, rerender: (value: ResultsViewRequest | null) => rendered.rerender(view(value)) };
}

const filter = () => screen.getByRole("searchbox", { name: /filter/iu });
const rowIds = () => [...document.querySelectorAll("tr[data-testid^='result-row-']")].map((item) => item.getAttribute("data-testid")!.slice("result-row-".length));

describe("ResultsPanel view request", () => {
  it("shows exactly the support-state rows of every case: a row that only mentions the kind is not one", () => {
    const { result, onSelectResult } = mount(request(1));
    const states = result.results.filter((item) => item.kind === SUPPORT_STATE_ROW_KIND).map((item) => item.id);
    expect(rowIds().sort()).toEqual([...states].sort());
    expect(rowIds()).not.toContain(COLLIDING.id);
    expect(screen.getByTestId("results-kind-filter")).toHaveTextContent(`Showing only rows of kind ${SUPPORT_STATE_ROW_KIND}.`);
    expect(filter()).toHaveValue("");
    expect(screen.getByTestId("results-navigator-case-position")).toHaveTextContent(/^All cases/u);
    expect(onSelectResult).not.toHaveBeenCalled();
    fireEvent.click(screen.getByTestId("results-kind-filter-clear"));
    expect(rowIds()).toContain(COLLIDING.id);
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
  });

  it("marks each withheld support-state row with its reason, and no other row", () => {
    const rows = [...keyRows(), row("state:support:S-D:comb:C", SUPPORT_STATE_ROW_KIND, "support:S-D", 1, "state_code", { basis_ref: COMBINED })];
    const standing = healthStanding({ eligible: true, states: { "load:L-1": "sensitive", "load:L-2": "checks_passed" } });
    const withheld = withheldSupportStateRows(keyResult(rows), standing);
    expect(withheld).toEqual({
      "state:support:S-D:load:L-1": supportLoadWithheldStateText("sensitive"),
      "state:support:S-D:comb:C": SUPPORT_LOAD_WITHHELD_NO_STATE
    });
    mount(request(1, { withheld }), rows);
    expect(screen.getByTestId("result-row-withheld-state:support:S-D:load:L-1")).toHaveTextContent("Withheld: numerical health");
    expect(screen.getByTestId("result-row-withheld-state:support:S-D:comb:C")).toHaveTextContent(SUPPORT_LOAD_WITHHELD_NO_STATE);
    expect(screen.queryByTestId("result-row-withheld-state:support:S-D:load:L-2")).toBeNull();
  });

  it("applies a request once: the user's own choice then stands until the next request", () => {
    const { rerender } = mount(request(1));
    fireEvent.click(screen.getByTestId("results-kind-filter-clear"));
    rerender(request(1));
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
    rerender(request(2));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
  });

  it("ignores a request for another run", () => {
    mount(request(1, { runId: "run:other" }));
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
    expect(rowIds()).toContain(COLLIDING.id);
  });
});
