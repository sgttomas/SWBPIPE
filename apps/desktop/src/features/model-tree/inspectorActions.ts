// What the inspector offers for one selected entity: which action forms apply to it,
// and, for those the operation engine would refuse whatever is entered, the reason in
// a few words. Each rule is the engine's own (named at each function); none is new.
import type { EntityRef, PreviewModel } from "../../types";
import { authoredShearModel, effectiveNodeProximityTolerance } from "../model-settings/modelSettingsIntents";
import { SHEAR_MODEL_CHOICES } from "../model-settings/ModelSettingsSection";
import { PROJECT_UNIT_KEYS } from "../../services/projectService";

type Vec3 = [number, number, number];

/** The engine's tangency tolerance alpha_tan (caepipe_mbf_engine runtime.rs `TANGENCY_TOLERANCE_RAD`). */
const TANGENCY_TOLERANCE_RAD = 1.0e-3;
/** The engine's upper bound on a corner's direction change: below pi (bend_authoring.rs). */
const REVERSAL_MARGIN_RAD = 1.0e-9;

const sub = (a: Vec3, b: Vec3): Vec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const add = (a: Vec3, b: Vec3): Vec3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const norm = (a: Vec3) => Math.hypot(a[0], a[1], a[2]);

function position(model: PreviewModel, id: string): Vec3 | null {
  const node = model.nodes.find((candidate) => candidate.id === id);
  const p = node?.position;
  if (!p || ![p.x, p.y, p.z].every((value) => typeof value === "number" && Number.isFinite(value))) return null;
  return [p.x, p.y, p.z];
}

function unit(a: Vec3): Vec3 | null {
  const length = norm(a);
  return length > 0 && Number.isFinite(length) ? [a[0] / length, a[1] / length, a[2] / length] : null;
}

/** The runtime's `kink`: 2 atan2(|a - t|, |a + t|) of the unit directions. */
function kink(a: Vec3, t: Vec3): number {
  return 2 * Math.atan2(norm(sub(a, t)), norm(add(a, t)));
}

const isBend = (kind: string | undefined) => kind === "bend" || kind === "elbow";
const text = (value: unknown) => (typeof value === "string" ? value.trim() : "");
const same = (a: unknown, b: unknown) => JSON.stringify(a ?? null) === JSON.stringify(b ?? null);

export type CornerOffer = { offered: false } | { offered: true; refusal: string | null };

/**
 * Insert bend at corner (operation_applier bend_authoring.rs `insert_bend_at_corner`).
 * Offered on a node that joins exactly two pipes whose directions there change by more
 * than alpha_tan and less than pi: a corner. Elsewhere the engine refuses it whatever
 * is entered, so it is not offered. On a corner, the refusals that no entry can avoid
 * are stated: a pipe that is already a bend's arc, pipes that differ in section,
 * material or condition set, and supports or other components on the corner. The
 * engine's own check of every other attachment stays its refusal when queued.
 */
export function bendAtCornerOffer(model: PreviewModel, nodeId: string): CornerOffer {
  const pipes = model.pipe_segments.filter((pipe) => text(pipe.from) && text(pipe.to) && (pipe.from === nodeId || pipe.to === nodeId));
  if (pipes.length !== 2) return { offered: false };
  const corner = position(model, nodeId);
  const far = (pipe: (typeof pipes)[number]) => position(model, pipe.from === nodeId ? pipe.to : pipe.from);
  const [farIn, farOut] = [far(pipes[0]), far(pipes[1])];
  if (!corner || !farIn || !farOut) return { offered: false };
  const [uIn, uOut] = [unit(sub(corner, farIn)), unit(sub(farOut, corner))];
  if (!uIn || !uOut) return { offered: false };
  const phi = kink(uIn, uOut);
  if (!(phi > TANGENCY_TOLERANCE_RAD && phi < Math.PI - REVERSAL_MARGIN_RAD)) return { offered: false };

  const [pIn, pOut] = pipes;
  const arc = model.components.find((component) => isBend(component.kind) && component.node !== nodeId &&
    [pIn.id, pOut.id].includes(text(component.geometry?.bend_pipe_ref)));
  if (arc) return { offered: true, refusal: `${arc.geometry?.bend_pipe_ref} is already the arc of ${arc.id}.` };
  if (!same(pIn.section, pOut.section) || !same(pIn.material, pOut.material) || !same(pIn.section_ref, pOut.section_ref)) {
    return { offered: true, refusal: `${pIn.id} and ${pOut.id} differ in section or material.` };
  }
  if (!same(pIn.condition_set_ref, pOut.condition_set_ref)) {
    return { offered: true, refusal: `${pIn.id} and ${pOut.id} carry different condition sets.` };
  }
  const attached = [
    ...model.supports.filter((support) => support.node === nodeId).map((support) => support.id),
    ...model.components.filter((component) => component.node === nodeId && !isBend(component.kind)).map((component) => component.id)
  ];
  if (attached.length) return { offered: true, refusal: `Remove ${attached.join(", ")} from this corner first.` };
  return { offered: true, refusal: null };
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
