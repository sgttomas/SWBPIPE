// Entry rules shared by the creation forms (PRD §5.5; DEL-02-02 REQ-022 and AC-011;
// DEL-07-03 AC-001).
//
// A number typed beside a unit is an authored quantity. When a unit the user did not
// change moves with the project units, the numbers beside it are cleared, everything
// else typed is kept, and the form says which unit changed and that the values beside
// it were cleared. A record without its source is refused by name, in place.
import { useLayoutEffect, useRef } from "react";

/** One unit of a form that may follow the project units, and the numbers typed beside it. */
export type FollowedUnit = {
  /** How the form names the unit in its notice, e.g. "length". */
  label: string;
  unit: string;
  /** True while the user has not changed this unit, so it follows the project units. */
  follows: boolean;
  values: string[];
};

export type UnitMove = { key: string; label: string; from: string; to: string };

/**
 * Calls `onClear` with each followed unit that moved since the last render while numbers
 * stood beside it. A change of `baseline` (a new project session, or the user choosing a
 * different kind of quantity on the form) starts afresh without clearing anything.
 */
export function useClearOnUnitMove(
  units: Record<string, FollowedUnit>,
  baseline: string,
  onClear: (moves: UnitMove[]) => void
): void {
  const last = useRef<{ baseline: string; units: Record<string, string> } | null>(null);
  useLayoutEffect(() => {
    const previous = last.current;
    last.current = { baseline, units: Object.fromEntries(Object.entries(units).map(([key, entry]) => [key, entry.unit])) };
    if (!previous || previous.baseline !== baseline) return;
    const moves = Object.entries(units)
      .filter(([key, entry]) =>
        entry.follows &&
        key in previous.units &&
        previous.units[key] !== entry.unit &&
        entry.values.some((value) => value.trim() !== "")
      )
      .map(([key, entry]) => ({ key, label: entry.label, from: previous.units[key] || "none", to: entry.unit || "none" }));
    if (moves.length > 0) onClear(moves);
  });
}

/** The form's notice for cleared values, naming each unit that changed, from and to. */
export function unitMoveNotice(moves: UnitMove[]): string {
  return moves
    .map((move) => `The ${move.label} unit changed from ${move.from} to ${move.to} with the project units; the values beside it were cleared.`)
    .join(" ");
}

/**
 * The in-place reason a record is not queued for want of its source, naming each missing
 * source field, or null when none is missing.
 */
export function missingSourceReason(record: string, sources: Array<{ name: string; value: string }>): string | null {
  const missing = sources.filter((source) => source.value.trim() === "").map((source) => source.name);
  if (missing.length === 0) return null;
  const named = missing.length === 1 ? missing[0] : `${missing.slice(0, -1).join(", ")} and ${missing[missing.length - 1]}`;
  return `Not queued: enter the ${record}'s ${named}. A record without its source is refused.`;
}
