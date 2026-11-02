import { isRetainedRoute } from "./numericalResultQuality";
import { sourceContract } from "./resultReader";
import { numericalResultStanding } from "./resultStanding";
import { validateRetainedPrecision } from "./retainedPrecisionReader";
import { useState } from "react";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, ModelHashEvidence, ProjectEnvelopeHashEvidence, PreviewModel } from "../../types";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import type { AnalysisRunRead } from "../../services/analysisRunService";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import { verifyAnalysisRunRecord, validateAnalysisRunV03, ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH, ANALYSIS_RUN_SCHEMA_VERSION_RETIRED, ANALYSIS_RUN_V03 } from "../../services/analysisRunCompatibility";
import type { BundledMechanicsReference } from "../../services/previewService";
import { ResultsPanel } from "./ResultsPanel";
import type { NativeResult } from "../../services/nativeResult";
import { restoredNativeResult, type NativeProjectEnvelope, type NativeRunRead, type RestoredRun } from "../../services/nativeReopen";

// Transient evidence only. Saved run references do not contain the historical
// input-manifest payload, so reopening cannot establish a current solve basis.
// A recorded run (W-12 S2) stores its manifest and names a stored state: its
// binding to that state is checked, but it establishes no current solve either.
export type HistoricalRunContext = {
  designation: "historical_saved_run" | "bundled_reference" | "recorded_run";
  referenceModel?: PreviewModel;
  referenceProvenance?: BundledMechanicsReference["provenance"];
  rawMechanicsResult: unknown;
  rawAnalysisRun: unknown;
  /** The rows the panel reads: the whole result (browser preview, bundled reference), or (C1
   * slice 6) the restored run's native handle, read through S and bounded pages. */
  mechanicsResult: MechanicsResult | NativeResult | null;
  /** C1 slice 6: the stored run held natively (desktop). Its raw values never enter the webview;
   * `rawMechanicsResult`/`rawAnalysisRun` are null. A project save re-saves it from the handle
   * (`saveRestoredLocalProject`); an approved C2 whole-value consumer loads it through the bounded
   * copy (`loadRestoredRun`). */
  restored?: RestoredRun;
  analysisRun: AnalysisRunEnvelope | null;
  modelHash: ModelHashEvidence | null;
  envelopeHash: ProjectEnvelopeHashEvidence | null;
  envelopePayloadHash: string | null;
  findings: string[];
  runId: string;
};

const RESULT_DIMENSIONS = new Set([
  "dimensionless", "length", "angle", "force", "moment", "stress", "area", "section_modulus",
  "second_moment_area", "ratio", "time", "temperature", "pressure", "linear_stiffness", "rotational_stiffness"
]);

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value && typeof value === "object" && !Array.isArray(value));
}

/** Validate every shape ResultsPanel and its interpretation helpers can read. */
function isRenderableMechanicsResult(value: unknown): value is MechanicsResult {
  if (!isRecord(value) || typeof value.schema_version !== "string" || typeof value.document_kind !== "string"
    || typeof value.run_id !== "string" || typeof value.model_ref !== "string" || !isRecord(value.status)
    || typeof value.status.mechanics !== "string" || typeof value.status.rule_check !== "string" || !isRecord(value.summary)
    || !Array.isArray(value.results) || !Array.isArray(value.diagnostics)) return false;
  const rowsAreSafe = value.results.every((row) => {
    if (!isRecord(row) || typeof row.id !== "string" || typeof row.kind !== "string"
      || typeof row.value !== "number" || !Number.isFinite(row.value) || typeof row.unit !== "string"
      || typeof row.entity_ref !== "string") return false;
    if ("dimension" in row && row.dimension !== undefined
      && (typeof row.dimension !== "string" || !RESULT_DIMENSIONS.has(row.dimension))) return false;
    if ("basis_ref" in row && row.basis_ref !== undefined
      && (!isRecord(row.basis_ref) || typeof row.basis_ref.ref_type !== "string" || typeof row.basis_ref.ref_id !== "string")) return false;
    if ("source_result_refs" in row && row.source_result_refs !== undefined
      && (!Array.isArray(row.source_result_refs) || row.source_result_refs.some((item) => typeof item !== "string"))) return false;
    if ("metadata" in row && row.metadata !== undefined) {
      const metadata: unknown = row.metadata;
      if (!isRecord(metadata)
        || ["component", "coordinate_system", "location", "basis", "sign_convention"].some((key) => typeof metadata[key] !== "string")) return false;
    }
    return true;
  });
  const diagnosticsAreSafe = value.diagnostics.every((item) => isRecord(item)
    && typeof item.code === "string" && ["info", "warning", "error", "blocking"].includes(String(item.severity))
    && typeof item.message === "string" && (!("id" in item) || item.id === undefined || typeof item.id === "string")
    && (!("affected_refs" in item) || item.affected_refs === undefined
      || (Array.isArray(item.affected_refs) && item.affected_refs.every((ref) => typeof ref === "string"))));
  return rowsAreSafe && diagnosticsAreSafe;
}

