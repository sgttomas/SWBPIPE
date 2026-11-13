// The bounds of the shared check on operations not authored by the person
// (operationAuthorship.ts writesUserEnteredModifier, DEL-16-03 REQ-013/AC-007/VER-005):
// the after text's length, read before the text is parsed; the walk's depth and member count,
// with its own storage held inside the member count before any child is read; each bound at
// exactly its value and one past it, at each entry that can reach it. Every number is read
// from PERSON_ENTERED_CHECK. All data are invented.
import { describe, expect, it, vi } from "vitest";
import type { OperationBatch } from "./operationBatchService";
import { findPersonEntered, PERSON_ENTERED_CHECK, requeueRetainedRecord, stampImportedBatch, writesUserEnteredModifier } from "./operationAuthorship";

const { maxDepth, maxMembers, maxAfterChars } = PERSON_ENTERED_CHECK;
const AGENT = { author_type: "agent", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } };
const material = (after: string, id = "op:bound") => ({ operation_id: id, operation_kind: "create", target: { object_type: "Material", ref: "material:m" },
  change: { change_id: "c", change_kind: "create_material", field_label: "m", field_path: "materials", before: "not_present", after, unit: "none", dimension: "dimensionless", source_note: "file" },
  rationale: "invented" });
const intake = (operation: unknown) => () => stampImportedBatch({ batch_id: "file", operations: [operation] } as unknown as OperationBatch);
const requeue = (operation: Record<string, unknown>) => requeueRetainedRecord({ ...operation, ...AGENT }, 3);
/** An array of `members` zeros: exactly that many members below the root. */
const wideArray = (members: number) => `[${"0,".repeat(members - 1)}0]`;
/** An object of `members` keys with scalar values. */
const wideObject = (members: number) => `{${Array.from({ length: members }, (_, i) => `"k${i}":0`).join(",")}}`;
/** A value whose deepest member is `depth` levels below the root (the root is level 0). */
const deepValue = (depth: number) => `${"[".repeat(depth)}0${"]".repeat(depth)}`;
/** A JSON text of exactly `chars` characters (a string literal). */
const textOf = (chars: number) => JSON.stringify("x".repeat(chars - 2));

describe("the numbers are derived in one place", () => {
  it("each bound is the next power of two at or above the factor times the measurement on the criterion 9 rack", () => {
    const next = (n: number) => 2 ** Math.ceil(Math.log2(n));
    expect(PERSON_ENTERED_CHECK.maxMembers).toBe(next(PERSON_ENTERED_CHECK.factor * PERSON_ENTERED_CHECK.measuredEntries));
    expect(PERSON_ENTERED_CHECK.maxAfterChars).toBe(next(PERSON_ENTERED_CHECK.factor * PERSON_ENTERED_CHECK.measuredChars));
    expect([PERSON_ENTERED_CHECK.measuredEntries, PERSON_ENTERED_CHECK.measuredChars, PERSON_ENTERED_CHECK.factor, maxMembers, maxAfterChars, maxDepth])
      .toEqual([25_717, 5_175_300, 4, 131_072, 33_554_432, 32]);
    expect(PERSON_ENTERED_CHECK.atRequeue).toBe(true);
  });
});

describe("the walk's storage is bounded before children are read", () => {
  // A proxy counts each child the walk reads. The walk admits children one at a time against
  // the member count, so it reads at most one past the bound, however wide the value is.
  function counted<T extends object>(target: T, size: number) {
    let reads = 0;
    const proxy = new Proxy(target, { get(object, key, receiver) {
      if (typeof key === "string" && key !== "length" && !(key in Object.prototype) && !(key in Array.prototype)) reads++;
      if (key === "length" && Array.isArray(object)) return size;
      return Reflect.get(object, key, receiver);
    } });
    return { proxy, reads: () => reads };
  }
  it("a wide array is read only up to one past the member count", () => {
    const { proxy, reads } = counted([] as number[], maxMembers * 8);
    const result = findPersonEntered(proxy);
    expect(reads()).toBeLessThanOrEqual(maxMembers + 1);
    expect(result).toEqual({ unchecked: "count" });
  });
  it("a wide object's values are read only up to one past the member count", () => {
    const keys = Array.from({ length: maxMembers * 2 }, (_, i) => `k${i}`);
    const target = Object.fromEntries(keys.map(key => [key, 0]));
    const { proxy, reads } = counted(target, 0);
    const result = findPersonEntered(proxy);
    expect(reads()).toBeLessThanOrEqual(maxMembers + 1);
    expect(result).toEqual({ unchecked: "count" });
  });
  it("the walk's queue holds containers only and stays inside the member count", () => {
    const observed = { queued: 0 };
    expect(findPersonEntered(JSON.parse(`[${"[0],".repeat(maxMembers / 2 - 1)}[0]]`), observed)).toBeNull();
    expect(observed.queued).toBeLessThanOrEqual(maxMembers + 1);
  });
});

