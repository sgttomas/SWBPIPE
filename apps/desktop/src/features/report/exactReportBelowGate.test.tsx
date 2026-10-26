/** W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §3.1-§3.6, §5.5, §7):
 * the desktop's exact-route report sections and their Export Result JSON admission,
 * exercised BELOW the output gate. Every report surface still refuses an exact result
 * (the `rendered-report`, `report-package` and `report` cells, ruling 10A), and the
 * native render and the package's Rust boundary refuse every exact identity; these
 * tests reach the sections through the builder, the panel's build step
 * (`assembleExactReportBelowOutputGate`) and the package assembly
 * (`assembleReportPackageRequestBelowOutputGate`) under the native-capture test seams.
 *
 * - the projection against the shared vectors `exact_report_sections_parity.json`;
 * - the admission binding, same-run substitution, copies and asynchronous retirement;
 * - the panel and the package request send no IPC once the admission is retired;
 * - the still-refused surfaces;
 * - the witness and substitute requests the native and Rust witnesses read
 *   (`fixtures/reports/invented/exact_report_package_requests/`, stale fails;
 *   `OPENPIPESTRESS_WRITE_EXACT_PACKAGE_REQUESTS=<dir>` writes them).
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. Job ids are the replay's invented ids; case labels and project
 * names are added where a committed request has none. Standard claim fence
 * applies (claims taxonomy per DEC-081). */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import {
  buildAnalysisRunPreview,
  deliveredMechanicsResultJob,
  disposeDeliveredMechanicsResult,
  resetSessionResultCustodyForTests,
  runPreviewMechanics
} from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { canonicalJsonString, canonicalSha256Hex, checkedJsonText } from "../../services/hashService";
import { renderCalculationReport } from "../../services/reportRenderService";
import { invokeReportPackageSave, prepareReportPackageSave } from "../../services/reportPackageSaveService";
import { IngressRequestRetiredError } from "../../services/nativeIngress";
import * as resultExportAdapter from "../result-export/resultExportAdapter";
import { sourceContract, sourceSemanticBinding } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { OUTPUT_POLICY, OUTPUT_SURFACES, PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL, surfaceOutputRefusal } from "../results/outputPolicy";
import { PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID } from "../results/numericalResultQuality";
import { controlReportPackageRequest } from "./reportRedactionProjector";
import {
  admitExactReportSource,
  checkExactReportAdmission,
  REPORT_EXACT_ADMISSION_BINDING_MISMATCH,
  REPORT_EXACT_ADMISSION_REQUIRED,
  REPORT_EXACT_ADMISSION_STALE,
  type ExactReportAdmission
} from "./exactReportAdmission";
import { exactLimitationEntries, exactRouteCore, REPORT_EXACT_SECTIONS_MISMATCH } from "./exactReportSections";
import { buildRenderableReportInput } from "./renderableReportInput";
import { reportResultRows } from "./reportResultRows";
import { numericalHealthView, readNumericalHealth } from "../results/numericalHealth";
import { assembleExactReportBelowOutputGate } from "./renderedReportAssembly";
import { RenderedReportPanel } from "./RenderedReportPanel";
import {
  assembleReportPackageRequestBelowOutputGate,
  buildReportPackageRequest,
  REPORT_PACKAGE_ADMITTED_CONTRACT_IDS,
  REPORT_PACKAGE_ROUTE_NOT_ADMITTED,
  reportPackageExactSubmission,
  type ReportPackageRequestInput
} from "./reportPackageRequest";
import parity from "../../../../../fixtures/reports/invented/exact_report_sections_parity.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = (path: string) => JSON.parse(text(path));
const ALL_IDS = [PREVIEW_PHYSICS_CONTRACT_ID, PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID] as const;
const PUBLIC = { exportContext: "public_report" as const, generatedAt: null };

/* -------------------------------------------------------------------------- *
 * The witness shapes and their solves under the native-capture test seams
 * -------------------------------------------------------------------------- */
type Shape = { name: string; mode: "sparse_interactive" | "dense_scrutiny"; rawPath: string; requestPath: string };
/** The committed exact inputs a desktop session can solve (a request model and the raw
 * bytes its producer wrote): pressure-2 straight (X0, the straight corpus's input),
 * pressure-2 with load/reference states (Y0), pressure-2 with an objective connector
 * (T4-U5b AS-F, the connector corpus's pressurized tied joint) and with a realized bend
 * (U2-L-ANCH-PTW-K2 0.3.0, the arc corpus's input), whose re-authored requests and
 * worker bytes `core/product_physics/tests/exact_route_limitations.rs` writes
 * (`exact_report_witness_inputs_are_current`), and pressure-source-1 (n05). */
