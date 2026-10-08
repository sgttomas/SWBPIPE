/** U6d (D-U6-8; D2 4.9.6; plan 1d and 4): every desktop output of the F2a
 * preview successor is refused until T6. Each refuses through the one shared
 * function (`loadReferenceOutputRefusal` / `refuseLoadReferenceOutput`), which
 * returns the successor's own reason, or through the report package's
 * fresh-result refusal. `isLoadReferenceRoute` is unchanged, so the
 * load-reference text is never shown for a successor.
 * T6S-3 (RR decisions 2 and 12): the Result Export and Stress-Neutral Export
 * panels now read their own entries in the output policy (`outputPolicy.ts`),
 * which admit a successor only at numerically eligible standing with the live
 * native capture; the eighteen other surfaces and the report package keep the
 * shared refusal, whose text is reworded. The successors here are unregistered
 * file bytes, so the two panels refuse them with the standing's own reason
 * (their admission is tested in retainedPrecisionStressNeutral.test.tsx and
 * retainedPrecisionResultExport.test.tsx).
 * Display stays. A preview-physics-1 result is the control: its outputs are
 * unchanged. Inputs are PP's pinned milestone successor bytes (D-U6-5); unit
 * rendering only, NOT a native witness (plan F-1). */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { ComponentType } from "react";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { computeModelHash } from "../../services/hashService";
import { hasNativeMechanicsInvocation, runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { initialSolveJob } from "../workspace/solveJobAudit";
import {
  LOAD_REFERENCE_OUTPUT_REFUSAL, RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE, RETAINED_PRECISION_OUTPUT_REFUSAL, RETAINED_PHYSICS_OUTPUT_REFUSAL,
  isLoadReferenceRoute, loadReferenceOutputRefusal, refuseLoadReferenceOutput,
} from "./loadReferenceOutputAvailability";
import { N_REPORT, REPORT_PACKAGE_FRESH_RESULT_UNAVAILABLE, isFreshSemanticResult } from "./knownSemanticLimitations";
import { currentSemanticContract, numericalResultStanding } from "./numericalResultQuality";
import { RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED, RETAINED_PRECISION_VALIDATION_REQUIRED } from "./retainedPrecisionStanding";
import { N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE } from "./outputPolicy";
import { KnownSemanticNotices } from "./KnownSemanticNotices";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket, StressNeutralExportPanel } from "../stress-neutral/StressNeutralExportPanel";
import { buildCurrentResultExport, currentReceivedOrigin, currentResultDocumentBase, deriveResultDocument, resultDigest, validateResultDocument } from "../result-export/resultExportAdapter";
import { ResultExportPanel } from "../result-export/ResultExportPanel";
import { PcfExportPanel } from "../pcf-export/PcfExportPanel";
import { CaepipeMbfExportPanel } from "../caepipe-mbf/CaepipeMbfExportPanel";
import { CaepipeExternalHarnessPanel } from "../caepipe-external/CaepipeExternalHarnessPanel";
import { ExportAdapterSdkPanel } from "../export-adapter-sdk/ExportAdapterSdkPanel";
import { AdapterFrameworkPanel } from "../adapter-framework/AdapterFrameworkPanel";
import { ExternalProverBoundaryPanel } from "../external-prover/ExternalProverBoundaryPanel";
import { MissingDataBlockingPanel } from "../missing-data/MissingDataBlockingPanel";
import { DesignWorkspacePanel } from "../design-workspace/DesignWorkspacePanel";
import { RuleCheckPanel } from "../rule-check/RuleCheckPanel";
import { ReportLintPanel } from "../report-lint/ReportLintPanel";
import { SolvePanel } from "../solve/SolvePanel";
import { HeadlessRunnerPanel } from "../headless-runner/HeadlessRunnerPanel";
import { LocalFeaHandoffPanel } from "../local-fea-handoff/LocalFeaHandoffPanel";
import { NativePackagePanel } from "../native-package/NativePackagePanel";
import { HandoffPanel } from "../handoff/HandoffPanel";
import { ExportReviewPanel } from "../export-review/ExportReviewPanel";
import { ReportPanel } from "../report/ReportPanel";
import { RenderedReportPanel } from "../report/RenderedReportPanel";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const caseFile = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_carrier_cases.json"), "utf8"));
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
const manifestFor = (source: MechanicsResult) => ({ manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-retained-output-refusal" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:invented" } } });
async function scenario(result: MechanicsResult, model: PreviewModel) {
  const analysisRun = await buildAnalysisRunV03(result, manifestFor(result), undefined, modelLoadBasisRefs(model));
  const props = {
    model, result, analysisRun, knowledge: null, comparison: null, editorIntents: [], proposal: null, selectedReviewTarget: null,
    projectOperation: "", projectSummary: null, storageCapability: null, modelHash: await computeModelHash(model),
    solveJob: initialSolveJob(), running: false, solverMode: "sparse_interactive", onCancel: () => {}, onRun: () => {}, onSolverModeChange: () => {},
  };
  return { result, model, analysisRun, props };
}
function milestone(mode: PreviewSolverMode) {
  const entry = caseFile.fixtures[`milestone_${mode}`];
  const bytes = readFileSync(resolve(root, entry.path));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(entry.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as MechanicsResult, model: doc.invocation.request.model as PreviewModel };
}
/** The pinned request model plus the desktop fields the panels read (invented
 * empty or label values only; no physics is read from them). */
