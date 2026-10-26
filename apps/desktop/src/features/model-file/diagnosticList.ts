// Status-line listing of a save's diagnostics or warnings. The native side
// names at most 64 and closes a longer list with one SAVE-DIAGNOSTICS-NOT-SHOWN
// item ("N more of the M diagnostics were not shown"); the "more" count shown
// here adds those N to the named items this line leaves out, so it states the
// true remainder.

export const SAVE_DIAGNOSTICS_NOT_SHOWN = "SAVE-DIAGNOSTICS-NOT-SHOWN";

export interface ListedItem {
  readonly code: string;
  readonly path: string;
  readonly message: string;
}

/** `limit` items as `code at path[: message]`, then " (+N more)" for the true remainder. */
export function listedItems(items: ReadonlyArray<ListedItem>, limit: number, withMessage: boolean): string {
  const last = items.at(-1);
  const summary = last?.code === SAVE_DIAGNOSTICS_NOT_SHOWN ? last : undefined;
  const named = summary ? items.slice(0, -1) : items;
  const notSent = summary ? Number(/^(\d+) more of /.exec(summary.message)?.[1] ?? Number.NaN) : 0;
  const shown = named.slice(0, limit).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}${withMessage ? `: ${item.message}` : ""}`);
  if (summary && !Number.isSafeInteger(notSent)) {
    // An unrecognised summary is shown as sent rather than counted.
    const more = named.length > shown.length ? ` (+${named.length - shown.length} more)` : "";
    return `${shown.join(" | ")}${more}; ${summary.message}`;
  }
  const remainder = named.length - shown.length + notSent;
  return `${shown.join(" | ")}${remainder > 0 ? ` (+${remainder} more)` : ""}`;
}
