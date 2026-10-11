import { afterAll, describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { retainedReaderInputs } from "../../test-support/retainedPrecisionReaderInputs";
import { readRetainedPrecisionCorpus } from "../../test-support/retainedPrecisionCorpus";
import { retainedJsonText, validateRetainedPrecision as wasmValidate, validateRetainedPrecisionTransport as wasmTransport } from "./retainedPrecisionReader";
import { RetainedPrecisionError, validateRetainedPrecision as tsValidate, validateRetainedPrecisionTransport as tsTransport } from "./retainedPrecision";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Form = "bound" | "unbound" | "transport";

/** R1 (b7-plan Unit R), the equivalence gate. Item 1: through wasm, every reader input of the shared
 * retained corpus gives the native Rust reader's outcome, committed by result_export's
 * `r1_native_reader_outcomes`: the same input (the sha256 of the JCS form of [source, invocation or
 * null]) and, per reading, the same outcome JSON (its JCS sha256). */
const NATIVE = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/retained_precision_reader_outcomes.json"), "utf8")).entries as [string, string, Record<Form, [string, string]>][];
const INPUTS = retainedReaderInputs();

function summary(outcome: Json): string {
  return outcome.ok
    ? `pass${outcome.validation.numerical_eligible ? " eligible" : ""} rows=${outcome.validation.classifications.length}`
    : `${outcome.error.gate} ${outcome.error.code}${outcome.error.detail != null ? `: ${outcome.error.detail}` : ""}`;
}

/** Item 2 (while the TypeScript reader is present): wasm gives the TypeScript reader's outcome (gate and
 * code, or the whole validation), except where Rust reports a finer base code at G7 than TypeScript (both
 * refuse at G7, with different codes). Every corpus expectation whose TypeScript member differs from its
 * Rust member (`expected_by_reader`, `expected_unbound_by_reader`) is one of them, and the two readers give
 * exactly those members. With R1_GATE_TS_OUT set, every difference (and every difference in detail alone)
 * is written there. */
const corpus = readRetainedPrecisionCorpus();
const DECLARED = new Map<string, Json>();
for (const [kind, list] of [["mutation", "mutations"], ["must_pass", "must_pass"], ["case", "cases"]] as const) {
  for (const e of corpus[list]) {
    for (const [form, member] of [["bound", "expected_by_reader"], ["unbound", "expected_unbound_by_reader"]] as const) {
      const r = e[member];
      if (r && JSON.stringify(r.typescript) !== JSON.stringify(r.rust)) DECLARED.set(`corpus:${kind}:${e.id}|${form}`, r);
    }
  }
}
const differences: Json[] = [];
async function reading(run: () => Promise<Json>): Promise<Json> {
  try { return { ok: true, validation: await run() }; } catch (error) {
    expect(error).toBeInstanceOf(RetainedPrecisionError);
    const e = error as RetainedPrecisionError;
    return { ok: false, error: { gate: e.gate, code: e.code, detail: e.detail } };
  }
}
const read = (o: Json) => o.ok ? "pass" : { gate: o.error.gate, code: o.error.code };
const sorted = (v: unknown): unknown => Array.isArray(v) ? v.map(sorted)
  : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, sorted((v as Record<string, unknown>)[k])])) : v;
const sameJson = (a: unknown, b: unknown) => JSON.stringify(sorted(a)) === JSON.stringify(sorted(b));
afterAll(() => {
  if (process.env.R1_GATE_TS_OUT) writeFileSync(process.env.R1_GATE_TS_OUT, JSON.stringify({ declared_expectations: DECLARED.size, differences }, null, 1));
});

describe("R1 equivalence gate: the wasm reader is the native Rust reader on the shared retained corpus", () => {
  it("reads the native list's inputs, in its order", () => {
    expect(INPUTS.map(x => x.key)).toEqual(NATIVE.map(x => x[0]));
  });
  INPUTS.forEach(({ key, build }, i) => it(key, async () => {
    const engine = await loadWasmEngine();
    const [, nativeInput, nativeReadings] = NATIVE[i];
    const { source, invocation } = await build();
    const sourceJson = retainedJsonText(source), invocationJson = invocation === undefined ? undefined : retainedJsonText(invocation);
    expect(engine.canonicalSha256Hex(`[${sourceJson},${invocationJson ?? "null"}]`), "the native input").toBe(nativeInput);
    const forms: [Form, () => string, () => Promise<Json>, () => Promise<Json>][] = [
      ["unbound", () => engine.validateRetainedPrecisionJson(sourceJson), () => wasmValidate(source), () => tsValidate(structuredClone(source))],
      ["transport", () => engine.validateRetainedPrecisionTransportJson(sourceJson), () => wasmTransport(source), () => tsTransport(structuredClone(source))],
    ];
    if (invocationJson !== undefined) forms.unshift(["bound", () => engine.validateRetainedPrecisionJson(sourceJson, invocationJson),
      () => wasmValidate(source, invocation), () => tsValidate(structuredClone(source), structuredClone(invocation))]);
    expect(forms.map(f => f[0]).sort()).toEqual(Object.keys(nativeReadings).sort());
    const desktop = new Map<Form, Json>();
    for (const [form, raw, viaModule] of forms) {
      const outcome = raw(), parsed = JSON.parse(outcome);
      // Item 1: the native outcome, byte for byte in its JCS form.
      expect([engine.canonicalSha256Hex(outcome), summary(parsed)], `${form}: the native outcome`).toEqual(nativeReadings[form]);
      // The desktop's reader module carries that outcome unchanged.
      const carried = await reading(viaModule);
      expect(carried.ok ? { ok: true, validation: { ...carried.validation, standing: undefined } } : carried)
        .toEqual(parsed.ok ? { ok: true, validation: { ...parsed.validation, standing: undefined } } : parsed);
      desktop.set(form, carried);
    }
    // Item 2.
    const undeclared: string[] = [];
    for (const [form, , , ts] of forms) {
      const wasm = desktop.get(form)!, other = await reading(ts);
      const same = sameJson(read(other), read(wasm)) && (!other.ok || sameJson(other.validation, wasm.validation));
      const finerG7 = !wasm.ok && !other.ok && wasm.error.gate === "G7" && other.error.gate === "G7" && wasm.error.code !== other.error.code;
      const declared = DECLARED.get(`${key}|${form}`) ?? null;
      if (!same) differences.push({ key, form, wasm: summary({ ...wasm, error: wasm.error && { ...wasm.error, detail: null } }), typescript: summary({ ...other, error: other.error && { ...other.error, detail: null } }), corpus_declared: declared !== null });
      else if (!other.ok && other.error.detail !== wasm.error.detail) differences.push({ key, form, detail_only: true, gate: wasm.error.gate, wasm: wasm.error.detail, typescript: other.error.detail });
      if (declared) expect([read(wasm), read(other)], `${form}: the corpus's declared difference`).toEqual([declared.rust, declared.typescript]);
      if (!same && !finerG7) undeclared.push(`${form}: wasm ${summary(wasm)}; TypeScript ${summary(other)}`);
    }
    expect(undeclared, "differences outside item 2").toEqual([]);
  }, 120_000));
});
