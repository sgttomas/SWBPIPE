import { Download, Terminal } from "lucide-react";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, ObjectRef, PreviewModel, SolveJobAuditState } from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

const HEADLESS_FINAL_CLI =
  "openpipestress-runner <solve|validate-input|export-results|run-benchmark|run-regression> [--input <request.json>|-] [--output <result.json>]";
const HEADLESS_SETTLED = "SETTLED_DEC_065";

export function HeadlessRunnerPanel({
  model,
  result,
  analysisRun,
  solveJob
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  solveJob: SolveJobAuditState;
}) {
  const packet = buildHeadlessRunnerPacket({ model, result, analysisRun, solveJob });

  return (
    <section className="panel headless-runner-panel" aria-label="Headless runner envelope" data-testid="headless-runner-panel">
      <div className="panel-title">
        <Terminal size={16} aria-hidden="true" />
        Headless Runner
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="headless-runner-export-link"
          download={`openpipestress-preview-headless-runner-${safeFileToken(packet.request.request_id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Runner JSON
        </a>
        <span data-testid="headless-runner-summary">
          available; job={packet.result.job.state}; outputs={packet.request.requested_outputs.length}; result_refs=
          {packet.result.result_refs.length}
        </span>
      </div>
      <div className="report-list" data-testid="headless-runner-body">
        <RunnerLine
          label="Interface"
          value={`${packet.runner_status.interface_kind}; cli=${packet.runner_status.final_cli_command_syntax}; scripts=${packet.runner_status.package_scripts}`}
          testId="headless-runner-interface"
        />
        <RunnerLine
          label="Job"
          value={`${packet.result.job.job_id}; ${packet.result.job.progress.current_step}/${packet.result.job.progress.total_steps}; cancel=${String(
            packet.result.job.cancellation.requested
          )}`}
          testId="headless-runner-job"
        />
        <RunnerLine
          label="Result handoff"
          value={`${packet.result.result_envelope_ref.compatibility}; audit=${packet.result.audit_manifest_ref.ref_id}; checksums=${packet.result.checksums.length}`}
          testId="headless-runner-result-handoff"
        />
        <RunnerLine
          label="Units"
          value={`${packet.result.unit_system_disclosure.unit_system_ref.ref}; ${unitDisclosureSummary(
            packet.result.unit_system_disclosure
          )}`}
          testId="headless-runner-units"
        />
        <RunnerLine
          label="Unit witnesses"
          value={`count=${packet.result.unit_preservation_witnesses.length}; policy=preserve_source_result_units; conversion=${String(
            packet.result.unit_preservation_witnesses.some((item) => item.conversion_performed)
          )}`}
          testId="headless-runner-unit-witnesses"
        />
        <RunnerLine
          label="Runtime Policy"
          value={`process=${packet.tbd_decisions.process_invocation}; network=${packet.tbd_decisions.network_access}; filesystem=${packet.tbd_decisions.filesystem_mutation_policy}`}
          testId="headless-runner-runtime-tbds"
        />
        <RunnerLine
          label="Boundary"
          value={runnerBoundarySummary(packet)}
          testId="headless-runner-boundary"
        />
      </div>
      <small className="report-note">
        Headless runner output is a local schema-first preview envelope under DEC-065; CI, release matrix, public
        transport, and external adapter formats remain TBD.
      </small>
    </section>
  );
}

function RunnerLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildHeadlessRunnerPacket({
  model,
  result,
  analysisRun,
  solveJob
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  solveJob: SolveJobAuditState;
}) {
  const run = analysisRun?.analysis_run;
  const loadBasisRefs = run?.load_basis_refs.map(objectReference) ?? model.load_cases.map((item) => reference("LoadCase", item.id));
  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])].map(runnerDiagnostic);
  const inputManifestRef = run?.reproducibility.input_manifest_refs[0]
    ? objectReference(run.reproducibility.input_manifest_refs[0])
    : reference("InputManifest", "input-manifest:preview-model-not-run");
  const resultRefs = run?.result_refs.map((item) => objectReference(item.result_ref)) ?? [
    reference("ResultSet", "result-set:not-generated")
  ];
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: {},
    conversionPolicy: "headless_runner_result_handoff_preserves_result_envelope_units_no_runner_conversion",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx"
  });
  const unitPreservationWitnesses = headlessUnitPreservationWitnesses(model, result);
  const checksums = run?.hashes.map(checksumRef) ?? [
    {
      algorithm: "TBD",
      canonicalization: "TBD",
      payload_ref: inputManifestRef,
      value: "TBD"
    }
  ];

  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-10-05",
    package_id: "PKG-10",
    scope_items: ["SOW-054", "SOW-032"],
    objectives: ["OBJ-008", "OBJ-009", "OBJ-012"],
    runner_status: {
      interface_kind: "schema_first_headless_runner_contract",
      physical_project_container: physicalProjectContainer(),
      final_cli_command_syntax: HEADLESS_FINAL_CLI,
      package_scripts: "dev_test_convenience_only",
      process_invocation: "single_foreground_local_process",
      network_access: "none",
      filesystem_mutation_policy: "stdout_default_explicit_output_path_only",
      public_transport_protocol: "TBD",
      ci_provider: "TBD",
      release_matrix: "TBD",
      external_adapter_formats: "TBD",
      local_fea_package_format: "TBD"
    },
    tbd_decisions: {
      final_cli_command_syntax: HEADLESS_SETTLED,
      package_scripts: HEADLESS_SETTLED,
      process_invocation: HEADLESS_SETTLED,
      network_access: HEADLESS_SETTLED,
      filesystem_mutation_policy: HEADLESS_SETTLED,
      ci_provider: "TBD",
      release_matrix: "TBD",
      public_transport_protocol: "TBD",
      external_adapter_formats: "TBD"
    },
    request: {
      request_id: `headless-request:${safeFileToken(model.project.id)}:preview-solve`,
      operation: "solve",
      operation_ref: reference("RunnerOperation", "operation:preview-mechanics-solve"),
      project_ref: reference("Project", model.project.id),
      model_ref: reference("Model", result?.model_ref ?? model.project.id),
      unit_system_ref: reference("UnitSystem", `${model.project.id}:units`),
      load_basis_refs: loadBasisRefs.length > 0 ? loadBasisRefs : [reference("LoadCase", "load-basis:not-declared")],
      input_manifest_ref: inputManifestRef,
      requested_outputs: ["result_envelope", "audit_manifest", "diagnostics", "regression_record"],
      privacy: privacyContext(),
      provenance: previewProvenance(),
      professional_boundary: professionalBoundary()
    },
    result: {
      run_id: run?.run_id ?? "run:not-generated",
      job: {
        job_id: solveJob.job_id,
        state: jobState(solveJob),
        progress: {
          current_step: progressStep(solveJob),
          total_steps: 3,
          message: solveJob.events.at(-1)?.message ?? "Preview mechanics job state is not available."
        },
        cancellation: {
          supported: true,
          requested: solveJob.cancellation_requested,
          reason: solveJob.cancellation_status
        }
      },
      analysis_status: run?.analysis_status ?? ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
      result_envelope_ref: {
        schema_ref: "schemas/results.schema.yaml",
        envelope_ref: reference("ResultExportEnvelope", result ? `result-envelope:${result.run_id}` : "result-envelope:not-generated"),
        compatibility: "schema_first_json_result_envelope"
      },
      result_refs: resultRefs,
      audit_manifest_ref: reference("AuditManifest", run ? `audit-manifest:${run.run_id}:preview` : "audit-manifest:not-generated"),
      checksums,
      unit_system_disclosure: unitSystemDisclosure,
      unit_witness_policy: "preserve_source_result_value_unit_and_dimension_per_headless_result_handoff_row",
      unit_preservation_witnesses: unitPreservationWitnesses,
      diagnostics,
      privacy: privacyContext(),
      provenance: previewProvenance(),
      professional_boundary: run?.professional_boundary ?? professionalBoundary()
    }
  };
}

function physicalProjectContainer() {
  return {
    profile: "sqlite_local_project_store",
    decision_ref: "SCA-003",
    storage_role: "local_store_index_projection",
    canonical_truth: "canonical_json_jcs_payload",
    sql_public_contract: false,
    direct_sql_access_allowed: false,
    hosted_db_allowed: false,
    network_required: false,
    sidecars_rebuildable: true
  };
}

function runnerDiagnostic(item: Diagnostic) {
  const affectedRef = item.affected_refs?.[0] ?? "headless-runner:preview";
  return {
    code: item.code,
    class: runnerDiagnosticClass(item),
    severity: item.severity === "error" ? "blocking" : item.severity,
    source: reference("Source", item.source ?? "apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx"),
    affected_object: stableRef(affectedRef),
    message: item.message,
    remediation: "Review preview inputs, diagnostics, private rule data, and the technical-preview runner limits before reliance.",
    provenance: previewProvenance()
  };
}

function runnerDiagnosticClass(item: Diagnostic): string {
  if (item.code.includes("RULE")) return "RULE_CHECK_BLOCKING";
  if (item.code.includes("PROVENANCE")) return "PROVENANCE_WARNING";
  if (item.code.includes("UNIT")) return "UNIT_WARNING";
  if (item.severity === "blocking" || item.severity === "error") return "RUNNER_BLOCKING";
  return "ASSUMPTION_WARNING";
}

function headlessUnitPreservationWitnesses(model: PreviewModel, result: MechanicsResult | null) {
  return (result?.results ?? [])
    .filter((item) => Number.isFinite(item.value))
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .map((item) => {
      const family = resultFamily(item);
      const dimension = resultDimension(family, item.kind);
      return {
        witness_id: `headless-runner-unit:${safeFileToken(item.id)}`,
        source_result_ref: reference("result_value", item.id),
        source_field_path: `mechanics_result.results[${item.id}].value`,
        source_quantity: {
          value: item.value,
          unit: item.unit,
          dimension
        },
        target_result_ref: reference("RunnerResultValue", `headless-result:${safeFileToken(item.id)}`),
        target_field_path: `result.unit_preservation_witnesses[${item.id}].target_quantity`,
        target_quantity: {
          value: item.value,
          unit: item.unit,
          dimension
        },
        target_quantity_policy: "headless_runner_handoff_preserves_source_value_unit_and_dimension",
        export_unit_policy: "preserve_source_result_unit_and_dimension",
        conversion_performed: false,
        unit_system_ref: reference("UnitSystem", `${model.project.id}:units`),
        decision_basis_refs: [
          reference("Decision", "DEC-018"),
          reference("Deliverable", "DEL-02-02"),
          reference("Deliverable", "DEL-08-04"),
          reference("Deliverable", "DEL-10-05")
        ],
        provenance: previewProvenance()
      };
    });
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

function jobState(
  solveJob: SolveJobAuditState
): "QUEUED" | "RUNNING" | "CANCELLATION_REQUESTED" | "CANCELLED" | "COMPLETED" | "FAILED" | "TBD" {
  if (solveJob.state === "queued") return "QUEUED";
  if (solveJob.state === "running") return "RUNNING";
  if (solveJob.state === "cancelling") return "CANCELLATION_REQUESTED";
  if (solveJob.state === "cancelled") return "CANCELLED";
  if (solveJob.state === "completed") return "COMPLETED";
  if (solveJob.state === "failed") return "FAILED";
  return "TBD";
}

function progressStep(solveJob: SolveJobAuditState): number {
  if (solveJob.state === "completed" || solveJob.state === "failed" || solveJob.state === "cancelled") return 3;
  if (solveJob.state === "running" || solveJob.state === "cancelling") return 2;
  if (solveJob.state === "queued") return 1;
  return 0;
}

function checksumRef(item: AnalysisRunEnvelope["analysis_run"]["hashes"][number]) {
  return {
    algorithm: item.algorithm,
    canonicalization: item.canonicalization,
    payload_ref: objectReference(item.payload_ref),
    value: item.value
  };
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

function privacyContext() {
  return {
    local_only: true,
    telemetry_allowed: false,
    private_payload_redacted: true,
    classification: "public_metadata"
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
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

function runnerBoundarySummary(packet: ReturnType<typeof buildHeadlessRunnerPacket>): string {
  const storage = packet.runner_status.physical_project_container;
  return [
    `local=${String(packet.request.privacy.local_only)}`,
    `telemetry=${String(packet.request.privacy.telemetry_allowed)}`,
    `network_required=${String(storage.network_required)}`,
    `direct_sql=${String(storage.direct_sql_access_allowed)}`,
    "no compliance, certification, sealing, authentication, or approval claim"
  ].join("; ");
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase() || "preview";
}
