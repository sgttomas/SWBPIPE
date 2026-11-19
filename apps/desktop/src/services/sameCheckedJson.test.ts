// `sameCheckedJson` (analysisRunCompatibility.ts) replaced, in `validateAnalysisRunV03`, a
// comparison of the checked text of each side (hashService's `checkedJsonText`, the left side
// wholly first), parsed, member-sorted and serialised again. That comparison is kept below as
// the reference and both are run over a generated corpus: the shapes in REFUSING and
// TWO_CHECKS, on either side and at depth, values nested up to 128 levels, and pairs drawn
// from the analysis-run fixtures' own members. The test requires the same boolean, or the
// same error, on every pair. `checkedJsonText` now shares its checks with the comparison
// (hashService's `checkedJsonWalk`), so its old implementation is kept here too and the
// text function is run against it over the same values: the same text or the same error.
import { readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { sameCheckedJson } from "./analysisRunCompatibility";
import { checkedJsonText } from "./hashService";

/** `checkedJsonTextOf` and its helper before the checks were shared: hashService.ts on main,
 * verbatim but for the two function names. */
function referenceValidUnicodeScalarString(value: string): boolean {
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return false;
      index += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) return false;
  }
  return true;
}
function referenceCheckedJsonTextOf(value: unknown): string {
  const seen = new Set<object>();
  const freeze = (item: unknown): unknown => {
    if (item === null || typeof item === "boolean") return item;
    if (typeof item === "string") {
      if (!referenceValidUnicodeScalarString(item)) throw new Error("CHECKED-JSON-LONE-SURROGATE");
      return item;
    }
    if (typeof item === "number") {
      if (!Number.isFinite(item) || (Number.isInteger(item) && !Number.isSafeInteger(item))) {
        throw new Error("CHECKED-JSON-NUMBER-OUTSIDE-PROFILE");
      }
      return item;
    }
    if (typeof item !== "object") throw new Error(`CHECKED-JSON-UNSUPPORTED-TYPE: ${typeof item}`);
    if (seen.has(item)) throw new Error("CHECKED-JSON-CYCLIC-VALUE");
    seen.add(item);
    try {
      const descriptors = Object.getOwnPropertyDescriptors(item);
      const keys = Reflect.ownKeys(descriptors);
      if (Array.isArray(item)) {
      if (keys.some((key) => typeof key !== "string")) throw new Error("CHECKED-JSON-SYMBOL-KEY");
      const lengthDescriptor = descriptors.length;
      const length = lengthDescriptor?.value;
      if (!lengthDescriptor || typeof length !== "number" || !Number.isSafeInteger(length) || length < 0) throw new Error("CHECKED-JSON-ARRAY-LENGTH");
      const allowed = new Set(["length", ...Array.from({ length }, (_, index) => String(index))]);
      if (keys.some((key) => !allowed.has(key as string))) throw new Error("CHECKED-JSON-ARRAY-OWN-PROPERTY");
      const frozen: unknown[] = [];
      for (let index = 0; index < length; index += 1) {
        const descriptor = descriptors[String(index)];
        if (!descriptor) throw new Error("CHECKED-JSON-SPARSE-ARRAY");
        if (!descriptor || descriptor.get || descriptor.set || !descriptor.enumerable) throw new Error("CHECKED-JSON-UNSUPPORTED-PROPERTY");
        frozen.push(freeze(descriptor.value));
      }
      return frozen;
      }
      const prototype = Object.getPrototypeOf(item);
      if (prototype !== Object.prototype && prototype !== null) throw new Error("CHECKED-JSON-NON-PLAIN-OBJECT");
      const frozen: Record<string, unknown> = Object.create(null) as Record<string, unknown>;
      for (const key of keys) {
        if (typeof key !== "string") throw new Error("CHECKED-JSON-SYMBOL-KEY");
        if (!referenceValidUnicodeScalarString(key)) throw new Error("CHECKED-JSON-LONE-SURROGATE");
        const descriptor = descriptors[key];
        if (!descriptor || descriptor.get || descriptor.set || !descriptor.enumerable) throw new Error("CHECKED-JSON-UNSUPPORTED-PROPERTY");
        frozen[key] = freeze(descriptor.value);
      }
      return frozen;
    } finally {
      seen.delete(item);
    }
  };
  return JSON.stringify(freeze(value));
}


