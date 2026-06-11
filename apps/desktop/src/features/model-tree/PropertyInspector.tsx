import { ListPlus, SearchCheck } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { EditorOperationIntent, EditorOperationObjectType, EntityRef, OperationOutcome, PreviewModel } from "../../types";
import { entityLabel, selectedProperties } from "../model-workspace/modelView";

export function PropertyInspector({
  model,
  onQueueIntent,
  onValidateIntent,
  operationBusy = false,
  operationOutcomes = {},
  queuedIntents = [],
  selection
}: {
  model: PreviewModel;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  onValidateIntent?: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
  operationOutcomes?: Record<string, OperationOutcome>;
  queuedIntents?: EditorOperationIntent[];
  selection: EntityRef;
}) {
  const properties = selectedProperties(model, selection);
  const editableFields = useMemo(() => editorFieldOptions(model, selection), [model, selection]);
  const [selectedFieldPath, setSelectedFieldPath] = useState(editableFields[0]?.fieldPath ?? "");
  const [proposedValue, setProposedValue] = useState(editableFields[0]?.before ?? "");
  const [rationale, setRationale] = useState("user_entered_preview_change");
  const selectedField = editableFields.find((field) => field.fieldPath === selectedFieldPath) ?? editableFields[0];
  const operationIntent = selectedField
    ? buildOperationIntent({
        field: selectedField,
        model,
        proposedValue,
        rationale,
        selection
      })
    : null;
  const inlineValidationOutcome = operationIntent
    ? matchingInlineValidationOutcome(operationOutcomes[operationIntentKey(operationIntent)], operationIntent)
    : null;
  const fieldChanged = Boolean(operationIntent && operationIntent.change.before !== operationIntent.change.after);

  useEffect(() => {
    const firstField = editableFields[0];
    setSelectedFieldPath(firstField?.fieldPath ?? "");
    setProposedValue(firstField?.before ?? "");
    setRationale("user_entered_preview_change");
  }, [editableFields, selection.id]);

  function handleFieldChange(fieldPath: string) {
    const nextField = editableFields.find((field) => field.fieldPath === fieldPath);
    setSelectedFieldPath(fieldPath);
    setProposedValue(nextField?.before ?? "");
  }

  function handleQueueIntent() {
    if (!operationIntent || !fieldChanged) return;
    onQueueIntent(operationIntent);
  }

  return (
    <div className="panel inspector" aria-label="Property inspector">
      <div className="panel-title">Properties</div>
      <h2>{entityLabel(model, selection.id)}</h2>
      <dl>
        {properties.map(([label, value]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{value}</dd>
          </div>
        ))}
      </dl>
      <section className="editor-intent" aria-label="Editor operation intent" data-testid="editor-intent-panel">
        <h3>Review-only edit intent</h3>
        {operationIntent ? (
          <>
            <div className="editor-intent-controls">
              <label>
                <span>Field</span>
                <select
                  aria-label="Intent field"
                  data-testid="editor-intent-field"
                  onChange={(event) => handleFieldChange(event.target.value)}
                  value={selectedField?.fieldPath ?? ""}
                >
                  {editableFields.map((field) => (
                    <option key={field.fieldPath} value={field.fieldPath}>
                      {field.label}
                    </option>
                  ))}
                </select>
              </label>
              <label>
                <span>Proposed value</span>
                <input
                  aria-label="Proposed editor value"
                  data-testid="editor-intent-value"
                  onChange={(event) => setProposedValue(event.target.value)}
                  value={proposedValue}
                />
              </label>
              <label>
                <span>Rationale</span>
                <input
                  aria-label="Editor intent rationale"
                  data-testid="editor-intent-rationale"
                  onChange={(event) => setRationale(event.target.value)}
                  value={rationale}
                />
              </label>
              <button
                data-testid="queue-editor-intent"
                disabled={!fieldChanged}
                onClick={handleQueueIntent}
                title="Queue as transient review-only operation"
                type="button"
              >
                <ListPlus size={14} aria-hidden="true" />
                Queue review intent
              </button>
              <button
                data-testid="validate-editor-intent-inline"
                disabled={!fieldChanged || operationBusy || !onValidateIntent}
                onClick={() => operationIntent && onValidateIntent?.(operationIntent)}
                title="Validate without applying"
                type="button"
              >
                <SearchCheck size={14} aria-hidden="true" />
                Validate intent
              </button>
            </div>
            <OperationIntentPreview intent={operationIntent} />
            <InlineValidationPreview intent={operationIntent} outcome={inlineValidationOutcome} />
            <IntentQueue intents={queuedIntents} />
          </>
        ) : (
          <p className="muted" data-testid="editor-intent-empty">
            Select an editable model entity to draft a structured operation intent.
          </p>
        )}
      </section>
    </div>
  );
}

