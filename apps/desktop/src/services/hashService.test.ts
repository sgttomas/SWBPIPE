import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../types";
import { canonicalJson, computeModelHash, computePackageHash } from "./hashService";

describe("hashService", () => {
  it("canonicalizes JSON with sorted object keys independent of insertion order", () => {
    const first = { b: [1, 2, { d: 4, c: 3 }], a: "x", skipped: undefined };
    const second = { a: "x", b: [1, 2, { c: 3, d: 4 }] };
    expect(canonicalJson(first)).toBe(canonicalJson(second));
    expect(canonicalJson(first)).toBe('{"a":"x","b":[1,2,{"c":3,"d":4}]}');
    expect(canonicalJson(null)).toBe("null");
    expect(canonicalJson("text")).toBe('"text"');
  });

  it("computes a deterministic local-preview sha256 model hash over canonical JSON", async () => {
    const model = { project: { id: "project:test-hash", name: "Hash Test" } } as unknown as PreviewModel;
    const reordered = { project: { name: "Hash Test", id: "project:test-hash" } } as unknown as PreviewModel;
    const changed = { project: { id: "project:test-hash", name: "Hash Test Changed" } } as unknown as PreviewModel;

    const hash = await computeModelHash(model);
    const reorderedHash = await computeModelHash(reordered);
    const changedHash = await computeModelHash(changed);

    expect(hash).not.toBeNull();
    expect(hash?.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(hash?.value).toBe(reorderedHash?.value);
    expect(hash?.value).not.toBe(changedHash?.value);
    expect(hash?.algorithm).toBe("sha256");
    expect(hash?.canonicalization).toBe("jcs_like_sorted_object_keys");
    expect(hash?.payload_scope).toBe("model_payload");
    expect(hash?.payload_ref).toBe("project:test-hash");
    expect(hash?.hash_status).toBe("computed_local_preview");
  });

  it("computes a deterministic local-preview sha256 package hash over a canonical packet payload", async () => {
    const payload = canonicalJson({ manifest: { package_members: [{ path: "manifest.json" }] }, package_id: "native-json-preview:project:test-hash" });
    const changedPayload = canonicalJson({ manifest: { package_members: [{ path: "manifest.json" }, { path: "model/project.json" }] }, package_id: "native-json-preview:project:test-hash" });

    const hash = await computePackageHash("native-json-preview:project:test-hash", payload);
    const repeatHash = await computePackageHash("native-json-preview:project:test-hash", payload);
    const changedHash = await computePackageHash("native-json-preview:project:test-hash", changedPayload);

    expect(hash).not.toBeNull();
    expect(hash?.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(hash?.value).toBe(repeatHash?.value);
    expect(hash?.value).not.toBe(changedHash?.value);
    expect(hash?.algorithm).toBe("sha256");
    expect(hash?.canonicalization).toBe("jcs_like_sorted_object_keys");
    expect(hash?.payload_scope).toBe("package_review_payload");
    expect(hash?.payload_excludes).toBe("validation_report_package_hash_fields");
    expect(hash?.payload_ref).toBe("native-json-preview:project:test-hash");
    expect(hash?.hash_status).toBe("computed_local_preview");

    const carrierExcludesHash = await computePackageHash(
      "native-json-preview:project:test-hash",
      payload,
      "manifest_and_validation_report_package_hash_carrier_fields"
    );
    expect(carrierExcludesHash?.payload_excludes).toBe("manifest_and_validation_report_package_hash_carrier_fields");
    expect(carrierExcludesHash?.value).toBe(hash?.value);
  });
});
