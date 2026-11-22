// DEL-08-01 REQ-024, "Package and user release": the pressure-free Timoshenko kind's shareable
// (user-released, record 1.3.0) and local-private package requests, as the desktop's builders
// assemble them, held equal to the native derivation. This test writes
// `fixtures/reports/invented/user_release_pf_timoshenko/<shape>.json` and fails when a vector
// is stale; `OPENPIPESTRESS_WRITE_PF_TIMOSHENKO_VECTORS=<dir>` writes them there instead. The
// native side derives each user-released request again from the vector's M, S, m, R, H and U
// and requires canonical equality, packages it and reads it back
// (`core/reporting/report_package/tests/user_release_package.rs`).
//
// Each request carries the standing record the reader derives and the identity's limitation 0
// entry, and neither carries the rendered report's `pf_timoshenko` section (a package binds no
// custody) or the delivered texts. The results are the product's own committed Timoshenko
// envelopes (`core/product_physics/tests/pf_timoshenko_envelopes.rs` keeps them current): two
// eligible ones for their frozen requests, and the sensitive torsion one, which is solved and
// readable and not eligible; each is solved through the unit solve-job transport, and the model
// adds only the display members the desktop shell needs (a project name, each load case's label
// where it has none and `status: "preview_only"`, a component list where it has none) and, for a
// model document, the Timoshenko shear model it was solved with. Unit
// transport replay of committed producer bytes; NOT a native UI qualification witness. The
// record is invented; nothing here records a user release or implies acceptance, issuance or
// lifecycle standing.
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { canonicalSha256Hex } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import desktopAppConfig from "../../../src-tauri/tauri.conf.json";
import type { PreviewModel } from "../../types";
import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import { admitExactReportSource } from "./exactReportAdmission";
import { PF_TIMOSHENKO_SECTION } from "./pfTimoshenkoReport";
import {
  assembleUserReleasedReportPackageRequest,
  buildReportPackageRequest,
  reportPackageExactSubmission,
  userReleaseRecord,
  USER_RELEASE_PF_TIMOSHENKO_RECORD_VERSION
} from "./reportPackageRequest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = path.resolve(__dirname, "../../../../..");
const load = (p: string) => JSON.parse(readFileSync(path.join(root, p), "utf8"));
const WRITE_ENV = "OPENPIPESTRESS_WRITE_PF_TIMOSHENKO_VECTORS";
const DIR = "fixtures/reports/invented/user_release_pf_timoshenko";
/** An invented confirmation time (the record's one caller-supplied value). */
const INVENTED_USER_RELEASED_AT = "2026-11-21T00:00:00Z";
const SOFTWARE = { name: desktopAppConfig.productName, version: desktopAppConfig.version };

afterEach(() => { invokeMock.mockReset(); resetSessionResultCustodyForTests(); delete (window as Json).__TAURI_INTERNALS__; });

/** Each vector: the producer's bytes, the frozen request or model document whose model they were
 * solved for (with `shearModel` selected where given), and each load case's health (null: every
 * case checks passed). */
const SHAPES: { shape: string; raw: string; request: string; shearModel?: string; reason: string; health: string[] | null }[] = [
  {
    shape: "straight_sparse",
    raw: "fixtures/results/preview_physics_timoshenko_straight_sparse.json",
    request: "validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json",
    reason: "the product's eligible Timoshenko result with straight members only",
    health: null
  },
  {
    shape: "arc_sparse",
    raw: "fixtures/results/preview_physics_timoshenko_arc_sparse.json",
    request: "validation/references/pf_timoshenko/requests/PFT-ARC-TIP-TIMOSHENKO.request.json",
    reason: "the product's eligible Timoshenko result with a realized arc (PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI)",
    health: null
  },
  {
    shape: "sensitive_torsion",
    raw: "fixtures/results/preview_physics_timoshenko_sensitive_torsion_sparse.json",
    request: "fixtures/product_preview/numerical_sensitive_torsion_model.json",
    shearModel: "timoshenko_annulus_energy_v1",
    reason: "the product's solved, readable Timoshenko result whose standing is not eligible: its one load case sensitive",
    health: ["sensitive"]
  }
];

