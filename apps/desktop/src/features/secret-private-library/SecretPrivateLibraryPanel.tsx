import { Download, KeyRound } from "lucide-react";
import type { LocalStorageCapability, PreviewModel } from "../../types";

type SecretPrivateReference = {
  reference_id: string;
  record_kind:
    | "private_material_library"
    | "private_component_library"
    | "private_rule_pack"
    | "private_path_reference"
    | "secret_field_reference";
  label: string;
  storage_locality: string;
  privacy_classification: string;
  redistribution_status: "private_only" | "unknown" | "TBD";
  review_status: "pending" | "TBD";
  source_state: string;
  source_note: string;
  checksum: string | null;
  checksum_status: string;
  value_descriptor: string;
  unit_metadata_status: string;
  contains_payload: false;
  secret_material_present: false;
  concrete_path_present: false;
  cloud_or_network_reference: false;
  external_secret_manager_reference: false;
  direct_sql_access: false;
  storage_bypass_requested: false;
  unresolved_tbd: string[];
};

type GuardDecision = {
  decision_id: string;
  reference_id: string;
  record_kind: SecretPrivateReference["record_kind"];
  release_context: string;
  action: "block_release" | "include_metadata_only";
  reason_code: string;
  metadata_only: true;
};

export function SecretPrivateLibraryPanel({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const packet = buildSecretPrivateLibraryPacket({ model, storageCapability });

  return (
    <section
      className="panel secret-private-library-panel"
      aria-label="Secret and private-library review"
      data-testid="secret-private-library-panel"
    >
      <div className="panel-title">
        <KeyRound size={16} aria-hidden="true" />
        Secret & Private Libraries
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="secret-private-library-export-link"
          download={`openpipestress-preview-secret-private-library-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Secret/private JSON
        </a>
        <span data-testid="secret-private-library-summary">
          records={packet.summary.reference_count}; metadata_only={String(packet.summary.metadata_only)};
          public_fixture_blocked={packet.summary.public_fixture_block_count}; secrets=
          {String(packet.summary.secret_material_present)}; private_payload={String(packet.private_payload_included)}
        </span>
      </div>
      <div className="report-list" data-testid="secret-private-library-body">
        <SecretLine
          label="Registry"
          value={`libraries=${packet.summary.private_library_count}; paths=${packet.summary.private_path_count}; credentials=${packet.summary.credential_reference_count}; payload=${String(packet.summary.payload_present)}`}
          testId="secret-private-library-registry"
        />
        <SecretLine
          label="Guards"
          value={`public_report=${packet.guard_contexts.public_report.summary.action}; public_fixture=${packet.guard_contexts.public_fixture.summary.action}; local_private=${packet.guard_contexts.local_private.summary.action}`}
          testId="secret-private-library-guards"
        />
        <SecretLine
          label="Secrets"
          value={`provider=${packet.runtime_deferrals.exact_secret_provider}; values=${String(packet.summary.secret_material_present)}; external_manager=${String(packet.summary.external_secret_manager_used)}; encryption=${packet.runtime_deferrals.encrypted_storage_default}`}
          testId="secret-private-library-secrets"
        />
        <SecretLine
          label="Unit policy"
          value={`unit_refs=${packet.unit_policy.private_unit_reference_count}; required=${String(packet.unit_policy.explicit_unit_metadata_required)}; payload=${String(packet.unit_policy.unit_payload_included)}; conversion=${String(packet.unit_policy.conversion_performed)}`}
          testId="secret-private-library-unit-policy"
        />
        <SecretLine
          label="No-bypass"
          value={`direct_sql=${String(packet.summary.direct_sql_access)}; storage_bypass=${String(packet.summary.storage_bypass_requested)}; network=${String(packet.summary.cloud_or_network_reference)}; telemetry=${String(packet.no_bypass_controls.telemetry_allowed)}`}
          testId="secret-private-library-no-bypass"
        />
        <SecretLine
          label="Boundary"
          value={`private_payload=${String(packet.private_payload_included)}; protected=${String(packet.protected_content_included)}; security_certification=${String(packet.security_certification_claim)}; professional=${String(packet.release_or_professional_claim)}`}
          testId="secret-private-library-boundary"
        />
      </div>
      <small className="report-note">
        Secret and private-library review records metadata-only references. It does not store credential values, read
        private libraries, choose a secret provider, or finalize encryption/key management. No protected standards
        content; code-specific data is user-supplied.
      </small>
    </section>
  );
}

function SecretLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildSecretPrivateLibraryPacket({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const references = secretPrivateReferences(model);
  const publicReportDecisions = guardDecisions(references, "public_report", "include_metadata_only");
  const publicFixtureDecisions = guardDecisions(references, "public_fixture", "block_release");
  const localPrivateDecisions = guardDecisions(references, "local_private", "include_metadata_only");
  const unitPolicy = secretPrivateUnitPolicy(references);

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.secret_private_library_boundary_review",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-12-04",
    package_id: "PKG-12",
    scope_items: ["SOW-040", "SOW-029"],
    objective: "OBJ-010",
    project_ref: model.project.id,
    source_basis: {
      policy_doc: "docs/security/secret_private_library_handling.md",
      guard_module: "core/security/secret_private_library/",
      guard_tests: "tests/security/test_secret_private_library_handling.py",
      app_surfaces: [
        "apps/desktop/src/features/secret-private-library/SecretPrivateLibraryPanel.tsx",
        "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
        "apps/desktop/src/features/report/ReportPanel.tsx",
        "apps/desktop/src/features/report-lint/ReportLintPanel.tsx"
      ]
    },
    summary: {
      reference_count: references.length,
      private_library_count: references.filter(
        (item) => item.record_kind.includes("library") || item.record_kind === "private_rule_pack"
      ).length,
      private_path_count: references.filter((item) => item.record_kind === "private_path_reference").length,
      credential_reference_count: references.filter((item) => item.record_kind === "secret_field_reference").length,
      metadata_only: true,
      payload_present: references.some((item) => item.contains_payload),
      secret_material_present: references.some((item) => item.secret_material_present),
      concrete_path_present: references.some((item) => item.concrete_path_present),
      cloud_or_network_reference: references.some((item) => item.cloud_or_network_reference),
      external_secret_manager_used: references.some((item) => item.external_secret_manager_reference),
      direct_sql_access: references.some((item) => item.direct_sql_access),
      storage_bypass_requested: references.some((item) => item.storage_bypass_requested),
      public_fixture_block_count: publicFixtureDecisions.filter((item) => item.action === "block_release").length,
      public_report_metadata_only_count: publicReportDecisions.filter((item) => item.action === "include_metadata_only")
        .length,
      local_private_metadata_only_count: localPrivateDecisions.filter((item) => item.action === "include_metadata_only")
        .length,
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false
    },
    unit_policy: unitPolicy,
    registry_records: references,
    guard_contexts: {
      public_report: guardContext("public_report", publicReportDecisions),
      public_fixture: guardContext("public_fixture", publicFixtureDecisions),
      local_private: guardContext("local_private", localPrivateDecisions)
    },
    no_bypass_controls: {
      adapters_must_preserve_redaction: true,
      plugins_denied_by_default: true,
      direct_sql_allowed: false,
      raw_sqlite_handle_allowed: false,
      storage_bypass_allowed: false,
      network_required: false,
      telemetry_allowed: false,
      private_payload_copy_allowed: false,
      protected_content_copy_allowed: false,
      repository_default_private_write: false
    },
    runtime_deferrals: {
      exact_secret_provider: "TBD",
      encrypted_storage_default: "TBD",
      storage_roots: "TBD",
      permission_grant_persistence: "TBD",
      physical_project_package_container: "TBD",
      public_api_transport: "TBD",
      external_secret_manager_behavior: "TBD",
      cloud_or_network_behavior: "not_authorized_in_preview"
    },
    storage_capability_ref: storageCapability
      ? {
          engine: storageCapability.engine,
          network_required: storageCapability.network_required,
          daemon_required: storageCapability.daemon_required,
          telemetry_enabled: storageCapability.telemetry_enabled,
          path_policy: storageCapability.path_policy
        }
      : null,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    security_certification_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function secretPrivateReferences(model: PreviewModel): SecretPrivateReference[] {
  return [
    reference({
      referenceId: "spl:private-material-library:invented-preview",
      recordKind: "private_material_library",
      label: "Private material library reference",
      storageLocality: "USER_PRIVATE_LIBRARY_ROOT",
      privacyClassification: "private_material_data",
      redistributionStatus: "private_only",
      reviewStatus: "pending",
      sourceState: "private_user_supplied",
      sourceNote: "metadata-only invented preview reference; private material values withheld",
      checksum: `sha256:metadata-only-${safeFileToken(model.project.id)}`,
      checksumStatus: "metadata_reference_only",
      valueDescriptor: "metadata-only private_material_library reference; version=TBD",
      unitMetadataStatus: "private_unit_bearing_values_withheld_explicit_unit_metadata_required_at_use",
      unresolvedTbd: ["version", "storage_root", "permission_grant_persistence"]
    }),
    reference({
      referenceId: "spl:private-rule-pack:invented-preview",
      recordKind: "private_rule_pack",
      label: "Private rule-pack reference",
      storageLocality: "USER_PRIVATE_RULE_PACK_ROOT",
      privacyClassification: "private_rule_pack_data",
      redistributionStatus: "private_only",
      reviewStatus: "pending",
      sourceState: "private_user_supplied",
      sourceNote: "metadata-only invented preview reference; formulas and private values withheld",
      checksum: null,
      checksumStatus: "TBD",
      valueDescriptor: "metadata-only private_rule_pack reference; version=TBD",
      unitMetadataStatus: "private_rule_inputs_withheld_explicit_unit_metadata_required_at_use",
      unresolvedTbd: ["checksum", "version", "rule_input_values"]
    }),
    reference({
      referenceId: "spl:private-path:library-root",
      recordKind: "private_path_reference",
      label: "Private library root reference",
      storageLocality: "USER_PRIVATE_LIBRARY_ROOT",
      privacyClassification: "path_data",
      redistributionStatus: "private_only",
      reviewStatus: "pending",
      sourceState: "private_user_supplied",
      sourceNote: "symbolic path class only",
      checksum: null,
      checksumStatus: "not_applicable",
      valueDescriptor: "symbolic path class only: USER_PRIVATE_LIBRARY_ROOT",
      unitMetadataStatus: "not_unit_bearing_path_metadata_only",
      unresolvedTbd: ["exact_storage_root"]
    }),
    reference({
      referenceId: "spl:credential-placeholder:import-access",
      recordKind: "secret_field_reference",
      label: "Import credential placeholder",
      storageLocality: "USER_SECRET_REFERENCE",
      privacyClassification: "secret_like_data",
      redistributionStatus: "private_only",
      reviewStatus: "pending",
      sourceState: "credential_reference",
      sourceNote: "placeholder descriptor only",
      checksum: null,
      checksumStatus: "not_applicable",
      valueDescriptor: "secret descriptor=local import credential reference; placeholder_key_id=TBD",
      unitMetadataStatus: "not_unit_bearing_secret_metadata_only",
      unresolvedTbd: ["exact_secret_provider", "key_lifecycle", "permission_grant_persistence"]
    })
  ];
}

function reference({
  referenceId,
  recordKind,
  label,
  storageLocality,
  privacyClassification,
  redistributionStatus,
  reviewStatus,
  sourceState,
  sourceNote,
  checksum,
  checksumStatus,
  valueDescriptor,
  unitMetadataStatus,
  unresolvedTbd
}: {
  referenceId: string;
  recordKind: SecretPrivateReference["record_kind"];
  label: string;
  storageLocality: string;
  privacyClassification: string;
  redistributionStatus: SecretPrivateReference["redistribution_status"];
  reviewStatus: SecretPrivateReference["review_status"];
  sourceState: string;
  sourceNote: string;
  checksum: string | null;
  checksumStatus: string;
  valueDescriptor: string;
  unitMetadataStatus: string;
  unresolvedTbd: string[];
}): SecretPrivateReference {
  return {
    reference_id: referenceId,
    record_kind: recordKind,
    label,
    storage_locality: storageLocality,
    privacy_classification: privacyClassification,
    redistribution_status: redistributionStatus,
    review_status: reviewStatus,
    source_state: sourceState,
    source_note: sourceNote,
    checksum,
    checksum_status: checksumStatus,
    value_descriptor: valueDescriptor,
    unit_metadata_status: unitMetadataStatus,
    contains_payload: false,
    secret_material_present: false,
    concrete_path_present: false,
    cloud_or_network_reference: false,
    external_secret_manager_reference: false,
    direct_sql_access: false,
    storage_bypass_requested: false,
    unresolved_tbd: unresolvedTbd
  };
}

function secretPrivateUnitPolicy(references: SecretPrivateReference[]) {
  const privateUnitReferences = references.filter(
    (item) => item.record_kind === "private_material_library" || item.record_kind === "private_component_library" || item.record_kind === "private_rule_pack"
  );
  return {
    evidence_id: "unit-policy:secret-private-library-metadata-only-preview",
    decision_basis_refs: ["DEC-018", "DEL-02-02", "DEL-12-04"],
    policy: "private_library_unit_bearing_values_are_metadata_only_until_user_import_or_rule_binding",
    private_unit_reference_count: privateUnitReferences.length,
    explicit_unit_metadata_required: true,
    unit_payload_included: false,
    conversion_performed: false,
    repository_default_private_write: false,
    unit_metadata_statuses: privateUnitReferences.map((item) => ({
      reference_id: item.reference_id,
      record_kind: item.record_kind,
      unit_metadata_status: item.unit_metadata_status
    }))
  };
}

function guardDecisions(
  references: SecretPrivateReference[],
  releaseContext: string,
  action: "block_release" | "include_metadata_only"
): GuardDecision[] {
  return references.map((item, index) => ({
    decision_id: `SPL-GRD-${String(index + 1).padStart(4, "0")}-${safeFileToken(releaseContext)}`,
    reference_id: item.reference_id,
    record_kind: item.record_kind,
    release_context: releaseContext,
    action,
    reason_code: guardReason(releaseContext, action),
    metadata_only: true
  }));
}

function guardReason(releaseContext: string, action: GuardDecision["action"]) {
  if (releaseContext === "public_fixture") return "PRIVATE_REFERENCE_PUBLIC_FIXTURE_BLOCKED";
  if (releaseContext === "local_private") return "PRIVATE_LOCAL_METADATA_ALLOWED";
  return action === "include_metadata_only" ? "PRIVATE_REFERENCE_METADATA_ONLY" : "UNKNOWN_REDIS_PRIVATE_DATA_BLOCKED";
}

function guardContext(releaseContext: string, decisions: GuardDecision[]) {
  const blockedCount = decisions.filter((item) => item.action === "block_release").length;
  return {
    release_context: releaseContext,
    explicit_local_private_intent: releaseContext === "local_private",
    decisions,
    summary: {
      action: blockedCount > 0 ? "blocked_until_review_or_metadata_only" : "metadata_only_allowed",
      reference_count: decisions.length,
      blocked_count: blockedCount,
      metadata_only_count: decisions.filter((item) => item.metadata_only).length,
      concrete_paths_emitted: false,
      cloud_transmission_attempted: false,
      external_secret_manager_used: false,
      encryption_or_key_management_finalized: false,
      professional_claims_made: false
    }
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_security_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
