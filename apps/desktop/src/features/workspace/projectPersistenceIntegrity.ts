// Integrity checks on what the local project store returns: whether a changed
// model in a create or save response is the one supported persisted
// normalization, and session observations of opened or returned persisted bytes.
// No React.

import type {
  LocalProjectEnvelope,
  ModelHashEvidence,
  ModelHashIntegrityEvidence,
  ProjectEnvelopeHashEvidence,
  ProjectEnvelopeHashIntegrityEvidence
} from "../../types";

const SUPPORTED_MODEL_NORMALIZATION_ID =
  "model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop";

export function isSupportedChangedModelPersistenceResponse(
  envelope: LocalProjectEnvelope,
  requestModelHash: ModelHashEvidence | null,
  returnedModelHash: ModelHashEvidence | null,
  recomputedReturnedEnvelopeHash: ProjectEnvelopeHashEvidence | null,
  priorLedgerCount: number
): boolean {
  if (
    !requestModelHash ||
    !returnedModelHash ||
    !recomputedReturnedEnvelopeHash ||
    requestModelHash.value === returnedModelHash.value
  ) return false;
  const migration = envelope.model_document_migration;
  const ledger = envelope.model_migration_ledger;
  if (
    migration?.status !== "migrated" ||
    migration.persistence_state !== "persisted_with_ledger_record" ||
    migration.source_schema_version !== "0.1.0" ||
    migration.target_schema_version !== "0.2.0" ||
    envelope.model.schema_version !== "0.2.0" ||
    ledger.length <= priorLedgerCount
  ) return false;
  const record = ledger.at(-1);
  const evidence = record?.hash_evidence;
  return Boolean(
    record &&
    evidence?.schema === "model_migration_hash_evidence_v1" &&
    evidence.source_payload_basis === "incoming_pre_migration_model" &&
    evidence.received_claim_verification === "not_asserted" &&
    record.source_schema_version === "0.1.0" &&
    record.target_schema_version === "0.2.0" &&
    record.applied_migration_ids.length === 1 &&
    record.applied_migration_ids[0] === SUPPORTED_MODEL_NORMALIZATION_ID &&
    migration.applied_migration_ids.length === 1 &&
    migration.applied_migration_ids[0] === SUPPORTED_MODEL_NORMALIZATION_ID &&
    record.pre_migration_model_hash === requestModelHash.value &&
    evidence.computed.pre_migration_model_hash === requestModelHash.value &&
    record.post_migration_model_hash === returnedModelHash.value &&
    evidence.computed.post_migration_model_hash === returnedModelHash.value &&
    envelope.model_hash?.algorithm === returnedModelHash.algorithm &&
    envelope.model_hash.canonicalization === returnedModelHash.canonicalization &&
    envelope.model_hash.payload_scope === returnedModelHash.payload_scope &&
    envelope.model_hash.payload_ref === returnedModelHash.payload_ref &&
    envelope.model_hash.value === returnedModelHash.value &&
    envelope.model_hash.hash_status === returnedModelHash.hash_status &&
    evidence.computed.post_migration_project_envelope_hash === recomputedReturnedEnvelopeHash.value &&
    envelope.project_envelope_hash?.algorithm === recomputedReturnedEnvelopeHash.algorithm &&
    envelope.project_envelope_hash.canonicalization === recomputedReturnedEnvelopeHash.canonicalization &&
    envelope.project_envelope_hash.payload_scope === recomputedReturnedEnvelopeHash.payload_scope &&
    envelope.project_envelope_hash.payload_excludes === recomputedReturnedEnvelopeHash.payload_excludes &&
    envelope.project_envelope_hash.payload_ref === recomputedReturnedEnvelopeHash.payload_ref &&
    envelope.project_envelope_hash.value === recomputedReturnedEnvelopeHash.value &&
    envelope.project_envelope_hash.hash_status === recomputedReturnedEnvelopeHash.hash_status
  );
}

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
