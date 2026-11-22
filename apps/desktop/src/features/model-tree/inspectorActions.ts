// What the inspector offers for one selected entity: which action lines apply to it, chosen
// on what a person can see. Any refusal is the operation engine's, shown by its own code.
import type { EntityRef, PreviewModel } from "../../types";
import { authoredPressureElongationCases, authoredShearModel, effectiveNodeProximityTolerance, pressurisedCasesNotIncluded } from "../model-settings/modelSettingsIntents";
import { documentShearModel, SHEAR_MODEL_CHOICES } from "../pressure-authoring/pressureContract";
import { SHEAR_MODEL_UNSTATED } from "../model-settings/ModelSettingsSection";
import { PROJECT_UNIT_KEYS } from "../../services/projectService";
import { storedProjectUnit } from "../project-units/projectUnitIntents";

/** Below this turn, in radians, two pipes run straight through their node beyond any doubt. This is
 * not the operation engine's rule: it is three orders of magnitude inside the engine's own lower bound
 * (alpha_tan, 1e-3 rad), so every corner the engine can accept is offered. */
const STRAIGHT_THROUGH_RAD = 1.0e-6;

type Vec3 = [number, number, number];
const position = (model: PreviewModel, id: string): Vec3 | null => {
  const p = model.nodes.find((node) => node.id === id)?.position;
  return p && [p.x, p.y, p.z].every((value) => typeof value === "number" && Number.isFinite(value)) ? [p.x, p.y, p.z] : null;
};

/**
 * Whether "Bend at this corner" is offered on a node: where it joins exactly two pipes that turn
 * there, the topology and shape a person can see. The line is hidden only where the two pipes run
 * straight through the node beyond any doubt (a turn below STRAIGHT_THROUGH_RAD, from the node
 * positions the model holds), with no reason given: there is no corner. Every other two-pipe node
 * is offered, including a turn between that and the engine's bound, or one near a half circle; the
 * inspector refuses nothing itself, and the operation engine refuses on Queue by its own code
 * (operation_applier bend_authoring.rs `insert_at_corner`).
 */
export function offersBendAtCorner(model: PreviewModel, nodeId: string): boolean {
  const pipes = model.pipe_segments.filter((pipe) => pipe.from === nodeId || pipe.to === nodeId);
  if (pipes.length !== 2) return false;
  const corner = position(model, nodeId);
  const far = pipes.map((pipe) => position(model, pipe.from === nodeId ? pipe.to : pipe.from));
  if (!corner || !far[0] || !far[1]) return true;
  const incoming: Vec3 = [corner[0] - far[0][0], corner[1] - far[0][1], corner[2] - far[0][2]];
  const outgoing: Vec3 = [far[1][0] - corner[0], far[1][1] - corner[1], far[1][2] - corner[2]];
  const length = (v: Vec3) => Math.hypot(v[0], v[1], v[2]);
  if (!(length(incoming) > 0) || !(length(outgoing) > 0)) return true;
  const a = incoming.map((value) => value / length(incoming)) as Vec3;
  const t = outgoing.map((value) => value / length(outgoing)) as Vec3;
  // The turn between the two directions, 2 atan2(|a - t|, |a + t|), as the engine measures it.
  const turn = 2 * Math.atan2(length([a[0] - t[0], a[1] - t[1], a[2] - t[2]]), length([a[0] + t[0], a[1] + t[1], a[2] + t[2]]));
  return !(turn < STRAIGHT_THROUGH_RAD);
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
  const labelFor = (value: string) => SHEAR_MODEL_CHOICES.find((choice) => choice.value === value)?.label ?? value;
  const profileShear = documentShearModel(model);
  const shearLabel = exact
    ? `${profileShear ? labelFor(profileShear) : "not stated"}, set by the pressure profile`
    : shear === null
      ? SHEAR_MODEL_UNSTATED
      : labelFor(shear);
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

/** "Project units" with the stored symbols, for its closed line. A key the document lacks is "not set";
 * one it holds as a non-string or a blank string is "not a unit symbol", as the section itself reads it. */
export function projectUnitValues(model: PreviewModel): string {
  return PROJECT_UNIT_KEYS.map((key) => {
    const stored = storedProjectUnit(model, key);
    if (stored === undefined) return `${key} not set`;
    return typeof stored === "string" && stored.trim() ? stored.trim() : `${key} not a unit symbol`;
  }).join(", ");
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
