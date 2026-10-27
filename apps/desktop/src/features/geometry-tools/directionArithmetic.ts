// B5 (DEL-07-01 Design `split-and-slope-entry.md` §3): an end point from a length and a
// direction, a length and an angle from the horizontal, or a "1 in n" slope. It is an entry aid:
// it computes the node coordinates the existing operations take (create_node, reconnect to a new
// point) and nothing else. The slope, angle and direction are never written to the model.
//
// The arithmetic is binary64, round to nearest, in the order written below (§3.2):
//   Ls     = convert_for_dimension(L, entry unit -> project length unit)    one rounding at most
//   |d|    = sqrt(dx*dx + dy*dy + dz*dz)        left to right; sqrt is correctly rounded
//   u_i    = d_i / |d|
//   vector: Δ_i = Ls * u_i
//   angle:  θ = convert_for_dimension(θ, entry unit -> rad); Δ_i = (Ls * cos θ) * h_i on the
//           horizontal axes, Δ_up = s_up * (Ls * sin θ)
//   slope:  Δ_i = Ls * h_i on the horizontal axes, Δ_up = s_up * s_sense * (Ls / n)
//   end_i  = start_i + Δ_i, then −0 becomes +0
// s_up and s_sense are ±1, so multiplying by them is exact. No value is rounded to a display
// precision. cos and sin are the runtime's Math.cos and Math.sin (ECMAScript leaves their last
// bit to the implementation); the value computed is the value shown and the value emitted.
import { convertDisplayQuantities } from "../../services/displayQuantityService";

export type Vec3 = { x: number; y: number; z: number };
export type Axis = "x" | "y" | "z";
export const AXES: readonly Axis[] = ["x", "y", "z"];
/** The global direction that is up. There is no default: the model has no vertical convention. */
export type UpDirection = "" | "+x" | "-x" | "+y" | "-y" | "+z" | "-z";
export const UP_DIRECTIONS: readonly Exclude<UpDirection, "">[] = ["+x", "-x", "+y", "-y", "+z", "-z"];
export type DirectionMode = "" | "vector" | "angle" | "slope";
export const DIRECTION_MODE_LABELS: Record<Exclude<DirectionMode, "">, string> = {
  vector: "Length along a direction vector",
  angle: "Length at an angle from the horizontal",
  slope: "Horizontal run at a slope of 1 in n"
};

export type DirectionDraft = {
  mode: DirectionMode;
  /** Vector and angle: the element's length. Slope: the horizontal run. */
  length: string;
  lengthUnit: string;
  /** Vector: any nonzero direction. Angle and slope: the horizontal direction (its up component must be 0). */
  direction: { x: string; y: string; z: string };
  up: UpDirection;
  angle: string;
  angleUnit: string;
  /** n in "1 in n": vertical rise or fall over horizontal run is 1/n. */
  slopeN: string;
  slopeSense: "" | "falls" | "rises";
};

export const emptyDirectionDraft = (lengthUnit = ""): DirectionDraft => ({
  mode: "", length: "", lengthUnit, direction: { x: "", y: "", z: "" }, up: "", angle: "", angleUnit: "", slopeN: "", slopeSense: ""
});

export type Convert = (value: number, from: string, to: string, dimension: "length" | "angle") => Promise<number>;

/** The engine's own convert_for_dimension, through the display-conversion facade (no factor here). */
export const engineConvert: Convert = async (value, from, to, dimension) => {
  const [item] = await convertDisplayQuantities([{ id: "entry", value, from_unit: from, to_unit: to, dimension_id: dimension }]);
  if (!item || item.status !== "converted" || !Number.isFinite(item.value)) {
    throw new Error(`Cannot convert ${dimension} from ${from || "(no unit)"} to ${to}: ${item && item.status === "unavailable" ? item.message : "conversion unavailable"}.`);
  }
  return item.value;
};

export type DirectionResult = {
  /** The end point in the project length unit, exactly as it is emitted. */
  position: Vec3;
  increment: Vec3;
  /** The converted length (or run) in the project length unit. */
  lengthInProjectUnit: number;
};

/** +0 for −0; every other value unchanged (JSON cannot carry −0). */
export const normalizeZero = (value: number): number => value === 0 ? 0 : value;

function finite(value: string, label: string): number {
  if (!value.trim() || !Number.isFinite(Number(value))) throw new Error(`${label} requires a finite number.`);
  return Number(value);
}

function upAxis(up: UpDirection): { axis: Axis; sign: 1 | -1 } {
  if (!up) throw new Error("State which global direction is up. The model records no vertical axis, so none is assumed.");
  return { axis: up[1] as Axis, sign: up[0] === "-" ? -1 : 1 };
}

