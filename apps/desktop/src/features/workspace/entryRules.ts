// Entry rules shared by the entry forms and the property task (PRD §5.5; DEL-02-02 REQ-022
// and AC-011; DEL-07-03 AC-001).
//
// One rule, enforced here and nowhere else: when a unit shown beside a typed number moves
// by anything other than the person's own choice of that unit, the number is cleared and
// the form says which unit changed, from and to; no number is converted. The only
// exemption is `chose(key)`, called solely from that entry's own unit control.
// A record without its source is refused by name, in place.
import { useCallback, useLayoutEffect, useMemo, useRef, useState } from "react";

/** One entry of a form: the unit shown beside it and the digits typed beside that unit. */
export type GuardedEntry = {
  /** How the form names the unit in its notice, e.g. "length". */
  label: string;
  unit: string;
  values: string[];
};

export type UnitMove = { key: string; label: string; from: string; to: string };

/** Marks that the person's own act on an entry's unit control chose its unit. */
export type UnitGuard = { chose: (key: string) => void };

/**
 * Remembers, per entry, the unit shown and whether digits stood beside it on the last
 * render. When the unit shown changes and the change was not the person's own choice of
 * that unit (`chose`), `onClear` gets the move if digits stood beside the old unit, else
 * `onSuperseded` gets it (so a notice of an earlier clearing can name where the unit now
 * is). Whatever moved the unit does not matter. A change of `baseline` means the entries
 * were replaced (another project session, or the task's other target or field): nothing
 * is compared across it.
 */
export function useClearOnUnitMove(
  entries: Record<string, GuardedEntry>,
  baseline: string,
  onClear: (moves: UnitMove[]) => void,
  onSuperseded?: (moves: UnitMove[]) => void
): UnitGuard {
  const last = useRef<{ baseline: string; entries: Record<string, { unit: string; typed: boolean }> } | null>(null);
  const chosen = useRef(new Set<string>());
  useLayoutEffect(() => {
    const previous = last.current;
    last.current = {
      baseline,
      entries: Object.fromEntries(Object.entries(entries).map(([key, entry]) => [key, { unit: entry.unit, typed: entry.values.some((value) => value.trim() !== "") }]))
    };
    const own = chosen.current;
    chosen.current = new Set();
    if (!previous || previous.baseline !== baseline) return;
    const moved = Object.entries(entries).filter(([key, entry]) =>
      key in previous.entries && previous.entries[key].unit !== entry.unit && !own.has(key)
    );
    const asMove = ([key, entry]: [string, GuardedEntry]): UnitMove => ({ key, label: entry.label, from: previous.entries[key].unit || "none", to: entry.unit || "none" });
    const cleared = moved.filter(([key]) => previous.entries[key].typed).map(asMove);
    const superseded = moved.filter(([key]) => !previous.entries[key].typed).map(asMove);
    if (cleared.length > 0) onClear(cleared);
    if (superseded.length > 0) onSuperseded?.(superseded);
  });
  const chose = useCallback((key: string) => { chosen.current.add(key); }, []);
  return useMemo(() => ({ chose }), [chose]);
}

let draftIdentitySequence = 0;

/**
 * A form's draft with its own identity. Replacing the draft (passing a whole new draft)
 * gives it a new identity in the same state update; editing it (passing a function of the
 * current draft) keeps the identity. The identity is the draft's baseline for
 * `useClearOnUnitMove`, so a replaced draft is never compared with the one it replaced, and
 * the draft and its baseline cannot change one render apart (DEL-02-02 REQ-022).
 */
export function useIdentifiedDraft<T>(initial: () => T) {
  const [state, setState] = useState(() => ({ identity: ++draftIdentitySequence, draft: initial() }));
  const set = useCallback((next: T | ((current: T) => T)) => {
    setState((current) => typeof next === "function"
      ? { identity: current.identity, draft: (next as (current: T) => T)(current.draft) }
      : { identity: ++draftIdentitySequence, draft: next });
  }, []);
  return [state.draft, String(state.identity), set] as const;
}

/**
 * A shown notice after a later move of the same unit that cleared nothing: the unit still
 * changed from where the values were typed, and it now stands at the later destination.
 */
export function supersedeMoves(current: UnitMove[] | null, later: UnitMove[]): UnitMove[] | null {
  if (!current) return current;
  const to = new Map(later.map((move) => [move.key, move.to]));
  if (!current.some((move) => to.has(move.key))) return current;
  return current.map((move) => (to.has(move.key) ? { ...move, to: to.get(move.key)! } : move));
}

/** The form's notice for cleared values, naming each unit that changed, from and to. */
export function unitMoveNotice(moves: UnitMove[]): string {
  return moves
    .map((move) => move.from === move.to
      ? `The ${move.label} unit changed and is ${move.to} again; the values beside it were cleared.`
      : `The ${move.label} unit changed from ${move.from} to ${move.to}; the values beside it were cleared.`)
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