describe("each bound at exactly its value and one past, at intake and at requeue", () => {
  const cases: [string, string, string, RegExp][] = [
    ["member count, wide array", wideArray(maxMembers), wideArray(maxMembers + 1), new RegExp(`with more than ${maxMembers} members`)],
    ["member count, wide object", wideObject(maxMembers), wideObject(maxMembers + 1), new RegExp(`with more than ${maxMembers} members`)],
    ["depth", deepValue(maxDepth), deepValue(maxDepth + 1), new RegExp(`nested deeper than ${maxDepth} levels`)],
    ["after text length", textOf(maxAfterChars), textOf(maxAfterChars + 1), new RegExp(`longer than ${maxAfterChars} characters`)],
  ];
  for (const [what, at, past, reason] of cases) {
    it(`${what}: at the bound it is read and passes; one past, it is refused for that reason`, () => {
      expect(writesUserEnteredModifier(material(at))).toBeNull();
      expect(intake(material(at))).not.toThrow();
      expect(requeue(material(at))).toHaveProperty("batch");
      expect(writesUserEnteredModifier(material(past))).toMatchObject({ route: "unchecked" });
      expect(intake(material(past))).toThrow(reason);
      expect(intake(material(past))).toThrow(/^Operation 1 \(op:bound\) of this file carries an after value .*, so nothing was queued\. Split it into smaller operations\.$/);
      const refused = requeue(material(past));
      expect(refused).toEqual({ refused: expect.stringMatching(reason) });
      expect(refused).toEqual({ refused: expect.stringMatching(/It was not queued for review; the record stays stored and nothing of yours is lost\. Prepare the change again in smaller operations\.$/) });
    }, 120_000);
  }
  it("the three reasons are told apart", () => {
    const route = (after: string) => writesUserEnteredModifier(material(after));
    expect(route(wideArray(maxMembers + 1))).toMatchObject({ reason: "count" });
    expect(route(deepValue(maxDepth + 1))).toMatchObject({ reason: "depth" });
    expect(route(textOf(maxAfterChars + 1))).toMatchObject({ reason: "size" });
    expect(route(textOf(maxAfterChars + 1))!.label).not.toMatch(/members|nested/);
  }, 60_000);
  it("the length is read before the text is parsed: a text past it that is not JSON is refused for its length", () => {
    const parse = vi.spyOn(JSON, "parse");
    try {
      expect(writesUserEnteredModifier(material(`{${"x".repeat(maxAfterChars)}`))).toMatchObject({ route: "unchecked", reason: "size" });
      expect(writesUserEnteredModifier(material(textOf(maxAfterChars + 1)))).toMatchObject({ route: "unchecked", reason: "size" });
      expect(parse).not.toHaveBeenCalled();
    } finally { parse.mockRestore(); }
  }, 60_000);
  it("the one place that applies the length at requeue: with no length cap, requeue reads a text past it", () => {
    const past = material(textOf(maxAfterChars + 1));
    expect(writesUserEnteredModifier(past, Number.POSITIVE_INFINITY)).toBeNull();
  }, 60_000);
});

describe("a stored self-weight refresh refused at requeue says how to refresh it from the model", () => {
  it("names the panel's own words", () => {
    const refresh = { operation_id: "op:sw", operation_kind: "modify", target: { object_type: "Load", ref: "load:SW" },
      change: { change_id: "c", change_kind: "update_load", field_label: "Generated self-weight", field_path: "generated_self_weight", before: "[]", after: wideArray(maxMembers + 1), unit: "none", dimension: "dimensionless", source_note: "s" },
      author_type: "user", rationale: "r" };
    expect(requeueRetainedRecord(refresh, 2)).toEqual({ refused: `This stored record is a self-weight refresh that carries an after value with more than ${maxMembers} members, which is not checked for values only the person enters. It was not queued for review; the record stays stored and nothing of yours is lost. Refresh the self-weight from the model instead: in Plan self-weight loads, choose "Refresh an existing case", choose the case, then "Generate self-weight refresh plan".` });
  });
});

describe("the walk's cost at its bounds", () => {
  // A record shaped like the person's self-weight refresh (12 walk entries per generated load),
  // at exactly the member count, and one at exactly the text length. The walk's queue stays
  // inside the member count on each. The time each takes is measured outside the suite (the
  // pull request gives it); this suite asserts storage, not time.
  const load = (i: number) => ({ category: "weight", dimension: "force_per_length", direction: "global_z", id: `load:SW-${i}`,
    magnitude: { unit: "N/m", value: -1070.6475813791571 }, provenance: "generated self-weight, invented", target: { pipe: `pipe:P-${i}`, type: "element" } });
  it("at exactly the member count, a self-weight-shaped record is read whole", () => {
    const loads = Math.floor(maxMembers / 12);
    const text = JSON.stringify([...Array.from({ length: loads }, (_, i) => load(i)), ...Array.from({ length: maxMembers - loads * 12 }, () => 0)]);
    const observed = { queued: 0 };
    expect(findPersonEntered(JSON.parse(text), observed)).toBeNull();
    expect(observed.queued).toBeLessThanOrEqual(maxMembers + 1);
    expect(writesUserEnteredModifier(material(text))).toBeNull();
  }, 60_000);
  it("at exactly the text length, a self-weight-shaped record past the member count is read up to it and refused", () => {
    let text = JSON.stringify(Array.from({ length: Math.floor(maxAfterChars / 400) }, (_, i) => load(i)));
    text = text.slice(0, text.lastIndexOf("},{") + 1);
    text = `${text},${JSON.stringify("x".repeat(maxAfterChars - text.length - 4))}]`;
    expect(text.length).toBe(maxAfterChars);
    const observed = { queued: 0 };
    expect(findPersonEntered(JSON.parse(text), observed)).toEqual({ unchecked: "count" });
    expect(observed.queued).toBeLessThanOrEqual(maxMembers + 1);
    expect(writesUserEnteredModifier(material(text))).toMatchObject({ route: "unchecked", reason: "count" });
  }, 60_000);
});
