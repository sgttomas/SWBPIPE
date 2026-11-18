import { describe, expect, it } from "vitest";
import { firstMatchResultRefLookup } from "./resultRefLookup";

type Entry = { result_ref: { ref: unknown }; tag: number };
type Outcome = { entry: unknown } | { thrown: string };

/** The search the lookup replaces, kept here as the meaning it must reproduce. */
const searchFromStart = (refs: readonly Entry[], id: unknown) => refs.find(x => x.result_ref.ref === id);
const outcome = (act: () => unknown): Outcome => {
  try { return { entry: act() }; } catch (error) { return { thrown: (error as Error).constructor.name }; }
};
/** Every lookup in `ids`, in order, gives the search's own entry (by identity) or its error. */
function expectSameAsSearch(refs: readonly Entry[], ids: readonly unknown[]) {
  const lookup = firstMatchResultRefLookup(refs);
  for (const id of ids) {
    const expected = outcome(() => searchFromStart(refs, id)), actual = outcome(() => lookup(id));
    if ("thrown" in expected) expect(actual).toStrictEqual(expected);
    else { expect("entry" in actual).toBe(true); expect((actual as { entry: unknown }).entry).toBe(expected.entry); }
  }
}
const entry = (ref: unknown, tag: number): Entry => ({ result_ref: { ref }, tag });
function mulberry32(seed: number) {
  return () => { seed = (seed + 0x6d2b79f5) | 0; let t = Math.imul(seed ^ (seed >>> 15), 1 | seed); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
}

describe("first-match result reference lookup", () => {
  it("returns the earliest of duplicate references, in any lookup order", () => {
    const refs = [entry("a", 0), entry("b", 1), entry("a", 2), entry("c", 3), entry("b", 4)];
    const lookup = firstMatchResultRefLookup(refs);
    // The later duplicate is asked for only after a lookup that read past it.
    expect(lookup("c")).toBe(refs[3]);
    expect(lookup("a")).toBe(refs[0]);
    expect(lookup("b")).toBe(refs[1]);
    for (const order of [["a", "b", "c"], ["c", "b", "a"], ["b", "b", "a", "c", "a"]]) expectSameAsSearch(refs, order);
  });

  it("answers a missing reference with undefined, before and after the list is exhausted", () => {
    const refs = [entry("a", 0), entry("b", 1)];
    const lookup = firstMatchResultRefLookup(refs);
    expect(lookup("absent")).toBeUndefined();
    expect(lookup("a")).toBe(refs[0]);
    expect(lookup("absent")).toBeUndefined();
    expect(firstMatchResultRefLookup([])("a")).toBeUndefined();
    expectSameAsSearch(refs, ["absent", "b", "absent", "a", ""]);
  });

  it("compares strictly: no coercion, signed zeros equal, NaN equal to nothing, objects by identity", () => {
    const object = {}, refs = [entry(0, 0), entry("0", 1), entry(null, 2), entry(NaN, 3), entry(object, 4), entry(false, 5), entry("", 6)];
    const lookup = firstMatchResultRefLookup(refs);
    expect(lookup(-0)).toBe(refs[0]);
    expect(lookup("0")).toBe(refs[1]);
    expect(lookup(NaN)).toBeUndefined();
    expect(lookup(NaN)).toBeUndefined();
    expect(lookup({})).toBeUndefined();
    expect(lookup(object)).toBe(refs[4]);
    expect(lookup(undefined)).toBeUndefined();
    expectSameAsSearch(refs, [NaN, -0, 0, "0", null, undefined, object, {}, false, "", NaN]);
  });

  it("throws on an unreadable entry exactly when the search would reach it", () => {
    const refs = [entry("a", 0), null, entry("b", 2)] as unknown as Entry[];
    const lookup = firstMatchResultRefLookup(refs);
    expect(lookup("a")).toBe(refs[0]);
    expect(() => lookup("b")).toThrow(TypeError);
    // The entry before it is still answered, and the one behind it is still unreachable.
    expect(lookup("a")).toBe(refs[0]);
    expect(() => lookup("b")).toThrow(TypeError);
    expect(() => lookup("absent")).toThrow(TypeError);
    for (const unreadable of [null, undefined, {}, 7, "text", { result_ref: null }])
      for (const at of [0, 1, 2]) {
        const list = [entry("a", 0), entry("b", 1)] as unknown[];
        list.splice(at, 0, unreadable);
        expectSameAsSearch(list as Entry[], ["a", "b", "absent", "b", "a"]);
      }
    const sparse = [entry("a", 0), , entry("b", 2)] as unknown as Entry[];
    expectSameAsSearch(sparse, ["a", "b", "a"]);
  });

  it("matches the search on generated lists with duplicate, missing and unreadable entries", () => {
    const random = mulberry32(0x5eed), shared = {}, pool: unknown[] = ["a", "b", "c", "d", "e", "", "0", 0, -0, 1, null, true, false, NaN, shared];
    const pick = () => pool[Math.floor(random() * pool.length)];
    let duplicates = 0, missing = 0, thrown = 0;
    for (let round = 0; round < 2000; round += 1) {
      const length = Math.floor(random() * 13), refs: unknown[] = [];
      for (let i = 0; i < length; i += 1) refs.push(random() < 0.04 ? [null, {}, 3, { result_ref: undefined }][Math.floor(random() * 4)] : entry(pick(), i));
      const ids = Array.from({ length: Math.floor(random() * 31) }, pick);
      expectSameAsSearch(refs as Entry[], ids);
      // The generated population holds the cases the test is for.
      const readable = refs.filter((x): x is Entry => typeof x === "object" && x !== null && typeof (x as Entry).result_ref === "object" && (x as Entry).result_ref !== null);
      if (new Set(readable.map(x => x.result_ref.ref)).size < readable.length) duplicates += 1;
      if (readable.length === refs.length && ids.some(id => !readable.some(x => x.result_ref.ref === id))) missing += 1;
      if (ids.some(id => "thrown" in outcome(() => searchFromStart(refs as Entry[], id)))) thrown += 1;
    }
    expect(duplicates).toBeGreaterThan(500);
    expect(missing).toBeGreaterThan(500);
    expect(thrown).toBeGreaterThan(100);
  });

  it("reads each entry at most once for a whole result, at the reference system's row count", () => {
    const rows = 20_331;
    let reads = 0;
    const refs: Entry[] = Array.from({ length: rows }, (_, i) => {
      const target = entry(`result:${i}`, i);
      return new Proxy(target, { get(t, key, receiver) { if (key === "result_ref") reads += 1; return Reflect.get(t, key, receiver); } });
    });
    // The search reads i + 1 entries for row i whatever the row order: N(N+1)/2 in all.
    const lookup = firstMatchResultRefLookup(refs);
    for (let i = rows - 1; i >= 0; i -= 1) expect(lookup(`result:${i}`)).toBe(refs[i]);
    expect(reads).toBe(rows);
    for (let i = 0; i < rows; i += 1) expect(lookup(`result:${i}`)).toBe(refs[i]);
    expect(lookup("absent")).toBeUndefined();
    expect(reads).toBe(rows);
    expect(rows * (rows + 1) / 2).toBe(206_684_946);
  });
});