/** The relation before the change, verbatim (validateAnalysisRunV03's `same`). */
function referenceSame(a: unknown, b: unknown): boolean {
  if (a === undefined || b === undefined) return a === b;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const order = (v: any): any => Array.isArray(v) ? v.map(order) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, order(v[k])])) : v;
  return JSON.stringify(order(JSON.parse(checkedJsonText(a)))) === JSON.stringify(order(JSON.parse(checkedJsonText(b))));
}

type Outcome = { value: boolean } | { error: string };
function outcome(compare: (a: unknown, b: unknown) => boolean, a: unknown, b: unknown): Outcome {
  try { return { value: compare(a, b) }; } catch (error) { return { error: `${(error as Error).constructor.name}: ${(error as Error).message}` }; }
}

// A seeded generator (mulberry32), so a failure names a reproducible case.
function generator(seed: number) {
  let state = seed >>> 0;
  const next = () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let t = state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
  const int = (n: number) => Math.floor(next() * n);
  const pick = <T,>(items: readonly T[]): T => items[int(items.length)];
  return { next, int, pick };
}
type Gen = ReturnType<typeof generator>;

const STRINGS = ["", "a", "Ω", "\u{1f600}", "__proto__", "0", "10", "constructor", "toJSON", "b\u0000c", "\"\\", "😀x", "é", "é"];
const KEYS = ["a", "b", "z", "A", "0", "1", "10", "2", "__proto__", "constructor", "toJSON", "Ω", "\u{1f600}", "", "metadata", "kind"];
/** Numbers the checked profile admits. */
const NUMBERS = [0, -0, 1, -1, 1.5, 0.1, 0.30000000000000004, 1e-7, 5e-324, 123456.789, 2 ** 53 - 1, -(2 ** 53 - 1), 6.02214076e-23];

function leaf(g: Gen): unknown {
  switch (g.int(4)) {
    case 0: return g.pick([null, true, false]);
    case 1: return g.pick(STRINGS);
    default: return g.pick(NUMBERS);
  }
}

/** A value the checked profile admits (plain objects, some null-prototype; arrays; shared
 * references, which are not cycles). */
function valid(g: Gen, depth: number): unknown {
  if (depth <= 0 || g.next() < 0.3) return leaf(g);
  if (g.next() < 0.45) return Array.from({ length: g.int(4) }, () => valid(g, depth - 1));
  const object: Record<string, unknown> = g.next() < 0.15 ? Object.create(null) : {};
  for (let count = g.int(5); count > 0; count -= 1) Object.defineProperty(object, g.pick(KEYS), { value: valid(g, depth - 1), enumerable: true, writable: true, configurable: true });
  if (g.next() < 0.1) {
    const shared = valid(g, depth - 1);
    object.left = shared;
    object.right = shared;
  }
  return object;
}

class Invented { constructor(public readonly a = 1) {} }

/** A Proxy over an array whose length descriptor reports `length`: a writable length may be
 * reported with another value, so the profile's array-length check is reachable. */
function lyingLength(target: unknown[], length: number): unknown[] {
  return new Proxy(target, { getOwnPropertyDescriptor(t, key) { const d = Reflect.getOwnPropertyDescriptor(t, key); return key === "length" && d ? { ...d, value: length } : d; } });
}
const getter = (o: object, key: string) => Object.defineProperty(o, key, { get: () => 1, enumerable: true, configurable: true });
const hidden = (o: object, key: string, value: unknown) => Object.defineProperty(o, key, { value, enumerable: false, configurable: true, writable: true });

