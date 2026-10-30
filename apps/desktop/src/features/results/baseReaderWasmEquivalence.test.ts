import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { baseReaderInputs, type BaseReaderInput } from "../../test-support/baseReaderInputs";
import { ResultReaderError, readerDispatch, readerRowSemantics, readerTransport, readerValidate, rememberReceivedText, type ReaderName, type TransportReaderName } from "./resultReader";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Entry = { id: string; parse?: string; dispatch?: Json; validate?: Record<string, Json>; transport?: Record<string, Json>; rows?: { sha256: string; signature_ids: string[] } };

/** R2 (b7-plan Unit R), the equivalence gate's item 1, kept as the desktop reader's
 * conformance test: through wasm, every statement of the shared base corpus gives the
 * native Rust outcome committed by result_export's `base_reader_outcomes` (the dispatch,
 * each reader's validation and transport, each row's semantics by digest), and the
 * desktop's reader module (`resultReader.ts`) carries those outcomes unchanged when it
 * reads the statement's received text. (Item 2, wasm against the TypeScript readers,
 * ran while they were present; b7-plan R2 records its counts.) */
const NATIVE = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/base_reader_outcomes.json"), "utf8")).entries as Entry[];
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
const sortKeys = (v: Json): Json => Array.isArray(v) ? v.map(sortKeys) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, sortKeys(v[k])])) : v;
/** A value read from its text, the text kept as received. */
function received(text: string | undefined): Json {
  if (text === undefined) return undefined;
  const value = JSON.parse(text);
  if (value && typeof value === "object") rememberReceivedText(value, text);
  return value;
}
function carried(run: () => unknown): Json {
  try { const value = run(); return { ok: true, ...(value as object ?? {}) }; } catch (error) {
    expect(error).toBeInstanceOf(ResultReaderError);
    const e = error as ResultReaderError;
    return { ok: false, error: { code: e.code, detail: e.detail } };
  }
}

const INPUTS = await baseReaderInputs();

describe("R2 equivalence gate, item 1: the wasm readers are the native Rust readers on the shared base corpus", () => {
  it("builds the native list's statements, in its order", () => {
    expect(INPUTS.map(x => x.id)).toEqual(NATIVE.map(x => x.id));
  });
  it.each(INPUTS.map((input, i) => [input.id, input, NATIVE[i]] as [string, BaseReaderInput, Entry]))("%s", async (_id, input, native) => {
    const engine = await loadWasmEngine();
    if (!input.parsed) {
      expect(native.parse).toBe("JSON_PARSE_REJECTED");
      expect(() => engine.readerDispatchJson(input.sourceJson)).toThrow(/WASM-ENGINE-INPUT-JSON-INVALID/);
      // A value JavaScript reads from such a text (a non-finite number) has no JSON text for the module.
      let value: Json = undefined;
      try { value = JSON.parse(input.sourceJson); } catch { return; }
      expect(readerDispatch(value).error?.code).toBe("JSON_PARSE_REJECTED");
      return;
    }
    const { sourceJson, modelJson, invocationJson } = input;
    const dispatch = JSON.parse(engine.readerDispatchJson(sourceJson, modelJson, invocationJson));
    expect(sortKeys(dispatch), "dispatch").toEqual(sortKeys(native.dispatch));
    const source = received(sourceJson), model = received(modelJson), invocation = received(invocationJson);
    expect(sortKeys(readerDispatch(source, model, invocation)), "dispatch, through the module").toEqual(sortKeys(dispatch));
    expect(Object.keys(native.validate ?? {}).sort()).toEqual([...input.readers].sort());
    for (const reader of input.readers) {
      const outcome = JSON.parse(engine.readerValidateJson(reader, sourceJson, modelJson, invocationJson));
      expect(sortKeys(outcome), `validate ${reader}`).toEqual(sortKeys(native.validate![reader]));
      expect(sortKeys(carried(() => readerValidate(reader as ReaderName, source, model, invocation))), `validate ${reader}, through the module`).toEqual(sortKeys(outcome));
    }
    expect(Object.keys(native.transport ?? {}).sort()).toEqual([...input.transports].sort());
    for (const reader of input.transports) {
      const outcome = JSON.parse(engine.readerTransportJson(reader, sourceJson));
      expect(sortKeys(outcome), `transport ${reader}`).toEqual(sortKeys(native.transport![reader]));
      expect(sortKeys(carried(() => readerTransport(reader as TransportReaderName, source))), `transport ${reader}, through the module`).toEqual(sortKeys(outcome));
    }
    expect(input.rowsJson !== null, "rows read").toBe(native.rows !== undefined);
    if (input.rowsJson !== null) {
      const rows = input.rowsJson.map(row => engine.readerRowSemanticsJson(row, sourceJson));
      expect(sha(`[${rows.join(",")}]`), "rows digest").toBe(native.rows!.sha256);
      const semantics = readerRowSemantics(source);
      rows.forEach((text, k) => {
        const outcome = JSON.parse(text);
        expect(native.rows!.signature_ids[k]).toBe(outcome.ok ? outcome.signature?.signature_id ?? null : outcome.error.code);
        expect(sortKeys(carried(() => semantics(source.results[k]))), `row ${k}, through the module`).toEqual(sortKeys(outcome));
      });
    }
  }, 120_000);
});
