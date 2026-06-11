import { invoke } from "@tauri-apps/api/core";
import { canonicalJson } from "./hashService";
import type {
  EditorOperationIntent,
  ModelHashEvidence,
  OperationDiffPreviewRow,
  OperationOutcome,
  OperationOutcomeDiagnostic,
  PreviewModel
} from "../types";

// Browser-mode mirror of `core/model_operations/operation_applier`. The Tauri
// backend is the authoritative apply route; this local engine exists so the
// browser fixture mode (vite dev without Tauri, jsdom tests) exercises the
// same operation → validation → diff → apply loop with an honestly labeled
// `application_route`. Semantics must stay aligned with the Rust crate; the
// crate's tests are the contract reference.

const OPERATION_APPLIER_VERSION = "0.1.0";
const OUTCOME_DOCUMENT_KIND = "openpipestress.desktop.operation_outcome";
const LOCAL_CANONICALIZATION = "jcs_like_sorted_object_keys";
const ENGINE_SOURCE = "apps/desktop/src/services/operationService.ts";

const RESTRAINT_TOKENS = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

const CANONICAL_DIMENSIONS = new Set([
  "dimensionless",
  "length",
  "mass",
  "time",
  "temperature",
  "temperature_interval",
  "angle",
  "rotation",
  "force",
  "moment",
  "pressure",
  "stress",
  "area",
  "volume",
  "density",
  "linear_stiffness",
  "rotational_stiffness",
  "displacement",
  "velocity",
  "acceleration",
  "thermal_conductivity",
  "specific_heat",
  "thermal_expansion_coefficient",
  "second_moment_area",
  "section_modulus",
  "mass_per_length",
  "volume_per_length",
  "slope",
  "force_per_length",
  "TBD"
]);

// Unit basis for a quantity: "sibling" reads the `unit` field next to the
// value; "project_units:<key>" reads the project unit system entry (bare
// numeric fields such as node positions).
type UnitSource = "sibling" | `project_units:${string}`;

type FieldKind =
  | { kind: "text" }
  | { kind: "quantity"; requirePositive: boolean; unitSource: UnitSource }
  | { kind: "entity_ref"; collection: string }
  | { kind: "restraint_set" };

const FIELD_RULES: Record<string, Record<string, FieldKind>> = {
  Material: {
    label: { kind: "text" },
    provenance: { kind: "text" },
    "elastic_modulus.value": { kind: "quantity", requirePositive: true, unitSource: "sibling" },
    "shear_modulus.value": { kind: "quantity", requirePositive: true, unitSource: "sibling" },
    "thermal_expansion_coefficient.value": { kind: "quantity", requirePositive: false, unitSource: "sibling" }
  },
  Node: {
    label: { kind: "text" },
    provenance: { kind: "text" },
    "position.x": { kind: "quantity", requirePositive: false, unitSource: "project_units:length" },
    "position.y": { kind: "quantity", requirePositive: false, unitSource: "project_units:length" },
    "position.z": { kind: "quantity", requirePositive: false, unitSource: "project_units:length" }
  },
  Element: {
    label: { kind: "text" },
    provenance: { kind: "text" },
    material: { kind: "entity_ref", collection: "materials" },
    "section.outside_diameter.value": { kind: "quantity", requirePositive: true, unitSource: "sibling" },
    "section.wall_thickness.value": { kind: "quantity", requirePositive: true, unitSource: "sibling" }
  },
  Support: {
    label: { kind: "text" },
    provenance: { kind: "text" },
    node: { kind: "entity_ref", collection: "nodes" },
    restraints: { kind: "restraint_set" }
  },
  Component: {
    label: { kind: "text" },
    provenance: { kind: "text" },
    node: { kind: "entity_ref", collection: "nodes" }
  },
  Load: {
    label: { kind: "text" },
    provenance: { kind: "text" }
  },
  Combination: {
    label: { kind: "text" },
    provenance: { kind: "text" }
  }
};

const DEFERRED_FIELDS: Array<{ objectType: string; fieldPath: string; reason: string }> = [
  {
    objectType: "Component",
    fieldPath: "kind",
    reason: "component-kind editing is deferred to the component editor scope (completion plan Phase C/D)"
  },
  {
    objectType: "Load",
    fieldPath: "status",
    reason: "load-case status editing is deferred to the load case manager (completion plan A4)"
  },
  {
    objectType: "Load",
    fieldPath: "kind",
    reason: "load-case kind editing is deferred to the load case manager (completion plan A4)"
  },
  {
    objectType: "Combination",
    fieldPath: "basis",
    reason: "combination basis editing is deferred to the load case manager (completion plan A4)"
  },
  {
    objectType: "Combination",
    fieldPath: "terms",
    reason: "combination term editing is deferred to the load case manager (completion plan A4)"
  }
];

const COLLECTIONS: Record<string, string> = {
  Material: "materials",
  Node: "nodes",
  Element: "pipe_segments",
  Support: "supports",
  Component: "components",
  Load: "load_cases",
  Combination: "combinations"
};

function tauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function validateModelOperation(
  model: PreviewModel,
  intent: EditorOperationIntent,
  modelHash: ModelHashEvidence | null
): Promise<OperationOutcome> {
  if (tauriAvailable()) {
    return invoke<OperationOutcome>("validate_model_operation", {
      model,
      intent,
      claimedModelHash: modelHash ?? null
    });
  }
  return runLocalOperation(model, intent, modelHash, "validate_only");
}

export async function applyModelOperation(
  model: PreviewModel,
  intent: EditorOperationIntent,
  modelHash: ModelHashEvidence | null
): Promise<OperationOutcome> {
  if (tauriAvailable()) {
    return invoke<OperationOutcome>("apply_model_operation", {
      model,
      intent,
      claimedModelHash: modelHash ?? null
    });
  }
  return runLocalOperation(model, intent, modelHash, "apply");
}

type LocalChecker = {
  diagnostics: OperationOutcomeDiagnostic[];
  schemaBlocked: boolean;
  referenceState: string;
  unitState: string;
  beforeState: string;
};

function pushDiagnostic(
  checker: LocalChecker,
  code: string,
  severity: string,
  message: string,
  remediation: string,
  affectedRefs: string[]
): void {
  checker.diagnostics.push({
    id: `diagnostic:operation-applier:${code.toLowerCase()}:${String(checker.diagnostics.length + 1).padStart(3, "0")}`,
    code,
    severity,
    message,
    remediation,
    affected_refs: affectedRefs,
    source: ENGINE_SOURCE
  });
}

function hasBlocking(checker: LocalChecker): boolean {
  return checker.diagnostics.some((item) => item.severity === "blocking");
}

async function runLocalOperation(
  model: PreviewModel,
  intent: EditorOperationIntent,
  modelHash: ModelHashEvidence | null,
  mode: "validate_only" | "apply"
): Promise<OperationOutcome> {
  const checker: LocalChecker = {
    diagnostics: [],
    schemaBlocked: false,
    referenceState: "not_run",
    unitState: "not_run",
    beforeState: "not_run"
  };
  const modelDocument = model as unknown as Record<string, unknown>;
  const operationId = intent.operation_id ?? "operation:unknown";
  const changeKind = intent.change?.change_kind ?? "";
  const objectType = intent.target?.object_type ?? "";
  const targetRef = intent.target?.ref ?? "";
  const fieldPath = intent.change?.field_path ?? "";

  checkIntentStructure(intent, operationId, checker);
  checkAuditBoundary(intent, operationId, checker);
  checkKinds(intent.operation_kind ?? "", changeKind, operationId, checker);

  let resolved: ResolvedField | null = null;
  if (!checker.schemaBlocked && ["set_field", "update_load", "update_support"].includes(changeKind)) {
    resolved = resolveField(modelDocument, objectType, targetRef, fieldPath, intent, checker);
  }
  let createdNode: Record<string, unknown> | null = null;
  if (!checker.schemaBlocked && changeKind === "create_node") {
    createdNode = resolveCreateNode(modelDocument, targetRef, intent, checker);
  }
  let createdPipe: Record<string, unknown> | null = null;
  if (!checker.schemaBlocked && changeKind === "connect_pipe_run") {
    createdPipe = resolveConnectPipeRun(modelDocument, targetRef, intent, checker);
  }

  const blocking = hasBlocking(checker);
  const diffPreview: OperationDiffPreviewRow[] =
    resolved && !blocking
      ? [
          {
            entity_ref: targetRef,
            object_type: objectType,
            field_path: fieldPath,
            before: resolved.currentDisplay,
            after: intent.change.after,
            unit: intent.change.unit,
            dimension: intent.change.dimension,
              change_kind: changeKind
            }
          ]
      : (createdNode || createdPipe) && !blocking
        ? [
            {
              entity_ref: targetRef,
              object_type: objectType,
              field_path: fieldPath,
              before: intent.change.before,
              after: intent.change.after,
              unit: intent.change.unit,
              dimension: intent.change.dimension,
              change_kind: changeKind
            }
          ]
      : [];

  let appliedModel: PreviewModel | null = null;
  let appliedModelHash: string | null = null;
  if (mode === "apply" && !blocking && (resolved || createdNode || createdPipe)) {
    const nextModel = JSON.parse(JSON.stringify(modelDocument)) as Record<string, unknown>;
    const applied = resolved
      ? applyResolvedField(nextModel, objectType, targetRef, resolved)
      : createdNode
        ? applyCreatedNode(nextModel, createdNode)
        : createdPipe
          ? applyCreatedPipe(nextModel, createdPipe)
        : false;
    if (applied) {
      appliedModelHash = await localHash(nextModel);
      appliedModel = nextModel as unknown as PreviewModel;
    }
  }

  const applied = appliedModel !== null;
  const blockingAfterApply = hasBlocking(checker);
  const diagnostics = [...checker.diagnostics].sort((left, right) =>
    left.code === right.code ? left.message.localeCompare(right.message) : left.code.localeCompare(right.code)
  );

  return {
    schema_version: OPERATION_APPLIER_VERSION,
    document_kind: OUTCOME_DOCUMENT_KIND,
    deliverable_refs: ["DEL-16-02", "DEL-16-03"],
    mode,
    application_route: "browser_fixture_local_apply",
    operation_id: operationId,
    change_id: intent.change?.change_id ?? "change:unknown",
    operation_kind: intent.operation_kind ?? "",
    change_kind: changeKind,
    target_object_type: objectType,
    target_ref: targetRef,
    validation: {
      schema_validation: checker.schemaBlocked ? "blocked" : "passed",
      reference_validation: checker.referenceState,
      unit_validation: checker.unitState,
      before_state_validation: checker.beforeState,
      diff_preview_status: blockingAfterApply ? "blocked_by_validation" : "generated",
      application_status:
        mode === "validate_only" ? "not_applied" : applied ? "applied_to_session_model" : "blocked"
    },
    diff_preview: diffPreview,
    diagnostics,
    model_basis: {
      claimed_model_hash: modelHash?.value ?? "not_provided",
      claimed_hash_canonicalization: modelHash ? modelHash.canonicalization : "not_provided",
      backend_model_hash: await localHash(modelDocument),
      backend_canonicalization: LOCAL_CANONICALIZATION,
      binding_status: modelHash
        ? "claimed_hash_echoed_cross_canonicalization_equality_not_evaluated"
        : "no_claimed_hash_before_state_check_is_the_staleness_guard"
    },
    input_model_unchanged: true,
    applied_model: appliedModel,
    applied_model_backend_hash: appliedModelHash,
    acceptance: applied
      ? {
          acceptance_basis: "user_initiated_apply_in_local_session",
          acceptance_is_professional_approval: false,
          persistence_status: "session_state_only_not_yet_saved"
        }
      : {
          acceptance_basis: "none_validation_only",
          acceptance_is_professional_approval: false,
          persistence_status: "not_applicable_no_application"
        },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      input_model_mutated_in_place: false,
      applied_model_is_new_document: applied
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    }
  };
}

