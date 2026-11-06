/** C1 slice 5's builder: the native analysis-run record's oracle and its preview adapter.
 *
 * - `apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json` equals its generator
 *   (`test-support/analysisRunV03Cases.ts`): every case's outcome is the unchanged TypeScript
 *   builder's on the case's held text and inputs. The native leg
 *   (`analysis_run_v03_build_tests.rs`) checks its records against these byte for byte, in the
 *   record's stored form. ANALYSIS_RUN_V03_CASES_WRITE=1 rewrites the file.
 * - The adapter reads the command's raw-body response (an ArrayBuffer), refuses text and every
 *   other shape by name, and sends `buildAnalysisRunPreview`'s inputs without the parsed manifest. */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { ANALYSIS_RUN_V03_CASES, analysisRunV03Cases, casesText, heldText, rawTopLevelMember } from "../test-support/analysisRunV03Cases";
import {
  ANALYSIS_RUN_PREVIEW_COMMAND, ANALYSIS_RUN_PREVIEW_KIND, ANALYSIS_RUN_PREVIEW_MALFORMED, ANALYSIS_RUN_ROW_SCALE_MEMBERS,
  decodeAnalysisRunPreview, previewChecksum, readAnalysisRunPreview
} from "./analysisRunPreview";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../");
const WRITE = process.env.ANALYSIS_RUN_V03_CASES_WRITE === "1";

describe("the native analysis-run builder's oracle", () => {
  it("the committed cases are the TypeScript builders' outcomes", async () => {
    const file = await analysisRunV03Cases();
    const text = casesText(file);
    const path = resolve(ROOT, ANALYSIS_RUN_V03_CASES);
    if (WRITE) writeFileSync(path, text);
    const committed = readFileSync(path, "utf8");
    // Compared case by case, so a difference names its case rather than printing the file.
    const lines = (t: string) => t.split("\n");
    expect(lines(committed).length).toBe(lines(text).length);
    lines(text).forEach((line, index) => expect(lines(committed)[index] === line, `line ${index + 1}: ${line.slice(0, 120)}`).toBe(true));
    const records = file.cases.filter(c => "record" in c.expected).length;
    expect(records).toBeGreaterThanOrEqual(25);
    expect(file.cases.length - records).toBeGreaterThanOrEqual(15);
    for (const route of ["pressure", "pressure_source", "preview_physics", "retained_preview_physics"]) {
      expect(file.cases.some(c => "record" in c.expected && c.expected.record.analysis_run.reproducibility.semantic_contract?.id.includes(route === "pressure" ? "pressure-2" : route === "pressure_source" ? "pressure-source" : route === "preview_physics" ? "preview-physics-2" : "retained")), route).toBe(true);
    }
  }, 900_000);

  it("reads a member's exact bytes as the native test does", () => {
    const text = '{ "a" : [1, {"b": "}\\"]"}] ,"source":  {"x": -0.0, "y": "\\u00e9"} , "source": {"z": 1.0}}';
    expect(rawTopLevelMember(text, "source")).toBe('{"z": 1.0}');
    expect(rawTopLevelMember(text, "a")).toBe('[1, {"b": "}\\"]"}]');
    const member = heldText({ path: "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json", member: "source" });
    expect(JSON.parse(member)).toEqual(JSON.parse(readFileSync(resolve(ROOT, "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"), "utf8")).source);
  });
});

/** A header as the native command writes it, from a committed record. */
function header(record: Json, extra: Json = {}): Json {
  const copy = structuredClone(record);
  for (const member of ANALYSIS_RUN_ROW_SCALE_MEMBERS) delete copy.analysis_run[member];
  return { document_kind: ANALYSIS_RUN_PREVIEW_KIND, version: 1, job_id: "job-1", route: "preview_physics", record_bytes: 1234, rule_check_status: "RULE_INPUTS_INCOMPLETE", result_ref_count: record.analysis_run.result_refs.length, producer_diagnostic_count: 0, carried_members: [], record: copy, rule_pack_ref_diagnostics: [], ...extra };
}
const bytes = (value: unknown) => new TextEncoder().encode(JSON.stringify(value)).buffer;

describe("the preview adapter", () => {
  let committed: Json = null;
  const recordOf = (): Json => {
    committed ??= JSON.parse(readFileSync(resolve(ROOT, ANALYSIS_RUN_V03_CASES), "utf8"));
    return committed.cases.find((c: Json) => c.kind === "preview" && c.expected.record).expected.record;
  };

  it("reads the raw-body ArrayBuffer and sends buildAnalysisRunPreview's inputs without the parsed manifest", async () => {
    const record = recordOf();
    const calls: Array<[string, Json]> = [];
    const evidence = { manifest: { never: "sent" }, manifest_ref: { object_type: "InputManifest" as const, ref: "input-manifest:x" }, manifest_sha256: "1".repeat(64), canonical_bytes: "{}" };
    const preview = await readAnalysisRunPreview(async (command, args) => { calls.push([command, args]); return bytes(header(record)); }, "job-1", { inputManifest: evidence, ruleCheckAggregate: "USER_RULE_CHECKED" });
    expect(calls).toEqual([[ANALYSIS_RUN_PREVIEW_COMMAND, { jobId: "job-1", inputManifest: { manifest_ref: evidence.manifest_ref, manifest_sha256: evidence.manifest_sha256, canonical_bytes: "{}" }, ruleCheckAggregate: "USER_RULE_CHECKED", rulePackExclusions: [] }]]);
    expect(previewChecksum(preview, "analysis_run_record")).toBe(record.analysis_run.hashes[0].value);
    expect(previewChecksum(preview, "received_result")).toBe(record.analysis_run.hashes[1].value);
    expect(preview.result_ref_count).toBe(record.analysis_run.result_refs.length);
    expect(Object.isFrozen(preview.record.analysis_run)).toBe(true);
    // A Uint8Array view decodes alike.
    expect(decodeAnalysisRunPreview(new Uint8Array(bytes(header(record))))).toEqual(preview);
  });

  it("refuses a text response and every other shape by name", async () => {
    const record = recordOf();
    await expect(readAnalysisRunPreview(async () => JSON.stringify(header(record)), "job-1", { inputManifest: { manifest_ref: { object_type: "InputManifest" as const, ref: "x" }, manifest_sha256: "1".repeat(64), canonical_bytes: "{}" } })).rejects.toThrow(ANALYSIS_RUN_PREVIEW_MALFORMED);
    const withRows = header(record); withRows.record.analysis_run.result_refs = record.analysis_run.result_refs;
    const noClaim = header(record); noClaim.record.analysis_run.hashes = noClaim.record.analysis_run.hashes.slice(1);
    const badHash = header(record); badHash.record.analysis_run.hashes[0].value = "X".repeat(64);
    for (const bad of [withRows, noClaim, badHash, header(record, { version: 2 }), header(record, { document_kind: "other" }), header(record, { record_bytes: -1 }), header(record, { carried_members: "x" })]) {
      expect(() => decodeAnalysisRunPreview(bytes(bad))).toThrow(ANALYSIS_RUN_PREVIEW_MALFORMED);
    }
    expect(() => decodeAnalysisRunPreview(new Uint8Array([0xff, 0xfe]))).toThrow(ANALYSIS_RUN_PREVIEW_MALFORMED);
  });
});
