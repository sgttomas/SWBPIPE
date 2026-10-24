// The history lists' entries (W-12; DEL-14-01 Design §2, §3): the native store
// lists each saved state or recorded run it can read as its verified summary,
// and each row it cannot read only by its id and its named refusal. One
// unreadable row never hides a project's history; none of its unverified fields
// is presented, and nothing that reads it (open, check, compare, record against
// it) is offered.

/** A stored row the native store could not read: its id (cut as messages quote it, for
 * display only), its refusal, its delete handle (the only key that deletes it) and how many
 * other stored rows name it (a state's child states and runs; they block its deletion). */
export type UnreadableRow = { id: string; refusal: string; handle: string; references: number };

/** The refusal code of a run recorded before rule-pack resolution (re-record it). */
export const RULE_PACKS_NOT_RESOLVED = "ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED";

/** What the History panel says beside an unreadable row. */
export function unreadableGuidance(refusal: string): string {
  return refusal.startsWith(RULE_PACKS_NOT_RESOLVED)
    ? "Re-record this run: solve the model again and record the new run; this one stays unreadable until it is deleted."
    : "The stored row does not verify. It is not read; it can be deleted.";
}

function unreadableOf(entry: unknown): UnreadableRow | null {
  if (!entry || typeof entry !== "object" || Array.isArray(entry)) return null;
  const record = entry as Record<string, unknown>;
  const unreadable = record.unreadable;
  if (Object.keys(record).length !== 1 || !unreadable || typeof unreadable !== "object") return null;
  const { id, refusal, handle, references } = unreadable as Record<string, unknown>;
  return typeof id === "string" && typeof refusal === "string" && typeof handle === "string" && typeof references === "number"
    ? { id, refusal, handle, references }
    : null;
}

/** A list response split into its readable summaries and its unreadable rows. */
export function partitionListed<T>(entries: Array<T | { unreadable: UnreadableRow }>): { readable: T[]; unreadable: UnreadableRow[] } {
  const readable: T[] = [];
  const unreadable: UnreadableRow[] = [];
  for (const entry of entries) {
    const row = unreadableOf(entry);
    if (row) unreadable.push(row);
    else readable.push(entry as T);
  }
  return { readable, unreadable };
}