type ResolvedField = {
  fieldKind: FieldKind;
  currentDisplay: string;
  appliedValue: unknown;
  segments: string[];
};

function checkIntentStructure(intent: EditorOperationIntent, operationId: string, checker: LocalChecker): void {
  const requiredPaths: Array<[string, unknown]> = [
    ["operation_id", intent.operation_id],
    ["operation_kind", intent.operation_kind],
    ["operation_status", intent.operation_status],
    ["target.object_type", intent.target?.object_type],
    ["target.ref", intent.target?.ref],
    ["change.change_id", intent.change?.change_id],
    ["change.change_kind", intent.change?.change_kind],
    ["change.field_path", intent.change?.field_path],
    ["audit_boundary", intent.audit_boundary],
    ["professional_boundary", intent.professional_boundary],
    ["change.before", intent.change?.before],
    ["change.after", intent.change?.after],
    ["change.unit", intent.change?.unit],
    ["change.dimension", intent.change?.dimension]
  ];
  for (const [path, value] of requiredPaths) {
    if (value === undefined || value === null) {
      checker.schemaBlocked = true;
      pushDiagnostic(
        checker,
        "OP-INTENT-FIELD-MISSING",
        "blocking",
        `Operation intent is missing required field \`${path}\`.`,
        "Provide a complete structured editor-operation intent record.",
        [operationId]
      );
    }
  }
}

function checkAuditBoundary(intent: EditorOperationIntent, operationId: string, checker: LocalChecker): void {
  const route = intent.audit_boundary?.mutation_route;
  const direct = intent.audit_boundary?.direct_model_mutation_allowed ?? true;
  if (route !== "structured_operations_only" || direct) {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-DIRECT-MUTATION-BLOCKED",
      "blocking",
      "Operation intent requests a mutation route outside structured operations; direct model mutation is not allowed.",
      "Route all model changes through structured operations with direct_model_mutation_allowed=false.",
      [operationId]
    );
  }
}

function checkKinds(operationKind: string, changeKind: string, operationId: string, checker: LocalChecker): void {
  const supported = [
    "set_field",
    "update_load",
    "update_support",
    "create_node",
    "connect_pipe_run",
    "insert_component_symbol"
  ];
  if (!supported.includes(changeKind)) {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-KIND-UNSUPPORTED",
      "blocking",
      `Change kind \`${changeKind}\` is outside the structured editor-operation taxonomy.`,
      "Use a supported change kind from the structured operation contract.",
      [operationId]
    );
    return;
  }
  const expectedOperationKind = ["set_field", "update_load", "update_support"].includes(changeKind)
    ? "modify"
    : changeKind === "create_node"
      ? "create"
      : changeKind === "connect_pipe_run"
        ? "connect"
        : "insert";
  if (operationKind !== expectedOperationKind) {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-CHANGE-KIND-MISMATCH",
      "blocking",
      `Operation kind \`${operationKind}\` does not match change kind \`${changeKind}\` (expected \`${expectedOperationKind}\`).`,
      "Emit operation and change kinds from the same structured-operation taxonomy row.",
      [operationId]
    );
  }
  if (changeKind === "insert_component_symbol") {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-GEOMETRY-INPUT-INCOMPLETE",
      "blocking",
      `Viewport gesture intent \`${changeKind}\` does not yet carry explicit geometry/connectivity inputs; values are not invented on the user's behalf.`,
      "Capture explicit geometry inputs in the viewport editing tools (completion plan A3) before applying this intent kind.",
      [operationId]
    );
  }
}

