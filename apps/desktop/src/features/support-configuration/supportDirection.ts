// R-6/R-7 S-K1 desktop entry (DEL-07-02 Work `support-direction.md`; contract in DEL-04-03
// Work `skew-restraints.md`, "The model, applier and authoring contract for S-K1"). The
// support creation form's optional Direction (x, y, z): three typed numbers, written
// exactly as entered and never normalized, no unit. Empty means absent. Nothing here
// judges admission: the operation engine (the document rules, then the S-K1 skew
// admission) refuses by name, and the form shows its finding in place.
import { parseNumber } from "../model-tree/runSheet/runSheetModel";
import type { SupportDirection } from "../../types";
import { DIRECTION_REMOVAL_REASON, engineHangerKind } from "../hanger-selection/hangerSelection";

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

/** Whether a support value carries a hanger as the operation engine reads it: a `hanger` member at all (any value), or
 * a family or hanger type the engine names as a hanger, or a member its decode refuses. A hanger takes no direction. */
export function supportCarriesHanger(value: unknown): boolean {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  return "hanger" in value || engineHangerKind(value) !== null;
}

/** In place of the direction fields while a support being created carries a hanger. */
export const HANGER_NO_DIRECTION_TEXT = "A hanger's load acts along the positive axis of its degree of freedom; no direction is entered.";
/** Said once where the fields were, when the hanger cleared a direction the person had typed. */
export const DIRECTION_CLEARED_TEXT = "The direction you entered was cleared.";

/** What the person reads before a configuration that gives a directed support a hanger is queued. `stored` is the
 * stored direction as the form prints it (`storedDirectionText`). */
export function directionRemovalStatement(stored: string): string {
  return `Adding the hanger removes this support's stored direction ${stored}. ${DIRECTION_REMOVAL_REASON}`;
}

/** The engine refused the hanger on a record that still holds the direction (SUPPORT_DIRECTION_RECORD_UNSUPPORTED). */
export function directionRemovalRefusedText(stored: string): string {
  return `${directionRemovalStatement(stored)} The operation engine refused the configuration while the direction is stored, so nothing was queued. ` +
    "To add the hanger, reopen the support and queue its configuration again, which queues the removal first; to keep the direction, leave out the hanger data and choose a family that is not a hanger.";
}

/** The removal's before no longer matches the stored direction (OP-BEFORE-VALUE-MISMATCH). */
export const STORED_DIRECTION_CHANGED_TEXT = "The stored direction of this support changed since the form opened, so nothing was queued. Reopen the support.";

