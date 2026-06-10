import type { ModelHashEvidence, PackageHashEvidence, PreviewModel } from "../types";

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