/** Shapes the checked profile refuses, one refusal each, each as a fresh value. */
const REFUSING: ReadonlyArray<[string, () => unknown]> = [
  ["lone high surrogate", () => "a\ud800"],
  ["lone low surrogate", () => "\udc00b"],
  ["reversed pair", () => "\udc00\ud800"],
  ["NaN", () => Number.NaN],
  ["Infinity", () => Number.POSITIVE_INFINITY],
  ["-Infinity", () => Number.NEGATIVE_INFINITY],
  ["2^53", () => 2 ** 53],
  ["-2^53", () => -(2 ** 53)],
  ["1e21", () => 1e21],
  ["max double", () => Number.MAX_VALUE],
  ["unsafe integral float", () => 1e20 / 3],
  ["undefined member", () => ({ a: undefined })],
  ["undefined element", () => [undefined]],
  ["function", () => () => 1],
  ["symbol value", () => Symbol("invented")],
  ["bigint", () => BigInt(1)],
  ["cycle", () => { const o: Record<string, unknown> = { a: 1 }; o.self = o; return o; }],
  ["array cycle", () => { const a: unknown[] = [1]; a.push(a); return a; }],
  ["symbol key", () => ({ a: 1, [Symbol("k")]: 2 })],
  ["array symbol key", () => Object.assign([1], { [Symbol("k")]: 2 })],
  ["lone surrogate key", () => ({ ["\ud800"]: 1 })],
  ["getter", () => Object.defineProperty({}, "a", { get: () => 1, enumerable: true })],
  ["setter", () => Object.defineProperty({}, "a", { set: () => {}, enumerable: true })],
  ["non-enumerable", () => Object.defineProperty({ b: 1 }, "a", { value: 1, enumerable: false })],
  ["array getter", () => Object.defineProperty([0], "0", { get: () => 1, enumerable: true })],
  ["array non-enumerable element", () => Object.defineProperty([0, 1], "1", { value: 1, enumerable: false })],
  ["sparse array", () => [1, , 3]], // eslint-disable-line no-sparse-arrays
  ["array of holes", () => new Array(2)],
  ["array own property", () => Object.assign([1, 2], { extra: 3 })],
  ["Date", () => new Date(0)],
  ["Map", () => new Map([["a", 1]])],
  ["class instance", () => new Invented()],
  ["inherited object", () => Object.create({ inherited: 1 })],
  ["boxed string", () => new String("a")],
  ["typed array", () => new Uint8Array(2)],
  ["array length reported -1", () => lyingLength([], -1)],
  ["array length reported 1.5", () => lyingLength([1], 1.5)]
];

/** Values that trip two of the profile's checks, in each order the profile distinguishes,
 * with the refusal the profile reaches first. */
