/** Desktop outputs of load/reference-state results. B7b: such a result is published
 * under pressure-2 (ordinary) or pressure-source-1 (joined), the v3 twins of the
 * retired load-reference-1 and load-reference-source-1. Epoch 8 (DEL-04-07 Design
 * `desktop-exact-outputs.md` §5.1-5.4, §6.1; owner ruling 11B, 2026-10-21: "A accept your
 * recommendations for the entire set.") ends T1's deferral for them: they follow the Rust
 * reader. The result JSON export, the stress-neutral export and the rule check add no
 * policy refusal and take Rust's standing, so the committed pressure-2 results pass and
 * every pressure-source-1 one (LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1) and
 * the pressure-2 fallback (NUMERICAL_INTEGRITY_NOT_QUALIFIED) are declined there by
 * standing. Every other gated surface refuses with PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE's
 * reworded texts, and the report package with its fresh-result refusal. Display stays. A
 * preview-physics-2 result is the control: its outputs are unchanged. Unit rendering over
 * committed producer bytes and records; all inputs are invented. */
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { ComponentType } from "react";
import type { AnalysisRunEnvelope, EditorOperationIntent, MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { computeModelHash } from "../../services/hashService";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL, loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED, numericalResultStanding, registerSourceRecovery } from "./resultStanding";
import { N_LOAD_REFERENCE_RULE_CHECK, loadReferenceDisclosure } from "./loadReferenceDisclosure";
import { LoadReferenceOutputGate } from "./LoadReferenceOutputGate";
import {
  OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL, OUTPUT_SURFACES, RETAINED_PHYSICS_OUTPUT_REFUSAL, RETAINED_PRECISION_OUTPUT_REFUSAL,
  routeSurfaceDecision, surfaceOutputRefusal, surfaceRouteRefusal,
} from "./outputPolicy";
import { isFreshSemanticResult } from "./knownSemanticLimitations";
import { buildReportPackageRequest, reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket, StressNeutralExportPanel } from "../stress-neutral/StressNeutralExportPanel";
import { buildCurrentResultExport } from "../result-export/resultExportAdapter";
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
 * desktop reads (B7b: the desktop builds the same record for pressure-2); otherwise the
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
const PRESSURE_LR = "fixtures/product_preview/pressure/load_reference";
const SOURCE_LR = "fixtures/product_preview/pressure_source/load_reference_source";
const FALLBACK = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz";
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
const short = (mode: string) => mode === "sparse_interactive" ? "sparse" : "dense";
/** A committed pressure-2 load/reference-state result with Python's product-path record. */
const pressureState = (name: string, mode: string) => scenario(`${PRESSURE_LR}/${name}-${mode}.raw.json`, `${PRESSURE_LR}/${name}.request.json`, `fixtures/results/pressure_load_state_${name}_${short(mode)}.analysis_run.json`);
/** A committed pressure-source-1 load/reference-state result; the desktop builds its record. */
const sourceState = (name: string, mode: string) => scenario(`${SOURCE_LR}/${name}-${mode}.raw.json`, `${SOURCE_LR}/${name}.request.json`);
const ordinary = () => pressureState("pressure", "sparse_interactive");
const joined = () => sourceState("mixed", "sparse_interactive");
const preview = () => scenario("fixtures/results/preview_physics_connected_sparse.json", "fixtures/model_operations/precision_connected_ui_model.json");
/** The two load/reference-state identities and each one's reason on an unopened surface. */
const STATES = [["pressure-2", ordinary, PRESSURE_2_OUTPUT_REFUSAL], ["pressure-source-1", joined, PRESSURE_SOURCE_OUTPUT_REFUSAL]] as const;
/** Every committed load/reference-state raw, by identity. */
const PRESSURE_RAWS = ["connected", "pressure"].flatMap(name => MODES.map(mode => [`${name}-${mode}`, name, mode] as const));
const SOURCE_RAWS = ["n05", "n06", "fields", "mixed", "eigen_motion"].flatMap(name => MODES.map(mode => [`${name}-${mode}`, name, mode] as const));
/** The surfaces the owner opened: three at epoch 8 (ruling 10A), and the rendered report
 * and the report package at W-11 (DEL-08-01 Design `exact-route-report-sections.md` §5.1;
 * owner, 2026-10-26). */
