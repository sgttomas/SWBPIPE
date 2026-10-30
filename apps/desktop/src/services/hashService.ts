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

function validUnicodeScalarString(value: string): boolean {
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return false;
      index += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) return false;
  }
  return true;
}

export function checkedJsonText(value: unknown): string {
  const seen = new Set<object>();
  const freeze = (item: unknown): unknown => {
    if (item === null || typeof item === "boolean") return item;
    if (typeof item === "string") {
      if (!validUnicodeScalarString(item)) throw new Error("CHECKED-JSON-LONE-SURROGATE");
      return item;
    }
    if (typeof item === "number") {
      if (!Number.isFinite(item) || (Number.isInteger(item) && !Number.isSafeInteger(item))) {
        throw new Error("CHECKED-JSON-NUMBER-OUTSIDE-PROFILE");
      }
      return item;
    }
    if (typeof item !== "object") throw new Error(`CHECKED-JSON-UNSUPPORTED-TYPE: ${typeof item}`);
    if (seen.has(item)) throw new Error("CHECKED-JSON-CYCLIC-VALUE");
    seen.add(item);
    try {
      const descriptors = Object.getOwnPropertyDescriptors(item);
      const keys = Reflect.ownKeys(descriptors);
      if (Array.isArray(item)) {
      if (keys.some((key) => typeof key !== "string")) throw new Error("CHECKED-JSON-SYMBOL-KEY");
      const lengthDescriptor = descriptors.length;
      const length = lengthDescriptor?.value;
      if (!lengthDescriptor || typeof length !== "number" || !Number.isSafeInteger(length) || length < 0) throw new Error("CHECKED-JSON-ARRAY-LENGTH");
      const allowed = new Set(["length", ...Array.from({ length }, (_, index) => String(index))]);
      if (keys.some((key) => !allowed.has(key as string))) throw new Error("CHECKED-JSON-ARRAY-OWN-PROPERTY");
      const frozen: unknown[] = [];
      for (let index = 0; index < length; index += 1) {
        const descriptor = descriptors[String(index)];
        if (!descriptor) throw new Error("CHECKED-JSON-SPARSE-ARRAY");
        if (!descriptor || descriptor.get || descriptor.set || !descriptor.enumerable) throw new Error("CHECKED-JSON-UNSUPPORTED-PROPERTY");
        frozen.push(freeze(descriptor.value));
      }
      return frozen;
      }
      const prototype = Object.getPrototypeOf(item);
      if (prototype !== Object.prototype && prototype !== null) throw new Error("CHECKED-JSON-NON-PLAIN-OBJECT");
      const frozen: Record<string, unknown> = Object.create(null) as Record<string, unknown>;
      for (const key of keys) {
        if (typeof key !== "string") throw new Error("CHECKED-JSON-SYMBOL-KEY");
        if (!validUnicodeScalarString(key)) throw new Error("CHECKED-JSON-LONE-SURROGATE");
        const descriptor = descriptors[key];
        if (!descriptor || descriptor.get || descriptor.set || !descriptor.enumerable) throw new Error("CHECKED-JSON-UNSUPPORTED-PROPERTY");
        frozen[key] = freeze(descriptor.value);
      }
      return frozen;
    } finally {
      seen.delete(item);
    }
  };
  return JSON.stringify(freeze(value));
}

export async function canonicalJsonString(value: unknown): Promise<string> {
  const engine = await loadWasmEngine();
  return engine.canonicalJsonString(asJsonText(value));
}

export async function canonicalSha256Hex(value: unknown): Promise<string> {
  const engine = await loadWasmEngine();
  return engine.canonicalSha256Hex(asJsonText(value));
}

export async function canonicalJsonCheckedV1(value: unknown): Promise<string> {
  const jsonText = checkedJsonText(value);
  const engine = await loadWasmEngine();
  return engine.canonicalJsonCheckedV1(jsonText);
}

export async function canonicalSha256HexCheckedV1(value: unknown): Promise<string> {
  const jsonText = checkedJsonText(value);
  const engine = await loadWasmEngine();
  return engine.canonicalSha256HexCheckedV1(jsonText);
}

export async function computeModelHash(model: PreviewModel): Promise<ModelHashEvidence | null> {
  const hex = await canonicalSha256Hex(model);
  return {
    algorithm: "sha256",
    canonicalization: "rfc8785_jcs",
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
    canonicalization: "rfc8785_jcs",
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
    canonicalization: "rfc8785_jcs",
    payload_scope: "package_review_payload",
    payload_excludes: payloadExcludes,
    payload_ref: packageId,
    value: `sha256:${hex}`,
    hash_status: "computed_local_preview"
  };
}
