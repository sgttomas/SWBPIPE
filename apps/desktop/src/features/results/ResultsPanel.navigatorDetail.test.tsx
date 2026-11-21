// The All rows tab's navigator, in the words a person uses: the case position counts the load cases
// and combinations a person can choose. Committed result only.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { navigatorCasePosition, ResultsPanel } from "./ResultsPanel";
import type { NavigatorCase } from "./resultsNavigator";
import type { MechanicsResult } from "../../types";
import unicodeSparse from "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json";

afterEach(cleanup);

const generic = () => structuredClone(unicodeSparse) as unknown as MechanicsResult;
const panel = (result: MechanicsResult, selectedResultId: string | null) =>
  render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={selectedResultId} onSelectResult={() => {}} />);

describe("the case navigator's position", () => {
  it("counts the load cases and combinations, not the positions of its stepper", () => {
    panel(generic(), null);
    // Two load cases and one combination: not "All cases (1 of 4)".
    expect(screen.getByTestId("results-navigator-case-position").textContent).toBe("All cases: 2 load cases, 1 combination, rows with no case stated");
    const select = screen.getByTestId("results-navigator-case-select") as HTMLSelectElement;
    fireEvent.change(select, { target: { value: [...select.options].find((option) => option.value.endsWith(":combination:Σ"))!.value } });
    expect(screen.getByTestId("results-navigator-case-position").textContent).toBe("Combination combination:Σ (1 of 1)");
    fireEvent.change(select, { target: { value: [...select.options].find((option) => option.value.endsWith(":load:𝔫"))!.value } });
    expect(screen.getByTestId("results-navigator-case-position").textContent).toBe("Load case load:𝔫 (2 of 2)");
  });

  it("says each kind once, in the singular where there is one", () => {
    const item = (ref_type: string, ref_id: string): NavigatorCase => ({ key: `${ref_type}:${ref_id}`, label: ref_id, basis: { ref_type, ref_id }, envelope: false } as NavigatorCase);
    const cases = [item("load_case", "L-100"), item("load_case", "L-200"), item("combination", "C-1")];
    expect(navigatorCasePosition("All cases", null, cases)).toBe("All cases: 2 load cases, 1 combination");
    expect(navigatorCasePosition("L-200", cases[1], cases)).toBe("L-200 (load case 2 of 2)");
    expect(navigatorCasePosition("Load case L-200", cases[1], cases)).toBe("Load case L-200 (2 of 2)");
  });
});

