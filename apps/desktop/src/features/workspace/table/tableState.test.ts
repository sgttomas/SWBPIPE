import { describe, expect, it } from "vitest";
import { entityKey } from "../selectionState";
import { capturedCellIsCurrent, coordinateError, type CapturedCell, type TableRow } from "./tableState";

const row: TableRow = { key: entityKey({ type: "node", id: "a:b" }), label: "a:b", cells: { x: { value: "1", unit: "m" } } };
const captured: CapturedCell = { rowKey: row.key, columnKey: "x", row, token: 1, before: "1", unit: "m", generation: "p:1" };
describe("captured cell basis", () => {
  it("rejects deletion, before/unit changes and same-ID session replacement", () => {
    expect(capturedCellIsCurrent(captured, [row], "p:1")).toBe(true);
    expect(capturedCellIsCurrent(captured, [], "p:1")).toBe(false);
    expect(capturedCellIsCurrent(captured, [row], "p:2")).toBe(false);
    expect(capturedCellIsCurrent(captured, [{ ...row, cells: { x: { value: "2", unit: "m" } } }], "p:1")).toBe(false);
    expect(capturedCellIsCurrent(captured, [{ ...row, cells: { x: { value: "1", unit: "mm" } } }], "p:1")).toBe(false);
  });
  it("accepts entered-unit decimal numbers without pretending suffix or feet-inch support", () => {
    for (const text of ["-0.5", "+1", ".5", "1e3"]) expect(coordinateError(text)).toBeUndefined();
    for (const text of ["", " ", "0x12", "NaN", "Infinity", "1e999", "2 mm", "1'6"]) expect(coordinateError(text)).toBeTruthy();
  });
});