function isHistoricalMechanicsEvidence(value: unknown): value is MechanicsResult {
  return isRecord(value) && typeof value.run_id === "string" && typeof value.model_ref === "string"
    && isRecord(value.status) && typeof value.status.mechanics === "string" && typeof value.status.rule_check === "string"
    && Array.isArray(value.results) && Array.isArray(value.diagnostics);
}

/** A named refusal: the record validator's codes, and those of the readers it calls,
 * are upper-case words joined by `_` or `-` (`ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH`,
 * `ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH`, Rust's `error_json` codes), alone or followed
 * by a detail (`SOURCE_UNIT_CONTRADICTION: <kind> / <unit>`). A runtime's own message is not. */
const NAMED_REFUSAL = /^[A-Z][A-Z0-9]*(?:[_-][A-Z0-9]+)+(?![\w-])/;

async function sameSavedReceipt(copy: unknown, receipt: unknown): Promise<boolean> {
  // An absent copy is refused by the checked profile, so it never matches.
  try { return await canonicalSha256HexCheckedV1(copy) === await canonicalSha256HexCheckedV1(receipt); } catch { return false; }
}

/** A context over the whole result (the browser preview's store, the legacy whole-value reads). */
export type WholeHistoricalRunContext = HistoricalRunContext & { mechanicsResult: MechanicsResult | null };

