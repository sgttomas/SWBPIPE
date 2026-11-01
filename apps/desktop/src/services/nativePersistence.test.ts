/** C1 slice 5's persistence adapter (`nativePersistence.ts`): what the webview sends (the job handle
 * and small inputs, exactly) and how it reads the native save's reply (the actual commands' replies,
 * `native_persistence_replies.json`; a reply carrying a model, result or record, or of another
 * shape, is refused by name). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import {
  decodeNativeProjectSave,
  decodeRestoredProjectSave,
  NATIVE_SAVE_RESTORED_PROJECT_COMMAND,
  saveRestoredLocalProject,
  NATIVE_PROJECT_SAVE_MALFORMED,
  NATIVE_RECORD_RUN_COMMAND,
  NATIVE_SAVE_PROJECT_COMMAND,
  recordNativeAnalysisRun,
  saveNativeLocalProject,
  storedModelHashEvidence,
  type NativePersistenceRequest
} from "./nativePersistence";
import type { PreviewModel } from "../types";
import type { RestoredRun } from "./nativeReopen";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const replies = JSON.parse(readFileSync(resolve(__dirname, "../test-support/native-replay/native_persistence_replies.json"), "utf8")) as { scenarios: Json[] };
const saveReplies = replies.scenarios.filter((scenario) => scenario.command === NATIVE_SAVE_PROJECT_COMMAND).map((scenario) => scenario.reply.value);

afterEach(() => invokeMock.mockReset());

const REQUEST: NativePersistenceRequest = {
  jobId: "job-1",
  inputManifest: { manifest_ref: { object_type: "InputManifest", ref: "input-manifest:x" }, manifest_sha256: "1".repeat(64), canonical_bytes: "{}" },
  ruleCheckAggregate: null,
  rulePackExclusions: [],
  expectedRecord: { run_id: "run:x", record_sha256: "2".repeat(64), received_sha256: "3".repeat(64) },
  intakeText: '{"project":{"id":"project:invented","name":"Invented"}}'
};
const MODEL = { project: { id: "project:invented", name: "Invented" } } as unknown as PreviewModel;

describe("the native save's reply", () => {
  it("reads the actual commands' replies, saved and not saved", () => {
    expect(saveReplies.map((reply) => decodeNativeProjectSave(reply).result.outcome)).toEqual(["saved", "not_saved"]);
    const [saved, unsaved] = saveReplies.map(decodeNativeProjectSave);
    expect(storedModelHashEvidence(saved, MODEL)).toEqual({ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "model_payload", payload_ref: "project:invented", value: saved.stored_model_hash, hash_status: "computed_local_preview" });
    expect(unsaved.result.outcome === "not_saved" && unsaved.result.code).toBe("PROJECT-SAVE-RESULT-TOO-LARGE");
    expect(unsaved.summary.persisted_mechanics_result_count).toBe(0);
    expect(saved.summary.persisted_mechanics_result_count).toBe(1);
  });

  it("refuses a reply carrying a model, result or record, or of another shape", () => {
    const reply = saveReplies[0];
    for (const bad of [
      null, [], "text",
      { ...reply, model: {} }, { ...reply, mechanics_result: {} }, { ...reply, analysis_run: {} }, { ...reply, editor_intents: [] },
      { ...reply, summary: null }, { ...reply, project_envelope_hash: null }, { ...reply, stored_model_hash: "sha256:x" },
      { ...reply, result: { outcome: "other" } }, { ...reply, result: { ...reply.result, result_bytes: -1 } }
    ]) {
      expect(() => decodeNativeProjectSave(bad), JSON.stringify(bad)?.slice(0, 80)).toThrow(NATIVE_PROJECT_SAVE_MALFORMED);
    }
  });
});

describe("what the webview sends", () => {
  it("a save sends the job handle and the project's small fields only; a create names no project", async () => {
    invokeMock.mockResolvedValue(saveReplies[0]);
    (window as Json).__TAURI_INTERNALS__ = {};
    const fields = { model: MODEL, editorIntents: [], proposal: null, selectedReviewTarget: null, modelHash: null, confirmedRevision: null };
    await saveNativeLocalProject(REQUEST, { ...fields, mode: "save", expectedBase: "sha256:base" });
    await saveNativeLocalProject(REQUEST, { ...fields, mode: "create", expectedBase: null });
    const [[saveCommand, saveArgs], [, createArgs]] = invokeMock.mock.calls;
    expect(saveCommand).toBe(NATIVE_SAVE_PROJECT_COMMAND);
    expect(Object.keys(saveArgs).sort()).toEqual(["confirmedRevision", "editorIntents", "expectedBase", "expectedRecord", "inputManifest", "intakeText", "jobId", "mode", "model", "modelHash", "projectId", "projectName", "proposal", "ruleCheckAggregate", "rulePackExclusions", "selectedReviewTarget"]);
    expect(saveArgs).toMatchObject({ ...REQUEST, mode: "save", projectId: "project:invented", projectName: "Invented", expectedBase: "sha256:base" });
    expect(createArgs).toMatchObject({ mode: "create", projectId: null, projectName: null, expectedBase: null });
    delete (window as Json).__TAURI_INTERNALS__;
  });

  it("Record Run sends the job handle and its own members under `request`", async () => {
    invokeMock.mockResolvedValue({ message: "Recorded." });
    const members = { expected_base: "sha256:base", state_id: null, new_state: null, run_labels: { run_name: "Run", tags: [], notes: [], assumptions: [] } };
    await recordNativeAnalysisRun(REQUEST, members);
    expect(invokeMock.mock.calls.map(([command, args]) => [command, args])).toEqual([[NATIVE_RECORD_RUN_COMMAND, { ...REQUEST, request: members }]]);
  });
});

// C1 slice 6's save continuity: the actual re-save reply (`native_reopen_replies.json`, written by
// `stored_custody_tests.rs`) on the restored handle the reopen returned.
const reopen = JSON.parse(readFileSync(resolve(__dirname, "../test-support/native-reopen/native_reopen_replies.json"), "utf8")) as Json;
const HANDLE = "restored-stored-result-unit-1";
const swap = (value: unknown): Json => JSON.parse(JSON.stringify(value).split("$HANDLE").join(HANDLE));
const RESTORED = swap(reopen.open.reply.value.stored_run) as RestoredRun;
const RESAVED = swap((reopen.requests as Json[]).find((entry) => entry.command === NATIVE_SAVE_RESTORED_PROJECT_COMMAND).reply.value);

describe("a reopened saved run's re-save", () => {
  it("sends the handle, its stored source, the envelope basis and the small fields; never a hash, result or record", async () => {
    invokeMock.mockResolvedValue(RESAVED);
    const basis = { payload_sha256: RESTORED.findings.envelope_payload_hash, evidence: reopen.open.reply.value.project_envelope_hash };
    const reply = await saveRestoredLocalProject(RESTORED, basis, { mode: "save", model: MODEL, editorIntents: [], proposal: null, selectedReviewTarget: null, modelHash: { value: "sha256:ignored" } as never, confirmedRevision: null, expectedBase: "sha256:base" });
    expect(reply.restored?.handle).toBe(HANDLE);
    const [command, args] = invokeMock.mock.calls[0];
    expect(command).toBe(NATIVE_SAVE_RESTORED_PROJECT_COMMAND);
    expect(args).toEqual({ jobId: HANDLE, expectedSource: RESTORED.source, envelopeBasis: basis, mode: "save", projectId: "project:invented", projectName: "Invented", model: MODEL, editorIntents: [], proposal: null, selectedReviewTarget: null, confirmedRevision: null, expectedBase: "sha256:base" });
  });

  it("reads the actual reply on its handle and refuses one that is not the handle's or of another shape", () => {
    expect(decodeRestoredProjectSave(RESAVED, RESTORED).restored?.findings?.findings).toEqual(RESTORED.findings.findings);
    // An unchanged re-save of a project stored without an envelope hash keeps none.
    expect(() => decodeRestoredProjectSave({ ...RESAVED, project_envelope_hash: null }, RESTORED)).not.toThrow();
    // A model-only re-save: not saved, no findings.
    const unsaved = { ...RESAVED, result: saveReplies[1].result, restored: { ...RESAVED.restored, findings: null } };
    expect(decodeRestoredProjectSave(unsaved, RESTORED).result.outcome).toBe("not_saved");
    for (const bad of [
      { ...RESAVED, restored: undefined },
      { ...RESAVED, restored: { ...RESAVED.restored, handle: "restored-stored-result-other" } },
      { ...RESAVED, restored: { ...RESAVED.restored, origin: "restored_recorded_run" } },
      { ...RESAVED, restored: { ...RESAVED.restored, source: { ...RESAVED.restored.source, result_sha256: "0".repeat(64) } } },
      { ...RESAVED, restored: { ...RESAVED.restored, findings: null } },
      { ...unsaved, restored: { ...unsaved.restored, findings: RESAVED.restored.findings } },
      { ...RESAVED, restored: { ...RESAVED.restored, envelope_payload_hash: null } },
      { ...RESAVED, mechanics_result: {} }
    ]) {
      expect(() => decodeRestoredProjectSave(bad, RESTORED), JSON.stringify(bad?.restored)?.slice(0, 80)).toThrow(NATIVE_PROJECT_SAVE_MALFORMED);
    }
    // A fresh save's reply never carries a restored run.
    expect(() => decodeNativeProjectSave(RESAVED)).toThrow(NATIVE_PROJECT_SAVE_MALFORMED);
  });
});
