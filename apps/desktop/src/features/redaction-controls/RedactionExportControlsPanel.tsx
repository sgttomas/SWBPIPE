// DEL-12-02 app-side binding (TP-E4-REDACTION-001): user-facing redaction and
// export-control workflow. The panel classifies a metadata-only export
// representation of the CURRENT user-entered model through the TypeScript
// mirror of `core/security/redaction/controls.py`, surfaces every warning and
// blocking finding BEFORE any export artifact is offered, withholds the
// download link entirely while any blocking finding exists, and records a
// canonical sha256 checksum for the redacted manifest it does offer.
//
// Private-by-default: the export context starts at `local_private`, and even
// there private values are refused until the user records explicit
// local-private intent. Boundary (F-PIP-2): warnings inform — nothing here
// certifies redaction sufficiency, release readiness, legal clearance, or
// professional acceptance.
import { useMemo, useState } from "react";
import { Download, EyeOff } from "lucide-react";
import type { PreviewModel } from "../../types";
import { usePackageHash } from "../../services/usePackageHash";
import {
  EXPORT_CONTEXTS,
  redactExportPayload,
  type RedactionExportContext,
  type RedactionFinding,
  type RedactionRunResult
} from "./redactionExportControls";

const CONTEXT_LABELS: Record<RedactionExportContext, string> = {
  local_private: "Local private (default)",
  public_report: "Public report",
  public_example: "Public / shared example",
  shared_model: "Shared model",
  downstream_tool: "Downstream tool"
};

export function RedactionExportControlsPanel({ model }: { model: PreviewModel }) {
  const [exportContext, setExportContext] = useState<RedactionExportContext>("local_private");
  const [explicitLocalPrivateIntent, setExplicitLocalPrivateIntent] = useState(false);

  const representation = useMemo(() => buildExportRepresentation(model), [model]);
  const intentApplies = exportContext === "local_private";
  const run = useMemo(
    () =>
      redactExportPayload(representation, {
        exportContext,
        explicitLocalPrivateIntent: intentApplies && explicitLocalPrivateIntent
      }),
    [representation, exportContext, intentApplies, explicitLocalPrivateIntent]
  );

  const packet = buildRedactionControlsPacket({
    model,
    exportContext,
    explicitLocalPrivateIntent: intentApplies && explicitLocalPrivateIntent,
    run
  });
  const packageHash = usePackageHash(
    `redaction-controls:${model.project.id}`,
    packet,
    "manifest_and_validation_report_package_hash_carrier_fields"
  );
  const exportPacket = {
    ...packet,
    checksum: packageHash ?? {
      hash_status: "pending_local_preview_hash" as const
    }
  };

  return (
    <section
      className="panel redaction-controls-panel"
      aria-label="Redaction and export controls"
      data-testid="redaction-controls-panel"
    >
      <div className="panel-title">
        <EyeOff size={16} aria-hidden="true" />
        Redaction & Export Controls
      </div>
      <div className="report-actions">
        <label className="report-line" data-testid="redaction-context-row">
          <span>Export context</span>
          <select
            aria-label="Export context"
            data-testid="redaction-context-select"
            onChange={(event) => setExportContext(event.target.value as RedactionExportContext)}
            value={exportContext}
          >
            {EXPORT_CONTEXTS.map((context) => (
              <option key={context} value={context}>
                {CONTEXT_LABELS[context]}
              </option>
            ))}
          </select>
        </label>
        <label className="report-line" data-testid="redaction-intent-row">
          <span>Explicit local-private intent</span>
          <input
            aria-label="Retain private values in this local private export"
            checked={explicitLocalPrivateIntent}
            data-testid="redaction-intent-toggle"
            disabled={!intentApplies}
            onChange={(event) => setExplicitLocalPrivateIntent(event.target.checked)}
            type="checkbox"
          />
        </label>
      </div>
      <div className="report-list" data-testid="redaction-controls-body">
        <ControlLine
          label="Posture"
          value={`context=${exportContext}; private_by_default=true; classification_basis=explicit_metadata_only; source_mutation=false; network=false`}
          testId="redaction-posture"
        />
        <ControlLine
          label="Decisions"
          value={`classified=${run.summary.decision_count}; included=${includedCount(run)}; redacted=${run.summary.redacted_count}; local_warning_retained=${warningOnlyCount(run)}; blocked=${blockedCount(run)}`}
          testId="redaction-decision-summary"
        />
        <ControlLine
          label="Warnings"
          value={`warning=${run.summary.warning_count}; blocking=${run.summary.blocking_count}`}
          testId="redaction-warning-summary"
        />
        {run.findings.length > 0 ? (
          <div className="operation-record-list" data-testid="redaction-findings">
            {run.findings.map((finding) => (
              <article className="operation-record" data-testid={`redaction-finding-${finding.code}`} key={finding.finding_id}>
                <strong>
                  {finding.severity}: {finding.code}
                </strong>
                <small>
                  {finding.path}; action={finding.action}; class={finding.class}
                </small>
                <p>{finding.message}</p>
              </article>
            ))}
          </div>
        ) : (
          <ControlLine
            label="Findings"
            value="none; every classified record carries explicit safe metadata"
            testId="redaction-findings-empty"
          />
        )}
        {run.blocked ? (
          <div className="report-line" data-testid="redaction-export-blocked" role="alert">
            <span>Export withheld</span>
            <strong>
              {run.summary.blocking_count} blocking finding(s); no export artifact is offered until the boundary or
              intent issue above is resolved
            </strong>
          </div>
        ) : (
          <div className="report-line" data-testid="redaction-export-ready">
            <span>Redacted export</span>
            <a
              className="report-export-link"
              data-testid="redaction-export-link"
              download={`openpipestress-preview-redacted-export-${safeFileToken(model.project.id)}.json`}
              href={jsonDataHref(exportPacket)}
            >
              <Download size={14} aria-hidden="true" />
              Redacted manifest JSON
            </a>
          </div>
        )}
        <ControlLine
          label="Checksum"
          value={
            packageHash
              ? `${packageHash.value} (${packageHash.canonicalization})`
              : "computing local preview sha256"
          }
          testId="redaction-checksum"
        />
      </div>
      <small className="report-note">
        Redaction warnings inform the local export decision only; they do not certify redaction sufficiency. No
        protected standards content; code-specific data is user-supplied. Classification uses explicit user-entered
        metadata only; the source model is never mutated and nothing is transmitted.
      </small>
    </section>
  );
}

function ControlLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function includedCount(run: RedactionRunResult): number {
  return run.decisions.filter((decision) => decision.action === "include").length;
}

function warningOnlyCount(run: RedactionRunResult): number {
  return run.decisions.filter((decision) => decision.action === "warning_only").length;
}

function blockedCount(run: RedactionRunResult): number {
  return run.decisions.filter((decision) => decision.action === "block_export").length;
}

type ExportItem = Record<string, unknown>;

// Explicit-metadata mapping for the preview model's user-entered records.
// Provenance is the user-entered basis: only the explicit invented-example
// markers (the repo convention is `invented_example` optionally suffixed with
// a no-catalog/no-code-table qualifier, e.g.
// `invented_example_no_material_standard`) map to the safe invented classes;
// any other recorded provenance is treated as private user data
// (private-by-default per PRD §17.3/§18.3); absent provenance stays unknown
// so the contract warns instead of leaking.
function privacyMetadataFor(
  kind: "project" | "material" | "component" | "rule_pack",
  provenance: string | undefined
): { privacy_classification: string; redistribution_status: string; review_status: string } {
  if (
    typeof provenance === "string" &&
    (provenance === "invented_public_example" || provenance.startsWith("invented_example"))
  ) {
    return {
      privacy_classification: "invented_public_example",
      redistribution_status: "invented_non_engineering_example",
      review_status: "accepted"
    };
  }
  if (typeof provenance === "string" && provenance.length > 0) {
    return {
      privacy_classification: `private_${kind}_data`,
      redistribution_status: "private_only",
      review_status: "pending"
    };
  }
  return { privacy_classification: "unknown", redistribution_status: "unknown", review_status: "unknown" };
}

