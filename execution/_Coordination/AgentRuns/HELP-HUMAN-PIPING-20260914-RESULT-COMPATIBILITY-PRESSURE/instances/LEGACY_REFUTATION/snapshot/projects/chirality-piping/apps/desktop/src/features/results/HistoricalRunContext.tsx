import { useState } from "react";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, ModelHashEvidence, ProjectEnvelopeHashEvidence } from "../../types";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { verifyAnalysisRunRecord } from "../../services/analysisRunCompatibility";
import { bindSourceResultDimensions } from "../../services/previewService";
import { ResultsPanel } from "./ResultsPanel";

// Transient evidence only. Saved run references do not contain the historical
// input-manifest payload, so reopening cannot establish a current solve basis.
export type HistoricalRunContext = {
  designation: "historical_saved_run";
  mechanicsResult: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  modelHash: ModelHashEvidence | null;
  envelopeHash: ProjectEnvelopeHashEvidence | null;
  envelopePayloadHash: string | null;
  findings: string[];
  runId: string;
};

function legacyDesktopJson(value: unknown): string {
  function sort(item: any): any {
    if (Array.isArray(item)) return item.map(sort);
    if (item && typeof item === "object") return Object.fromEntries(Object.entries(item).sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, sort(child)]));
    return item;
  }
  return JSON.stringify(sort(value));
}

