// The key-results session and the results table together (DEL-07-05 REQ-013): "Show contact and
// lift-off state" marks the rows the standing of THAT result withholds, and nothing it asked for
// reaches a later result. Every solve publishes the same run id, so two solves are told apart by
// the result object only. The Results section is remounted for each new result, as App's keyed
// section is, and the result passes through null between them, as a solve's start and an edit
// make it. Invented data only; the run id is the producer's constant one.
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { healthStanding } from "../../test-support/resultOverlayFixtures";
import { keyResult } from "../../test-support/keyResultsFixtures";
import type { SessionResult } from "../../services/nativeResult";
import type { MechanicsResult } from "../../types";
import type { ResultOverlayStanding } from "./numericalHealth";
import { supportLoadWithheldStateText } from "./supportLoadSummary";
import { SUPPORT_STATE_ROW_KIND, withheldSupportStateRows } from "./keyResults";
import { ResultsPanel } from "./ResultsPanel";
import { useKeyResultsSession } from "./useKeyResultsSession";

const saved: string[] = [];
vi.mock("../result-export/csvSave", () => ({
  saveCsvTable: vi.fn(async (input: { text: string; fileName: string }) => {
    saved.push(input.text);
    return { mode: "browser", file_name: input.fileName, byte_count: new TextEncoder().encode(input.text).length };
  })
}));
afterEach(() => { cleanup(); saved.length = 0; });

const RUN_ID = "run:preview-linear-static-001";
const L1_ROW = "state:support:S-D:load:L-1";
const L2_ROW = "state:support:S-D:load:L-2";
const SENSITIVE = supportLoadWithheldStateText("sensitive");

/** A solve of the invented model under `states`: the producer's run id, a new object each time. */
function solve(states: Record<string, "checks_passed" | "sensitive">): { result: MechanicsResult; standing: ResultOverlayStanding } {
  return { result: { ...keyResult(), run_id: RUN_ID }, standing: healthStanding({ eligible: true, states }) };
}

type Shown = { result: SessionResult | null; standing: ResultOverlayStanding | null };
const sections = new WeakMap<object, number>();
let sectionCount = 0;
/** App's Results section is keyed on a sequence that advances when the result object changes. */
const sectionKey = (result: SessionResult | null) => {
  if (result === null) return "none";
  if (!sections.has(result)) sections.set(result, ++sectionCount);
  return `results:${sections.get(result)}`;
};

function Workspace({ result, standing }: Shown) {
  const session = useKeyResultsSession({
    result, standing, historical: false, setActiveSection: () => undefined, selectEntity: () => undefined,
    viewCommandRef: { current: null }, enterModelStage: () => undefined, openRuleChecks: () => undefined
  });
  return (
    <>
      <button type="button" data-testid="show-contact" onClick={session.actions.onShowContactStatus}>Show contact and lift-off state</button>
      <ResultsPanel key={sectionKey(result)} result={result as MechanicsResult | null} knowledge={null} analysisRun={null}
        selectedResultId={null} onSelectResult={() => undefined} viewRequest={session.resultsRequest} />
    </>
  );
}

const withheldMark = (id: string) => screen.queryByTestId(`result-row-withheld-${id}`);
/** The CSV text of every result group the table would export now. */
async function exportedCsv(): Promise<string> {
  saved.length = 0;
  const buttons = [...document.querySelectorAll<HTMLButtonElement>("[data-testid^='result-group-'][data-testid$='-export-csv']")];
  expect(buttons.length).toBeGreaterThan(0);
  for (const button of buttons) {
    const before = saved.length;
    await act(async () => { fireEvent.click(button); });
    await waitFor(() => expect(saved.length).toBe(before + 1));
  }
  return saved.join("\n");
}

describe("Show contact and lift-off state across two solves with the same run id", () => {
  it("marks only the shown result's withheld rows, in the table and its CSV", async () => {
    // A withholds the L-2 row; B, the next solve under another standing, withholds the L-1 row.
    const a = solve({ "load:L-1": "checks_passed", "load:L-2": "sensitive" });
    const b = solve({ "load:L-1": "sensitive", "load:L-2": "checks_passed" });
    expect(withheldSupportStateRows(a.result, a.standing)).toEqual({ [L2_ROW]: SENSITIVE });
    expect(withheldSupportStateRows(b.result, b.standing)).toEqual({ [L1_ROW]: SENSITIVE });
    const view = render(<Workspace {...a} />);

    fireEvent.click(screen.getByTestId("show-contact"));
    expect(screen.getByTestId("results-kind-filter")).toHaveTextContent(`Showing only rows of kind ${SUPPORT_STATE_ROW_KIND}.`);
    expect(withheldMark(L2_ROW)).toHaveTextContent(SENSITIVE);
    expect(withheldMark(L1_ROW)).toBeNull();

    // The next solve: the result is cleared at its start, then B is published with the same run id.
    view.rerender(<Workspace result={null} standing={null} />);
    view.rerender(<Workspace {...b} />);
    // No row carries A's mark (the L-2 row B does not withhold), and the table is not narrowed.
    expect(withheldMark(L2_ROW)).toBeNull();
    expect(withheldMark(L1_ROW)).toBeNull();
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
    const plain = await exportedCsv();
    expect(plain).toContain("kind: any");
    expect(plain).not.toContain(SENSITIVE);

    // Asked for on B, the rows are B's, marked with B's standing.
    fireEvent.click(screen.getByTestId("show-contact"));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
    expect(withheldMark(L1_ROW)).toHaveTextContent(SENSITIVE);
    expect(withheldMark(L2_ROW)).toBeNull();
    const marked = await exportedCsv();
    expect(marked).toContain(`kind: ${SUPPORT_STATE_ROW_KIND}`);
    const markedLines = marked.split(/\r?\n/u).filter((line) => line.includes(SENSITIVE));
    expect(markedLines).toHaveLength(1);
    expect(markedLines[0]).toContain(L1_ROW);
  });
});
