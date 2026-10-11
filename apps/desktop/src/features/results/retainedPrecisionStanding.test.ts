import { beforeEach, describe, expect, it, vi } from "vitest";
import { RETAINED_PRECISION_READER_UNAVAILABLE, registerRetainedPrecision, retainedPrecisionRegistration } from "./retainedPrecisionStanding";
import { RetainedPrecisionError } from "./retainedPrecisionTypes";
import type { MechanicsResult } from "../../types";

const engine = vi.hoisted(() => ({ load: null as null | (() => Promise<unknown>) }));
vi.mock("../../services/wasmEngine/loadWasmEngine", () => ({ loadWasmEngine: () => engine.load!() }));

const source = () => ({ schema_version: "0.2.0", results: [] }) as unknown as MechanicsResult;
const outcome = (text: string) => ({ validateRetainedPrecisionJson: () => text, validateRetainedPrecisionTransportJson: () => text });

describe("registration records the reader's code, or a named code when the reader could not run", () => {
  beforeEach(() => { engine.load = null; });
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
