// W-18b desktop slice (DEL-08-01 Design `report-user-release.md` §2.2, §2.3 "Session", §2.5,
// §11): the webview side of one shareable save, from a replayed solve to the native arguments
// of route `DREP-PACKAGE-USER-RELEASE-010`. The request it sends must be the committed
// derivation vectors' request (the native derivation's), for the pressure-free and both exact
// shapes. Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
// qualification. The confirmation time is invented; no user release is recorded.
// Standard claim fence applies (claims taxonomy per DEC-081).
import { readFileSync } from "node:fs";
import path from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { controlUserReleasedReportPackageRequest } from "../redaction-controls/redactionExportControls";
import { invokeUserReleasedReportPackageSave, prepareShareableReportPackageSave, prepareUserReleasedReportPackageSave } from "./userReleasedReportPackageSave";

afterEach(() => {
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

type Json = Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = path.resolve(__dirname, "../../../../..");
const load = (file: string) => JSON.parse(readFileSync(path.join(root, file), "utf8")) as Json;
/** The invented confirmation time the committed vectors' records carry. */
const INVENTED_USER_RELEASED_AT = "2026-10-27T00:00:00Z";
const SPARSE_SETTINGS = { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true };

async function manifestFor(model: PreviewModel, result: MechanicsResult) {
  const producer = result.producer!;
  return buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: producer.component_name,
      solver_version: producer.component_version,
      solver_build_ref: `${producer.component_name}@${producer.component_version}`,
      solver_mode: "sparse_interactive",
      settings: SPARSE_SETTINGS
    },
    active_rule_packs: [],
    external_assets: []
  });
}

async function pressureFreeSession() {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
  const result = await runPreviewMechanics(model, "sparse_interactive");
  return { model, result, inputManifest: await manifestFor(model, result) };
}

