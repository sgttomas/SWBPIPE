// What the inspector offers for one selected entity: which action lines apply to it, chosen
// on what a person can see. Any refusal is the operation engine's, shown by its own code.
import type { EntityRef, PreviewModel } from "../../types";
import { authoredPressureElongationCases, authoredShearModel, effectiveNodeProximityTolerance, pressurisedCasesNotIncluded } from "../model-settings/modelSettingsIntents";
import { documentShearModel, SHEAR_MODEL_CHOICES as PROFILE_SHEAR_CHOICES } from "../pressure-authoring/pressureContract";
import { SHEAR_MODEL_CHOICES } from "../model-settings/ModelSettingsSection";
import { PROJECT_UNIT_KEYS } from "../../services/projectService";

/**
 * Whether "Bend at this corner" is offered on a node: where it joins exactly two pipes, the
 * topology a person can see. The inspector refuses nothing itself; whether the corner can
 * take a bend is the operation engine's to say, by its own code, when the form is queued
 * (operation_applier bend_authoring.rs `insert_at_corner`).
 */
export function offersBendAtCorner(model: PreviewModel, nodeId: string): boolean {
  return model.pipe_segments.filter((pipe) => pipe.from === nodeId || pipe.to === nodeId).length === 2;
}

/** The selection kinds the support form acts on (capabilityCatalog's support context). */
export function offersSupportForm(selection: EntityRef): boolean {
  return selection.type === "support" || selection.type === "node" || selection.type === "project";
}

/** The support form's line: what it does to this entity. */
export function supportFormLine(selection: EntityRef): string {
  if (selection.type === "support") return "Edit this support";
  if (selection.type === "node") return "Add a support here";
  return "Add a support";
}

/** "Model settings" with its current values, for its closed line. On a pressure-profile document it
 * names the profile's theory and says beside the setting whether pressure elongation is included
 * (DEL-02-01: a document without it "solves without pressure elongation, and the desktop says so"). */
export function modelSettingsValues(model: PreviewModel): string {
  const exact = model.pressure_contract != null;
  const shear = authoredShearModel(model);
  const profileShear = PROFILE_SHEAR_CHOICES.find((choice) => choice.value === documentShearModel(model))?.label;
  const shearLabel = exact
    ? `${profileShear ?? "not stated"}, set by the pressure profile`
    : shear === null
      ? "Euler–Bernoulli (default)"
      : SHEAR_MODEL_CHOICES.find((choice) => choice.value === shear)?.label.replace(" (the default)", "") ?? shear;
  const parts = [`Shear deformation: ${shearLabel}`, `Node proximity: ${effectiveNodeProximityTolerance(model)}`];
  if (exact) {
    const included = authoredPressureElongationCases(model);
    const name = (loadCase: PreviewModel["load_cases"][number]) => loadCase.label || loadCase.id;
    if (included === null) parts.push("Pressure elongation: off (default), not included in any load case");
    else {
      parts.push(`Pressure elongation: ${included.length ? included.map((id) => name(model.load_cases.find((loadCase) => loadCase.id === id) ?? { id, label: "" } as PreviewModel["load_cases"][number])).join(", ") : "no operating case"}`);
      const missing = pressurisedCasesNotIncluded(model);
      if (missing.length) parts.push(`not included: ${missing.map(name).join(", ")}`);
    }
  }
  return parts.join(" · ");
}

/** "Project units" with the stored symbols, for its closed line. */
export function projectUnitValues(model: PreviewModel): string {
  const units = (model.project.units ?? {}) as Record<string, unknown>;
  return PROJECT_UNIT_KEYS.map((key) => (typeof units[key] === "string" && units[key] ? units[key] : `${key} not set`)).join(", ");
}

/** The inspector's names for entity kinds, singular and plural. */
export const KIND_NAMES: Readonly<Record<EntityRef["type"], [string, string]>> = {
  project: ["project", "projects"],
  material: ["material", "materials"],
  section: ["section", "sections"],
  node: ["node", "nodes"],
  pipe: ["pipe", "pipes"],
  support: ["support", "supports"],
  component: ["component", "components"],
  load: ["load case", "load cases"],
  combination: ["combination", "combinations"],
  diagnostic: ["diagnostic", "diagnostics"]
} as Readonly<Record<EntityRef["type"], [string, string]>>;

export function kindName(type: EntityRef["type"], count = 1): string {
  const names = KIND_NAMES[type] ?? [type, `${type}s`];
  return count === 1 ? names[0] : names[1];
}

/** A Delete control's line: what the press does today, what goes with the entity (the operation
 * applier's own consequences), and its refusal, which the engine gives with the dependent records. */
export const deleteLine = (consequence: string) =>
  `Queued in Review changes; nothing is deleted until you apply it there. ${consequence} Refused while another record depends on it; the refusal names which.`;