const TWO_CHECKS: ReadonlyArray<[string, string, () => unknown]> = [
  ["array: symbol key, before an own property", "CHECKED-JSON-SYMBOL-KEY", () => Object.assign([1], { extra: 1, [Symbol("k")]: 2 })],
  ["array: symbol key, before a hole", "CHECKED-JSON-SYMBOL-KEY", () => Object.assign([1, , 3], { [Symbol("k")]: 2 })], // eslint-disable-line no-sparse-arrays
  ["array: symbol key, before a lying length", "CHECKED-JSON-SYMBOL-KEY", () => lyingLength(Object.assign([1], { [Symbol("k")]: 2 }), -1)],
  ["array: lying length, before an own property", "CHECKED-JSON-ARRAY-LENGTH", () => lyingLength(Object.assign([1], { extra: 1 }), -1)],
  ["array: own property, before a hole", "CHECKED-JSON-ARRAY-OWN-PROPERTY", () => Object.assign([1, , 3], { extra: 1 })], // eslint-disable-line no-sparse-arrays
  ["array: own property, before a getter element", "CHECKED-JSON-ARRAY-OWN-PROPERTY", () => Object.assign(getter([0], "0"), { extra: 1 })],
  ["array: own property, before a refusing element", "CHECKED-JSON-ARRAY-OWN-PROPERTY", () => Object.assign([Number.NaN], { extra: 1 })],
  ["array: hole, before a getter element", "CHECKED-JSON-SPARSE-ARRAY", () => getter(new Array(2), "1")],
  ["array: getter element, before a hole", "CHECKED-JSON-UNSUPPORTED-PROPERTY", () => getter(new Array(2), "0")],
  ["array: hole, before a refusing element", "CHECKED-JSON-SPARSE-ARRAY", () => [, Number.NaN]], // eslint-disable-line no-sparse-arrays
  ["array: refusing element, before a hole", "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE", () => [Number.NaN, , 1]], // eslint-disable-line no-sparse-arrays
  ["array: hidden element, before a refusing element", "CHECKED-JSON-UNSUPPORTED-PROPERTY", () => hidden([1, Number.NaN], "0", 1)],
  ["array: refusing element, before a hidden element", "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE", () => hidden([Number.NaN, 1], "1", 1)],
  ["array: hidden element holding a refusing value", "CHECKED-JSON-UNSUPPORTED-PROPERTY", () => hidden([1], "0", Number.NaN)],
  ["object: non-plain, before a getter", "CHECKED-JSON-NON-PLAIN-OBJECT", () => getter(Object.create({ inherited: 1 }), "a")],
  ["object: non-plain, before a symbol key", "CHECKED-JSON-NON-PLAIN-OBJECT", () => Object.assign(new Invented(), { [Symbol("k")]: 1 })],
  ["object: non-plain, before a refusing value", "CHECKED-JSON-NON-PLAIN-OBJECT", () => Object.assign(Object.create({}), { a: Number.NaN })],
  ["object: getter, before a symbol key (symbol keys come last)", "CHECKED-JSON-UNSUPPORTED-PROPERTY", () => getter({ [Symbol("k")]: 1 }, "a")],
  ["object: refusing value, before a symbol key", "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE", () => ({ [Symbol("k")]: 1, a: Number.NaN })],
  ["object: lone surrogate key, before a getter", "CHECKED-JSON-LONE-SURROGATE", () => getter({ ["\ud800"]: 1 }, "b")],
  ["object: getter, before a lone surrogate key", "CHECKED-JSON-UNSUPPORTED-PROPERTY", () => Object.assign(getter({}, "a"), { ["\ud800"]: 1 })],
  ["object: lone surrogate key holding a refusing value", "CHECKED-JSON-LONE-SURROGATE", () => ({ ["\ud800"]: Number.NaN })],
  ["object: integer-like key first, its lone surrogate before an earlier-inserted refusing value", "CHECKED-JSON-LONE-SURROGATE", () => ({ b: Number.NaN, 1: "\ud800" })],
  ["object: hidden member holding a refusing value", "CHECKED-JSON-UNSUPPORTED-PROPERTY", () => hidden({}, "a", Number.NaN)],
  ["object: refusing value, before a cycle", "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE", () => { const o: Record<string, unknown> = { a: Number.NaN }; o.b = o; return o; }],
  ["object: cycle, before a refusing value", "CHECKED-JSON-CYCLIC-VALUE", () => { const o: Record<string, unknown> = {}; o.a = o; o.b = Number.NaN; return o; }],
  ["object: undefined member, before a function member", "CHECKED-JSON-UNSUPPORTED-TYPE: undefined", () => ({ a: undefined, b: () => 1 })],
  ["object: function member, before an undefined member", "CHECKED-JSON-UNSUPPORTED-TYPE: function", () => ({ a: () => 1, b: undefined })],
  ["object: unsafe integer, before a lone surrogate string", "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE", () => ({ a: 2 ** 53, b: "\ud800" })],
  ["object: lone surrogate string, before an unsafe integer", "CHECKED-JSON-LONE-SURROGATE", () => ({ a: "\ud800", b: 2 ** 53 })]
];

