// Thin-adapter tests (TP-SEAM-SWAP-001, DEC-020 / ADR-0001). The adapter
// contains routing, wasm input-error envelope handling, and engine-status
// reporting ONLY. Operation semantics live in
// `core/model_operations/operation_applier` and are proven by the
// cross-engine contract corpus (operationContractCorpus.test.ts: 44 cases,
// native↔wasm parity including canonical hashes) — there is no TypeScript
// engine logic left to test. The 17 former browser-engine tests were
// superseded by that corpus per the operation-seam plan §3 T4.

import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, ModelHashEvidence, OperationOutcome, PreviewModel } from "../types";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import {
  applyModelOperation,
  initialOperationEngineStatus,
  outcomeFromWasmEngineJson,
  validateModelOperation,
  warmupOperationEngine
} from "./operationService";

const corpusDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../../fixtures/model_operations/contract_corpus"
);

type CorpusCase = {
  case_id: string;
  mode: "apply" | "validate_only";
  base_model: PreviewModel;
  intent: EditorOperationIntent;
  claimed_model_hash: ModelHashEvidence | null;
  expected: {
    outcome: { validation: Record<string, string> };
    applied_model: unknown | null;
  } | null;
};

function loadCorpusCases(): CorpusCase[] {
  return readdirSync(corpusDir)
    .filter((name) => name.endsWith(".json"))
    .sort()
    .map((name) => JSON.parse(readFileSync(path.join(corpusDir, name), "utf8")) as CorpusCase);
}

function acceptedApplyCase(): CorpusCase {
  const found = loadCorpusCases().find(
    (corpusCase) =>
      corpusCase.mode === "apply" &&
      corpusCase.expected?.outcome.validation.application_status === "applied_to_session_model"
  );
  expect(found, "corpus must contain an accepted apply case").toBeDefined();
  return found!;
}

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("operationService thin routing adapter", () => {
  it("routes browser-mode apply through the wasm engine with the honest route receipt", async () => {
    const corpusCase = acceptedApplyCase();
    const model = JSON.parse(JSON.stringify(corpusCase.base_model)) as PreviewModel;
    const snapshot = JSON.parse(JSON.stringify(corpusCase.base_model));

    const outcome = await applyModelOperation(model, corpusCase.intent, corpusCase.claimed_model_hash);

    expect(invokeMock).not.toHaveBeenCalled();
    expect(outcome.application_route).toBe("local_wasm_engine");
    expect(outcome.mode).toBe("apply");
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    expect(outcome.applied_model).toEqual(corpusCase.expected!.applied_model);
    expect(model, "the adapter must never mutate the input model document").toEqual(snapshot);
  });

  it("routes browser-mode validate through the wasm engine without applying", async () => {
    const corpusCase = acceptedApplyCase();
    const outcome = await validateModelOperation(
      corpusCase.base_model,
      corpusCase.intent,
      corpusCase.claimed_model_hash
    );

    expect(invokeMock).not.toHaveBeenCalled();
    expect(outcome.application_route).toBe("local_wasm_engine");
    expect(outcome.mode).toBe("validate_only");
    expect(outcome.applied_model).toBeNull();
  });

  it("routes to the Tauri backend commands verbatim when a backend is present", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const corpusCase = acceptedApplyCase();
    const sentinel = { application_route: "tauri_backend_apply" } as unknown as OperationOutcome;
    invokeMock.mockResolvedValue(sentinel);

    const applied = await applyModelOperation(corpusCase.base_model, corpusCase.intent, null);
    expect(applied).toBe(sentinel);
    expect(invokeMock).toHaveBeenCalledWith("apply_model_operation", {
      model: corpusCase.base_model,
      intent: corpusCase.intent,
      claimedModelHash: null
    });

    const validated = await validateModelOperation(
      corpusCase.base_model,
      corpusCase.intent,
      corpusCase.claimed_model_hash
    );
    expect(validated).toBe(sentinel);
    expect(invokeMock).toHaveBeenCalledWith("validate_model_operation", {
      model: corpusCase.base_model,
      intent: corpusCase.intent,
      claimedModelHash: corpusCase.claimed_model_hash
    });
  });

  it("surfaces the wasm input-error envelope as an explicit thrown error, never a silent outcome", () => {
    const envelope = JSON.stringify({
      document_kind: "openpipestress.desktop.wasm_engine_input_error",
      error: {
        code: "WASM-ENGINE-INPUT-JSON-INVALID",
        severity: "blocking",
        input: "model",
        message: "model is not valid JSON: expected value at line 1 column 2"
      }
    });
    expect(() => outcomeFromWasmEngineJson(envelope)).toThrowError(/WASM-ENGINE-INPUT-JSON-INVALID/);

    const passthrough = outcomeFromWasmEngineJson(JSON.stringify({ document_kind: "other", mode: "apply" }));
    expect(passthrough.mode).toBe("apply");
  });

  it("reports honest engine status for browser mode (loading, then ready)", async () => {
    expect(initialOperationEngineStatus()).toEqual({
      route: "local_wasm_engine",
      state: "loading",
      detail: null
    });
    expect(await warmupOperationEngine()).toEqual({
      route: "local_wasm_engine",
      state: "ready",
      detail: null
    });
  });

  it("reports the Tauri backend route as immediately ready when a backend is present", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    expect(initialOperationEngineStatus()).toEqual({
      route: "tauri_backend_apply",
      state: "ready",
      detail: null
    });
    expect(await warmupOperationEngine()).toEqual({
      route: "tauri_backend_apply",
      state: "ready",
      detail: null
    });
  });
});