function desktopModel(model: PreviewModel): PreviewModel {
  const desktop = structuredClone(model) as PreviewModel & Record<string, unknown>;
  for (const loadCase of desktop.load_cases) Object.assign(loadCase, { status: (loadCase as Json).status ?? "invented_test_status" });
  desktop.components ??= [];
  desktop.data_boundary ??= { source: "invented" };
  desktop.diagnostics ??= [];
  return desktop;
}
const successor = (mode: PreviewSolverMode) => { const { source, model } = milestone(mode); return scenario(source, desktopModel(model)); };
const preview = () => scenario(JSON.parse(readFileSync(resolve(root, "fixtures/results/preview_physics_connected_sparse.json"), "utf8")), JSON.parse(readFileSync(resolve(root, "fixtures/model_operations/precision_connected_ui_model.json"), "utf8")));

/** Surfaces whose downloads or handoffs carry result data (T1's group a). */
const GATED: [string, ComponentType<Json>, string][] = [
  ["PCF export", PcfExportPanel, "pcf-export"],
  ["CAEPIPE .mbf export", CaepipeMbfExportPanel, "caepipe-mbf"],
  ["CAEPIPE external harness", CaepipeExternalHarnessPanel, "caepipe-external"],
  ["Export adapter SDK", ExportAdapterSdkPanel, "export-adapter-sdk"],
  ["Adapter framework", AdapterFrameworkPanel, "adapter-framework"],
  ["External prover boundary", ExternalProverBoundaryPanel, "external-prover"],
  ["Missing-data blocking", MissingDataBlockingPanel, "missing-data"],
  ["Design workspace", DesignWorkspacePanel, "design-workspace"],
  ["Rule-check completeness", RuleCheckPanel, "rule-check"],
  ["Report lint", ReportLintPanel, "report-lint"],
  ["Solve job", SolvePanel, "solve-job"],
  ["Headless runner", HeadlessRunnerPanel, "headless-runner"],
  ["Local FEA handoff", LocalFeaHandoffPanel, "local-fea"],
  ["Native package", NativePackagePanel, "native-package"],
  ["Handoff package", HandoffPanel, "handoff"],
  ["Export safety review", ExportReviewPanel, "export-review"],
  ["Report packet", ReportPanel, "report"],
  ["Rendered report", RenderedReportPanel, "rendered-report"],
];

