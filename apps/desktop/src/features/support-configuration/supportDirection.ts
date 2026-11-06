// R-6/R-7 S-K1 desktop entry (DEL-07-02 Work `support-direction.md`; contract in DEL-04-03
// Work `skew-restraints.md`, "The model, applier and authoring contract for S-K1"). The
// support creation form's optional Direction (x, y, z): three typed numbers, written
// exactly as entered and never normalized, no unit. Empty means absent. Nothing here
// judges admission: the operation engine (the document rules, then the S-K1 skew
// admission) refuses by name, and the form shows its finding in place.
import { parseNumber } from "../model-tree/runSheet/runSheetModel";
import type { SupportDirection } from "../../types";

export const DIRECTION_AXES = ["x", "y", "z"] as const;
export type DirectionAxis = (typeof DIRECTION_AXES)[number];
/** The three inputs' text, kept exactly as typed. */
export type DirectionText = Record<DirectionAxis, string>;
export type { SupportDirection };

export const EMPTY_DIRECTION_TEXT: DirectionText = Object.freeze({ x: "", y: "", z: "" });

/** The stored-direction label: S-K1 has no `update_support`/`set_field` path for it. */
export const STORED_DIRECTION_NOTE = "edit/remove not supported yet (S-K1)";

export const DIRECTION_REFUSAL = {
  partial: (missing: readonly DirectionAxis[]) =>
    `Direction needs all three components (x, y, z), or none. Enter ${missing.join(", ")}, or clear the direction; a blank component is not taken as zero.`,
  notNumber: (axis: DirectionAxis) => `Direction ${axis} is not a number. Enter a plain decimal number (no unit).`,
  negativeZero: (axis: DirectionAxis) =>
    `Direction ${axis} is -0. A negative zero cannot be stored apart from 0, so it would not be written as entered; enter 0.`
} as const;

/**
 * The entered direction: `null` when every input is blank (the member is not written),
 * the vector of the typed numbers otherwise. A partial vector, a non-number and `-0`
 * (which the JSON payload cannot carry as entered) are refused before queueing.
 */
export function parseDirectionEntry(text: DirectionText): SupportDirection | null {
  const blank = DIRECTION_AXES.filter((axis) => text[axis].trim() === "");
  if (blank.length === DIRECTION_AXES.length) return null;
  if (blank.length > 0) throw new Error(DIRECTION_REFUSAL.partial(blank));
  const vector = {} as SupportDirection;
  for (const axis of DIRECTION_AXES) {
    const value = parseNumber(text[axis]);
    if (value === null) throw new Error(DIRECTION_REFUSAL.notNumber(axis));
    if (Object.is(value, -0)) throw new Error(DIRECTION_REFUSAL.negativeZero(axis));
    vector[axis] = value;
  }
  return vector;
}

const componentText = (value: unknown): string =>
  typeof value === "number" ? (Object.is(value, -0) ? "-0" : String(value)) : JSON.stringify(value) ?? "absent";

/**
 * A stored `direction` exactly as stored: each component's shortest round-trip text, as
 * the applier writes it. A malformed value (the document rules refuse it by name) is shown
 * as its JSON. `null` when the record has no `direction` member.
 */
export function storedDirectionText(support: unknown): string | null {
  if (!support || typeof support !== "object" || !("direction" in support)) return null;
  const direction = (support as { direction?: unknown }).direction;
  if (direction === undefined) return null;
  if (direction && typeof direction === "object" && !Array.isArray(direction)) {
    const members = direction as Record<string, unknown>;
    const keys = Object.keys(members);
    if (keys.length === 3 && DIRECTION_AXES.every((axis) => axis in members))
      return `(${DIRECTION_AXES.map((axis) => componentText(members[axis])).join(", ")})`;
  }
  return JSON.stringify(direction);
}