const OPENED = ["result-export", "stress-neutral", "rule-check", "rendered-report", "report-package"] as const;
/** The opened surfaces among the eighteen gated prefixes. */
const OPENED_GATED: readonly string[] = ["rule-check", "rendered-report"];

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
  it("names the desktop route only, states the reworded refusal and never calls the result invalid or unsupported", async () => {
    for (const [, make, reason] of STATES) {
      // §5.3: the code is kept; the text names the opened surfaces instead of refusing every output.
      expect(reason.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE: This output of ")).toBe(true);
      expect(reason).toContain("is not available on the desktop; only the result JSON and stress-neutral exports, the rule check, the rendered report and the report package admit a numerically eligible result.");
      expect(reason).toContain("The result remains readable here; this is not a finding about the result.");
      expect(reason).not.toMatch(/\bT[0-9]/);
      expect(reason.toLowerCase()).not.toMatch(/invalid|unsupported/);
      const { result } = await make();
      expect(Array.isArray((result.contract_evidence as Json).load_reference_states)).toBe(true);
      // The route's shared reason (no product surface reads it since W-11).
      expect(loadReferenceOutputRefusal(result)).toBe(reason);
    }
    expect(loadReferenceOutputRefusal((await preview()).result)).toBeNull();
  });
});

/** Epoch 8 (§5.2): the gate reads its surface's own decision. The rule-check
 * completeness panel admits the exact routes, and since W-11 the rendered report does too;
 * the other sixteen gated surfaces (the fifteen interop and handoff surfaces and the
 * report JSON packet) still refuse (§5.4). */
describe.each(GATED.filter(([, , prefix]) => !OPENED_GATED.includes(prefix)))("%s", (_label, Panel, prefix) => {
  it.each(STATES)("refuses %s output with the reworded reason and keeps its display", async (_id, make, reason) => {
    const { props } = await make();
    render(<Panel {...props} />);
    const shown = screen.getByTestId(`${prefix}-load-reference-output-unavailable`).textContent;
    expect(shown).toBe(reason);
    expect(shown!.toLowerCase()).not.toMatch(/invalid|unsupported/);
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
    expect(document.querySelector("[data-testid$='-load-reference-disclosure']")).toBeNull();
  });
  it("leaves a preview-physics-2 result's outputs unchanged", async () => {
    const { props } = await preview();
    render(<Panel {...props} />);
    expect(screen.queryByTestId(`${prefix}-load-reference-output-unavailable`)).toBeNull();
  });
});

describe("Rendered report (an opened surface, W-11 §5.1)", () => {
  it.each(STATES)("admits %s at the gate: no refusal, and the render control is offered", async (_id, make) => {
    const { props } = await make();
    render(<RenderedReportPanel {...props} />);
    expect(screen.queryByTestId("rendered-report-load-reference-output-unavailable")).toBeNull();
    expect(screen.getByTestId("rendered-report-render")).toBeTruthy();
  });
});

describe("Rule-check completeness (an opened surface, §4.4)", () => {
  it.each(STATES)("admits %s at the gate and keeps its display", async (_id, make) => {
    const { props } = await make();
    render(<RuleCheckPanel {...props} />);
    expect(screen.queryByTestId("rule-check-load-reference-output-unavailable")).toBeNull();
    expect(screen.getByTestId("rule-check-export-link")).toBeTruthy();
    expect(screen.getByTestId("rule-check-summary")).toBeTruthy();
  });
  it("shows the §5.4 disclosure line for a pressure-2 load/reference-state result (the panel receives only the Current result)", async () => {
    const { props, result } = await ordinary();
    render(<RuleCheckPanel {...props} />);
    const line = screen.getByTestId("rule-check-load-reference-disclosure").textContent;
    expect(line).toBe(loadReferenceDisclosure(result, "rule-check"));
    expect(line).toBe(N_LOAD_REFERENCE_RULE_CHECK.replace("{configurations}", "reference:installed").replace("{cases}", "case:cold-pressure, case:hot-pressure").replace("{histories}", "independent_equilibrium"));
  });
  it("leaves a preview-physics-2 result's outputs unchanged, with no disclosure line", async () => {
    const { props } = await preview();
    render(<RuleCheckPanel {...props} />);
    expect(screen.queryByTestId("rule-check-load-reference-output-unavailable")).toBeNull();
    expect(screen.queryByTestId("rule-check-load-reference-disclosure")).toBeNull();
  });
});