const SHAPES: readonly Shape[] = [
  { name: "pressure_2_straight", mode: "sparse_interactive", rawPath: "fixtures/results/pressure_connected_mechanics_sparse.json", requestPath: "core/product_physics/tests/fixtures/exact_pressure_connected_request.json" },
  { name: "pressure_2_load_reference_states", mode: "sparse_interactive", rawPath: "fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json", requestPath: "fixtures/product_preview/pressure/load_reference/pressure.request.json" },
  { name: "pressure_2_connector", mode: "sparse_interactive", rawPath: "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector-sparse_interactive.raw.json", requestPath: "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector.request.json" },
  { name: "pressure_2_realized_bend", mode: "sparse_interactive", rawPath: "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_realized_bend-sparse_interactive.raw.json", requestPath: "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_realized_bend.request.json" },
  { name: "pressure_source_1", mode: "sparse_interactive", rawPath: "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", requestPath: "fixtures/product_preview/pressure_source/physics_source/n05.request.json" }
];
const shape = (name: string) => SHAPES.find((item) => item.name === name)!;

type Session = { model: PreviewModel; result: MechanicsResult; analysisRun: Json; inputManifest: CurrentSessionInputManifestEvidence; intakeText: string; rawText: string };

/** One native-capture solve of `rawText` for the shape's request model (case labels
 * added where the request lacks them; invented labels only), its manifest and record. */
async function solve(item: Shape, rawText = text(item.rawPath), jobIdPrefix = `invented-w11b-${item.name}-job`): Promise<Session> {
  const model = structuredClone(json(item.requestPath).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  // The report title names the project; an invented name where the request has none.
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: `Invented exact-route witness ${item.name}` });
  (window as Json).__TAURI_INTERNALS__ = {};
  let intakeText = "";
  const transport = createSolveJobTransport((sent) => { intakeText = sent.text; return rawText; }, { jobIdPrefix });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model, item.mode);
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: `${result.producer!.component_name}@${result.producer!.component_version}`, solver_mode: item.mode, settings: {} },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun, intakeText, rawText };
}

function packageInput(session: Session): ReportPackageRequestInput {
  const { model, result, analysisRun, inputManifest } = session;
  return { model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null };
}

async function utf8Sha256(value: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(value));
  return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/* -------------------------------------------------------------------------- *
 * The projection against the shared vectors (§3.5)
 * -------------------------------------------------------------------------- */
function resolveVector(spec: Json): MechanicsResult {
  const document = JSON.parse(text(spec.path));
  const pointer = (value: Json, path: string) => path.split("/").slice(1).reduce((item, key) => item[key.replaceAll("~1", "/").replaceAll("~0", "~")], value);
  const source = structuredClone(spec.pointer ? pointer(document, spec.pointer) : document);
  for (const edit of spec.edits as Json[]) {
    const keys = (edit.pointer as string).split("/").slice(1);
    const last = keys.pop()!;
    const parent = keys.reduce((item: Json, key: string) => item[key], source);
    if (edit.op === "set") parent[last] = edit.value;
    else if (edit.op === "remove") delete parent[last];
    else if (edit.op === "append") parent[last].push(edit.value);
    else if (edit.op === "replace_text") parent[last] = (parent[last] as string).replace(edit.from, edit.to);
    else throw new Error(`unknown edit ${edit.op}`);
  }
  return source as MechanicsResult;
}

describe("the exact-section projection against the shared vectors (§3.5)", () => {
  it.each(parity.vectors.map((vector) => [vector.name, vector] as const))("%s: the core and the limitation entries equal Rust's", async (_name, vector) => {
    const source = resolveVector(vector.source);
    expect(await exactRouteCore(source)).toEqual(vector.expected.exact_route_core);
    const run = (vector.expected.exact_route_core as Json).run_ref as string;
    expect(await exactLimitationEntries(source, run, sourceSemanticBinding(source))).toEqual(vector.expected.limitations);
  });

  it.each(parity.refusals.map((refusal) => [refusal.name, refusal] as const))("%s refuses with the same code", async (_name, refusal) => {
    const source = resolveVector(refusal.source);
    const project = async () => {
      const core = await exactRouteCore(source);
      await exactLimitationEntries(source, core.run_ref, sourceSemanticBinding(source));
    };
    // The pressure-free source is the native host's section refusal on the desktop side.
    const code = refusal.code === "REPORT-PACKAGE-EXACT-SECTIONS-MISMATCH" ? REPORT_EXACT_SECTIONS_MISMATCH : refusal.code;
    await expect(project()).rejects.toThrow(new RegExp(`^${code}: `));
  });

  it("names Rust's admitted set as the package's (REPORT_PACKAGE_ADMITTED_CONTRACT_IDS)", () => {
    expect([...REPORT_PACKAGE_ADMITTED_CONTRACT_IDS]).toEqual(parity.admitted_semantic_contracts);
    expect([...REPORT_PACKAGE_ADMITTED_CONTRACT_IDS]).toEqual([PREVIEW_PHYSICS_CONTRACT_ID]);
  });
});

