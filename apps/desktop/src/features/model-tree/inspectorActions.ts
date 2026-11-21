// What the inspector offers for one selected entity: which action lines apply to it, chosen
// on what a person can see. Any refusal is the operation engine's, shown by its own code.
import type { EntityRef, PreviewModel } from "../../types";
import { authoredShearModel, effectiveNodeProximityTolerance } from "../model-settings/modelSettingsIntents";
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

/** "Model settings" with its current values, for its closed line. */
export function modelSettingsValues(model: PreviewModel): string {
  const shear = authoredShearModel(model);
  const shearLabel = model.pressure_contract != null
    ? "set by the pressure profile"
    : shear === null
      ? "Euler–Bernoulli (default)"
      : SHEAR_MODEL_CHOICES.find((choice) => choice.value === shear)?.label.replace(" (the default)", "") ?? shear;
  return `Shear deformation: ${shearLabel} · Node proximity: ${effectiveNodeProximityTolerance(model)}`;
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
