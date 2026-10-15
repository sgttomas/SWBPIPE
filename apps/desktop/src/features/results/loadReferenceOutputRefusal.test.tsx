/** Every desktop output of a load/reference-state result is refused with its route's
 * one shared reason, the report package included. B7b: such a result is published
 * under pressure-1 (ordinary) or pressure-source-1 (joined), the v3 twins of the
 * retired load-reference-1 and load-reference-source-1, so the reasons are
 * PRESSURE-1-OUTPUT-NOT-YET-AVAILABLE's two texts. Display stays. A preview-physics-1
 * result is the control: its outputs are unchanged. Unit rendering over committed
 * producer bytes and records; all inputs are invented. */
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { ComponentType } from "react";
import type { AnalysisRunEnvelope, EditorOperationIntent, MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { computeModelHash } from "../../services/hashService";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { PRESSURE_1_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL, loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket, StressNeutralExportPanel } from "../stress-neutral/StressNeutralExportPanel";
import { buildCurrentResultExport, deriveResultDocument, resultDigest, validateResultDocument } from "../result-export/resultExportAdapter";
import { ResultExportPanel } from "../result-export/ResultExportPanel";
import { PcfExportPanel } from "../pcf-export/PcfExportPanel";
import { CaepipeMbfExportPanel } from "../caepipe-mbf/CaepipeMbfExportPanel";
import { CaepipeExternalHarnessPanel } from "../caepipe-external/CaepipeExternalHarnessPanel";
import { ExportAdapterSdkPanel } from "../export-adapter-sdk/ExportAdapterSdkPanel";
import { AdapterFrameworkPanel } from "../adapter-framework/AdapterFrameworkPanel";
import { ExternalProverBoundaryPanel } from "../external-prover/ExternalProverBoundaryPanel";
import { ReviewGeometryPanel, buildReviewGeometryPacket } from "../review-geometry/ReviewGeometryPanel";
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
import { DiffPreviewPanel, buildDiffPreviewPacket } from "../diff-preview/DiffPreviewPanel";
import { OperationLedgerPanel, buildOperationReviewLedger } from "../operations/OperationLedgerPanel";

afterEach(() => cleanup());
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
import { receivedFixture } from "../../test-support/receivedText";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

function desktopModel(modelPath: string): PreviewModel {
  const document = json(modelPath);
  const model = structuredClone(document.model ?? document) as PreviewModel & Record<string, unknown>;
  for (const loadCase of model.load_cases) Object.assign(loadCase, { label: loadCase.label ?? loadCase.id, kind: loadCase.kind ?? "invented_test_case", status: loadCase.status ?? "invented_test_status" });
  model.data_boundary ??= { source: "invented" };
  model.diagnostics ??= [];
  return model;
}
/** `recordPath`: the committed Python product-path record of the result, which the
 * desktop reads (B7b: the desktop builds the same record for pressure-1); otherwise the
 * desktop builds it. */
async function scenario(rawPath: string, modelPath: string, recordPath?: string) {
  const result = receivedFixture<MechanicsResult>(rawPath), model = desktopModel(modelPath);
  const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-output-refusal" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: result.model_ref }, solver_basis: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:invented" } } };
  const analysisRun = recordPath ? json(recordPath) as AnalysisRunEnvelope : await buildAnalysisRunV03(result, manifest, undefined, modelLoadBasisRefs(model));
  if (recordPath) await validateAnalysisRunV03(analysisRun, result, modelLoadBasisRefs(model));
  expect(analysisRun.analysis_run.run_id).toBe(result.run_id);
  const props = {
    model, result, analysisRun, knowledge: null, comparison: null, editorIntents: [], proposal: null, selectedReviewTarget: null,
    projectOperation: "", projectSummary: null, storageCapability: null, modelHash: await computeModelHash(model),
    solveJob: initialSolveJob(), running: false, solverMode: "sparse_interactive", onCancel: () => {}, onRun: () => {}, onSolverModeChange: () => {},
  };
  return { result, model, analysisRun, props };
}
const ordinary = () => scenario("fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json", "fixtures/product_preview/pressure/load_reference/pressure.request.json", "fixtures/results/pressure_load_state_pressure_sparse.analysis_run.json");
const joined = () => scenario("fixtures/product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json", "fixtures/product_preview/pressure_source/load_reference_source/mixed.request.json");
const preview = () => scenario("fixtures/results/preview_physics_connected_sparse.json", "fixtures/model_operations/precision_connected_ui_model.json");
/** The two load/reference-state identities and each one's reason. */
const STATES = [["pressure-1", ordinary, PRESSURE_1_OUTPUT_REFUSAL], ["pressure-source-1", joined, PRESSURE_SOURCE_OUTPUT_REFUSAL]] as const;

