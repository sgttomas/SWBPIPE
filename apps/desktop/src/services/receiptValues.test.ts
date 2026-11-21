/** The full-copy receipt's values test: for every result in the corpora the desktop's reader
 * tests read, the new path's values equal the old path's, computed in this test by main's
 * functions (`test-support/receiptBaseline.ts`, copied from main unchanged) and by an
 * independent SHA-256 (`node:crypto`):
 * - the fingerprint's text written in pieces is main's fingerprint text, and its equality
 *   digest is the SHA-256 of that text;
 * - the reader's written text written in pieces is main's written text, and its digest is
 *   the SHA-256 of that text;
 * - the checked hash of the canonical text (validation's end) is the checked hash of the
 *   value (main's `canonicalSha256HexCheckedV1`), and one throws exactly when the other does;
 * - after a change, two digests are equal exactly when main's two texts are.
 * Nothing is recorded; nothing here is an expected value. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { baseReaderInputs } from "../test-support/baseReaderInputs";
import { retainedReaderInputs } from "../test-support/retainedPrecisionReaderInputs";
import { baselineNativeContentFingerprintOf, baselineWrittenText } from "../test-support/receiptBaseline";
import { EqualityDigest } from "./equalityDigest";
import { canonicalSha256HexCheckedV1 } from "./hashService";
import { jsonTextKeepingNegativeZero, writeJsonTextKeepingNegativeZero } from "./jsonText";
import { nativeContentDigest, writeCheckedCanonicalJson, writeNativeContentFingerprint } from "./previewService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../");
const sha256 = (text: string) => createHash("sha256").update(Buffer.from(text, "utf8")).digest("hex");
const outcome = <T>(run: () => T): { value: T } | { error: string } => {
  try { return { value: run() }; } catch (error) { return { error: error instanceof Error ? `${error.name}: ${error.message}` : String(error) }; }
};
const joined = (write: (put: (piece: string) => void) => unknown): string => {
  const pieces: string[] = [];
  write((piece) => pieces.push(piece));
  return pieces.join("");
};
/** Validation's checked hash as the new path computes it: the canonical text written once,
 * read by `canonicalSha256HexCheckedV1` in place of the checked text. */
async function newPathHash(value: unknown): Promise<{ value: string } | { error: string }> {
  let text: string;
  try { text = joined((put) => writeCheckedCanonicalJson(value, put)); } catch (error) { return { error: (error as Error).message }; }
  return canonicalSha256HexCheckedV1(value, text).then((hex) => ({ value: hex }), (error: Error) => ({ error: error.message }));
}
const digestOf = (write: (put: (piece: string) => void) => unknown): string => {
  const digest = new EqualityDigest();
  write((piece) => digest.put(piece));
  return digest.hex();
};

/** Values JSON has, whose text depends on what the old path did: negative zeros at depth,
 * keys out of sorted order, array-index keys, keys `localeCompare` ties, `__proto__`,
 * non-ASCII keys and strings, and numbers whose JSON form matters. */
function craftedValues(): [string, Json][] {
  const numbers = [0, -0, 1e21, 1e-7, -1e-7, 5e-324, 0.1 + 0.2, 1.7976931348623157e308, 123456789012345680000, 9007199254740991, -9007199254740991, 2 ** -1074, 1 / 3];
  return [
    ["numbers", { numbers, nested: [[-0, [-0]], { z: -0 }] }],
    ["keys out of order", { zeta: 1, alpha: { y: 2, b: -0, a: [3, -0] }, Mid: true, mid: false, _x: null }],
    ["array-index keys", { b: 1, "10": "ten", "2": "two", "4294967295": "not an index", "4294967294": "last index", "01": "not an index", "0": -0 }],
    ["localeCompare ties", { "é": 1, "é": 2, "a\u0000": 3, a: 4, "x​": 5, x: 6 }],
    ["localeCompare ties, other order", { "é": 2, "é": 1, a: 4, "a\u0000": 3, x: 6, "x​": 5 }],
    ["__proto__ member", JSON.parse('{"__proto__":{"b":1,"a":-0},"constructor":"c","toJSON":"not a function"}')],
    ["paths needing escapes", { "a/b": { "~c": -0, "~/": [-0] }, "": { "": -0 } }],
    ["non-ASCII", { "中文": "\u{1f600}é  ", emoji: ["\u{1f600}", "\u0007\u001f\"\\"], "\u{1f600}": -0 }],
    ["null prototype", Object.assign(Object.create(null), { b: -0, a: 1 })],
  ];
}

