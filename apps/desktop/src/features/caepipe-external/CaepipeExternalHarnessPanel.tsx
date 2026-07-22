import { Download, FileOutput } from "lucide-react";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

type CaePipeExternalReference = {
  object_type: string;
  ref: string;
};

type CaePipeExternalCsvRow = {
  row_number: number;
  section: "NODE_DISPLACEMENTS" | "ELEMENT_FORCES";
  stable_id: string;
  target_id: string;
  load_case: string;
  unit: string;
  values: Record<string, number>;
  correlation_status: "canonical_id_map";
  source_csv_ref: CaePipeExternalReference;
  provenance: ReturnType<typeof previewProvenance>;
};

const CAEPIPE_EXTERNAL_VERSION = "0.1.0";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_canonical_package_hash";
const SCHEMA_VALIDATION_STATUS = "desktop_preview_shape_aligned_not_runtime_json_schema_validated";
const INVOCATION_PROFILE_TBD = "TBD-17-05-invocation-profile";
const CONFIGURATION_SURFACE_TBD = "TBD-17-05-PH-001";

export function CaepipeExternalHarnessPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = buildCaePipeExternalHarnessPacket({ model, result, analysisRun });

  return (
    <section
      className="panel caepipe-external-panel"
      aria-label="CAEPIPE external harness"
      data-testid="caepipe-external-panel"
    >
      <div className="panel-title">
        <FileOutput size={16} aria-hidden="true" />
        CAEPIPE External Harness
      </div>
      <div className="report-actions">
        <ControlledExportLink
          className="report-export-link"
          data-testid="caepipe-external-export-link"
          download={`openpipestress-preview-caepipe-external-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Harness JSON
        </ControlledExportLink>
        <ControlledExportLink
          className="report-export-link"
          data-testid="caepipe-external-csv-link"
          download={`openpipestress-preview-caepipe-external-${safeFileToken(model.project.id)}.csv`}
          href={textDataHref(packet.csv_text)}
        >
          <Download size={14} aria-hidden="true" />
          Parser CSV
        </ControlledExportLink>
        <span data-testid="caepipe-external-summary">
          available; status={packet.package_status}; rows={packet.parsed_csv.row_count}; sections=
          {packet.parser_coverage.length}; external_invoked={String(packet.execution_result.attempted)};
          diagnostics={packet.diagnostics.length}
        </span>
      </div>
      <div className="report-list" data-testid="caepipe-external-body">
        <CaePipeExternalLine
          label="Harness"
          value={`config=${packet.executable_config.configured_path_state}; invocation=${packet.command_profile.profile_id}; mode=${packet.command_profile.invocation_mode}`}
          testId="caepipe-external-harness"
        />
        <CaePipeExternalLine
          label="State binding"
          value={`${packet.mbf_package_ref.ref}; ${packet.model_state_ref.ref}; ${packet.analysis_run_ref.ref}`}
          testId="caepipe-external-state-binding"
        />
        <CaePipeExternalLine
          label="Parser"
          value={`status=${packet.parsed_csv.parser_status}; coverage=${packet.parser_coverage.length}; correlation=${packet.validation_report.correlation_status}`}
          testId="caepipe-external-parser"
        />
        <CaePipeExternalLine
          label="Units"
          value={`${packet.unit_system_disclosure.unit_system_ref.ref}; ${unitDisclosureSummary(
            packet.unit_system_disclosure
          )}`}
          testId="caepipe-external-units"
        />
        <CaePipeExternalLine
          label="Unit witnesses"
          value={`count=${packet.unit_preservation_witnesses.length}; policy=preserve_parser_row_units; conversion=${String(
            packet.unit_preservation_witnesses.some((item) => item.conversion_performed)
          )}`}
          testId="caepipe-external-unit-witnesses"
        />
        <CaePipeExternalLine
          label="Run directory"
          value={`working=${packet.run_directory.working_directory}; expected_csv=${packet.run_directory.expected_csv_path}; discovery=${packet.run_directory.output_discovery_status}`}
          testId="caepipe-external-run-directory"
        />
        <CaePipeExternalLine
          label="Boundary"
          value={`bundled=${String(packet.professional_boundary.software_bundles_caepipe)}; license_bypass=false; compatibility=${String(packet.professional_boundary.software_makes_caepipe_compatibility_claim)}; solver_validation=${String(packet.professional_boundary.software_makes_solver_validation_claim)}; code_compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}; professional_reliance=${String(packet.professional_boundary.software_creates_professional_reliance_record)}`}
          testId="caepipe-external-boundary"
        />
      </div>
      <small className="report-note">
        CAEPIPE external harness output is parser-only browser preview evidence; executable path, license responsibility,
        environment responsibility, invocation profile, live execution, and target compatibility remain user-owned or TBD.
      </small>
    </section>
  );
}

function CaePipeExternalLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildCaePipeExternalHarnessPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const modelStateRef = run?.model_state_ref ?? reference("ModelState", "state:TBD");
  const analysisRunRef = run ? reference("AnalysisRun", run.run_id) : reference("AnalysisRun", "not generated");
  const runId = `caepipe-run:desktop-preview:${safeFileToken(result?.run_id ?? model.project.id)}`;
  const mbfPackageRef = reference("CaePipeMbfExportPackage", `caepipe-mbf:desktop-preview-del-17-04:${safeFileToken(model.project.id)}`);
  const parsedCsv = parsedCsvPackage(model);
  const csvText = renderCsv(parsedCsv.rows);
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: {
      element_forces: "N",
      node_displacements: "m"
    },
    conversionPolicy: "caepipe_external_parser_rows_preserve_declared_csv_units_no_external_run_conversion",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx"
  });
  const unitPreservationWitnesses = caepipeExternalUnitPreservationWitnesses(parsedCsv.rows);
  const diagnostics = caepipeExternalDiagnostics({ parsedCsv });

  return {
    schema_version: CAEPIPE_EXTERNAL_VERSION,
    document_kind: "openpipestress.technical_preview.caepipe_external_run_package",
    deliverable_id: "DEL-17-05",
    package_id: "PKG-17",
    scope_items: ["SOW-030", "SOW-046", "SOW-075"],
    objectives: ["OBJ-007", "OBJ-009", "OBJ-017", "OBJ-018"],
    export_scope: "local_browser_download_preview",
    run_id: runId,
    package_status: "parser_only_evidence",
    mbf_package_ref: mbfPackageRef,
    source_model_ref: reference("Model", model.project.id),
    model_state_ref: modelStateRef,
    analysis_run_ref: analysisRunRef,
    result_ref: result ? reference("MechanicsResult", result.run_id) : reference("MechanicsResult", "not generated"),
    executable_config: {
      configuration_surface: CONFIGURATION_SURFACE_TBD,
      configured_path_state: "absent",
      license_responsibility_acknowledged: false,
      environment_responsibility_acknowledged: false,
      path_record: "not_configured"
    },
    command_profile: {
      profile_id: INVOCATION_PROFILE_TBD,
      invocation_mode: "parser_only_not_invoked",
      command_shape: "not_invoked_public_browser_fixture_only",
      profile_basis: CONFIGURATION_SURFACE_TBD
    },
    run_directory: {
      run_directory_ref: `fixture:desktop-caepipe-external:${safeFileToken(model.project.id)}`,
      working_directory: "apps/desktop/browser-preview",
      input_mbf_path: "download:openpipestress-preview-caepipe-mbf-package.json",
      expected_csv_path: "openpipestress-preview-caepipe-external-results.csv",
      observed_csv_path: "openpipestress-preview-caepipe-external-results.csv",
      output_discovery_status: "fixture_generated_browser_preview"
    },
    execution_result: {
      attempted: false,
      exit_status: null,
      stdout_capture: "not_available",
      stderr_capture: "not_available",
      skip_reason: "Parser-only desktop preview; external CAEPIPE execution was not attempted."
    },
    parser_coverage: parserCoverage(),
    unit_system_disclosure: unitSystemDisclosure,
    unit_witness_policy: "preserve_parser_csv_row_value_unit_and_dimension_per_row",
    unit_preservation_witnesses: unitPreservationWitnesses,
    parsed_csv: parsedCsv,
    csv_text: csvText,
    validation_report: {
      validation_status: diagnostics.some((item) => item.severity === "blocking") ? "blocked" : "boundary_checked",
      schema_validation_status: SCHEMA_VALIDATION_STATUS,
      hash_validation_status: HASH_STATUS_TBD,
      row_count: parsedCsv.row_count,
      section_count: parserCoverage().length,
      correlation_status: "canonical_id_map",
      checks: [
        check("external_execution_not_attempted", true),
        check("executable_configuration_absent", true),
        check("mbf_package_bound_to_del_17_04", mbfPackageRef.ref.includes("del-17-04")),
        check("parser_rows_present", parsedCsv.row_count > 0),
        check("parser_rows_canonical_id_mapped", parsedCsv.rows.every((row) => row.correlation_status === "canonical_id_map")),
        check("unit_preservation_witness_per_parser_row", unitPreservationWitnesses.length === parsedCsv.rows.length),
        check("unit_preservation_witnesses_match_parser_rows", unitPreservationWitnesses.every((witness, index) => witnessMatchesParserRow(witness, parsedCsv.rows[index]))),
        check("no_professional_or_compatibility_claims", professionalBoundaryIsClear())
      ],
      human_review_required: true,
      provenance: previewProvenance()
    },
    diagnostics,
    checksums: [
      checksum(runId, "caepipe_external_run_metadata"),
      checksum(runId, "caepipe_external_parsed_csv"),
      checksum(runId, "caepipe_external_diagnostics")
    ],
    boundary_notes: [
      "CAEPIPE external execution is optional and user-owned.",
      "This desktop preview does not configure, discover, invoke, bundle, download, install, or license CAEPIPE.",
      "Parsed CSV rows are invented parser-only regression and handoff evidence.",
      "Evidence does not assert CAEPIPE compatibility. Validation occurs in the user's accepted professional tools; this is screening and handoff evidence."
    ],
    privacy: {
      classification: "invented_public_example",
      local_only: true,
      telemetry_allowed: false,
      private_payload_embedded: false,
      protected_payload_embedded: false,
      commercial_tool_payload_embedded: false,
      redaction_refs: []
    },
    data_boundary: {
      public_examples_policy: model.data_boundary.public_examples_policy,
      private_payload_included: false,
      protected_content_included: false,
      commercial_tool_payload_embedded: false,
      copied_commercial_solver_file_included: false
    },
    provenance: previewProvenance(),
    professional_boundary: professionalBoundary(),
    private_payload_included: false,
    protected_content_included: false,
    caepipe_compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false,
    release_or_professional_claim: false
  };
}

function parsedCsvPackage(model: PreviewModel) {
  const rows = parserRows(model);
  return {
    source_csv_ref: reference("CsvArtifact", "csv:desktop-preview-caepipe-results"),
    parser_status: "parsed_parser_only_fixture",
    row_count: rows.length,
    rows,
    diagnostics: [] as Array<Record<string, never>>,
    csv_checksum: {
      algorithm: "TBD",
      canonicalization: "normalized_ascii_lf_text",
      payload_ref: reference("CsvArtifact", "csv:desktop-preview-caepipe-results"),
      payload_scope: "caepipe_csv_text",
      value: HASH_STATUS_TBD
    },
    provenance: previewProvenance()
  };
}

function parserRows(model: PreviewModel): CaePipeExternalCsvRow[] {
  const sourceCsvRef = reference("CsvArtifact", "csv:desktop-preview-caepipe-results");
  const nodes = model.nodes.slice().sort((left, right) => left.id.localeCompare(right.id));
  const pipes = model.pipe_segments.slice().sort((left, right) => left.id.localeCompare(right.id));
  const firstNode = nodes[0];
  const secondNode = nodes[1] ?? nodes[0];
  const firstPipe = pipes[0];
  const loadCase = model.load_cases.slice().sort((left, right) => left.id.localeCompare(right.id))[0]?.id ?? "load:TBD";
  const rows: CaePipeExternalCsvRow[] = [];

  if (firstNode) {
    rows.push(csvRow(2, "NODE_DISPLACEMENTS", firstNode.id, "N001", loadCase, "m", { ux: 0, uy: 0, uz: 0 }, sourceCsvRef));
  }
  if (secondNode) {
    rows.push(csvRow(3, "NODE_DISPLACEMENTS", secondNode.id, "N002", loadCase, "m", { ux: 0.001, uy: 0, uz: 0 }, sourceCsvRef));
  }
  if (firstPipe) {
    rows.push(csvRow(4, "ELEMENT_FORCES", firstPipe.id, "P001", loadCase, "N", { axial: 12.5, shear_y: 0.1, shear_z: 0.2 }, sourceCsvRef));
  }

  return rows;
}

function csvRow(
  rowNumber: number,
  section: CaePipeExternalCsvRow["section"],
  stableId: string,
  targetId: string,
  loadCase: string,
  unit: string,
  values: Record<string, number>,
  sourceCsvRef: CaePipeExternalReference
): CaePipeExternalCsvRow {
  return {
    row_number: rowNumber,
    section,
    stable_id: stableId,
    target_id: targetId,
    load_case: loadCase,
    unit,
    values,
    correlation_status: "canonical_id_map",
    source_csv_ref: sourceCsvRef,
    provenance: previewProvenance()
  };
}

function parserCoverage() {
  return [
    {
      section_name: "ELEMENT_FORCES",
      basis: "invented_public_fixture",
      support_status: "parser_only_supported",
      expected_columns: ["stable_id", "target_id", "load_case", "axial", "shear_y", "shear_z", "unit"],
      unmapped_row_policy: "diagnostic_warning",
      diagnostic_severity: "warning",
      fixture_provenance: "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
      provenance: previewProvenance()
    },
    {
      section_name: "NODE_DISPLACEMENTS",
      basis: "invented_public_fixture",
      support_status: "parser_only_supported",
      expected_columns: ["stable_id", "target_id", "load_case", "ux", "uy", "uz", "unit"],
      unmapped_row_policy: "diagnostic_warning",
      diagnostic_severity: "warning",
      fixture_provenance: "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
      provenance: previewProvenance()
    }
  ];
}

function caepipeExternalDiagnostics({ parsedCsv }: { parsedCsv: ReturnType<typeof parsedCsvPackage> }) {
  if (parsedCsv.row_count === 0) {
    return [
      {
        code: "CAEPIPE-EXTERNAL-PARSER-NO-ROWS",
        severity: "blocking",
        class: "PARSER_COVERAGE_WARNING",
        message: "Parser-only desktop preview did not produce any rows.",
        remediation: "Provide invented parser-only rows before presenting DEL-17-05 evidence.",
        affected_refs: [reference("CsvArtifact", "csv:desktop-preview-caepipe-results")],
        provenance: previewProvenance()
      }
    ];
  }
  return [];
}

function caepipeExternalUnitPreservationWitnesses(rows: CaePipeExternalCsvRow[]) {
  return rows.map((row) => ({
    witness_id: `caepipe-external-unit:${safeFileToken(`${row.section}-${row.stable_id}-${row.row_number}`)}`,
    source_csv_ref: row.source_csv_ref,
    source_row_ref: reference("CaePipeExternalCsvRow", `csv-row:${row.row_number}`),
    source_field_path: `parsed_csv.rows[${row.row_number}].values`,
    source_quantity: {
      values: row.values,
      unit: row.unit,
      dimension: parserRowDimension(row)
    },
    target_row_ref: reference("CaePipeExternalParserRow", `parser-row:${row.row_number}`),
    target_field_path: `parsed_csv.rows[${row.row_number}]`,
    target_quantity: {
      values: row.values,
      unit: row.unit,
      dimension: parserRowDimension(row)
    },
    target_quantity_policy: "parser_row_preserves_declared_csv_value_unit_and_dimension",
    export_unit_policy: "preserve_parser_csv_row_unit_and_dimension",
    conversion_performed: false,
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-17-05")
    ],
    provenance: previewProvenance()
  }));
}

function parserRowDimension(row: CaePipeExternalCsvRow): "length" | "force" {
  return row.section === "NODE_DISPLACEMENTS" ? "length" : "force";
}

function witnessMatchesParserRow(
  witness: ReturnType<typeof caepipeExternalUnitPreservationWitnesses>[number],
  row: CaePipeExternalCsvRow | undefined
): boolean {
  return Boolean(
    row &&
      witness.source_csv_ref.ref === row.source_csv_ref.ref &&
      witness.source_row_ref.ref === `csv-row:${row.row_number}` &&
      witness.target_row_ref.ref === `parser-row:${row.row_number}` &&
      witness.source_quantity.unit === row.unit &&
      witness.target_quantity.unit === row.unit &&
      witness.source_quantity.dimension === parserRowDimension(row) &&
      witness.target_quantity.dimension === parserRowDimension(row) &&
      JSON.stringify(witness.source_quantity.values) === JSON.stringify(row.values) &&
      JSON.stringify(witness.target_quantity.values) === JSON.stringify(row.values) &&
      witness.conversion_performed === false
  );
}

function check(checkId: string, passed: boolean) {
  return {
    check_id: checkId,
    status: passed ? "pass" : "fail",
    provenance: previewProvenance()
  };
}

function checksum(runId: string, payloadScope: string) {
  return {
    algorithm: "TBD",
    canonicalization: "TBD_browser_preview_does_not_emit_canonical_package_hash",
    payload_ref: reference("CaePipeExternalRunMember", `${runId}:${payloadScope}`),
    payload_scope: payloadScope,
    value: HASH_STATUS_TBD
  };
}

function renderCsv(rows: CaePipeExternalCsvRow[]): string {
  const header = "section,stable_id,target_id,load_case,ux,uy,uz,axial,shear_y,shear_z,unit";
  const body = rows.map((row) =>
    [
      row.section,
      row.stable_id,
      row.target_id,
      row.load_case,
      row.values.ux ?? "",
      row.values.uy ?? "",
      row.values.uz ?? "",
      row.values.axial ?? "",
      row.values.shear_y ?? "",
      row.values.shear_z ?? "",
      row.unit
    ]
      .map(csvValue)
      .join(",")
  );
  return `${[header, ...body].join("\n")}\n`;
}

function csvValue(value: string | number): string {
  const raw = String(value);
  if (!/[",\n]/.test(raw)) return raw;
  return `"${raw.replaceAll("\"", "\"\"")}"`;
}