/* -------------------------------------------------------------------------- *
 * The admission (§3.1)
 * -------------------------------------------------------------------------- */
describe("admitExactReportSource (§3.1)", () => {
  it("binds each field to its document value and its recomputed value", async () => {
    const session = await solve(shape("pressure_2_straight"));
    const { model, result, analysisRun, inputManifest } = session;
    expect(sourceContract(result)).toBe("pressure");
    expect(numericalResultStanding(result, model).eligible).toBe(true);
    const admission = await admitExactReportSource({ model, result, analysisRun, inputManifest });
    const e = admission.document.result_envelope;
    const b = admission.binding;
    expect(b).toEqual({
      semantic_contract_id: PRESSURE_CONTRACT_ID,
      run_ref: result.run_id,
      result_envelope_id: `result-envelope:${result.run_id}`,
      received_result_sha256: await resultExportAdapter.resultDigest(result),
      model_sha256: await canonicalSha256Hex(model),
      input_manifest_ref: inputManifest.manifest_ref.ref,
      input_manifest_sha256: inputManifest.manifest_sha256,
      derivative_sha256: e.reproducibility.derivative_hash.value,
      native_job_id: deliveredMechanicsResultJob(result),
      received_bytes_sha256: await utf8Sha256(session.rawText)
    });
    expect(e.producer.semantic_contract_id).toBe(b.semantic_contract_id);
    expect(e.run_ref.ref_id).toBe(b.run_ref);
    expect(e.envelope_id).toBe(b.result_envelope_id);
    expect(e.reproducibility.source_origin_bindings[0].received_carrier_checksum.value).toBe(b.received_result_sha256);
    expect(e.reproducibility.model_hash.value).toBe(b.model_sha256);
    expect(e.reproducibility.audit_manifest_ref.ref_id).toBe(b.input_manifest_ref);
    expect(analysisRun.analysis_run.reproducibility.input_manifest_hashes[0].value).toBe(b.input_manifest_sha256);
    // The document is the one Export Result JSON saves.
    const saved = await resultExportAdapter.buildCurrentResultExport({ model, result, analysisRun, inputManifest });
    expect(await canonicalJsonString(admission.document)).toBe(await canonicalJsonString(saved));
    expect(() => checkExactReportAdmission(admission, { model, result, analysisRun, inputManifest })).not.toThrow();
  });

  it.each([
    ["semantic_contract_id", (doc: Json) => { doc.result_envelope.producer.semantic_contract_id = PRESSURE_SOURCE_CONTRACT_ID; }],
    ["run_ref", (doc: Json) => { doc.result_envelope.run_ref.ref_id = "run:invented-other"; }],
    ["result_envelope_id", (doc: Json) => { doc.result_envelope.envelope_id = "result-envelope:invented-other"; }],
    ["received_result_sha256", (doc: Json) => { doc.result_envelope.reproducibility.source_origin_bindings[0].received_carrier_checksum.value = "0".repeat(64); }],
    ["model_sha256", (doc: Json) => { doc.result_envelope.reproducibility.model_hash.value = "0".repeat(64); }],
    ["input_manifest_ref", (doc: Json) => { doc.result_envelope.reproducibility.audit_manifest_ref.ref_id = "input-manifest:invented-other"; }],
    ["derivative_sha256", (doc: Json) => { doc.result_envelope.reproducibility.derivative_hash.value = "not-a-digest"; }]
  ] as const)("refuses a document whose %s disagrees", async (field, mutate) => {
    const { model, result, analysisRun, inputManifest } = await solve(shape("pressure_2_straight"));
    const original = resultExportAdapter.buildCurrentResultExport;
    const spy = vi.spyOn(resultExportAdapter, "buildCurrentResultExport").mockImplementation(async (args) => {
      const document = await original(args);
      mutate(document);
      return document;
    });
    await expect(admitExactReportSource({ model, result, analysisRun, inputManifest })).rejects.toThrow(new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: ${field}`));
    expect(spy).toHaveBeenCalledTimes(1);
  });

  // Codex r1 finding 1: an edit made after Export Result JSON admitted the inputs, while
  // the admission still awaits its own digests, makes the admission stale. Here the
  // analysis record's status is changed the moment the Current builder has resolved.
  it("refuses an analysis-record edit made after Export Result JSON admitted the inputs", async () => {
    const { model, result, analysisRun, inputManifest } = await solve(shape("pressure_2_straight"));
    const original = resultExportAdapter.buildCurrentResultExport;
    let resolved = false;
    const spy = vi.spyOn(resultExportAdapter, "buildCurrentResultExport").mockImplementationOnce(async (args) => {
      const document = await original(args);
      resolved = true;
      // Mid-flight, after the Current builder succeeded and before the admission returns.
      analysisRun.analysis_run.analysis_status.push("USER_RULE_FAILED");
      return document;
    });
    await expect(admitExactReportSource({ model, result, analysisRun, inputManifest })).rejects.toThrow(new RegExp(`^${REPORT_EXACT_ADMISSION_STALE}: `));
    expect(spy).toHaveBeenCalledTimes(1);
    expect(resolved).toBe(true);
    // A fresh Export Result JSON build refuses the same inputs.
    await expect(original({ model, result, analysisRun, inputManifest })).rejects.toThrow("CURRENT_STATUS_BINDING_MISMATCH");
  });

  it("refuses a pressure-free source and a result without live custody", async () => {
    const session = await solve(shape("pressure_2_straight"));
    const { model, result, analysisRun, inputManifest } = session;
    const copy = structuredClone(result);
    await expect(admitExactReportSource({ model, result: copy, analysisRun, inputManifest })).rejects.toThrow();
    const preview = json("fixtures/results/preview_physics_connected_sparse.json") as MechanicsResult;
    await expect(admitExactReportSource({ model, result: preview, analysisRun, inputManifest })).rejects.toThrow(new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: semantic_contract_id`));
  });

  it("refuses a same-run substitute offered with the genuine admission (STALE)", async () => {
    const { model, result, analysisRun, inputManifest } = await solve(shape("pressure_2_straight"));
    const admission = await admitExactReportSource({ model, result, analysisRun, inputManifest });
    const substitute = structuredClone(result);
    substitute.results[substitute.results.length - 1].value += 1;
    expect([substitute.run_id, substitute.producer?.semantic_contract_id]).toEqual([result.run_id, result.producer?.semantic_contract_id]);
    await expect(buildRenderableReportInput({ model, result: substitute, analysisRun, inputManifest, projectSummary: null, renderContext: PUBLIC, exactAdmission: admission }))
      .rejects.toThrow(new Error(`${REPORT_EXACT_ADMISSION_STALE}: the result Export Result JSON admitted is no longer this session's live result.`));
  });

  it("refuses a JSON copy of an admission and a missing one (REQUIRED)", async () => {
    const { model, result, analysisRun, inputManifest } = await solve(shape("pressure_2_straight"));
    const admission = await admitExactReportSource({ model, result, analysisRun, inputManifest });
    const required = new Error(`${REPORT_EXACT_ADMISSION_REQUIRED}: an exact-route report is built only from a result that Export Result JSON admits.`);
    for (const exactAdmission of [JSON.parse(JSON.stringify(admission)) as ExactReportAdmission, { ...admission }, null]) {
      await expect(buildRenderableReportInput({ model, result, analysisRun, inputManifest, projectSummary: null, renderContext: PUBLIC, exactAdmission })).rejects.toThrow(required);
    }
  });

  it.each([
    ["the result disposed", (session: Session) => { void disposeDeliveredMechanicsResult(session.result); }],
    ["the model edited", (session: Session) => { (session.model as Json).nodes[0].position.x += 1; }],
    ["the result edited", (session: Session) => { session.result.results[0].value += 1; }]
  ] as const)("refuses at the builder's end check when %s while it awaits", async (_label, retire) => {
    const session = await solve(shape("pressure_2_straight"));
    const { model, result, analysisRun, inputManifest } = session;
    const admission = await admitExactReportSource({ model, result, analysisRun, inputManifest });
    const pending = buildRenderableReportInput({ model, result, analysisRun, inputManifest, projectSummary: null, renderContext: PUBLIC, exactAdmission: admission });
    retire(session);
    await expect(pending).rejects.toThrow(new RegExp(`^${REPORT_EXACT_ADMISSION_STALE}: `));
  });
});

