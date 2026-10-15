import { describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const engine = vi.hoisted(() => ({ loaded: true }));
vi.mock("../../services/wasmEngine/loadWasmEngine", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/wasmEngine/loadWasmEngine")>();
  return { ...actual, loadedWasmEngine: () => (engine.loaded ? actual.loadedWasmEngine() : null) };
});
import { JSON_PARSE_REJECTED, RESULT_READER_UNAVAILABLE, ResultReaderError, readerDispatch, readerRowSemantics, readerSourceText, readerTransport, readerValidate, rememberReceivedText } from "./resultReader";
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
