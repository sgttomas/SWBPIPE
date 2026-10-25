// The split report-package save (DEL-08-01 Design `report-package-current-results.md`
// §2.5, §2.6): the preparation's evidence checks and canonical texts, then the
// invocation's native arguments and its `stillCurrent` guard before every send.
import { createHash } from "node:crypto";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ControlledRouteExport } from "../features/redaction-controls/redactionExportControls";
import { invokeReportPackageSave, prepareReportPackageSave } from "./reportPackageSaveService";
import { IngressRequestRetiredError } from "./nativeIngress";
import { canonicalJsonString } from "./hashService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

const REQUEST = { package_id: "invented", report: { z: 1, a: [2, "Ω"] } };
const MODEL = { project: { id: "project:invented", name: "Invented" }, nodes: [{ id: "node:A", x: 1.5 }] };
const INPUT_MANIFEST = { document_kind: "openpipestress.current_session_input_manifest", active_rule_packs: [], model_basis: { model_ref: "project:invented" } };

function controlledRequest(payload: unknown = structuredClone(REQUEST)): ControlledRouteExport {
  return {
    payload,
    decisions: [],
    findings: [],
    blocked: false,
    summary: {
      route_id: "DREP-PACKAGE-SAVE-009",
      decision_count: 3,
      finding_count: 1,
      redacted_count: 0,
      omitted_count: 0,
      warning_count: 1,
      blocking_count: 0,
      cloud_transmission_attempted: false as const,
      professional_claims_made: false as const,
      materialization_withheld: false,
      local_first: {
        route_id: "DREP-PACKAGE-SAVE-009",
        export_context: "local_private",
        storage_context: "local_private",
        action: "include_metadata_only" as const,
        reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED" as const,
        blocked: false,
        metadata_only: true as const,
        explicit_local_private_intent: true
      }
    }
  };
}

const receipt = {
  outcome: "cancelled",
  stage: "picker",
  code: "REPORT-PACKAGE-SAVE-CANCELLED",
  container_file_name: "invented.opsproj",
  byte_count: 0,
  package_identity_sha256_hex: "a".repeat(64),
  container_sha256_hex: "b".repeat(64),
  members: [],
  replaced_existing: false,
  durability: "not_applicable",
  redaction_route_id: "DREP-PACKAGE-SAVE-009",
  redaction_decision_count: 3,
  redaction_finding_count: 1,
  redaction_blocking_count: 0,
  local_first_route_id: "DREP-PACKAGE-SAVE-009",
  local_first_export_context: "local_private",
  local_first_storage_context: "local_private",
  local_first_action: "include_metadata_only",
  local_first_reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED",
  local_first_blocked: false,
  local_first_metadata_only: true,
  local_first_explicit_local_private_intent: true,
  selected_basename: "",
  path_containment: "best_effort_non_adversarial",
  limitation: "Invented TOCTOU limitation",
  warnings: []
};

const desktop = () => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; };

async function ready() {
  desktop();
  const prepared = await prepareReportPackageSave(controlledRequest(), MODEL, REQUEST, INPUT_MANIFEST);
  if (prepared.route !== "ready") throw new Error(`not ready: ${JSON.stringify(prepared)}`);
  return prepared;
}

beforeEach(() => invokeMock.mockReset());
afterEach(() => {
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  vi.useRealTimers();
});

