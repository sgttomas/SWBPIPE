// DEL-16-03 AC-007: an after value that is not text is read, never passed unread; and no text the
// file or the agent chose appears in a refusal, keys included. The refusal carries the
// operation's identifier, the shape of where the member was found, the member's name and the
// rule or bound that refused. All data are invented.
import { describe, expect, it } from "vitest";
import type { OperationBatch } from "./operationBatchService";
import { PERSON_ENTERED_CHECK, requeueRetainedRecord, stampImportedBatch, writesUserEnteredModifier } from "./operationAuthorship";

const { maxDepth, maxMembers } = PERSON_ENTERED_CHECK;
const ORIGINS = {
  agent: { author_type: "agent", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } },
  imported: { author_type: "user", source: { source_ref: "f", source_channel: "offline_intake", source_role: "imported" } },
  user: { author_type: "user", source: { source_ref: "me", source_channel: "local_desktop_preview", source_role: "gui_editor" } },
} as const;
const op = (after: unknown, over: Record<string, unknown> = {}) => ({ operation_id: "op:x", operation_kind: "create", target: { object_type: "Material", ref: "material:m" },
  change: { change_id: "c", change_kind: "create_material", field_label: "m", field_path: "materials", before: "not_present", after, unit: "none", dimension: "dimensionless", source_note: "s" },
  rationale: "r", ...over });
const intakeMessage = (operation: unknown) => {
  try { stampImportedBatch({ batch_id: "file", operations: [operation] } as unknown as OperationBatch); return null; } catch (error) { return (error as Error).message; }
};
const MODIFIERS = "modifiers (k, SIFs, stiffness and their source)";
const MALFORMED_INTAKE = "Operation 1 (op:x) of this file has an after value that is not text, which the operation engine does not read, so nothing was queued. Write the after value as text in the file.";
const MALFORMED_REQUEUE = "This stored record has an after value that is not text, which the operation engine does not read. It was not queued for review; the record stays stored and nothing of yours is lost. Prepare the change again.";

describe("an after value that is not text", () => {
  it("an object or an array is walked directly: a member in it is named, at intake and at requeue for each origin", () => {
    for (const after of [{ id: "m", modifiers: {} }, [{ modifiers: {} }]]) {
      const where = Array.isArray(after) ? "two levels inside the value, in item 1 of a list" : "at the top level of the value";
      expect(intakeMessage(op(after))).toBe(`Operation 1 (op:x) of this file has a value that names ${MODIFIERS} ${where}, which only you enter. A value naming it cannot be told apart from it, even as text, so nothing was queued. Remove it from the file, then enter the value yourself in the component inspector.`);
      for (const claim of Object.values(ORIGINS)) expect(requeueRetainedRecord(op(after, claim), 1)).toEqual({ refused: expect.stringMatching(/^This stored record has a value that names modifiers/) });
    }
  });
  it("an object or an array is held to the depth and member bounds", () => {
    let deep: unknown = 0;
    for (let i = 0; i <= maxDepth; i++) deep = [deep];
    expect(intakeMessage(op(deep))).toMatch(`nested deeper than ${maxDepth} levels`);
    expect(requeueRetainedRecord(op(Array.from({ length: maxMembers + 1 }, () => 0), ORIGINS.user), 1)).toEqual({ refused: expect.stringMatching(`with more than ${maxMembers} members`) });
  });
  it("an object, an array, a number, a boolean or null with no member is refused as an after value the engine does not read", () => {
    for (const after of [{ id: "m" }, [1, 2], 5, true, null]) {
      expect(writesUserEnteredModifier(op(after)), JSON.stringify(after)).toMatchObject({ route: "malformed" });
      expect(intakeMessage(op(after))).toBe(MALFORMED_INTAKE);
      for (const claim of Object.values(ORIGINS)) expect(requeueRetainedRecord(op(after, claim), 1)).toEqual({ refused: MALFORMED_REQUEUE });
    }
  });
  it("an operation with no after member at all is left for the engine, which refuses it as incomplete", () => {
    expect(writesUserEnteredModifier({ operation_id: "op:x", change: { change_kind: "create_material" } })).toBeNull();
  });
});

describe("a forbidden member is recognised before it is counted", () => {
  it("at the first member past the count, the member is named, not the bound", () => {
    const keys = Array.from({ length: maxMembers }, (_, i) => `"k${i}":0`).join(",");
    expect(writesUserEnteredModifier(op(`{${keys},"modifiers":{}}`))).toMatchObject({ member: "modifiers", route: "dropped" });
  });
  it("at the first level past the depth, the member is named, not the bound", () => {
    const text = `${"[".repeat(maxDepth)}{"modifiers":0}${"]".repeat(maxDepth)}`; // the member is maxDepth + 1 levels down
    expect(writesUserEnteredModifier(op(text))).toMatchObject({ member: "modifiers", route: "dropped" });
  });
});

describe("no text chosen by the file or the agent appears in a refusal", () => {
  const SECRET = "Zq7SECRETPERSONTEXTxv";
  const cases: [string, unknown, string | null][] = [
    ["a secret key above the member", { [SECRET]: { modifiers: {} } }, "two levels inside the value"],
    ["a secret nested key in a list", { a: [{ [SECRET]: { bend_realization: 1 } }] }, "four levels inside the value, in item 1 of a list"],
    ["nested lists", [[0, { [SECRET]: [{ modifiers: 1 }] }]], "five levels inside the value, in items 1, 2 and 1 of nested lists, outermost first"],
    ["a key that looks like a member name", { modifiers_note: SECRET, [`modifiers_${SECRET}`]: 1, label: "M" }, null],
    ["a member name where it is not the member found", { mechanics_interface: { [SECRET]: SECRET, solver_consumption: "not_solver_consumed" }, x: { [SECRET]: { modifiers: 1 } } }, "three levels inside the value"],
  ];
  for (const [what, after, where] of cases) it(what, () => {
    const text = JSON.stringify(after);
    const found = writesUserEnteredModifier(op(text));
    if (where === null) { expect(found).toBeNull(); return; }
    expect(found!.label).toContain(where);
    const messages = [intakeMessage(op(text)), ...Object.values(ORIGINS).map(claim => JSON.stringify(requeueRetainedRecord(op(text, claim), 1)))];
    for (const message of messages) { expect(message).not.toBeNull(); expect(message).not.toContain(SECRET); expect(message).toContain(where); }
  });
  it("a component modifier field path and a library record key are named by a fixed label", () => {
    const field = { ...op("1"), operation_kind: "modify", target: { object_type: "Component", ref: "component:c" }, change: { ...op("1").change, change_kind: "set_field", field_path: `modifiers.${SECRET}` } };
    const library = { ...op(JSON.stringify({ [`modifiers.${SECRET}`]: 1 })), operation_kind: "modify", target: { object_type: "Component", ref: "component:c" }, change: { ...op("1").change, change_kind: "apply_library_record", after: JSON.stringify({ [`modifiers.${SECRET}`]: 1 }) } };
    for (const operation of [field, library]) {
      expect(writesUserEnteredModifier(operation)).toMatchObject({ route: "refused", label: "a component modifier (k, a SIF, a stiffness or their source)" });
      expect(intakeMessage(operation)).not.toContain(SECRET);
      for (const claim of Object.values(ORIGINS)) expect(JSON.stringify(requeueRetainedRecord({ ...operation, ...claim }, 1))).not.toContain(SECRET);
    }
  });
});
