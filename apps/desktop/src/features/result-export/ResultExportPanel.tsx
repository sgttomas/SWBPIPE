import { useEffect, useState } from "react";
import { buildCurrentResultExport, type JsonObject } from "./resultExportAdapter";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { Download, FileJson } from "lucide-react";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, ObjectRef, PreviewModel } from "../../types";

export function ResultExportPanel({
  model,
  result,
  analysisRun,
  inputManifest
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  inputManifest?: CurrentSessionInputManifestEvidence | null;
}) {
  const [packet,setPacket] = useState<JsonObject|null>(null);
  const [finding,setFinding] = useState<string|null>(null);
  // Bind completed packet to the exact props so an edit cannot expose a stale
  // downloadable document while the asynchronous proof check is running.
  const [binding,setBinding] = useState<object[]|null>(null);
  const currentPacket = binding?.[0]===model && binding?.[1]===result && binding?.[2]===analysisRun && binding?.[3]===inputManifest ? packet : null;
  useEffect(()=>{let active=true;setPacket(null);setFinding(null);setBinding(null);
    if(result&&analysisRun)buildCurrentResultExport({model,result,analysisRun,inputManifest}).then(doc=>{if(active){setPacket(doc);setBinding([model,result,analysisRun,inputManifest!]);}}).catch(error=>{if(active)setFinding(String(error));});
    return ()=>{active=false;};
  },[model,result,analysisRun,inputManifest]);
  return (
    <section className="panel result-export-panel" aria-label="Result export audit" data-testid="result-export-panel">
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Result Export
      </div>
      {currentPacket ? (
        <>
          <div className="report-actions">
        <ControlledExportLink
              className="report-export-link"
              data-testid="result-export-link"
              download={`openpipestress-preview-results-${safeFileToken(currentPacket.result_envelope.run_ref.ref_id)}.json`}
              href={jsonDataHref(currentPacket)}
            >
              <Download size={14} aria-hidden="true" />
              Local result JSON
        </ControlledExportLink>
            <span data-testid="result-export-summary">
              available; rows={currentPacket.result_envelope.result_sets[0]?.values.length ?? 0}; sets=
              {currentPacket.result_envelope.result_sets.length}; diagnostics={currentPacket.result_envelope.diagnostics.length}
            </span>
          </div>
          <div className="report-list" data-testid="result-export-body">
            <ExportLine
              label="Format"
              value={`${currentPacket.export_format_status.baseline_format}; additional_formats=${currentPacket.export_format_status.additional_formats}`}
              testId="result-export-format"
            />
            <ExportLine
              label="State binding"
              value={`${currentPacket.result_envelope.model_ref.ref_id}; ${currentPacket.result_envelope.run_ref.ref_id}`}
              testId="result-export-state-binding"
            />
            <ExportLine
              label="Units"
              value={`${unitCount(currentPacket)} explicit units; dimensions=${dimensionSummary(currentPacket)}`}
              testId="result-export-units"
            />
            <ExportLine
              label="Unit witnesses"
              value={`count=${currentPacket.result_envelope.unit_preservation_witnesses.length}; conversion=${String(
                currentPacket.result_envelope.unit_preservation_witnesses.some((item: JsonObject) => item.conversion_performed)
              )}`}
              testId="result-export-unit-witnesses"
            />
            <ExportLine
              label="Reproducibility"
              value={`deterministic_ordering=${String(currentPacket.result_envelope.reproducibility.deterministic_ordering)}; run_hashes=${currentPacket.result_envelope.reproducibility.run_hashes.length}`}
              testId="result-export-reproducibility"
            />
            <ExportLine
              label="Boundary"
              value={boundarySummary(currentPacket.result_envelope.professional_boundary)}
              testId="result-export-boundary"
            />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="result-export-empty">
          {finding ?? "Run mechanics preview with valid Current input proof to assemble a local result JSON for review."}
        </p>
      )}
      <small className="report-note">
        Result export is a local technical-preview JSON envelope; stress-neutral CSV/JSON preview is available after a
        mechanics run, while public transport and local FEA package formats remain TBD.
      </small>
    </section>
  );
}

function ExportLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function unitCount(packet: JsonObject): number {
  return new Set(packet.result_envelope.result_sets.flatMap((set: JsonObject) => set.values.map((value: JsonObject) => value.unit))).size;
}

function dimensionSummary(packet: JsonObject): string {
  return Array.from(new Set(packet.result_envelope.result_sets.flatMap((set: JsonObject) => set.values.map((value: JsonObject) => value.dimension))))
    .sort()
    .join(", ");
}

function boundarySummary(boundary: Record<string, boolean>): string {
  if (
    boundary.human_review_required &&
    !boundary.software_makes_compliance_claim &&
    !boundary.software_makes_certification_claim &&
    !boundary.software_makes_sealing_claim &&
    !boundary.software_makes_approval_claim &&
    !boundary.software_makes_authentication_claim
  ) {
    return "human review remains required; acceptance stays with the responsible engineer";
  }
  return "professional boundary requires attention";
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "ref";
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