describe("the shared refusal", () => {
  it("names the desktop route only, routes it to T6, never calls the result invalid or unsupported, and is distinct from load-reference", async () => {
    expect(RETAINED_PRECISION_OUTPUT_REFUSAL.startsWith(`${RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE}: `)).toBe(true);
    expect(RETAINED_PRECISION_OUTPUT_REFUSAL).toContain("not yet available on the desktop");
    expect(RETAINED_PRECISION_OUTPUT_REFUSAL).toContain("routed to T6");
    expect(RETAINED_PRECISION_OUTPUT_REFUSAL.toLowerCase()).not.toMatch(/invalid|unsupported/);
    expect(RETAINED_PRECISION_OUTPUT_REFUSAL).not.toBe(LOAD_REFERENCE_OUTPUT_REFUSAL);
    // B3b (B3D-14): the exact successor's own reason, re-exported beside the preview one, which is untouched.
    expect(RETAINED_PHYSICS_OUTPUT_REFUSAL).toBe(`${RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE}: This output of retained-precision results (physics-retained-1) is not yet available on the desktop; only the result JSON and stress-neutral exports admit a numerically eligible result. It is routed to T6. The result remains readable here; this is not a finding about the result.`);
    expect(RETAINED_PRECISION_OUTPUT_REFUSAL).toContain("(preview-physics-retained-1)");
    for (const mode of MODES) {
      const { result } = await successor(mode);
      expect(isLoadReferenceRoute(result)).toBe(false);
      expect(loadReferenceOutputRefusal(result)).toBe(RETAINED_PRECISION_OUTPUT_REFUSAL);
      expect(() => refuseLoadReferenceOutput(result)).toThrow(RETAINED_PRECISION_OUTPUT_REFUSAL);
    }
    const control = (await preview()).result;
    expect(loadReferenceOutputRefusal(control)).toBeNull();
    expect(() => refuseLoadReferenceOutput(control)).not.toThrow();
    expect(loadReferenceOutputRefusal(null)).toBeNull();
    // A relabelled successor is unsupported, not a successor route.
    const relabelled = structuredClone((await successor("sparse_interactive")).result) as Json;
    relabelled.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1";
    relabelled.formulation_basis.profile_id = "product_preview_mechanics_v1";
    expect(loadReferenceOutputRefusal(relabelled)).toBeNull();
  });
});

