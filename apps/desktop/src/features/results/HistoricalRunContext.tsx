import { useState } from "react";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, ModelHashEvidence, ProjectEnvelopeHashEvidence } from "../../types";
import { canonicalSha256Hex, computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
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

export async function buildHistoricalRunContext(opened: LocalProjectEnvelope): Promise<HistoricalRunContext | null> {
  const mechanicsResult = opened.mechanics_result ?? null;
  const analysisRun = opened.analysis_run ?? null;
  if (!mechanicsResult && !analysisRun) return null;
  const findings = ["HISTORICAL_INPUT_MANIFEST_MISSING"];
  const record = analysisRun && typeof analysisRun === "object" && analysisRun.analysis_run && typeof analysisRun.analysis_run === "object" ? analysisRun.analysis_run : null;
  if (analysisRun && (!record || !Array.isArray(record.hashes) || !Array.isArray(record.result_refs) || typeof record.run_id !== "string" || typeof record.model_state_ref?.ref !== "string")) findings.push("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
  if (mechanicsResult && mechanicsResult.model_ref !== opened.model.project.id) findings.push("HISTORICAL_MODEL_REF_MISMATCH");
  if (record && record.model_state_ref?.ref !== `state:${opened.model.project.id}:preview`) findings.push("HISTORICAL_MODEL_STATE_REF_MISMATCH");
  if (!mechanicsResult || !record) findings.push("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
  if (mechanicsResult && record && mechanicsResult.run_id !== record.run_id) findings.push("HISTORICAL_RUN_REF_MISMATCH");
  const modelHash = opened.model_hash ?? null;
  if (!modelHash) findings.push("HISTORICAL_MODEL_HASH_MISSING");
  if (!opened.project_envelope_hash) findings.push("HISTORICAL_ENVELOPE_HASH_MISSING");
  let envelopePayloadHash: string | null = null;
  try {
    const recomputedModel = await computeModelHash(opened.model);
    if (modelHash && (modelHash.value !== recomputedModel?.value || modelHash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_MODEL_HASH_MISMATCH");
    const recomputedEnvelope = await computeProjectEnvelopeHash({ model: opened.model, editor_intents: opened.editor_intents ?? [], proposal: opened.proposal ?? null, selected_review_target: opened.selected_review_target ?? null, mechanics_result: mechanicsResult, analysis_run: analysisRun, model_hash: modelHash });
    envelopePayloadHash = recomputedEnvelope?.value ?? null;
    if (opened.project_envelope_hash && (opened.project_envelope_hash.value !== recomputedEnvelope?.value || opened.project_envelope_hash.payload_ref !== opened.model.project.id)) findings.push("HISTORICAL_ENVELOPE_HASH_MISMATCH");
    if (mechanicsResult) {
      const storedResultHash = Array.isArray(record?.hashes) ? record.hashes.find((hash) => hash && typeof hash === "object" && hash.payload_scope === "result_envelope") : null;
      if (!storedResultHash) findings.push("HISTORICAL_RESULT_HASH_MISSING");
      // Analysis hashes bind dimensions; preserve raw saved rows everywhere else.
      else if ((typeof storedResultHash.value === "string" ? storedResultHash.value.replace(/^sha256:/, "") : null) !== await canonicalSha256Hex(bindSourceResultDimensions(mechanicsResult)) || storedResultHash.payload_ref?.ref !== `result-envelope:${mechanicsResult.run_id}`) findings.push("HISTORICAL_RESULT_HASH_MISMATCH");
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