/* -------------------------------------------------------------------------- *
 * The sections in the report, below the gate (§2.4, §3.2-§3.4)
 * -------------------------------------------------------------------------- */
describe("the exact sections in the renderer input, below the gate", () => {
  async function assemble(session: Session) {
    const { model, result, analysisRun, inputManifest } = session;
    return assembleExactReportBelowOutputGate({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null });
  }

  it.each(SHAPES.map((item) => [item.name, item] as const))("%s: the exact block, the binding, the headlines, the rows and the limitation entries", async (_name, item) => {
    const session = await solve(item);
    const assembly = await assemble(session);
    const input = assembly.input as Json;
    const { result, analysisRun } = session;
    const run = result.run_id;
    const core = await exactRouteCore(result);
    expect(input.exact_route).toEqual({ semantic_contract_id: result.producer!.semantic_contract_id, admission: { ...assembly.admission!.binding }, load_reference_states: core.load_reference_states, headlines: core.headlines });
    // The binding agrees with the report's own records, as the renderer checks (§3.2).
    const binding = input.exact_route.admission;
    expect(binding.run_ref).toBe(input.report_sections.run_ref.ref_id);
    expect(binding.model_sha256).toBe(input.calculation_report.model_input_summary.model_hash.value);
    expect(binding.input_manifest_ref).toBe(input.calculation_report.model_input_summary.input_manifest_ref.ref_id);
    const received = input.calculation_report.audit_manifest_refs.find((entry: Json) => entry.ref.ref_type === "result_envelope");
    expect(received.ref.ref_id).toBe(binding.result_envelope_id);
    expect(received.checksum.value).toBe(binding.received_result_sha256);
    // Headlines: one per section basis, nominal then reduced, as published.
    expect(input.exact_route.headlines.map((headline: Json) => headline.basis)).toEqual(["nominal_section", "reduced_section"]);
    for (const headline of input.exact_route.headlines) {
      const published = (result.summary as Json)[`max_open_formula_stress_${headline.basis}`];
      if (published === null) expect(headline).toMatchObject({ location_ref: null, source_ref: null });
      else expect(headline).toMatchObject({ value_display: `${published.value} ${published.unit}`, location_ref: published.location_ref, source_ref: published.result_ref });
    }
    // The load/reference line only for a load/reference-state result.
    const states = (result.contract_evidence as Json)?.load_reference_states;
    if (Array.isArray(states) && states.length) expect(input.exact_route.load_reference_states).toMatch(/Their published limitations are listed under Limitations\.$/);
    else expect(input.exact_route.load_reference_states).toBeNull();
    // Rows: the section-basis suffix on stress rows.
    const labels = input.result_rows.map((row: Json) => row.label as string);
    expect(labels.some((label: string) => label.endsWith(" [nominal section]"))).toBe(true);
    expect(labels.some((label: string) => label.endsWith(" [reduced section]"))).toBe(true);
    // Limitations: the technical preview first, then one entry per producer string, then
    // the profile reference, sharing the technical-preview entry's effect and provenance.
    const limitations = input.report_sections.limitations as Json[];
    const published = (result.formulation_basis as Json).limitations as string[];
    expect(limitations[0].limitation_id).toBe(`limitation:${run}:technical-preview`);
    expect(limitations.map((entry) => entry.limitation_id).slice(1)).toEqual([...published.map((_, index) => `limitation:${run}:exact:${index + 1}`), `limitation:${run}:exact:profile-reference`]);
    const semantic = analysisRun.analysis_run.reproducibility.semantic_contract;
    expect(limitations.slice(1)).toEqual((await exactLimitationEntries(result, run, semantic)).map((entry) => ({ ...entry, effect: limitations[0].effect, provenance: limitations[0].provenance })));
    expect(limitations.at(-1)!.statement).toContain(`semantic table ${semantic.id} (sha256 ${semantic.sha256})`);
    // The omitted sentence is in no statement (DEC-107).
    expect(JSON.stringify(limitations)).not.toContain("professional acceptance is produced");
    // The pre-IPC re-check yields the texts the native render re-reads.
    const submission = assembly.exactSubmission!();
    expect(submission.exactResultText).toBe(session.rawText);
    expect(submission.exactIntakeText).toBe(session.intakeText);
    expect(submission.exactIntakeText).toBe(checkedJsonText(session.model));
    expect(await utf8Sha256(submission.inputManifestText)).toBe(session.inputManifest.manifest_sha256);
  });

  // Codex r1 finding 2: an exact report carries no numerical-health line (the native
  // render re-derives its rows exactly and admits none it cannot derive).
  it("adds no numerical-health line to an exact report, even when the panel supplies one", async () => {
    const session = await solve(shape("pressure_2_straight"));
    const { model, result, analysisRun, inputManifest } = session;
    const health = numericalHealthView(readNumericalHealth(result, numericalResultStanding(result, model).findings), { eligible: true, verdictHold: false });
    expect(health?.route).toBe("generic");
    expect(health!.cases.length).toBeGreaterThan(0);
    const assembly = await assembleExactReportBelowOutputGate({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: health, context: "local_private", intent: true, generatedAt: null });
    const rows = (assembly.input as Json).result_rows as Json[];
    expect(rows).toEqual(reportResultRows(result));
    expect(rows.some((row) => String(row.row_id).startsWith("numerical-health:"))).toBe(false);
  });

  it("a pressure-free source takes no admission and its input is unchanged", async () => {
    const result = json("fixtures/results/preview_physics_connected_sparse.json") as MechanicsResult;
    const admission = { document: {}, binding: {} } as unknown as ExactReportAdmission;
    await expect(buildRenderableReportInput({ model: json("fixtures/model_operations/precision_connected_ui_model.json"), result, analysisRun: null as never, projectSummary: null, renderContext: PUBLIC, exactAdmission: admission }))
      .rejects.toThrow(new Error(`${REPORT_EXACT_ADMISSION_BINDING_MISMATCH}: semantic_contract_id`));
  });
});

