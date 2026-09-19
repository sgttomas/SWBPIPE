// Integrity checks on what the local project store returns: whether a changed
// model in a create or save response is the one supported persisted
// normalization, and the hash comparisons made when a project is opened.
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
  payloadRef: string
): ModelHashIntegrityEvidence {
  if (!storedHash) {
    return {
      integrity_status: "not_persisted",
      persisted_value: "not_persisted",
      recomputed_value: recomputedHash?.value ?? "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_model"
    };
  }
  if (!recomputedHash) {
    return {
      integrity_status: "hash_recompute_unavailable",
      persisted_value: storedHash.value,
      recomputed_value: "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_model"
    };
  }
  return {
    integrity_status: storedHash.value === recomputedHash.value ? "verified_match" : "mismatch_review_required",
    persisted_value: storedHash.value,
    recomputed_value: recomputedHash.value,
    payload_ref: payloadRef,
    verification_basis: "recomputed_on_open_from_restored_model"
  };
}

export function deriveProjectEnvelopeHashIntegrity(
  storedHash: ProjectEnvelopeHashEvidence | null,
  recomputedHash: ProjectEnvelopeHashEvidence | null,
  payloadRef: string
): ProjectEnvelopeHashIntegrityEvidence {
  if (!storedHash) {
    return {
      integrity_status: "not_persisted",
      persisted_value: "not_persisted",
      recomputed_value: recomputedHash?.value ?? "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_envelope_payload"
    };
  }
  if (!recomputedHash) {
    return {
      integrity_status: "hash_recompute_unavailable",
      persisted_value: storedHash.value,
      recomputed_value: "unavailable",
      payload_ref: payloadRef,
      verification_basis: "recomputed_on_open_from_restored_envelope_payload"
    };
  }
  return {
    integrity_status: storedHash.value === recomputedHash.value ? "verified_match" : "mismatch_review_required",
    persisted_value: storedHash.value,
    recomputed_value: recomputedHash.value,
    payload_ref: payloadRef,
    verification_basis: "recomputed_on_open_from_restored_envelope_payload"
  };
}
