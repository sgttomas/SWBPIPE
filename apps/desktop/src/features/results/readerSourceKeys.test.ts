/** The reader's cache keys (`resultReader.ts`): a received text is cached by its receipt, a
 * written text by its digest, so no cache keeps a whole text, and no reading is served from
 * the entry of another text: a changed value is read again as its rewritten text, two raw
 * texts of equal values keep apart, and each receipt starts afresh. */
import { describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readerDispatch, readerSourceRead, readerSourceText, readerValidate, rememberReceivedText } from "./resultReader";
import { loadedWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { equalityDigestOf } from "../../services/equalityDigest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const PREVIEW = readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"), "utf8");
/** The texts the engine's dispatch was handed while `run` ran. */
function dispatched(run: () => void): string[] {
  const spy = vi.spyOn(loadedWasmEngine()!, "readerDispatchJson");
  try { run(); return spy.mock.calls.map((call) => call[0]); } finally { spy.mockRestore(); }
}
/** A fresh preview statement with its own run id, so no earlier test's entry is shared. */
function statement(tag: string): { text: string; value: Json } {
  const value = JSON.parse(PREVIEW) as Json;
  const text = PREVIEW.replace(`"run_id": ${JSON.stringify(value.run_id)}`, `"run_id": ${JSON.stringify(`${value.run_id}:${tag}`)}`);
  expect(text).not.toBe(PREVIEW);
  return { text, value: JSON.parse(text) };
}

describe("the reader's cache keys", () => {
  it("keys a received text by its receipt and a written text by its digest, in disjoint namespaces", () => {
    const { text, value } = statement("keys");
    rememberReceivedText(value, text);
    expect(readerSourceRead(value)).toEqual({ text, key: expect.stringMatching(/^r:\d+$/) });
    const copy = structuredClone(value);
    const written = jsonTextKeepingNegativeZero(copy);
    expect(readerSourceRead(copy)).toEqual({ text: written, key: `w:${equalityDigestOf(written)}` });
  });

  it("reads a received result's text, then its rewritten text after a change, never from the stale entry", () => {
    const { text, value } = statement("changed");
    rememberReceivedText(value, text);
    expect(dispatched(() => readerDispatch(value))).toEqual([text]);
    expect(dispatched(() => readerDispatch(value))).toEqual([]); // the receipt's entry
    value.producer.semantic_contract_id = "invented:not-a-contract";
    const rewritten = jsonTextKeepingNegativeZero(value);
    expect(readerSourceText(value)).toBe(rewritten);
    let route = "";
    expect(dispatched(() => { route = readerDispatch(value).route; })).toEqual([rewritten]);
    expect(route).toBe("unsupported");
    expect(() => readerValidate("preview-physics", value)).toThrow();
    // Restored, it is the received statement again, read from the receipt's entry.
    value.producer.semantic_contract_id = JSON.parse(text).producer.semantic_contract_id;
    expect(readerSourceText(value)).toBe(text);
    expect(dispatched(() => { route = readerDispatch(value).route; })).toEqual([]);
    expect(route).toBe("preview_physics");
  });

  it("keeps two raw texts of equal values apart", () => {
    const { text } = statement("raw-forms");
    expect(text).toContain("-0.0");
    const other = text.replace(/-0\.0(?![0-9])/g, "-0");
    expect(other).not.toBe(text);
    const a = JSON.parse(text) as Json, b = JSON.parse(other) as Json;
    expect(jsonTextKeepingNegativeZero(a)).toBe(jsonTextKeepingNegativeZero(b));
    rememberReceivedText(a, text);
    rememberReceivedText(b, other);
    expect(readerSourceRead(a).key).not.toBe(readerSourceRead(b).key);
    expect(dispatched(() => { readerDispatch(a); readerDispatch(b); })).toEqual([text, other]);
    // 1.0 and 1: a number's form in the text is its own statement too.
    const one = '{"n":1.0}', plain = '{"n":1}';
    const c = JSON.parse(one), d = JSON.parse(plain);
    rememberReceivedText(c, one); rememberReceivedText(d, plain);
    expect([readerSourceText(c), readerSourceText(d)]).toEqual([one, plain]);
    expect(readerSourceRead(c).key).not.toBe(readerSourceRead(d).key);
  });

  it("starts afresh at each receipt, also of the same value", () => {
    const { text, value } = statement("again");
    rememberReceivedText(value, text);
    const first = readerSourceRead(value).key;
    expect(dispatched(() => readerDispatch(value))).toEqual([text]);
    // The same value received again as another text of it: read as that text, not from the first entry.
    const spaced = `\n${text}\n`;
    rememberReceivedText(value, spaced);
    expect(readerSourceRead(value)).toEqual({ text: spaced, key: expect.not.stringMatching(`^${first}$`) });
    expect(dispatched(() => readerDispatch(value))).toEqual([spaced]);
    // Received again as the same text: a new receipt, read afresh.
    rememberReceivedText(value, spaced);
    expect(dispatched(() => readerDispatch(value))).toEqual([spaced]);
  });
});