/** u = d / |d|, with |d| = sqrt(dx*dx + dy*dy + dz*dz) evaluated left to right. */
export function unitDirection(d: Vec3): Vec3 {
  const norm = Math.sqrt(d.x * d.x + d.y * d.y + d.z * d.z);
  if (!(norm > 0) || !Number.isFinite(norm)) throw new Error("The direction must be a nonzero vector whose length is a finite number.");
  return { x: d.x / norm, y: d.y / norm, z: d.z / norm };
}

function enteredDirection(draft: DirectionDraft, label: string): Vec3 {
  return { x: finite(draft.direction.x, `${label} X`), y: finite(draft.direction.y, `${label} Y`), z: finite(draft.direction.z, `${label} Z`) };
}

/** Request checks that need no conversion; the message names the first missing input. */
export function directionDraftIssue(draft: DirectionDraft, projectLengthUnit: string | undefined): string {
  try { checkDraft(draft, projectLengthUnit); return ""; } catch (error) { return error instanceof Error ? error.message : String(error); }
}

function checkDraft(draft: DirectionDraft, projectLengthUnit: string | undefined) {
  if (!projectLengthUnit?.trim()) throw new Error("The project length unit is not set, so no coordinate can be computed.");
  if (!draft.mode) throw new Error("Choose how the end point is entered.");
  const length = finite(draft.length, draft.mode === "slope" ? "Horizontal run" : "Length");
  if (!(length > 0)) throw new Error(`${draft.mode === "slope" ? "Horizontal run" : "Length"} must be greater than zero.`);
  if (!draft.lengthUnit.trim()) throw new Error("Choose the length unit.");
  if (draft.mode === "vector") { unitDirection(enteredDirection(draft, "Direction")); return; }
  const { axis } = upAxis(draft.up);
  const horizontal = enteredDirection(draft, "Horizontal direction");
  if (horizontal[axis] !== 0) throw new Error(`The horizontal direction must have a zero ${axis.toUpperCase()} component, because ${axis.toUpperCase()} is vertical.`);
  unitDirection(horizontal);
  if (draft.mode === "angle") {
    finite(draft.angle, "Angle from the horizontal");
    if (!draft.angleUnit.trim()) throw new Error("Choose the angle unit.");
  } else {
    const n = finite(draft.slopeN, "Slope n");
    if (!(n > 0)) throw new Error("Slope n must be greater than zero.");
    if (!draft.slopeSense) throw new Error("Choose whether the line falls or rises along the horizontal direction.");
  }
}

/** The end point from `start` (stored coordinates, project length unit). See the header for the formulas. */
export async function computeDirectionEnd(draft: DirectionDraft, start: Vec3, projectLengthUnit: string | undefined, convert: Convert = engineConvert): Promise<DirectionResult> {
  checkDraft(draft, projectLengthUnit);
  const unit = projectLengthUnit!.trim();
  const ls = await convert(Number(draft.length), draft.lengthUnit.trim(), unit, "length");
  if (!(ls > 0) || !Number.isFinite(ls)) throw new Error("The converted length is not a positive finite number.");
  let increment: Vec3;
  if (draft.mode === "vector") {
    const u = unitDirection(enteredDirection(draft, "Direction"));
    increment = { x: ls * u.x, y: ls * u.y, z: ls * u.z };
  } else {
    const { axis, sign } = upAxis(draft.up);
    const h = unitDirection(enteredDirection(draft, "Horizontal direction"));
    let horizontal: number;
    let vertical: number;
    if (draft.mode === "angle") {
      const theta = await convert(Number(draft.angle), draft.angleUnit.trim(), "rad", "angle");
      if (!(Math.abs(theta) < Math.PI / 2)) throw new Error("The angle from the horizontal must lie strictly between −90° and 90°. Enter a vertical element as a direction vector.");
      horizontal = ls * Math.cos(theta);
      vertical = ls * Math.sin(theta);
    } else {
      horizontal = ls;
      vertical = (draft.slopeSense === "rises" ? 1 : -1) * (ls / Number(draft.slopeN));
    }
    increment = { x: 0, y: 0, z: 0 };
    for (const component of AXES) increment[component] = component === axis ? sign * vertical : horizontal * h[component];
  }
  const position = { x: 0, y: 0, z: 0 };
  for (const component of AXES) {
    const value = normalizeZero(start[component] + increment[component]);
    if (!Number.isFinite(value)) throw new Error("The computed end point is not finite.");
    position[component] = value;
  }
  return { position, increment, lengthInProjectUnit: ls };
}

/** The exact text of a coordinate: ECMAScript's shortest round-trip form, which parses back to the same double. */
export const exactCoordinateText = (value: number): string => String(normalizeZero(value));
