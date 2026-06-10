import type { ModelHashEvidence, PreviewModel } from "../types";

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

export async function computeModelHash(model: PreviewModel): Promise<ModelHashEvidence | null> {
  const subtle = subtleCrypto();
  if (!subtle) return null;
  const payload = new TextEncoder().encode(canonicalJson(model));
  const digest = await subtle.digest("SHA-256", payload);
  const hex = Array.from(new Uint8Array(digest))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
  return {
    algorithm: "sha256",
    canonicalization: "jcs_like_sorted_object_keys",
    payload_scope: "model_payload",
    payload_ref: model.project.id,
    value: `sha256:${hex}`,
    hash_status: "computed_local_preview"
  };
}
