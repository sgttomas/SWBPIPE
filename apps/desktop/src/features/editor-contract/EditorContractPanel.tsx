import { Download, SlidersHorizontal } from "lucide-react";
import type { EditorOperationIntent, PreviewModel } from "../../types";

type EditorKind = "material" | "component" | "rule_pack_reference";
type EditorValidationState =
  | "ready_for_service_validation"
  | "blocked_by_private_rule_inputs"
  | "blocked_by_user_supplied_modifier";

type EditorField = {
  field_id: string;
  label: string;
  value: string | number;
  unit?: string;
  dimension?: string;
  editable: boolean;
  source_ref: EditorReference;
  validation_state: "ready_for_service_validation" | "unresolved_TBD";
};

type EditorReference = {
  ref_type: string;
  ref_id: string;
};

type EditorDiagnostic = {
  code: string;
  class: "RULE_PACK_WARNING" | "PRIVATE_LIBRARY_WARNING" | "PROVENANCE_WARNING";
  severity: "info" | "warning";
  affected_ref: EditorReference;
  message: string;
  remediation: string;
};

export function EditorContractPanel({
  editorIntents,
  model
}: {
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
}) {
  const packet = buildEditorContractPacket({ editorIntents, model });

  return (
    <section className="panel editor-contract-panel" aria-label="Editor contract review" data-testid="editor-contract-panel">
      <div className="panel-title">
        <SlidersHorizontal size={16} aria-hidden="true" />
        Editor Contract
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="editor-contract-export-link"
          download={`openpipestress-preview-editor-contract-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Editor JSON
        </a>
        <span data-testid="editor-contract-summary">
          available; editors={packet.summary.editor_count}; surfaces={packet.summary.surface_count}; diagnostics=
          {packet.summary.diagnostic_count}; private_payload={String(packet.private_payload_included)}
        </span>
      </div>
      <div className="report-list" data-testid="editor-contract-body">
        <EditorLine
          label="Surface coverage"
          value={packet.surface_inventory.map((item) => `${item.surface}:${item.status}`).join("; ")}
          testId="editor-contract-coverage"
        />
        <EditorLine
          label="Validation"
          value={`ready=${packet.summary.ready_editor_count}; blocked=${packet.summary.blocked_editor_count}; queued=${packet.summary.queued_intent_count}`}
          testId="editor-contract-validation"
        />
        <EditorLine
          label="Unit contract"
          value={`contract=${packet.unit_contract.contract_ref}; schema=${packet.unit_contract.schema_ref}; policy=${packet.unit_contract.unit_policy}; missing=${packet.unit_contract.missing_unit_behavior}`}
          testId="editor-contract-unit-contract"
        />
        <EditorLine
          label="Rule pack"
          value={`${packet.rule_pack_reference.rule_pack_id}; checksum=${packet.rule_pack_reference.checksum_status}; required_inputs=${packet.rule_pack_reference.required_input_status}`}
          testId="editor-contract-rule-pack"
        />
        <EditorLine
          label="Mutation boundary"
          value={`${packet.command_policy}; direct_model_mutation_allowed=${String(packet.source_of_truth_boundary.direct_model_mutation_allowed)}; accepted_state_mutated=${String(packet.source_of_truth_boundary.accepted_model_state_mutated)}`}
          testId="editor-contract-mutation-boundary"
        />
        <EditorLine
          label="Professional boundary"
          value={`human_review=${String(packet.professional_boundary.human_review_required)}; compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}; approval=${String(packet.professional_boundary.software_makes_approval_claim)}`}
          testId="editor-contract-professional-boundary"
        />
      </div>
      <small className="report-note">
        Editor records are review-only command intent context. Private rule packs, private libraries, protected source
        content, durable mutation, approval, authentication, certification, sealing, and code-compliance claims are not
        bundled or generated.
      </small>
    </section>
  );
}

function EditorLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildEditorContractPacket({
  editorIntents,
  model
}: {
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
}) {
  const editors = [
    ...materialEditors(model),
    ...componentEditors(model),
    rulePackReferenceEditor(model)
  ].sort((left, right) => left.editor_id.localeCompare(right.editor_id));
  const diagnostics = editorDiagnostics(model);
  const blockedEditorCount = editors.filter((item) => item.validation_state !== "ready_for_service_validation").length;
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.editor_contract_review",
    deliverable_id: "DEL-07-03",
    package_id: "PKG-07",
    scope_item: "SOW-021",
    objectives: ["OBJ-006"],
    editor_set_id: `editor-set:${safeRefToken(model.project.id)}:desktop-preview`,
    project_ref: model.project.id,
    source_model_ref: model.project.id,
    editors,
    surface_inventory: editorSurfaceInventory(model),
    rule_pack_reference: {
      rule_pack_id: "rule-pack:user-supplied:not-loaded",
      lifecycle_state: "referenced_placeholder",
      checksum_status: "TBD_private_rule_pack_not_loaded",
      required_input_status: model.analysis_status.rule_check,
      private_payload_redacted: true,
      protected_content_redacted: true
    },
    queued_operation_intent_refs: editorIntents.map((intent) => intent.operation_id),
    diagnostics,
    summary: {
      editor_count: editors.length,
      field_count: editors.reduce((count, item) => count + item.fields.length, 0),
      surface_count: editorSurfaceInventory(model).length,
      ready_editor_count: editors.length - blockedEditorCount,
      blocked_editor_count: blockedEditorCount,
      queued_intent_count: editorIntents.length,
      diagnostic_count: diagnostics.length,
      private_payload_copied: false,
      durable_project_mutated: false,
      protected_content_included: false,
      compliance_claim_made: false
    },
    command_policy: "bounded_apply_cancel_intents_only",
    private_payload_policy: "references_and_checksums_only_no_private_payload_copy",
    unit_contract: {
      contract_ref: "DEL-02-02",
      schema_ref: "schemas/units.schema.yaml#/$defs/DimensionId",
      unit_policy: "unit_bearing_values_require_explicit_unit_metadata",
      missing_unit_behavior: "diagnostic_blocking"
    },
    source_of_truth_boundary: {
      surface: "DEL-07-03 editor_contract",
      gui_is_source_of_truth: false,
      mutation_route: "application_service_validation_before_persistence",
      direct_model_mutation_allowed: false,
      accepted_model_state_mutated: false
    },
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: professionalBoundary(),
    provenance: previewProvenance()
  };
}

function materialEditors(model: PreviewModel) {
  return (model.materials ?? []).map((material) => ({
    editor_id: `editor:${safeRefToken(material.id)}:material`,
    editor_kind: "material" as EditorKind,
    target_ref: reference("material", material.id),
    library_classification: "invented_public_example",
    source_provenance: material.provenance,
    fields: [
      field("label", "Label", material.label, "dimensionless", "none", material.id),
      field("elastic_modulus", "Elastic modulus", material.elastic_modulus.value, "stress", material.elastic_modulus.unit, material.id),
      field("shear_modulus", "Shear modulus", material.shear_modulus.value, "stress", material.shear_modulus.unit, material.id),
      field(
        "thermal_expansion_coefficient",
        "Thermal expansion",
        material.thermal_expansion_coefficient?.value ?? "TBD",
        "thermal_expansion_coefficient",
        material.thermal_expansion_coefficient?.unit ?? "TBD",
        material.id
      ),
      field("provenance", "Provenance", material.provenance, "dimensionless", "none", material.id)
    ],
    rule_pack_lifecycle: null,
    validation_state: "ready_for_service_validation" as EditorValidationState,
    save_intent: saveIntent()
  }));
}

function componentEditors(model: PreviewModel) {
  return model.components.map((component) => {
    const needsPrivateModifier = component.provenance.toLowerCase().includes("no_flexibility_factor");
    return {
      editor_id: `editor:${safeRefToken(component.id)}:component`,
      editor_kind: "component" as EditorKind,
      target_ref: reference("component", component.id),
      library_classification: "invented_public_example",
      source_provenance: component.provenance,
      fields: [
        field("label", "Label", component.label, "dimensionless", "none", component.id),
        field("kind", "Kind", component.kind, "dimensionless", "none", component.id),
        field("node", "Node", component.node, "dimensionless", "none", component.id),
        field("provenance", "Provenance", component.provenance, "dimensionless", "none", component.id)
      ],
      rule_pack_lifecycle: null,
      validation_state: needsPrivateModifier
        ? ("blocked_by_user_supplied_modifier" as EditorValidationState)
        : ("ready_for_service_validation" as EditorValidationState),
      save_intent: saveIntent()
    };
  });
}

function rulePackReferenceEditor(model: PreviewModel) {
  return {
    editor_id: "editor:rule-pack:user-supplied:not-loaded",
    editor_kind: "rule_pack_reference" as EditorKind,
    target_ref: reference("rule_pack", "rule-pack:user-supplied:not-loaded"),
    library_classification: "private_reference_only",
    source_provenance: "private_rule_pack_not_loaded_in_public_preview",
    fields: [
      field("rule_pack_id", "Rule pack ID", "rule-pack:user-supplied:not-loaded", "dimensionless", "none", model.project.id),
      field("version", "Version", "TBD", "dimensionless", "none", model.project.id),
      field("checksum", "Checksum", "TBD", "dimensionless", "none", model.project.id),
      field("required_inputs", "Required inputs", model.analysis_status.rule_check, "dimensionless", "none", model.project.id)
    ],
    rule_pack_lifecycle: {
      state: "referenced_placeholder",
      checksum: "TBD_private_rule_pack_not_loaded",
      private_payload_redacted: true
    },
    validation_state: "blocked_by_private_rule_inputs" as EditorValidationState,
    save_intent: saveIntent()
  };
}

function editorSurfaceInventory(model: PreviewModel) {
  return [
    {
      surface: "materials",
      count: model.materials?.length ?? 0,
      status: (model.materials?.length ?? 0) > 0 ? "review_editor_available" : "not_present"
    },
    {
      surface: "components",
      count: model.components.length,
      status: model.components.length > 0 ? "review_editor_available" : "not_present"
    },
    {
      surface: "load_cases",
      count: model.load_cases.length,
      status: model.load_cases.length > 0 ? "operation_intent_editor_available" : "not_present"
    },
    {
      surface: "supports",
      count: model.supports.length,
      status: model.supports.length > 0 ? "operation_intent_editor_available" : "not_present"
    },
    {
      surface: "viewport_gestures",
      count: 3,
      status: "operation_intent_editor_available"
    },
    {
      surface: "rule_pack_references",
      count: 1,
      status: "private_reference_only_missing_required_inputs"
    },
    {
      surface: "private_libraries",
      count: 0,
      status: "reference_slots_only_no_private_payload"
    }
  ];
}

function editorDiagnostics(model: PreviewModel): EditorDiagnostic[] {
  const diagnostics: EditorDiagnostic[] = [
    diagnostic(
      "RULE_PACK_CHECKSUM_TBD",
      "RULE_PACK_WARNING",
      "warning",
      reference("rule_pack", "rule-pack:user-supplied:not-loaded"),
      "Private rule-pack checksum is not available in the public technical preview.",
      "Load a user-owned rule pack through a governed private path before rule-check reliance."
    ),
    diagnostic(
      "RULE_PACK_REQUIRED_INPUTS_MISSING",
      "RULE_PACK_WARNING",
      "warning",
      reference("rule_pack", "rule-pack:user-supplied:not-loaded"),
      "Rule-check required inputs are absent, so editor state cannot become user-rule checked.",
      "Provide private project criteria, required inputs, provenance, and checksums before rule-pack evaluation."
    ),
    diagnostic(
      "PRIVATE_LIBRARY_PAYLOAD_NOT_LOADED",
      "PRIVATE_LIBRARY_WARNING",
      "info",
      reference("private_library", "private-library:user-controlled:not-loaded"),
      "Private material/component libraries are represented by slots only; no private payload is copied.",
      "Keep private libraries user-controlled unless an explicit export or contribution review authorizes sharing."
    )
  ];

  for (const component of model.components.filter((item) => item.provenance.toLowerCase().includes("no_flexibility_factor"))) {
    diagnostics.push(
      diagnostic(
        "COMPONENT_MODIFIER_PROVENANCE_REQUIRED",
        "PROVENANCE_WARNING",
        "warning",
        reference("component", component.id),
        "Component correction data is intentionally absent from the public preview.",
        "Treat SIF, flexibility, and proprietary component data as user-supplied or privately imported with provenance."
      )
    );
  }

  return diagnostics.sort((left, right) => left.code.localeCompare(right.code));
}

function field(
  fieldId: string,
  label: string,
  value: string | number,
  dimension: string,
  unit: string,
  sourceRef: string
): EditorField {
  return {
    field_id: fieldId,
    label,
    value,
    unit,
    dimension,
    editable: true,
    source_ref: reference("source", sourceRef),
    validation_state: value === "TBD" ? "unresolved_TBD" : "ready_for_service_validation"
  };
}

function diagnostic(
  code: EditorDiagnostic["code"],
  diagnosticClass: EditorDiagnostic["class"],
  severity: EditorDiagnostic["severity"],
  affectedRef: EditorReference,
  message: string,
  remediation: string
): EditorDiagnostic {
  return {
    code,
    class: diagnosticClass,
    severity,
    affected_ref: affectedRef,
    message,
    remediation
  };
}

function saveIntent() {
  return {
    status: "pending_service_validation",
    mutates_persistent_project: false
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
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/editor-contract/EditorContractPanel.tsx",
    source_license: "project-invented metadata only",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented editor metadata only; no protected standards, private project payload, private rule pack, or proprietary library payload.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending",
    privacy_classification: "public_metadata"
  };
}

function reference(refType: string, refId: string): EditorReference {
  return {
    ref_type: refType,
    ref_id: refId
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "ref";
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "preview";
}