/** Values the profile refuses, for the throw comparison: same error from both paths. */
function refusedValues(): [string, Json][] {
  const cyclic: Json = { a: 1 }; cyclic.self = cyclic;
  const sparse: Json[] = [1, , 3]; // eslint-disable-line no-sparse-arrays
  return [
    ["NaN", { a: Number.NaN }], ["Infinity", [1, Number.POSITIVE_INFINITY]], ["unsafe integer", { n: 2 ** 53 }],
    ["lone surrogate value", { s: "\ud800" }], ["lone surrogate key", { "\udc00": 1 }], ["undefined member", { a: undefined }],
    ["function", { f: () => 1 }], ["cyclic", cyclic], ["sparse", sparse], ["date", { d: new Date(0) }],
    ["symbol key", { [Symbol("s")]: 1 }], ["getter", Object.defineProperty({}, "g", { get: () => 1, enumerable: true })],
  ];
}

/** Values JSON has no text for, for the reader's written form (main writes them its own way). */
function writtenQuirks(): [string, Json][] {
  const sparse: Json[] = [1, , -0]; // eslint-disable-line no-sparse-arrays
  return [
    ["undefined member", { a: undefined, b: -0 }], ["function member", { f: () => 1, g: Symbol("s") }],
    ["array elements with no text", [undefined, () => 1, Symbol("s"), null, -0]], ["hole", sparse],
    ["top-level undefined", undefined], ["top-level function", () => 1], ["string", " x"], ["null", null], ["-0", -0],
  ];
}

/** Changes after receipt: each may or may not change a text; main's texts decide. */
const MUTATIONS: [string, (value: Json) => boolean][] = [
  ["a number to -0", (v) => mutateFirst(v, (o, k) => typeof o[k] === "number" && !Object.is(o[k], -0), (o, k) => { o[k] = -0; })],
  ["a -0 to 0", (v) => mutateFirst(v, (o, k) => Object.is(o[k], -0), (o, k) => { o[k] = 0; })],
  ["a member moved last", (v) => mutateFirst(v, (o, k) => !Array.isArray(o) && Object.keys(o).length > 1 && Object.keys(o)[0] === k, (o, k) => { const item = o[k]; delete o[k]; o[k] = item; })],
  ["a string changed", (v) => mutateFirst(v, (o, k) => typeof o[k] === "string", (o, k) => { o[k] = `${o[k]}.`; })],
  ["a member added", (v) => { if (!v || typeof v !== "object" || Array.isArray(v)) return false; v.added_after_receipt = 1; return true; }],
];
function mutateFirst(value: Json, match: (owner: Json, key: string) => boolean, change: (owner: Json, key: string) => void): boolean {
  const stack: Json[] = [value];
  while (stack.length) {
    const owner = stack.pop();
    if (!owner || typeof owner !== "object") continue;
    for (const key of Object.keys(owner)) {
      if (match(owner, key)) { change(owner, key); return true; }
      stack.push(owner[key]);
    }
  }
  return false;
}

/** A deep copy with the same own members, prototypes (plain or null) and negative zeros:
 * `structuredClone` would make an own `__proto__` member a prototype. */
function faithfulCopy(value: Json): Json {
  if (Array.isArray(value)) return value.map(faithfulCopy);
  if (value === null || typeof value !== "object") return value;
  const copy = Object.getPrototypeOf(value) === null ? Object.create(null) : {};
  for (const key of Object.keys(value)) Object.defineProperty(copy, key, { value: faithfulCopy(value[key]), enumerable: true, writable: true, configurable: true });
  return copy;
}
async function corpus(): Promise<{ id: string; value: Json }[]> {
  const entries: { id: string; value: Json }[] = [];
  const base = (await baseReaderInputs()).filter((input) => input.parsed);
  for (const input of base) entries.push({ id: `base/${input.id}`, value: JSON.parse(input.sourceJson) });
  for (const input of retainedReaderInputs()) {
    const { source } = await input.build();
    entries.push({ id: `retained/${input.key}`, value: structuredClone(source) });
  }
  for (const path of ["fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json", "fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json"]) {
    entries.push({ id: path, value: JSON.parse(readFileSync(resolve(ROOT, path), "utf8")) });
  }
  for (const [id, value] of craftedValues()) entries.push({ id: `crafted/${id}`, value });
  return entries;
}

