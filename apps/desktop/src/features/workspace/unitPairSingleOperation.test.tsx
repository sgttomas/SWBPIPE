// Issue 382 (DEL-16-02 REQ-010 and AC-006): the stored value/unit rule on the session's
// single-operation route and in its history. The real workspace session validates and applies
// one operation through the operation service, which outside the native application calls the
// Wasm engine's single-operation exports (validate_operation_json, apply_operation_json) built
// from the candidate source; nothing in that path is mocked. A refused edit leaves the document
// and both history stacks as they were, and undo and redo restore whole documents, so neither
// can leave a number beside another unit. The model and every value are invented
// (test-support/unitPairFamilyModel.ts).
import { act, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent } from "../../types";
import { unitPairFamilyModel } from "../../test-support/unitPairFamilyModel";
import { FAMILY_MEMBERS, familyOperation, pair, type FamilyMember } from "../../test-support/unitPairFamilyMembers";
import { renderSurface, storedDocument, type SessionRef } from "../../test-support/unitPairSurfaceHarness";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const MISMATCH = "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE";
const open = async () => (await renderSurface(unitPairFamilyModel(), () => null)).session;
const bare = (member: FamilyMember, token: string) =>
  familyOperation(member, String(member.otherUnit.value), member.otherUnit.unit, member.dimension, token) as unknown as EditorOperationIntent;
const paired = (member: FamilyMember, token: string) =>
  familyOperation(member, pair(member.otherUnit), member.otherUnit.unit, member.dimension, token) as unknown as EditorOperationIntent;

/** The outcome the session holds for the one operation it was given. */
function outcomeOf(session: SessionRef) {
  const outcomes = Object.values(session.current.operations.operationOutcomes);
  expect(outcomes).toHaveLength(1);
  return outcomes[0];
}
const blocking = (session: SessionRef) => outcomeOf(session).diagnostics.filter((item) => item.severity === "blocking").map((item) => item.code);
const history = (session: SessionRef) => [session.current.operations.undoStack.length, session.current.operations.redoStack.length];
/** Whether any outcome the session holds carries the blocking unit refusal: after an applied pair, only a later refused edit can. */
const refusedOnUnit = (session: SessionRef) =>
  Object.values(session.current.operations.operationOutcomes).some((outcome) => outcome.diagnostics.some((item) => item.severity === "blocking" && item.code === MISMATCH));

describe.each(FAMILY_MEMBERS)("single operation: $name ($fieldPath)", (member) => {
  it("validation refuses a bare number with another unit of the field's dimension", async () => {
    const session = await open();
    const before = structuredClone(session.current.model.model!);
    await act(async () => { await session.current.operations.handleValidateIntent(bare(member, "validate-bare")); });
    expect(blocking(session)).toContain(MISMATCH);
    expect(outcomeOf(session).applied_model ?? null).toBeNull();
    expect(session.current.model.model).toEqual(before);
    expect(history(session)).toEqual([0, 0]);
  });

  it("apply refuses it, writes nothing and adds no history", async () => {
    const session = await open();
    const before = structuredClone(session.current.model.model!);
    let applied: boolean | undefined;
    await act(async () => { applied = await session.current.operations.handleApplyIntent(bare(member, "apply-bare")); });
    expect(applied).toBe(false);
    expect(blocking(session)).toContain(MISMATCH);
    expect(session.current.model.model).toEqual(before);
    expect(history(session)).toEqual([0, 0]);
    expect(session.current.operations.appliedOperations).toHaveLength(0);
    expect(member.read(await storedDocument(session))).toEqual(member.stored);
  });
});

describe("history around a refused edit (material elastic modulus, the issue 382 witness)", () => {
  const member = FAMILY_MEMBERS.find((item) => item.fieldPath === "elastic_modulus.value")!;

  it("a refusal after an applied pair keeps the one checkpoint; undo and redo restore whole pairs", async () => {
    const session = await open();
    let applied: boolean | undefined;
    await act(async () => { applied = await session.current.operations.handleApplyIntent(paired(member, "pair")); });
    expect(applied).toBe(true);
    expect(member.read(session.current.model.model!)).toEqual({ value: 210, unit: "GPa" });
    expect(history(session)).toEqual([1, 0]);

    // A bare number naming the unit the document held before the pair was applied: 210 GPa is
    // stored now, so "200000000000" with unit Pa is refused and nothing moves.
    const stale = familyOperation(member, "200000000000", "Pa", member.dimension, "bare-old-unit") as unknown as EditorOperationIntent;
    (stale.change as { before: string }).before = "210";
    expect(refusedOnUnit(session)).toBe(false);
    await act(async () => { applied = await session.current.operations.handleApplyIntent(stale); });
    expect(applied).toBe(false);
    expect(refusedOnUnit(session)).toBe(true);
    expect(member.read(session.current.model.model!)).toEqual({ value: 210, unit: "GPa" });
    expect(history(session)).toEqual([1, 0]);

    await act(async () => { session.current.operations.handleUndoSessionModelEdit(); });
    expect(member.read(session.current.model.model!)).toEqual({ value: 200000000000, unit: "Pa" });
    expect(history(session)).toEqual([0, 1]);

    // A refused edit between undo and redo leaves the redo stack as it was: the issue's own bare
    // 210 with unit GPa against the restored 200000000000 Pa.
    await act(async () => { applied = await session.current.operations.handleApplyIntent(bare(member, "bare-between")); });
    expect(applied).toBe(false);
    expect(refusedOnUnit(session)).toBe(true);
    expect(member.read(session.current.model.model!)).toEqual({ value: 200000000000, unit: "Pa" });
    expect(history(session)).toEqual([0, 1]);

    await act(async () => { session.current.operations.handleRedoSessionModelEdit(); });
    expect(member.read(session.current.model.model!)).toEqual({ value: 210, unit: "GPa" });
    expect(history(session)).toEqual([1, 0]);
    expect(member.read(await storedDocument(session))).toEqual({ value: 210, unit: "GPa" });
  });
});
