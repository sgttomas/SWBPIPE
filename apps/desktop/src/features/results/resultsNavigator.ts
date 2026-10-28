/** The results navigator (survey item A1; DEL-07-05 Design `results-navigator.md`). A position
 * over the Results table's existing rows on three axes: the table kind (the row families the
 * family filter already offers), the load case or combination a row states, and the support or
 * element a row is stated for. Stepping an axis moves along its list and stops at either end.
 *
 * Pure presentation of existing rows: nothing here reads, forms or orders a value. A row's case
 * is its own `basis_ref`; its entity is its own `entity_ref`; its kind is the reader's family,
 * passed in. No row is left out of a list for its numerical standing or health. */

import type { MechanicsResult, ResultBasisRef } from "../../types";

type ResultRow = MechanicsResult["results"][number];

/** The case list's "every case" position and the key of rows that state no case. */
export const ALL_CASES = "all";
export const UNSTATED_CASE = "unstated";
/** The range-envelope basis (`metadata.basis`), as the viewport overlays name it. */
const RANGE_ENVELOPE_BASIS = "explicit_user_range_envelope";

export type NavigatorAxis = "kind" | "case" | "entity";

export type NavigatorCase = Readonly<{
  key: string;
  label: string;
  /** The row's own basis, or null for rows that state none. */
  basis: ResultBasisRef | null;
  /** Some row of this case is a range envelope: its values are not one solved state. */
  envelope: boolean;
}>;

/** A row's case key: its `basis_ref` as `ref_type:ref_id`, or `UNSTATED_CASE`. */
export function rowCaseKey(row: Pick<ResultRow, "basis_ref">): string {
  return row.basis_ref ? `${row.basis_ref.ref_type}:${row.basis_ref.ref_id}` : UNSTATED_CASE;
}

function caseLabel(basis: ResultBasisRef | null): string {
  if (!basis) return "No case stated";
  if (basis.ref_type === "load_case") return `Load case ${basis.ref_id}`;
  if (basis.ref_type === "combination") return `Combination ${basis.ref_id}`;
  return `${basis.ref_type} ${basis.ref_id}`;
}

/** The cases the rows state, each once: load cases, then combinations, then other bases, each in
 * the order they first appear; rows that state none last. Every row's case is listed. */
export function navigatorCases(rows: readonly ResultRow[]): NavigatorCase[] {
  const seen = new Map<string, { basis: ResultBasisRef | null; envelope: boolean }>();
  for (const row of rows) {
    const key = rowCaseKey(row);
    const entry = seen.get(key) ?? { basis: row.basis_ref ? { ref_type: row.basis_ref.ref_type, ref_id: row.basis_ref.ref_id } : null, envelope: false };
    if (row.metadata?.basis === RANGE_ENVELOPE_BASIS) entry.envelope = true;
    seen.set(key, entry);
  }
  const rank = (basis: ResultBasisRef | null) => !basis ? 3 : basis.ref_type === "load_case" ? 0 : basis.ref_type === "combination" ? 1 : 2;
  return [...seen.entries()]
    .map(([key, entry], order) => ({ key, order, ...entry }))
    .sort((a, b) => rank(a.basis) - rank(b.basis) || a.order - b.order)
    .map(({ key, basis, envelope }) => Object.freeze({ key, basis, envelope, label: `${caseLabel(basis)}${envelope ? " (range envelope)" : ""}` }));
}

/** The entities the rows are stated for, each once, in the order they first appear. */
export function navigatorEntities(rows: readonly ResultRow[]): string[] {
  return [...new Set(rows.map((row) => row.entity_ref))];
}

/** The index one step from `index` along a list of `length`, or null when the step cannot move:
 * an empty list, or the first or last item already reached (the navigator stops; it does not
 * wrap). From no position (`index` < 0), Next goes to the first item and Previous to the last. */
export function stepIndex(length: number, index: number, delta: 1 | -1): number | null {
  if (length <= 0) return null;
  if (index < 0 || index >= length) return delta > 0 ? 0 : length - 1;
  const next = index + delta;
  return next < 0 || next >= length ? null : next;
}

/** The first of `rows` stated for `entity`, or null when it has none there. */
export function firstRowOf(rows: readonly ResultRow[], entity: string | null): ResultRow | null {
  if (entity === null) return null;
  return rows.find((row) => row.entity_ref === entity) ?? null;
}

/** The key chord of each step. Alt+ArrowLeft/Right is the support summary's own support step, so
 * the entity step takes the same chord; the case and kind steps take the chords next to it. */
export const NAVIGATOR_KEYS: Readonly<Record<NavigatorAxis, Readonly<{ previous: string; next: string }>>> = Object.freeze({
  kind: Object.freeze({ previous: "Alt+Shift+ArrowLeft", next: "Alt+Shift+ArrowRight" }),
  case: Object.freeze({ previous: "Alt+ArrowUp", next: "Alt+ArrowDown" }),
  entity: Object.freeze({ previous: "Alt+ArrowLeft", next: "Alt+ArrowRight" })
});

type KeyChord = Readonly<{ key: string; altKey: boolean; shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }>;

/** The step a key chord asks for, or null. Only Alt (Option) with the arrows, optionally Shift
 * for the kind axis; Ctrl or Meta, or Shift with Up or Down, is never a navigator step. */
export function navigatorKeyStep(event: KeyChord): Readonly<{ axis: NavigatorAxis; delta: 1 | -1 }> | null {
  if (!event.altKey || event.ctrlKey || event.metaKey) return null;
  if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
    return { axis: event.shiftKey ? "kind" : "entity", delta: event.key === "ArrowRight" ? 1 : -1 };
  }
  if (!event.shiftKey && (event.key === "ArrowUp" || event.key === "ArrowDown")) {
    return { axis: "case", delta: event.key === "ArrowDown" ? 1 : -1 };
  }
  return null;
}

/** Whether a key's target edits or chooses something itself (a text field, a select, a
 * contenteditable region, a combobox, listbox or menu), so the navigator leaves the key to it. */
export function isNavigatorExcludedTarget(target: EventTarget | null): boolean {
  if (!target || typeof (target as Element).closest !== "function") return false;
  const element = target as Element;
  if ((element as HTMLElement).isContentEditable) return true;
  return element.closest('input, textarea, select, [contenteditable]:not([contenteditable="false"]), [role="combobox"], [role="listbox"], [role="menu"]') !== null;
}