async function legacyDesktopDigest(value: unknown): Promise<string> {
  const bytes = new TextEncoder().encode(legacyDesktopJson(value));
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

type LegacyDesktopVerification = { result: "match" | "mismatch" | "unverifiable"; record: "match" | "mismatch" | "unverifiable" };

async function verifyLegacyDesktopAnalysis(record: AnalysisRunEnvelope, received: MechanicsResult): Promise<LegacyDesktopVerification> {
  try {
    const run = record.analysis_run;
    const canonicalizations = [
      ...(run.hashes ?? []),
      ...(run.result_refs ?? []).flatMap((item) => item.hash_refs ?? []),
    ].map((claim) => claim.canonicalization);
    const isDesktopProfile =
      canonicalizations.length > 0 &&
      canonicalizations.every((label) => label === "rfc8785_jcs") &&
      !("created_at" in run) && !("solver_version" in run) && !("provenance" in run);
    // The full Python 0.1 producer used sorted compact Python JSON. Once its
    // numbers have crossed JSON into JavaScript, integer-vs-float spelling
    // (1 versus 1.0) is irrecoverable, so it must not be replayed as desktop.
    if (!isDesktopProfile) return { result: "unverifiable", record: "unverifiable" };
    const enriched = bindSourceResultDimensions(received);
    const resultClaim = run.hashes.filter((hash) => hash.payload_scope === "result_envelope");
    const recordClaim = run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
    const manifestRefs = run.reproducibility?.input_manifest_refs ?? [];
    const manifestHashes = run.reproducibility?.input_manifest_hashes ?? [];
    const ruleStatus = run.analysis_status.find((status) => ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"].includes(status));
    let resultStatus: LegacyDesktopVerification["result"] = "match";
    let recordStatus: LegacyDesktopVerification["record"] = "match";
    const validClaim = (claim: any, scope: string, type: string, ref: string) => claim.algorithm === "sha256" && claim.canonicalization === "rfc8785_jcs" && claim.payload_scope === scope && claim.payload_ref?.object_type === type && claim.payload_ref?.ref === ref && /^[0-9a-f]{64}$/.test(claim.value);
    if (resultClaim.length !== 1 || run.result_refs.length !== enriched.results.length) resultStatus = "unverifiable";
    else if (!validClaim(resultClaim[0], "result_envelope", "ResultEnvelope", `result-envelope:${received.run_id}`) || resultClaim[0].value !== await legacyDesktopDigest(enriched)) resultStatus = "mismatch";
    if (resultStatus === "match") {
      const sortedRows = enriched.results.slice().sort((left, right) => left.id.localeCompare(right.id));
      for (const [index, row] of sortedRows.entries()) {
        const ref = run.result_refs[index];
        const claim = ref?.hash_refs?.[0];
        if (!claim || ref.result_ref?.ref !== row.id || !validClaim(claim, "result_value", "Result", row.id) || claim.value !== await legacyDesktopDigest(row)) { resultStatus = "mismatch"; break; }
      }
    }
    if (recordClaim.length !== 1 || manifestRefs.length !== 1 || manifestHashes.length !== 1 || !ruleStatus) return { result: resultStatus, record: "unverifiable" };
    if (!validClaim(recordClaim[0], "analysis_run_record", "AnalysisRun", run.run_id)) recordStatus = "mismatch";
    const manifestClaim = manifestHashes[0];
    if (manifestClaim.payload_ref?.ref !== manifestRefs[0].ref || manifestClaim.payload_scope !== "input_manifest" || typeof manifestClaim.value !== "string") return { result: resultStatus, record: "unverifiable" };
    const payload = {
      run_id: received.run_id,
      model_ref: received.model_ref,
      status: { ...received.status, rule_check: ruleStatus },
      load_basis_refs: run.load_basis_refs,
      result_ids: enriched.results.map((row) => row.id).sort(),
      diagnostic_ids: enriched.diagnostics.map((item) => item.id ?? "diagnostic:unknown").sort(),
      input_manifest_ref: manifestRefs[0],
      input_manifest_sha256: manifestClaim.value,
      result_dimensions: enriched.results.map((row) => ({ result_id: row.id, dimension: row.dimension })).sort((left, right) => left.result_id.localeCompare(right.result_id)),
    };
    if (recordStatus === "match" && recordClaim[0].value !== await legacyDesktopDigest(payload)) recordStatus = "mismatch";
    return { result: resultStatus, record: recordStatus };
  } catch {
    return { result: "unverifiable", record: "unverifiable" };
  }
}

export async function buildHistoricalRunContext(opened: LocalProjectEnvelope): Promise<HistoricalRunContext | null> {
  const mechanicsResult = opened.mechanics_result ?? null;
  const analysisRun = opened.analysis_run ?? null;
  if (!mechanicsResult && !analysisRun) return null;
  const findings = ["HISTORICAL_INPUT_MANIFEST_MISSING"];
  const record = analysisRun && typeof analysisRun === "object" && analysisRun.analysis_run && typeof analysisRun.analysis_run === "object" ? analysisRun.analysis_run : null;
  if (analysisRun && (!record || !Array.isArray(record.hashes) || !Array.isArray(record.result_refs) || typeof record.run_id !== "string" || typeof record.model_state_ref?.ref !== "string")) findings.push("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
  if (mechanicsResult && mechanicsResult.model_ref !== opened.model.project.id) findings.push("HISTORICAL_MODEL_REF_MISMATCH");
  const expectedStateRef = `state:${opened.model.project.id}:preview`;
  if (record && record.model_state_ref?.ref !== expectedStateRef) findings.push("HISTORICAL_MODEL_STATE_REF_MISMATCH");
  if (!mechanicsResult || !record) findings.push("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
  if (mechanicsResult && record && mechanicsResult.run_id !== record.run_id) findings.push("HISTORICAL_RUN_REF_MISMATCH");
  const modelHash = opened.model_hash ?? null;
  if (!modelHash) findings.push("HISTORICAL_MODEL_HASH_MISSING");
  if (!opened.project_envelope_hash) findings.push("HISTORICAL_ENVELOPE_HASH_MISSING");
  let envelopePayloadHash: string | null = null;
  let legacyVerification: LegacyDesktopVerification | null = null;
  try {
    const recomputedModel = await computeModelHash(opened.model);
    if (modelHash && (modelHash.value !== recomputedModel?.value || modelHash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_MODEL_HASH_MISMATCH");
    const recomputedEnvelope = await computeProjectEnvelopeHash({ model: opened.model, editor_intents: opened.editor_intents ?? [], proposal: opened.proposal ?? null, selected_review_target: opened.selected_review_target ?? null, mechanics_result: mechanicsResult, analysis_run: analysisRun, model_hash: modelHash });
    envelopePayloadHash = recomputedEnvelope?.value ?? null;
    if (opened.project_envelope_hash && (opened.project_envelope_hash.value !== recomputedEnvelope?.value || opened.project_envelope_hash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    if (mechanicsResult) {
      const receivedScope = analysisRun?.schema_version === "0.2.0" ? "received_result" : "result_envelope";
      const storedResultHash = Array.isArray(record?.hashes) ? record.hashes.find((hash) => hash && typeof hash === "object" && hash.payload_scope === receivedScope) : null;
      if (!storedResultHash) findings.push("HISTORICAL_RESULT_HASH_MISSING");
      else if (analysisRun?.schema_version === "0.2.0" && ((typeof storedResultHash.value === "string" ? storedResultHash.value.replace(/^sha256:/, "") : null) !== await canonicalSha256HexCheckedV1(mechanicsResult) || storedResultHash.payload_ref?.ref !== `result-envelope:${mechanicsResult.run_id}`)) findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
      else if (analysisRun?.schema_version === "0.1.0") {
        legacyVerification = await verifyLegacyDesktopAnalysis(analysisRun, mechanicsResult);
        if (legacyVerification.result === "mismatch") findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
        if (legacyVerification.result === "unverifiable") findings.push("HISTORICAL_RESULT_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
      }
    }
    if (analysisRun?.schema_version === "0.2.0") {
      const verification = await verifyAnalysisRunRecord(analysisRun);
      if (verification === "mismatch") findings.push("HISTORICAL_ANALYSIS_HASH_MISMATCH");
      if (verification === "unverifiable") findings.push("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE");
    } else if (analysisRun?.schema_version === "0.1.0" && mechanicsResult) {
      legacyVerification ??= await verifyLegacyDesktopAnalysis(analysisRun, mechanicsResult);
      if (legacyVerification.record === "mismatch") findings.push("HISTORICAL_ANALYSIS_HASH_MISMATCH");
      if (legacyVerification.record === "unverifiable") findings.push("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE_LEGACY_PREIMAGE");
    }
  } catch {
    findings.push("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
  }
  return { designation: "historical_saved_run", mechanicsResult, analysisRun, modelHash, envelopeHash: opened.project_envelope_hash ?? null, envelopePayloadHash, findings, runId: mechanicsResult?.run_id ?? (typeof record?.run_id === "string" ? record.run_id : "unknown saved run") };
}

export function HistoricalRunPanel({ context }: { context: HistoricalRunContext }) {
  const [selectedResultId, setSelectedResultId] = useState<string | null>(null);
  return <section className="panel" aria-label="Historical saved run" data-testid="historical-run-context">
    <div className="panel-title">Historical saved run</div>
    <p>Saved evidence for {context.runId}; mechanics={context.mechanicsResult?.status.mechanics ?? "missing"}. Run a fresh solve to establish current results.</p>
    <p>Historical results cannot drive current overlays, rule checks, comparisons or report readiness.</p>
    <ul>{context.findings.map((finding) => <li key={finding}>{finding}</li>)}</ul>
    <ResultsPanel result={context.mechanicsResult} knowledge={null} analysisRun={null} selectedResultId={selectedResultId} onSelectResult={setSelectedResultId} />
  </section>;
}