function resolveCreateNode(
  model: Record<string, unknown>,
  targetRef: string,
  intent: EditorOperationIntent,
  checker: LocalChecker
): Record<string, unknown> | null {
  if (intent.target.object_type !== "Node" || intent.change.field_path !== "nodes") {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-CREATE-NODE-SHAPE-INVALID",
      "blocking",
      "Create-node intents must target object_type `Node` with field_path `nodes`.",
      "Refresh the viewport create-node intent from the explicit node geometry form.",
      [targetRef]
    );
    return null;
  }
  checkBefore("not_present", intent.change.before, targetRef, intent.change.field_path, checker);
  const storedLengthUnit = valueAtSegments(model, ["project", "units", "length"]);
  checker.unitState = "passed";
  if (typeof storedLengthUnit !== "string") {
    checker.unitState = "blocked";
    pushDiagnostic(
      checker,
      "OP-UNIT-METADATA-MISSING",
      "blocking",
      "Project length unit metadata is missing; explicit node coordinates cannot be accepted.",
      "Repair the model document's project.units.length metadata before creating nodes.",
      [targetRef]
    );
    return null;
  }
  if (intent.change.unit !== storedLengthUnit) {
    checker.unitState = "blocked";
    pushDiagnostic(
      checker,
      "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
      "blocking",
      `Intent unit \`${intent.change.unit}\` does not match project length unit \`${storedLengthUnit}\`; unit conversion is unavailable until the units engine lands.`,
      "Enter node coordinates in the project length unit; no silent conversion is performed.",
      [targetRef]
    );
    return null;
  }
  if (intent.change.dimension !== "length") {
    checker.unitState = "blocked";
    pushDiagnostic(
      checker,
      "OP-UNIT-DIMENSION-UNKNOWN",
      "blocking",
      `Create-node dimension \`${intent.change.dimension}\` must be \`length\`.`,
      "Emit create-node coordinates with explicit length dimension metadata.",
      [targetRef]
    );
    return null;
  }
  if (findEntity(model, "nodes", targetRef)) {
    checker.referenceState = "blocked";
    pushDiagnostic(
      checker,
      "OP-TARGET-ALREADY-EXISTS",
      "blocking",
      `Node \`${targetRef}\` already exists in the current model.`,
      "Choose a new stable node id; create operations never overwrite existing entities.",
      [targetRef]
    );
    return null;
  }
  checker.referenceState = "passed";

  let payload: unknown;
  try {
    payload = JSON.parse(intent.change.after);
  } catch {
    pushDiagnostic(
      checker,
      "OP-CREATE-NODE-PAYLOAD-INVALID",
      "blocking",
      "Create-node payload is not valid JSON.",
      "Emit the explicit node geometry payload as JSON in change.after.",
      [targetRef]
    );
    return null;
  }
  if (typeof payload !== "object" || payload === null || Array.isArray(payload)) {
    pushDiagnostic(
      checker,
      "OP-CREATE-NODE-PAYLOAD-INVALID",
      "blocking",
      "Create-node payload must be a JSON object.",
      "Emit id, label, position, and provenance fields for the new node.",
      [targetRef]
    );
    return null;
  }
  const record = payload as Record<string, unknown>;
  const position = record.position;
  const id = stringPayload(record, "id");
  const label = stringPayload(record, "label");
  const provenance = stringPayload(record, "provenance");
  if (id !== targetRef || !label || !provenance || typeof position !== "object" || position === null || Array.isArray(position)) {
    pushDiagnostic(
      checker,
      "OP-CREATE-NODE-PAYLOAD-INVALID",
      "blocking",
      "Create-node payload must include matching id, non-empty label, non-empty provenance, and position.",
      "Refresh the viewport create-node intent from explicit user-entered node fields.",
      [targetRef]
    );
    return null;
  }
  const positionRecord = position as Record<string, unknown>;
  const x = numericPayload(positionRecord, "x");
  const y = numericPayload(positionRecord, "y");
  const z = numericPayload(positionRecord, "z");
  if (x === null || y === null || z === null) {
    pushDiagnostic(
      checker,
      "OP-CREATE-NODE-PAYLOAD-INVALID",
      "blocking",
      "Create-node position must include finite numeric x, y, and z coordinates.",
      "Enter explicit finite coordinates in the project length unit.",
      [targetRef]
    );
    return null;
  }
  return {
    id,
    label,
    position: { x, y, z },
    provenance
  };
}

