import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { isPressureV3 } from "../pressure-authoring/pressureContract";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";

export const creatableComponentKinds = ["bend", "tee", "reducer", "valve", "flange", "expansion_joint"] as const;
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
} & ConnectorDraft;

/** T4-U3/T4-U2: an expansion joint is authored as an objective connector
 * (objective_connector 1.0.0) on a 3.0.0/exact_pressure_v3 document, replacing
 * the selected span (the primary pipe), whose end nodes are its ends. Every
 * value is the user's; nothing is inferred from the pipe or defaulted. Matrices
 * are three rows of three numbers ("1 0 0; 0 1 0; 0 0 1"), vectors three
 * numbers, and the work matrix the 21 upper-triangle coefficients (N*m) in the
 * order tx, ty, tz, rx, ry, rz. */
export type ConnectorDraft = {
  connectorAxes: string;
  endINodeAxes: string;
  endJNodeAxes: string;
  endIOffset: string;
  endJOffset: string;
  offsetUnit: string;
  qRefTranslation: string;
  qRefTranslationUnit: string;
  qRefRotation: string;
  qRefRotationUnit: string;
  referenceState: string;
  installedTemperature: string;
  installedTemperatureUnit: string;
  translationScale: string;
  translationScaleUnit: string;
  workMatrixUpperTriangle: string;
  stiffnessSourceReference: string;
  measurementRestraints: string;
  basisTransformReference: string;
  validityStatement: string;
  installedGeometry: string;
  hardwareKind: string;
  /** T4-U5b: the explicit tie rods of `explicit_axial_rods_v1`, one
   * `TieRodDraft` per row, held as the JSON text of that row list ("" when
   * none) so the draft stays a record of strings. */
  tieRods: string;
  pressureModel: string;
  effectiveArea: string;
  effectiveAreaUnit: string;
  pressureBasis: string;
  pressureSourceReference: string;
};
export const CONNECTOR_REFERENCE_STATES = ["stress_free", "prestressed"] as const;
/** The hardware a connector admits: untied, and on the current model version (T4-U5b,
 * DEL-04-07 Design u4b-u5b-implementation.md §3.1) explicit bilateral linear
 * elastic tie rods. Ideal, tension-only and gapped rods, nuts, stops and
 * collective constraints are not offered. */
export const CONNECTOR_HARDWARE_KINDS = ["untied"] as const;
export const CURRENT_CONNECTOR_HARDWARE_KINDS = ["untied", "explicit_axial_rods_v1"] as const;
export function connectorHardwareKinds(model: PreviewModel): readonly string[] {
  return model.schema_version === MODEL_SCHEMA_VERSION ? CURRENT_CONNECTOR_HARDWARE_KINDS : CONNECTOR_HARDWARE_KINDS;
}
/** One tie rod of the table. Offsets are in the connector axes from the
 * connector's end nodes i and j; every value is the user's. */
export type TieRodDraft = {
  id: string;
  offsetI: string;
  offsetJ: string;
  offsetUnit: string;
  stiffness: string;
  stiffnessUnit: string;
  referenceExtension: string;
  referenceExtensionUnit: string;
  sourceReference: string;
};
export const TIE_ROD_LAW = "bilateral_linear";
export function emptyTieRodDraft(): TieRodDraft {
  return { id: "", offsetI: "", offsetJ: "", offsetUnit: "", stiffness: "", stiffnessUnit: "", referenceExtension: "", referenceExtensionUnit: "", sourceReference: "" };
}
/** The rows of a draft's tie-rod table (none when absent or unreadable). */
export function tieRodDrafts(text: string): TieRodDraft[] {
  if (!text.trim()) return [];
  try {
    const rows = JSON.parse(text);
    return Array.isArray(rows) ? rows.map((row) => ({ ...emptyTieRodDraft(), ...(row as Partial<TieRodDraft>) })) : [];
  } catch {
    return [];
  }
}
/**
 * Rod ids are distinct as the producer compares them: each trimmed id and its
 * encoded form (':' replaced by '-', the producer's stable suffix, which its
 * row ids carry) must be unique, so "rod:a" and "rod-a" collide.
 */