/** The gate itself (§5.2, review follow-up): it reads no standing, so it opens only where
 * the policy adds no refusal at all (`routeSurfaceDecision` null). Any other decision
 * refuses with its reason, `admitted_when_eligible` included. */
describe("LoadReferenceOutputGate", () => {
  const gate = (result: MechanicsResult | null, prefix: string) => {
    const view = render(<LoadReferenceOutputGate result={result} testIdPrefix={prefix}><a data-testid="gated-child" href="data:application/json,{}" download="gated.json">child</a></LoadReferenceOutputGate>);
    const refusal = view.container.querySelector(`[data-testid="${prefix}-load-reference-output-unavailable"]`)?.textContent ?? null;
    const open = view.container.querySelector("[data-testid='gated-child']") !== null;
    view.unmount();
    return { refusal, open };
  };
  it.each([
    ["preview-physics-retained-1", "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json", "retained_preview_physics", RETAINED_PRECISION_OUTPUT_REFUSAL],
    ["physics-retained-1", "fixtures/results/retained_precision_exact_successor_sparse_interactive.json", "retained_physics", RETAINED_PHYSICS_OUTPUT_REFUSAL],
  ] as const)("refuses a retained successor (%s) on every surface, including its admitted_when_eligible panels", (_id, path, route, reason) => {
    const successor = json(path).source as MechanicsResult;
    expect(routeSurfaceDecision(route, "result-export")).toStrictEqual({ decision: "admitted_when_eligible", reason });
    expect(gate(successor, "result-export")).toStrictEqual({ refusal: reason, open: false });
    expect(gate(successor, "stress-neutral")).toStrictEqual({ refusal: reason, open: false });
    for (const [, , prefix] of GATED) expect(gate(successor, prefix), prefix).toStrictEqual({ refusal: reason, open: false });
  });
  it.each(STATES)("opens for %s on rule-check and rendered-report only among the eighteen gated prefixes, and refuses the other sixteen", async (_id, make, reason) => {
    const { result } = await make();
    for (const [, , prefix] of GATED) {
      expect(gate(result, prefix), prefix).toStrictEqual(OPENED_GATED.includes(prefix) ? { refusal: null, open: true } : { refusal: reason, open: false });
    }
    // The other opened surfaces add no refusal either (report-package since W-11); an unnamed prefix refuses.
    for (const surface of OUTPUT_SURFACES.filter(surface => !GATED.some(([, , prefix]) => prefix === surface))) {
      expect(gate(result, surface), surface).toStrictEqual(OPENED.includes(surface as never) ? { refusal: null, open: true } : { refusal: reason, open: false });
    }
    expect(gate(result, "invented-surface")).toStrictEqual({ refusal: reason, open: false });
  });
  it("opens for an ungated route and for no result; a route without an entry fails closed", async () => {
    const { result } = await preview();
    for (const [, , prefix] of GATED) expect(gate(result, prefix), prefix).toStrictEqual({ refusal: null, open: true });
    expect(gate(null, "rule-check")).toStrictEqual({ refusal: null, open: true });
    expect(routeSurfaceDecision("physics_retained", "rule-check")).toStrictEqual({ decision: "refused", reason: OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL });
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

/** §5.1, §6.1: on the opened surfaces the policy adds no refusal; each surface's own
 * standing check takes the Rust reader's verdict. */
describe("the opened surfaces follow the reader's standing", () => {
  it.each(PRESSURE_RAWS)("pressure-2 %s: passes every opened surface's policy and standing check; the stress-neutral package builds and validates", async (_id, name, mode) => {
    const { model, result, analysisRun } = await pressureState(name, mode);
    for (const surface of OPENED) {
      expect(surfaceRouteRefusal(result, surface)).toBeNull();
      expect(surfaceOutputRefusal(result, model, surface)).toBeNull();
    }
    // The standing every opened surface reads (the builders' and the rule-check run's).
    expect(numericalResultStanding(result, model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
    expect(isFreshSemanticResult(result)).toBe(true);
    // Result export: the Current builder passes its policy, standing and fresh-membership
    // checks and stops only at the Current binding this unit test does not supply.
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest: null })).rejects.toThrow("CURRENT_INPUT_MANIFEST_UNAVAILABLE");
    // Stress-neutral: the pure projection builds over Python's record and validates against its source.
    const packet = await buildStressNeutralExportPacket({ model, result, analysisRun });
    expect(packet.contract_evidence).toStrictEqual(result.contract_evidence);
    expect(Object.hasOwn(packet, "source_block_recovery")).toBe(false);
    await validateStressNeutralExportPacket(JSON.parse(JSON.stringify(packet)), result, analysisRun, modelLoadBasisRefs(model));
  }, 120_000); // heavy native-replay builds; the hosted runner is several times slower
  it.each(SOURCE_RAWS)("pressure-source-1 %s: the policy adds no refusal; standing declines it with Rust's finding, and both builders refuse by their standing code", async (_id, name, mode) => {
    const { model, result, analysisRun } = await sourceState(name, mode);
    for (const surface of OPENED) {
      expect(surfaceRouteRefusal(result, surface)).toBeNull();
      expect(surfaceOutputRefusal(result, model, surface)).toBeNull();
    }
    const builders = async () => {
      await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest: null })).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
      await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
    };
    // Unregistered (a received copy, no captured invocation).
    const unregistered = numericalResultStanding(result, model);
    expect(unregistered).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false });
    expect(unregistered.findings).toContain(SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED);
    await builders();
    // Registered with its captured invocation: still never eligible, by Rust's own finding.
    await registerSourceRecovery(result, { request: json(`${SOURCE_LR}/${name}.request.json`), solver_mode: mode });
    const registered = numericalResultStanding(result, json(`${SOURCE_LR}/${name}.request.json`).model);
    expect(registered).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false });
    expect(registered.findings).toContain(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE);
    await builders();
  }, 120_000); // heavy native-replay builds; the hosted runner is several times slower
  it.each(MODES)("the pressure-2 fallback (%s): the policy adds no refusal; standing declines it, and both builders refuse by their standing code", async (mode) => {
    const { model, result, analysisRun } = await scenario(`${FALLBACK}-${mode}.raw.json`, `${FALLBACK}.request.json`);
    expect(Array.isArray((result.contract_evidence as Json).load_reference_states)).toBe(true);
    for (const surface of OPENED) expect(surfaceOutputRefusal(result, model, surface)).toBeNull();
    const standing = numericalResultStanding(result, model);
    expect(standing).toMatchObject({ contract: "pressure", status: "needs_recompute", eligible: false });
    expect(standing.findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest: null })).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
  }, 120_000); // heavy native-replay builds; the hosted runner is several times slower
  it.each(STATES)("%s: the export panels add no policy refusal, and offer nothing without the live capture", async (_id, make, reason) => {
    // A received copy is never Current (no live native capture); the panels show their
    // empty state, never the policy's refusal, and no disclosure line.
    const { model, result, analysisRun } = await make();
    render(<StressNeutralExportPanel model={model} result={result} analysisRun={analysisRun} />);
    expect(screen.queryByTestId("stress-neutral-load-reference-output-unavailable")).toBeNull();
    expect(screen.getByTestId("stress-neutral-empty")).toBeTruthy();
    expect(screen.queryByTestId("stress-neutral-load-reference-disclosure")).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
    cleanup();
    render(<ResultExportPanel model={model} result={result} analysisRun={analysisRun} inputManifest={null} />);
    expect(await screen.findByTestId("result-export-empty")).toBeTruthy();
    expect(document.body.textContent).not.toContain(reason);
    expect(screen.queryByTestId("result-export-load-reference-disclosure")).toBeNull();
    expect(document.querySelectorAll("a[download]")).toHaveLength(0);
  });
  it.each(STATES)("the report package admits %s past its report-package cell, and the report JSON packet still refuses it (W-11 §5.1, §5.4)", async (_id, make, reason) => {
    const { model, result, analysisRun } = await make();
    expect(reportPackageUnavailableReason(result)).toBeNull();
    expect(surfaceRouteRefusal(result, "report-package")).toBeNull();
    expect(surfaceRouteRefusal(result, "report")).toBe(reason);
    // Past the cell, the builder's own admission decides (here: no input manifest).
    let message = "";
    try { await buildReportPackageRequest({ model, result, analysisRun } as never); } catch (error) { message = String(error); }
    expect(message).not.toBe("");
    expect(message).not.toContain(reason);
  });
});

void (null as unknown as AnalysisRunEnvelope);
