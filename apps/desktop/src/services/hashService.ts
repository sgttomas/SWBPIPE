import type {
  AgentProposal,
  AnalysisRunEnvelope,
  EditorOperationIntent,
  MechanicsResult,
  ModelHashEvidence,
  PackageHashEvidence,
  PreviewModel,
  ProjectEnvelopeHashEvidence,
  SelectedReviewTarget
} from "../types";

export function canonicalJson(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map((item) => canonicalJson(item)).join(",")}]`;
  const entries = Object.entries(value as Record<string, unknown>)
    .filter(([, entryValue]) => entryValue !== undefined)
    .sort(([first], [second]) => (first < second ? -1 : first > second ? 1 : 0));
  return `{${entries.map(([key, entryValue]) => `${JSON.stringify(key)}:${canonicalJson(entryValue)}`).join(",")}}`;
}

function subtleCrypto(): SubtleCrypto | null {
  return globalThis.crypto?.subtle ?? null;
}

async function sha256Hex(payload: string): Promise<string | null> {
  const subtle = subtleCrypto();
  if (!subtle) return null;
  const digest = await subtle.digest("SHA-256", new TextEncoder().encode(payload));
  return Array.from(new Uint8Array(digest))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

export async function computeModelHash(model: PreviewModel): Promise<ModelHashEvidence | null> {
  const hex = await sha256Hex(canonicalJson(model));
  if (!hex) return null;
  return {
    algorithm: "sha256",
    canonicalization: "jcs_like_sorted_object_keys",
    payload_scope: "model_payload",
    payload_ref: model.project.id,
    value: `sha256:${hex}`,
    hash_status: "computed_local_preview"
  };
}

export type ProjectEnvelopeHashPayload = {
  model: PreviewModel;
  editor_intents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selected_review_target: SelectedReviewTarget | null;
  mechanics_result: MechanicsResult | null;
  analysis_run: AnalysisRunEnvelope | null;
  model_hash: ModelHashEvidence | null;
};

// The envelope hash covers the persisted payload only; the volatile storage
// summary and the envelope-hash carrier field itself are excluded so the hash
// can be recomputed from a restored envelope.
export async function computeProjectEnvelopeHash(
  payload: ProjectEnvelopeHashPayload
): Promise<ProjectEnvelopeHashEvidence | null> {
  const hex = await sha256Hex(canonicalJson(payload));
  if (!hex) return null;
  return {
    algorithm: "sha256",
    canonicalization: "jcs_like_sorted_object_keys",
    payload_scope: "project_envelope_payload",
    payload_excludes: "storage_summary_and_envelope_hash_carrier_fields",
    payload_ref: payload.model.project.id,
    value: `sha256:${hex}`,
    hash_status: "computed_local_preview"
  };
}

export async function computePackageHash(
  packageId: string,
  canonicalPayload: string,
  payloadExcludes: PackageHashEvidence["payload_excludes"] = "validation_report_package_hash_fields"
): Promise<PackageHashEvidence | null> {
  const hex = await sha256Hex(canonicalPayload);
  if (!hex) return null;
  return {
    algorithm: "sha256",
    canonicalization: "jcs_like_sorted_object_keys",
    payload_scope: "package_review_payload",
    payload_excludes: payloadExcludes,
    payload_ref: packageId,
    value: `sha256:${hex}`,
    hash_status: "computed_local_preview"
  };
}