/** Surfaces whose downloads or handoffs carry result data (group a). */
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
  it("names the desktop route only, states the current refusal and never calls the result invalid or unsupported", async () => {
    for (const [, make, reason] of STATES) {
      expect(reason).toContain("is not yet available on the desktop");
      expect(reason).not.toMatch(/\bT[0-9]/);
      expect(reason.toLowerCase()).not.toMatch(/invalid|unsupported/);
      const { result } = await make();
      expect(Array.isArray((result.contract_evidence as Json).load_reference_states)).toBe(true);
      expect(loadReferenceOutputRefusal(result)).toBe(reason);
    }
    expect(loadReferenceOutputRefusal((await preview()).result)).toBeNull();
  });
});

describe.each(GATED)("%s", (_label, Panel, prefix) => {
  it.each(STATES)("refuses %s output with the shared reason and keeps its display", async (_id, make, reason) => {
    const { props } = await make();
    render(<Panel {...props} />);
    expect(screen.getByTestId(`${prefix}-load-reference-output-unavailable`).textContent).toBe(reason);
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
  it("leaves a preview-physics-1 result's outputs unchanged", async () => {
    const { props } = await preview();
    render(<Panel {...props} />);
    expect(screen.queryByTestId(`${prefix}-load-reference-output-unavailable`)).toBeNull();
  });
});

/** Group (c) under §12 as ROOT confirmed: these downloads carry only the result's
 * and AnalysisRun's identifiers (run_id, model_state_ref), never result values,
 * so they are not gated. Proof: perturbing every result value, and every
 * AnalysisRun field other than those identifiers, leaves each packet unchanged.
 * The editing panels show a download only when a review record exists, so one
 * invented intent is queued. */
const INTENT = {
  operation_id: "operation:invented", operation_kind: "modify", operation_status: "proposed", author_type: "user",
  target: { object_type: "Model", ref: "model:invented" },
  change: { change_id: "change:invented", change_kind: "set_field", field_label: "Invented field", field_path: "project.name", before: "a", after: "b", unit: "", dimension: "", source_note: "invented" },
  validation: { schema_validation: "invented", constraint_validation: "invented", unit_validation: "invented", diff_preview_status: "invented", application_status: "invented" },
  audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
  professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
  rationale: "invented",
} as unknown as EditorOperationIntent;
/** Perturbs every value outside the kept identifiers: numbers, strings and
 * booleans change, and every array gains one more element (so counts change),
 * per T1_WAVE2_REVIEW F2. */
function perturbed(value: Json, keep: (path: string[]) => boolean, path: string[] = []): Json {
  if (keep(path)) return value;
  if (typeof value === "number") return value + 1;
  if (typeof value === "string") return `${value}:perturbed`;
  if (typeof value === "boolean") return !value;
  if (Array.isArray(value)) {
    const items = value.map((item, index) => perturbed(item, keep, [...path, String(index)]));
    return [...items, items.length ? structuredClone(items[items.length - 1]) : "perturbed:extra"];
  }
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, perturbed(item, keep, [...path, key])]));
  return value;
}
const PROPOSAL = json("fixtures/product_preview/invented_agent_proposal.json");
const RESULT_IDENTIFIERS = new Set(["run_id"]);
const keepResultIdentifier = (path: string[]) => path.length === 1 && RESULT_IDENTIFIERS.has(path[0]);
const keepRunIdentifiers = (path: string[]) => path.length === 0 ? false
  : path[0] !== "analysis_run" ? false
  : path.length === 2 && (path[1] === "run_id" || path[1] === "model_state_ref")
    || (path.length > 2 && path[1] === "model_state_ref");
