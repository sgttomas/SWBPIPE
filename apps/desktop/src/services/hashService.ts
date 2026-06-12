// H1 / F-5a (completion-plan §3 hardening lane; verification F-5): frontend
// hashing routes through the wasm build of the Rust engine's
// `canonical_json` / `sha256_hex` — the same functions the engines use for
// model hashes — via the `loadWasmEngine` seam. The TypeScript
// canonicalization that previously lived here is deleted; no fallback
// exists (an absent wasm artifact fails loudly with
// WASM-ENGINE-ASSET-ABSENT from the loader, never a second hash
// implementation). Parity with the native lane is pinned by
// `fixtures/canonical_hash/cases.json`.
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
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";

function asJsonText(value: unknown): string {
  // JSON.stringify drops undefined-valued keys, matching the filtering the
  // deleted TS canonicalization applied before hashing.
  return JSON.stringify(value === undefined ? null : value);
}

export async function canonicalJsonString(value: unknown): Promise<string> {
  const engine = await loadWasmEngine();
  return engine.canonicalJsonString(asJsonText(value));
}

export async function canonicalSha256Hex(value: unknown): Promise<string> {
  const engine = await loadWasmEngine();
  return engine.canonicalSha256Hex(asJsonText(value));
}

export async function computeModelHash(model: PreviewModel): Promise<ModelHashEvidence | null> {
  const hex = await canonicalSha256Hex(model);
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
  const hex = await canonicalSha256Hex(payload);
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
  packetPayload: unknown,
  payloadExcludes: PackageHashEvidence["payload_excludes"] = "validation_report_package_hash_fields"
): Promise<PackageHashEvidence | null> {
  const hex = await canonicalSha256Hex(packetPayload);
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