/** `value` wrapped in `levels` objects or arrays. */
function wrapped(g: Gen, value: unknown, levels: number): unknown {
  let out = value;
  for (let i = 0; i < levels; i += 1) out = g.next() < 0.5 ? [out] : { [g.pick(KEYS)]: out };
  return out;
}

function refusing(g: Gen): unknown {
  return g.pick(REFUSING)[1]();
}

/** `value` with `inner` placed at a random depth (in a member or element). */
function nest(g: Gen, inner: unknown): unknown {
  let value = inner;
  for (let depth = g.int(4); depth > 0; depth -= 1) {
    if (g.next() < 0.5) value = [valid(g, 1), value].slice(g.int(2) === 0 ? 0 : 1);
    else value = { [g.pick(KEYS)]: value, ...(g.next() < 0.5 ? { other: valid(g, 1) } : {}) };
  }
  return value;
}

function clone(value: unknown, shuffle: Gen | null, flipZero: boolean): unknown {
  if (Array.isArray(value)) return value.map((item) => clone(item, shuffle, flipZero));
  if (value && typeof value === "object") {
    const keys = Object.keys(value);
    if (shuffle) for (let i = keys.length - 1; i > 0; i -= 1) { const j = shuffle.int(i + 1); [keys[i], keys[j]] = [keys[j], keys[i]]; }
    const out: Record<string, unknown> = Object.getPrototypeOf(value) === null ? Object.create(null) : {};
    for (const key of keys) Object.defineProperty(out, key, { value: clone((value as Record<string, unknown>)[key], shuffle, flipZero), enumerable: true, writable: true, configurable: true });
    return out;
  }
  if (flipZero && typeof value === "number" && value === 0) return Object.is(value, -0) ? 0 : -0;
  return value;
}

/** `value` with one change somewhere: a leaf replaced, a member added or removed, an
 * array lengthened, or an array and an object exchanged. */
function mutate(g: Gen, value: unknown): unknown {
  const copy = clone(value, null, false);
  const containers: Array<Record<string, unknown> | unknown[]> = [];
  const walk = (item: unknown) => {
    if (item && typeof item === "object") { containers.push(item as Record<string, unknown>); for (const child of Object.values(item)) walk(child); }
  };
  walk(copy);
  if (containers.length === 0) return g.pick([leaf(g), [], {}, [copy], { a: copy }]);
  const target = g.pick(containers);
  if (Array.isArray(target)) {
    if (target.length && g.next() < 0.5) target[g.int(target.length)] = g.pick([leaf(g), [], {}]);
    else target.push(leaf(g));
  } else {
    const keys = Object.keys(target);
    const choice = g.int(3);
    if (choice === 0 && keys.length) delete target[g.pick(keys)];
    else if (choice === 1 && keys.length) target[g.pick(keys)] = g.pick([leaf(g), [], {}, null]);
    else target[`added-${g.int(3)}`] = leaf(g);
  }
  return copy;
}

const FIXTURES = resolve(__dirname, "../../../../fixtures/results");
/** Members of the analysis-run fixtures that `validateAnalysisRunV03` compares, by kind. */
function callSiteValues(): Array<[string, unknown]> {
  const out: Array<[string, unknown]> = [];
  for (const name of readdirSync(FIXTURES).filter((file) => file.endsWith(".analysis_run.json")).sort()) {
    const run = JSON.parse(readFileSync(resolve(FIXTURES, name), "utf8")).analysis_run;
    out.push(["model_state_ref", run.model_state_ref], ["load_basis_refs", run.load_basis_refs], ["diagnostics", run.diagnostics], ["semantic_contract", run.reproducibility.semantic_contract],
      ["received hashes", run.hashes.filter((h: { payload_scope: string }) => h.payload_scope === "received_result")]);
    for (const member of ["source_block_recovery", "contract_evidence", "retained_precision"]) if (Object.hasOwn(run, member)) out.push([member, run[member]]);
    for (const row of run.result_refs.slice(0, 6)) {
      out.push(["result_ref", row.result_ref], ["row semantic_contract", row.semantic_contract], ["hash_refs", row.hash_refs], ["interpretation", row.interpretation], ["source_annotation", row.source_annotation]);
      const { metadata, ...absent } = row.source_annotation;
      out.push(["source_annotation, metadata absent", absent], ["source_annotation, metadata null", { ...absent, metadata: null }], ["source_annotation, metadata set", { ...absent, metadata: metadata ?? { invented: [1, -0, "x"] } }]);
    }
  }
  return out;
}

