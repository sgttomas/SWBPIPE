/** First-match lookup of an analysis record's result reference by row id.
 *
 * Each call answers what `refs.find(x => x.result_ref.ref === id)` answers on the same list:
 * the earliest entry whose reference strictly equals `id`, `undefined` when none does, and the
 * same thrown error when an entry the search would reach cannot be read.
 *
 * The list is read forward once. An entry is read when a lookup first needs it and is never
 * read again, so the lookups for a whole result examine each entry at most once, where a
 * search from the start for every row examines N(N+1)/2 entries for N rows.
 *
 * A list that is not an array, or has no `find` to call, is refused with a `TypeError` at the
 * first lookup, where the search threw one: an object holding numbered entries and a length
 * is not a list of references.
 *
 * The equivalence is for a list whose reads are stable: an ordinary array that nothing
 * changes between the first lookup and the last. The search read the live list again for
 * every row and this lookup does not, so the two can differ for a list edited in place
 * between lookups; the builder's closing comparison of its inputs' text covers an edit that
 * is still there at the end. */
export function firstMatchResultRefLookup<T extends { result_ref: { ref: unknown } }>(refs: readonly T[]): (id: unknown) => T | undefined {
  const first = new Map<unknown, T>();
  let next = 0;
  return id => {
    if (!Array.isArray(refs) || typeof refs.find !== "function") throw new TypeError("result references are not an array");
    // `Map` compares keys by SameValueZero, which differs from `===` only in that NaN equals
    // NaN. `===` matches NaN to nothing, so a NaN id takes no answer from the map.
    if (id === id && first.has(id)) return first.get(id);
    while (next < refs.length) {
      const entry = refs[next], key = entry.result_ref.ref;
      if (!first.has(key)) first.set(key, entry);
      next += 1;
      if (key === id) return entry;
    }
    return undefined;
  };
}
