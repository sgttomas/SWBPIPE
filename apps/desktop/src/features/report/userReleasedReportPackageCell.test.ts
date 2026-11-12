// W-18b desktop slice (DEL-08-01 Design `report-user-release.md` §2.2, §2.3, §2.7, §11): the
// route-010 cell, the webview's early model check and the record's version. The cell admits a
// pressure-free preview-physics-2 result and the exact pressure routes (pressure-2,
// pressure-source-1) that native route 010 admits since D-6; it refuses what the package cell
// refuses and the identities route 010 does not admit. Unit transport replay of committed
// producer bytes; every model edit and record here is invented.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import {
  buildUserReleasedReportPackageRequest,
  REPORT_PACKAGE_ROUTE_NOT_ADMITTED,
  REPORT_USER_RELEASE_LIBRARY_REFERENCED,
  REPORT_USER_RELEASE_RULE_PACK_REFERENCED,
  reportPackageUnavailableReason,
  USER_RELEASED_ADMITTED_CONTRACT_IDS,
  userReleaseModelRefusal,
  userReleaseRecord,
  userReleasedAtOf,
  userReleasedReportPackageUnavailableReason
} from "./reportPackageRequest";

afterEach(() => {
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const root = resolve(__dirname, "../../../../../");
const raw = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as MechanicsResult;

async function pressureFreeSession() {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
  const result = await runPreviewMechanics(model, "sparse_interactive");
  const producer = result.producer!;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: producer.component_name, solver_version: producer.component_version,
      solver_build_ref: `${producer.component_name}@${producer.component_version}`, solver_mode: "sparse_interactive",
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true }
    },
    active_rule_packs: [],
    external_assets: []
  });
  return { model, result, inputManifest };
}

describe("the route-010 cell (§2.2, §11)", () => {
  it("admits the three identities native route 010 admits", async () => {
    expect([...USER_RELEASED_ADMITTED_CONTRACT_IDS]).toEqual([
      "openpipestress.result_semantics/0.3.0/preview-physics-2",
      "openpipestress.result_semantics/0.3.0/pressure-3",
      "openpipestress.result_semantics/0.3.0/pressure-source-1"
    ]);
    const { result } = await pressureFreeSession();
    expect(userReleasedReportPackageUnavailableReason(result)).toBeNull();
    expect(userReleasedReportPackageUnavailableReason(null)).toBeNull();
    for (const path of [
      "fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json",
      "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"
    ]) {
      expect(userReleasedReportPackageUnavailableReason(raw(path)), path).toBeNull();
    }
  });

  it("refuses the pressure-free Timoshenko identity, which the local-private package admits", async () => {
    const { result } = await pressureFreeSession();
    const timoshenko = structuredClone(result);
    timoshenko.producer!.semantic_contract_id = PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID;
    expect(reportPackageUnavailableReason(timoshenko)).toBeNull();
    expect(userReleasedReportPackageUnavailableReason(timoshenko)).toBe(
      `${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: a shareable report package carries pressure-free (preview-physics-2) results and eligible exact pressure route (pressure-2, pressure-source-1) results; this result is ${PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}. It can be saved in a local private package.`
    );
  });

  it("refuses every other identity by name", async () => {
    const { result } = await pressureFreeSession();
    const retained = structuredClone(result);
    retained.producer!.semantic_contract_id = "openpipestress.result_semantics/0.3.0/physics-retained-1";
    expect(userReleasedReportPackageUnavailableReason(retained)).toContain(
      `${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: a shareable report package carries`
    );
  });
});

