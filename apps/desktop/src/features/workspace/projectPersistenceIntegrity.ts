// Integrity checks on what the local project store returns: session
// observations of opened or returned persisted bytes. The store never rewrites
// a model document (DEL-02-05 CLM-003), so a create or save response whose
// model differs from the request is refused by the session. No React.

import type {
  ModelHashEvidence,
  ModelHashIntegrityEvidence,
  ProjectEnvelopeHashEvidence,
  ProjectEnvelopeHashIntegrityEvidence
} from "../../types";

export function deriveModelHashIntegrity(
  storedHash: ModelHashEvidence | null,
  recomputedHash: ModelHashEvidence | null,
  payloadRef: string,
  source: "open" | "save" | "create" = "open",
  observedAt = new Date().toISOString(),
  responseValid = true
): ModelHashIntegrityEvidence {
  if (!storedHash) {
    return {
      integrity_status: "not_persisted",
      persisted_value: "not_persisted",
      recomputed_value: recomputedHash?.value ?? "unavailable",
      payload_ref: payloadRef,
      verification_basis: source === "open" ? "recomputed_on_open_from_restored_model" : `recomputed_at_${source}_from_returned_model`,
      verification_source: source, observed_at: observedAt, observation_scope: "persisted_snapshot_not_current_local_model"
    };
  }
  if (!recomputedHash) {
    return {
      integrity_status: "hash_recompute_unavailable",
      persisted_value: storedHash.value,
      recomputed_value: "unavailable",
      payload_ref: payloadRef,
      verification_basis: source === "open" ? "recomputed_on_open_from_restored_model" : `recomputed_at_${source}_from_returned_model`,
      verification_source: source, observed_at: observedAt, observation_scope: "persisted_snapshot_not_current_local_model"
    };
  }
  return {
    integrity_status: responseValid && storedHash.payload_ref === payloadRef && Object.entries(recomputedHash).every(([key, value]) => Reflect.get(storedHash, key) === value) ? "verified_match" : "mismatch_review_required",
    persisted_value: storedHash.value,
    recomputed_value: recomputedHash.value,
    payload_ref: payloadRef,
    verification_basis: source === "open" ? "recomputed_on_open_from_restored_model" : `recomputed_at_${source}_from_returned_model`,
      verification_source: source, observed_at: observedAt, observation_scope: "persisted_snapshot_not_current_local_model"
  };
}

export function deriveProjectEnvelopeHashIntegrity(
  storedHash: ProjectEnvelopeHashEvidence | null,
  recomputedHash: ProjectEnvelopeHashEvidence | null,
  payloadRef: string,
  source: "open" | "save" | "create" = "open",
  observedAt = new Date().toISOString(),
  responseValid = true
): ProjectEnvelopeHashIntegrityEvidence {
  if (!storedHash) {
    return {
      integrity_status: "not_persisted",
      persisted_value: "not_persisted",
      recomputed_value: recomputedHash?.value ?? "unavailable",
      payload_ref: payloadRef,
      verification_basis: source === "open" ? "recomputed_on_open_from_restored_envelope_payload" : `recomputed_at_${source}_from_returned_envelope_payload`,
      verification_source: source, observed_at: observedAt, observation_scope: "persisted_snapshot_not_current_local_model"
    };
  }
  if (!recomputedHash) {
    return {
      integrity_status: "hash_recompute_unavailable",
      persisted_value: storedHash.value,
      recomputed_value: "unavailable",
      payload_ref: payloadRef,
      verification_basis: source === "open" ? "recomputed_on_open_from_restored_envelope_payload" : `recomputed_at_${source}_from_returned_envelope_payload`,
      verification_source: source, observed_at: observedAt, observation_scope: "persisted_snapshot_not_current_local_model"
    };
  }
  return {
    integrity_status: responseValid && storedHash.payload_ref === payloadRef && Object.entries(recomputedHash).every(([key, value]) => Reflect.get(storedHash, key) === value) ? "verified_match" : "mismatch_review_required",
    persisted_value: storedHash.value,
    recomputed_value: recomputedHash.value,
    payload_ref: payloadRef,
    verification_basis: source === "open" ? "recomputed_on_open_from_restored_envelope_payload" : `recomputed_at_${source}_from_returned_envelope_payload`,
      verification_source: source, observed_at: observedAt, observation_scope: "persisted_snapshot_not_current_local_model"
  };
}

/** Fulfillment order belongs to persisted snapshots, independently of UI edit epochs. */
export function canPublishPersistenceObservation(generation: number, currentGeneration: number, observation: number, latestObservation: number): boolean {
  return generation === currentGeneration && observation >= latestObservation;
}

export interface PersistedModelBasis {
  generation: number; revision: number; hash: string | null;
  source: "loaded-source" | "open" | "create" | "save";
}

export function verifiedWriteBasis(verified: boolean, generation: number, currentGeneration: number,
  observation: number, latestVerifiedObservation: number, revision: number,
  hash: ModelHashEvidence | null, source: "save" | "create"): PersistedModelBasis | null {
  if (!verified || !hash || generation !== currentGeneration || observation <= latestVerifiedObservation) return null;
  return { generation, revision, hash: hash.value, source };
}

export function isLocalModelEdited(basis: PersistedModelBasis | null, generation: number,
  revision: number, currentHashOwned: boolean, hash: ModelHashEvidence | null): boolean {
  return !basis || basis.generation !== generation ||
    (basis.revision !== revision && (!currentHashOwned || !hash || hash.value !== basis.hash));
}

interface OpenObservationOwner {
  request: number;
  generation: number;
  projectId: string | undefined;
  basisSequence: number;
}

/** An adopted snapshot stays observable through draft invalidation and local edits. */
export function ownsOpenPersistenceObservation(captured: OpenObservationOwner, current: OpenObservationOwner): boolean {
  return captured.request === current.request && captured.generation === current.generation &&
    captured.projectId === current.projectId && captured.basisSequence === current.basisSequence;
}