const EDITING: [string, ComponentType<Json>, string][] = [
  ["Operation diff preview", DiffPreviewPanel, "diff-preview"],
  ["Operation review ledger", OperationLedgerPanel, "operation-ledger"],
];
describe("identifier-only downloads are not gated and carry no result values", () => {
  it.each(STATES)("%s: review geometry packet is invariant under every result value", async (_id, make) => {
    const { model, result, analysisRun } = await make();
    const base = buildReviewGeometryPacket({ model, result, analysisRun });
    const changed = buildReviewGeometryPacket({ model, result: perturbed(result, keepResultIdentifier) as MechanicsResult, analysisRun: perturbed(analysisRun, keepRunIdentifiers) as AnalysisRunEnvelope });
    expect(JSON.stringify(changed)).toBe(JSON.stringify(base));
    render(<ReviewGeometryPanel model={model} result={result} analysisRun={analysisRun} />);
    expect(screen.queryByTestId("review-geometry-load-reference-output-unavailable")).toBeNull();
    expect(screen.getByTestId("review-geometry-export-link")).toBeTruthy();
  });
  it.each(STATES)("%s: diff preview and review ledger packets are invariant under every AnalysisRun value", async (_id, make) => {
    const { model, analysisRun } = await make();
    const changedRun = perturbed(analysisRun, keepRunIdentifiers) as AnalysisRunEnvelope;
    for (const proposal of [null, PROPOSAL]) {
      const args = { model, editorIntents: [INTENT], proposal, selectedReviewTarget: null };
      const diff = buildDiffPreviewPacket({ ...args, analysisRun });
      const ledger = buildOperationReviewLedger({ ...args, analysisRun });
      if (proposal) expect(ledger.records.length).toBe(2);
      expect(JSON.stringify(buildDiffPreviewPacket({ ...args, analysisRun: changedRun }))).toBe(JSON.stringify(diff));
      expect(JSON.stringify(buildOperationReviewLedger({ ...args, analysisRun: changedRun }))).toBe(JSON.stringify(ledger));
    }
  });
  it.each(EDITING)("%s shows its download for a load-reference result", async (_label, Panel, prefix) => {
    const { props } = await ordinary();
    render(<Panel {...props} editorIntents={[INTENT]} onClearReviewQueue={() => {}} />);
    expect(screen.queryByTestId(`${prefix}-load-reference-output-unavailable`)).toBeNull();
    expect(screen.getByTestId(`${prefix}-export-link`)).toBeTruthy();
  });
});

describe("stress-neutral and result export keep their own refusal points", () => {
  it.each(STATES)("%s: packet build, packet validation and panel all refuse", async (_id, make, reason) => {
    const { model, result, analysisRun } = await make();
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow(reason);
    await expect(validateStressNeutralExportPacket({ schema_version: "0.3.0" }, result)).rejects.toThrow(reason);
    await expect(validateStressNeutralExportPacket({ schema_version: "0.3.0", producer: result.producer, numerical_quality: result.numerical_quality, formulation_basis: result.formulation_basis, contract_evidence: result.contract_evidence, ...(result.source_block_recovery ? { source_block_recovery: result.source_block_recovery } : {}), export_profile: {} })).rejects.toThrow(reason);
    render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />);
    expect(screen.getByTestId("stress-neutral-load-reference-output-unavailable").textContent).toBe(reason);
    cleanup();
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest: null })).rejects.toThrow(reason);
    await expect(validateResultDocument({ schema_version: "0.3.0", result_envelope: { schema_version: "0.3.0" } }, result)).rejects.toThrow(reason);
    // T1_WAVE2_REVIEW F3: a hash-consistent origin passes every earlier gate, so
    // deriveResultDocument reaches its own route refusal.
    expect(model.project.id).toBe(result.model_ref);
    const origin = { origin_id: "source-origin:invented", received_carrier_checksum: { value: await resultDigest(result) }, authentic_producer_available: true, original_producer_checksum: null };
    await expect(deriveResultDocument({ result_envelope: {} }, model, result, origin)).rejects.toThrow(reason);
    render(<ResultExportPanel model={model} result={result} analysisRun={analysisRun} inputManifest={null} />);
    expect(await screen.findByText(reason, { exact: false })).toBeTruthy();
  });
  it.each(STATES)("the report package refuses %s with its route's reason (B7b)", async (_id, make, reason) => {
    expect(reportPackageUnavailableReason((await make()).result)).toBe(reason);
  });
});

void (null as unknown as AnalysisRunEnvelope);