/* -------------------------------------------------------------------------- *
 * No IPC once the admission is retired (§3.1)
 * -------------------------------------------------------------------------- */
describe("retirement after the build sends no IPC", () => {
  it("the panel's build step: the pre-IPC re-check refuses and the render is not sent", async () => {
    const session = await solve(shape("pressure_2_straight"));
    const { model, result, analysisRun, inputManifest } = session;
    const assembly = await assembleExactReportBelowOutputGate({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null });
    expect(() => assembly.exactSubmission!()).not.toThrow();
    await disposeDeliveredMechanicsResult(result);
    invokeMock.mockClear();
    expect(() => assembly.exactSubmission!()).toThrow(new RegExp(`^${REPORT_EXACT_ADMISSION_STALE}: `));
    const stillCurrent = () => { try { assembly.exactSubmission!(); return true; } catch { return false; } };
    const route = await renderCalculationReport(assembly.controlled, { stillCurrent, exactSubmission: null });
    expect(route).toMatchObject({ route: "redaction_blocked", diagnostic: expect.stringMatching(/^REPORT-RENDER-REQUEST-RETIRED/) });
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("render_calculation_report");
  });

  it("the package request: the pre-IPC re-check refuses and the save is not sent", async () => {
    const session = await solve(shape("pressure_2_straight"));
    const request = await assembleReportPackageRequestBelowOutputGate(packageInput(session), ALL_IDS);
    expect(reportPackageExactSubmission(request)).toEqual({ exactResultText: session.rawText, exactIntakeText: session.intakeText });
    const controlled = controlReportPackageRequest(request, true);
    const prepared = await prepareReportPackageSave(controlled, session.model, request, session.inputManifest.manifest, reportPackageExactSubmission(request));
    expect(prepared.route).toBe("ready");
    if (prepared.route !== "ready") return;
    expect(prepared.args.exactResultText).toBe(session.rawText);
    expect(prepared.args.exactIntakeText).toBe(session.intakeText);
    await disposeDeliveredMechanicsResult(session.result);
    invokeMock.mockClear();
    expect(() => reportPackageExactSubmission(request)).toThrow(new RegExp(`^${REPORT_EXACT_ADMISSION_STALE}: `));
    const guard = () => { try { reportPackageExactSubmission(request); return true; } catch { return false; } };
    await expect(invokeReportPackageSave(prepared, guard)).rejects.toBeInstanceOf(IngressRequestRetiredError);
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("save_report_package");
    // A pressure-free request carries no exact texts.
    expect(reportPackageExactSubmission({})).toBeNull();
  });
});

