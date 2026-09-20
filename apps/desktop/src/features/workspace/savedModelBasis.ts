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
  const matches = (claimed: ModelHashEvidence | ProjectEnvelopeHashEvidence | null | undefined,
    computed: ModelHashEvidence | ProjectEnvelopeHashEvidence) => !claimed ||
      Object.entries(computed).every(([key, value]) => (claimed as unknown as Record<string, unknown>)[key] === value);
  // An unchanged Historical save deliberately carries its original recorded
  // evidence hash. The caller proves unchanged canonical model identity before
  // offering this exact retained carrier; it never becomes the comparison hash.
  const modelClaimMatches = matches(envelope.model_hash, modelHash) ||
    (retainedHistoricalModelHash !== null && envelope.model_hash != null &&
      matches(envelope.model_hash, retainedHistoricalModelHash));
  return modelClaimMatches && matches(envelope.project_envelope_hash, envelopeHash);
}