function resolveConnectPipeRun(
  model: Record<string, unknown>,
  targetRef: string,
  intent: EditorOperationIntent,
  checker: LocalChecker
): Record<string, unknown> | null {
  if (intent.target.object_type !== "Element" || intent.change.field_path !== "pipe_segments") {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-CONNECT-PIPE-SHAPE-INVALID",
      "blocking",
      "Connect-pipe intents must target object_type `Element` with field_path `pipe_segments`.",
      "Refresh the viewport connect-pipe intent from the explicit pipe connectivity form.",
      [targetRef]
    );
    return null;
  }
  checkBefore("not_present", intent.change.before, targetRef, intent.change.field_path, checker);
  const storedLengthUnit = valueAtSegments(model, ["project", "units", "length"]);
  checker.unitState = "passed";
  if (typeof storedLengthUnit !== "string") {
    checker.unitState = "blocked";
    pushDiagnostic(
      checker,
      "OP-UNIT-METADATA-MISSING",
      "blocking",
      "Project length unit metadata is missing; explicit pipe geometry cannot be accepted.",
      "Repair the model document's project.units.length metadata before creating pipe segments.",
      [targetRef]
    );
    return null;
  }
  if (intent.change.unit !== storedLengthUnit) {
    checker.unitState = "blocked";
    pushDiagnostic(
      checker,
      "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
      "blocking",
      `Intent unit \`${intent.change.unit}\` does not match project length unit \`${storedLengthUnit}\`; unit conversion is unavailable until the units engine lands.`,
      "Enter pipe section dimensions in the project length unit; no silent conversion is performed.",
      [targetRef]
    );
    return null;
  }
  if (intent.change.dimension !== "length") {
    checker.unitState = "blocked";
    pushDiagnostic(
      checker,
      "OP-UNIT-DIMENSION-UNKNOWN",
      "blocking",
      `Connect-pipe dimension \`${intent.change.dimension}\` must be \`length\`.`,
      "Emit pipe section geometry with explicit length dimension metadata.",
      [targetRef]
    );
    return null;
  }
  if (findEntity(model, "pipe_segments", targetRef)) {
    checker.referenceState = "blocked";
    pushDiagnostic(
      checker,
      "OP-TARGET-ALREADY-EXISTS",
      "blocking",
      `Pipe segment \`${targetRef}\` already exists in the current model.`,
      "Choose a new stable pipe id; connect operations never overwrite existing entities.",
      [targetRef]
    );
    return null;
  }

  let payload: unknown;
  try {
    payload = JSON.parse(intent.change.after);
  } catch {
    pushDiagnostic(
      checker,
      "OP-CONNECT-PIPE-PAYLOAD-INVALID",
      "blocking",
      "Connect-pipe payload is not valid JSON.",
      "Emit the explicit pipe connectivity payload as JSON in change.after.",
      [targetRef]
    );
    return null;
  }
  if (typeof payload !== "object" || payload === null || Array.isArray(payload)) {
    pushDiagnostic(
      checker,
      "OP-CONNECT-PIPE-PAYLOAD-INVALID",
      "blocking",
      "Connect-pipe payload must be a JSON object.",
      "Emit id, label, from, to, section, material, y_reference, and provenance fields for the new pipe segment.",
      [targetRef]
    );
    return null;
  }
  const record = payload as Record<string, unknown>;
  const id = stringPayload(record, "id");
  const label = stringPayload(record, "label");
  const from = stringPayload(record, "from");
  const to = stringPayload(record, "to");
  const material = stringPayload(record, "material");
  const provenance = stringPayload(record, "provenance");
  const outsideDiameter = quantityPayload(record, ["section", "outside_diameter"], storedLengthUnit);
  const wallThickness = quantityPayload(record, ["section", "wall_thickness"], storedLengthUnit);
  const yReference = vectorPayload(record, "y_reference");
  if (
    id !== targetRef ||
    !label ||
    !from ||
    !to ||
    from === to ||
    !material ||
    !provenance ||
    outsideDiameter === null ||
    wallThickness === null ||
    yReference === null
  ) {
    pushDiagnostic(
      checker,
      "OP-CONNECT-PIPE-PAYLOAD-INVALID",
      "blocking",
      "Connect-pipe payload must include matching id, non-empty label/from/to/material/provenance, distinct endpoint nodes, positive OD/wall quantities in the project length unit, and a non-zero y_reference vector.",
      "Refresh the viewport connect-pipe intent from explicit user-entered pipe fields.",
      [targetRef]
    );
    return null;
  }
  if (!findEntity(model, "nodes", from) || !findEntity(model, "nodes", to)) {
    checker.referenceState = "blocked";
    pushDiagnostic(
      checker,
      "OP-PIPE-ENDPOINT-NOT-FOUND",
      "blocking",
      `Pipe \`${targetRef}\` references endpoint nodes \`${from}\` and \`${to}\`, but at least one is absent from the current model.`,
      "Create or select existing endpoint nodes before connecting a pipe segment.",
      [targetRef, from, to]
    );
    return null;
  }
  if (!findEntity(model, "materials", material)) {
    checker.referenceState = "blocked";
    pushDiagnostic(
      checker,
      "OP-PIPE-MATERIAL-NOT-FOUND",
      "blocking",
      `Pipe \`${targetRef}\` references material \`${material}\`, which is absent from the current model.`,
      "Select an existing material before connecting a pipe segment.",
      [targetRef, material]
    );
    return null;
  }
  checker.referenceState = "passed";

  return {
    id,
    label,
    from,
    to,
    section: {
      outside_diameter: { value: outsideDiameter, unit: storedLengthUnit },
      wall_thickness: { value: wallThickness, unit: storedLengthUnit }
    },
    material,
    y_reference: yReference,
    provenance
  };
}