describe("Save Shareable Report Package… preparation (route 010)", () => {
  for (const name of ["connected_sparse_interactive", "composed_model_input_sparse_interactive"]) {
    it(`sends the derivation vector's request, run record and input manifest (${name})`, async () => {
      const vector = (load("fixtures/reports/invented/user_release_derivation_vectors.json").vectors as Json[]).find((item) => item.name === name)!;
      const { model, result, inputManifest } = await pressureFreeSession();
      const { controlled, prepared } = await prepareShareableReportPackageSave({
        model,
        result,
        inputManifest,
        reportComposition: vector.report_composition ? structuredClone(vector.report_composition) : null,
        confirmation: { userReleasedAt: INVENTED_USER_RELEASED_AT }
      });
      expect(controlled.blocked).toBe(false);
      expect(prepared.route).toBe("ready");
      if (prepared.route !== "ready") return;
      const { args } = prepared;
      expect(Object.keys(args).sort()).toEqual([
        "analysisRunCanonicalText", "inputManifestCanonicalText", "localFirstEvidence", "modelCanonicalText", "requestCanonicalText", "screening"
      ]);
      const request = JSON.parse(args.requestCanonicalText) as Json;
      expect(await canonicalSha256Hex(request)).toBe(vector.expected.request_sha256);
      expect(request.report.report_context.user_release).toEqual(vector.user_release);
      expect(request.report.report_context.user_release.record_version).toBe("1.0.0");
      expect(args.analysisRunCanonicalText).toBe(await canonicalJsonString(vector.run_record));
      expect(args.inputManifestCanonicalText).toBe(await canonicalJsonString(vector.input_manifest));
      expect(args.modelCanonicalText).toBe(await canonicalJsonString(model));
      expect(args.screening).toEqual({
        route_id: "DREP-PACKAGE-USER-RELEASE-010",
        export_context: "user_released_report",
        explicit_local_private_intent: false,
        explicit_user_release_intent: true,
        blocked: false,
        materialization_withheld: false,
        lossless_required: true,
        exact_payload_match: true,
        blocking_count: 0,
        decision_count: controlled.summary.decision_count,
        finding_count: controlled.summary.finding_count,
        payload_sha256: `sha256:${await canonicalSha256Hex(request)}`
      });
      expect(args.localFirstEvidence).toMatchObject({
        route_id: "DREP-PACKAGE-USER-RELEASE-010",
        export_context: "user_released_report",
        storage_context: "private_report",
        action: "include_metadata_only",
        reason_code: "USER_RELEASE_METADATA_ALLOWED",
        blocked: false,
        metadata_only: true,
        explicit_local_private_intent: false,
        explicit_user_release_intent: true
      });
    });
  }

  for (const shape of ["pressure_2_straight", "pressure_source_1"]) {
    it(`sends the D-6 vector's request with record 1.1.0 and the delivered texts (${shape})`, async () => {
      const fixture = load(`fixtures/reports/invented/exact_report_package_requests/${shape}.json`);
      const vector = load(`fixtures/reports/invented/user_release_exact/${shape}.json`);
      const model = fixture.model as PreviewModel;
      const bytes = readFileSync(path.join(root, fixture.raw_path), "utf8");
      (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
      invokeMock.mockImplementation(createSolveJobTransport(() => bytes, { jobIdPrefix: `invented-d6-${shape}` }).invoke);
      const result = await runPreviewMechanics(model);
      const inputManifest = await manifestFor(model, result);
      const { prepared } = await prepareShareableReportPackageSave({
        model, result, inputManifest, reportComposition: null, confirmation: { userReleasedAt: INVENTED_USER_RELEASED_AT }
      });
      expect(prepared.route).toBe("ready");
      if (prepared.route !== "ready") return;
      const request = JSON.parse(prepared.args.requestCanonicalText) as Json;
      expect(request.report.report_context.user_release.record_version).toBe("1.1.0");
      expect(request.report.report_context.user_release.scope.not_carried.at(-1)).toBe("ineligible_exact_route_results");
      expect(prepared.args.requestCanonicalText).toBe(await canonicalJsonString(vector.request));
      expect(prepared.args.analysisRunCanonicalText).toBe(await canonicalJsonString(vector.run_record));
      expect(prepared.args.exactResultText).toBe(bytes);
      expect(prepared.args.exactIntakeText).toBe(vector.exact_intake_text);
    });
  }

  it("refuses route evidence that is not route 010's, and a browser without the native host", async () => {
    const { model, result, inputManifest } = await pressureFreeSession();
    const { prepared, request } = await prepareShareableReportPackageSave({
      model, result, inputManifest, reportComposition: null, confirmation: { userReleasedAt: INVENTED_USER_RELEASED_AT }
    });
    expect(prepared.route).toBe("ready");
    const withoutIntent = controlUserReleasedReportPackageRequest(request, model, false);
    expect((await prepareUserReleasedReportPackageSave(withoutIntent, model, request, inputManifest.manifest, {})).route).toBe("redaction_blocked");
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    const allowed = controlUserReleasedReportPackageRequest(request, model, true);
    expect(await prepareUserReleasedReportPackageSave(allowed, model, request, inputManifest.manifest, {})).toEqual({
      route: "unavailable_browser", diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY"
    });
  });

  it("refuses a confirmation time that is not the record's form", async () => {
    const { model, result, inputManifest } = await pressureFreeSession();
    await expect(prepareShareableReportPackageSave({
      model, result, inputManifest, reportComposition: null, confirmation: { userReleasedAt: "2026-10-27T00:00:00.000Z" }
    })).rejects.toThrow("REPORT-USER-RELEASE-RECORD-INVALID");
  });

  it("invokes save_report_package with the prepared arguments only while current", async () => {
    const { model, result, inputManifest } = await pressureFreeSession();
    const { prepared } = await prepareShareableReportPackageSave({
      model, result, inputManifest, reportComposition: null, confirmation: { userReleasedAt: INVENTED_USER_RELEASED_AT }
    });
    if (prepared.route !== "ready") throw new Error("not ready");
    // The native receipt carries the binding; the status returned keeps only that a user release
    // is recorded (REQ-019: the binding stays in the report and its package).
    for (const [code, durability] of [["REPORT-PACKAGE-SAVED", "durable"], ["REPORT-PACKAGE-SAVED-DURABILITY-UNCERTAIN", "saved_durability_uncertain"]]) {
      invokeMock.mockClear();
      const receipt = { outcome: "saved", code, durability, user_release_recorded: true, user_release_model_document_sha256: "c".repeat(64) };
      invokeMock.mockImplementation(async (command: string) => (command === "save_report_package" ? receipt : null));
      const saved = await invokeUserReleasedReportPackageSave(prepared, () => true);
      expect(saved).toEqual({ route: "tauri_report_package_save", receipt: { outcome: "saved", code, durability, user_release_recorded: true } });
      expect(saved.route === "tauri_report_package_save" && "user_release_model_document_sha256" in saved.receipt).toBe(false);
      expect(JSON.stringify(saved)).not.toContain("c".repeat(64));
    }
    const sent = invokeMock.mock.calls.filter(([command]) => command === "save_report_package");
    expect(sent).toHaveLength(1);
    expect(sent[0][1]).toEqual(prepared.args);
    invokeMock.mockClear();
    await expect(invokeUserReleasedReportPackageSave(prepared, () => false)).rejects.toThrow();
    expect(invokeMock.mock.calls.filter(([command]) => command === "save_report_package")).toHaveLength(0);
  });
});