export async function buildHistoricalRunContext(opened: LocalProjectEnvelope): Promise<WholeHistoricalRunContext | null> {
  const rawMechanicsResult: unknown = opened.mechanics_result;
  const rawAnalysisRun: unknown = opened.analysis_run;
  const hasReceivedMechanics = rawMechanicsResult !== null && rawMechanicsResult !== undefined;
  const hasReceivedAnalysis = rawAnalysisRun !== null && rawAnalysisRun !== undefined;
  if (!hasReceivedMechanics && !hasReceivedAnalysis) return null;
  const evidenceResult = isHistoricalMechanicsEvidence(rawMechanicsResult) ? rawMechanicsResult : null;
  const mechanicsResult = isRenderableMechanicsResult(rawMechanicsResult) ? rawMechanicsResult : null;
  const analysisRun = rawAnalysisRun as AnalysisRunEnvelope | null;
  const findings = ["HISTORICAL_INPUT_MANIFEST_MISSING"];
  if (mechanicsResult) findings.push(...numericalResultStanding(mechanicsResult, opened.model).findings);
  if (hasReceivedMechanics && !mechanicsResult) findings.push("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
  const record = analysisRun && typeof analysisRun === "object" && analysisRun.analysis_run && typeof analysisRun.analysis_run === "object" ? analysisRun.analysis_run : null;
  if (analysisRun && (!record || !Array.isArray(record.hashes) || !Array.isArray(record.result_refs) || typeof record.run_id !== "string" || typeof record.model_state_ref?.ref !== "string")) findings.push("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
  // U6d reopen: a saved successor is revalidated by the accepted reader without an
  // invocation (its first code is a finding), and the AnalysisRun's copy must equal
  // its receipt. Nothing registers here, so standing stays needs_recompute.
  if (mechanicsResult && isRetainedRoute(sourceContract(mechanicsResult))) {
    try { await validateRetainedPrecision(mechanicsResult); } catch (error) { findings.push((error as Error).message); }
    if (record && !(await sameSavedReceipt(record.retained_precision, mechanicsResult.retained_precision))) findings.push(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
  }
  if (evidenceResult && evidenceResult.model_ref !== opened.model.project.id) findings.push("HISTORICAL_MODEL_REF_MISMATCH");
  const expectedStateRef = `state:${opened.model.project.id}:preview`;
  if (record && record.model_state_ref?.ref !== expectedStateRef) findings.push("HISTORICAL_MODEL_STATE_REF_MISMATCH");
  if (!evidenceResult || !record) findings.push("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
  if (evidenceResult && record && evidenceResult.run_id !== record.run_id) findings.push("HISTORICAL_RUN_REF_MISMATCH");
  const modelHash = opened.model_hash ?? null;
  if (!modelHash) findings.push("HISTORICAL_MODEL_HASH_MISSING");
  if (!opened.project_envelope_hash) findings.push("HISTORICAL_ENVELOPE_HASH_MISSING");
  let envelopePayloadHash: string | null = null;
  // One current analysis-run version: a saved 0.1.0 or 0.2.0 record is refused by
  // name and not verified (re-run the analysis to record a current run).
  const savedVersion = analysisRun?.schema_version;
  const currentRecord = savedVersion === ANALYSIS_RUN_V03;
  if (analysisRun && ["0.1.0", "0.2.0"].includes(savedVersion as string)) findings.push(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
  else if (analysisRun && record && !currentRecord) findings.push("HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED");
  try {
    const recomputedModel = await computeModelHash(opened.model);
    if (modelHash && (modelHash.value !== recomputedModel?.value || modelHash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_MODEL_HASH_MISMATCH");
    const recomputedEnvelope = await computeProjectEnvelopeHash({ model: opened.model, editor_intents: opened.editor_intents ?? [], proposal: opened.proposal ?? null, selected_review_target: opened.selected_review_target ?? null, mechanics_result: rawMechanicsResult as MechanicsResult | null, analysis_run: rawAnalysisRun as AnalysisRunEnvelope | null, model_hash: modelHash });
    envelopePayloadHash = recomputedEnvelope?.value ?? null;
    if (opened.project_envelope_hash && (opened.project_envelope_hash.value !== recomputedEnvelope?.value || opened.project_envelope_hash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    if (evidenceResult && (currentRecord || !analysisRun)) {
      const storedResultHash = Array.isArray(record?.hashes) ? record.hashes.find((hash) => hash && typeof hash === "object" && hash.payload_scope === "received_result") : null;
      if (!storedResultHash) findings.push("HISTORICAL_RESULT_HASH_MISSING");
      else if ((typeof storedResultHash.value === "string" ? storedResultHash.value.replace(/^sha256:/, "") : null) !== await canonicalSha256HexCheckedV1(evidenceResult) || storedResultHash.payload_ref?.ref !== `result-envelope:${evidenceResult.run_id}`) findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
    }
    if (currentRecord) {
      const verification = await verifyAnalysisRunRecord(analysisRun!);
      if (verification === "mismatch") findings.push("HISTORICAL_ANALYSIS_HASH_MISMATCH");
      if (verification === "unverifiable") findings.push("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE");
    }
  } catch {
    findings.push("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
  }
  // B7b: a current (0.3.0) record is read against its saved source by the record
  // validator, as on the record's own path (its table, embedded namespaces, receipt and
  // row bindings). The saved load basis, as saved, is checked for well-formedness and for
  // covering its source's rows and cases; no saved manifest states which basis the solve
  // requested. A malformed list is refused as malformed (ANALYSIS_LOAD_BASIS_INVALID); for a
  // record without the list none is passed, so the validator derives the basis from the
  // source's rows and names the absent list (ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH). The current
  // model is never read for it. Its refusal is a finding only: reopening displays
  // evidence and establishes nothing, so nothing here changes standing, rows or bytes.
  if (currentRecord && mechanicsResult && record) {
    try { await validateAnalysisRunV03(analysisRun!, mechanicsResult, record.load_basis_refs); }
    catch (error) {
      // A named refusal is shown as the validator states it. Anything else (a malformed
      // record read where the validator does not guard it) is one named finding, never
      // the runtime's own text.
      const finding = !(error instanceof Error) ? "HISTORICAL_ANALYSIS_VALIDATION_UNAVAILABLE" : NAMED_REFUSAL.test(error.message) ? error.message : "HISTORICAL_ANALYSIS_EVIDENCE_INVALID";
      if (!findings.includes(finding)) findings.push(finding);
    }
  }
  return { designation: "historical_saved_run", rawMechanicsResult, rawAnalysisRun, mechanicsResult, analysisRun, modelHash, envelopeHash: opened.project_envelope_hash ?? null, envelopePayloadHash, findings, runId: evidenceResult?.run_id ?? (typeof record?.run_id === "string" ? record.run_id : "unknown saved run") };
}

/** A recorded run read from the store (W-12 S2), checked as a saved run is, with
 * two differences: its stored input manifest is present, so no
 * `HISTORICAL_INPUT_MANIFEST_MISSING`; and its `model_state_ref` is a real state
 * ref, accepted only when it names the stored state it was read with and the
 * manifest's model payload hashes to that state's model hash (the record
 * validator's recorded form). Any other mismatch is still flagged. Reading it
 * establishes nothing: no standing, overlay, rule check or report readiness. */
export async function buildRecordedRunContext(read: AnalysisRunRead): Promise<WholeHistoricalRunContext> {
  const findings: string[] = [];
  const parse = (text: string, finding: string): unknown => {
    try { return JSON.parse(text); } catch { findings.push(finding); return null; }
  };
  const rawMechanicsResult = parse(read.result_json, "HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
  const manifest = parse(read.input_manifest_canonical, "RECORDED_RUN_INPUT_MANIFEST_MALFORMED") as CurrentSessionInputManifest | null;
  const rawAnalysisRun: unknown = read.record;
  const analysisRun = read.record ?? null;
  const evidenceResult = isHistoricalMechanicsEvidence(rawMechanicsResult) ? rawMechanicsResult : null;
  const mechanicsResult = isRenderableMechanicsResult(rawMechanicsResult) ? rawMechanicsResult : null;
  if (rawMechanicsResult !== null && !mechanicsResult && !findings.includes("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED")) findings.push("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
  const model = manifest?.model_basis?.model_payload ?? null;
  if (mechanicsResult && model) findings.push(...numericalResultStanding(mechanicsResult, model).findings);
  const record = analysisRun && typeof analysisRun === "object" && analysisRun.analysis_run && typeof analysisRun.analysis_run === "object" ? analysisRun.analysis_run : null;
  if (!record || !Array.isArray(record.hashes) || !Array.isArray(record.result_refs) || typeof record.run_id !== "string" || typeof record.model_state_ref?.ref !== "string") findings.push("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
  if (mechanicsResult && isRetainedRoute(sourceContract(mechanicsResult))) {
    try { await validateRetainedPrecision(mechanicsResult); } catch (error) { findings.push((error as Error).message); }
    if (record && !(await sameSavedReceipt(record.retained_precision, mechanicsResult.retained_precision))) findings.push(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
  }
  // The state ref names the stored state this run was read with, and nothing else.
  const stateRef = record?.model_state_ref;
  if (!stateRef || stateRef.object_type !== "ModelState" || stateRef.ref !== read.state.state_id || read.run.state_id !== read.state.state_id) findings.push("HISTORICAL_MODEL_STATE_REF_MISMATCH");
  if (evidenceResult && evidenceResult.model_ref !== read.state.project_id) findings.push("HISTORICAL_MODEL_REF_MISMATCH");
  if (!evidenceResult || !record) findings.push("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
  if (evidenceResult && record && (evidenceResult.run_id !== record.run_id || read.run.producer_run_id !== record.run_id)) findings.push("HISTORICAL_RUN_REF_MISMATCH");
  const savedVersion = analysisRun?.schema_version;
  const currentRecord = savedVersion === ANALYSIS_RUN_V03;
  if (analysisRun && ["0.1.0", "0.2.0"].includes(savedVersion as string)) findings.push(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
  else if (analysisRun && record && !currentRecord) findings.push("HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED");
  let modelHash: ModelHashEvidence | null = null;
  try {
    if (model) {
      modelHash = await computeModelHash(model);
      if (modelHash?.value !== read.state.model_hash) findings.push("RECORDED_RUN_MODEL_STATE_HASH_MISMATCH");
    }
    if (manifest && record && record.reproducibility?.input_manifest_hashes?.[0]?.value !== await canonicalSha256Hex(manifest)) findings.push("RECORDED_RUN_INPUT_MANIFEST_HASH_MISMATCH");
    if (evidenceResult && currentRecord) {
      const storedResultHash = Array.isArray(record?.hashes) ? record.hashes.find((hash) => hash && typeof hash === "object" && hash.payload_scope === "received_result") : null;
      if (!storedResultHash) findings.push("HISTORICAL_RESULT_HASH_MISSING");
      else if (storedResultHash.value !== await canonicalSha256HexCheckedV1(evidenceResult) || storedResultHash.payload_ref?.ref !== `result-envelope:${evidenceResult.run_id}`) findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
    }
    if (currentRecord) {
      const verification = await verifyAnalysisRunRecord(analysisRun!);
      if (verification === "mismatch") findings.push("HISTORICAL_ANALYSIS_HASH_MISMATCH");
      if (verification === "unverifiable") findings.push("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE");
      if (verification === "match" && record && !record.hashes.some((hash) => hash.payload_scope === "analysis_run_record" && hash.value === read.run.run_record_hash)) findings.push("RECORDED_RUN_RECORD_HASH_MISMATCH");
    }
  } catch {
    findings.push("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
  }
  // The record validator's recorded form: the state ref, the manifest's hash and
  // project, its model payload against the state's model hash, and every member the
  // stored manifest fixes (solver, settings, units, load basis, references), all
  // derived from the manifest. No basis is passed: the record's own list is never
  // its warrant.
  if (currentRecord && mechanicsResult && record && manifest) {
    try { await validateAnalysisRunV03(analysisRun!, mechanicsResult, undefined, { stateId: read.state.state_id, modelHash: read.state.model_hash, inputManifest: manifest }); }
    catch (error) {
      const finding = !(error instanceof Error) ? "HISTORICAL_ANALYSIS_VALIDATION_UNAVAILABLE" : NAMED_REFUSAL.test(error.message) ? error.message : "HISTORICAL_ANALYSIS_EVIDENCE_INVALID";
      if (!findings.includes(finding)) findings.push(finding);
    }
  }
  return {
    designation: "recorded_run", rawMechanicsResult, rawAnalysisRun, mechanicsResult, analysisRun, modelHash,
    envelopeHash: null, envelopePayloadHash: null, findings,
    runId: evidenceResult?.run_id ?? (typeof record?.run_id === "string" ? record.run_id : "unknown recorded run")
  };
}

/** The findings of a recorded run that concern its record and its binding (not its numerical standing). */
export function recordedRunBindingFindings(context: HistoricalRunContext): string[] {
  return context.findings.filter((finding) => /^(HISTORICAL_|RECORDED_RUN_|ANALYSIS[_-])/.test(finding));
}

/** C1 slice 6: a project's saved run reopened into native custody. The findings are the native
 * port's (`analysis_run_v03_validate.rs`) of `buildHistoricalRunContext` on the stored texts, the
 * same list in the same order (`historicalReopenParity.test.ts`, `stored_custody_tests.rs`); a
 * renderable result is read through its restored handle. Null when the project stores no run. */
export async function buildNativeHistoricalRunContext(opened: NativeProjectEnvelope): Promise<HistoricalRunContext | null> {
  const run = opened.stored_run;
  if (run.state !== "restored") return null;
  return contextOfRestored("historical_saved_run", run, opened.project_envelope_hash ?? null, run.findings.envelope_payload_hash, opened.model_hash ?? null);
}

/** C1 slice 6: a recorded run reopened into native custody after the store's verified read; the
 * findings are the native port's of `buildRecordedRunContext`. A record check shows findings and the
 * record's rule-pack references, never rows, so no S is read (`mechanicsResult` null); its caller
 * releases the handle once the check is built. */
export function buildNativeRecordedRunContext(read: NativeRunRead): HistoricalRunContext {
  const found = read.restored.findings;
  return {
    designation: "recorded_run", rawMechanicsResult: null, rawAnalysisRun: null, mechanicsResult: null,
    analysisRun: null, restored: read.restored, modelHash: (found.model_hash ?? null) as ModelHashEvidence | null,
    envelopeHash: null, envelopePayloadHash: null, findings: [...found.findings], runId: found.run_id
  };
}

async function contextOfRestored(
  designation: "historical_saved_run",
  restored: RestoredRun,
  envelopeHash: ProjectEnvelopeHashEvidence | null,
  envelopePayloadHash: string | null,
  modelHash: ModelHashEvidence | null
): Promise<HistoricalRunContext> {
  const found = restored.findings;
  const mechanicsResult = found.renderable ? await restoredNativeResult(restored) : null;
  return {
    designation, rawMechanicsResult: null, rawAnalysisRun: null, mechanicsResult,
    analysisRun: null, restored, modelHash, envelopeHash, envelopePayloadHash,
    findings: [...found.findings], runId: found.run_id
  };
}

/** A bundled example has its own model context and no current run evidence. */
export function buildBundledReferenceContext(reference: BundledMechanicsReference): HistoricalRunContext {
  return {
    designation: "bundled_reference", referenceModel: reference.model,
    referenceProvenance: reference.provenance,
    rawMechanicsResult: reference.source, mechanicsResult: reference.source,
    rawAnalysisRun: null, analysisRun: null, modelHash: null,
    envelopeHash: null, envelopePayloadHash: null,
    findings: ["BUNDLED_REFERENCE_ONLY", "NO_CURRENT_MODEL_INVOCATION"],
    runId: reference.source.run_id,
  };
}

export function HistoricalRunPanel({ context }: { context: HistoricalRunContext }) {
  const [selectedResultId, setSelectedResultId] = useState<string | null>(null);
  const isReference = context.designation === "bundled_reference";
  const title = isReference ? "Bundled reference — not a solve for the current model" : "Historical saved run";
  return <section className="panel" aria-label={title} data-testid="historical-run-context">
    <div className="panel-title">{title}</div>
    {isReference ? <>
      <p>{context.referenceProvenance?.notice}</p>
      <p>Reference model: {context.referenceModel?.project.id}; retained run: {context.runId}. The current model and its edit history are unchanged.</p>
    </> : <p>Saved evidence for {context.runId}; mechanics={context.mechanicsResult?.status.mechanics ?? "missing"}. Run a fresh solve to establish current results.</p>}
    <p>{isReference ? "Bundled reference" : "Historical"} results cannot drive current overlays, rule checks, comparisons or report readiness.</p>
    <ul>{context.findings.map((finding) => <li key={finding}>{finding}</li>)}</ul>
    <ResultsPanel result={context.mechanicsResult} knowledge={null} analysisRun={null} selectedResultId={selectedResultId} onSelectResult={setSelectedResultId} />
  </section>;
}