function resolveField(
  model: Record<string, unknown>,
  objectType: string,
  targetRef: string,
  fieldPath: string,
  intent: EditorOperationIntent,
  checker: LocalChecker
): ResolvedField | null {
  const collection = COLLECTIONS[objectType];
  if (!collection) {
    checker.schemaBlocked = true;
    pushDiagnostic(
      checker,
      "OP-OBJECT-TYPE-UNSUPPORTED",
      "blocking",
      `Target object type \`${objectType}\` is not an editable model collection.`,
      "Target one of the structured model collections.",
      [targetRef]
    );
    return null;
  }
  const entity = findEntity(model, collection, targetRef);
  if (!entity) {
    checker.referenceState = "blocked";
    pushDiagnostic(
      checker,
      "OP-TARGET-NOT-FOUND",
      "blocking",
      `Target \`${targetRef}\` was not found in model collection \`${collection}\`.`,
      "Refresh the editor intent against the current model document.",
      [targetRef]
    );
    return null;
  }
  const deferred = DEFERRED_FIELDS.find((row) => row.objectType === objectType && row.fieldPath === fieldPath);
  if (deferred) {
    pushDiagnostic(
      checker,
      "OP-FIELD-EDIT-DEFERRED",
      "blocking",
      `Editing \`${objectType}.${fieldPath}\` is not applied in this tranche: ${deferred.reason}.`,
      "Wait for the named completion-plan item; the intent remains reviewable and is not silently applied.",
      [targetRef]
    );
    return null;
  }
  const fieldKind: FieldKind | undefined =
    objectType === "Load" && isPrimitiveMagnitudePath(fieldPath)
      ? { kind: "quantity", requirePositive: false, unitSource: "sibling" }
      : FIELD_RULES[objectType]?.[fieldPath];
  if (!fieldKind) {
    pushDiagnostic(
      checker,
      "OP-FIELD-PATH-UNSUPPORTED",
      "blocking",
      `Field path \`${fieldPath}\` on \`${objectType}\` is not an editable field in the apply-operation seam.`,
      "Edit one of the supported structured fields for this object type.",
      [targetRef]
    );
    return null;
  }

  const segments = fieldPath.split(".");
  const before = intent.change.before;
  const after = intent.change.after;

  if (fieldKind.kind === "text") {
    const current = valueAtSegments(entity, segments);
    if (typeof current !== "string") {
      pushDiagnostic(
        checker,
        "OP-FIELD-NOT-PRESENT",
        "blocking",
        `Field \`${fieldPath}\` is not present as text on \`${targetRef}\`.`,
        "Refresh the editor intent against the current model document.",
        [targetRef]
      );
      return null;
    }
    checkBefore(current, before, targetRef, fieldPath, checker);
    const trimmed = after.trim();
    if (!trimmed) {
      pushDiagnostic(
        checker,
        "OP-VALUE-EMPTY",
        "blocking",
        `Replacement value for \`${fieldPath}\` is empty.`,
        "Provide a non-empty replacement value (or `TBD` to mark an explicit unknown).",
        [targetRef]
      );
      return null;
    }
    return { fieldKind, currentDisplay: current, appliedValue: trimmed, segments };
  }

  if (fieldKind.kind === "quantity") {
    const currentValue = valueAtSegments(entity, segments);
    if (typeof currentValue !== "number") {
      checker.unitState = "blocked";
      pushDiagnostic(
        checker,
        "OP-QUANTITY-OBJECT-MISSING",
        "blocking",
        `Quantity field \`${fieldPath}\` is not present on \`${targetRef}\`; authoring a new quantity requires explicit unit entry (completion plan Phase B, decision D-01).`,
        "Author new quantity fields once unit entry exists; existing quantities remain editable.",
        [targetRef]
      );
      return null;
    }
    const storedUnit =
      fieldKind.unitSource === "sibling"
        ? valueAtSegments(entity, [...segments.slice(0, -1), "unit"])
        : valueAtSegments(model, ["project", "units", fieldKind.unitSource.slice("project_units:".length)]);
    checker.unitState = "passed";
    if (typeof storedUnit !== "string") {
      checker.unitState = "blocked";
      pushDiagnostic(
        checker,
        "OP-UNIT-METADATA-MISSING",
        "blocking",
        `Stored quantity \`${fieldPath}\` on \`${targetRef}\` has no unit metadata.`,
        "Repair the model document's unit metadata before editing this quantity.",
        [targetRef]
      );
      return null;
    }
    if (intent.change.unit !== storedUnit) {
      checker.unitState = "blocked";
      pushDiagnostic(
        checker,
        "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
        "blocking",
        `Intent unit \`${intent.change.unit}\` does not match stored unit \`${storedUnit}\` for \`${fieldPath}\`; unit conversion is unavailable until the units engine lands (completion plan Phase B, decision D-01).`,
        "Enter the value in the stored unit; no silent conversion is performed.",
        [targetRef]
      );
      return null;
    }
    if (!CANONICAL_DIMENSIONS.has(intent.change.dimension)) {
      checker.unitState = "blocked";
      pushDiagnostic(
        checker,
        "OP-UNIT-DIMENSION-UNKNOWN",
        "blocking",
        `Dimension \`${intent.change.dimension}\` is outside the canonical dimension vocabulary.`,
        "Use a canonical dimension token; vocabulary changes await the D-01 unit-catalog ruling.",
        [targetRef]
      );
      return null;
    }
    checkBeforeNumeric(currentValue, before, targetRef, fieldPath, checker);
    const parsed = parseFiniteNumber(after);
    if (parsed === null) {
      pushDiagnostic(
        checker,
        "OP-VALUE-NOT-NUMERIC",
        "blocking",
        `Replacement value \`${after}\` for \`${fieldPath}\` is not a finite number.`,
        "Provide a finite numeric value in the stored unit.",
        [targetRef]
      );
      return null;
    }
    if (fieldKind.requirePositive && parsed <= 0) {
      pushDiagnostic(
        checker,
        "OP-VALUE-NOT-POSITIVE",
        "blocking",
        `Replacement value \`${after}\` for \`${fieldPath}\` must be greater than zero to remain physically meaningful.`,
        "Provide a positive value for geometric and stiffness quantities.",
        [targetRef]
      );
      return null;
    }
    return { fieldKind, currentDisplay: String(currentValue), appliedValue: parsed, segments };
  }

  if (fieldKind.kind === "entity_ref") {
    const current = valueAtSegments(entity, segments);
    if (typeof current !== "string") {
      pushDiagnostic(
        checker,
        "OP-FIELD-NOT-PRESENT",
        "blocking",
        `Reference field \`${fieldPath}\` is not present on \`${targetRef}\`.`,
        "Refresh the editor intent against the current model document.",
        [targetRef]
      );
      return null;
    }
    checkBefore(current, before, targetRef, fieldPath, checker);
    const replacement = after.trim();
    if (!findEntity(model, fieldKind.collection, replacement)) {
      checker.referenceState = "blocked";
      pushDiagnostic(
        checker,
        "OP-REFERENCE-NOT-FOUND",
        "blocking",
        `Replacement reference \`${replacement}\` does not exist in model collection \`${fieldKind.collection}\`.`,
        "Reference an existing entity id; referenced entities are not created implicitly.",
        [targetRef]
      );
      return null;
    }
    if (checker.referenceState === "not_run") checker.referenceState = "passed";
    return { fieldKind, currentDisplay: current, appliedValue: replacement, segments };
  }

  const currentTokens = Array.isArray(valueAtSegments(entity, segments))
    ? ((valueAtSegments(entity, segments) as unknown[]).filter((item) => typeof item === "string") as string[])
    : [];
  const currentDisplay = currentTokens.join(", ");
  checkBefore(currentDisplay, before, targetRef, fieldPath, checker);
  const tokens: string[] = [];
  for (const raw of after.split(",")) {
    const token = raw.trim().toUpperCase();
    if (!token) continue;
    if (!RESTRAINT_TOKENS.includes(token)) {
      pushDiagnostic(
        checker,
        "OP-RESTRAINT-TOKEN-INVALID",
        "blocking",
        `Restraint token \`${token}\` is not in the restraint vocabulary ${RESTRAINT_TOKENS.join("/")}.`,
        "Use comma-separated restraint direction tokens from the model vocabulary.",
        [targetRef]
      );
      return null;
    }
    if (!tokens.includes(token)) tokens.push(token);
  }
  if (tokens.length === 0) {
    pushDiagnostic(
      checker,
      "OP-RESTRAINT-SET-EMPTY",
      "blocking",
      "Replacement restraint set is empty; removing a support entirely is a delete operation, not a field edit.",
      "Provide at least one restraint direction token.",
      [targetRef]
    );
    return null;
  }
  return { fieldKind, currentDisplay, appliedValue: tokens, segments };
}

