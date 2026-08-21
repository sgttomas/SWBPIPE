import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";

export type BendComponentDraft = {
  id: string;
  label: string;
  node: string;
  pipeRef: string;
  radius: string;
  radiusUnit: string;
  angle: string;
  angleUnit: string;
  planeOrientation: string;
  geometrySourceReference: string;
  provenance: string;
};

export function defaultBendComponentDraft(
  model: PreviewModel,
  selection: EntityRef | null,
  queuedIntents: EditorOperationIntent[]
): BendComponentDraft {
  const id = nextComponentId(model, queuedIntents);
  const selectedNode =
    selection?.type === "node" && model.nodes.some((node) => node.id === selection.id) ? selection.id : null;
  const node = selectedNode ?? model.nodes[0]?.id ?? "";
  const connectedPipe = model.pipe_segments.find((pipe) => pipe.from === node || pipe.to === node);
  return {
    id,
    label: `Bend ${shortEntityToken(id)}`,
    node,
    pipeRef: connectedPipe?.id ?? "",
    radius: "",
    radiusUnit: model.project.units.length ?? "TBD",
    angle: "",
    angleUnit: model.project.units.angle ?? "rad",
    planeOrientation: "",
    geometrySourceReference: "user_entered_component_form",
    provenance: "user_entered_local_preview"
  };
}

export function isBendComponentDraftValid(
  model: PreviewModel,
  draft: BendComponentDraft,
  queuedIntents: EditorOperationIntent[] = []
): boolean {
  const node = draft.node.trim();
  const pipeRef = draft.pipeRef.trim();
  const pipe = model.pipe_segments.find((candidate) => candidate.id === pipeRef);
  return Boolean(
    draft.id.trim() &&
      draft.label.trim() &&
      node &&
      pipe &&
      (pipe.from === node || pipe.to === node) &&
      positiveFinite(draft.radius) &&
      validUnit(draft.radiusUnit) &&
      positiveFinite(draft.angle) &&
      validUnit(draft.angleUnit) &&
      draft.planeOrientation.trim() &&
      draft.geometrySourceReference.trim() &&
      draft.provenance.trim() &&
      !model.components.some((component) => component.id === draft.id.trim()) &&
      !queuedIntents.some(
        (intent) =>
          intent.change.change_kind === "insert_component_symbol" &&
          intent.change.field_path === "components" &&
          intent.target.ref === draft.id.trim()
      )
  );
}

export function buildCreateBendComponentIntent({
  draft,
  sourceRef,
  sourceRole,
  unitValidation,
  sequence
}: {
  draft: BendComponentDraft;
  sourceRef: string;
  sourceRole: "gui_editor" | "viewport_editor";
  unitValidation: string;
  sequence?: number;
}): EditorOperationIntent {
  const componentId = draft.id.trim();
  const suffix = sequence === undefined ? "" : `-${sequence.toString().padStart(3, "0")}`;
  const payload = {
    id: componentId,
    label: draft.label.trim(),
    kind: "bend",
    node: draft.node.trim(),
    geometry: {
      bend_pipe_ref: draft.pipeRef.trim(),
      bend_radius: { value: Number(draft.radius), unit: draft.radiusUnit.trim() },
      bend_angle: { value: Number(draft.angle), unit: draft.angleUnit.trim() },
      bend_plane_orientation: draft.planeOrientation.trim(),
      bend_geometry_source_reference: draft.geometrySourceReference.trim()
    },
    provenance: draft.provenance.trim()
  };
  return {
    operation_id: `op:create-bend-${safeToken(componentId)}${suffix}`,
    operation_kind: "insert",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: sourceRef,
      source_channel: "local_desktop_preview",
      source_role: sourceRole
    },
    target: { object_type: "Component", ref: componentId },
    change: {
      change_id: `change:create-bend:${safeToken(componentId)}`,
      change_kind: "insert_component_symbol",
      field_label: "Explicit bend component",
      field_path: "components",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note:
        "explicit user-entered bend node, realized pipe connectivity, radius, angle, plane orientation, and geometry source"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: unitValidation,
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
    rationale: "explicit bend component creation; requires service validation before durable model change."
  };
}

function nextComponentId(model: PreviewModel, queuedIntents: EditorOperationIntent[]): string {
  const reserved = new Set(model.components.map((component) => component.id));
  for (const intent of queuedIntents) {
    if (intent.change.change_kind === "insert_component_symbol" && intent.change.field_path === "components") {
      reserved.add(intent.target.ref);
    }
  }
  for (let index = 1; index < 100000; index += 1) {
    const candidate = `component:C-${index}`;
    if (!reserved.has(candidate)) return candidate;
  }
  return "component:C-TBD";
}

function positiveFinite(value: string): boolean {
  return value.trim() !== "" && Number.isFinite(Number(value)) && Number(value) > 0;
}

function validUnit(value: string): boolean {
  const unit = value.trim();
  return Boolean(unit && unit !== "TBD");
}

function safeToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_.-]+/g, "-").replace(/^-+|-+$/g, "") || "unknown";
}

function shortEntityToken(value: string): string {
  const parts = value.split(":");
  return parts[parts.length - 1] || value;
}