for (const { shape, raw, request: requestPath, shearModel, reason, health } of SHAPES) {
  it(`the ${shape} Timoshenko result's shareable and local-private requests carry its standing record`, async () => {
    const document = load(requestPath);
    const model = structuredClone(document.model ?? document) as Json;
    if (shearModel) model.model_settings = { ...(model.model_settings ?? {}), shear_model: shearModel };
    model.project.name = model.project.id;
    // The report's load case summary states each case's label, which the native report reads.
    for (const loadCase of model.load_cases) Object.assign(loadCase, { label: loadCase.label ?? loadCase.id, status: "preview_only" });
    if (model.components === undefined) model.components = [];
    const bytes = readFileSync(path.join(root, raw), "utf8");
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createSolveJobTransport(() => bytes, { jobIdPrefix: `invented-pf-timoshenko-release-${shape}` }).invoke);
    const result = await runPreviewMechanics(model as PreviewModel);
    expect(result.producer?.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    const producer = result.producer!;
    const inputManifest = await buildCurrentSessionInputManifest({ model, solver: {
      solver_name: producer.component_name, solver_version: producer.component_version,
      solver_build_ref: `${producer.component_name}@${producer.component_version}`, solver_mode: "sparse_interactive",
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true }
    }, active_rule_packs: [], external_assets: [] });
    const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
    const admission = await admitExactReportSource({ model, result, analysisRun, inputManifest, purpose: "report" });
    const healths = (admission.numericalStanding as Json).cases.map((c: Json) => c.health);
    expect(healths).toEqual(health ?? healths.map(() => "checks_passed"));
    const userRelease = await userReleaseRecord({ model, result, inputManifest, document: admission.document, userReleasedAt: INVENTED_USER_RELEASED_AT });
    expect(userRelease.record_version).toBe(USER_RELEASE_PF_TIMOSHENKO_RECORD_VERSION);
    const request = await assembleUserReleasedReportPackageRequest({ model, result, analysisRun, inputManifest, userRelease });
    const localPrivate = await buildReportPackageRequest({ model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null });
    for (const built of [request, localPrivate]) {
      const report = built.report as Json;
      expect(report.numerical_standing).toEqual(admission.numericalStanding);
      expect(PF_TIMOSHENKO_SECTION in report).toBe(false);
      expect("exact_route" in report).toBe(false);
      expect(report.report_sections.limitations.map((item: Json) => item.limitation_id)).toEqual([`limitation:${result.run_id}:technical-preview`, `limitation:${result.run_id}:pf-timoshenko:1`]);
      // A Timoshenko request resubmits no delivered texts: the native save derives from its own source.
      expect(reportPackageExactSubmission(built)).toBeNull();
      expect(built.source_results).toEqual([result]);
    }
    expect((request.report as Json).report_context.user_release).toEqual(userRelease);
    const vector = {
      shape, reason, raw_path: raw, model, source: result, solver_mode: "sparse_interactive", software: SOFTWARE,
      run_record: analysisRun, input_manifest: inputManifest.manifest, user_release: userRelease,
      request, request_sha256: await canonicalSha256Hex(request),
      local_private_request: localPrivate, local_private_request_sha256: await canonicalSha256Hex(localPrivate)
    };
    const content = `${JSON.stringify(vector)}\n`;
    const target = process.env[WRITE_ENV];
    if (target) writeFileSync(path.join(target, `${shape}.json`), content);
    else expect(readFileSync(path.join(root, `${DIR}/${shape}.json`), "utf8") === content, `${DIR}/${shape}.json is stale; regenerate with ${WRITE_ENV}=<dir>`).toBe(true);
  });
}
