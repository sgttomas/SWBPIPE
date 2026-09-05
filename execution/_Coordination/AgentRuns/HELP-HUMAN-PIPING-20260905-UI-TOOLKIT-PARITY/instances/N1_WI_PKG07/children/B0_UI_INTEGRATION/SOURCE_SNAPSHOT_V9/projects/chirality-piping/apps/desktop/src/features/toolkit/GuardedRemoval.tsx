import { useEffect, useState } from "react";
import { canonicalJsonString } from "../../services/hashService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
type RemovalKind = "material" | "section" | "component";
const removalTypes = {
  material: "Material",
  section: "Section",
  component: "Component"
} as const;
const collections = {
  material: "materials",
  section: "sections",
  component: "components"
} as const;
export function removalEntity(model: PreviewModel, selection: EntityRef) {
  if (!(selection.type in removalTypes))
    return undefined;
  return model[collections[selection.type as RemovalKind]]?.find((entry) => entry.id === selection.id);
}
export function buildGuardedRemoval(selection: EntityRef, before: string): EditorOperationIntent {
  if (!(selection.type in removalTypes))
    throw new Error("Unsupported removal target");
  const kind = selection.type as RemovalKind;
  return {
    operation_id: `op:delete-${kind}:${selection.id}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/toolkit/GuardedRemoval.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: removalTypes[kind],
      ref: selection.id
    },
    change: {
      change_id: `change:delete-${kind}:${selection.id}`,
      change_kind: `delete_${kind}`,
      field_label: `Remove ${kind}`,
      field_path: collections[kind],
      before,
      after: "deleted",
      unit: "none",
      dimension: "dimensionless",
      source_note: "Explicit removal; whole entity precondition; inbound references block deletion without cascade."
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_required_dimensionless",
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
    rationale: "Remove the explicitly selected entity only after reference validation and human review."
  };
}
export function GuardedRemoval({ model, selection, onQueueIntent, operationBusy = false }: {
  model: PreviewModel;
  selection: EntityRef;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
}) {
  const entity = removalEntity(model, selection);
  const [prepared, setPrepared] = useState<{
    model: PreviewModel;
    entity: unknown;
    type: string;
    intent: EditorOperationIntent;
  } | null>(null);
  const [error, setError] = useState("");
  useEffect(() => {
    let active = true;
    setPrepared(null);
    setError("");
    if (entity)
      canonicalJsonString(entity).then((before) => {
        if (active)
          setPrepared({
            model,
        entity,
        type: selection.type,
        intent: buildGuardedRemoval(selection, before)
          });
      }).catch(() => {
        if (active)
          setError("The removal preview could not be prepared. Check that the operation engine is available.");
      });
    return () => {
      active = false;
    };
  }, [model, entity, selection.id, selection.type]);
  if (!entity)
    return null;
  const ready = prepared?.model === model && prepared?.entity === entity && prepared?.type === selection.type && prepared.intent.target.ref === selection.id;
  return (
    <section
      className="editor-intent"
      tabIndex={-1}
      data-testid="guarded-removal-panel"
      aria-label="Remove selected entity"
    >
      <h3>Remove {selection.type}</h3>
      <p>
        Remove <strong>{"label" in entity ? entity.label : entity.name}</strong> ({entity.id}).
        References from other records can block removal. No connected records will be removed automatically.
      </p>
      <button
        type="button"
        data-testid={`queue-delete-${selection.type}-intent`}
        disabled={!ready || operationBusy}
        onClick={() => {
          if (ready && prepared && !operationBusy) onQueueIntent(prepared.intent);
        }}
      >
        Queue removal for review
      </button>
      {error ? <p role="alert">{error}</p> : !ready ? <p role="status">Preparing the current entity snapshot…</p> : null}
      <p>The queued change must be validated and explicitly applied in Pending changes.</p>
    </section>
  );
}