function checkBefore(current: string, claimedBefore: string, targetRef: string, fieldPath: string, checker: LocalChecker): void {
  if (current === claimedBefore) {
    checker.beforeState = "passed";
  } else {
    checker.beforeState = "blocked_stale";
    pushDiagnostic(
      checker,
      "OP-STALE-BEFORE-VALUE",
      "blocking",
      `Intent before-value \`${claimedBefore}\` no longer matches the current value \`${current}\` of \`${fieldPath}\`.`,
      "Re-queue the edit from the current model state; stale intents are not applied.",
      [targetRef]
    );
  }
}

function checkBeforeNumeric(current: number, claimedBefore: string, targetRef: string, fieldPath: string, checker: LocalChecker): void {
  const claimed = parseFiniteNumber(claimedBefore);
  if (claimed !== null && claimed === current) {
    checker.beforeState = "passed";
  } else {
    checker.beforeState = "blocked_stale";
    pushDiagnostic(
      checker,
      "OP-STALE-BEFORE-VALUE",
      "blocking",
      `Intent before-value \`${claimedBefore}\` no longer matches the current value \`${String(current)}\` of \`${fieldPath}\`.`,
      "Re-queue the edit from the current model state; stale intents are not applied.",
      [targetRef]
    );
  }
}

function applyResolvedField(
  model: Record<string, unknown>,
  objectType: string,
  targetRef: string,
  field: ResolvedField
): boolean {
  const collection = COLLECTIONS[objectType];
  if (!collection) return false;
  const entity = findEntity(model, collection, targetRef);
  if (!entity) return false;
  let slot: unknown = entity;
  for (const segment of field.segments.slice(0, -1)) {
    slot = stepInto(slot, segment);
    if (slot === undefined || slot === null) return false;
  }
  const leaf = field.segments[field.segments.length - 1];
  if (Array.isArray(slot)) {
    const index = Number.parseInt(leaf, 10);
    if (Number.isNaN(index)) return false;
    slot[index] = field.appliedValue;
    return true;
  }
  if (typeof slot === "object" && slot !== null) {
    (slot as Record<string, unknown>)[leaf] = field.appliedValue;
    return true;
  }
  return false;
}