export function tieRodIdsDistinct(ids: readonly string[]): boolean {
  const trimmed = ids.map((id) => id.trim());
  const encoded = trimmed.map((id) => id.replaceAll(":", "-"));
  return new Set(trimmed).size === trimmed.length && new Set(encoded).size === encoded.length;
}
function tieRodsValid(text: string): boolean {
  const rows = tieRodDrafts(text);
  const ids = rows.map((row) => row.id.trim());
  return rows.length > 0 && tieRodIdsDistinct(ids) && rows.every((row) =>
    row.id.trim() && vector3(row.offsetI) && vector3(row.offsetJ) && validUnit(row.offsetUnit) &&
    positiveFinite(row.stiffness) && validUnit(row.stiffnessUnit) &&
    finiteNumber(row.referenceExtension) && validUnit(row.referenceExtensionUnit) && row.sourceReference.trim());
}
export const CONNECTOR_PRESSURE_MODELS = ["unpressurized"] as const;
export const CURRENT_CONNECTOR_PRESSURE_MODELS = ["unpressurized", "constant_effective_area_reference_v1"] as const;
export function connectorPressureModels(model: PreviewModel): readonly string[] {
  return model.schema_version === MODEL_SCHEMA_VERSION ? CURRENT_CONNECTOR_PRESSURE_MODELS : CONNECTOR_PRESSURE_MODELS;
}
export function emptyConnectorDraft(): ConnectorDraft {
  return {
    connectorAxes: "", endINodeAxes: "", endJNodeAxes: "", endIOffset: "", endJOffset: "", offsetUnit: "",
    qRefTranslation: "", qRefTranslationUnit: "", qRefRotation: "", qRefRotationUnit: "", referenceState: "",
    installedTemperature: "", installedTemperatureUnit: "", translationScale: "", translationScaleUnit: "",
    workMatrixUpperTriangle: "", stiffnessSourceReference: "", measurementRestraints: "", basisTransformReference: "",
    validityStatement: "", installedGeometry: "", hardwareKind: "", tieRods: "", pressureModel: "",
    effectiveArea: "", effectiveAreaUnit: "", pressureBasis: "", pressureSourceReference: ""
  };
}
/** Expansion joints are created only on a 3.0.0/exact_pressure_v3 document. */
export function connectorCreationAvailable(model: PreviewModel): boolean {
  return isPressureV3(model);
}
export const CONNECTOR_CONTRACT_NOTICE = "An expansion joint is created as an objective connector, which needs a 3.0.0/exact_pressure_v3 document: select that pressure profile first. It replaces the selected span; its hardware is untied or explicit bilateral linear elastic tie rods.";
function numbers(text: string): number[] | null {
  const parts = text.split(/[\s,]+/).filter(Boolean);
  const values = parts.map(Number);
  return parts.length && values.every(Number.isFinite) ? values : null;
}
function vector3(text: string): number[] | null {
  const values = numbers(text);
  return values && values.length === 3 ? values : null;
}
function matrix3(text: string): number[][] | null {
  const rows = text.split(";").map(row => vector3(row));
  return rows.length === 3 && rows.every(Boolean) ? rows as number[][] : null;
}
function upperTriangle(text: string): number[] | null {
  const values = numbers(text);
  return values && values.length === 21 ? values : null;
}
function connectorDraftValid(model: PreviewModel, draft: ConnectorDraft): boolean {
  return Boolean(
    matrix3(draft.connectorAxes) && matrix3(draft.endINodeAxes) && matrix3(draft.endJNodeAxes) &&
    vector3(draft.endIOffset) && vector3(draft.endJOffset) && validUnit(draft.offsetUnit) &&
    vector3(draft.qRefTranslation) && validUnit(draft.qRefTranslationUnit) &&
    vector3(draft.qRefRotation) && validUnit(draft.qRefRotationUnit) &&
    (CONNECTOR_REFERENCE_STATES as readonly string[]).includes(draft.referenceState) &&
    finiteNumber(draft.installedTemperature) && validUnit(draft.installedTemperatureUnit) &&
    positiveFinite(draft.translationScale) && validUnit(draft.translationScaleUnit) &&
    upperTriangle(draft.workMatrixUpperTriangle) &&
    [draft.stiffnessSourceReference, draft.measurementRestraints, draft.basisTransformReference, draft.validityStatement, draft.installedGeometry].every(text => text.trim()) &&
    connectorHardwareKinds(model).includes(draft.hardwareKind) &&
    (draft.hardwareKind !== "explicit_axial_rods_v1" || tieRodsValid(draft.tieRods)) &&
    connectorPressureModels(model).includes(draft.pressureModel) &&
    (draft.pressureModel !== "constant_effective_area_reference_v1" ||
      (positiveFinite(draft.effectiveArea) && validUnit(draft.effectiveAreaUnit) && draft.pressureBasis === "internal_differential_zero_external_v1" && draft.pressureSourceReference.trim()))
  );
}
/** The objective connector record for the span the draft names. */
function objectiveConnector(model: PreviewModel, draft: ConnectorDraft & { primaryPipeRef: string }): Record<string, unknown> {
  const span = model.pipe_segments.find(pipe => pipe.id === draft.primaryPipeRef.trim());
  if (!span) throw new Error("CONNECTOR_SPAN_MISSING");
  const xyz = (text: string, unit: string) => { const [x, y, z] = vector3(text)!; return { x, y, z, unit: unit.trim() }; };
  const provenance = {
    source_reference: draft.stiffnessSourceReference.trim(),
    measurement_restraints: draft.measurementRestraints.trim(),
    basis_transform_reference: draft.basisTransformReference.trim()
  };
  return {
    version: "1.0.0",
    motion_basis: "symmetric_midpoint_small_rotation_v1",
    end_i: { node_ref: span.from, initial_node_axes_global: matrix3(draft.endINodeAxes), offset_local: xyz(draft.endIOffset, draft.offsetUnit) },
    end_j: { node_ref: span.to, initial_node_axes_global: matrix3(draft.endJNodeAxes), offset_local: xyz(draft.endJOffset, draft.offsetUnit) },
    connector_axes_global: matrix3(draft.connectorAxes),
    installed_reference_temperature: { value: Number(draft.installedTemperature), unit: draft.installedTemperatureUnit.trim() },
    temperature_applicability: "fixed_installed_parameters_v1",
    reference_state: draft.referenceState,
    q_ref: { translation: xyz(draft.qRefTranslation, draft.qRefTranslationUnit), rotation: xyz(draft.qRefRotation, draft.qRefRotationUnit) },
    stiffness: {
      version: "1.0.0",
      representation: "scaled_work_coefficients_v1",
      translation_scale: { value: Number(draft.translationScale), unit: draft.translationScaleUnit.trim() },
      rotation_scale: { value: 1, unit: "rad" },
      coefficient_unit: "N*m",
      upper_triangle: upperTriangle(draft.workMatrixUpperTriangle),
      coordinate_order: ["tx", "ty", "tz", "rx", "ry", "rz"],
      provenance
    },
    topology: { type: "replaces_span", span_ref: span.id },
    calibration: { kind: "constant_structural_elasticity_v1", includes_pressure_dependent_tangent: false, installed_geometry: draft.installedGeometry.trim() },
    hardware: draft.hardwareKind === "explicit_axial_rods_v1"
      ? {
          kind: draft.hardwareKind,
          rods: tieRodDrafts(draft.tieRods).map((row) => {
            const offset = (text: string) => {
              const [x, y, z] = vector3(text)!;
              const unit = row.offsetUnit.trim();
              return { x: { value: x, unit }, y: { value: y, unit }, z: { value: z, unit } };
            };
            return {
              id: row.id.trim(),
              law: TIE_ROD_LAW,
              offset_i: offset(row.offsetI),
              offset_j: offset(row.offsetJ),
              stiffness: { value: Number(row.stiffness), unit: row.stiffnessUnit.trim() },
              reference_extension: { value: Number(row.referenceExtension), unit: row.referenceExtensionUnit.trim() },
              source_reference: row.sourceReference.trim()
            };
          })
        }
      : { kind: draft.hardwareKind },
    pressure_model: draft.pressureModel === "constant_effective_area_reference_v1"
      ? { kind: draft.pressureModel, effective_area: { value: Number(draft.effectiveArea), unit: draft.effectiveAreaUnit.trim() }, pressure_basis: draft.pressureBasis, source_reference: draft.pressureSourceReference.trim() }
      : { kind: draft.pressureModel },
    provenance: { ...provenance, validity_statement: draft.validityStatement.trim() }
  };
}

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
  const units = model.project.units as PreviewModel["project"]["units"] | undefined;
  return {
    id,
    label: `${componentKindLabel(kind)} ${shortEntityToken(id)}`,
    kind,
    node,
    primaryPipeRef: kind === "bend" ? (connectedPipes[0]?.id ?? "") : "",
    secondaryPipeRef: "",
    // W-17 §3: an absent project key leaves the unit unselected ("TBD"); nothing is assumed.
    lengthUnit: units?.length ?? "TBD",
    angleUnit: units?.angle ?? "TBD",
    forceUnit: units?.force ?? "TBD",
    bendRadius: "",
    bendAngle: "",
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
    ...emptyConnectorDraft(),
    geometrySourceReference: kind === "expansion_joint" ? "" : "user_entered_component_form",
    provenance: kind === "expansion_joint" ? "" : "user_entered_local_preview"
  };
}