describe("the early model check (§2.7)", () => {
  const clean = { project: { id: "project:invented", name: "Invented" }, nodes: [{ id: "node:1", provenance: "user entered" }], rule_pack_refs: [] };
  const variant = (edit: (model: Record<string, any>) => void) => { // eslint-disable-line @typescript-eslint/no-explicit-any
    const model = structuredClone(clean) as Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
    edit(model);
    return model;
  };

  it("admits a model with no library or rule-pack mark", () => {
    expect(userReleaseModelRefusal(clean)).toBeNull();
  });

  it.each([
    ["rule 1, an object library_ref", (m: any) => { m.nodes[0].provenance_record = { library_ref: { library_kind: "material" } }; }, "/nodes/0/provenance_record/library_ref"], // eslint-disable-line @typescript-eslint/no-explicit-any
    ["rule 1, a null library_ref", (m: any) => { m.nodes[0].library_ref = null; }, "/nodes/0/library_ref"], // eslint-disable-line @typescript-eslint/no-explicit-any
    ["rule 2, a library provenance string", (m: any) => { m.nodes[0].provenance = "library:material:invented#a"; }, "/nodes/0/provenance"], // eslint-disable-line @typescript-eslint/no-explicit-any
    ["rule 3 (a), legacy hanger evidence text", (m: any) => { m.nodes[0].provenance = "user_imported_hanger_selection"; }, "/nodes/0/provenance"], // eslint-disable-line @typescript-eslint/no-explicit-any
    ["rule 3 (b), embedded legacy evidence", (m: any) => { m.nodes[0].provenance = ' {"hanger_id":"h","record_snapshot":{}}'; }, "/nodes/0/provenance"] // eslint-disable-line @typescript-eslint/no-explicit-any
  ])("refuses %s", (_, edit, pointer) => {
    expect(userReleaseModelRefusal(variant(edit))).toBe(
      `${REPORT_USER_RELEASE_LIBRARY_REFERENCED}: the model carries a library reference at ${pointer}; a shareable package does not yet carry library record values.`
    );
  });

  it("takes rule 1 before rule 2", () => {
    const model = variant((m) => { m.nodes[0].provenance = "library:section:invented#s"; m.nodes.push({ id: "node:2", library_ref: {} }); });
    expect(userReleaseModelRefusal(model)).toContain("/nodes/1/library_ref");
  });

  it("refuses a rule-pack reference in the model or an active pack in the run's input manifest", () => {
    expect(userReleaseModelRefusal(variant((m) => { m.rule_pack_refs = [{ rule_pack_id: "invented" }]; }))).toBe(
      `${REPORT_USER_RELEASE_RULE_PACK_REFERENCED}: the model carries a rule-pack reference at /rule_pack_refs; a shareable package does not yet carry rule-pack checks.`
    );
    expect(userReleaseModelRefusal(clean, { manifest: { active_rule_packs: [{}] } } as never)).toBe(
      `${REPORT_USER_RELEASE_RULE_PACK_REFERENCED}: the run's input manifest records an active rule pack; a shareable package does not yet carry rule-pack checks.`
    );
  });

  it("refuses a library-referenced model in the builder before any assembly", async () => {
    const session = await pressureFreeSession();
    const model = structuredClone(session.model) as PreviewModel & { nodes: Array<Record<string, unknown>> };
    model.nodes[0].provenance = "library:material:invented#a";
    await expect(buildUserReleasedReportPackageRequest({
      ...session, model, analysisRun: null as never, userReleasedAt: "2026-10-27T00:00:00Z"
    })).rejects.toThrow(`${REPORT_USER_RELEASE_LIBRARY_REFERENCED}: the model carries a library reference at /nodes/0/provenance`);
  });
});

describe("the record (§2.3, §11)", () => {
  const document = { result_envelope: { envelope_id: "result-envelope:invented", reproducibility: { derivative_hash: { value: "d".repeat(64) } } } };
  const inputManifest = { manifest_ref: { object_type: "InputManifest", ref: "input-manifest:invented" }, manifest_sha256: "e".repeat(64) } as never;
  const model = { project: { id: "project:invented" } } as never;

  it("is record 1.0.0 for a pressure-free result and 1.1.0 for an exact result", async () => {
    const pressureFree = await userReleaseRecord({ model, result: { run_id: "run:a b", producer: { semantic_contract_id: USER_RELEASED_ADMITTED_CONTRACT_IDS[0] } } as never, inputManifest, document, userReleasedAt: "2026-10-27T00:00:00Z" }) as Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
    expect(pressureFree.record_version).toBe("1.0.0");
    expect(pressureFree.scope.not_carried).toEqual(["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"]);
    expect(pressureFree.scope.package_id).toBe("desktop-user-released-report-run-a-b");
    expect(pressureFree.bound_to.result_document_derivative_sha256).toBe("d".repeat(64));
    for (const id of USER_RELEASED_ADMITTED_CONTRACT_IDS.slice(1)) {
      const exact = await userReleaseRecord({ model, result: { run_id: "run:x", producer: { semantic_contract_id: id } } as never, inputManifest, document, userReleasedAt: "2026-10-27T00:00:00Z" }) as Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
      expect(exact.record_version).toBe("1.1.0");
      expect(exact.scope.not_carried.at(-1)).toBe("ineligible_exact_route_results");
    }
  });

  it("states the confirmation time in whole UTC seconds", () => {
    expect(userReleasedAtOf(new Date("2026-11-09T01:02:03.999Z"))).toBe("2026-11-09T01:02:03Z");
  });
});
