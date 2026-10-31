// A1 (DEL-07-05 Design `results-navigator.md`): the Results panel's navigator. Steps over the
// table kind, the load case or combination and the support or element, by key chord and by
// button; the current row's page and scroll follow, and the session's selection follows through
// `onSelectResult` only. Fixtures are the product's own preview-physics-2 output for the
// invented unicode-ids vector, and an invented unsupported result.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { useState } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ResultsPanel } from "./ResultsPanel";
import { numericalHealthView, parseNumericalHealth, type NumericalHealthView } from "./numericalHealth";
import { semanticLabels } from "./resultSemantics";
import { setSurfaceVisibilityProbeForTests } from "../workspace/surfaceReveal";
import type { MechanicsResult } from "../../types";
import unicodeSparse from "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json";

type Row = MechanicsResult["results"][number];
const fixture = () => structuredClone(unicodeSparse) as unknown as MechanicsResult;
const SUPPORT = "support:锚";

let scrolled: Element[] = [];
beforeEach(() => {
  scrolled = [];
  // jsdom has no scrollIntoView and does no layout; record the call and model visibility.
  Element.prototype.scrollIntoView = function (this: Element) { scrolled.push(this); } as Element["scrollIntoView"];
  setSurfaceVisibilityProbeForTests((element) => element.isConnected && !element.closest("[hidden]"));
});
afterEach(() => {
  cleanup();
  setSurfaceVisibilityProbeForTests(null);
  delete (Element.prototype as { scrollIntoView?: unknown }).scrollIntoView;
});

/** The panel as a session holds it: the selected result row is the one last chosen. */
function Harness({ result, onSelect, health }: { result: MechanicsResult; onSelect: (id: string) => void; health?: NumericalHealthView | null }) {
  const [selected, setSelected] = useState<string | null>(null);
  return (
    <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={selected}
      onSelectResult={(id) => { onSelect(id); setSelected(id); }} numericalHealth={health} />
  );
}

const position = (axis: string) => screen.getByTestId(`results-navigator-${axis}-position`).textContent ?? "";
const button = (axis: string, which: "next" | "previous") => screen.getByTestId(`results-navigator-${axis}-${which}`);
const key = { kind: { next: { key: "ArrowRight", altKey: true, shiftKey: true }, previous: { key: "ArrowLeft", altKey: true, shiftKey: true } },
  case: { next: { key: "ArrowDown", altKey: true }, previous: { key: "ArrowUp", altKey: true } },
  entity: { next: { key: "ArrowRight", altKey: true }, previous: { key: "ArrowLeft", altKey: true } } } as const;
type Axis = keyof typeof key;

/** Steps `axis` by button until its position starts with `text`. */
function stepUntil(axis: Axis, text: string) {
  for (let guard = 0; guard < 40 && !position(axis).startsWith(text); guard += 1) fireEvent.click(button(axis, "next"));
  expect(position(axis).startsWith(text)).toBe(true);
}

function rowsOf(result: MechanicsResult, family: string, entity: string, caseId: string | null): Row[] {
  const labels = semanticLabels(result);
  return result.results.filter((row) => labels.family(row) === family && row.entity_ref === entity && (caseId === null || row.basis_ref?.ref_id === caseId));
}

