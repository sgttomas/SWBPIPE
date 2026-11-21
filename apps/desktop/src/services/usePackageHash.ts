import { useEffect, useState } from "react";
import type { PackageHashEvidence } from "../types";
import { computePackageHash } from "./hashService";

type PayloadExcludes = PackageHashEvidence["payload_excludes"];

/** A computed hash with the inputs it was computed from. */
type BoundPackageHash = {
  packageId: string;
  packetJson: string;
  payloadExcludes: PayloadExcludes;
  hash: PackageHashEvidence | null;
};

/** The package hash of `basePacket` under `packageId` and `payloadExcludes`, or null while it
 * is pending. The hash is returned only on a render whose inputs are the ones it was computed
 * from (issue 497): on the render in which the inputs change it is already null, so a caller
 * never assembles a packet with another packet's hash. Going back to earlier inputs returns
 * null until their hash is computed again. */
export function usePackageHash(
  packageId: string | null,
  basePacket: unknown,
  payloadExcludes: PayloadExcludes = "validation_report_package_hash_fields"
): PackageHashEvidence | null {
  // Plain JSON text as the effect key: stable for identical packet content
  // across re-renders. Canonicalization and hashing happen inside the wasm
  // engine (H1 / F-5a) — the frontend no longer canonicalizes anything.
  const packetJson = basePacket === undefined || basePacket === null ? null : JSON.stringify(basePacket);
  const [bound, setBound] = useState<BoundPackageHash | null>(null);
  useEffect(() => {
    let active = true;
    setBound(null);
    if (!packetJson || !packageId) return;
    computePackageHash(packageId, JSON.parse(packetJson), payloadExcludes).then((hash) => {
      // A computation that resolves after its inputs changed is not stored.
      if (active) setBound({ packageId, packetJson, payloadExcludes, hash });
    });
    return () => {
      active = false;
    };
  }, [packetJson, packageId, payloadExcludes]);
  return bound !== null &&
    bound.packageId === packageId &&
    bound.packetJson === packetJson &&
    bound.payloadExcludes === payloadExcludes
    ? bound.hash
    : null;
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
