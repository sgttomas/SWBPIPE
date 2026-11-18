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
 * The equivalence is for a list that does not change between lookups. */
export function firstMatchResultRefLookup<T extends { result_ref: { ref: unknown } }>(refs: readonly T[]): (id: unknown) => T | undefined {
  const first = new Map<unknown, T>();
  let next = 0;
  return id => {
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