afterEach(() => vi.restoreAllMocks());

describe("sameCheckedJson", () => {
  it("returns what the old text comparison returned, or throws its error, on every generated pair", () => {
    const g = generator(0x5eed1);
    const pairs: Array<[string, unknown, unknown]> = [];
    // Identity, key order, zero sign, one change: valid values against each other.
    for (let i = 0; i < 1500; i += 1) {
      const v = valid(g, 5);
      pairs.push(["same object", v, v], ["clone", v, clone(v, null, false)], ["shuffled", v, clone(v, g, false)], ["zero sign", v, clone(v, g, true)], ["mutated", v, mutate(g, v)], ["mutated left", mutate(g, v), v], ["unrelated", v, valid(g, 3)]);
    }
    // Refusing shapes at depth, on either side and both; the undefined rule before any check.
    for (let i = 0; i < 1500; i += 1) {
      const v = valid(g, 3);
      const bad = nest(g, refusing(g));
      const otherBad = nest(g, refusing(g));
      pairs.push(["refusing left", bad, v], ["refusing right", v, bad], ["refusing both", bad, otherBad], ["refusing same", bad, bad],
        ["undefined left", undefined, g.next() < 0.5 ? bad : v], ["undefined right", g.next() < 0.5 ? bad : v, undefined], ["undefined both", undefined, undefined],
        ["refusing within valid", { a: v, b: bad }, { b: bad, a: v }]);
    }
    for (const [name, make] of REFUSING) pairs.push([`top ${name}`, make(), make()], [`top ${name} vs null`, make(), null], [`null vs top ${name}`, null, make()]);
    // Two checks tripped at once, alone and nested, on either side and against one another.
    for (const [name, , make] of TWO_CHECKS) {
      pairs.push([`two checks ${name}`, make(), make()], [`two checks ${name} left`, make(), valid(g, 2)], [`two checks ${name} right`, valid(g, 2), make()],
        [`two checks ${name} nested left`, nest(g, make()), nest(g, refusing(g))], [`two checks ${name} nested right`, valid(g, 2), nest(g, make())],
        [`two checks ${name} vs another`, make(), g.pick(TWO_CHECKS)[2]()]);
    }
    // Nesting up to 128 levels, on either side: equal, shuffled, changed and refused at the bottom.
    for (let levels = 1; levels <= 128; levels += 1) {
      const seed = generator(levels);
      const v = valid(seed, 2);
      const deepV = wrapped(generator(levels + 1000), v, levels);
      pairs.push([`deep ${levels}`, deepV, clone(deepV, null, false)], [`deep ${levels} shuffled`, deepV, clone(deepV, seed, false)],
        [`deep ${levels} changed`, deepV, wrapped(generator(levels + 1000), mutate(seed, v), levels)],
        [`deep ${levels} refused left`, wrapped(generator(levels + 1000), refusing(seed), levels), deepV],
        [`deep ${levels} refused right`, deepV, wrapped(generator(levels + 1000), g.pick(TWO_CHECKS)[2](), levels)],
        [`deep ${levels} vs shallow`, deepV, v]);
    }
    // The call sites' own values: each against itself, a shuffled copy, a mutated copy and the others.
    const sites = callSiteValues();
    for (const [kind, value] of sites) {
      pairs.push([kind, value, clone(value, null, false)], [`${kind} shuffled`, value, clone(value, g, false)], [`${kind} mutated`, value, mutate(g, value)], [`${kind} vs other`, value, g.pick(sites)[1]]);
    }
    let equal = 0, different = 0, refused = 0;
    const errors = new Set<string>();
    for (const [index, [name, a, b]] of pairs.entries()) {
      const expected = outcome(referenceSame, a, b);
      const actual = outcome(sameCheckedJson, a, b);
      if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`pair ${index} (${name}): reference ${JSON.stringify(expected)}, sameCheckedJson ${JSON.stringify(actual)}`);
      if ("error" in expected) { refused += 1; errors.add(expected.error); } else if (expected.value) equal += 1; else different += 1;
    }
    // 1,500 × 7 valid pairs, 1,500 × 8 refusing pairs, 3 per REFUSING shape, 6 per TWO_CHECKS value,
    // 6 per nesting level 1 to 128, and 4 per fixture member.
    expect(pairs.length).toBe(1500 * 7 + 1500 * 8 + REFUSING.length * 3 + TWO_CHECKS.length * 6 + 128 * 6 + sites.length * 4);
    // The corpus reaches all three outcomes and each error the profile throws.
    expect(equal).toBeGreaterThan(3000);
    expect(different).toBeGreaterThan(3000);
    expect(refused).toBeGreaterThan(3000);
    expect([...errors].map((e) => e.replace(/: (function|symbol|bigint|undefined)$/, "")).sort()).toEqual([
      "Error: CHECKED-JSON-ARRAY-LENGTH",
      "Error: CHECKED-JSON-ARRAY-OWN-PROPERTY",
      "Error: CHECKED-JSON-CYCLIC-VALUE",
      "Error: CHECKED-JSON-LONE-SURROGATE",
      "Error: CHECKED-JSON-NON-PLAIN-OBJECT",
      "Error: CHECKED-JSON-NUMBER-OUTSIDE-PROFILE",
      "Error: CHECKED-JSON-SPARSE-ARRAY",
      "Error: CHECKED-JSON-SYMBOL-KEY",
      "Error: CHECKED-JSON-UNSUPPORTED-PROPERTY",
      "Error: CHECKED-JSON-UNSUPPORTED-TYPE",
      "Error: CHECKED-JSON-UNSUPPORTED-TYPE",
      "Error: CHECKED-JSON-UNSUPPORTED-TYPE",
      "Error: CHECKED-JSON-UNSUPPORTED-TYPE"
    ]);
  });

  it("checks the left side wholly before the right, and the right even when the left is plainly different", () => {
    expect(() => sameCheckedJson({ a: [1, Number.NaN] }, { a: "\ud800" })).toThrow(/^CHECKED-JSON-NUMBER-OUTSIDE-PROFILE$/);
    expect(() => sameCheckedJson({ a: 1 }, [() => 1])).toThrow(/^CHECKED-JSON-UNSUPPORTED-TYPE: function$/);
    expect(() => sameCheckedJson(1, "\udc00")).toThrow(/^CHECKED-JSON-LONE-SURROGATE$/);
    expect(sameCheckedJson(undefined, Number.NaN)).toBe(false);
    expect(sameCheckedJson({ a: { b: [-0, 1] }, c: null }, { c: null, a: { b: [0, 1] } })).toBe(true);
    expect(sameCheckedJson([], {})).toBe(false);
    expect(sameCheckedJson({ kind: "k", unit: "mm" }, { kind: "k", unit: "mm", metadata: null })).toBe(false);
  });

  it("makes no text: no checked text, no JSON serialisation and no parse", () => {
    const [, value] = callSiteValues().find(([kind]) => kind === "diagnostics")!;
    const shuffled = clone(value, generator(7), false);
    const stringify = vi.spyOn(JSON, "stringify");
    const parse = vi.spyOn(JSON, "parse");
    expect(sameCheckedJson(value, shuffled)).toBe(true);
    expect(stringify).not.toHaveBeenCalled();
    expect(parse).not.toHaveBeenCalled();
  });
});

