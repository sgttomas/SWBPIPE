import { Download, FileSearch } from "lucide-react";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";
import type { AnalysisRunEnvelope, MechanicsResult, ObjectRef, PreviewModel } from "../../types";

export function ExternalProverBoundaryPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = buildExternalProverBoundaryPacket({ model, result, analysisRun });

  return (
    <section
      className="panel external-prover-panel"
      aria-label="External prover boundary metadata"
      data-testid="external-prover-panel"
    >
      <div className="panel-title">
        <FileSearch size={16} aria-hidden="true" />
        External Prover Boundary
      </div>
      <div className="report-actions">
        <ControlledExportLink
          className="report-export-link"
          data-testid="external-prover-export-link"
          download={`openpipestress-preview-external-prover-${safeFileToken(packet.metadata_record_id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Metadata JSON
        </ControlledExportLink>
        <span data-testid="external-prover-summary">
          available; metadata={packet.metadata_contract_status}; refs={packet.external_references.length};
          diagnostics={packet.diagnostics.length}
        </span>
      </div>
      <div className="report-list" data-testid="external-prover-body">
        <ExternalProverLine
          label="Contract"
          value={`${packet.deliverable_id}; ${packet.scope_item}; schema=${packet.schema_version}`}
          testId="external-prover-contract"
        />
        <ExternalProverLine
          label="Metadata groups"
          value={`names=${packet.names.length}; tags=${packet.tags.length}; notes=${packet.notes.length}; attachments=${packet.attachments.length}`}
          testId="external-prover-groups"
        />
        <ExternalProverLine
          label="Context links"
          value={`handoff=${packet.handoff_package_refs.length}; mapping=${packet.target_mapping_refs.length}; export=${packet.export_workflow_refs.length}; state=${packet.immutable_model_state_refs.length}`}
          testId="external-prover-context-links"
        />
        <ExternalProverLine
          label="Unsupported flags"
          value={packet.unsupported_target_flags.map((item) => item.behavior_label).join(", ")}
          testId="external-prover-unsupported-flags"
        />
        <ExternalProverLine
          label="Run boundary"
          value={externalRunBoundary(packet)}
          testId="external-prover-run-boundary"
        />
        <ExternalProverLine
          label="Unit policy"
          value={`${unitDisclosureSummary(packet.unit_policy_evidence)}; policy=${packet.unit_policy_evidence.external_prover_unit_policy}; witnesses=${packet.unit_policy_evidence.witness_count}`}
          testId="external-prover-unit-policy"
        />
        <ExternalProverLine
          label="Authority boundary"
          value={authorityBoundary(packet)}
          testId="external-prover-authority-boundary"
        />
      </div>
      <small className="report-note">
        External-prover metadata is descriptive review context only; external tools and target parsers remain outside
        this packet. Validation occurs in the user's accepted professional tools; this packet is screening and handoff
        evidence.
      </small>
    </section>
  );
}

function ExternalProverLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildExternalProverBoundaryPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const metadataRecordId = `external-prover-metadata:${safeFileToken(result?.run_id ?? model.project.id)}`;
  const externalRef = reference("ExternalReference", "external:desktop-preview-metadata-only");
  const handoffRef = reference(
    "HandoffPackage",
    result ? `handoff:${safeFileToken(model.project.id)}:${safeFileToken(result.run_id)}` : "handoff:TBD"
  );
  const targetMappingRef = reference("TargetMapping", "target-mapping:native-open-json-preview");
  const exportWorkflowRef = reference(
    "ExportWorkflow",
    result ? `export-workflow:desktop-preview:${safeFileToken(result.run_id)}` : "export-workflow:TBD"
  );
  const modelStateRef = run?.model_state_ref ?? reference("ModelState", "state:TBD");
  const checksumRefs = run ? run.hashes.map(checksumFromRunHash) : [tbdChecksum(model.project.id, "metadata_context")];
  const provenance = previewProvenance();
  const boundary = professionalBoundary();
  const unitPolicyEvidence = {
    ...buildExportUnitSystemDisclosure({
      model,
      result,
      targetExportUnits: {},
      conversionPolicy: "external_prover_metadata_records_units_without_target_conversion",
      conversionPerformed: false,
      conversionScope: [],
      sourceLocation: "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx"
    }),
    evidence_id: "unit-policy-evidence:external-prover-preview",
    external_prover_scope: "metadata_only_external_review_context",
    external_prover_unit_policy: "record_units_for_external_reviewer_without_invoking_target_solver",
    witness_policy: "record_external_prover_unit_policy_without_claiming_target_writer_conversion",
    witness_count: 1,
    external_ref: externalRef,
    analysis_run_ref: run ? reference("AnalysisRun", run.run_id) : reference("AnalysisRun", "analysis-run:TBD")
  };

  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-15-04",
    package_id: "PKG-15",
    scope_item: "SOW-075",
    objectives: ["OBJ-017", "OBJ-018"],
    metadata_record_id: metadataRecordId,
    metadata_contract_status: "non_authoritative_workflow_metadata",
    names: [
      {
        name_id: "name:desktop-preview-workflow",
        label: "Desktop preview external reference workflow",
        name_kind: "external_workflow_label",
        language: "en",
        provenance
      }
    ],
    tags: [
      "metadata-only",
      "provider-neutral",
      result ? "mechanics-context-bound" : "run-context-pending"
    ].sort(),
    notes: [
      {
        note_id: "note:metadata-boundary",
        note_kind: "metadata_note",
        text: "Invented desktop preview metadata records external review context only; no external tool is invoked.",
        related_refs: [handoffRef, targetMappingRef, exportWorkflowRef, modelStateRef],
        provenance
      }
    ],
    external_references: [
      {
        reference_id: "external-ref:desktop-preview",
        reference_kind: "external_reference",
        display_name: "Desktop preview external workspace placeholder",
        external_ref: externalRef,
        hash_refs: checksumRefs,
        related_refs: [handoffRef, targetMappingRef],
        provenance
      }
    ],
    attachments: [
      {
        attachment_id: "attachment:desktop-preview-note",
        attachment_kind: "external_file_reference",
        display_name: "Desktop preview metadata attachment reference",
        uri_or_path_ref: reference("ExternalReference", "attachment:desktop-preview-note"),
        content_hash: checksumRefs[0],
        privacy_classification: "invented_public_example",
        payload_embedded: false,
        related_refs: [exportWorkflowRef],
        provenance
      }
    ],
    handoff_package_refs: [
      metadataLink("handoff:desktop-preview", "handoff_package", handoffRef, checksumRefs, "context_for_external_review")
    ],
    target_mapping_refs: [
      metadataLink(
        "target-mapping:desktop-preview",
        "target_mapping",
        targetMappingRef,
        checksumRefs,
        "stable_id_mapping_context"
      )
    ],
    export_workflow_refs: [
      metadataLink("export-workflow:desktop-preview", "export_workflow", exportWorkflowRef, checksumRefs, "local_export_context")
    ],
    immutable_model_state_refs: [
      metadataLink("model-state:desktop-preview", "model_state", modelStateRef, checksumRefs, "hash_bound_state_context")
    ],
    unit_policy_evidence: unitPolicyEvidence,
    assumptions: [
      {
        assumption_id: "assumption:external-context-review",
        statement: "External context remains metadata-only until a user supplies and reviews target-specific artifacts.",
        affected_refs: [externalRef, handoffRef],
        provenance
      }
    ],
    warnings: [
      {
        code: "EPM-DESKTOP-PREVIEW-NO-EXTERNAL-RUN",
        severity: "warning",
        message: "The desktop preview did not invoke an external prover or parse commercial result payloads.",
        affected_refs: [externalRef],
        provenance
      }
    ],
    unsupported_target_flags: [
      unsupportedTargetFlag(
        "unsupported-target:external-solver-not-invoked",
        "external_solver_not_invoked",
        externalRef,
        [exportWorkflowRef]
      ),
      unsupportedTargetFlag(
        "unsupported-target:commercial-result-ingestion-deferred",
        "commercial_result_ingestion_deferred",
        externalRef,
        [externalRef]
      ),
      unsupportedTargetFlag(
        "unsupported-target:formal-prover-status-not-generated",
        "formal_prover_status_not_generated",
        externalRef,
        [handoffRef, modelStateRef]
      )
    ],
    proposed_authority_claims: [] as Array<Record<string, never>>,
    diagnostics: externalProverDiagnostics({ metadataRecordId, externalRef, boundary, provenance }),
    provenance,
    professional_boundary: boundary
  };
}

function metadataLink(
  linkId: string,
  referenceKind: "handoff_package" | "target_mapping" | "export_workflow" | "model_state",
  ref: ReturnType<typeof reference>,
  hashRefs: ReturnType<typeof tbdChecksum>[],
  relationship: string
) {
  return {
    link_id: linkId,
    reference_kind: referenceKind,
    ref,
    hash_refs: hashRefs,
    relationship,
    provenance: previewProvenance()
  };
}

function unsupportedTargetFlag(
  flagId: string,
  behaviorLabel: string,
  targetRef: ReturnType<typeof reference>,
  affectedRefs: ReturnType<typeof reference>[]
) {
  return {
    flag_id: flagId,
    behavior_label: behaviorLabel,
    status: "not_implemented",
    target_ref: targetRef,
    affected_refs: affectedRefs,
    assumption_refs: [reference("Assumption", "assumption:external-context-review")],
    warning_refs: [reference("Diagnostic", "EPM-DESKTOP-PREVIEW-NO-EXTERNAL-RUN")],
    human_review_required: true,
    provenance: previewProvenance()
  };
}

function externalProverDiagnostics({
  metadataRecordId,
  externalRef,
  boundary,
  provenance
}: {
  metadataRecordId: string;
  externalRef: ReturnType<typeof reference>;
  boundary: ReturnType<typeof professionalBoundary>;
  provenance: ReturnType<typeof previewProvenance>;
}) {
  return [
    diagnostic(
      "EPM-DESKTOP-METADATA-ONLY",
      "info",
      "EXTERNAL_PROVER_METADATA_BOUNDARY",
      "External-prover metadata is available as descriptive review context.",
      "Keep records as names, tags, notes, references, and hash-bound links.",
      [reference("ExternalProverMetadata", metadataRecordId)],
      provenance
    ),
    diagnostic(
      "EPM-DESKTOP-EXTERNAL-EXECUTION-NOT-INVOKED",
      "warning",
      "EXTERNAL_PROVER_METADATA_BOUNDARY",
      "No external prover was invoked by the desktop preview packet.",
      "Use this packet as metadata context only and keep external execution user-controlled.",
      [externalRef],
      provenance
    ),
    diagnostic(
      "EPM-DESKTOP-COMMERCIAL-RESULT-NOT-INGESTED",
      "warning",
      "EXTERNAL_PROVER_METADATA_BOUNDARY",
      "Commercial stress-software result payloads are not ingested by this preview packet.",
      "Record references and checksums only unless a later target parser is explicitly authorized.",
      [externalRef],
      provenance
    ),
    diagnostic(
      boundary.external_tool_invoked ? "EPM-DESKTOP-BOUNDARY-REVIEW" : "EPM-DESKTOP-AUTHORITY-FLAGS-FALSE",
      boundary.external_tool_invoked ? "blocking" : "info",
      "AUTHORITY_BOUNDARY",
      "Software authority flags are false for this metadata envelope.",
      "Preserve metadata-only status and keep human-owned acceptance outside software-generated status fields.",
      [reference("ProfessionalBoundary", "external-prover-boundary")],
      provenance
    )
  ];
}

function diagnostic(
  code: string,
  severity: "info" | "warning" | "blocking",
  diagnosticClass: "EXTERNAL_PROVER_METADATA_BOUNDARY" | "AUTHORITY_BOUNDARY",
  message: string,
  remediation: string,
  affectedReferences: ReturnType<typeof reference>[],
  provenance: ReturnType<typeof previewProvenance>
) {
  return {
    code,
    severity,
    class: diagnosticClass,
    source: reference("ExternalReference", "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx"),
    affected_references: affectedReferences,
    message,
    remediation,
    provenance
  };
}

function checksumFromRunHash(item: AnalysisRunEnvelope["analysis_run"]["hashes"][number]) {
  return {
    algorithm: item.algorithm === "sha256" || item.algorithm === "sha512" ? item.algorithm : "TBD",
    canonicalization: item.canonicalization,
    payload_ref: objectReference(item.payload_ref),
    payload_scope: item.payload_scope,
    value: item.value
  };
}

function objectReference(item: ObjectRef) {
  return reference(item.object_type, item.ref);
}

function tbdChecksum(projectId: string, payloadScope: string) {
  return {
    algorithm: "TBD",
    canonicalization: "TBD",
    payload_ref: reference("Project", projectId),
    payload_scope: payloadScope,
    value: "TBD"
  };
}

function reference(objectType: string, ref: string) {
  return {
    object_type: objectType,
    ref
  };
}

function externalRunBoundary(packet: ReturnType<typeof buildExternalProverBoundaryPacket>): string {
  return [
    `tool_invoked=${String(packet.professional_boundary.external_tool_invoked)}`,
    `commercial_results=${String(packet.professional_boundary.commercial_result_payload_ingested)}`,
    `metadata_only=${String(packet.professional_boundary.metadata_only)}`
  ].join("; ");
}

function authorityBoundary(packet: ReturnType<typeof buildExternalProverBoundaryPacket>): string {
  return [
    `human_review=${String(packet.professional_boundary.human_review_required)}`,
    `compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}`,
    `certification=${String(packet.professional_boundary.software_makes_certification_claim)}`,
    `sealing=${String(packet.professional_boundary.software_makes_sealing_claim)}`,
    `authentication=${String(packet.professional_boundary.software_makes_authentication_claim)}`,
    `acceptance_record=${String(packet.professional_boundary.software_creates_professional_reliance_record)}`
  ].join("; ");
}

function professionalBoundary() {
  return {
    human_review_required: true,
    metadata_only: true,
    external_tool_invoked: false,
    commercial_result_payload_ingested: false,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false,
    software_creates_professional_reliance_record: false,
    software_creates_external_validation_record: false
  };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx",
    source_license: "project-invented-preview-data",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented metadata only; no protected standards, private project payloads, or commercial result payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending",
    privacy_classification: "public_metadata"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9:_-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase() || "preview";
}
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