function InlineValidationPreview({
  intent,
  outcome
}: {
  intent: EditorOperationIntent;
  outcome: OperationOutcome | null;
}) {
  if (!outcome) {
    return (
      <p className="muted editor-inline-validation" data-testid="editor-intent-inline-validation-empty">
        Validate this draft intent to preview structured-operation findings before queuing or applying it.
      </p>
    );
  }

  return (
    <article className="editor-inline-validation" data-testid="editor-intent-inline-validation">
      <strong data-testid="editor-intent-inline-validation-status">
        {outcome.mode}; application_status={outcome.validation.application_status}; schema=
        {outcome.validation.schema_validation}; unit={outcome.validation.unit_validation}; before_state=
        {outcome.validation.before_state_validation}
      </strong>
      {outcome.diff_preview.map((row) => (
        <small data-testid="editor-intent-inline-validation-diff" key={`${row.entity_ref}-${row.field_path}-${row.after}`}>
          diff: {row.entity_ref} {row.field_path} {row.before} to {row.after} [{row.unit}]
        </small>
      ))}
      {outcome.diagnostics.length === 0 ? (
        <small data-testid="editor-intent-inline-validation-boundary">
          {intent.operation_id}; validate-only; no accepted model mutation; no professional approval.
        </small>
      ) : (
        outcome.diagnostics.map((diagnostic) => (
          <small data-testid={`editor-intent-inline-validation-diagnostic-${diagnostic.code}`} key={diagnostic.id}>
            {diagnostic.severity}: {diagnostic.code} - {diagnostic.message}
          </small>
        ))
      )}
    </article>
  );
}

type EditableField = {
  label: string;
  fieldPath: string;
  before: string;
  objectType: EditorOperationObjectType;
  changeKind: "set_field" | "update_load" | "update_support";
  dimension: string;
  unit: string;
  sourceNote: string;
};

function OperationIntentPreview({ intent }: { intent: EditorOperationIntent }) {
  return (
    <article className="editor-intent-preview" data-testid="editor-operation-preview">
      <div className="editor-intent-meta">
        <IntentFact label="Operation" value={`${intent.operation_id}; ${intent.operation_kind}; ${intent.operation_status}`} />
        <IntentFact label="Target" value={`${intent.target.object_type}; ${intent.target.ref}`} />
        <IntentFact
          label="Change"
          value={`${intent.change.change_kind}; ${intent.change.field_label}; ${intent.change.field_path}; before=${intent.change.before}; after=${intent.change.after}`}
        />
        <IntentFact label="Unit basis" value={`${intent.change.dimension}; unit=${intent.change.unit}; ${intent.change.source_note}`} />
        <IntentFact
          label="Validation"
          value={`${intent.validation.schema_validation}; ${intent.validation.constraint_validation}; ${intent.validation.unit_validation}; ${intent.validation.diff_preview_status}; ${intent.validation.application_status}`}
          testId="editor-intent-validation"
        />
        <IntentFact
          label="Audit boundary"
          value={`${intent.audit_boundary.mutation_route}; direct_model_mutation_allowed=${String(
            intent.audit_boundary.direct_model_mutation_allowed
          )}; requires_user_acceptance=${String(intent.audit_boundary.requires_user_acceptance)}; mutates_accepted_model_state=${String(
            intent.audit_boundary.mutates_accepted_model_state
          )}`}
          testId="editor-intent-audit-boundary"
        />
        <IntentFact
          label="Professional boundary"
          value={`human_review_required=${String(
            intent.professional_boundary.human_review_required
          )}; software_makes_compliance_claim=${String(
            intent.professional_boundary.software_makes_compliance_claim
          )}; software_makes_approval_claim=${String(intent.professional_boundary.software_makes_approval_claim)}`}
          testId="editor-intent-professional-boundary"
        />
        <IntentFact label="Rationale" value={intent.rationale || "TBD"} />
      </div>
    </article>
  );
}

