// The result rows' Case column: the id as the result states it ("Not stated" where a row states
// none), the model's name as its hover text, and "producer's recovery" beside the id of a load case
// whose values come from the producer's recovery, on that case's rows only. Committed results only.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { ResultsPanel } from "./ResultsPanel";
import { CASE_NOT_STATED, PRODUCER_RECOVERY_WORDS, ResultRowsTable } from "./ResultRowsTable";
import { recoveredCaseIds } from "./integrityStatus";
import type { MechanicsResult } from "../../types";
import unicodeSparse from "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json";

afterEach(cleanup);

const mixed = () => JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json"), "utf8")) as MechanicsResult;
const generic = () => structuredClone(unicodeSparse) as unknown as MechanicsResult;
const panel = (result: MechanicsResult) =>
  render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
const shownRows = () => screen.getAllByTestId(/^result-row-result/).filter((row) => row.tagName === "TR");
const rowOf = (result: MechanicsResult, row: HTMLElement) => result.results.find((item) => item.id === row.getAttribute("data-row-id"))!;
/** Restricts the table to the case whose option names `id`. */
function chooseCase(id: string) {
  const select = screen.getByTestId("results-navigator-case-select") as HTMLSelectElement;
  const option = [...select.options].find((item) => (id === "" ? item.textContent === "All cases" : item.value.endsWith(`:${id}`)));
  expect(option, id).toBeDefined();
  fireEvent.change(select, { target: { value: option!.value } });
}

describe("the result rows' Case column", () => {
  it("says producer's recovery beside the recovered case's id, on its rows only", () => {
    const result = mixed();
    expect([...recoveredCaseIds(result)]).toEqual(["case"]);
    panel(result);
    for (const [id, recovered] of [["case", true], ["case:ordinary-pressure", false]] as const) {
      chooseCase(id);
      const rows = shownRows();
      expect(rows.length, id).toBeGreaterThan(0);
      for (const row of rows) {
        const item = rowOf(result, row);
        expect(item.basis_ref?.ref_id).toBe(id);
        const cell = within(row).getByTestId(`result-row-case-${item.id}`);
        expect(cell.textContent).toBe(recovered ? `${id} ${PRODUCER_RECOVERY_WORDS}` : id);
      }
    }
  });

  it("never says it on another route's rows, combination rows among them, and reads Not stated without a case", () => {
    const result = generic();
    // One row with no case stated (removed in memory from the committed result).
    const unstatedId = result.results.find((row) => row.basis_ref)!.id;
    delete (result.results.find((row) => row.id === unstatedId) as { basis_ref?: unknown }).basis_ref;
    panel(result);
    expect(screen.queryAllByTestId(/^result-row-case-recovery-/)).toHaveLength(0);
    expect(document.body.textContent).not.toContain(PRODUCER_RECOVERY_WORDS);
    expect(screen.getByTestId(`result-row-case-${unstatedId}`).textContent).toBe(CASE_NOT_STATED);
    // A combination's rows, chosen by the case selector: none says it.
    const combination = result.results.find((row) => row.basis_ref?.ref_type === "combination")!.basis_ref!.ref_id;
    chooseCase(combination);
    const combinationRows = shownRows();
    expect(combinationRows.length).toBeGreaterThan(0);
    for (const row of combinationRows) {
      expect(rowOf(result, row).basis_ref?.ref_type).toBe("combination");
      expect(row.textContent).not.toContain(PRODUCER_RECOVERY_WORDS);
    }
    chooseCase("");
    // Without the model, the id is the cell, with no name beside it.
    const stated = shownRows().find((row) => rowOf(result, row).basis_ref)!;
    const item = rowOf(result, stated);
    const cell = within(stated).getByTestId(`result-row-case-${item.id}`);
    expect(cell.textContent).toBe(item.basis_ref!.ref_id);
    expect(cell.getAttribute("title")).toBeNull();
  });

  it("never says it on a combination row, even of a combination whose id a recovered case shares", () => {
    const row = (id: string, ref_type: string) => ({ id, kind: "global_nodal_displacement_x", entity_ref: "node:A", value: 1, unit: "mm", basis_ref: { ref_type, ref_id: "case" }, metadata: {} });
    const items = [row("r:load", "load_case"), row("r:combo", "combination")] as unknown as MechanicsResult["results"];
    render(
      <ResultRowsTable csvKind="t" items={items} selectedResultId={null} onSelectResult={() => {}} entityName={(ref) => ref} caseName={(basis) => basis?.ref_id ?? CASE_NOT_STATED}
        recoveredCases={new Set(["case"])} dimension={() => "length"} frameText={() => ""} notes={() => null} sort={null} onSort={() => {}} sortDisabledReason={null} />
    );
    expect(screen.getByTestId("result-row-case-r:load").textContent).toBe(`case ${PRODUCER_RECOVERY_WORDS}`);
    expect(screen.getByTestId("result-row-case-r:combo").textContent).toBe("case");
  });
});
