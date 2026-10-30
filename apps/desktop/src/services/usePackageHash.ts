import { useEffect, useState } from "react";
import type { PackageHashEvidence } from "../types";
import { computePackageHash } from "./hashService";

export function usePackageHash(
  packageId: string | null,
  basePacket: unknown,
  payloadExcludes: PackageHashEvidence["payload_excludes"] = "validation_report_package_hash_fields"
): PackageHashEvidence | null {
  // Plain JSON text as the effect key: stable for identical packet content
  // across re-renders. Canonicalization and hashing happen inside the wasm
  // engine (H1 / F-5a) — the frontend no longer canonicalizes anything.
  const packetJson = basePacket === undefined || basePacket === null ? null : JSON.stringify(basePacket);
  const [packageHash, setPackageHash] = useState<PackageHashEvidence | null>(null);
  useEffect(() => {
    let active = true;
    setPackageHash(null);
    if (!packetJson || !packageId) return;
    computePackageHash(packageId, JSON.parse(packetJson), payloadExcludes).then((hash) => {
      if (active) setPackageHash(hash);
    });
    return () => {
      active = false;
    };
  }, [packetJson, packageId, payloadExcludes]);
  return packageHash;
}

export function withCanonicalPackageHash<T extends { manifest: object; validation_report: object }>(
  packet: T,
  packageHash: PackageHashEvidence | null
) {
  return {
    ...packet,
    manifest: {
      ...packet.manifest,
      canonical_package_hash_status: packageHash
        ? "computed_local_preview_sha256"
        : "TBD_browser_preview_does_not_emit_canonical_package_hash",
      canonical_package_hash: packageHash
    },
    validation_report: {
      ...packet.validation_report,
      hash_validation_status: packageHash
        ? "package_hash_computed_local_preview_not_independently_validated"
        : "TBD_browser_preview_does_not_emit_canonical_package_hash"
    }
  };
}
