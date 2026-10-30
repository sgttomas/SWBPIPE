import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { retainedJsonText, validateRetainedPrecision, validateRetainedPrecisionBytes, validateRetainedPrecisionTransport } from "./retainedPrecisionReader";
import { RetainedPrecisionError } from "./retainedPrecisionTypes";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const text = readFileSync(resolve(__dirname, "../../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"), "utf8");
const refusal = (run: () => Promise<unknown>) => run().then(() => null, (e: RetainedPrecisionError) => {
  expect(e).toBeInstanceOf(RetainedPrecisionError);
  return { gate: e.gate, code: e.code, message: e.message };
});

describe("the desktop's retained reader module carries bytes to the Rust reader and its outcome back", () => {
  it("writes a parsed value as JSON text that keeps negative zeros", () => {
    const value = { a: -0, b: [0, -0, { c: -0 }], d: "x", e: null, f: 1.5e-7, g: true };
    expect(JSON.stringify(value)).toBe('{"a":0,"b":[0,0,{"c":0}],"d":"x","e":null,"f":1.5e-7,"g":true}');
    expect(retainedJsonText(value)).toBe('{"a":-0,"b":[0,-0,{"c":-0}],"d":"x","e":null,"f":1.5e-7,"g":true}');
    expect(Object.is(JSON.parse(retainedJsonText(value)).b[1], -0)).toBe(true);
  });
  it("reads exact bytes and a parsed value alike, frozen, with the standing the eligibility implies", async () => {
    const doc = JSON.parse(text) as Json;
    const fromBytes = await validateRetainedPrecisionBytes(JSON.stringify(doc.source), doc.invocation);
    const fromValue = await validateRetainedPrecision(doc.source, doc.invocation);
    expect(fromValue).toStrictEqual(fromBytes);
    expect([fromValue.invocation_bound, fromValue.numerical_eligible, fromValue.standing]).toEqual([true, true, "eligible"]);
    expect(Object.isFrozen(fromValue) && Object.isFrozen(fromValue.classifications[0])).toBe(true);
    const unbound = await validateRetainedPrecision(doc.source);
    expect([unbound.invocation_bound, unbound.numerical_eligible, unbound.standing]).toEqual([false, false, "needs_recompute"]);
    expect((await validateRetainedPrecisionTransport(doc.source)).classifications).toEqual([]);
  });
  it("throws the reader's refusal with its gate, code and detail; the message is the code", async () => {
    const doc = JSON.parse(text) as Json;
    doc.source.results[0].value = 12345;
    expect(await refusal(() => validateRetainedPrecision(doc.source, doc.invocation))).toEqual({ gate: "G1", code: "RETAINED_PRECISION_RECEIPT_MISMATCH", message: "RETAINED_PRECISION_RECEIPT_MISMATCH" });
  });
  it("refuses at G1 a value outside checked JSON and text that is not JSON", async () => {
    const G1 = { gate: "G1", code: "RETAINED_PRECISION_RECEIPT_MISMATCH", message: "RETAINED_PRECISION_RECEIPT_MISMATCH" };
    expect(await refusal(() => validateRetainedPrecision({ a: Number.NaN }))).toEqual(G1);
    expect(await refusal(() => validateRetainedPrecision({ a: 2 ** 60 }))).toEqual(G1);
    expect(await refusal(() => validateRetainedPrecisionTransport({ a: undefined }))).toEqual(G1);
    expect(await refusal(() => validateRetainedPrecisionBytes("{"))).toEqual(G1);
    expect(await refusal(() => validateRetainedPrecisionBytes(text, { a: Number.POSITIVE_INFINITY }))).toEqual(G1);
  });
});