describe("the results navigator", () => {
  it("offers each step as a labelled button with its key chord, in three labelled groups", () => {
    render(<Harness result={fixture()} onSelect={vi.fn()} />);
    const navigator = screen.getByRole("region", { name: "Results navigator" });
    for (const [axis, label] of [["kind", "Table"], ["case", "Case"], ["entity", "Support or element"]] as const) {
      expect(within(navigator).getByRole("group", { name: label })).toBe(screen.getByTestId(`results-navigator-${axis}`));
    }
    expect(screen.getByRole("button", { name: "Next table kind" })).toHaveAttribute("aria-keyshortcuts", "Alt+Shift+ArrowRight");
    expect(screen.getByRole("button", { name: "Previous table kind" })).toHaveAttribute("aria-keyshortcuts", "Alt+Shift+ArrowLeft");
    expect(screen.getByRole("button", { name: "Next load case or combination" })).toHaveAttribute("aria-keyshortcuts", "Alt+ArrowDown");
    expect(screen.getByRole("button", { name: "Previous load case or combination" })).toHaveAttribute("aria-keyshortcuts", "Alt+ArrowUp");
    expect(screen.getByRole("button", { name: "Next support or element" })).toHaveAttribute("aria-keyshortcuts", "Alt+ArrowRight");
    expect(screen.getByRole("button", { name: "Previous support or element" })).toHaveAttribute("aria-keyshortcuts", "Alt+ArrowLeft");
    expect(screen.getByTestId("results-navigator-status")).toHaveAttribute("role", "status");
    // Kind: All plus each family the result has rows of, as the family buttons. Case: All plus four.
    const familyButtons = within(screen.getByRole("group", { name: "Result family filter" })).getAllByRole("button");
    expect(position("kind")).toBe(`All kinds (1 of ${familyButtons.length})`);
    expect(position("case")).toBe("All cases (1 of 5)");
    expect(within(screen.getByTestId("results-navigator-case-select")).getAllByRole("option").map((option) => option.textContent)).toEqual([
      "All cases", "Load case load:é", "Load case load:𝔫", "Combination combination:Σ", "No case stated"
    ]);
  });

  it("reads one support's reactions case by case with one key each, the selection following", () => {
    const result = fixture();
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} />);
    stepUntil("kind", "Reaction");
    stepUntil("entity", SUPPORT);
    const all = rowsOf(result, "reaction", SUPPORT, null);
    expect(onSelect).toHaveBeenLastCalledWith(all[0].id);
    for (const caseId of ["load:é", "load:𝔫", "combination:Σ"]) {
      fireEvent.keyDown(screen.getByTestId(`result-row-${onSelect.mock.lastCall![0]}`), key.case.next);
      const expected = rowsOf(result, "reaction", SUPPORT, caseId)[0];
      expect(onSelect).toHaveBeenLastCalledWith(expected.id);
      expect(position("case")).toContain(caseId);
      expect(position("entity")).toMatch(new RegExp(`^${SUPPORT} \\(1 of \\d+\\)$`));
      // The table shows this case only, and the current row is marked selected and focused.
      expect(screen.getByTestId(`result-row-${expected.id}`)).toHaveAttribute("aria-selected", "true");
      expect(document.activeElement).toBe(screen.getByTestId(`result-row-${expected.id}`));
      // 531 rows: the capture's 459 plus W-7's 72 global end-action rows (preview_physics_capture, DEL-03-03 regeneration).
      expect(screen.getByTestId("result-filter-summary").textContent).toMatch(/^\d+ of 531 results match filter$/);
    }
  });

  it("does the same with keys as with buttons (parity)", () => {
    const run = (route: "key" | "button") => {
      const onSelect = vi.fn();
      render(<Harness result={fixture()} onSelect={onSelect} />);
      const trail: string[] = [];
      const steps: [Axis, "next" | "previous"][] = [["kind", "next"], ["entity", "next"], ["entity", "next"], ["case", "next"], ["case", "next"], ["kind", "next"], ["entity", "previous"], ["case", "previous"], ["kind", "previous"]];
      for (const [axis, which] of steps) {
        if (route === "button") fireEvent.click(button(axis, which));
        else fireEvent.keyDown(screen.getByTestId("results-panel"), key[axis][which]);
        trail.push(`${position("kind")}|${position("case")}|${position("entity")}|${onSelect.mock.lastCall?.[0] ?? "none"}`);
      }
      cleanup();
      return trail;
    };
    const byKey = run("key");
    expect(byKey).toEqual(run("button"));
    expect(new Set(byKey).size).toBe(byKey.length);
  });

  it("stops at each end, says so, and keeps the end's button focusable", () => {
    const onSelect = vi.fn();
    render(<Harness result={fixture()} onSelect={onSelect} />);
    const previous = button("kind", "previous");
    expect(previous).toHaveAttribute("aria-disabled", "true");
    previous.focus();
    fireEvent.click(previous);
    expect(position("kind")).toMatch(/^All kinds \(1 of/);
    expect(screen.getByTestId("results-navigator-status")).toHaveTextContent("First table kind reached.");
    expect(document.activeElement).toBe(previous);
    // "All cases" is the first case position: Previous stays there.
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.case.previous);
    expect(position("case")).toBe("All cases (1 of 5)");
    expect(screen.getByTestId("results-navigator-status")).toHaveTextContent("First load case or combination reached.");
    // Last case: the next key does not wrap to the first.
    stepUntil("case", "No case stated");
    expect(position("case")).toBe("No case stated (5 of 5)");
    expect(button("case", "next")).toHaveAttribute("aria-disabled", "true");
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.case.next);
    expect(position("case")).toBe("No case stated (5 of 5)");
    expect(screen.getByTestId("results-navigator-status")).toHaveTextContent("Last load case or combination reached.");
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("states an empty table and steps no support or element in it", () => {
    const onSelect = vi.fn();
    render(<Harness result={fixture()} onSelect={onSelect} />);
    stepUntil("kind", "Reaction");
    fireEvent.change(screen.getByTestId("results-navigator-case-select"), { target: { value: "unstated" } });
    expect(screen.getByTestId("result-filter-empty")).toBeInTheDocument();
    expect(screen.getByTestId("results-navigator-status")).toHaveTextContent("No rows of Reaction are stated for No case stated under the current filters.");
    expect(position("entity")).toBe("None chosen; 0 in this table");
    expect(button("entity", "next")).toHaveAttribute("aria-disabled", "true");
    expect(button("entity", "previous")).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(button("entity", "next"));
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("keeps focus in the panel when a key step from a row reaches an empty table", () => {
    const result = fixture();
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} />);
    stepUntil("kind", "Reaction");
    stepUntil("entity", SUPPORT);
    screen.getByTestId(`result-row-${onSelect.mock.lastCall![0]}`).focus();
    for (const caseId of ["load:é", "load:𝔫", "combination:Σ"]) {
      fireEvent.keyDown(document.activeElement!, key.case.next);
      expect(position("case")).toContain(caseId);
    }
    // From the focused row, the next case ("No case stated") has no Reaction rows: the row leaves,
    // and focus goes to the step's own button, not to the document.
    expect(document.activeElement).toBe(screen.getByTestId(`result-row-${onSelect.mock.lastCall![0]}`));
    const calls = onSelect.mock.calls.length;
    fireEvent.keyDown(document.activeElement!, key.case.next);
    expect(screen.getByTestId("result-filter-empty")).toBeInTheDocument();
    expect(document.activeElement).toBe(button("case", "next"));
    expect(onSelect).toHaveBeenCalledTimes(calls);
    // The keys still work from there.
    fireEvent.keyDown(document.activeElement!, key.case.previous);
    expect(position("case")).toContain("combination:Σ");
    expect(onSelect).toHaveBeenLastCalledWith(rowsOf(result, "reaction", SUPPORT, "combination:Σ")[0].id);
  });

  it("keeps focus in the panel when a key step from a row reaches a table without that entity", () => {
    const result = fixture();
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} />);
    stepUntil("kind", "Reaction");
    stepUntil("entity", SUPPORT);
    const row = screen.getByTestId(`result-row-${onSelect.mock.lastCall![0]}`);
    row.focus();
    fireEvent.keyDown(row, key.kind.next);
    expect(position("entity")).toBe(`${SUPPORT}: no rows in this table`);
    expect(row.isConnected).toBe(false);
    expect(document.activeElement).toBe(button("kind", "next"));
    // The keys still work from there.
    fireEvent.keyDown(document.activeElement!, key.kind.previous);
    expect(position("kind")).toMatch(/^Reaction/);
    expect(onSelect).toHaveBeenLastCalledWith(rowsOf(result, "reaction", SUPPORT, null)[0].id);
  });

  it("keeps a support or element that has no rows in the next table, and says so", () => {
    const onSelect = vi.fn();
    render(<Harness result={fixture()} onSelect={onSelect} />);
    stepUntil("kind", "Reaction");
    stepUntil("entity", SUPPORT);
    const calls = onSelect.mock.calls.length;
    fireEvent.click(button("kind", "next"));
    expect(position("entity")).toBe(`${SUPPORT}: no rows in this table`);
    expect(screen.getByTestId("results-navigator-status")).toHaveTextContent(`${SUPPORT} has no rows here.`);
    expect(onSelect).toHaveBeenCalledTimes(calls);
  });

  it("moves the table to the current row's page and scrolls that row into view", () => {
    const result = fixture();
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} />);
    // 531 rows at 50 per page (W-7's 72 global end-action rows make the eleventh page).
    expect(screen.getByTestId("result-page-summary")).toHaveTextContent("page 1 of 11");
    // From no position, Previous goes to the last support or element.
    const initiating = button("entity", "previous");
    initiating.focus();
    fireEvent.click(initiating);
    const id = onSelect.mock.lastCall![0] as string;
    const at = result.results.findIndex((row) => row.id === id);
    expect(at).toBeGreaterThanOrEqual(50);
    expect(screen.getByTestId("result-page-summary")).toHaveTextContent(`page ${Math.floor(at / 50) + 1} of 11`);
    const rowElement = screen.getByTestId(`result-row-${id}`);
    expect(scrolled).toEqual([rowElement]);
    // A button step leaves focus on its button.
    expect(document.activeElement).toBe(initiating);
  });

  it("leaves keys in the filter field and the page-size select to them", () => {
    const onSelect = vi.fn();
    render(<Harness result={fixture()} onSelect={onSelect} />);
    for (const target of [screen.getByTestId("result-filter-input"), screen.getByTestId("result-page-size"), screen.getByTestId("results-navigator-case-select")]) {
      for (const chord of [key.entity.next, key.case.next, key.kind.next]) {
        expect(fireEvent.keyDown(target, chord)).toBe(true);
      }
    }
    expect(position("kind")).toMatch(/^All kinds/);
    expect(position("case")).toMatch(/^All cases/);
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("changes no result and calls nothing but the row selection", () => {
    const result = fixture();
    const before = JSON.stringify(result);
    const onSelect = vi.fn(), onCause = vi.fn(), onDiagnostic = vi.fn();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={onSelect}
      onSelectCauseLocation={onCause} onShowDiagnostic={onDiagnostic} />);
    for (const axis of ["kind", "case", "entity", "entity", "case", "kind"] as Axis[]) fireEvent.keyDown(screen.getByTestId("results-panel"), key[axis].next);
    expect(onSelect).toHaveBeenCalled();
    expect(onCause).not.toHaveBeenCalled();
    expect(onDiagnostic).not.toHaveBeenCalled();
    expect(JSON.stringify(result)).toBe(before);
  });
});