describe("prepareReportPackageSave", () => {
  it("reports honest desktop-only browser routing without invoke", async () => {
    await expect(prepareReportPackageSave(controlledRequest(), MODEL, REQUEST, INPUT_MANIFEST)).resolves.toEqual({
      route: "unavailable_browser",
      diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY"
    });
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses raw, blocked and withheld requests", async () => {
    desktop();
    expect((await prepareReportPackageSave({ package_id: "raw" }, MODEL, REQUEST, INPUT_MANIFEST)).route).toBe("redaction_blocked");
    const blocked = controlledRequest();
    blocked.blocked = true;
    blocked.payload = null as never;
    expect((await prepareReportPackageSave(blocked, MODEL, REQUEST, INPUT_MANIFEST)).route).toBe("redaction_blocked");
    const withheld = controlledRequest();
    withheld.summary.materialization_withheld = true;
    expect((await prepareReportPackageSave(withheld, MODEL, REQUEST, INPUT_MANIFEST)).route).toBe("redaction_blocked");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses missing or blocked local-first evidence", async () => {
    desktop();
    const missing = controlledRequest();
    delete missing.summary.local_first;
    await expect(prepareReportPackageSave(missing, MODEL, REQUEST, INPUT_MANIFEST)).resolves.toEqual({
      route: "redaction_blocked",
      diagnostic: "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-REQUIRED: save requires allowed metadata-only local-first route evidence."
    });
    const blocked = controlledRequest();
    blocked.summary.local_first!.blocked = true;
    blocked.summary.local_first!.action = "block_storage";
    blocked.summary.local_first!.reason_code = "LOCAL_PRIVATE_INTENT_REQUIRED";
    expect((await prepareReportPackageSave(blocked, MODEL, REQUEST, INPUT_MANIFEST)).route).toBe("redaction_blocked");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses a controlled payload that is not canonically the screened request", async () => {
    desktop();
    const changed = await prepareReportPackageSave(controlledRequest({ ...REQUEST, package_id: "other" }), MODEL, REQUEST, INPUT_MANIFEST);
    expect(changed).toEqual({
      route: "redaction_blocked",
      diagnostic: "REPORT-PACKAGE-REDACTION-BLOCKED: the route control changed the request (CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED)."
    });
  });

  it("binds the screening evidence to the request text's SHA-256 and sends the three canonical texts", async () => {
    const prepared = await ready();
    const requestText = await canonicalJsonString(REQUEST);
    expect(prepared.args.requestCanonicalText).toBe(requestText);
    expect(prepared.args.modelCanonicalText).toBe(await canonicalJsonString(MODEL));
    // The run's input manifest, for the native rule-pack re-check (§2.6 item 3).
    expect(prepared.args.inputManifestCanonicalText).toBe(await canonicalJsonString(INPUT_MANIFEST));
    const independent = `sha256:${createHash("sha256").update(requestText, "utf8").digest("hex")}`;
    expect(prepared.requestSha256).toBe(independent);
    expect(prepared.args.screening).toEqual({
      route_id: "DREP-PACKAGE-SAVE-009",
      export_context: "local_private",
      explicit_local_private_intent: true,
      blocked: false,
      materialization_withheld: false,
      lossless_required: true,
      exact_payload_match: true,
      blocking_count: 0,
      decision_count: 3,
      finding_count: 1,
      payload_sha256: independent
    });
    expect(prepared.args.localFirstEvidence).toEqual(controlledRequest().summary.local_first);
  });
});

describe("invokeReportPackageSave", () => {
  it("sends exactly the native arguments and preserves the receipt", async () => {
    const prepared = await ready();
    invokeMock.mockResolvedValue(receipt);
    const route = await invokeReportPackageSave(prepared, () => true);
    expect(invokeMock).toHaveBeenCalledOnce();
    expect(invokeMock).toHaveBeenCalledWith("save_report_package", {
      requestCanonicalText: prepared.args.requestCanonicalText,
      modelCanonicalText: prepared.args.modelCanonicalText,
      inputManifestCanonicalText: prepared.args.inputManifestCanonicalText,
      screening: prepared.args.screening,
      localFirstEvidence: prepared.args.localFirstEvidence
    });
    expect(Object.keys(invokeMock.mock.calls[0][1]).sort()).toEqual(["inputManifestCanonicalText", "localFirstEvidence", "modelCanonicalText", "requestCanonicalText", "screening"]);
    expect(route).toEqual({ route: "tauri_report_package_save", receipt });
  });

  it("sends nothing for a retired basis", async () => {
    const prepared = await ready();
    invokeMock.mockResolvedValue(receipt);
    await expect(invokeReportPackageSave(prepared, () => false)).rejects.toBeInstanceOf(IngressRequestRetiredError);
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("evaluates the guard before a headroom re-send and sends nothing once retired", async () => {
    const prepared = await ready();
    vi.useFakeTimers();
    let current = true;
    const guard = vi.fn(() => current);
    invokeMock
      .mockRejectedValueOnce({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 2, available_bytes: 1 }, pool: null })
      .mockResolvedValue(receipt);
    const pending = invokeReportPackageSave(prepared, guard).catch((error: unknown) => error);
    await vi.advanceTimersByTimeAsync(0);
    expect(invokeMock).toHaveBeenCalledOnce();
    expect(guard).toHaveBeenCalledTimes(1);
    current = false;
    await vi.runAllTimersAsync();
    expect(await pending).toBeInstanceOf(IngressRequestRetiredError);
    expect(guard).toHaveBeenCalledTimes(2);
    expect(invokeMock).toHaveBeenCalledOnce();
  });
});
