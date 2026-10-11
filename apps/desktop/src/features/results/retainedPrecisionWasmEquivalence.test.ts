import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { retainedReaderInputs } from "../../test-support/retainedPrecisionReaderInputs";
import { retainedJsonText, validateRetainedPrecision, validateRetainedPrecisionTransport } from "./retainedPrecisionReader";
import { RetainedPrecisionError } from "./retainedPrecisionTypes";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Form = "bound" | "unbound" | "transport";

/** R1 (b7-plan Unit R), the equivalence gate's item 1, kept as the desktop reader's conformance test:
 * through wasm, every reader input of the shared retained corpus gives the native Rust reader's outcome,
 * committed by result_export's `r1_native_reader_outcomes`: the same input (the sha256 of the JCS form of
 * [source, invocation or null]) and, per reading, the same outcome JSON (its JCS sha256); and the desktop's
 * reader module (`retainedPrecisionReader.ts`) carries that outcome unchanged. */
const NATIVE = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/retained_precision_reader_outcomes.json"), "utf8")).entries as [string, string, Record<Form, [string, string]>][];
const INPUTS = retainedReaderInputs();

function summary(outcome: Json): string {
  return outcome.ok
    ? `pass${outcome.validation.numerical_eligible ? " eligible" : ""} rows=${outcome.validation.classifications.length}`
    : `${outcome.error.gate} ${outcome.error.code}${outcome.error.detail != null ? `: ${outcome.error.detail}` : ""}`;
}

async function reading(run: () => Promise<Json>): Promise<Json> {
  try { return { ok: true, validation: await run() }; } catch (error) {
    expect(error).toBeInstanceOf(RetainedPrecisionError);
    const e = error as RetainedPrecisionError;
    return { ok: false, error: { gate: e.gate, code: e.code, detail: e.detail } };
  }
}

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
    const forms: [Form, () => string, () => Promise<Json>][] = [
      ["unbound", () => engine.validateRetainedPrecisionJson(sourceJson), () => validateRetainedPrecision(source)],
      ["transport", () => engine.validateRetainedPrecisionTransportJson(sourceJson), () => validateRetainedPrecisionTransport(source)],
    ];
    if (invocationJson !== undefined) forms.unshift(["bound", () => engine.validateRetainedPrecisionJson(sourceJson, invocationJson), () => validateRetainedPrecision(source, invocation)]);
    expect(forms.map(f => f[0]).sort()).toEqual(Object.keys(nativeReadings).sort());
    for (const [form, raw, viaModule] of forms) {
      const outcome = raw(), parsed = JSON.parse(outcome);
      expect([engine.canonicalSha256Hex(outcome), summary(parsed)], `${form}: the native outcome`).toEqual(nativeReadings[form]);
      const carried = await reading(viaModule);
      expect(carried.ok ? { ok: true, validation: { ...carried.validation, standing: undefined } } : carried)
        .toEqual(parsed.ok ? { ok: true, validation: { ...parsed.validation, standing: undefined } } : parsed);
    }
  }, 120_000));
});
