import { Download, FileJson } from "lucide-react";
import { usePackageHash, withCanonicalPackageHash } from "../../services/usePackageHash";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel, ResultBasisRef } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

const STRESS_NEUTRAL_EXPORT_VERSION = "0.1.0";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_canonical_package_hash";
const SCHEMA_VALIDATION_STATUS = "desktop_preview_shape_aligned_not_runtime_json_schema_validated";
const CSV_COLUMNS = [
  "result_id",
  "canonical_ref",
  "row_kind",
  "result_family",
  "load_case_ref",
  "station_ref",
  "component_ref",
  "value",
  "unit",
  "dimension",
  "correlation_status"
] as const;

type CsvColumn = (typeof CSV_COLUMNS)[number];

type StressNeutralRef = {
  object_type: string;
  ref: string;
};

type StressNeutralRow = {
  result_id: string;
  canonical_ref: StressNeutralRef;
  row_kind: "result_value";
  result_family: string;
  load_case_ref: StressNeutralRef;
  station_ref: StressNeutralRef;
  component_ref: StressNeutralRef;
  value: number;
  unit: string;
  dimension: string;
  correlation_status: "canonical_id_map" | "unit_or_dimension_blocking_review_required";
  source_result_ref: StressNeutralRef;
  provenance: ReturnType<typeof previewProvenance>;
};

export function StressNeutralExportPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const basePacket = result && analysisRun ? buildStressNeutralExportPacket({ model, result, analysisRun }) : null;
  const packageHash = usePackageHash(
    basePacket?.manifest.manifest_id ?? null,
    basePacket,
    "manifest_and_validation_report_package_hash_carrier_fields"
  );
  const packet = basePacket ? withCanonicalPackageHash(basePacket, packageHash) : null;
  return (
    <section
      className="panel stress-neutral-export-panel"
      aria-label="Stress-neutral CSV JSON export"
      data-testid="stress-neutral-panel"
    >
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Stress-neutral CSV/JSON
      </div>
      {packet ? (
        <>
          <div className="report-actions">
            <a
              className="report-export-link"
              data-testid="stress-neutral-export-link"
              download={`openpipestress-preview-stress-neutral-${safeFileToken(packet.source_result_ref.ref)}.json`}
              href={jsonDataHref(packet)}
            >
              <Download size={14} aria-hidden="true" />
              Package JSON
            </a>
            <a
              className="report-export-link"
              data-testid="stress-neutral-csv-link"
              download={`openpipestress-preview-stress-neutral-${safeFileToken(packet.source_result_ref.ref)}.csv`}
              href={csvDataHref(packet.csv_text)}
            >
              <Download size={14} aria-hidden="true" />
              CSV
            </a>
            <span data-testid="stress-neutral-summary">
              available; rows={packet.result_rows.length}; csv_columns={packet.export_profile.csv_columns.length};
              diagnostics={packet.diagnostics.length}
            </span>
          </div>
          <div className="report-list" data-testid="stress-neutral-body">
            <StressNeutralLine
              label="Format"
              value={`${packet.export_profile.target_family}; profile=${packet.export_profile.profile_id}; schema=${packet.validation_report.schema_validation_status}`}
              testId="stress-neutral-format"
            />
            <StressNeutralLine
              label="State binding"
              value={`${packet.source_model_ref.ref}; ${packet.source_run_ref.ref}; ${packet.source_result_ref.ref}`}
              testId="stress-neutral-state-binding"
            />
            <StressNeutralLine
              label="Units"
              value={`${unitCount(packet)} explicit units; dimensions=${dimensionSummary(
                packet
              )}; ${unitDisclosureSummary(packet.unit_system_disclosure)}`}
              testId="stress-neutral-units"
            />
            <StressNeutralLine
              label="Package"
              value={`members=${packet.manifest.package_members.length}; stable_ids=${packet.stable_id_map.length}; loss_entries=${packet.loss_report.entries.length}; validation=${packet.validation_report.validation_status}; package_hash=${packet.manifest.canonical_package_hash_status}`}
              testId="stress-neutral-package"
            />
            <StressNeutralLine
              label="Boundary"
              value={`vendor_format=${String(packet.vendor_format_claim)}; solver_validation=${String(
                packet.solver_validation_claim
              )}; code_compliance=${String(packet.code_compliance_claim)}; professional_reliance=${String(
                packet.professional_reliance_claim
              )}`}
              testId="stress-neutral-boundary"
            />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="stress-neutral-empty">
          Run mechanics preview to assemble a stress-neutral CSV/JSON package for local review and downstream adapter
          development.
        </p>
      )}
      <small className="report-note">
        Stress-neutral export is a local technical-preview review package only; it is not a vendor format, solver input
        deck, solver validation, code compliance, or professional reliance record.
      </small>
    </section>
  );
}

function StressNeutralLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildStressNeutralExportPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
}) {
  const run = analysisRun.analysis_run;
  const resultRows = result.results
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((item) => stressNeutralRow(item, run.run_id));
  const csvText = renderCsv(resultRows);
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: {},
    conversionPolicy: "result_row_units_preserved_no_export_time_conversion",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx"
  });
  const stableIdMap = resultRows.map((row, index) => ({
    map_id: `stress-neutral-map:${safeFileToken(row.result_id)}`,
    canonical_ref: row.canonical_ref,
    export_ref: reference("StressNeutralResultRow", `stress-neutral-row:${safeFileToken(row.result_id)}`),
    mapping_status: "mapped",
    loss_category: "exported",
    row_index: index,
    provenance: previewProvenance()
  }));
  const diagnostics = stressNeutralDiagnostics(resultRows);
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking").length;
  const validationStatus = blockingCount === 0 ? "passed" : "blocked_missing_unit_or_dimension";

  return {
    schema_version: STRESS_NEUTRAL_EXPORT_VERSION,
    document_kind: "openpipestress.technical_preview.stress_neutral_csv_json_package",
    deliverable_id: "DEL-17-06",
    package_id: "PKG-17",
    scope_items: ["SOW-046", "SOW-074"],
    objectives: ["OBJ-007", "OBJ-017", "OBJ-018"],
    export_scope: "local_browser_download_preview",
    export_id: `stress-neutral:${safeFileToken(result.run_id)}`,
    package_status: "stress_neutral_export_package",
    source_model_ref: reference("Model", model.project.id),
    source_run_ref: reference("AnalysisRun", run.run_id),
    source_result_ref: reference("ResultEnvelope", `result-envelope:${result.run_id}`),
    source_basis_refs: [
      { deliverable_id: "DEL-08-04", status: "source_result_envelope" },
      { deliverable_id: "DEL-14-02", status: "source_analysis_run" },
      { deliverable_id: "DEL-14-05", status: "source_hash_boundary" },
      { deliverable_id: "DEL-17-02", status: "source_native_json_boundary" }
    ],
    source_hashes: run.hashes.map((item) => ({
      ...item,
      hash_role: "source_run_evidence_not_package_member_checksum"
    })),
    export_profile: {
      profile_id: "ops.stress_neutral.v1",
      target_family: "stress_neutral_csv_json",
      csv_columns: CSV_COLUMNS,
      identity_policy: "canonical_ref_per_row_plus_stable_id_map",
      unit_policy: "unit_and_dimension_required_per_row",
      loss_report_policy: "mandatory_for_every_package",
      comparison_semantics: "diagnostic_export_only_no_pass_fail"
    },
    unit_system_disclosure: unitSystemDisclosure,
    result_rows: resultRows,
    csv_text: csvText,
    stable_id_map: stableIdMap,
    loss_report: {
      report_id: `loss-report:${safeFileToken(result.run_id)}:stress-neutral`,
      entries: [
        {
          loss_id: "loss:desktop-preview:stress-neutral-exported",
          category: "exported",
          severity: "info",
          affected_ref: reference("ResultEnvelope", `result-envelope:${result.run_id}`),
          reason: "Mechanics result rows were emitted with canonical refs, units, dimensions, and stable map entries.",
          downstream_implication: "Rows are suitable for local spreadsheet review and downstream adapter prototype input."
        },
        {
          loss_id: "loss:desktop-preview:comparison-semantics-tbd",
          category: "tbd",
          severity: "warning",
          affected_ref: reference("ComparisonSemantics", "stress-neutral:comparison-pass-fail"),
          reason: "Pass/fail, tolerances, and external target interpretation are outside this preview package.",
          downstream_implication: "Consumers must not infer acceptance, equivalence, or compliance from this export."
        },
        {
          loss_id: "loss:desktop-preview:private-rule-payload-redacted",
          category: "omitted",
          severity: "warning",
          affected_ref: reference("RulePack", "rule-pack:user-supplied:not-loaded"),
          reason: "Private rule criteria and compliance inputs are not bundled in the public technical preview.",
          downstream_implication: "Completeness and code-rule interpretation require separate governed review."
        }
      ],
      provenance: previewProvenance()
    },
    manifest: {
      manifest_id: `manifest:${safeFileToken(result.run_id)}:stress-neutral`,
      package_members: [
        member("manifest", "manifest.json", "json", 1),
        member("csv_text", "stress_neutral_results.csv", "csv", resultRows.length),
        member("result_rows", "result_rows.json", "json", resultRows.length),
        member("unit_system_disclosure", "unit_system_disclosure.json", "json", 1),
        member("stable_id_map", "stable_id_map.json", "json", stableIdMap.length),
        member("loss_report", "loss_report.json", "json", 3),
        member("validation_report", "validation_report.json", "json", 1),
        member("diagnostics", "diagnostics.json", "json", diagnostics.length)
      ],
      schema_ref: "schemas/stress_neutral_export.schema.json",
      provenance: previewProvenance()
    },
    validation_report: {
      validation_id: `validation:${safeFileToken(result.run_id)}:stress-neutral`,
      validation_status: validationStatus,
      schema_validation_status: SCHEMA_VALIDATION_STATUS,
      checks: [
        check("csv_json_row_sync", resultRows.length === csvText.trimEnd().split("\n").length - 1),
        check("canonical_ref_per_row", resultRows.every((row) => Boolean(row.canonical_ref.ref))),
        check("unit_and_dimension_per_row", resultRows.every((row) => Boolean(row.unit && row.dimension && row.dimension !== "TBD"))),
        check("stable_id_map_per_row", stableIdMap.length === resultRows.length),
        check("loss_report_present", true)
      ],
      blocking_diagnostic_count: blockingCount,
      provenance: previewProvenance()
    },
    diagnostics,
    boundary_notes: [
      "Local browser preview does not emit canonical package member hashes.",
      "The package is not a vendor format, solver input deck, or external target compatibility claim.",
      "Comparison pass/fail, tolerances, release gates, and professional acceptance remain outside this export."
    ],
    privacy: {
      privacy_classification: "public_metadata_and_invented_results",
      private_payload_embedded: false,
      protected_content_embedded: false,
      source_model_mutated: false
    },
    professional_boundary: professionalBoundary(),
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    vendor_format_claim: false,
    solver_input_deck_claim: false,
    target_compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false
  };
}

