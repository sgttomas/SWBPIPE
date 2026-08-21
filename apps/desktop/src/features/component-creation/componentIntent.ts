import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";

export const creatableComponentKinds = ["bend", "tee", "reducer", "valve", "flange"] as const;
export type CreatableComponentKind = (typeof creatableComponentKinds)[number];

export type ComponentDraft = {
  id: string;
  label: string;
  kind: CreatableComponentKind;
  node: string;
  primaryPipeRef: string;
  secondaryPipeRef: string;
  lengthUnit: string;
  angleUnit: string;
  forceUnit: string;
  bendRadius: string;
  bendAngle: string;
  bendPlaneOrientation: string;
  branchRunSize: string;
  branchHeaderSize: string;
  branchConnectionAngle: string;
  branchConnectionType: string;
  branchReinforcementReference: string;
  rigidBodyLength: string;
  endASize: string;
  endBSize: string;
  weight: string;
  centerOfGravityX: string;
  centerOfGravityY: string;
  centerOfGravityZ: string;
  connectionEndAReference: string;
  connectionEndBReference: string;
  stiffnessBehaviorReference: string;
  geometrySourceReference: string;
  provenance: string;
};

export function defaultComponentDraft(
  model: PreviewModel,
  selection: EntityRef | null,
  queuedIntents: EditorOperationIntent[],
  kind: CreatableComponentKind = "bend"
): ComponentDraft {
  const id = nextComponentId(model, queuedIntents);
  const selectedNode =
    selection?.type === "node" && model.nodes.some((node) => node.id === selection.id) ? selection.id : null;
  const node = selectedNode ?? model.nodes[0]?.id ?? "";
  const connectedPipes = incidentPipes(model, node);
  return {
    id,
    label: `${componentKindLabel(kind)} ${shortEntityToken(id)}`,
    kind,
    node,
    primaryPipeRef: connectedPipes[0]?.id ?? "",
    secondaryPipeRef: connectedPipes[1]?.id ?? "",
    lengthUnit: model.project.units.length ?? "TBD",
    angleUnit: model.project.units.angle ?? "rad",
    forceUnit: model.project.units.force ?? "TBD",
    bendRadius: "",
    bendAngle: "",
    bendPlaneOrientation: "",
    branchRunSize: "",
    branchHeaderSize: "",
    branchConnectionAngle: "",
    branchConnectionType: "",
    branchReinforcementReference: "",
    rigidBodyLength: "",
    endASize: "",
    endBSize: "",
    weight: "",
    centerOfGravityX: "",
    centerOfGravityY: "",
    centerOfGravityZ: "",
    connectionEndAReference: "",
    connectionEndBReference: "",
    stiffnessBehaviorReference: "",
    geometrySourceReference: "user_entered_component_form",
    provenance: "user_entered_local_preview"
  };
}

export function componentDraftForKind(draft: ComponentDraft, kind: CreatableComponentKind): ComponentDraft {
  return {
    ...draft,
    kind,
    label: `${componentKindLabel(kind)} ${shortEntityToken(draft.id)}`
  };
}

export function componentDraftForNode(model: PreviewModel, draft: ComponentDraft, node: string): ComponentDraft {
  const connectedPipes = incidentPipes(model, node);
  return {
    ...draft,
    node,
    primaryPipeRef: connectedPipes[0]?.id ?? "",
    secondaryPipeRef: connectedPipes[1]?.id ?? ""
  };
}