type TextOutcome = { text: string } | { error: string };
function textOutcome(text: (value: unknown) => string, value: unknown): TextOutcome {
  try { return { text: text(value) }; } catch (error) { return { error: `${(error as Error).constructor.name}: ${(error as Error).message}` }; }
}

describe("checkedJsonText, its checks now shared with sameCheckedJson", () => {
  it("gives the old implementation's text, or its error, on the same corpus", () => {
    const g = generator(0x7e47);
    const values: Array<[string, unknown]> = [];
    for (let i = 0; i < 3000; i += 1) values.push(["valid", valid(g, 5)], ["refusing nested", nest(g, refusing(g))]);
    for (const [name, make] of REFUSING) values.push([name, make()], [`${name} nested`, nest(g, make())]);
    for (const [name, , make] of TWO_CHECKS) values.push([name, make()], [`${name} nested`, nest(g, make())]);
    for (let levels = 1; levels <= 128; levels += 1) {
      values.push([`deep ${levels}`, wrapped(generator(levels), valid(generator(levels + 1), 2), levels)], [`deep ${levels} refused`, wrapped(generator(levels), g.pick(TWO_CHECKS)[2](), levels)]);
    }
    for (const [kind, value] of callSiteValues()) values.push([kind, value]);
    let texts = 0, refused = 0;
    for (const [index, [name, value]] of values.entries()) {
      const expected = textOutcome(referenceCheckedJsonTextOf, value);
      const actual = textOutcome(checkedJsonText, value);
      if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`value ${index} (${name}): reference ${JSON.stringify(expected).slice(0, 200)}, checkedJsonText ${JSON.stringify(actual).slice(0, 200)}`);
      if ("error" in expected) refused += 1; else texts += 1;
    }
    expect(texts).toBeGreaterThan(3000);
    expect(refused).toBeGreaterThan(3000);
  });

  it("refuses each two-check value with the profile's first refusal, in the text and in the comparison, on either side", () => {
    for (const [name, error, make] of TWO_CHECKS) {
      const exact = new RegExp(`^${error}$`);
      expect(() => referenceCheckedJsonTextOf(make()), name).toThrow(exact);
      expect(() => checkedJsonText(make()), name).toThrow(exact);
      expect(() => sameCheckedJson(make(), { a: 1 }), name).toThrow(exact);
      expect(() => sameCheckedJson({ a: 1 }, make()), name).toThrow(exact);
      expect(() => sameCheckedJson(make(), "\ud800"), name).toThrow(exact);
    }
  });
});

describe("a known difference outside sameCheckedJson's domain, not repaired", () => {
  // The old comparison serialised with JSON.stringify, which reads a toJSON on Array.prototype or
  // Object.prototype; sameCheckedJson does not. The product patches neither prototype.
  it("a toJSON added to a built-in prototype changes the old comparison's answer only", () => {
    const results: Record<string, boolean> = {};
    for (const [name, prototype, a, b] of [["Array.prototype", Array.prototype, [1], [2]], ["Object.prototype", Object.prototype, { a: 1 }, { a: 2 }]] as const) {
      try {
        Object.defineProperty(prototype, "toJSON", { value: () => "patched", configurable: true, writable: true, enumerable: false });
        results[`${name} old`] = referenceSame(a, b);
        results[`${name} new`] = sameCheckedJson(a, b);
      } finally {
        delete (prototype as { toJSON?: unknown }).toJSON;
      }
    }
    expect(results).toEqual({ "Array.prototype old": true, "Array.prototype new": false, "Object.prototype old": true, "Object.prototype new": false });
  });
});
