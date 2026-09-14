import { Download, FileJson } from "lucide-react";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, ObjectRef, PreviewModel } from "../../types";

export function ResultExportPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = result && analysisRun ? buildResultExportPacket({ model, result, analysisRun }) : null;
  return (
    <section className="panel result-export-panel" aria-label="Result export audit" data-testid="result-export-panel">
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Result Export
      </div>
      {packet ? (
        <>
          <div className="report-actions">
        <ControlledExportLink
              className="report-export-link"
              data-testid="result-export-link"
              download={`openpipestress-preview-results-${safeFileToken(packet.result_envelope.run_ref.ref_id)}.json`}
              href={jsonDataHref(packet)}
            >
              <Download size={14} aria-hidden="true" />
              Local result JSON
        </ControlledExportLink>
            <span data-testid="result-export-summary">
              available; rows={packet.result_envelope.result_sets[0]?.values.length ?? 0}; sets=
              {packet.result_envelope.result_sets.length}; diagnostics={packet.result_envelope.diagnostics.length}
            </span>
          </div>
          <div className="report-list" data-testid="result-export-body">
            <ExportLine
              label="Format"
              value={`${packet.export_format_status.baseline_format}; additional_formats=${packet.export_format_status.additional_formats}`}
              testId="result-export-format"
            />
            <ExportLine
              label="State binding"
              value={`${packet.result_envelope.model_ref.ref_id}; ${packet.result_envelope.run_ref.ref_id}`}
              testId="result-export-state-binding"
            />
            <ExportLine
              label="Units"
              value={`${unitCount(packet)} explicit units; dimensions=${dimensionSummary(packet)}`}
              testId="result-export-units"
            />
            <ExportLine
              label="Unit witnesses"
              value={`count=${packet.result_envelope.unit_preservation_witnesses.length}; conversion=${String(
                packet.result_envelope.unit_preservation_witnesses.some((item) => item.conversion_performed)
              )}`}
              testId="result-export-unit-witnesses"
            />
            <ExportLine
              label="Reproducibility"
              value={`deterministic_ordering=${String(packet.result_envelope.reproducibility.deterministic_ordering)}; run_hashes=${packet.result_envelope.reproducibility.run_hashes.length}`}
              testId="result-export-reproducibility"
            />
            <ExportLine
              label="Boundary"
              value={boundarySummary(packet.result_envelope.professional_boundary)}
              testId="result-export-boundary"
            />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="result-export-empty">
          Run mechanics preview to assemble a schema-first local result envelope for review and regression use.
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

function buildResultExportPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
}) {
  const run = analysisRun.analysis_run;
  const values = result.results
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((item) => quantityResult(item));
  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-08-04",
    package_id: "PKG-08",
    scope_item: "SOW-046",
    objectives: ["OBJ-007", "OBJ-009"],
    export_format_status: {
      baseline_format: "schema_first_json_result_envelope",
      additional_formats: "stress_neutral_csv_json_preview_available",
      public_transport_protocol: "TBD",
      local_fea_package_format: "TBD",
      external_adapter_formats: "TBD"
    },
    result_envelope: {
      schema_version: "0.1.0",
      envelope_id: `result-envelope:${result.run_id}`,
      model_ref: reference("Model", result.model_ref),
      run_ref: reference("AnalysisRun", run.run_id),
      solver_version: {
        solver_name: "OpenPipeStress product_physics technical preview",
        solver_version: "0.1.0",
        solver_build_ref: "local_fixture_preview"
      },
      unit_system_ref: reference("UnitSystem", `${model.project.id}:units`),
      load_basis_refs: run.load_basis_refs.map(objectReference),
      result_sets: [
        {
          set_id: `result-set:${result.run_id}:mechanics`,
          set_type: "mechanics",
          basis_ref: reference("AnalysisRun", run.run_id),
          values
        }
      ],
      unit_witness_policy: "preserve_source_result_value_unit_and_dimension_per_exported_result_row",
      unit_preservation_witnesses: resultExportUnitPreservationWitnesses(model, values),
      diagnostics: [...model.diagnostics, ...result.diagnostics].map(resultExportDiagnostic),
      provenance: previewProvenance(),
      reproducibility: {
        model_hash: {
          algorithm: "TBD",
          canonicalization: "TBD",
          payload_ref: objectReference(run.model_state_ref),
          value: "TBD"
        },
        run_hashes: run.hashes.map(checksum),
        audit_manifest_ref: reference("AuditManifest", `audit-manifest:${run.run_id}:preview`),
        deterministic_ordering: true
      },
      analysis_status: run.analysis_status,
      rule_pack_refs: [
        {
          rule_pack_id: "rule-pack:user-supplied:not-loaded",
          version: "TBD",
          checksum: {
            algorithm: "TBD",
            canonicalization: "TBD",
            payload_ref: reference("RulePack", "rule-pack:user-supplied:not-loaded"),
            value: "TBD"
          },
          source_notice: "User/private rule pack is not loaded in the public technical preview.",
          redistribution_status: "TBD",
          completeness_status: "missing_required_inputs",
          private_payload_redacted: true
        }
      ],
      professional_boundary: run.professional_boundary,
      downstream_use: {
        review: true,
        regression_comparison: true,
        report_consumption: true,
        headless_automation: true,
        governed_downstream_tooling: true,
        additional_export_formats: "stress_neutral_csv_json_preview_available"
      }
    }
  };
}