function stressNeutralRow(item: MechanicsResult["results"][number], runId: string): StressNeutralRow {
  const family = resultFamily(item);
  const dimension = resultDimension(family, item.kind);
  return {
    result_id: item.id,
    canonical_ref: reference("Result", item.id),
    row_kind: "result_value",
    result_family: family,
    load_case_ref: basisReference(item.basis_ref, runId),
    station_ref: reference("Station", item.metadata?.location ?? "summary"),
    component_ref: entityReference(item.entity_ref),
    value: item.value,
    unit: item.unit,
    dimension,
    correlation_status: item.unit && dimension && dimension !== "TBD" ? "canonical_id_map" : "unit_or_dimension_blocking_review_required",
    source_result_ref: reference("Result", item.id),
    provenance: previewProvenance()
  };
}

function member(role: string, path: string, format: string, recordCount: number) {
  return {
    role,
    path,
    format,
    record_count: recordCount,
    hash_status: HASH_STATUS_TBD
  };
}

function check(checkId: string, passed: boolean) {
  return {
    check_id: checkId,
    status: passed ? "passed" : "blocking",
    blocking: !passed
  };
}

function stressNeutralDiagnostics(rows: StressNeutralRow[]) {
  const missingRows = rows.filter((row) => !row.unit || !row.dimension || row.dimension === "TBD");
  const diagnostics = [
    {
      diagnostic_id: "diagnostic:stress-neutral:hash-tbd",
      code: "SN-DESKTOP-PREVIEW-HASH-TBD",
      severity: "warning",
      message: "Browser preview records source hashes but does not emit canonical package member hashes.",
      affected_ref: reference("StressNeutralExportPackage", "stress-neutral:browser-preview"),
      provenance: previewProvenance()
    },
    {
      diagnostic_id: "diagnostic:stress-neutral:comparison-semantics-tbd",
      code: "SN-DESKTOP-PREVIEW-COMPARISON-TBD",
      severity: "warning",
      message: "Comparison pass/fail semantics and tolerance profiles are not defined by this preview export.",
      affected_ref: reference("ComparisonSemantics", "stress-neutral:comparison-pass-fail"),
      provenance: previewProvenance()
    }
  ];
  if (missingRows.length > 0) {
    diagnostics.push({
      diagnostic_id: "diagnostic:stress-neutral:unit-dimension-missing",
      code: "SN-UNIT-DIMENSION-MISSING",
      severity: "blocking",
      message: `${missingRows.length} result rows are missing unit or dimensional metadata.`,
      affected_ref: reference("StressNeutralResultRows", "stress-neutral:result-rows"),
      provenance: previewProvenance()
    });
  }
  return diagnostics;
}

