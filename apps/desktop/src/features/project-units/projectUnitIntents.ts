// W-17 slice (b): editing the project units after creation (DEL-02-02 Design
// `project-unit-selection.md` §2.5, §2.7, §3 "A Project units section").
//
// Wire (core/model_operations/operation_applier/src/project_units_authoring.rs):
// operation kind `modify`, change kind `set_project_unit`, the Project
// singleton target (ref = project.id), field_path `project.units.<key>`;
// `before` is the stored symbol (a non-string member as its canonical JSON) or
// `not_present`; `after` and `unit` are the chosen catalog symbol; `dimension`
// is the key. There is no remove form.
import { PROJECT_UNIT_KEYS, type ProjectUnitKey } from "../../services/projectService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { PROJECT_UNIT_LABELS } from "./projectUnitSelection";

export const SET_PROJECT_UNIT = "set_project_unit";

export type Canonicalize = (value: unknown) => Promise<string>;

export function projectUnitFieldPath(key: ProjectUnitKey): string {
  return `project.units.${key}`;
}

/** The stored value of a key, read without assuming its type (an opened document may hold anything). */
export function storedProjectUnit(model: PreviewModel, key: ProjectUnitKey): unknown {
  const units = model.project.units as Record<string, unknown> | undefined;
  return units && typeof units === "object" && Object.hasOwn(units, key) ? units[key] : undefined;
}

/** The keys the document lacks, in the order of the seven keys. */
export function missingProjectUnitKeys(model: PreviewModel): ProjectUnitKey[] {
  return PROJECT_UNIT_KEYS.filter((key) => storedProjectUnit(model, key) === undefined);
}

/** The wire's `before` for a key: the stored symbol, its canonical JSON when not a string, or `not_present`. */
export async function projectUnitBefore(canonicalize: Canonicalize, model: PreviewModel, key: ProjectUnitKey): Promise<string> {
  const stored = storedProjectUnit(model, key);
  if (stored === undefined) return "not_present";
  return typeof stored === "string" ? stored : canonicalize(stored);
}

/** One `set_project_unit` change of `key` to `symbol`, prepared from the current model. */
export async function setProjectUnitIntent(
  canonicalize: Canonicalize,
  model: PreviewModel,
  key: ProjectUnitKey,
  symbol: string
): Promise<EditorOperationIntent> {
  const before = await projectUnitBefore(canonicalize, model, key);
  const fieldPath = projectUnitFieldPath(key);
  const label = `Set project ${PROJECT_UNIT_LABELS[key].toLowerCase()} unit`;
  const token = `${SET_PROJECT_UNIT}:${fieldPath}`;
  return {
    operation_id: `op:project-units:${token}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/project-units",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: { object_type: "Project", ref: model.project.id },
    change: {
      change_id: `change:project-units:${token}`,
      change_kind: SET_PROJECT_UNIT,
      field_label: label,
      field_path: fieldPath,
      before,
      after: symbol,
      unit: symbol,
      dimension: key,
      source_note: "User-chosen project unit; no stored value is converted, and existing quantities keep their entered units."
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
    rationale: `${label}; requires validation and review before application.`
  };
}