function quantityResult(item: MechanicsResult["results"][number]) {
  const family = resultFamily(item);
  const output = {
    result_id: item.id,
    family,
    object_ref: stableRef(item.entity_ref),
    basis_ref: item.basis_ref ? basisReference(item.basis_ref) : reference("AnalysisRun", "preview-mechanics"),
    station_ref: reference("ResultLocation", item.metadata?.location ?? "summary"),
    magnitude: item.value,
    unit: item.unit,
    dimension: resultDimension(family, item.kind),
    provenance: previewProvenance(),
    metadata: resultMetadata(item),
    diagnostics: []
  };
  if (!item.metadata && family !== "force" && family !== "moment") {
    delete (output as Partial<typeof output>).metadata;
  }
  return output;
}

function resultExportUnitPreservationWitnesses(
  model: PreviewModel,
  values: ReturnType<typeof quantityResult>[]
) {
  return values.map((item) => ({
    witness_id: `result-export-unit:${safeRefToken(item.result_id)}`,
    source_result_ref: reference("result_value", item.result_id),
    source_field_path: `result_envelope.result_sets[].values[${item.result_id}].magnitude`,
    source_quantity: {
      value: item.magnitude,
      unit: item.unit,
      dimension: item.dimension
    },
    target_result_ref: reference("result_value", item.result_id),
    target_field_path: `result_envelope.result_sets[].values[${item.result_id}]`,
    target_quantity: {
      value: item.magnitude,
      unit: item.unit,
      dimension: item.dimension
    },
    target_quantity_policy: "exported_result_row_preserves_source_value_unit_and_dimension",
    export_unit_policy: "preserve_source_result_unit_and_dimension",
    conversion_performed: false,
    unit_system_ref: reference("UnitSystem", `${model.project.id}:units`),
    provenance: previewProvenance()
  }));
}

function resultMetadata(item: MechanicsResult["results"][number]) {
  return {
    component: item.metadata?.component ?? "TBD",
    coordinate_system: item.metadata?.coordinate_system ?? "TBD",
    location: item.metadata?.location ?? "summary",
    basis: item.metadata?.basis ?? "TBD",
    sign_convention: item.metadata?.sign_convention ?? "preview sign convention not expanded for this value"
  };
}

function resultExportDiagnostic(item: Diagnostic) {
  const affectedRef = item.affected_refs?.[0] ?? "result-export:preview";
  return {
    code: item.code,
    class: diagnosticClass(item),
    severity: item.severity === "error" ? "blocking" : item.severity,
    source: reference("Source", item.source ?? "apps/desktop/src/features/result-export/ResultExportPanel.tsx"),
    affected_object: stableRef(affectedRef),
    message: item.message,
    remediation: "Review the source model, private rule inputs, provenance, and technical-preview limitations before reliance.",
    provenance: previewProvenance()
  };
}

function diagnosticClass(item: Diagnostic): string {
  if (item.code.includes("RULE")) return "RULE_CHECK_BLOCKING";
  if (item.code.includes("PROVENANCE")) return "PROVENANCE_WARNING";
  if (item.code.includes("UNIT")) return "UNIT_WARNING";
  if (item.severity === "blocking" || item.severity === "error") return "EXPORT_BLOCKING";
  return "ASSUMPTION_WARNING";
}

function resultFamily(item: MechanicsResult["results"][number]): string {
  const kind = item.kind.toLowerCase();
  const id = item.id.toLowerCase();
  if (kind.includes("displacement") || id.includes("disp")) return "displacement";
  if (kind.includes("reaction") || id.includes("reaction")) return "reaction";
  if (kind.includes("force") || id.includes("force")) return "force";
  if (kind.includes("moment") || id.includes("moment")) return "moment";
  if (kind.includes("stress") || id.includes("stress")) return "stress";
  if (kind.includes("ratio") || id.includes("ratio")) return "ratio";
  return "ratio";
}

function resultDimension(family: string, kind: string): string {
  if (family === "displacement") return "length";
  if (family === "reaction" || family === "force") return "force";
  if (family === "moment") return "moment";
  if (family === "stress") return "stress";
  if (family === "ratio") return "dimensionless";
  if (kind.toLowerCase().includes("rotation")) return "angle";
  return "TBD";
}

function checksum(item: AnalysisRunEnvelope["analysis_run"]["hashes"][number]) {
  return {
    algorithm: item.algorithm,
    canonicalization: item.canonicalization,
    payload_ref: objectReference(item.payload_ref),
    value: item.value
  };
}

function basisReference(item: MechanicsResult["results"][number]["basis_ref"]) {
  if (!item) return reference("AnalysisRun", "preview-mechanics");
  if (item.ref_type === "load_case") return reference("LoadCase", item.ref_id);
  if (item.ref_type === "combination") return reference("Combination", item.ref_id);
  return reference(item.ref_type, item.ref_id);
}

function objectReference(item: ObjectRef) {
  return reference(item.object_type, item.ref);
}

function stableRef(value: string) {
  const [type] = value.split(":", 1);
  return reference(type || "Object", value);
}

function reference(refType: string, refId: string) {
  return {
    ref_type: refType,
    ref_id: refId
  };
}

function previewProvenance() {
  return {
    source_name: "invented preview fixture",
    source_location: "fixtures/product_preview",
    source_license: "project-invented",
    contributor: "OpenPipeStress project",
    contributor_certification: "invented data; no protected or private payload",
    redistribution_status: "invented_non_engineering_example",
    review_status: "accepted"
  };
}

function unitCount(packet: ReturnType<typeof buildResultExportPacket>): number {
  return new Set(packet.result_envelope.result_sets.flatMap((set) => set.values.map((value) => value.unit))).size;
}

function dimensionSummary(packet: ReturnType<typeof buildResultExportPacket>): string {
  return Array.from(new Set(packet.result_envelope.result_sets.flatMap((set) => set.values.map((value) => value.dimension))))
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
