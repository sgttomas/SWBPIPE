// Element axis triads for the viewport (A7; FR-SOL-007, DEL-07-05). The local frame is the
// one the solver forms for a straight member (`FrameOrientation::from_x_axis_and_y_reference`
// in `core/solver/frame_kernel/src/lib.rs`, used through `FrameElement::orientation`):
//
//   x = (to - from) / |to - from|                  (node i is the pipe's `from`)
//   y = (y_ref - (y_ref . x) x) / |...|            (the pipe's `y_reference`)
//   z = x cross y
//
// each normalization refused at a magnitude of 1e-12 or less, the kernel's tolerance (the
// kernel measures the member axis in metres, so in another length unit the zero-length
// refusal here differs from it only at that scale). No
// default orientation is applied: a pipe without a `y_reference` has no local frame here, as
// the solver blocks it. A span realized as a curved bend publishes its end rows in end-tangent
// frames, not in a straight member frame, so no straight-member triad is drawn for it. The
// global frame is the model's X, Y and Z. This is a display of the convention only: it forms no
// result and transforms no result value.

import type { PreviewComponent, PreviewModel, Vec3 } from "../../types";

export type ElementAxesFrame = "local" | "global";

export type ElementAxisTriad = Readonly<{
  pipeId: string;
  /** The member's midpoint in authored coordinates. */
  origin: Vec3;
  x: Vec3;
  y: Vec3;
  z: Vec3;
  /** The member's length in model length units. */
  length: number;
}>;

export type ElementAxesView = Readonly<{
  frame: ElementAxesFrame;
  triads: ReadonlyMap<string, ElementAxisTriad>;
  /** Members with no triad, and why. */
  withheld: ReadonlyMap<string, string>;
}>;

/** The kernel's `AXIS_TOLERANCE`. */
const AXIS_TOLERANCE = 1e-12;

export const REALIZED_CURVED_BEND = "curved_bend_macro_element";

export const WITHHELD_REASONS = Object.freeze({
  noYReference: "no y_reference entered; no default orientation is applied",
  parallelYReference: "y_reference is parallel to the member axis",
  zeroLength: "the member has zero length",
  missingNode: "an end node is missing",
  curvedBend: "realized curved bend: its end rows are in end-tangent frames"
});

/** Pipes whose span the solver replaces with a curved-bend macro-element. */
export function realizedCurvedBendPipeIds(components: readonly PreviewComponent[]): ReadonlySet<string> {
  return new Set(components.flatMap((component) => {
    const bend = component.kind === "bend" || component.kind === "elbow";
    const realized = component.mechanics_interface?.solver_consumption === REALIZED_CURVED_BEND;
    const pipe = component.geometry?.bend_pipe_ref?.trim();
    return bend && realized && pipe ? [pipe] : [];
  }));
}

/** The kernel's local frame of a straight member, or the reason it has none. */
export function straightMemberLocalAxes(from: Vec3, to: Vec3, yReference: Vec3 | undefined):
  { x: Vec3; y: Vec3; z: Vec3 } | { reason: string } {
  const x = normalize(subtract(to, from));
  if (!x) return { reason: WITHHELD_REASONS.zeroLength };
  if (!yReference) return { reason: WITHHELD_REASONS.noYReference };
  const y = normalize(subtract(yReference, scale(x, dot(yReference, x))));
  if (!y) return { reason: WITHHELD_REASONS.parallelYReference };
  const z = normalize(cross(x, y));
  if (!z) return { reason: WITHHELD_REASONS.parallelYReference };
  return { x, y, z };
}

const GLOBAL_AXES = Object.freeze({
  x: Object.freeze({ x: 1, y: 0, z: 0 }),
  y: Object.freeze({ x: 0, y: 1, z: 0 }),
  z: Object.freeze({ x: 0, y: 0, z: 1 })
});

export function buildElementAxesView(model: PreviewModel, frame: ElementAxesFrame): ElementAxesView {
  const nodes = new Map(model.nodes.map((node) => [node.id, node.position] as const));
  const curved = realizedCurvedBendPipeIds(model.components ?? []);
  const triads = new Map<string, ElementAxisTriad>();
  const withheld = new Map<string, string>();
  for (const pipe of model.pipe_segments) {
    const from = nodes.get(pipe.from);
    const to = nodes.get(pipe.to);
    if (!from || !to) { withheld.set(pipe.id, WITHHELD_REASONS.missingNode); continue; }
    const length = Math.hypot(to.x - from.x, to.y - from.y, to.z - from.z);
    if (!(length > AXIS_TOLERANCE) || !Number.isFinite(length)) { withheld.set(pipe.id, WITHHELD_REASONS.zeroLength); continue; }
    const origin = { x: (from.x + to.x) / 2, y: (from.y + to.y) / 2, z: (from.z + to.z) / 2 };
    if (frame === "global") {
      triads.set(pipe.id, { pipeId: pipe.id, origin, ...GLOBAL_AXES, length });
      continue;
    }
    if (curved.has(pipe.id)) { withheld.set(pipe.id, WITHHELD_REASONS.curvedBend); continue; }
    const axes = straightMemberLocalAxes(from, to, pipe.y_reference);
    if ("reason" in axes) { withheld.set(pipe.id, axes.reason); continue; }
    triads.set(pipe.id, { pipeId: pipe.id, origin, ...axes, length });
  }
  return { frame, triads, withheld };
}

function subtract(a: Vec3, b: Vec3): Vec3 {
  return { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z };
}

function scale(a: Vec3, factor: number): Vec3 {
  return { x: a.x * factor, y: a.y * factor, z: a.z * factor };
}

function dot(a: Vec3, b: Vec3): number {
  return a.x * b.x + a.y * b.y + a.z * b.z;
}

function cross(a: Vec3, b: Vec3): Vec3 {
  return { x: a.y * b.z - a.z * b.y, y: a.z * b.x - a.x * b.z, z: a.x * b.y - a.y * b.x };
}

function normalize(a: Vec3): Vec3 | null {
  const magnitude = Math.hypot(a.x, a.y, a.z);
  if (!Number.isFinite(magnitude) || magnitude <= AXIS_TOLERANCE) return null;
  return { x: a.x / magnitude, y: a.y / magnitude, z: a.z / magnitude };
}
