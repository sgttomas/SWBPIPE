// Issue 382 (DEL-16-02 REQ-010 and AC-006): an imported proposal reaches every member of the
// stored value/unit quantity family. The real intake panel (OfflineProposalIntakePanel) takes the
// pasted batch, stamps it imported, and queues it into the real workspace session as App.tsx
// binds it; the session's Review Apply route runs the operation applier in the Wasm engine built
// from the candidate source, and the result is read back from the saved and reopened project.
// Requeue of a retained imported record reaches the same engine through the session's requeue
// handler (the handler App.tsx's "Requeue for fresh validation" button calls). The model and every
// value are invented (test-support/unitPairFamilyModel.ts).
import { act, cleanup, fireEvent, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { OfflineProposalIntakePanel } from "./OfflineProposalIntakePanel";
import { stampImportedBatch } from "../../services/operationAuthorship";
import { unitPairFamilyModel } from "../../test-support/unitPairFamilyModel";
import { FAMILY_MEMBERS, familyOperation, pair, type FamilyMember } from "../../test-support/unitPairFamilyMembers";
import { applyQueuedBatch, refusalCodes, renderSurface, storedDocument, type SessionRef } from "../../test-support/unitPairSurfaceHarness";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

async function openIntake() {
  return (await renderSurface(unitPairFamilyModel(), (session) => (
    <OfflineProposalIntakePanel model={session.model.model!} selection={{ type: "project", id: session.model.model!.project.id }}
      onQueueBatch={session.operations.handleQueueOperationBatch} busy={session.operations.operationBusy} requestEpoch={session.operations.requestEpoch} />
  ))).session;
}

/** Paste one operation as an imported batch, queue it, and apply it through Review. */
async function importAndApply(session: SessionRef, operation: ReturnType<typeof familyOperation>) {
  fireEvent.change(screen.getByLabelText("Proposal batch JSON"), { target: { value: JSON.stringify({ batch_id: `batch:${operation.operation_id}`, operations: [operation] }) } });
  fireEvent.click(screen.getByRole("button", { name: "Queue imported batch for review" }));
  expect(screen.queryByRole("alert")).toBeNull();
  const entry = await applyQueuedBatch(session);
  // The queued operation is the imported one, stamped as agent-proposed by intake.
  expect(entry.batch.operations[0]).toMatchObject({ author_type: "agent", change: operation.change });
  return entry;
}

async function expectRefused(member: FamilyMember, after: string, unit: string, dimension: string, token: string, code: string) {
  const session = await openIntake();
  const before = structuredClone(session.current.model.model!);
  const entry = await importAndApply(session, familyOperation(member, after, unit, dimension, token));
  expect(refusalCodes(session, entry.key)).toContain(code);
  expect(session.current.model.model).toEqual(before);
  expect(member.read(await storedDocument(session))).toEqual(member.stored);
}

async function expectApplied(member: FamilyMember, after: string, unit: string, dimension: string, token: string, expected: unknown) {
  const session = await openIntake();
  const entry = await importAndApply(session, familyOperation(member, after, unit, dimension, token));
  expect(session.current.operations.batchOutcomes[entry.key]?.diagnostics ?? []).toEqual([]);
  expect(session.current.operations.batchMessage ?? "").not.toMatch(/not applied/);
  expect(member.read(session.current.model.model!)).toEqual(expected);
  expect(member.read(await storedDocument(session))).toEqual(expected);
}

describe.each(FAMILY_MEMBERS)("imported proposal: $name ($fieldPath)", (member) => {
  it("refuses a bare number with another unit of the field's dimension, and leaves the model unchanged", async () => {
    await expectRefused(member, String(member.otherUnit.value), member.otherUnit.unit, member.dimension, "bare-other", "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
  });

  it("writes the explicit value/unit pair together", async () => {
    await expectApplied(member, pair(member.otherUnit), member.otherUnit.unit, member.dimension, "pair-other", member.otherUnit);
  });

  it("applies a bare number naming the stored unit and keeps that unit", async () => {
    await expectApplied(member, String(member.inStoredUnit), member.stored.unit, member.dimension, "bare-stored",
      { value: member.inStoredUnit, unit: member.stored.unit });
  });

  it("refuses an explicit pair whose unit belongs to another dimension", async () => {
    await expectRefused(member, pair({ value: member.otherUnit.value, unit: member.wrongDimensionUnit }), member.wrongDimensionUnit, member.dimension,
      "pair-wrong-unit", "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
  });

  it("refuses an intent dimension that is not the field's", async () => {
    const { dimension, unit } = member.wrongIntentDimension;
    await expectRefused(member, pair({ value: member.otherUnit.value, unit }), unit, dimension, "intent-dimension", "OP-UNIT-DIMENSION-MISMATCH");
  });
});

describe("requeue of a retained imported record (material elastic modulus, the issue 382 witness)", () => {
  const member = FAMILY_MEMBERS.find((item) => item.fieldPath === "elastic_modulus.value")!;
  /** The record as offline intake stamped and the project stored it. */
  const retained = (after: string, unit: string, token: string) =>
    stampImportedBatch({ batch_id: `batch:${token}`, operations: [familyOperation(member, after, unit, member.dimension, token) as never] }).batch.operations[0];

  async function requeueAndApply(record: unknown) {
    const session = await openIntake();
    const before = structuredClone(session.current.model.model!);
    let refusal: string | null = "not run";
    await act(async () => { refusal = await session.current.operations.handleRequeueRetainedRecord(record, 0); });
    expect(refusal).toBeNull();
    const entry = await applyQueuedBatch(session);
    expect(entry.batch.batch_id).toBe("requeue-0");
    expect(entry.batch.operations[0]).toMatchObject({ author_type: "agent" });
    return { session, before, entry };
  }

  it("refuses the bare 210 with unit GPa against stored Pa, and leaves the model unchanged", async () => {
    const { session, before, entry } = await requeueAndApply(retained("210", "GPa", "requeue-bare"));
    expect(refusalCodes(session, entry.key)).toContain("OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE");
    expect(session.current.model.model).toEqual(before);
    await waitFor(() => expect(member.read(session.current.model.model!)).toEqual({ value: 200000000000, unit: "Pa" }));
  });

  it("writes the explicit pair 210 GPa", async () => {
    const { session } = await requeueAndApply(retained(pair({ value: 210, unit: "GPa" }), "GPa", "requeue-pair"));
    expect(member.read(await storedDocument(session))).toEqual({ value: 210, unit: "GPa" });
  });
});
