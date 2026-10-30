import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expandRetainedPrecisionDefinition } from "../../test-support/retainedPrecisionCorpus";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { ResultReaderError, readerDispatch, readerSourceText, readerTransport, readerValidate, rememberReceivedText, type ReaderName, type TransportReaderName } from "./resultReader";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Entry = { id: string; reader: ReaderName; source: Json; invocation: Json; transport_source: Json; raw_fixture?: string; expected: { validation: Json; unbound: Json; transport: Json; dispatch: Json } };
const ROOT = resolve(__dirname, "../../../../../");
const CORPUS = expandRetainedPrecisionDefinition(JSON.parse(readFileSync(resolve(ROOT, "fixtures/results/pressure_source_reader_corpus.json"), "utf8")));
const ENTRIES = CORPUS.entries as Entry[];
const sorted = (value: Json): Json => Array.isArray(value) ? value.map(sorted) : value && typeof value === "object" ? Object.fromEntries(Object.keys(value).sort().map(key => [key, sorted(value[key])])) : value;
function carried(run: () => unknown): Json {
  try { return { ok: true, ...(run() as object ?? {}) }; }
  catch (error) {
    expect(error).toBeInstanceOf(ResultReaderError);
    const refusal = error as ResultReaderError;
    return { ok: false, error: { code: refusal.code, detail: refusal.detail } };
  }
}
function received(source: Json, path?: string): { source: Json; text: string } {
  const text = path ? readFileSync(resolve(ROOT, path), "utf8") : readerSourceText(source);
  const value = JSON.parse(text);
  expect(sorted(value)).toEqual(sorted(source));
  rememberReceivedText(value, text);
  return { source: value, text };
}
describe("J2 pressure-source corpus, unchanged native outcomes through real wasm", () => {
  it("has all 24 genuine public producer twins and named negative families", () => {
    expect(CORPUS.artifact).toBe("pressure-source-reader-corpus");
    expect(CORPUS.schema_version).toBe("1.0.0");
    expect(ENTRIES.filter(entry => entry.raw_fixture)).toHaveLength(24);
    expect(new Set(ENTRIES.map(entry => entry.id)).size).toBe(ENTRIES.length);
    // B7b: MJ5's v3-offered-to-v2 family went with the v2 reader; the regenerated J2
    // corpus names the retired v2 identity, the pressure-2 cross-offer and a captured v2 context. The RF
    // source-method extension adds receipt statements the current producer cannot make (a 5-DOF block, an unknown
    // level, a stored unsupported_block of order 3 or 4), each refused at the receipt shape.
    for (const name of ["MJ2-wrong-domain", "MJ4-stale-receipt", "MJ4-publication-binding", "MJ5-retired-v2-identity", "MJ5-pressure-source-offered-to-pressure-1", "MJ5-captured-v2-context", "MJ5-relabelled-shape", "MJ7-downgrade", "MJ3-selected-pressure-", "RF-free-block-5-dofs", "RF-unknown-source-level", "RF-unsupported-block-order-3", "RF-unsupported-block-order-4"]) {
      expect(ENTRIES.some(entry => entry.id.includes(name)), name).toBe(true);
    }
  });
  it.each(ENTRIES)("$id", async entry => {
    const engine = await loadWasmEngine();
    const { source, text } = received(entry.source, entry.raw_fixture);
    const model = entry.invocation.request.model;
    const modelText = readerSourceText(model), invocationText = readerSourceText(entry.invocation);
    const transported = received(entry.transport_source);
    const wasm = {
      validation: JSON.parse(engine.readerValidateJson(entry.reader, text, modelText, invocationText)),
      unbound: JSON.parse(engine.readerValidateJson(entry.reader, text)),
      transport: JSON.parse(engine.readerTransportJson(entry.reader, transported.text)),
      dispatch: JSON.parse(engine.readerDispatchJson(text, modelText, invocationText)),
    };
    expect(sorted(wasm)).toEqual(sorted(entry.expected));
    expect(sorted({
      validation: carried(() => readerValidate(entry.reader, source, model, entry.invocation)),
      unbound: carried(() => readerValidate(entry.reader, source)),
      transport: carried(() => readerTransport(entry.reader as TransportReaderName, transported.source)),
      dispatch: readerDispatch(source, model, entry.invocation),
    })).toEqual(sorted(entry.expected));
  });
});
