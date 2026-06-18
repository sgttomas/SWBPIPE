import { Download, FileJson } from "lucide-react";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult, ObjectRef, PreviewModel } from "../../types";

export function AdapterFrameworkPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = buildAdapterFrameworkPacket({ model, result, analysisRun });

  return (
    <section className="panel adapter-framework-panel" aria-label="Adapter framework envelope" data-testid="adapter-framework-panel">
      <div className="panel-title">
        <FileJson size={16} aria-hidden="true" />
        Adapter Framework
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="adapter-framework-export-link"
          download={`openpipestress-preview-adapter-framework-${safeFileToken(packet.adapter_declaration.adapter_id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Adapter JSON
        </a>
        <span data-testid="adapter-framework-summary">
          available; capabilities={packet.adapter_declaration.capabilities.length}; parse=
          {packet.operation_result.parse_status}; diagnostics={packet.operation_result.diagnostics.length}
        </span>
      </div>
      <div className="report-list" data-testid="adapter-framework-body">
        <AdapterLine
          label="Framework"
          value={`${packet.framework_status.interface_kind}; formats=${packet.framework_status.external_format_list}; transport=${packet.framework_status.public_transport_protocol}`}
          testId="adapter-framework-status"
        />
        <AdapterLine
          label="Capabilities"
          value={packet.adapter_declaration.capabilities.join(", ")}
          testId="adapter-framework-capabilities"
        />
        <AdapterLine
          label="Validation"
          value={`schema=${packet.validation_plan.schema_validation}; units=${packet.validation_plan.unit_validation}; protected=${packet.validation_plan.protected_content_screening}; export=${packet.validation_plan.export_review}`}
          testId="adapter-framework-validation"
        />
        <AdapterLine
          label="Units"
          value={`${unitDisclosureSummary(packet.unit_policy_evidence)}; policy=${packet.unit_policy_evidence.framework_unit_policy}; witnesses=${packet.unit_policy_evidence.witness_count}`}
          testId="adapter-framework-units"
        />
        <AdapterLine
          label="No bypass"
          value={noBypassSummary(packet.adapter_declaration.no_bypass_controls)}
          testId="adapter-framework-no-bypass"
        />
        <AdapterLine
          label="Runtime TBDs"
          value={`plugin=${packet.tbd_decisions.plugin_runtime}; endpoint=${packet.tbd_decisions.endpoint_syntax}; package_scripts=${packet.tbd_decisions.package_scripts}`}
          testId="adapter-framework-runtime-tbds"
        />
        <AdapterLine
          label="Boundary"
          value={adapterBoundarySummary(packet)}
          testId="adapter-framework-boundary"
        />
      </div>
      <small className="report-note">
        Adapter framework output is a local format-neutral declaration only; concrete external formats, public transport,
        plugin runtime, package scripts, CI, release matrix, redaction workflow, and filesystem roots remain TBD.
      </small>
    </section>
  );
}

function AdapterLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildAdapterFrameworkPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])].map(adapterDiagnostic);
  const unitPolicyEvidence = {
    ...buildExportUnitSystemDisclosure({
      model,
      result,
      targetExportUnits: {},
      conversionPolicy: "adapter_framework_declares_unit_validation_no_format_conversion",
      conversionPerformed: false,
      conversionScope: [],
      sourceLocation: "apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx"
    }),
    evidence_id: "unit-policy-evidence:adapter-framework-preview",
    framework_scope: "format_neutral_adapter_contract_metadata_only",
    framework_unit_policy: "unit_validation_required_before_adapter_payload_exchange",
    witness_policy: "record_framework_unit_policy_without_claiming_target_writer_conversion",
    witness_count: 1,
    adapter_ref: reference("adapter", "ops.adapter.desktop_preview")
  };

  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-10-02",
    package_id: "PKG-10",
    scope_item: "SOW-030",
    objective: "OBJ-009",
    framework_status: frameworkStatus(),
    tbd_decisions: tbdDecisions(),
    adapter_declaration: {
      adapter_id: "ops.adapter.desktop_preview",
      adapter_name: "Desktop preview format-neutral adapter declaration",
      adapter_kind: "format_neutral_contract",
      format_status: "TBD",
      capabilities: ["export_model", "export_results", "validate_payload", "contribution_review"],
      operation_boundary: {
        api_operation_id: "ops.export.job",
        operation_category: "job",
        mutability: "metadata_only",
        transport: "TBD"
      },
      provenance: previewProvenance(),
      privacy: privacyContext(),
      no_bypass_controls: noBypassControls(),
      professional_boundary: professionalBoundary()
    },
    validation_plan: {
      syntactic_parse: "not_performed_by_framework",
      schema_validation: "required",
      unit_validation: "required",
      dimension_validation: "required",
      provenance_validation: "required",
      redistribution_review: "required",
      privacy_classification: "required",
      protected_content_screening: "required",
      mechanics_readiness: "required_for_solve",
      rule_check_readiness: "required_for_rule_check",
      export_review: "required_before_shared_payload",
      human_review_required: true
    },
    unit_policy_evidence: unitPolicyEvidence,
    operation_result: {
      operation_id: `adapter-operation:desktop-preview:${safeFileToken(result?.run_id ?? "not-run")}`,
      operation_class: "export",
      parse_status: "not_parsed_by_framework",
      validation_plan_ref: reference("operation", "validation-plan:desktop-preview-format-neutral-adapter"),
      diagnostics,
      privacy: privacyContext(),
      provenance: previewProvenance(),
      checksums: run ? run.hashes.map(checksumRef) : [tbdChecksum(model.project.id)],
      audit_manifest_refs: [
        reference("audit_manifest", run ? `audit-manifest:${run.run_id}:preview` : "audit-manifest:not-generated")
      ],
      result_envelope_ref: {
        schema_ref: "schemas/results.schema.yaml",
        compatibility: "schema_first_json_result_envelope",
        ref: reference("result_envelope", result ? `result-envelope:${result.run_id}` : "result-envelope:not-generated")
      },
      professional_boundary: professionalBoundary()
    }
  };
}

function frameworkStatus() {
  return {
    interface_kind: "schema_first_format_neutral_adapter_framework",
    external_format_list: "TBD",
    public_transport_protocol: "TBD",
    endpoint_syntax: "TBD",
    adapter_execution_model: "TBD",
    plugin_runtime: "TBD",
    permission_grant_persistence: "TBD",
    physical_project_container: "TBD",
    local_fea_package_format: "TBD",
    redaction_workflow: "TBD",
    ci_provider: "TBD",
    release_matrix: "TBD"
  };
}

function tbdDecisions() {
  return {
    external_format_list: "TBD",
    public_transport_protocol: "TBD",
    endpoint_syntax: "TBD",
    adapter_execution_model: "TBD",
    plugin_runtime: "TBD",
    permission_grant_persistence: "TBD",
    package_scripts: "TBD",
    ci_provider: "TBD",
    release_matrix: "TBD",
    physical_project_container: "TBD",
    local_fea_package_format: "TBD",
    redaction_workflow: "TBD"
  };
}

function noBypassControls() {
  return {
    must_use_public_api_boundary: true,
    must_use_unit_validation: true,
    must_preserve_provenance: true,
    must_preserve_redistribution_review: true,
    must_preserve_privacy_classification: true,
    must_screen_protected_content: true,
    must_preserve_diagnostics: true,
    must_preserve_rule_pack_sandbox: true,
    must_preserve_persistence_hash_controls: true,
    must_route_persistence_through_application_services: true,
    must_not_expose_sql_or_raw_sqlite: true,
    must_not_expose_table_names: true,
    must_not_mutate_project_store_directly: true,
    must_preserve_report_controls: true,
    must_preserve_human_acceptance_boundary: true,
    must_not_execute_arbitrary_code: true,
    must_not_access_network: true,
    must_not_choose_filesystem_roots: true,
    must_not_claim_code_compliance: true,
    must_not_transmit_private_data_by_default: true
  };
}

function adapterDiagnostic(item: Diagnostic) {
  const affectedRef = item.affected_refs?.[0] ?? "adapter-framework:preview";
  return {
    code: item.code,
    class: adapterDiagnosticClass(item),
    severity: item.severity === "error" || item.severity === "blocking" ? "blocking" : item.severity,
    source: reference("diagnostic", item.source ?? "apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx"),
    affected_object: reference("payload", affectedRef),
    message: item.message,
    remediation: "Review adapter provenance, units, privacy, protected-content screening, and technical-preview limitations before sharing.",
    provenance: previewProvenance()
  };
}

function adapterDiagnosticClass(item: Diagnostic): string {
  if (item.code.includes("RULE")) return "RULE_CHECK_BLOCKING";
  if (item.code.includes("PROVENANCE")) return "PROVENANCE_WARNING";
  if (item.code.includes("UNIT")) return "UNIT_WARNING";
  if (item.severity === "blocking" || item.severity === "error") return "ADAPTER_BLOCKING";
  return "ASSUMPTION_WARNING";
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
  return reference("payload", `${item.object_type}:${item.ref}`);
}

function tbdChecksum(projectId: string) {
  return {
    algorithm: "TBD",
    canonicalization: "TBD",
    payload_ref: reference("payload", projectId),
    value: "TBD"
  };
}

function reference(refType: "schema" | "adapter" | "operation" | "payload" | "diagnostic" | "audit_manifest" | "result_envelope" | "TBD", refId: string) {
  return {
    ref_type: refType,
    ref_id: refId
  };
}

function privacyContext() {
  return {
    classification: "export_review_required",
    local_first: true,
    telemetry_allowed: false,
    export_review_required: true,
    private_payload_redacted: true
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    mechanics_solve_distinct: true,
    user_rule_check_distinct: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_security_certification_claim: false
  };
}

function previewProvenance() {
  return {
    source_name: "invented preview fixture",
    source_location: "fixtures/product_preview",
    source_license: "project-invented",
    contributor: "OpenPipeStress project",
    contributor_certification: "invented data; no protected or private payload",
    redistribution_status: "public_permissive",
    review_status: "accepted"
  };
}

function noBypassSummary(controls: ReturnType<typeof noBypassControls>): string {
  return [
    `api=${String(controls.must_use_public_api_boundary)}`,
    `units=${String(controls.must_use_unit_validation)}`,
    `direct_sql=${String(!controls.must_not_expose_sql_or_raw_sqlite)}`,
    `network=${String(!controls.must_not_access_network)}`,
    `code_compliance_claim=${String(!controls.must_not_claim_code_compliance)}`
  ].join("; ");
}

function adapterBoundarySummary(packet: ReturnType<typeof buildAdapterFrameworkPacket>): string {
  return [
    `local_first=${String(packet.adapter_declaration.privacy.local_first)}`,
    `telemetry=${String(packet.adapter_declaration.privacy.telemetry_allowed)}`,
    `private_redacted=${String(packet.adapter_declaration.privacy.private_payload_redacted)}`,
    "no compliance, certification, sealing, approval, or security-certification claim"
  ].join("; ");
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase() || "preview";
}