describe("the navigator keeps results that are not checks passed, withheld or unsupported visible as such", () => {
  const SENSITIVE = { case_id: "load:é", state: "sensitive", code: "NUMERICAL_INTEGRITY_SENSITIVE", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:é", rcond: 1e-11, digits: 11, conditioning_sensitive: true, causes: [] };
  const PASSED = { case_id: "load:𝔫", state: "checks_passed", code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:𝔫", rcond: 0.01, digits: 2, conditioning_sensitive: false, causes: [] };
  const health = (withheldCombination: boolean) => numericalHealthView(parseNumericalHealth({
    route: "generic", verdict_only: true, cases: [SENSITIVE, PASSED],
    combinations: [{ combination_id: "combination:Σ", state: withheldCombination ? null : "sensitive", cases: withheldCombination ? [] : ["load:é", "load:𝔫"], withheld: withheldCombination }]
  }), { eligible: false, verdictHold: true })!;

  it("steps onto a sensitive case, names its health, and keeps its rows' badges", () => {
    const result = fixture();
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} health={health(false)} />);
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.case.next);
    expect(position("case")).toBe("Load case load:é (numerical health: sensitive) (2 of 5)");
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.entity.next);
    const id = onSelect.mock.lastCall![0] as string;
    expect(screen.getByTestId(`result-row-health-${id}`)).toHaveTextContent("sensitive");
    // A passed case carries no qualifier; the health header stays.
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.case.next);
    expect(position("case")).toBe("Load case load:𝔫 (3 of 5)");
    expect(screen.getByTestId("numerical-health-header")).toHaveTextContent("1 sensitive");
    expect(within(screen.getByTestId("results-navigator-case-select")).getByRole("option", { name: "Combination combination:Σ (numerical health: sensitive)" })).toBeInTheDocument();
  });

  it("steps onto a withheld combination's rows as the table shows them: listed, with no badge", () => {
    const result = fixture();
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} health={health(true)} />);
    fireEvent.change(screen.getByTestId("results-navigator-case-select"), { target: { value: "combination:combination:Σ" } });
    expect(position("case")).toBe("Combination combination:Σ (4 of 5)");
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.entity.next);
    const id = onSelect.mock.lastCall![0] as string;
    expect(result.results.find((row) => row.id === id)?.basis_ref?.ref_id).toBe("combination:Σ");
    expect(screen.getByTestId(`result-row-${id}`)).toBeInTheDocument();
    expect(screen.queryByTestId(`result-row-health-${id}`)).toBeNull();
  });

  it("steps an unsupported result's rows under its unchanged 'Needs recompute' standing", () => {
    const result = { schema_version: "0.1.0", document_kind: "openpipestress.mechanics_result", run_id: "tiny", model_ref: "fixture",
      status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" }, summary: {}, diagnostics: [],
      results: [["n1", "L1"], ["n2", "L1"], ["n1", "L2"]].map(([entity, loadCase], index) => ({ id: `tiny:${index}`, kind: "global_nodal_rotation_x", value: index, unit: "rad", entity_ref: entity, basis_ref: { ref_type: "load_case", ref_id: loadCase } })) } as unknown as MechanicsResult;
    const onSelect = vi.fn();
    render(<Harness result={result} onSelect={onSelect} />);
    expect(screen.getByTestId("numerical-result-standing")).toHaveTextContent("Needs recompute");
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.entity.next);
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.entity.next);
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.case.next);
    fireEvent.keyDown(screen.getByTestId("results-panel"), key.case.next);
    expect(onSelect.mock.calls.map(([id]) => id)).toEqual(["tiny:0", "tiny:1", "tiny:1"]);
    expect(position("entity")).toBe("n2: no rows in this table");
    expect(screen.getByTestId("numerical-result-standing")).toHaveTextContent("Needs recompute");
  });
});
