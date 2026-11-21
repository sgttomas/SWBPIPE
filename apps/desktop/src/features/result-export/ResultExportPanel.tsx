import { KnownSemanticNotices } from "../results/KnownSemanticNotices";
import { checkedJsonText } from "../../services/hashService";
import { hasNativeMechanicsInvocation } from "../../services/previewService";
import { hasCurrentSourceContract } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { surfaceOutputRefusal } from "../results/outputPolicy";
import { retainedPrecisionSummaryLine } from "../results/retainedPrecisionDisclosure";
import { LoadReferenceDisclosure, loadReferenceDisclosure } from "../results/loadReferenceDisclosure";
import { useEffect, useState } from "react";
import { buildCurrentResultExport, type JsonObject } from "./resultExportAdapter";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { Download, FileJson } from "lucide-react";
import { ExportRow, ExportStepButton } from "../export-step/ExportStep";
import { AS_GIVEN, EXPORT_FORM_WORDS, type PackageRecord } from "../export-step/exportForm";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, ObjectRef, PreviewModel } from "../../types";

function liveResultBinding(model: PreviewModel, result: MechanicsResult | null, analysisRun: AnalysisRunEnvelope | null, inputManifest?: CurrentSessionInputManifestEvidence | null): string | null {
  try {
    // T6S-3 (RR decision 2): an explicit gate on this surface's entry in the output
    // policy. A retained-precision successor is admitted only at numerically eligible
    // standing with the live native capture; epoch 8's exact routes, load/reference-state
    // results included, add no policy refusal and take Rust's standing below.
    if (!result || !analysisRun || !inputManifest || !hasCurrentSourceContract(result) || surfaceOutputRefusal(result, model, "result-export") !== null
      || !hasNativeMechanicsInvocation(result, model, inputManifest.manifest.solver_basis.solver_mode)
      || !numericalResultStanding(result, model).eligible) return null;
    return checkedJsonText({ model, result, analysisRun, inputManifest });
  } catch { return null; }
}

/** The person's own local copy: the row's press states the intent to save it, with the project's values, so the
 * link shows no box of its own. The one place this row passes it. */
const OWN_LOCAL_COPY = true;

/** The package's record entries for the row's form. The solve's own diagnostics carried in the document are the
 * result's content, not a loss, unsupported entry or failed validation of the export: they never open the record
 * first. */
export function resultExportPackageRecord(packet: JsonObject): PackageRecord {
  return {
    format: "result-export",
    entries: (Array.isArray(packet.result_envelope?.diagnostics) ? packet.result_envelope.diagnostics : []).map((item: JsonObject) => ({ kind: "content" as const, term: String(item.code ?? item.id), severity: item.severity, text: item.message })),
    validation: null
  };
}

/** What the file holds, in the generator's terms with counts: every source row, as a quantity, a review row or a
 * row disclosed without a value. */