function reference(objectType: string, ref: string): CaePipeExternalReference {
  return {
    object_type: objectType,
    ref
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    supports_regression_evidence: true,
    supports_handoff_review_evidence: true,
    software_bundles_caepipe: false,
    software_invokes_caepipe_without_user_configuration: false,
    software_makes_release_claim: false,
    software_makes_caepipe_compatibility_claim: false,
    software_makes_solver_validation_claim: false,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_creates_professional_reliance_record: false
  };
}

function professionalBoundaryIsClear(): boolean {
  const boundary = professionalBoundary();
  return (
    !boundary.software_bundles_caepipe &&
    !boundary.software_invokes_caepipe_without_user_configuration &&
    !boundary.software_makes_release_claim &&
    !boundary.software_makes_caepipe_compatibility_claim &&
    !boundary.software_makes_solver_validation_claim &&
    !boundary.software_makes_compliance_claim &&
    !boundary.software_makes_certification_claim &&
    !boundary.software_makes_sealing_claim &&
    !boundary.software_makes_approval_claim &&
    !boundary.software_creates_professional_reliance_record
  );
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx",
    source_license: "project-invented metadata only",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented parser-only metadata; no CAEPIPE executable, commercial output, protected standards, or private project payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function textDataHref(text: string): string {
  return `data:text/csv;charset=utf-8,${encodeURIComponent(text)}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase() || "preview";
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
