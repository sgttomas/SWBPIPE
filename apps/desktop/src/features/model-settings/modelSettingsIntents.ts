// Model settings (DEL-02-01 Design model-document-0.7.0.md §3.8; DEL-04-06
// Design model-topology-checks.md): the node-proximity tolerance of the
// near-duplicate node warning. The owner's ruling fixes 5 mm "and give the user
// the means to change it". The desktop never writes the default: an absent
// setting shows "5 mm (default)", and Reset to default removes the member.
//
// Wire (core/model_operations/operation_applier/src/model_settings_authoring.rs):
// the Project singleton target (ref = project.id), field_path
// `model_settings.node_proximity_tolerance`; `before` is the current member's
// canonical JSON or `not_present`. Set: `set_model_setting`, `after` the
// entered number text, `unit` its length unit, dimension `length`. Clear:
// `clear_model_setting`, `after` `not_present`, unit `none`, dimension
// `dimensionless`.
import type { EditorOperationIntent, PreviewModel, QuantityValue } from "../../types";

export const NODE_PROXIMITY_TOLERANCE_PATH = "model_settings.node_proximity_tolerance";
/** Model document 0.8.0: the pressure-free document's beam theory; absent means Euler–Bernoulli. */
export const SHEAR_MODEL_PATH = "model_settings.shear_model";
export type SettingPath = typeof NODE_PROXIMITY_TOLERANCE_PATH | typeof SHEAR_MODEL_PATH;
/** The default the product applies when the setting is absent (never written). */
export const DEFAULT_NODE_PROXIMITY_TOLERANCE: Readonly<QuantityValue> = Object.freeze({ value: 5, unit: "mm" });

export type Canonicalize = (value: unknown) => Promise<string>;

/** The authored tolerance, or null when the default applies. */
export function authoredNodeProximityTolerance(model: PreviewModel): QuantityValue | null {
  return model.model_settings?.node_proximity_tolerance ?? null;
}

/** The effective tolerance as the control shows it: "5 mm (default)" or the authored value with its unit. */
export function effectiveNodeProximityTolerance(model: PreviewModel): string {
  const authored = authoredNodeProximityTolerance(model);
  if (!authored) return `${DEFAULT_NODE_PROXIMITY_TOLERANCE.value} ${DEFAULT_NODE_PROXIMITY_TOLERANCE.unit} (default)`;
  return `${authored.value} ${authored.unit}`;
}

function intent(
  model: PreviewModel,
  changeKind: "set_model_setting" | "clear_model_setting",
  before: string,
  after: string,
  unit: string,
  dimension: string,
  label: string,
  path: SettingPath = NODE_PROXIMITY_TOLERANCE_PATH
): EditorOperationIntent {
  const token = `${changeKind}:${path}`;
  return {
    operation_id: `op:model-settings:${token}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-settings",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: { object_type: "Project", ref: model.project.id },
    change: {
      change_id: `change:model-settings:${token}`,
      change_kind: changeKind,
      field_label: label,
      field_path: path,
      before,
      after,
      unit,
      dimension,
      source_note: path === SHEAR_MODEL_PATH
        ? "User-selected beam theory of a pressure-free document; the operation engine judges it."
        : "User-entered node-proximity tolerance for the near-duplicate node warning; it changes no solved value."
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: changeKind === "set_model_setting" ? "not_run" : "not_required_dimensionless",
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
    rationale: `${label}; requires validation and review before application.`
  };
}

async function currentBefore(canonicalize: Canonicalize, model: PreviewModel): Promise<string> {
  const authored = authoredNodeProximityTolerance(model);
  return authored ? canonicalize(authored) : "not_present";
}

/** Set the tolerance to the entered value (number text as typed) in `unit`. */
export async function setNodeProximityToleranceIntent(
  canonicalize: Canonicalize,
  model: PreviewModel,
  valueText: string,
  unit: string
): Promise<EditorOperationIntent> {
  const before = await currentBefore(canonicalize, model);
  return intent(model, "set_model_setting", before, valueText.trim(), unit, "length", "Set node-proximity tolerance");
}

/** Reset the tolerance to its default: the member is removed. */
export async function clearNodeProximityToleranceIntent(canonicalize: Canonicalize, model: PreviewModel): Promise<EditorOperationIntent> {
  const before = await currentBefore(canonicalize, model);
  return intent(model, "clear_model_setting", before, "not_present", "none", "dimensionless", "Reset node-proximity tolerance to default");
}

/** The authored shear model, or null when Euler–Bernoulli applies by default. */
export function authoredShearModel(model: PreviewModel): string | null {
  return model.model_settings?.shear_model ?? null;
}

async function shearBefore(canonicalize: Canonicalize, model: PreviewModel): Promise<string> {
  const authored = authoredShearModel(model);
  return authored === null ? "not_present" : canonicalize(authored);
}

/** Set the shear model to `name` (the engine refuses by name: an unknown name, a document with a
 * pressure contract and, in 0.8.0, `timoshenko_annulus_energy_v1`). */
export async function setShearModelIntent(canonicalize: Canonicalize, model: PreviewModel, name: string): Promise<EditorOperationIntent> {
  return intent(model, "set_model_setting", await shearBefore(canonicalize, model), name, "none", "dimensionless", "Set shear model", SHEAR_MODEL_PATH);
}

/** Clear the shear model, so Euler–Bernoulli applies: the member is removed. */
export async function clearShearModelIntent(canonicalize: Canonicalize, model: PreviewModel): Promise<EditorOperationIntent> {
  return intent(model, "clear_model_setting", await shearBefore(canonicalize, model), "not_present", "none", "dimensionless", "Clear shear model", SHEAR_MODEL_PATH);
}