function IntentQueue({ intents = [] }: { intents?: EditorOperationIntent[] }) {
  if (!intents.length) {
    return (
      <p className="muted" data-testid="editor-intent-queue-empty">
        No transient editor intents queued.
      </p>
    );
  }

  return (
    <section className="editor-intent-queue" aria-label="Queued editor intents" data-testid="editor-intent-queue">
      {intents.map((intent) => (
        <article key={`${intent.queue_id}-${intent.operation_id}`} data-testid={`queued-${intent.queue_id}`}>
          <strong>{intent.queue_id}</strong>
          <span>{intent.operation_id}</span>
          <small>
            {intent.change.field_path}: {intent.change.before} to {intent.change.after}; application_status=
            {intent.validation.application_status}; mutates_accepted_model_state=
            {String(intent.audit_boundary.mutates_accepted_model_state)}
          </small>
        </article>
      ))}
    </section>
  );
}

function IntentFact({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="editor-intent-fact" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function editorFieldOptions(model: PreviewModel, selection: EntityRef): EditableField[] {
  const material = model.materials?.find((item) => item.id === selection.id);
  if (material) {
    return [
      scalarField("Label", "label", material.label, "Material", "dimensionless", "none", "material label only"),
      quantityField("Elastic modulus", "elastic_modulus.value", material.elastic_modulus.value, "Material", "stress", material.elastic_modulus.unit),
      quantityField("Shear modulus", "shear_modulus.value", material.shear_modulus.value, "Material", "stress", material.shear_modulus.unit),
      quantityField(
        "Thermal expansion",
        "thermal_expansion_coefficient.value",
        material.thermal_expansion_coefficient?.value ?? "TBD",
        "Material",
        "thermal_expansion_coefficient",
        material.thermal_expansion_coefficient?.unit ?? "TBD"
      ),
      scalarField("Provenance", "provenance", material.provenance, "Material", "dimensionless", "none", "public/private source note")
    ];
  }

  const node = model.nodes.find((item) => item.id === selection.id);
  if (node) {
    return [
      scalarField("Label", "label", node.label, "Node", "dimensionless", "none", "node label only"),
      quantityField("X position", "position.x", node.position.x, "Node", "length", model.project.units.length ?? "m"),
      quantityField("Y position", "position.y", node.position.y, "Node", "length", model.project.units.length ?? "m"),
      quantityField("Z position", "position.z", node.position.z, "Node", "length", model.project.units.length ?? "m"),
      scalarField("Provenance", "provenance", node.provenance, "Node", "dimensionless", "none", "public/private source note")
    ];
  }

  const pipe = model.pipe_segments.find((item) => item.id === selection.id);
  if (pipe) {
    return [
      scalarField("Label", "label", pipe.label, "Element", "dimensionless", "none", "pipe segment label only"),
      quantityField(
        "Outside diameter",
        "section.outside_diameter.value",
        pipe.section.outside_diameter?.value ?? "TBD",
        "Element",
        "length",
        pipe.section.outside_diameter?.unit ?? "TBD"
      ),
      quantityField(
        "Wall thickness",
        "section.wall_thickness.value",
        pipe.section.wall_thickness?.value ?? "TBD",
        "Element",
        "length",
        pipe.section.wall_thickness?.unit ?? "TBD"
      ),
      scalarField("Material", "material", pipe.material, "Element", "dimensionless", "none", "material reference"),
      scalarField("Provenance", "provenance", pipe.provenance, "Element", "dimensionless", "none", "public/private source note")
    ];
  }

  const support = model.supports.find((item) => item.id === selection.id);
  if (support) {
    return [
      scalarField("Label", "label", support.label, "Support", "dimensionless", "none", "support label only", "update_support"),
      scalarField("Node", "node", support.node, "Support", "dimensionless", "none", "target node reference", "update_support"),
      scalarField("Restraints", "restraints", support.restraints.join(", "), "Support", "dimensionless", "none", "restraint direction set", "update_support"),
      scalarField("Provenance", "provenance", support.provenance, "Support", "dimensionless", "none", "public/private source note", "update_support")
    ];
  }

  const component = model.components.find((item) => item.id === selection.id);
  if (component) {
    return [
      scalarField("Label", "label", component.label, "Component", "dimensionless", "none", "component label only"),
      scalarField("Kind", "kind", component.kind, "Component", "dimensionless", "none", "component type"),
      scalarField("Node", "node", component.node, "Component", "dimensionless", "none", "target node reference"),
      scalarField("Provenance", "provenance", component.provenance, "Component", "dimensionless", "none", "public/private source note")
    ];
  }

  const loadCase = model.load_cases.find((item) => item.id === selection.id);
  if (loadCase) {
    const firstLoad = loadCase.primitive_loads?.[0] ?? {};
    const firstMagnitude = firstLoad.magnitude as { value?: unknown; unit?: unknown } | undefined;
    return [
      scalarField("Label", "label", loadCase.label, "Load", "dimensionless", "none", "load case label only", "update_load"),
      scalarField("Status", "status", loadCase.status, "Load", "dimensionless", "none", "load case status", "update_load"),
      scalarField("Kind", "kind", loadCase.kind, "Load", "dimensionless", "none", "load case kind", "update_load"),
      quantityField(
        "First primitive magnitude",
        "primitive_loads.0.magnitude.value",
        typeof firstMagnitude?.value === "number" || typeof firstMagnitude?.value === "string" ? firstMagnitude.value : "TBD",
        "Load",
        typeof firstLoad.dimension === "string" ? firstLoad.dimension : "TBD",
        typeof firstMagnitude?.unit === "string" ? firstMagnitude.unit : "TBD",
        "update_load"
      ),
      scalarField("Provenance", "provenance", loadCase.provenance, "Load", "dimensionless", "none", "public/private source note", "update_load")
    ];
  }

  const combination = model.combinations?.find((item) => item.id === selection.id);
  if (combination) {
    return [
      scalarField("Label", "label", combination.label, "Combination", "dimensionless", "none", "combination label only"),
      scalarField("Basis", "basis", combination.basis, "Combination", "dimensionless", "none", "mechanics or user rule basis"),
      scalarField(
        "Terms",
        "terms",
        combination.terms.map((term) => `${term.load_case} x ${term.factor}`).join("; "),
        "Combination",
        "dimensionless",
        "none",
        "explicit user-defined terms"
      ),
      scalarField("Provenance", "provenance", combination.provenance, "Combination", "dimensionless", "none", "public/private source note")
    ];
  }

  return [];
}

function scalarField(
  label: string,
  fieldPath: string,
  before: string,
  objectType: EditorOperationObjectType,
  dimension: string,
  unit: string,
  sourceNote: string,
  changeKind: EditableField["changeKind"] = "set_field"
): EditableField {
  return { label, fieldPath, before, objectType, dimension, unit, sourceNote, changeKind };
}

function quantityField(
  label: string,
  fieldPath: string,
  before: number | string,
  objectType: EditorOperationObjectType,
  dimension: string,
  unit: string,
  changeKind: EditableField["changeKind"] = "set_field"
): EditableField {
  return {
    label,
    fieldPath,
    before: String(before),
    objectType,
    dimension,
    unit,
    sourceNote: "unit metadata required",
    changeKind
  };
}

function buildOperationIntent({
  field,
  model,
  proposedValue,
  rationale,
  selection
}: {
  field: EditableField;
  model: PreviewModel;
  proposedValue: string;
  rationale: string;
  selection: EntityRef;
}): EditorOperationIntent {
  const operationToken = `${safeToken(selection.id)}-${safeToken(field.fieldPath)}`;
  return {
    operation_id: `op:editor-intent-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    target: {
      object_type: field.objectType,
      ref: selection.id
    },
    change: {
      change_id: `change:${operationToken}`,
      change_kind: field.changeKind,
      field_label: field.label,
      field_path: field.fieldPath,
      before: field.before,
      after: proposedValue.trim() || "TBD",
      unit: field.unit,
      dimension: field.dimension,
      source_note: field.sourceNote
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `preview edit intent for ${model.project.id}`
  };
}

function matchingInlineValidationOutcome(
  outcome: OperationOutcome | undefined,
  intent: EditorOperationIntent
): OperationOutcome | null {
  if (!outcome) return null;
  if (outcome.operation_id !== intent.operation_id || outcome.change_id !== intent.change.change_id) return null;
  if (outcome.target_ref !== intent.target.ref || outcome.target_object_type !== intent.target.object_type) return null;
  if (outcome.change_kind !== intent.change.change_kind) return null;
  if (outcome.diff_preview.length === 0) {
    return outcome.validation.application_status === "blocked" ? outcome : null;
  }
  return outcome.diff_preview.some(
    (row) =>
      row.entity_ref === intent.target.ref &&
      row.field_path === intent.change.field_path &&
      row.before === intent.change.before &&
      row.after === intent.change.after
  )
    ? outcome
    : null;
}

function operationIntentKey(intent: EditorOperationIntent): string {
  return intent.queue_id ?? intent.operation_id;
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
