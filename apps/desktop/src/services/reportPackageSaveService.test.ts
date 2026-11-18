// The split report-package save (DEL-08-01 Design `report-package-current-results.md`
// §2.5, §2.6): the preparation's evidence checks and canonical texts, then the
// invocation's native arguments and its `stillCurrent` guard before every send.
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ControlledRouteExport } from "../features/redaction-controls/redactionExportControls";
import { invokeReportPackageSave, prepareReportPackageSave } from "./reportPackageSaveService";
import { IngressRequestRetiredError } from "./nativeIngress";
import * as hashService from "./hashService";
import { canonicalJsonString } from "./hashService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// The engine's canonical functions, counted (each call goes through to the engine).
vi.mock("./hashService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./hashService")>();
  return { ...actual, canonicalJsonString: vi.fn(actual.canonicalJsonString), canonicalSha256Hex: vi.fn(actual.canonicalSha256Hex) };
});

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
    // The run's input manifest, for the native rule-pack re-check.
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

/** The preparation's canonical texts and hash as computed before the payload was
 * canonicalised once: five engine calls, the payload's hash by its own canonicalisation. */
async function referencePreparation(controlled: ControlledRouteExport, model: unknown, request: unknown, inputManifest: unknown) {
  const actual = await vi.importActual<typeof import("./hashService")>("./hashService");
  const [requestCanonicalText, screenedRequestText, hex, modelCanonicalText, inputManifestCanonicalText] = await Promise.all([
    actual.canonicalJsonString(controlled.payload),
    actual.canonicalJsonString(request),
    actual.canonicalSha256Hex(controlled.payload),
    actual.canonicalJsonString(model),
    actual.canonicalJsonString(inputManifest)
  ]);
  return { lossless: requestCanonicalText === screenedRequestText, requestCanonicalText, requestSha256: `sha256:${hex}`, modelCanonicalText, inputManifestCanonicalText };
}

const fixture = (file: string) => JSON.parse(readFileSync(resolve(__dirname, "../../../../", file), "utf8")) as Record<string, unknown>;
/** Eight payloads: the invented request; an invented request of escaped texts, ordered keys and
 * numbers; the committed exact package request; the committed user-release request; and the four
 * derivation vectors' run records, used as payloads. */
function requestCorpus(): Array<[string, unknown, unknown, unknown]> {
  const exact = fixture("fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json");
  const released = fixture("fixtures/reports/invented/user_release_exact/pressure_source_1.json");
  const vectors = fixture("fixtures/reports/invented/user_release_derivation_vectors.json").vectors as Array<Record<string, unknown>>;
  return [
    ["invented", REQUEST, MODEL, INPUT_MANIFEST],
    ["texts and numbers", {
      z: ["\u2028\u2029", "</script>", "\u0000\u001f\u007f", "\ud83d\ude00 astral", "e\u0301 é", "\"\\/", "\ufeff"],
      "é": 1, "\u{1f600}": 2, "\u00e9a": 3, "10": 4, "2": 5, a: { b: [1e21, 1e-7, -0, 0.1, 5e-324, 2 ** 53 + 2, -1.5e300, 123456789012] }, A: null, "": true
    }, MODEL, INPUT_MANIFEST],
    ["exact package request", exact.request, exact.model, exact.input_manifest],
    ["user release request", released.request, released.model, released.input_manifest],
    ...vectors.map((vector) => [`run record ${vector.name}`, vector.run_record, MODEL, vector.input_manifest] as [string, unknown, unknown, unknown])
  ];
}

describe("prepareReportPackageSave canonicalises the payload once", () => {
  it("prepares the same texts, hash and screening as the five-call preparation, on each of eight payloads", async () => {
    desktop();
    for (const [name, request, model, inputManifest] of requestCorpus()) {
      const controlled = controlledRequest(structuredClone(request));
      const reference = await referencePreparation(controlled, model, request, inputManifest);
      expect(reference.lossless, name).toBe(true);
      const prepared = await prepareReportPackageSave(controlled, model, request, inputManifest, { exactResultText: "invented result", exactIntakeText: "invented intake" });
      if (prepared.route !== "ready") throw new Error(`${name}: not ready`);
      expect(prepared.requestSha256, name).toBe(reference.requestSha256);
      expect(prepared.args.requestCanonicalText === reference.requestCanonicalText, name).toBe(true);
      expect(prepared.args.modelCanonicalText === reference.modelCanonicalText, name).toBe(true);
      expect(prepared.args.inputManifestCanonicalText === reference.inputManifestCanonicalText, name).toBe(true);
      expect(prepared.args.screening.payload_sha256, name).toBe(reference.requestSha256);
      expect(prepared.args, name).toEqual({
        requestCanonicalText: reference.requestCanonicalText,
        modelCanonicalText: reference.modelCanonicalText,
        inputManifestCanonicalText: reference.inputManifestCanonicalText,
        screening: { ...prepared.args.screening, payload_sha256: reference.requestSha256 },
        localFirstEvidence: controlled.summary.local_first,
        exactResultText: "invented result",
        exactIntakeText: "invented intake"
      });
    }
  });

  it("makes four canonical texts and no second canonicalisation of the payload for its hash", async () => {
    desktop();
    vi.mocked(hashService.canonicalJsonString).mockClear();
    vi.mocked(hashService.canonicalSha256Hex).mockClear();
    const controlled = controlledRequest();
    const prepared = await prepareReportPackageSave(controlled, MODEL, REQUEST, INPUT_MANIFEST);
    expect(prepared.route).toBe("ready");
    expect(hashService.canonicalSha256Hex).not.toHaveBeenCalled();
    expect(vi.mocked(hashService.canonicalJsonString).mock.calls.map(([value]) => value)).toEqual([controlled.payload, REQUEST, MODEL, INPUT_MANIFEST]);
  });

  it("admits or refuses seven payload variants as canonical-text equality does", async () => {
    desktop();
    const cases: Array<[string, unknown, boolean]> = [
      ["member order only", { report: { a: [2, "Ω"], z: 1 }, package_id: "invented" }, true],
      ["a number's spelling only", { package_id: "invented", report: { z: 1.0, a: [2e0, "Ω"] } }, true],
      ["negative zero for zero", { package_id: "invented", report: { z: 1, a: [2, "Ω"], zero: -0 } }, true],
      ["one value", { package_id: "invented", report: { z: 2, a: [2, "Ω"] } }, false],
      ["an extra member", { package_id: "invented", report: { z: 1, a: [2, "Ω"], y: null } }, false],
      ["a missing member", { package_id: "invented", report: { z: 1 } }, false],
      ["array order", { package_id: "invented", report: { z: 1, a: ["Ω", 2] } }, false]
    ];
    for (const [name, payload, equal] of cases) {
      const screened = name === "negative zero for zero" ? { ...REQUEST, report: { ...REQUEST.report, zero: 0 } } : REQUEST;
      const reference = await referencePreparation(controlledRequest(payload), MODEL, screened, INPUT_MANIFEST);
      expect(reference.lossless, name).toBe(equal);
      const prepared = await prepareReportPackageSave(controlledRequest(payload), MODEL, screened, INPUT_MANIFEST);
      expect(prepared.route, name).toBe(equal ? "ready" : "redaction_blocked");
      if (prepared.route === "ready") expect(prepared.requestSha256, name).toBe(reference.requestSha256);
      else expect(prepared.diagnostic, name).toBe("REPORT-PACKAGE-REDACTION-BLOCKED: the route control changed the request (CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED).");
    }
  });
});
