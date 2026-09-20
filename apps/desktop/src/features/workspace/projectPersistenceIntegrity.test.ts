import { describe, expect, it, vi } from "vitest";
import { canPublishPersistenceObservation, ownsOpenPersistenceObservation, deriveModelHashIntegrity, deriveProjectEnvelopeHashIntegrity } from "./projectPersistenceIntegrity";
import type { ModelHashEvidence } from "../../types";
const model: ModelHashEvidence = { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "model_payload", payload_ref: "project:test", value: `sha256:${"a".repeat(64)}`, hash_status: "computed_local_preview" };
describe("B3B persisted snapshot observations", () => {
  it("records save verification completion time and source without backend metadata", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-09-20T12:00:00Z"));
    try {
      const evidence = deriveModelHashIntegrity(model, model, model.payload_ref, "save");
      expect(evidence).toMatchObject({ integrity_status: "verified_match", verification_source: "save", observed_at: "2026-09-20T12:00:00.000Z", verification_basis: "recomputed_at_save_from_returned_model" });
    } finally { vi.useRealTimers(); }
  });
  it("rejects identical values with a wrong claim identity", () => {
    expect(deriveModelHashIntegrity({ ...model, payload_ref: "project:other" }, model, model.payload_ref, "create").integrity_status).toBe("mismatch_review_required");
  });
  it("retains canonical mismatch for a Historical carrier and absence for legacy claims", () => {
    expect(deriveModelHashIntegrity({ ...model, value: "historical" }, model, model.payload_ref, "save").integrity_status).toBe("mismatch_review_required");
    expect(deriveModelHashIntegrity(null, model, model.payload_ref, "save").integrity_status).toBe("not_persisted");
    expect(deriveModelHashIntegrity(model, null, model.payload_ref, "save").integrity_status).toBe("hash_recompute_unavailable");
    expect(deriveProjectEnvelopeHashIntegrity(null, null, model.payload_ref, "save").verification_source).toBe("save");
  });
});

// Production publication guard: UI epochs deliberately do not enter this contract.
it("orders fulfilled observations and rejects an old generation even with the same project ID", () => {
  expect(canPublishPersistenceObservation(2, 2, 2, 1)).toBe(true);
  expect(canPublishPersistenceObservation(2, 2, 1, 2)).toBe(false);
  expect(canPublishPersistenceObservation(1, 2, 3, 2)).toBe(false);
  expect(canPublishPersistenceObservation(2, 2, 2, 2)).toBe(true);
});

import { isLocalModelEdited, verifiedWriteBasis, type PersistedModelBasis } from "./projectPersistenceIntegrity";
import { loadPreviewModel } from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";

it.each([true, false])("preserves saved canonical basis under reversed verification completion (later valid=%s)", async (laterValid) => {
  const a = await loadPreviewModel();
  const b = structuredClone(a);
  b.nodes[0].position.y += 0.5;
  const hashA = (await computeModelHash(a))!;
  const hashB = (await computeModelHash(b))!;
  let basis: PersistedModelBasis | null = null;
  let latest = 0;
  const complete = (ordinal: number, verified: boolean, hash: ModelHashEvidence, generation = 2) => {
    const next = verifiedWriteBasis(verified, generation, 2, ordinal, latest, -1, hash, "save");
    if (next) { basis = next; latest = ordinal; }
  };
  // B lands first but its canonical computation finishes after A's.
  complete(2, laterValid, hashA);
  complete(1, true, hashB);
  expect(isLocalModelEdited(basis, 2, 9, true, hashA)).toBe(!laterValid); // Undo to A
  expect(isLocalModelEdited(basis, 2, 10, true, hashB)).toBe(laterValid); // Redo to B
  const retained = basis;
  complete(3, true, hashA, 1); // Same ID from a replaced session cannot publish.
  expect(basis).toBe(retained);
  expect(isLocalModelEdited(basis, 2, 11, false, hashB)).toBe(true); // Delayed live hash is not owned.
});


it("keeps Open snapshot ownership separate from UI epoch, rejecting replacement and newer evidence", () => {
  const captured = { request: 2, generation: 3, projectId: "same-id", basisSequence: 4 };
  expect(ownsOpenPersistenceObservation(captured, { ...captured })).toBe(true);
  for (const next of [
    { ...captured, request: 3 },
    { ...captured, generation: 4 },
    { ...captured, projectId: "different-id" },
    { ...captured, basisSequence: 5 }
  ]) expect(ownsOpenPersistenceObservation(captured, next)).toBe(false);
});