function resultExportContent(packet: JsonObject): string | null {
  const envelope = packet.result_envelope;
  if (![envelope?.result_sets, envelope?.review_evidence, envelope?.row_disclosures, envelope?.diagnostics].every(Array.isArray)) return null;
  const quantities = envelope.result_sets.reduce((count: number, set: JsonObject) => count + (Array.isArray(set.values) ? set.values.length : 0), 0);
  const reviews = envelope.review_evidence.length, disclosed = envelope.row_disclosures.length;
  return `${quantities + reviews + disclosed} rows: ${quantities} quantities, ${reviews} review rows, ${disclosed} disclosed without a value; diagnostics=${envelope.diagnostics.length}`;
}

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
  const [publicationFingerprint, setPublicationFingerprint] = useState<string|null>(null);
  const retainedSummary = retainedPrecisionSummaryLine(result, model);
  const currentFingerprint = liveResultBinding(model, result, analysisRun, inputManifest);
  // Epoch 8 (design §5.4): shown only while the live binding admits the result.
  const loadReferenceLine = currentFingerprint !== null ? loadReferenceDisclosure(result, "result-export") : null;
  const currentPacket = binding?.[0]===model && binding?.[1]===result && binding?.[2]===analysisRun && binding?.[3]===inputManifest
    && currentFingerprint !== null && currentFingerprint === publicationFingerprint ? packet : null;
  useEffect(()=>{let active=true;setPacket(null);setFinding(null);setBinding(null);setPublicationFingerprint(null);
    const capturedFingerprint = liveResultBinding(model, result, analysisRun, inputManifest);
    if(result&&analysisRun)buildCurrentResultExport({model,result,analysisRun,inputManifest}).then(doc=>{if(active && capturedFingerprint !== null && liveResultBinding(model,result,analysisRun,inputManifest) === capturedFingerprint){setPacket(doc);setBinding([model,result,analysisRun,inputManifest!]);setPublicationFingerprint(capturedFingerprint);}}).catch(error=>{if(active)setFinding(String(error));});
    return ()=>{active=false;};
  },[model,result,analysisRun,inputManifest]);
  // The export control's disposition for this row: as given until the control reports it.
  const disposition = AS_GIVEN;
  const stale = (event: { preventDefault(): void; stopPropagation(): void }) => {
    if (liveResultBinding(model,result,analysisRun,inputManifest) !== publicationFingerprint) {
      event.preventDefault(); event.stopPropagation();
      setPacket(null); setBinding(null); setPublicationFingerprint(null);
    }
  };
  return (
    <ExportRow
      testId="result-export-panel"
      className="panel result-export-panel"
      ariaLabel="Result export audit"
      stepTestId="result-export"
      icon={<FileJson size={16} aria-hidden="true" />}
      name="Result Export"
      what="Writes the solved result as one local JSON file."
      press={EXPORT_FORM_WORDS.ownLocalCopy}
      content={currentPacket && resultExportContent(currentPacket) !== null ? (
        <ExportLine label="Rows" value={resultExportContent(currentPacket)!} testId="result-export-content" />
      ) : null}
      packageRecord={currentPacket ? resultExportPackageRecord(currentPacket) : null}
      disposition={disposition}
      units={currentPacket ? (
        <ExportLine
          label="Unit witnesses"
          value={`count=${currentPacket.result_envelope.unit_preservation_witnesses.length}; conversion=${String(
            currentPacket.result_envelope.unit_preservation_witnesses.some((item: JsonObject) => item.conversion_performed)
          )}`}
          testId="result-export-unit-witnesses"
        />
      ) : null}
      notices={(
        <>
          <KnownSemanticNotices result={result} testIdPrefix="result-export" />
          {retainedSummary ? <p className="muted" data-testid="result-export-retained-precision-summary">{retainedSummary}</p> : null}
          <LoadReferenceDisclosure text={loadReferenceLine} surface="result-export" />
        </>
      )}
      action={({ open, label, write }) => currentPacket ? (
        write ?? <ExportStepButton stepTestId="result-export" open={open}>{label}</ExportStepButton>
      ) : (
        <>
          <p className="muted export-row-status" id="result-export-empty" data-testid="result-export-empty">
            {finding ?? "Run mechanics preview with valid Current input proof to assemble a local result JSON for review."}
          </p>
          <ExportStepButton stepTestId="result-export" open={open} disabled describedBy="result-export-empty" />
        </>
      )}
      record={currentPacket ? (
        <>
          <span className="export-step-summary" data-testid="result-export-summary">
            available; rows={currentPacket.result_envelope.result_sets[0]?.values.length ?? 0}; sets=
            {currentPacket.result_envelope.result_sets.length}; diagnostics={currentPacket.result_envelope.diagnostics.length}
          </span>
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
      ) : null}
      note={(
        <small className="report-note">
          Result export is a local JSON envelope; stress-neutral CSV/JSON preview is available after a
          mechanics run, while public transport and local FEA package formats remain TBD.
        </small>
      )}
      saves={currentPacket ? (
        <div className="report-actions" onClickCapture={stale}>
        <ControlledExportLink
              className="report-export-link"
              data-testid="result-export-link"
              explicitLocalPrivateIntent={OWN_LOCAL_COPY}
              nativeCurrentBinding={binding}
              download={`openpipestress-preview-results-${safeFileToken(currentPacket.result_envelope.run_ref.ref_id)}.json`}
              href={jsonDataHref(currentPacket)}
            >
              <Download size={14} aria-hidden="true" />
              Local result JSON
        </ControlledExportLink>
        </div>
      ) : null}
    />
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
    return "human_review_required=true; professional_claim=false";
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