function renderCsv(rows: StressNeutralRow[]): string {
  const body = rows.map((row) => CSV_COLUMNS.map((column) => csvValue(csvColumnValue(row, column))).join(","));
  return `${CSV_COLUMNS.join(",")}\n${body.join("\n")}\n`;
}

function csvColumnValue(row: StressNeutralRow, column: CsvColumn): string | number | StressNeutralRef {
  return row[column];
}

function csvValue(value: string | number | StressNeutralRef): string {
  const text = typeof value === "object" ? value.ref : String(value);
  if (/[",\n\r]/.test(text)) return `"${text.replaceAll('"', '""')}"`;
  return text;
}

function basisReference(item: ResultBasisRef | undefined, runId: string) {
  if (!item) return reference("AnalysisRun", runId);
  if (item.ref_type === "load_case") return reference("LoadCase", item.ref_id);
  if (item.ref_type === "combination") return reference("Combination", item.ref_id);
  return reference("ResultBasis", item.ref_id);
}

function entityReference(value: string) {
  if (value.startsWith("pipe:")) return reference("PipeElement", value);
  if (value.startsWith("node:")) return reference("Node", value);
  if (value.startsWith("support:")) return reference("Support", value);
  if (value.startsWith("component:")) return reference("Component", value);
  if (value.startsWith("material:")) return reference("Material", value);
  return reference("CanonicalObject", value);
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

function unitCount(packet: ReturnType<typeof buildStressNeutralExportPacket>): number {
  return new Set(packet.result_rows.map((row) => row.unit)).size;
}

function dimensionSummary(packet: ReturnType<typeof buildStressNeutralExportPacket>): string {
  return Array.from(new Set(packet.result_rows.map((row) => row.dimension))).sort().join(", ");
}

function professionalBoundary() {
  return {
    supports_review: true,
    supports_regression_comparison_input: true,
    supports_downstream_tooling: true,
    supports_release_readiness: false,
    supports_external_target_compatibility: false,
    supports_solver_validation: false,
    supports_code_compliance: false,
    supports_certification: false,
    supports_sealing: false,
    supports_approval: false,
    supports_professional_reliance_record: false,
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function reference(objectType: string, ref: string): StressNeutralRef {
  return {
    object_type: objectType,
    ref
  };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
    source_license: "PolyForm-Noncommercial-1.0.0 project license context",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented preview metadata only; no protected standards or private project payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function csvDataHref(csvText: string): string {
  return `data:text/csv;charset=utf-8,${encodeURIComponent(csvText)}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