// Metadata-only export representation of the current user-entered model.
// Values are the user's own entries (never invented here); classification is
// carried as explicit metadata beside each value so the redaction contract
// decides from metadata alone.
export function buildExportRepresentation(model: PreviewModel): Record<string, unknown> {
  const project: Record<string, ExportItem> = {
    name: {
      field_id: "project.name",
      field_class: "project",
      // Project identity is user-entered with no provenance slot: private by
      // default, never unknown.
      privacy_classification: "private_project_data",
      redistribution_status: "private_only",
      review_status: "pending",
      value: model.project.name
    },
    description: {
      field_id: "project.description",
      field_class: "project",
      privacy_classification: "private_project_data",
      redistribution_status: "private_only",
      review_status: "pending",
      value: model.project.description
    }
  };

  const materials: ExportItem[] = (model.materials ?? []).map((material) => ({
    field_id: material.id,
    field_class: "material",
    ...privacyMetadataFor("material", material.provenance),
    value: `${material.label}; E=${material.elastic_modulus.value} ${material.elastic_modulus.unit}; G=${material.shear_modulus.value} ${material.shear_modulus.unit}`
  }));

  const components: ExportItem[] = model.components.map((component) => ({
    field_id: component.id,
    field_class: "component",
    ...privacyMetadataFor("component", component.provenance),
    value: `${component.label}; kind=${component.kind}; node=${component.node}`
  }));

  return {
    project,
    materials,
    components
  };
}

function buildRedactionControlsPacket({
  model,
  exportContext,
  explicitLocalPrivateIntent,
  run
}: {
  model: PreviewModel;
  exportContext: RedactionExportContext;
  explicitLocalPrivateIntent: boolean;
  run: RedactionRunResult;
}) {
  const privateRetained = run.decisions.some((decision) => decision.action === "warning_only");
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.redaction_export_controls_review",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-12-02",
    package_id: "PKG-12",
    scope_items: ["SOW-040"],
    objective: "OBJ-010",
    project_ref: model.project.id,
    source_basis: {
      contract_module: "core/security/redaction/controls.py",
      contract_schema: "schemas/redaction_export_controls.schema.yaml",
      contract_tests: "tests/security/test_redaction_export_controls.py",
      parity_fixture: "fixtures/redaction_export_controls/cases.json",
      app_mirror: "apps/desktop/src/features/redaction-controls/redactionExportControls.ts"
    },
    control_profile: {
      profile_id: "app-preview-default",
      local_first: true,
      source_mutation_allowed: false,
      cloud_transmission_allowed: false,
      unknown_metadata_behavior: "redact_value",
      missing_provenance_behavior: "redact_value",
      private_default_behavior: "redact_value",
      local_private_requires_explicit_intent: true,
      professional_claims_allowed: false,
      classification_basis: "explicit_metadata_only"
    },
    field_policies: [
      fieldPolicy("project"),
      fieldPolicy("material"),
      fieldPolicy("component")
    ],
    export_run: {
      run_id: `redaction-run:${model.project.id}:${exportContext}`,
      export_context: exportContext,
      explicit_local_private_intent: explicitLocalPrivateIntent,
      input_summary: {
        representation_only: true,
        source_payload_mutated: false,
        contains_real_private_data: false,
        contains_real_secrets: false,
        contains_protected_standards_content: false
      },
      decisions: run.decisions,
      findings: run.findings,
      summary: run.summary
    },
    redacted_payload: run.blocked ? null : run.payload,
    boundary: {
      private_payload_included: privateRetained,
      protected_content_included: false,
      release_or_professional_claim: false,
      warning_informational_only: true
    }
  };
}

function fieldPolicy(fieldClass: "project" | "material" | "component") {
  return {
    policy_id: `app-preview-${fieldClass}`,
    field_class: fieldClass,
    default_action: "redact_value",
    public_actions: ["redact_value", "block_export"],
    local_private_action: "warning_only",
    required_metadata: ["privacy_classification", "redistribution_status", "review_status"],
    safe_metadata_fields: ["id", "name", "version", "checksum", "schema_version"]
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-");
}