function applyCreatedNode(model: Record<string, unknown>, node: Record<string, unknown>): boolean {
  if (!Array.isArray(model.nodes)) return false;
  model.nodes.push(node);
  return true;
}

function applyCreatedPipe(model: Record<string, unknown>, pipe: Record<string, unknown>): boolean {
  if (!Array.isArray(model.pipe_segments)) return false;
  model.pipe_segments.push(pipe);
  return true;
}

function stepInto(value: unknown, segment: string): unknown {
  if (Array.isArray(value)) {
    const index = Number.parseInt(segment, 10);
    return Number.isNaN(index) ? undefined : value[index];
  }
  if (typeof value === "object" && value !== null) {
    return (value as Record<string, unknown>)[segment];
  }
  return undefined;
}

function stringPayload(record: Record<string, unknown>, key: string): string {
  const value = record[key];
  return typeof value === "string" ? value.trim() : "";
}

function numericPayload(record: Record<string, unknown>, key: string): number | null {
  const value = record[key];
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

function quantityPayload(record: Record<string, unknown>, path: string[], expectedUnit: string): number | null {
  const quantity = valueAtSegments(record, path);
  if (typeof quantity !== "object" || quantity === null || Array.isArray(quantity)) return null;
  const quantityRecord = quantity as Record<string, unknown>;
  const value = numericPayload(quantityRecord, "value");
  const unit = stringPayload(quantityRecord, "unit");
  if (value === null || value <= 0 || unit !== expectedUnit) return null;
  return value;
}

function vectorPayload(record: Record<string, unknown>, key: string): { x: number; y: number; z: number } | null {
  const vector = record[key];
  if (typeof vector !== "object" || vector === null || Array.isArray(vector)) return null;
  const vectorRecord = vector as Record<string, unknown>;
  const x = numericPayload(vectorRecord, "x");
  const y = numericPayload(vectorRecord, "y");
  const z = numericPayload(vectorRecord, "z");
  if (x === null || y === null || z === null) return null;
  if (x === 0 && y === 0 && z === 0) return null;
  return { x, y, z };
}

function isPrimitiveMagnitudePath(fieldPath: string): boolean {
  return /^primitive_loads\.\d+\.magnitude\.value$/.test(fieldPath);
}

function findEntity(model: Record<string, unknown>, collection: string, entityRef: string): Record<string, unknown> | null {
  const rows = model[collection];
  if (!Array.isArray(rows)) return null;
  const match = rows.find(
    (item) => typeof item === "object" && item !== null && (item as Record<string, unknown>).id === entityRef
  );
  return (match as Record<string, unknown>) ?? null;
}

function valueAtSegments(value: unknown, segments: string[]): unknown {
  let current = value;
  for (const segment of segments) {
    current = stepInto(current, segment);
    if (current === undefined) return undefined;
  }
  return current;
}

// Match the Rust crate's f64 parse acceptance: plain decimal/scientific
// notation only (JS Number() would also accept hex and empty strings).
function parseFiniteNumber(raw: string): number | null {
  const trimmed = raw.trim();
  if (!/^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/.test(trimmed)) return null;
  const parsed = Number.parseFloat(trimmed);
  return Number.isFinite(parsed) ? parsed : null;
}

async function localHash(payload: unknown): Promise<string> {
  const canonical = canonicalJson(payload);
  if (globalThis.crypto?.subtle) {
    const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(canonical));
    const hex = Array.from(new Uint8Array(digest))
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
    return `sha256:${hex}`;
  }
  return "sha256_unavailable_no_subtle_crypto_in_this_environment";
}