/* -------------------------------------------------------------------------- *
 * Still refused (§5.1, §7)
 * -------------------------------------------------------------------------- */
describe("still refused: no exact-route result reaches a report output", () => {
  it("the exact routes' cells: report, rendered-report and report-package refuse; only result-export, stress-neutral and rule-check admit", () => {
    for (const route of ["pressure", "pressure_source"] as const) {
      const policy = OUTPUT_POLICY[route];
      if (policy.gate !== "per_surface") throw new Error("gated");
      expect(OUTPUT_SURFACES.filter((surface) => policy.surfaces[surface] !== "refused")).toEqual(["rule-check", "result-export", "stress-neutral"]);
      for (const surface of ["report", "rendered-report", "report-package"] as const) expect(policy.surfaces[surface]).toBe("refused");
      expect(policy.reason.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE: ")).toBe(true);
    }
  });

  it.each(SHAPES.map((item) => [item.name, item] as const))("%s: the panel shows the refusal and no render control, and buildReportPackageRequest refuses the witness input", async (_name, item) => {
    const session = await solve(item);
    const { model, result, analysisRun } = session;
    const refusal = sourceContract(result) === "pressure" ? PRESSURE_2_OUTPUT_REFUSAL : PRESSURE_SOURCE_OUTPUT_REFUSAL;
    expect(surfaceOutputRefusal(result, model, "rendered-report")).toBe(refusal);
    render(<RenderedReportPanel model={model} result={result} analysisRun={analysisRun} projectSummary={null} inputManifest={session.inputManifest} />);
    expect(screen.getByTestId("rendered-report-load-reference-output-unavailable")).toHaveTextContent(refusal);
    expect(screen.queryByTestId("rendered-report-render")).toBeNull();
    const exportSpy = vi.spyOn(resultExportAdapter, "buildCurrentResultExport");
    await expect(buildReportPackageRequest(packageInput(session))).rejects.toThrow(new Error(refusal));
    expect(exportSpy).not.toHaveBeenCalled();
    // Below the cell, the production admitted set still refuses the exact document by name.
    await expect(assembleReportPackageRequestBelowOutputGate(packageInput(session), REPORT_PACKAGE_ADMITTED_CONTRACT_IDS))
      .rejects.toThrow(new RegExp(`^${REPORT_PACKAGE_ROUTE_NOT_ADMITTED}: the report package admits only ${PREVIEW_PHYSICS_CONTRACT_ID} results; this document is ${result.producer!.semantic_contract_id}\\.`));
  });
});

/* -------------------------------------------------------------------------- *
 * The witness and substitute requests (§5.5)
 * -------------------------------------------------------------------------- */
const WITNESS_DIR = "fixtures/reports/invented/exact_report_package_requests";
const GENERATOR = "apps/desktop/src/features/report/exactReportBelowGate.test.tsx";

function witnessFile(item: Shape, session: Session, request: Json, extra: Record<string, unknown> = {}) {
  return `${JSON.stringify({
    schema_version: "1.0.0",
    generator: GENERATOR,
    description: "W-11 slice (b) witness request (DEL-08-01 Design exact-route-report-sections.md section 5.5): the version 2.0.0 report-package request the desktop assembles below the output gate for an exact-route result, with the model, the run's input manifest and the texts the native host binds (the intake model text; the delivered result bytes are the committed raw file). Every production entry point refuses it (REPORT-PACKAGE-ROUTE-NOT-ADMITTED; the native save first at its custody binding). Job ids are invented; unit transport replay of committed producer bytes, not a native UI qualification witness.",
    shape: item.name,
    solver_mode: item.mode,
    raw_path: item.rawPath,
    request_path: item.requestPath,
    exact_intake_text: session.intakeText,
    model: session.model,
    input_manifest: session.inputManifest.manifest,
    request,
    ...extra
  })}\n`;
}

/** The substitute (§5.5, §7): the pressure-2 straight source with one row value
 * changed, solved, recorded and assembled by the same builders, so every hash in the
 * document, report and handoff records is rebound; then given the genuine request's
 * `native_job_id` and `received_bytes_sha256`. */
async function substituteRequest(genuine: Json) {
  const item = shape("pressure_2_straight");
  const raw = JSON.parse(text(item.rawPath));
  // A nodal rotation row, which no section-basis pair or magnitude row repeats.
  const target = raw.results.findIndex((row: Json) => row.kind === "global_nodal_rotation_x" && row.value !== 0);
  expect(target).toBeGreaterThanOrEqual(0);
  raw.results[target].value = raw.results[target].value * (1 + 1e-6);
  const session = await solve(item, solveBytes(raw), "invented-w11b-substitute-job");
  expect(numericalResultStanding(session.result, session.model).eligible).toBe(true);
  const request = await assembleReportPackageRequestBelowOutputGate(packageInput(session), ALL_IDS) as Json;
  for (const field of ["native_job_id", "received_bytes_sha256"]) request.report.exact_route.admission[field] = genuine.report.exact_route.admission[field];
  return { session, request, row: raw.results[target].id as string };
}

/** The leaf replacements that turn `base` into `variant`, as `[{pointer, value}]` in
 * document order (the headless runner's alias-overlay form,
 * `del1005_package_witness.rs` `leaf_overlay`). Both must have the same shape. */
function leafOverlay(base: Json, variant: Json, pointer = "", out: Json[] = []): Json[] {
  const escape = (key: string) => key.replaceAll("~", "~0").replaceAll("/", "~1");
  if (Array.isArray(base) && Array.isArray(variant)) {
    expect(variant.length, `${pointer}: the substitute changes an array length`).toBe(base.length);
    base.forEach((item, index) => leafOverlay(item, variant[index], `${pointer}/${index}`, out));
  } else if (base && variant && typeof base === "object" && typeof variant === "object" && !Array.isArray(base) && !Array.isArray(variant)) {
    expect(Object.keys(variant), `${pointer}: the substitute changes an object's shape`).toEqual(Object.keys(base));
    for (const key of Object.keys(base)) leafOverlay(base[key], variant[key], `${pointer}/${escape(key)}`, out);
  } else if (!Object.is(base, variant) && JSON.stringify(base) !== JSON.stringify(variant)) {
    out.push({ pointer, value: variant });
  }
  return out;
}

/** Applies a leaf overlay to a copy of `base`. */
function applyOverlay(base: Json, replacements: Json[]): Json {
  const copy = structuredClone(base);
  for (const { pointer, value } of replacements) {
    const keys = (pointer as string).split("/").slice(1).map((key) => key.replaceAll("~1", "/").replaceAll("~0", "~"));
    const last = keys.pop()!;
    keys.reduce((item: Json, key: string) => item[key], copy)[last] = value;
  }
  return copy;
}

describe("the witness requests (§5.5)", () => {
  it("writes each witness shape's request and the substitute; stale fails", async () => {
    const outDir = process.env.OPENPIPESTRESS_WRITE_EXACT_PACKAGE_REQUESTS;
    const files: [string, string][] = [];
    let straight: Json = null;
    for (const item of SHAPES) {
      const session = await solve(item);
      const request = await assembleReportPackageRequestBelowOutputGate(packageInput(session), ALL_IDS) as Json;
      expect(request.result_documents[0].result_envelope.producer.semantic_contract_id).toBe(session.result.producer!.semantic_contract_id);
      expect(request.report.exact_route.admission.native_job_id).toBe(deliveredMechanicsResultJob(session.result));
      // No rule-check run in the witness inputs (§1.7, §5.5).
      expect(request.report.user_rule_checks).toBeNull();
      expect(request.rule_check_aggregate).toBeNull();
      // buildReportPackageRequest refuses the same input at its cell.
      await expect(buildReportPackageRequest(packageInput(session))).rejects.toThrow(/^PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE: /);
      if (item.name === "pressure_2_straight") straight = request;
      files.push([`${item.name}.json`, witnessFile(item, session, request)]);
    }
    const substitute = await substituteRequest(straight);
    expect(await canonicalSha256Hex(substitute.request.source_results[0])).not.toBe(await canonicalSha256Hex(straight.source_results[0]));
    expect(substitute.request.report.exact_route.admission.received_result_sha256).not.toBe(straight.report.exact_route.admission.received_result_sha256);
    // The substitute is committed as leaf replacements over the straight witness's request
    // (Codex r1 finding 5); the overlay reproduces it exactly.
    expect(substitute.session.intakeText).toBe(JSON.parse(files[0][1]).exact_intake_text);
    expect(await canonicalJsonString(substitute.session.inputManifest.manifest)).toBe(await canonicalJsonString(JSON.parse(files[0][1]).input_manifest));
    const replacements = leafOverlay(straight, substitute.request);
    expect(applyOverlay(straight, replacements)).toEqual(substitute.request);
    files.push(["pressure_2_straight_substitute.overlay.json", `${JSON.stringify({
      schema_version: "1.0.0",
      generator: GENERATOR,
      description: "W-11 slice (b) same-run substitute (DEL-08-01 Design exact-route-report-sections.md sections 5.5 and 7): leaf replacements over the request of pressure_2_straight.json, the straight source with one nodal rotation row changed, solved, recorded and assembled by the same builders so that every hash in the document, report and handoff records is rebound, keeping the genuine request's native_job_id and received_bytes_sha256. Its model, input manifest and intake text are the straight witness's.",
      base: "pressure_2_straight.json",
      substituted_row: substitute.row,
      replacements
    })}\n`]);
    if (outDir) {
      mkdirSync(outDir, { recursive: true });
      for (const [name, content] of files) writeFileSync(resolve(outDir, name), content);
      return;
    }
    for (const [name, content] of files) {
      expect(text(`${WITNESS_DIR}/${name}`) === content, `${WITNESS_DIR}/${name} is stale; regenerate it with OPENPIPESTRESS_WRITE_EXACT_PACKAGE_REQUESTS=<dir>`).toBe(true);
    }
  }, 120_000);
});
