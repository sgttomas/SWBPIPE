import { FileText } from "lucide-react";
import type { AgentProposal, AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

export function ReportPanel({
  model,
  result,
  analysisRun,
  proposal
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  proposal: AgentProposal | null;
}) {
  const resultRefs = result ? selectedResultRefs(result) : [];
  const diagnostics = result?.diagnostics ?? [];
  const run = analysisRun?.analysis_run;
  const resultHashCount = run?.result_refs.reduce((count, item) => count + item.hash_refs.length, 0) ?? 0;
  const envelopeHash = run?.hashes.find((item) => item.payload_scope === "result_envelope");
  const loadBasisRefs = run?.load_basis_refs?.map((item) => item.ref).join(", ") ?? "not generated";
  return (
    <section className="panel report-panel" aria-label="Report packet" data-testid="report-panel">
      <div className="panel-title">
        <FileText size={16} />
        Report Packet
      </div>
      {result ? (
        <div className="report-list" data-testid="report-packet-body">
          <ReportLine label="Model" value={result.model_ref} />
          <ReportLine label="Mechanics" value={result.status.mechanics.replaceAll("_", " ")} />
          <ReportLine label="Rule check" value={result.status.rule_check.replaceAll("_", " ")} />
          <ReportLine label="Professional acceptance" value={result.status.professional_acceptance.replaceAll("_", " ")} />
          <ReportLine label="Selected result refs" value={resultRefs.join(", ")} testId="report-selected-result-refs" />
          <ReportLine label="Diagnostics" value={`${diagnostics.length} computed finding${diagnostics.length === 1 ? "" : "s"}`} />
          {run ? (
            <>
              <ReportLine label="Analysis run" value={`${analysisRun.deliverable_id}; ${run.run_id}`} testId="report-analysis-run" />
              <ReportLine label="Load basis refs" value={loadBasisRefs} testId="report-load-basis-refs" />
              <ReportLine label="Run immutability" value={run.immutability_policy.mutation_policy.replaceAll("_", " ")} />
              <ReportLine label="Result hashes" value={`${resultHashCount} result value hash${resultHashCount === 1 ? "" : "es"}`} />
              <ReportLine label="Envelope hash" value={envelopeHash ? `${envelopeHash.algorithm}; ${envelopeHash.payload_scope}` : "not available"} />
              <ReportLine label="Run boundary" value={boundarySummary(run.professional_boundary)} />
            </>
          ) : null}
          <ReportLine label="Proposal" value={proposal?.proposal_id ?? "not generated"} />
          <ReportLine label="Boundary" value="technical preview only; human review required; no compliance or professional approval claim" />
        </div>
      ) : (
        <p className="muted">
          Run the bounded preview mechanics path to assemble a report packet from computed result and diagnostic IDs.
        </p>
      )}
      <small className="report-note">
        Uses invented or cleared preview data for {model.project.id}; private rule criteria and professional acceptance are not bundled.
      </small>
    </section>
  );
}

function ReportLine({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function selectedResultRefs(result: MechanicsResult): string[] {
  return [
    result.summary.max_displacement?.result_ref,
    result.summary.max_open_formula_stress?.result_ref,
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial:end-j")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:midspan:axial")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:quarter-1:shear-y")?.id,
    result.results.find((item) => item.id === "result:force:pipe-P-120:shear-y")?.id,
    result.results.find((item) => item.id === "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial")?.id,
    result.results.find((item) => item.id === "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y")?.id,
    result.results.find((item) => item.id === "result:stress:pipe-P-120:end-j:torsional-shear")?.id,
    result.results.find((item) => item.id === "result:stress:pipe-P-120:quarter-1:torsional-shear")?.id
  ].filter((value): value is string => Boolean(value));
}

function boundarySummary(boundary: Record<string, boolean>): string {
  if (
    boundary.human_review_required &&
    !boundary.software_makes_compliance_claim &&
    !boundary.software_makes_certification_claim &&
    !boundary.software_makes_sealing_claim &&
    !boundary.software_makes_approval_claim
  ) {
    return "human review required; no compliance, certification, sealing, or approval claim";
  }
  return "review boundary requires attention";
}