describe("the full-copy receipt's values: the new path's equal main's", () => {
  it("over the corpora: fingerprint text and digest, written text and digest, and the checked hash", async () => {
    const entries = await corpus();
    // The corpora the desktop's reader tests read: the shared base corpus's parsed statements,
    // the shared retained corpus, the two bundled preview results, and the crafted values.
    expect(entries.filter((e) => e.id.startsWith("base/")).length).toBeGreaterThan(400);
    expect(entries.filter((e) => e.id.startsWith("retained/")).length).toBeGreaterThan(10);
    let negativeZeroResults = 0, refused = 0;
    for (const { id, value } of entries) {
      const old = outcome(() => baselineNativeContentFingerprintOf(value));
      if ("error" in old) {
        // A corpus statement outside the checked profile: both paths refuse it alike.
        refused += 1;
        expect(outcome(() => joined((put) => writeNativeContentFingerprint(value, put))), id).toEqual(old);
        expect(outcome(() => nativeContentDigest(value)), id).toEqual(old);
      } else {
        if (old.value.includes("negative_zero_paths=[\"")) negativeZeroResults += 1;
        expect(joined((put) => writeNativeContentFingerprint(value, put)), id).toBe(old.value);
        expect(nativeContentDigest(value), id).toBe(sha256(old.value));
      }
      const written = outcome(() => baselineWrittenText(value));
      if ("error" in written) expect(outcome(() => joined((put) => writeJsonTextKeepingNegativeZero(value, put))), id).toEqual(written);
      else {
        expect(joined((put) => writeJsonTextKeepingNegativeZero(value, put)), id).toBe(written.value);
        expect(digestOf((put) => writeJsonTextKeepingNegativeZero(value, put)), id).toBe(sha256(written.value));
      }
      const oldHash = await canonicalSha256HexCheckedV1(value).then((hex) => ({ value: hex }), (error: Error) => ({ error: error.message }));
      expect(await newPathHash(value), id).toEqual(oldHash);
    }
    expect(negativeZeroResults).toBeGreaterThan(3);
    expect(refused).toBeLessThan(entries.length / 10);
  }, 600_000);

  it("after each change: two digests are equal exactly when main's two texts are", async () => {
    const entries = await corpus();
    let equal = 0, unequal = 0;
    // Every crafted and bundled value, and every tenth corpus statement (each mutation re-runs
    // main's whole fingerprint, so the full corpus is left to the test above).
    const chosen = entries.filter((e, i) => !e.id.startsWith("base/") && !e.id.startsWith("retained/") || i % 10 === 0);
    for (const { id, value } of chosen) {
      if ("error" in outcome(() => baselineNativeContentFingerprintOf(value))) continue;
      const beforeFingerprint = baselineNativeContentFingerprintOf(value), beforeWritten = baselineWrittenText(value);
      const fingerprintDigest = nativeContentDigest(value), writtenDigest = digestOf((put) => writeJsonTextKeepingNegativeZero(value, put));
      for (const [label, mutate] of MUTATIONS) {
        const changed = faithfulCopy(value);
        if (!mutate(changed)) continue;
        // A change can leave the profile (moving a `__proto__` member assigns a prototype):
        // then both paths refuse it alike.
        const changedFingerprint = outcome(() => baselineNativeContentFingerprintOf(changed));
        if ("error" in changedFingerprint) { expect(outcome(() => nativeContentDigest(changed)), `${id}: ${label}`).toEqual(changedFingerprint); continue; }
        const sameFingerprint = changedFingerprint.value === beforeFingerprint;
        expect(nativeContentDigest(changed) === fingerprintDigest, `${id}: ${label}`).toBe(sameFingerprint);
        const sameWritten = baselineWrittenText(changed) === beforeWritten;
        expect(digestOf((put) => writeJsonTextKeepingNegativeZero(changed, put)) === writtenDigest, `${id}: ${label}`).toBe(sameWritten);
        if (sameFingerprint) equal += 1; else unequal += 1;
      }
    }
    // A member moved last leaves the fingerprint equal and the written text not: both decisions occur.
    expect(equal).toBeGreaterThan(0);
    expect(unequal).toBeGreaterThan(0);
  }, 600_000);

  it("refuses what main refuses, with the same error, and the checked hash throws where it threw", async () => {
    for (const [id, value] of refusedValues()) {
      const old = outcome(() => baselineNativeContentFingerprintOf(value));
      expect("error" in old, id).toBe(true);
      expect(outcome(() => joined((put) => writeNativeContentFingerprint(value, put))), id).toEqual(old);
      expect(outcome(() => nativeContentDigest(value)), id).toEqual(old);
      const oldHash = await canonicalSha256HexCheckedV1(value).then(() => "passed", (error: Error) => error.message);
      const newHash = await newPathHash(value);
      expect("error" in newHash ? newHash.error : "passed", id).toBe(oldHash);
    }
    // A depth the Rust checked profile refuses (serde_json's recursion limit) and the page admits.
    let deep: Json = 1;
    for (let i = 0; i < 200; i += 1) deep = [deep];
    const oldDeep = await canonicalSha256HexCheckedV1(deep).then(() => "passed", (error: Error) => error.message);
    expect(oldDeep).not.toBe("passed");
    expect(await newPathHash(deep)).toEqual({ error: oldDeep });
    expect(nativeContentDigest(deep)).toBe(sha256(baselineNativeContentFingerprintOf(deep)));
  });

  it("writes the reader's written form of values JSON has no text for as main does", () => {
    for (const [id, value] of writtenQuirks()) {
      const old = outcome(() => jsonTextKeepingNegativeZero(value));
      const pieces: string[] = [];
      const wrote = outcome(() => writeJsonTextKeepingNegativeZero(value, (piece) => pieces.push(piece)));
      if ("error" in old) { expect(wrote, id).toEqual(old); continue; }
      if (old.value === undefined) { expect(wrote, id).toEqual({ value: false }); expect(pieces, id).toEqual([]); continue; }
      expect(wrote, id).toEqual({ value: true });
      expect(pieces.join(""), id).toBe(old.value);
    }
    expect(outcome(() => writeJsonTextKeepingNegativeZero({ n: 1n }, () => undefined))).toEqual(outcome(() => jsonTextKeepingNegativeZero({ n: 1n })));
  });

  it("digests a text the same whatever pieces it is fed in, and different texts differently", () => {
    const text = "{\"\u{1f600}\":\"\ud800é\",\"x\":[-0,1e21]}\udc00";
    const whole = new EqualityDigest(); whole.put(text);
    const units = new EqualityDigest(); for (const unit of text.split("")) units.put(unit);
    const empty = new EqualityDigest(); empty.put(""); empty.put(text.slice(0, 3)); empty.put(""); empty.put(text.slice(3));
    const expected = whole.hex();
    expect(units.hex()).toBe(expected);
    expect(empty.hex()).toBe(expected);
    // The well-formed text's digest is SHA-256 of its UTF-8.
    const wellFormed = "{\"\u{1f600}\":\"é中\",\"x\":[-0,1e21]}";
    const digest = new EqualityDigest(); digest.put(wellFormed);
    expect(digest.hex()).toBe(sha256(wellFormed));
    // A lone surrogate is not its replacement character, nor any other text.
    const lone = new EqualityDigest(); lone.put("\ud800");
    const replaced = new EqualityDigest(); replaced.put("�");
    expect(lone.hex()).not.toBe(replaced.hex());
    for (const length of [0, 55, 56, 63, 64, 65, 119, 120, 128]) {
      const filler = "a".repeat(length);
      const d = new EqualityDigest(); d.put(filler);
      expect(d.hex(), String(length)).toBe(sha256(filler));
    }
  });
});
