import { describe, expect, it } from "vitest";
import type { MechanicsResult } from "../types";
import { buildAnalysisRunV02, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { checkedJsonText } from "./hashService";
import expectedRecord from "../../../../fixtures/analysis_runs/invented/analysis_run_v0_2.json";

const result: MechanicsResult = {
  schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "run:test", model_ref: "model:test",
  status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" }, diagnostics: [], summary: {},
  results: [{ id: "result:test", entity_ref: "node:test", kind: "displacement_magnitude", value: 1, unit: "mm" }],
};
const manifest = { manifest_ref: { object_type: "InputManifest" as const, ref: "input-manifest:test" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: "model:test" }, solver_basis: { solver_name: "solver", solver_version: "0.1.0", solver_build_ref: "build:test" } } };

describe("analysis record 0.2", () => {
  it("preserves raw rows and binds immutable rule revisions", async () => {
    const before = structuredClone(result);
    const first = await buildAnalysisRunV02(result, manifest);
    const revised = await buildAnalysisRunV02(result, manifest, "USER_RULE_CHECKED");
    expect(result).toEqual(before);
    expect(first.schema_version).toBe("0.2.0");
    expect(first.analysis_run.result_refs).toHaveLength(1);
    expect(first.analysis_run.analysis_status).not.toContain("NOT_PROVIDED");
    expect(first.analysis_run.solver_version).toEqual({ solver_name: "solver", solver_version: "0.1.0", build_ref: { object_type: "ExternalReference", ref: "build:test" } });
    expect(first.analysis_run.hashes.find((row) => row.payload_scope === "received_result")?.value).toBe(revised.analysis_run.hashes.find((row) => row.payload_scope === "received_result")?.value);
    expect(first.analysis_run.hashes.find((row) => row.payload_scope === "analysis_run_record")?.value).not.toBe(revised.analysis_run.hashes.find((row) => row.payload_scope === "analysis_run_record")?.value);
    expect(await verifyAnalysisRunRecord(first)).toBe("match");
  });

  it("matches the language-neutral Python fixture byte for byte", async () => {
    const withBasis = structuredClone(result);
    withBasis.results[0].basis_ref = { ref_type: "load_case", ref_id: "load:test" };
    expect(await buildAnalysisRunV02(withBasis, manifest)).toEqual(expectedRecord);
  });

  it("rejects unsupported JavaScript shapes before transport", () => {
    expect(() => checkedJsonText([, 1])).toThrow("SPARSE-ARRAY");
    expect(() => checkedJsonText({ bad: "\ud800" })).toThrow("LONE-SURROGATE");
    expect(() => checkedJsonText({ [Symbol("bad")]: 1 })).toThrow("SYMBOL-KEY");
    const accessor = {}; Object.defineProperty(accessor, "x", { enumerable: true, get: () => 1 });
    expect(() => checkedJsonText(accessor)).toThrow("UNSUPPORTED-PROPERTY");
    const custom = {}; Object.defineProperty(custom, "toJSON", { value: () => ({}) });
    expect(() => checkedJsonText(custom)).toThrow("UNSUPPORTED-PROPERTY");
    const mutable = { x: 1 }; let reads = 0;
    const proxy = new Proxy(mutable, { getOwnPropertyDescriptor(target, key) { const descriptor = Reflect.getOwnPropertyDescriptor(target, key)!; if (key === "x") descriptor.value = ++reads; return descriptor; }, get() { throw new Error("validated source was traversed twice"); } });
    expect(checkedJsonText(proxy)).toBe('{"x":1}');
    expect(reads).toBe(1);
    const topLevelProto = JSON.parse('{"__proto__":{"polluted":true},"x":1}');
    const nestedProto = JSON.parse('{"outer":{"__proto__":{"polluted":true},"x":1}}');
    expect(JSON.parse(checkedJsonText(topLevelProto))).toEqual(topLevelProto);
    expect(JSON.parse(checkedJsonText(nestedProto))).toEqual(nestedProto);
    expect(({} as { polluted?: boolean }).polluted).toBeUndefined();
  });
});