export function isComponentDraftValid(
  model: PreviewModel,
  draft: ComponentDraft,
  queuedIntents: EditorOperationIntent[] = []
): boolean {
  const commonValid = Boolean(
    draft.id.trim() &&
      draft.label.trim() &&
      draft.node.trim() &&
      model.nodes.some((node) => node.id === draft.node.trim()) &&
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
  if (!commonValid) return false;

  if (draft.kind === "bend") {
    return Boolean(
      incidentPipe(model, draft.node, draft.primaryPipeRef) &&
        positiveFinite(draft.bendRadius) &&
        validUnit(draft.lengthUnit) &&
        positiveFinite(draft.bendAngle) &&
        validUnit(draft.angleUnit) &&
        draft.bendPlaneOrientation.trim()
    );
  }
  if (draft.kind === "tee") {
    return Boolean(
      draft.primaryPipeRef.trim() !== draft.secondaryPipeRef.trim() &&
        incidentPipe(model, draft.node, draft.primaryPipeRef) &&
        incidentPipe(model, draft.node, draft.secondaryPipeRef) &&
        positiveFinite(draft.branchRunSize) &&
        positiveFinite(draft.branchHeaderSize) &&
        validUnit(draft.lengthUnit) &&
        positiveFinite(draft.branchConnectionAngle) &&
        validUnit(draft.angleUnit) &&
        draft.branchConnectionType.trim() &&
        draft.branchReinforcementReference.trim()
    );
  }
  return Boolean(
    incidentPipe(model, draft.node, draft.primaryPipeRef) &&
      positiveFinite(draft.rigidBodyLength) &&
      positiveFinite(draft.endASize) &&
      positiveFinite(draft.endBSize) &&
      validUnit(draft.lengthUnit) &&
      positiveFinite(draft.weight) &&
      validUnit(draft.forceUnit) &&
      finiteNumber(draft.centerOfGravityX) &&
      finiteNumber(draft.centerOfGravityY) &&
      finiteNumber(draft.centerOfGravityZ) &&
      draft.connectionEndAReference.trim() &&
      draft.connectionEndBReference.trim() &&
      draft.stiffnessBehaviorReference.trim()
  );
}

export function buildCreateComponentIntent({
  draft,
  sourceRef,
  sourceRole,
  unitValidation,
  sequence
}: {
  draft: ComponentDraft;
  sourceRef: string;
  sourceRole: "gui_editor" | "viewport_editor";
  unitValidation: string;
  sequence?: number;
}): EditorOperationIntent {
  const componentId = draft.id.trim();
  const kind = draft.kind;
  const suffix = sequence === undefined ? "" : `-${sequence.toString().padStart(3, "0")}`;
  const payload = {
    id: componentId,
    label: draft.label.trim(),
    kind,
    node: draft.node.trim(),
    geometry: componentGeometry(draft),
    provenance: draft.provenance.trim()
  };
  return {
    operation_id: `op:create-${kind}-${safeToken(componentId)}${suffix}`,
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
      change_id: `change:create-${kind}:${safeToken(componentId)}`,
      change_kind: "insert_component_symbol",
      field_label: `Explicit ${kind} component`,
      field_path: "components",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note: componentSourceNote(kind)
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
    rationale: `explicit ${kind} component creation; requires service validation before durable model change.`
  };
}

function componentGeometry(draft: ComponentDraft): Record<string, unknown> {
  if (draft.kind === "bend") {
    return {
      bend_pipe_ref: draft.primaryPipeRef.trim(),
      bend_radius: { value: Number(draft.bendRadius), unit: draft.lengthUnit.trim() },
      bend_angle: { value: Number(draft.bendAngle), unit: draft.angleUnit.trim() },
      bend_plane_orientation: draft.bendPlaneOrientation.trim(),
      bend_geometry_source_reference: draft.geometrySourceReference.trim()
    };
  }
  if (draft.kind === "tee") {
    return {
      branch_header_pipe_ref: draft.primaryPipeRef.trim(),
      branch_branch_pipe_ref: draft.secondaryPipeRef.trim(),
      branch_run_size: { value: Number(draft.branchRunSize), unit: draft.lengthUnit.trim() },
      branch_header_size: { value: Number(draft.branchHeaderSize), unit: draft.lengthUnit.trim() },
      branch_connection_angle: { value: Number(draft.branchConnectionAngle), unit: draft.angleUnit.trim() },
      branch_connection_type: draft.branchConnectionType.trim(),
      branch_reinforcement_reference: draft.branchReinforcementReference.trim(),
      branch_geometry_source_reference: draft.geometrySourceReference.trim()
    };
  }
  return {
    rigid_pipe_ref: draft.primaryPipeRef.trim(),
    rigid_body_length: { value: Number(draft.rigidBodyLength), unit: draft.lengthUnit.trim() },
    end_a_size: { value: Number(draft.endASize), unit: draft.lengthUnit.trim() },
    end_b_size: { value: Number(draft.endBSize), unit: draft.lengthUnit.trim() },
    weight: { value: Number(draft.weight), unit: draft.forceUnit.trim() },
    center_of_gravity: {
      x: Number(draft.centerOfGravityX),
      y: Number(draft.centerOfGravityY),
      z: Number(draft.centerOfGravityZ),
      unit: draft.lengthUnit.trim()
    },
    connection_end_a_reference: draft.connectionEndAReference.trim(),
    connection_end_b_reference: draft.connectionEndBReference.trim(),
    stiffness_behavior_reference: draft.stiffnessBehaviorReference.trim(),
    rigid_component_source_reference: draft.geometrySourceReference.trim()
  };
}

function componentSourceNote(kind: CreatableComponentKind): string {
  if (kind === "bend") {
    return "explicit user-entered bend node, realized pipe connectivity, radius, angle, plane orientation, and geometry source";
  }
  if (kind === "tee") {
    return "explicit user-entered tee node, two-pipe connectivity, run/header sizes, connection angle/type, reinforcement reference, and geometry source";
  }
  return `explicit user-entered ${kind} node, realized pipe connectivity, dimensions, weight, center of gravity, connection references, stiffness behavior, and geometry source`;
}

function incidentPipes(model: PreviewModel, node: string) {
  return model.pipe_segments.filter((pipe) => pipe.from === node || pipe.to === node);
}

function incidentPipe(model: PreviewModel, node: string, pipeRef: string): boolean {
  return incidentPipes(model, node).some((pipe) => pipe.id === pipeRef.trim());
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

function finiteNumber(value: string): boolean {
  return value.trim() !== "" && Number.isFinite(Number(value));
}

function validUnit(value: string): boolean {
  const unit = value.trim();
  return Boolean(unit && unit !== "TBD");
}

function componentKindLabel(kind: CreatableComponentKind): string {
  return kind.charAt(0).toUpperCase() + kind.slice(1);
}

function safeToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_.-]+/g, "-").replace(/^-+|-+$/g, "") || "unknown";
}

function shortEntityToken(value: string): string {
  const parts = value.split(":");
  return parts[parts.length - 1] || value;
}
