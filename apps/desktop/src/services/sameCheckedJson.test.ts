// `sameCheckedJson` (analysisRunCompatibility.ts), the equality `validateAnalysisRunV03`
// compares its source bindings by, is the relation it replaces: the checked text of each
// side (hashService's `checkedJsonText`, the left side wholly first), parsed, member-sorted
// and serialised again, then compared. That relation is kept below as the reference and
// both are run over a generated corpus: every refusing shape of the checked profile, on
// either side and at depth, and pairs drawn from the analysis-run fixtures' own members.
// They must agree on every pair: the same boolean, or the same error.
import { readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { sameCheckedJson } from "./analysisRunCompatibility";
import { checkedJsonText } from "./hashService";

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
const NUMBERS = [0, -0, 1, -1, 1.5, 0.1, 0.30000000000000004, 1e-7, 5e-324, 123456.789, 2 ** 53 - 1, -(2 ** 53 - 1), 1e20 / 3];

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

/** Every shape the checked profile refuses, each as a fresh value. */
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
  ["typed array", () => new Uint8Array(2)]
];

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
  it("is the serialise-and-parse relation on a generated corpus, refusing shapes included", () => {
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
    expect(pairs.length).toBeGreaterThan(23000);
    // The corpus reaches every outcome and every refusal the profile names.
    expect(equal).toBeGreaterThan(3000);
    expect(different).toBeGreaterThan(3000);
    expect(refused).toBeGreaterThan(3000);
    expect([...errors].map((e) => e.replace(/: (function|symbol|bigint|undefined)$/, "")).sort()).toEqual([
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
