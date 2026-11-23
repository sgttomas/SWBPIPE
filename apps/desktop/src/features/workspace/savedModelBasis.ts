import type { LocalProjectEnvelope, ModelHashEvidence, ProjectEnvelopeHashEvidence } from "../../types";

/** Validate returned claims against computed payloads; missing legacy claims
 * do not substitute for recomputation. This never changes integrity records. */
export function persistenceHashesMatch(
  envelope: LocalProjectEnvelope,
  modelHash: ModelHashEvidence | null,
  envelopeHash: ProjectEnvelopeHashEvidence | null,
  retainedHistoricalModelHash: ModelHashEvidence | null = null
): boolean {
  if (!modelHash || !envelopeHash) return false;
  return modelHashClaimMatches(envelope, modelHash, retainedHistoricalModelHash) && matches(envelope.project_envelope_hash, envelopeHash);
}

const matches = (claimed: ModelHashEvidence | ProjectEnvelopeHashEvidence | null | undefined,
  computed: ModelHashEvidence | ProjectEnvelopeHashEvidence) => !claimed ||
    Object.entries(computed).every(([key, value]) => (claimed as unknown as Record<string, unknown>)[key] === value);

/** The model-hash half of `persistenceHashesMatch`, for a reply whose envelope claim is accepted on
 * other grounds (issue 519: a kept save that preserved a stored project's absent claim). A computed
 * model hash is required. */
export function modelHashClaimMatches(
  envelope: LocalProjectEnvelope,
  modelHash: ModelHashEvidence | null,
  retainedHistoricalModelHash: ModelHashEvidence | null = null
): boolean {
  if (!modelHash) return false;
  // An unchanged Historical save deliberately carries its original recorded
  // evidence hash. The caller proves unchanged canonical model identity before
  // offering this exact retained carrier; it never becomes the comparison hash.
  return matches(envelope.model_hash, modelHash) ||
    (retainedHistoricalModelHash !== null && envelope.model_hash != null &&
      matches(envelope.model_hash, retainedHistoricalModelHash));
}
