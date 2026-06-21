import { ClipboardCheck, Download } from "lucide-react";
import type {
  AgentProposal,
  EditorOperationIntent,
  LocalProjectSummary,
  LocalStorageCapability,
  ModelDocumentMigrationStatus,
  ModelHashEvidence,
  ModelMigrationLedgerRecord,
  ModelHashIntegrityEvidence,
  ObjectRef,
  PreviewModel,
  ProjectEnvelopeHashEvidence,
  ProjectEnvelopeHashIntegrityEvidence
} from "../../types";

type RoundTripCategory = {
  category: string;
  semantic_equality_status: string;
  checked_ref_count: number;
  silent_default_inserted: false;
  protected_content_expanded: false;
  private_payload_expanded: false;
};

export function ProjectValidationPanel({
  model,
  storageCapability,
  projectSummary,
  projectOperation,
  editorIntents,
  proposal,
  modelHash,
  modelHashIntegrity,
  projectEnvelopeHash,
  projectEnvelopeHashIntegrity,
  modelDocumentMigration,
  modelMigrationLedger
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null;
  projectEnvelopeHashIntegrity: ProjectEnvelopeHashIntegrityEvidence | null;
  modelDocumentMigration: ModelDocumentMigrationStatus | null;
  modelMigrationLedger: ModelMigrationLedgerRecord[];
}) {
  const packet = buildProjectValidationPacket({
    model,
    storageCapability,
    projectSummary,
    projectOperation,
    editorIntents,
    proposal,
    modelHash,
    modelHashIntegrity,
    projectEnvelopeHash,
    projectEnvelopeHashIntegrity,
    modelDocumentMigration,
    modelMigrationLedger
  });

  return (
    <section
      className="panel project-validation-panel"
      aria-label="Project validation preflight"
      data-testid="project-validation-panel"
    >
      <div className="panel-title">
        <ClipboardCheck size={16} aria-hidden="true" />
        Project Validation Preflight
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="project-validation-export-link"
          download={`openpipestress-preview-validation-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Validation JSON
        </a>
        <span data-testid="project-validation-summary">
          validation={packet.summary.validation_status}; version=
          {packet.summary.version_check_status}; migration=
          {packet.summary.migration_status}; round_trip=
          {packet.summary.round_trip_status}
        </span>
      </div>
      <div className="report-list" data-testid="project-validation-body">
        <ValidationLine
          label="Schema and version"
          value={`schema_version=${packet.summary.schema_version}; document_kind=${packet.summary.document_kind}; profile=${packet.validation_profile.profile_id}`}
          testId="project-validation-schema"
        />
        <ValidationLine
          label="Round-trip manifest"
          value={`${packet.round_trip_manifest.category_count} categories; unit metadata=${categoryStatus(
            packet.round_trip_manifest.categories,
            "unit_metadata"
          )}; provenance=${categoryStatus(
            packet.round_trip_manifest.categories,
            "provenance_metadata"
          )}; reproducibility=${categoryStatus(packet.round_trip_manifest.categories, "reproducibility_metadata")}`}
          testId="project-validation-round-trip"
        />
        <ValidationLine
          label="Unit round-trip evidence"
          value={`status=${packet.summary.unit_round_trip_status}; checked_refs=${
            packet.summary.unit_round_trip_checked_ref_count
          }; signature=${packet.summary.unit_round_trip_signature}`}
          testId="project-validation-unit-round-trip"
        />
        <ValidationLine
          label="Unit policy"
          value={projectValidationUnitPolicySummary(packet.unit_policy_evidence)}
          testId="project-validation-unit-policy"
        />
        <ValidationLine
          label="Model hash evidence"
          value={`model_hash=${packet.summary.model_hash_status}; persisted_model_hashes=${
            packet.summary.persisted_model_hash_count
          }; persisted_model_hash_ref=${packet.summary.persisted_model_hash_ref}; integrity=${
            packet.summary.model_hash_integrity_status
          }`}
          testId="project-validation-model-hash"
        />
        <ValidationLine
          label="Envelope hash evidence"
          value={`envelope_hash=${packet.summary.project_envelope_hash_status}; persisted_envelope_hashes=${
            packet.summary.persisted_project_envelope_hash_count
          }; persisted_envelope_hash_ref=${packet.summary.persisted_project_envelope_hash_ref}; integrity=${
            packet.summary.project_envelope_hash_integrity_status
          }`}
          testId="project-validation-envelope-hash"
        />
        <ValidationLine
          label="Store migration evidence"
          value={`framework=${packet.store_migration.migration_framework}; store_schema_version=${
            packet.store_migration.store_schema_version
          }; target=${packet.store_migration.store_schema_target_version}; applied_on_open=${
            packet.store_migration.migrations_applied_on_open.length
          }; status=${packet.store_migration.migration_status}`}
          testId="project-validation-store-migration"
        />
        <ValidationLine
          label="Model document migration"
          value={`status=${packet.model_document_migration.status}; source=${
            packet.model_document_migration.source_schema_version
          }; target=${packet.model_document_migration.target_schema_version}; framework=${
            packet.model_document_migration.migration_framework
          }; persistence=${packet.model_document_migration.persistence_state}; ledger_records=${
            packet.model_document_migration.ledger_record_count
          }`}
          testId="project-validation-model-document-migration"
        />
        <ValidationLine
          label="Persistence operations"
          value={`validate=${operationStatus(packet.service_operations, "validate")}; version_check=${operationStatus(
            packet.service_operations,
            "version_check"
          )}; migrate=${operationStatus(packet.service_operations, "migrate")}; pending operations=${
            packet.summary.pending_operation_count
          }; persisted editor intents=${packet.summary.persisted_editor_intent_count}; proposals=${
            packet.summary.proposal_operation_count
          }; persisted proposals=${packet.summary.persisted_proposal_count}; persisted review targets=${
            packet.summary.persisted_selected_review_target_count
          }; persisted review target ref=${
            packet.summary.persisted_selected_review_target_ref
          }; persisted mechanics results=${packet.summary.persisted_mechanics_result_count}; persisted analysis runs=${
            packet.summary.persisted_analysis_run_count
          }; persisted analysis run ref=${packet.summary.persisted_analysis_run_ref}`}
          testId="project-validation-operations"
        />
        <ValidationLine
          label="Validation boundary"
          value={`accepted_state_mutated=${String(
            packet.summary.accepted_model_state_mutated
          )}; network=${String(packet.boundary.network_required)}; telemetry=${String(
            packet.boundary.telemetry_enabled
          )}; private/protected payload=false`}
          testId="project-validation-boundary"
        />
      </div>
      <small className="report-note">
        Validation preflight records version, migration, and round-trip review evidence for the local technical preview
        without applying queued operations or making professional reliance claims.
      </small>
    </section>
  );
}

function ValidationLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildProjectValidationPacket({
  model,
  storageCapability,
  projectSummary,
  projectOperation,
  editorIntents,
  proposal,
  modelHash,
  modelHashIntegrity,
  projectEnvelopeHash,
  projectEnvelopeHashIntegrity,
  modelDocumentMigration,
  modelMigrationLedger
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null;
  projectEnvelopeHashIntegrity: ProjectEnvelopeHashIntegrityEvidence | null;
  modelDocumentMigration: ModelDocumentMigrationStatus | null;
  modelMigrationLedger: ModelMigrationLedgerRecord[];
}) {
  const modelHashStatus = modelHashEvidenceStatus({
    modelHash,
    projectSummary,
    modelHashIntegrity
  });
  const envelopeHashStatus = projectEnvelopeHashEvidenceStatus({
    projectEnvelopeHash,
    projectSummary,
    projectEnvelopeHashIntegrity
  });
  const categories = buildRoundTripCategories(model, modelHashStatus);
  const unitPolicyEvidence = buildProjectValidationUnitPolicyEvidence({
    model,
    projectSummary
  });
  const migrationStatus = projectSummary?.migration_status ?? "not_persisted_this_session";
  const storeMigration = buildStoreMigrationEvidence({
    projectSummary,
    storageCapability
  });
  const versionCheckStatus =
    model.schema_version === "0.1.0" ? "supported_current_schema" : "unsupported_schema_review_required";
  const validationStatus = projectSummary ? "preview_current" : "preview_not_persisted";
  const proposalCount = proposal ? 1 : 0;
  const pendingOperationCount = editorIntents.length + proposalCount;
  const reviewOperationStatuses = unique([
    ...editorIntents.map((intent) => intent.validation.application_status),
    ...(proposal ? [proposal.validation.application_status] : [])
  ]);

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.project_validation_preflight",
    export_scope: "local_browser_download_preview",
    deliverable_refs: ["DEL-02-02", "DEL-02-05", "DEL-12-01"],
    scope_items: ["SOW-050", "SOW-041", "SOW-029"],
    objectives: ["OBJ-001", "OBJ-010", "OBJ-012"],
    project_ref: model.project.id,
    project_name: model.project.name,
    summary: {
      validation_status: validationStatus,
      version_check_status: versionCheckStatus,
      migration_status: migrationStatus,
      round_trip_status: "semantic_categories_declared",
      schema_version: model.schema_version,
      document_kind: model.document_kind,
      storage_mode: projectSummary?.storage_mode ?? "not_persisted_this_session",
      last_operation: projectOperation,
      pending_operation_count: pendingOperationCount,
      editor_intent_count: editorIntents.length,
      persisted_editor_intent_count: projectSummary?.editor_intent_count ?? 0,
      proposal_operation_count: proposalCount,
      persisted_proposal_count: projectSummary?.proposal_count ?? 0,
      persisted_selected_review_target_count: projectSummary?.selected_review_target_count ?? 0,
      persisted_selected_review_target_ref: projectSummary?.selected_review_target_ref ?? "not_selected",
      persisted_mechanics_result_count: projectSummary?.persisted_mechanics_result_count ?? 0,
      persisted_analysis_run_count: projectSummary?.persisted_analysis_run_count ?? 0,
      persisted_analysis_run_ref: projectSummary?.persisted_analysis_run_ref ?? "not_persisted",
      unit_round_trip_status: projectSummary?.unit_round_trip_status ?? "not_persisted_this_session",
      unit_round_trip_checked_ref_count: projectSummary?.unit_round_trip_checked_ref_count ?? 0,
      unit_round_trip_signature: projectSummary?.unit_round_trip_signature ?? "not_persisted",
      model_hash_status: modelHashStatus,
      persisted_model_hash_count: projectSummary?.persisted_model_hash_count ?? 0,
      persisted_model_hash_ref: projectSummary?.persisted_model_hash_ref ?? "not_persisted",
      model_hash_integrity_status: modelHashIntegrity?.integrity_status ?? "open_verification_not_run_this_session",
      project_envelope_hash_status: envelopeHashStatus,
      persisted_project_envelope_hash_count: projectSummary?.persisted_project_envelope_hash_count ?? 0,
      persisted_project_envelope_hash_ref: projectSummary?.persisted_project_envelope_hash_ref ?? "not_persisted",
      project_envelope_hash_integrity_status:
        projectEnvelopeHashIntegrity?.integrity_status ?? "open_verification_not_run_this_session",
      accepted_model_state_mutated: false,
      copied_external_files: Boolean(projectSummary?.copied_external_files),
      network_required: Boolean(storageCapability?.network_required),
      daemon_required: Boolean(storageCapability?.daemon_required),
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled)
    },
    validation_profile: {
      profile_id: "technical_preview_project_persistence_preflight",
      json_schema_baseline: "2020-12",
      canonicalization_basis: "JCS-compatible",
      hash_service_status: modelHash
        ? "canonical_model_hash_service_available_model_payload_scope"
        : "model_hash_service_unavailable_in_this_runtime",
      project_envelope_hash_status: envelopeHashStatus,
      project_envelope_hash_scope: "persisted_envelope_payload_excluding_storage_summary_and_hash_carrier",
      physical_container_status: projectSummary ? projectSummary.storage_mode : "not_persisted_this_session",
      store_migration_framework_status: storeMigration.migration_framework,
      model_document_migration_status: modelDocumentMigrationEvidence(
        model,
        modelDocumentMigration,
        modelMigrationLedger
      ).status
    },
    round_trip_manifest: {
      category_count: categories.length,
      categories,
      parse_validate_normalize_serialize_parse_basis: "preview_fixture_semantic_category_review",
      volatile_fields: ["storage.database_path", "storage.message"],
      normalization_rules: [
        "stable IDs and typed references are compared semantically",
        "unit-bearing values must retain explicit unit metadata",
        "missing optional rule-pack references remain explicit not-present records"
      ]
    },
    unit_round_trip_evidence: {
      status: projectSummary?.unit_round_trip_status ?? "not_persisted_this_session",
      checked_ref_count: projectSummary?.unit_round_trip_checked_ref_count ?? 0,
      signature: projectSummary?.unit_round_trip_signature ?? "not_persisted",
      evidence_source: projectSummary ? "local_project_summary" : "not_persisted_this_session",
      comparison_basis: "deterministic_unit_metadata_signature_from_restored_local_project_envelope"
    },
    unit_policy_evidence: unitPolicyEvidence,
    service_operations: buildServiceOperations({
      projectSummary,
      projectOperation,
      validationStatus,
      versionCheckStatus
    }),
    store_migration: storeMigration,
    model_document_migration: modelDocumentMigrationEvidence(model, modelDocumentMigration, modelMigrationLedger),
    storage_capability: storageCapability,
    project_summary: projectSummary,
    model_hash: modelHash,
    model_hash_integrity: modelHashIntegrity,
    project_envelope_hash: projectEnvelopeHash,
    project_envelope_hash_integrity: projectEnvelopeHashIntegrity,
    editor_intent_refs: editorIntents.map((intent) => intent.operation_id),
    proposal_refs: proposal ? [proposal.proposal_id] : [],
    editor_operation_statuses: unique(editorIntents.map((intent) => intent.validation.application_status)),
    review_operation_statuses: reviewOperationStatuses,
    boundary: {
      local_only_project_store: true,
      repository_default_private_write: false,
      external_file_copy_performed: Boolean(projectSummary?.copied_external_files),
      network_required: Boolean(storageCapability?.network_required),
      daemon_required: Boolean(storageCapability?.daemon_required),
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled),
      accepted_model_state_mutated: false,
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false
    },
    diagnostics: validationDiagnostics({
      storageCapability,
      projectSummary,
      versionCheckStatus,
      modelHash,
      modelHashIntegrity,
      projectEnvelopeHash,
      projectEnvelopeHashIntegrity
    }),
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
}

type ProjectValidationUnitPolicyEvidence = {
  evidence_id: string;
  unit_system_ref: ObjectRef;
  source_model_ref: ObjectRef;
  storage_convention: "entered_units_preserved";
  validation_unit_policy: string;
  model_units: Record<string, string>;
  unit_bearing_record_count: number;
  unit_round_trip_status: string;
  unit_round_trip_checked_ref_count: number;
  unit_round_trip_signature: string;
  conversion_policy: string;
  conversion_performed: false;
  decision_basis_refs: ObjectRef[];
  protected_content_included: false;
  private_payload_included: false;
};

function buildProjectValidationUnitPolicyEvidence({
  model,
  projectSummary
}: {
  model: PreviewModel;
  projectSummary: LocalProjectSummary | null;
}): ProjectValidationUnitPolicyEvidence {
  return {
    evidence_id: "unit-policy-evidence:project-validation-preflight",
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    source_model_ref: reference("Model", model.project.id),
    storage_convention: "entered_units_preserved",
    validation_unit_policy: "validate_round_trip_preserves_explicit_model_unit_metadata_without_conversion",
    model_units: sortedStringRecord(model.project.units),
    unit_bearing_record_count: countUnitBearingRecords(model),
    unit_round_trip_status: projectSummary?.unit_round_trip_status ?? "not_persisted_this_session",
    unit_round_trip_checked_ref_count: projectSummary?.unit_round_trip_checked_ref_count ?? 0,
    unit_round_trip_signature: projectSummary?.unit_round_trip_signature ?? "not_persisted",
    conversion_policy: "project_validation_records_unit_round_trip_metadata_without_conversion",
    conversion_performed: false,
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-02-05")
    ],
    protected_content_included: false,
    private_payload_included: false
  };
}

function projectValidationUnitPolicySummary(evidence: ProjectValidationUnitPolicyEvidence): string {
  return [
    `model=${formatUnitRecord(evidence.model_units)}`,
    `records=${evidence.unit_bearing_record_count}`,
    `round_trip=${evidence.unit_round_trip_status}`,
    `conversion=${String(evidence.conversion_performed)}`
  ].join("; ");
}

function modelHashEvidenceStatus({
  modelHash,
  projectSummary,
  modelHashIntegrity
}: {
  modelHash: ModelHashEvidence | null;
  projectSummary: LocalProjectSummary | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
}): string {
  if (modelHashIntegrity?.integrity_status === "verified_match") return "model_hash_verified_on_open";
  if (modelHashIntegrity?.integrity_status === "mismatch_review_required") return "model_hash_mismatch_review_required";
  if (modelHashIntegrity?.integrity_status === "hash_recompute_unavailable") {
    return "model_hash_recompute_unavailable_review_required";
  }
  if ((projectSummary?.persisted_model_hash_count ?? 0) > 0) return "model_hash_persisted_open_verification_not_run";
  if (modelHash) return "model_hash_computed_not_persisted";
  return "model_hash_service_unavailable_in_this_runtime";
}

function projectEnvelopeHashEvidenceStatus({
  projectEnvelopeHash,
  projectSummary,
  projectEnvelopeHashIntegrity
}: {
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null;
  projectSummary: LocalProjectSummary | null;
  projectEnvelopeHashIntegrity: ProjectEnvelopeHashIntegrityEvidence | null;
}): string {
  if (projectEnvelopeHashIntegrity?.integrity_status === "verified_match") {
    return "project_envelope_hash_verified_on_open";
  }
  if (projectEnvelopeHashIntegrity?.integrity_status === "mismatch_review_required") {
    return "project_envelope_hash_mismatch_review_required";
  }
  if (projectEnvelopeHashIntegrity?.integrity_status === "hash_recompute_unavailable") {
    return "project_envelope_hash_recompute_unavailable_review_required";
  }
  if ((projectSummary?.persisted_project_envelope_hash_count ?? 0) > 0) {
    return "project_envelope_hash_persisted_open_verification_not_run";
  }
  if (projectEnvelopeHash) return "project_envelope_hash_computed_not_persisted";
  return "project_envelope_hash_not_computed_no_save_this_session";
}

function buildRoundTripCategories(model: PreviewModel, modelHashStatus: string): RoundTripCategory[] {
  const rulePackRefs = ((model as unknown as { rule_pack_refs?: unknown[] }).rule_pack_refs ?? []) as unknown[];
  return [
    category("model_content", "ready_for_preview_round_trip", model.nodes.length + model.pipe_segments.length),
    category(
      "unit_metadata",
      unitMetadataPresent(model) ? "ready_for_preview_round_trip" : "finding_missing_unit_metadata",
      countUnitBearingRecords(model)
    ),
    category(
      "load_payloads",
      loadPayloadsHaveUnits(model) ? "ready_for_preview_round_trip" : "finding_missing_load_unit_metadata",
      model.load_cases.length
    ),
    category(
      "rule_pack_references",
      rulePackRefs.length > 0 ? "ready_for_preview_round_trip" : "not_present_in_invented_fixture",
      rulePackRefs.length
    ),
    category(
      "provenance_metadata",
      provenancePresent(model) ? "ready_for_preview_round_trip" : "finding_missing_provenance_metadata",
      countProvenanceRecords(model)
    ),
    category("reproducibility_metadata", modelHashStatus, 1)
  ];
}

function buildServiceOperations({
  projectSummary,
  projectOperation,
  validationStatus,
  versionCheckStatus
}: {
  projectSummary: LocalProjectSummary | null;
  projectOperation: string;
  validationStatus: string;
  versionCheckStatus: string;
}) {
  return [
    {
      operation: "create",
      operation_status: projectOperation === "create" ? "last_operation_completed" : "available",
      result_available: projectOperation === "create" || Boolean(projectSummary)
    },
    {
      operation: "open",
      operation_status: projectOperation === "open" ? "last_operation_completed" : "available",
      result_available: projectOperation === "open"
    },
    {
      operation: "save",
      operation_status: projectOperation === "save" ? "last_operation_completed" : "available",
      result_available: projectOperation === "save"
    },
    {
      operation: "validate",
      operation_status: `preflight_generated_${validationStatus}`,
      result_available: true
    },
    {
      operation: "version_check",
      operation_status: versionCheckStatus,
      result_available: true
    },
    {
      operation: "migrate",
      operation_status: projectSummary ? projectSummary.migration_status : "not_run_no_local_snapshot_this_session",
      result_available: Boolean(projectSummary)
    }
  ];
}

// DEC-019 evidence: in-document semver is the version authority; migration
// runs in memory on open and persists on save with a ledger record. Before a
// persistence operation runs this session, the current session document is
// evaluated locally so the surface is never a hardcoded TBD marker.
function modelDocumentMigrationEvidence(
  model: PreviewModel,
  status: ModelDocumentMigrationStatus | null,
  ledger: ModelMigrationLedgerRecord[]
) {
  return {
    decision_basis: "DEC-019_model_document_schema_migration_policy",
    version_authority: "in_document_schema_version_semver",
    evidence_source: status ? "persistence_operation_envelope" : "session_document_local_evaluation",
    status: status?.status ?? (model.schema_version === "0.1.0" ? "current" : "unsupported_schema_review_required"),
    source_schema_version: status?.source_schema_version ?? model.schema_version,
    target_schema_version: status?.target_schema_version ?? "0.1.0",
    migration_framework: status?.migration_framework ?? "application_service_separate_db_and_product_schema",
    persistence_state: status?.persistence_state ?? "no_persistence_operation_this_session",
    applied_migration_ids: status?.applied_migration_ids ?? [],
    ledger_record_count: ledger.length,
    ledger_records: ledger,
    destructive_rewrite: false,
    down_migration_performed: false
  };
}

function buildStoreMigrationEvidence({
  projectSummary,
  storageCapability
}: {
  projectSummary: LocalProjectSummary | null;
  storageCapability: LocalStorageCapability | null;
}) {
  const source = projectSummary ?? storageCapability;
  return {
    migration_framework: source?.migration_framework ?? "store_migration_evidence_not_loaded_this_session",
    migration_status: source?.migration_status ?? "store_migration_evidence_not_loaded_this_session",
    store_schema_version: source?.store_schema_version ?? 0,
    store_schema_target_version: source?.store_schema_target_version ?? 0,
    migrations_applied_on_open: source?.migrations_applied_on_open ?? [],
    evidence_source: projectSummary
      ? "local_project_summary"
      : storageCapability
        ? "storage_capability_probe"
        : "not_loaded_this_session",
    migration_scope: "local_store_schema_ddl_only_model_document_schema_tracked_separately_per_dec_019",
    destructive_migration_performed: false
  };
}

function category(categoryName: string, semanticEqualityStatus: string, checkedRefCount: number): RoundTripCategory {
  return {
    category: categoryName,
    semantic_equality_status: semanticEqualityStatus,
    checked_ref_count: checkedRefCount,
    silent_default_inserted: false,
    protected_content_expanded: false,
    private_payload_expanded: false
  };
}

function unitMetadataPresent(model: PreviewModel): boolean {
  return (
    Object.values(model.project.units).every((unit) => unit.length > 0) &&
    countUnitBearingRecords(model) > 0 &&
	    model.pipe_segments.every((segment) => Object.values(segment.section).every((quantity) => hasUnit(quantity))) &&
	    (model.materials ?? []).every((material) => hasUnit(material.elastic_modulus) && hasUnit(material.shear_modulus)) &&
	    model.supports.every((support) =>
	      supportUnitQuantities(support).every((quantity) => !quantity || hasUnit(quantity))
	    ) &&
	    model.components.every((component) =>
	      componentUnitQuantities(component).every((quantity) => !quantity || hasUnit(quantity))
	    )
  );
}

function loadPayloadsHaveUnits(model: PreviewModel): boolean {
  return model.load_cases.every((loadCase) =>
    (loadCase.primitive_loads ?? []).every((primitiveLoad) => hasUnit(primitiveLoad.magnitude))
  );
}

function provenancePresent(model: PreviewModel): boolean {
  return provenanceRecords(model).every(
    (record) => typeof record.provenance === "string" && record.provenance.length > 0
  );
}

function countUnitBearingRecords(model: PreviewModel): number {
  const pipeSectionQuantities = model.pipe_segments.flatMap((segment) => Object.values(segment.section));
  const materialQuantities = (model.materials ?? []).flatMap((material) => [
    material.elastic_modulus,
    material.shear_modulus,
    material.thermal_expansion_coefficient
  ]);
  const supportQuantities = model.supports.flatMap(supportUnitQuantities);
  const componentQuantities = model.components.flatMap(componentUnitQuantities);
  const loadQuantities = model.load_cases.flatMap((loadCase) =>
    (loadCase.primitive_loads ?? []).map((primitiveLoad) => primitiveLoad.magnitude)
  );
  return [
    ...pipeSectionQuantities,
    ...materialQuantities,
    ...supportQuantities,
    ...componentQuantities,
    ...loadQuantities
  ].filter(hasUnit).length;
}

function supportUnitQuantities(support: PreviewModel["supports"][number]): unknown[] {
  return [
    support.stiffness?.value,
    support.properties?.linear_stiffness,
    support.hanger?.stiffness?.value,
    support.hanger?.installed_load,
    support.hanger?.cold_load,
    support.hanger?.hot_load,
    support.hanger?.constant_load,
    support.hanger?.travel_range,
    support.hanger?.movement_limit
  ];
}

function componentUnitQuantities(component: PreviewModel["components"][number]): unknown[] {
  return [
    component.geometry?.bend_radius,
    component.geometry?.bend_angle,
    component.geometry?.branch_run_size,
    component.geometry?.branch_header_size,
    component.geometry?.branch_connection_angle,
    component.geometry?.branch_reinforcement_area,
    component.geometry?.rigid_body_length,
    component.geometry?.end_a_size,
    component.geometry?.end_b_size,
    component.geometry?.weight,
    component.geometry?.center_of_gravity,
    component.modifiers?.sif_user_value,
    component.modifiers?.branch_header_sif_user_value,
    component.modifiers?.branch_branch_sif_user_value,
    component.modifiers?.flexibility_factor_user_value,
    component.modifiers?.stiffness_scaling_user_value,
    component.modifiers?.linear_stiffness_user_value,
    component.modifiers?.rotational_stiffness_user_value
  ];
}

function countProvenanceRecords(model: PreviewModel): number {
  return provenanceRecords(model).length;
}

function provenanceRecords(model: PreviewModel): Array<{ provenance?: string }> {
  return [
    ...(model.materials ?? []),
    ...model.nodes,
    ...model.pipe_segments,
    ...model.supports,
    ...model.components,
    ...model.load_cases,
    ...(model.combinations ?? []),
    ...model.load_cases.flatMap((loadCase) => loadCase.primitive_loads ?? [])
  ];
}

function hasUnit(value: unknown): value is { unit: string } {
  return (
    typeof value === "object" &&
    value !== null &&
    "unit" in value &&
    typeof (value as { unit?: unknown }).unit === "string" &&
    (value as { unit: string }).unit.length > 0
  );
}

function sortedStringRecord(record: Record<string, string>): Record<string, string> {
  return Object.fromEntries(
    Object.entries(record)
      .filter(([, value]) => typeof value === "string" && value.length > 0)
      .sort(([left], [right]) => left.localeCompare(right))
  );
}

function formatUnitRecord(units: Record<string, string>): string {
  const entries = Object.entries(units).sort(([left], [right]) => left.localeCompare(right));
  if (entries.length === 0) return "none";
  return entries.map(([key, value]) => `${key}=${value}`).join(",");
}

function reference(objectType: string, ref: string): ObjectRef {
  return { object_type: objectType, ref };
}

function validationDiagnostics({
  storageCapability,
  projectSummary,
  versionCheckStatus,
  modelHash,
  modelHashIntegrity,
  projectEnvelopeHash,
  projectEnvelopeHashIntegrity
}: {
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  versionCheckStatus: string;
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null;
  projectEnvelopeHashIntegrity: ProjectEnvelopeHashIntegrityEvidence | null;
}) {
  const diagnostics = [
    diagnostic(
      "PROJECT-VALIDATION-PREFLIGHT-ONLY",
      "info",
      "Validation preflight is local technical-preview evidence and does not create professional acceptance."
    ),
    modelHashDiagnostic({ modelHash, modelHashIntegrity }),
    projectEnvelopeHashDiagnostic({
      projectEnvelopeHash,
      projectEnvelopeHashIntegrity
    }),
    storeMigrationDiagnostic({ projectSummary, storageCapability })
  ];
  if (!storageCapability) {
    diagnostics.push(
      diagnostic(
        "PROJECT-VALIDATION-STORAGE-CAPABILITY-PENDING",
        "warning",
        "Local storage capability has not loaded yet."
      )
    );
  }
  if (!projectSummary) {
    diagnostics.push(
      diagnostic(
        "PROJECT-VALIDATION-SNAPSHOT-NOT-WRITTEN",
        "warning",
        "No local project snapshot is available for persisted round-trip evidence."
      )
    );
  }
  if (versionCheckStatus !== "supported_current_schema") {
    diagnostics.push(
      diagnostic(
        "PROJECT-VALIDATION-UNSUPPORTED-SCHEMA",
        "blocking",
        "Project schema version is not supported by this technical-preview preflight."
      )
    );
  }
  return diagnostics;
}

function modelHashDiagnostic({
  modelHash,
  modelHashIntegrity
}: {
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
}) {
  if (!modelHash) {
    return diagnostic(
      "PROJECT-VALIDATION-MODEL-HASH-SERVICE-UNAVAILABLE",
      "warning",
      "Canonical model hash could not be computed in this runtime; reproducibility evidence is unavailable."
    );
  }
  if (modelHashIntegrity?.integrity_status === "mismatch_review_required") {
    return diagnostic(
      "PROJECT-VALIDATION-MODEL-HASH-MISMATCH",
      "warning",
      "Persisted model hash does not match the hash recomputed from the restored model; human review is required."
    );
  }
  return diagnostic(
    "PROJECT-VALIDATION-MODEL-HASH-REVIEW-ONLY",
    "info",
    "Canonical model hash is a local technical-preview review-reproducibility signal only, scoped to the model payload; it is not an acceptance, certification, sealing, authentication, or code-compliance record."
  );
}

function projectEnvelopeHashDiagnostic({
  projectEnvelopeHash,
  projectEnvelopeHashIntegrity
}: {
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null;
  projectEnvelopeHashIntegrity: ProjectEnvelopeHashIntegrityEvidence | null;
}) {
  if (projectEnvelopeHashIntegrity?.integrity_status === "mismatch_review_required") {
    return diagnostic(
      "PROJECT-VALIDATION-ENVELOPE-HASH-MISMATCH",
      "warning",
      "Persisted project-envelope hash does not match the hash recomputed from the restored envelope payload; human review is required."
    );
  }
  if (!projectEnvelopeHash && !projectEnvelopeHashIntegrity) {
    return diagnostic(
      "PROJECT-VALIDATION-ENVELOPE-HASH-NOT-COMPUTED",
      "info",
      "Project-envelope hash is computed at save time over the persisted envelope payload; no save has produced envelope-hash evidence this session."
    );
  }
  return diagnostic(
    "PROJECT-VALIDATION-ENVELOPE-HASH-REVIEW-ONLY",
    "info",
    "Project-envelope hash is a local technical-preview review-reproducibility signal over the persisted envelope payload (storage summary and hash carrier excluded); it is not an acceptance, certification, sealing, authentication, or code-compliance record."
  );
}

function storeMigrationDiagnostic({
  projectSummary,
  storageCapability
}: {
  projectSummary: LocalProjectSummary | null;
  storageCapability: LocalStorageCapability | null;
}) {
  const appliedOnOpen =
    projectSummary?.migrations_applied_on_open?.length ?? storageCapability?.migrations_applied_on_open?.length ?? 0;
  if (appliedOnOpen > 0) {
    return diagnostic(
      "PROJECT-VALIDATION-STORE-MIGRATED-ON-OPEN",
      "info",
      "Versioned store migrations were applied when the local project store was opened; this is local store maintenance evidence only, not a model document migration or acceptance record."
    );
  }
  return diagnostic(
    "PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY",
    "info",
    "Store migration evidence covers the local store schema ledger only; model document migrations remain TBD and no migration claim exceeds the local technical preview."
  );
}

function diagnostic(code: string, severity: "info" | "warning" | "blocking", message: string) {
  return {
    code,
    class: code.includes("SCHEMA") ? "SCHEMA_VALIDATION" : code.includes("HASH") ? "REPRODUCIBILITY" : "MIGRATION",
    severity,
    source: "apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx",
    message
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

function categoryStatus(categories: RoundTripCategory[], categoryName: string): string {
  return (
    categories.find((categoryItem) => categoryItem.category === categoryName)?.semantic_equality_status ?? "missing"
  );
}

function operationStatus(
  serviceOperations: Array<{ operation: string; operation_status: string }>,
  operation: string
): string {
  return (
    serviceOperations.find((operationItem) => operationItem.operation === operation)?.operation_status ?? "missing"
  );
}

function unique(values: string[]): string[] {
  return Array.from(new Set(values)).sort();
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value
    .replace(/[^a-z0-9]+/gi, "-")
    .replace(/^-+|-+$/g, "")
    .toLowerCase();
}
