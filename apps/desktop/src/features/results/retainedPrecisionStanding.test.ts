import { beforeEach, describe, expect, it, vi } from "vitest";
import { RETAINED_PRECISION_READER_UNAVAILABLE, registerRetainedPrecision, retainedPrecisionRegistration, retainedPrecisionStanding, classificationSummary } from "./retainedPrecisionStanding";
import { RetainedPrecisionError } from "./retainedPrecisionTypes";
import type { MechanicsResult } from "../../types";

const engine = vi.hoisted(() => ({ load: null as null | (() => Promise<unknown>), current: null as unknown }));
vi.mock("../../services/wasmEngine/loadWasmEngine", () => ({
  loadWasmEngine: async () => { engine.current = await engine.load!(); return engine.current; },
  loadedWasmEngine: () => engine.current,
}));

const source = () => ({ schema_version: "0.2.0", results: [] }) as unknown as MechanicsResult;
const outcome = (text: string) => ({ validateRetainedPrecisionJson: () => text, validateRetainedPrecisionTransportJson: () => text });

describe("registration records the reader's code, or a named code when the reader could not run", () => {
  beforeEach(() => { engine.load = null; engine.current = null; });
  it("records the reader's refusal code and rethrows the refusal", async () => {
    engine.load = async () => outcome(JSON.stringify({ ok: false, error: { gate: "G1", code: "RETAINED_PRECISION_RECEIPT_MISMATCH", detail: null } }));
    const doc = source();
    await expect(registerRetainedPrecision(doc, null)).rejects.toBeInstanceOf(RetainedPrecisionError);
    expect(retainedPrecisionRegistration(doc)).toEqual({ validation: null, error: "RETAINED_PRECISION_RECEIPT_MISMATCH" });
  });
  it("records RETAINED_PRECISION_READER_UNAVAILABLE, not the raw message, when the engine does not load or fails", async () => {
    for (const load of [
      async () => { throw new Error("WASM-ENGINE-ASSET-ABSENT: /wasm-engine/x.wasm"); },
      async () => ({ validateRetainedPrecisionJson: () => { throw new Error("RuntimeError: unreachable"); } }),
    ]) {
      engine.load = load;
      const doc = source();
      await expect(registerRetainedPrecision(doc, null)).rejects.not.toBeInstanceOf(RetainedPrecisionError);
      expect(retainedPrecisionRegistration(doc)).toEqual({ validation: null, error: RETAINED_PRECISION_READER_UNAVAILABLE });
    }
  });
});

describe("the registered semantic verdict fails closed when Rust cannot read it", () => {
  it.each(["missing", "invalid wire", "engine failure"])("%s keeps standing unsupported and publishes no summary", async failure => {
    const validation = { invocation_bound: true, numerical_eligible: true, publication_sha256: "a".repeat(64), classifications: [] };
    const unavailable = failure === "invalid wire" ? "WASM-ENGINE-INPUT-VALIDATION-INVALID" : "RuntimeError: unreachable";
    engine.load = async () => ({
      ...outcome(JSON.stringify({ ok: true, validation })),
      retainedPrecisionSemanticsJson: () => { throw new Error(unavailable); },
    });
    const doc = source();
    await registerRetainedPrecision(doc, null, () => true);
    if (failure === "missing") engine.current = null;
    expect(retainedPrecisionStanding(doc, { load_cases: [] })).toEqual({ standing: "unsupported", eligible: false, findings: [RETAINED_PRECISION_READER_UNAVAILABLE] });
    expect(classificationSummary(doc, { load_cases: [] })).toEqual([]);
  });
});