describe.each(GATED)("%s", (_label, Panel, prefix) => {
  it.each(MODES)("refuses %s successor output with its own reason and keeps its display", async (mode) => {
    const { props } = await successor(mode);
    render(<Panel {...props} />);
    expect(screen.getByTestId(`${prefix}-load-reference-output-unavailable`).textContent).toBe(RETAINED_PRECISION_OUTPUT_REFUSAL);
    expect(screen.queryByText(LOAD_REFERENCE_OUTPUT_REFUSAL)).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
  it("leaves a preview-physics-1 result's outputs unchanged", async () => {
    const { props } = await preview();
    render(<Panel {...props} />);
    expect(screen.queryByTestId(`${prefix}-load-reference-output-unavailable`)).toBeNull();
  });
});

describe.each(MODES)("%s: stress-neutral, result export and the report package keep their own refusal points", (mode) => {
  it("stress-neutral packet build, packet validation and panel all refuse", async () => {
    const { model, result, analysisRun } = await successor(mode);
    // T6S-3: the panel's builder admits a successor only at eligible standing; these
    // unregistered bytes refuse with the standing's own reason.
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow(`${RETAINED_PRECISION_VALIDATION_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`);
    await expect(validateStressNeutralExportPacket({ schema_version: "0.3.0" }, result)).rejects.toThrow("SN-PRECISION-SOURCE-METADATA-MISMATCH");
    // T6S-5 (I67's F4): the packet header now carries the receipt, so a header-only
    // packet dispatches to the successor route; it then needs the UTF-8 CSV profile,
    // passes the reader's transport checks, and stops at its absent annotations.
    const header = { schema_version: "0.3.0", producer: result.producer, numerical_quality: result.numerical_quality, formulation_basis: result.formulation_basis, contract_evidence: result.contract_evidence, retained_precision: result.retained_precision };
    await expect(validateStressNeutralExportPacket({ ...header, export_profile: {} })).rejects.toThrow("SN-CSV-ENCODING-PROFILE-MISMATCH");
    await expect(validateStressNeutralExportPacket({ ...header, export_profile: { csv_encoding: "utf-8", csv_row_order: "unicode_scalar_value_result_id" } })).rejects.toThrow("SN-SOURCE-ANNOTATION-COVERAGE");
    render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />);
    // U7 slice T (RV91 N-5), then T6S-3: the panel shows its policy refusal for a
    // successor it does not admit (here, the standing's reason); the load/reference
    // text is never shown.
    expect(screen.getByTestId("stress-neutral-load-reference-output-unavailable").textContent).toBe(`${RETAINED_PRECISION_VALIDATION_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`);
    expect(screen.queryByTestId("stress-neutral-empty")).toBeNull();
    expect(screen.queryByText(LOAD_REFERENCE_OUTPUT_REFUSAL)).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
  it("result export: the Current build and the panel refuse; the pure projection and its validator read no standing", async () => {
    const { model, result, analysisRun } = await successor(mode);
    // T6S-3/T6S-4: the Current builder and the panel admit a successor only at eligible
    // standing; these unregistered bytes refuse with the standing's own reason.
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest: null })).rejects.toThrow(`${RETAINED_PRECISION_VALIDATION_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`);
    // As in Rust, the pure projection and its validator take no standing: a document
    // without the successor's receipt does not bind it.
    await expect(validateResultDocument({ schema_version: "0.3.0", result_envelope: { schema_version: "0.3.0" } }, result)).rejects.toThrow("RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH");
    // A hash-consistent origin and the desktop-shaped base pass every gate, and the
    // derivative carries the receipt whole (retainedPrecisionResultExport.test.tsx pins its bytes).
    expect(model.project.id).toBe(result.model_ref);
    const run = { ...analysisRun.analysis_run, hashes: [] };
    const base = currentResultDocumentBase(model, result, run, "test:invented-refusal-manifest", { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:invented" });
    const origin = await currentReceivedOrigin(model, result, "test:invented-refusal-manifest", false, "Test-built origin; not a qualified Current received carrier.");
    expect(origin.received_carrier_checksum.value).toBe(await resultDigest(result));
    expect((await deriveResultDocument(base, model, result, origin)).result_envelope.retained_precision).toStrictEqual(result.retained_precision);
    render(<ResultExportPanel model={model} result={result} analysisRun={analysisRun} inputManifest={null} />);
    expect(await screen.findByText(`${RETAINED_PRECISION_VALIDATION_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`, { exact: false })).toBeTruthy();
    expect(screen.queryByText(RETAINED_PRECISION_OUTPUT_REFUSAL, { exact: false })).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
  it("the report package keeps T0R's fresh-result refusal", async () => {
    const { result } = await successor(mode);
    expect(reportPackageUnavailableReason(result)).toBe(`${REPORT_PACKAGE_FRESH_RESULT_UNAVAILABLE}: ${N_REPORT}`);
  });
});

describe("other consumers are unchanged; the standing session Current reads of a successor", () => {
  // The session hook may be imported only by workspaceSession.ts (sessionBoundary.test.ts),
  // and the pinned request model is not a complete desktop session model (no load-case
  // status), so a full session replay is not possible here. Pinned instead: the inputs
  // resultsSessionState.ts `currentSolvedResult` composes. Current requires a fresh identity, eligible
  // standing and a live native capture for the manifest's model, among its other
  // conjuncts. Since U7 a registered successor is fresh and eligible for its captured
  // model; a copy, or another current model, is not eligible.
  it.each(MODES)("%s: a registered successor is fresh and numerically eligible for its captured model only (U7)", async (mode) => {
    const { source, model } = milestone(mode);
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async () => structuredClone(source));
    const received = await runPreviewMechanics(model, mode);
    expect(hasNativeMechanicsInvocation(received, model, mode)).toBe(true);
    expect(isFreshSemanticResult(received)).toBe(true);
    expect(currentSemanticContract(received)).not.toBeNull();
    expect(numericalResultStanding(received, model)).toMatchObject({ status: "integrity_checked", eligible: true, findings: [] });
    expect(numericalResultStanding(structuredClone(received), model)).toMatchObject({ eligible: false, findings: [RETAINED_PRECISION_VALIDATION_REQUIRED] });
    const moved = structuredClone(model) as Json; moved.nodes[0].position.x += 1;
    expect(hasNativeMechanicsInvocation(received, moved, mode)).toBe(false);
    expect(numericalResultStanding(received, moved)).toMatchObject({ eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] });
    // The pinned request model lacks desktop load-case status, which the manifest requires.
    await expect(buildCurrentSessionInputManifest({ model, solver: { solver_name: "x", solver_version: "x", solver_build_ref: "x", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] })).rejects.toThrow("INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE");
  });
  it("the notices component renders the successor's preview and class notices as text only", async () => {
    const { result } = await successor("sparse_interactive");
    const before = JSON.stringify(result);
    render(<KnownSemanticNotices result={result} testIdPrefix="t" />);
    expect(screen.getByTestId("t-notice-headline-label")).toBeTruthy();
    expect(screen.getByTestId("t-notice-retained-precision-unvalidated")).toBeTruthy();
    expect(JSON.stringify(result)).toBe(before);
  });
});