export function componentDraftForKind(
  model: PreviewModel,
  draft: ComponentDraft,
  kind: CreatableComponentKind
): ComponentDraft {
  const primaryPipeRef = kind === "bend" ? (incidentPipes(model, draft.node)[0]?.id ?? "") : "";
  const leavingExpansionJoint = draft.kind === "expansion_joint" && kind !== "expansion_joint";
  return {
    ...draft,
    ...(kind === "expansion_joint" ? emptyExpansionJointInputs() : {}),
    ...(leavingExpansionJoint
      ? {
          geometrySourceReference: "user_entered_component_form",
          provenance: "user_entered_local_preview"
        }
      : {}),
    kind,
    label: `${componentKindLabel(kind)} ${shortEntityToken(draft.id)}`,
    primaryPipeRef,
    secondaryPipeRef: ""
  };
}

function emptyExpansionJointInputs(): Partial<ComponentDraft> {
  return { ...emptyConnectorDraft(), geometrySourceReference: "", provenance: "" };
}

export function componentDraftForNode(model: PreviewModel, draft: ComponentDraft, node: string): ComponentDraft {
  const connectedPipes = incidentPipes(model, node);
  return {
    ...draft,
    node,
    primaryPipeRef: draft.kind === "bend" ? (connectedPipes[0]?.id ?? "") : "",
    secondaryPipeRef: ""
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
      (draft.kind === "expansion_joint" || draft.geometrySourceReference.trim()) &&
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
        validUnit(draft.angleUnit)
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
  if (draft.kind === "expansion_joint") {
    return Boolean(
      connectorCreationAvailable(model) &&
        incidentPipe(model, draft.node, draft.primaryPipeRef) &&
        connectorDraftValid(model, draft)
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
  model,
  sourceRef,
  sourceRole,
  unitValidation,
  sequence
}: {
  draft: ComponentDraft;
  /** The document the component is created in (the connector's span ends). */
  model: PreviewModel;
  sourceRef: string;
  sourceRole: "gui_editor" | "viewport_editor";
  unitValidation: string;
  sequence?: number;
}): EditorOperationIntent {
  const componentId = draft.id.trim();
  const kind = draft.kind;
  const suffix = sequence === undefined ? "" : `-${sequence.toString().padStart(3, "0")}`;
  const payload = kind === "expansion_joint"
    ? {
        id: componentId,
        label: draft.label.trim(),
        kind,
        node: draft.node.trim(),
        objective_connector: objectiveConnector(model, draft),
        provenance: draft.provenance.trim()
      }
    : {
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
    return "explicit user-entered bend node, realized pipe connectivity, radius, angle, and geometry source";
  }
  if (kind === "tee") {
    return "explicit user-entered tee node, two-pipe connectivity, run/header sizes, connection angle/type, reinforcement reference, and geometry source";
  }
  if (kind === "expansion_joint") {
    return "explicit user-entered objective connector (objective_connector 1.0.0): replaced span, frames, offsets, installed reference state, 6x6 work matrix, calibration, hardware, pressure model and provenance";
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
