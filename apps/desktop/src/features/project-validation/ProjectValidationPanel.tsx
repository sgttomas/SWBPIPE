import { ClipboardCheck, Download } from "lucide-react";
import type {
  AgentProposal,
  EditorOperationIntent,
  LocalProjectSummary,
  LocalStorageCapability,
  ModelHashEvidence,
  ModelHashIntegrityEvidence,
  PreviewModel
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
  modelHashIntegrity
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
}) {
  const packet = buildProjectValidationPacket({
    model,
    storageCapability,
    projectSummary,
    projectOperation,
    editorIntents,
    proposal,
    modelHash,
    modelHashIntegrity
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
          validation={packet.summary.validation_status}; version={packet.summary.version_check_status};
          migration={packet.summary.migration_status}; round_trip={packet.summary.round_trip_status}
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
          label="Model hash evidence"
          value={`model_hash=${packet.summary.model_hash_status}; persisted_model_hashes=${
            packet.summary.persisted_model_hash_count
          }; persisted_model_hash_ref=${packet.summary.persisted_model_hash_ref}; integrity=${
            packet.summary.model_hash_integrity_status
          }`}
          testId="project-validation-model-hash"
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
          label="Persistence operations"
          value={`validate=${operationStatus(packet.service_operations, "validate")}; version_check=${operationStatus(
            packet.service_operations,
            "version_check"
          )}; migrate=${operationStatus(packet.service_operations, "migrate")}; pending operations=${
            packet.summary.pending_operation_count
          }; persisted editor intents=${packet.summary.persisted_editor_intent_count}; proposals=${
            packet.summary.proposal_operation_count
          }; persisted proposals=${
            packet.summary.persisted_proposal_count
          }; persisted review targets=${
            packet.summary.persisted_selected_review_target_count
          }; persisted review target ref=${
            packet.summary.persisted_selected_review_target_ref
          }; persisted mechanics results=${
            packet.summary.persisted_mechanics_result_count
          }; persisted analysis runs=${
            packet.summary.persisted_analysis_run_count
          }; persisted analysis run ref=${
            packet.summary.persisted_analysis_run_ref
          }`}
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
  modelHashIntegrity
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  projectOperation: string;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
}) {
  const modelHashStatus = modelHashEvidenceStatus({ modelHash, projectSummary, modelHashIntegrity });
  const categories = buildRoundTripCategories(model, modelHashStatus);
  const migrationStatus = projectSummary?.migration_status ?? "not_persisted_this_session";
  const storeMigration = buildStoreMigrationEvidence({ projectSummary, storageCapability });
  const versionCheckStatus = model.schema_version === "0.1.0" ? "supported_current_schema" : "unsupported_schema_review_required";
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
    deliverable_refs: ["DEL-02-05", "DEL-12-01"],
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
      model_hash_status: modelHashStatus,
      persisted_model_hash_count: projectSummary?.persisted_model_hash_count ?? 0,
      persisted_model_hash_ref: projectSummary?.persisted_model_hash_ref ?? "not_persisted",
      model_hash_integrity_status: modelHashIntegrity?.integrity_status ?? "open_verification_not_run_this_session",
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
      project_envelope_hash_status: "model_payload_scope_only_full_project_envelope_hash_tbd",
      physical_container_status: projectSummary ? projectSummary.storage_mode : "not_persisted_this_session",
      store_migration_framework_status: storeMigration.migration_framework,
      model_document_migration_status: storeMigration.model_document_migration_status
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
    service_operations: buildServiceOperations({ projectSummary, projectOperation, validationStatus, versionCheckStatus }),
    store_migration: storeMigration,
    storage_capability: storageCapability,
    project_summary: projectSummary,
    model_hash: modelHash,
    model_hash_integrity: modelHashIntegrity,
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
    diagnostics: validationDiagnostics({ storageCapability, projectSummary, versionCheckStatus, modelHash, modelHashIntegrity }),
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary()
  };
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
      operation_status: projectSummary
        ? projectSummary.migration_status
        : "not_run_no_local_snapshot_this_session",
      result_available: Boolean(projectSummary)
    }
  ];
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
    migration_scope: "local_store_schema_only_not_model_document_schema",
    model_document_migration_status: "model_document_migrations_not_defined_tbd",
    destructive_migration_performed: false
  };
}

function category(
  categoryName: string,
  semanticEqualityStatus: string,
  checkedRefCount: number
): RoundTripCategory {
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
    (model.materials ?? []).every((material) => hasUnit(material.elastic_modulus) && hasUnit(material.shear_modulus))
  );
}

function loadPayloadsHaveUnits(model: PreviewModel): boolean {
  return model.load_cases.every((loadCase) =>
    (loadCase.primitive_loads ?? []).every((primitiveLoad) => hasUnit(primitiveLoad.magnitude))
  );
}

function provenancePresent(model: PreviewModel): boolean {
  return provenanceRecords(model).every((record) => typeof record.provenance === "string" && record.provenance.length > 0);
}

function countUnitBearingRecords(model: PreviewModel): number {
  const pipeSectionQuantities = model.pipe_segments.flatMap((segment) => Object.values(segment.section));
  const materialQuantities = (model.materials ?? []).flatMap((material) => [
    material.elastic_modulus,
    material.shear_modulus,
    material.thermal_expansion_coefficient
  ]);
  const loadQuantities = model.load_cases.flatMap((loadCase) =>
    (loadCase.primitive_loads ?? []).map((primitiveLoad) => primitiveLoad.magnitude)
  );
  return [...pipeSectionQuantities, ...materialQuantities, ...loadQuantities].filter(hasUnit).length;
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

function validationDiagnostics({
  storageCapability,
  projectSummary,
  versionCheckStatus,
  modelHash,
  modelHashIntegrity
}: {
  storageCapability: LocalStorageCapability | null;
  projectSummary: LocalProjectSummary | null;
  versionCheckStatus: string;
  modelHash: ModelHashEvidence | null;
  modelHashIntegrity: ModelHashIntegrityEvidence | null;
}) {
  const diagnostics = [
    diagnostic(
      "PROJECT-VALIDATION-PREFLIGHT-ONLY",
      "info",
      "Validation preflight is local technical-preview evidence and does not create professional acceptance."
    ),
    modelHashDiagnostic({ modelHash, modelHashIntegrity }),
    storeMigrationDiagnostic({ projectSummary, storageCapability })
  ];
  if (!storageCapability) {
    diagnostics.push(diagnostic("PROJECT-VALIDATION-STORAGE-CAPABILITY-PENDING", "warning", "Local storage capability has not loaded yet."));
  }
  if (!projectSummary) {
    diagnostics.push(diagnostic("PROJECT-VALIDATION-SNAPSHOT-NOT-WRITTEN", "warning", "No local project snapshot is available for persisted round-trip evidence."));
  }
  if (versionCheckStatus !== "supported_current_schema") {
    diagnostics.push(diagnostic("PROJECT-VALIDATION-UNSUPPORTED-SCHEMA", "blocking", "Project schema version is not supported by this technical-preview preflight."));
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
  return categories.find((categoryItem) => categoryItem.category === categoryName)?.semantic_equality_status ?? "missing";
}

function operationStatus(
  serviceOperations: Array<{ operation: string; operation_status: string }>,
  operation: string
): string {
  return serviceOperations.find((operationItem) => operationItem.operation === operation)?.operation_status ?? "missing";
}

function unique(values: string[]): string[] {
  return Array.from(new Set(values)).sort();
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
