import { describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const engine = vi.hoisted(() => ({ loaded: true }));
vi.mock("../../services/wasmEngine/loadWasmEngine", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/wasmEngine/loadWasmEngine")>();
  return { ...actual, loadedWasmEngine: () => (engine.loaded ? actual.loadedWasmEngine() : null) };
});
import { JSON_PARSE_REJECTED, RESULT_READER_UNAVAILABLE, ResultReaderError, readerDispatch, readerRowSemantics, readerSourceText, readerTransport, readerValidate, rememberReceivedText } from "./resultReader";
import { loadedWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { MechanicsResult } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const PREVIEW = "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";

/** R2: the reader module's text and engine boundaries (`resultReader.ts`). */
describe("the result reader module", () => {
  it("reads a value's received text while the value is unchanged, and its written text after an edit", () => {
    const bytes = text(PREVIEW), value = JSON.parse(bytes) as Json;
    rememberReceivedText(value, bytes);
    expect(readerSourceText(value)).toBe(bytes);
    expect(readerSourceText(structuredClone(value))).not.toBe(bytes); // a copy has no received text
    value.run_id = `${value.run_id}:edited`;
    const written = readerSourceText(value);
    expect(written).not.toBe(bytes);
    expect(JSON.parse(written).run_id).toBe(value.run_id);
    // Written text keeps signed zeros (the bundled result carries -0.0 moments).
    expect(bytes).toContain("-0.0");
    expect(written).toContain(":-0");
  });
  it("refuses a value with no JSON text at the text boundary", () => {
    const value = JSON.parse(text(PREVIEW)) as Json;
    value.results[0].value = Number.NaN;
    expect(readerDispatch(value)).toMatchObject({ ok: false, route: "unsupported", error: { code: JSON_PARSE_REJECTED } });
    expect(() => readerValidate("preview-physics", value)).toThrow(JSON_PARSE_REJECTED);
  });
  it("reads nothing without the engine, fails closed, and caches no such verdict", () => {
    const value = JSON.parse(text(PREVIEW)) as MechanicsResult;
    engine.loaded = false;
    try {
      expect(readerDispatch(value)).toMatchObject({ ok: false, route: "unsupported", binding: null, error: { code: RESULT_READER_UNAVAILABLE }, standing: { eligible: false, findings: [RESULT_READER_UNAVAILABLE] } });
      expect(() => readerValidate("preview-physics", value)).toThrow(ResultReaderError);
      expect(() => readerRowSemantics(value)(value.results[0])).toThrow(RESULT_READER_UNAVAILABLE);
    } finally { engine.loaded = true; }
    expect(readerDispatch(value)).toMatchObject({ ok: true, route: "preview_physics" });
    expect(readerRowSemantics(value)(value.results[0]).signature).not.toBeNull();
  });
  /** A row's outcome through the call per row, as the module read it before the batch: its
   * semantics, or the refusal's whole code and detail (JSON_PARSE_REJECTED for text the engine
   * cannot parse). */
  const perRow = (row: Json, sourceJson: string) => {
    let o: Json;
    try { o = JSON.parse(loadedWasmEngine()!.readerRowSemanticsJson(JSON.stringify(row), sourceJson)); }
    catch (error) { if ((error as Error).message.startsWith("WASM-ENGINE-INPUT-JSON-INVALID")) return { code: JSON_PARSE_REJECTED, detail: null }; throw error; }
    return o.ok ? { signature: o.signature, complete_metadata: o.complete_metadata, canonical_metadata: o.canonical_metadata } : { code: o.error.code, detail: o.error.detail };
  };
  const carried = (f: () => unknown) => { try { return f(); } catch (error) { const { code, detail } = error as ResultReaderError; return { code, detail }; } };
  const contradictory = (row: Json) => ({ ...row, id: `${row.id}:contradictory`, kind: "element_local_axial_normal_stress", unit: "MPa",
    metadata: { ...row.metadata, basis: "recovered_from_local_element_stiffness" } });
  it("reads a source's rows in one engine call, each row as its own call reads it", () => {
    const e = loadedWasmEngine()!;
    const value = JSON.parse(text(PREVIEW)) as Json;
    value.run_id = `${value.run_id}:rows-read-once`; // text no other test has read
    const sourceJson = readerSourceText(value);
    const outside = contradictory(value.results[0]);
    const expected = [...value.results, outside].map((row: Json) => perRow(row, sourceJson));
    const each = vi.spyOn(e, "readerRowSemanticsEachJson"), single = vi.spyOn(e, "readerRowSemanticsJson");
    try {
      const semantics = readerRowSemantics(value);
      expect(value.results.map((row: Json) => carried(() => semantics(row)))).toEqual(expected.slice(0, -1));
      expect(each).toHaveBeenCalledTimes(1);
      expect(single).not.toHaveBeenCalled();
      // Another reading of the same text reads nothing again; a row outside the source is read alone.
      expect(carried(() => readerRowSemantics(value)(value.results[1]))).toEqual(expected[1]);
      expect(carried(() => semantics(outside))).toEqual(expected.at(-1));
      expect([each.mock.calls.length, single.mock.calls.length]).toEqual([1, 1]);
    } finally { each.mockRestore(); single.mockRestore(); }
  });
  it("reads each row alone, with the per-row code and detail, when the engine refuses the batch", () => {
    const e = loadedWasmEngine()!;
    // A source the engine cannot parse: a lone surrogate (written as its escape) outside every row.
    const broken = JSON.parse(text(PREVIEW)) as Json;
    broken.run_id = `${broken.run_id}:\ud800`;
    const brokenJson = readerSourceText(broken);
    expect(brokenJson).toContain("\\ud800");
    // A source the engine reads, with a contradictory row among its own, and the batch refused.
    const value = JSON.parse(text(PREVIEW)) as Json;
    value.run_id = `${value.run_id}:batch-refused`;
    value.results.push(contradictory(value.results[0]));
    const sourceJson = readerSourceText(value);
    const expectedBroken = broken.results.map((row: Json) => perRow(row, brokenJson));
    const expected = value.results.map((row: Json) => perRow(row, sourceJson));
    expect(expectedBroken.every((o: Json) => o.code === JSON_PARSE_REJECTED && o.detail === null)).toBe(true);
    expect(expected.some((o: Json) => typeof o.detail === "string")).toBe(true); // a refusal that carries its detail
    const keys = (rows: Json[]) => new Set(rows.map(row => JSON.stringify([row.kind, row.unit, row.metadata]))).size;
    const each = vi.spyOn(e, "readerRowSemanticsEachJson"), single = vi.spyOn(e, "readerRowSemanticsJson");
    try {
      const semanticsBroken = readerRowSemantics(broken);
      expect(broken.results.map((row: Json) => carried(() => semanticsBroken(row)))).toEqual(expectedBroken);
      expect(each).toHaveBeenCalledTimes(1);
      expect(each.mock.results[0].type).toBe("throw");
      expect(single).toHaveBeenCalledTimes(keys(broken.results));
      each.mockClear(); single.mockClear();
      each.mockImplementation(() => { throw new Error("batch refused"); });
      const semantics = readerRowSemantics(value);
      expect(value.results.map((row: Json) => carried(() => semantics(row)))).toEqual(expected);
      expect(each).toHaveBeenCalledTimes(1);
      expect(single).toHaveBeenCalledTimes(keys(value.results));
    } finally { each.mockRestore(); single.mockRestore(); }
  });
});

it("carries the composite source transport and portable source commitments through wasm", () => {
  // B4/D1 (b): source blocks remain only on the composites; source-blocks-1 has no reader.
  // B7b: the composite is pressure-source-1; the retired physics-source reader name is unknown.
  const source = JSON.parse(text("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")) as Json;
  expect(readerValidate("source-recovery", source)).toEqual({});
  expect(() => readerTransport("pressure-source", structuredClone(source))).not.toThrow();
  const bad = structuredClone(source);
  bad.source_block_recovery.receipt_sha256 = "0".repeat(64);
  expect(() => readerTransport("pressure-source", bad)).toThrow();
  expect(() => readerTransport("source-blocks" as never, structuredClone(source))).toThrow("READER_UNKNOWN");
  expect(() => readerTransport("physics-source" as never, structuredClone(source))).toThrow("READER_UNKNOWN");
  source.run_id = "edited-publication";
  expect(() => readerValidate("source-recovery", source)).toThrow("SOURCE_BLOCKS_PUBLICATION_HASH");
});
