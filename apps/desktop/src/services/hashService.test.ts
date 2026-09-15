// H1 / F-5a: the frontend hash seam is the wasm build of the Rust engine's
// canonical_json / sha256_hex. The wasm lane of the canonical-hash parity
// corpus lives here: the SAME case files are asserted against the native
// engine by core/model_operations/operation_applier/tests/
// canonical_hash_parity.rs (the blessing authority), so a pass in both
// suites proves the lanes agree byte-for-byte on identical input text.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../types";
import { canonicalJsonCheckedV1, canonicalJsonString, canonicalSha256Hex, canonicalSha256HexCheckedV1, computeModelHash, computePackageHash } from "./hashService";
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";

const corpusPath = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../../fixtures/canonical_hash/cases.json"
);

type ParityCase = {
  case_id: string;
  input_json: string;
  expected_canonical: string;
  expected_sha256: string;
};

const corpus = JSON.parse(readFileSync(corpusPath, "utf8")) as { cases: ParityCase[] };
const checkedCaseIds = new Set([
  "scalar-string-unicode",
  "number-negative-zero-renders-zero",
  "number-ecma-notation-boundaries",
  "number-beyond-2-53-integer-kept-exact"
]);
const checkedCorpus: Array<ParityCase & { outcome: "accept" | "reject" }> = [
  ...corpus.cases.filter((item) => checkedCaseIds.has(item.case_id)).map((item) => ({
    ...item,
    outcome: ["number-beyond-2-53-integer-kept-exact", "number-ecma-notation-boundaries"].includes(item.case_id) ? "reject" as const : "accept" as const
  })),
  { case_id: "one-and-one-point-zero", input_json: "[1,1.0]", expected_canonical: "[1,1]",
    expected_sha256: "e61b9f584dbe27741cef6e9ee440831d7d94470c0871b0871541f0308916efea", outcome: "accept" },
  { case_id: "safe-exponent-boundaries", input_json: "[1e-6,1e-7]", expected_canonical: "[0.000001,1e-7]",
    expected_sha256: "19ca01c5d07894d9ce68294ad32b64d9c2a851c244ae8010e0a2b8a26f3734a0", outcome: "accept" }
];
const engine = await loadWasmEngine();

describe("canonical-hash parity corpus — wasm lane (native↔wasm, Rust-blessed)", () => {
  it("keeps the coverage floor", () => {
    expect(corpus.cases.length).toBeGreaterThanOrEqual(12);
  });

  for (const parityCase of corpus.cases) {
    it(`case ${parityCase.case_id} matches the native engine expectations`, () => {
      expect(
        parityCase.expected_canonical,
        "expectations missing — bless with CANONICAL_HASH_BLESS=1 cargo test"
      ).not.toBe("TBD");
      expect(engine.canonicalJsonString(parityCase.input_json)).toBe(parityCase.expected_canonical);
      expect(engine.canonicalSha256Hex(parityCase.input_json)).toBe(parityCase.expected_sha256);
    });
  }

  it("throws the named input diagnostic on malformed JSON instead of hashing it", () => {
    expect(() => engine.canonicalSha256Hex("{not json")).toThrowError(/WASM-ENGINE-INPUT-JSON-INVALID/);
  });
});

describe("checked I-JSON parity corpus — Rust/WASM/adapter contract", () => {
  for (const parityCase of checkedCorpus) {
    it(`checked case ${parityCase.case_id}`, async () => {
      const value = JSON.parse(parityCase.input_json);
      if (parityCase.outcome === "reject") {
        expect(() => engine.canonicalJsonCheckedV1(parityCase.input_json)).toThrow(/UNSAFE|OUTSIDE/);
        await expect(canonicalJsonCheckedV1(value)).rejects.toThrow(/OUTSIDE-PROFILE|UNSAFE/);
        return;
      }
      expect(engine.canonicalJsonCheckedV1(parityCase.input_json)).toBe(parityCase.expected_canonical);
      expect(engine.canonicalSha256HexCheckedV1(parityCase.input_json)).toBe(parityCase.expected_sha256);
      expect(await canonicalJsonCheckedV1(value)).toBe(parityCase.expected_canonical);
      expect(await canonicalSha256HexCheckedV1(value)).toBe(parityCase.expected_sha256);
    });
  }
});

describe("hashService adapter (wasm-engine-backed)", () => {
  const model = {
    project: { id: "project:hash-test", label: "invented" },
    nodes: [{ id: "node:N-1", x: 1.5, y: 0, z: -2 }]
  } as unknown as PreviewModel;

  it("canonicalizes through the engine: key order is irrelevant, content is not", async () => {
    const first = await canonicalJsonString({ b: [1, 2, { d: 4, c: 3 }], a: "x" });
    const second = await canonicalJsonString({ a: "x", b: [1, 2, { c: 3, d: 4 }] });
    expect(first).toBe(second);
    expect(first).toBe('{"a":"x","b":[1,2,{"c":3,"d":4}]}');
    expect(await canonicalJsonString(null)).toBe("null");
    expect(await canonicalJsonString("text")).toBe('"text"');
  });

  it("computeModelHash is stable under reordering and sensitive to change", async () => {
    const reordered = JSON.parse(
      '{"nodes":[{"z":-2,"y":0,"x":1.5,"id":"node:N-1"}],"project":{"label":"invented","id":"project:hash-test"}}'
    ) as PreviewModel;
    const changed = { ...model, nodes: [{ id: "node:N-1", x: 1.5, y: 0, z: -2.0001 }] } as unknown as PreviewModel;
    const hash = await computeModelHash(model);
    const reorderedHash = await computeModelHash(reordered);
    const changedHash = await computeModelHash(changed);
    expect(hash?.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(hash?.value).toBe(reorderedHash?.value);
    expect(hash?.value).not.toBe(changedHash?.value);
    expect(hash?.canonicalization).toBe("rfc8785_jcs");
    expect(hash?.hash_status).toBe("computed_local_preview");
  });

  it("computeModelHash equals the engine hash of the same payload (no second implementation)", async () => {
    const direct = await canonicalSha256Hex(model);
    const evidence = await computeModelHash(model);
    expect(evidence?.value).toBe(`sha256:${direct}`);
  });

  it("computePackageHash is deterministic, content-sensitive, and carries the exclusion label", async () => {
    const packet = {
      manifest: { package_members: [{ path: "manifest.json" }] },
      package_id: "native-json-preview:project:test-hash"
    };
    const changedPacket = {
      manifest: { package_members: [{ path: "manifest.json" }, { path: "model/project.json" }] },
      package_id: "native-json-preview:project:test-hash"
    };
    const hash = await computePackageHash("native-json-preview:project:test-hash", packet);
    const repeatHash = await computePackageHash("native-json-preview:project:test-hash", packet);
    const changedHash = await computePackageHash("native-json-preview:project:test-hash", changedPacket);
    const carrierExcludesHash = await computePackageHash(
      "native-json-preview:project:test-hash",
      packet,
      "manifest_and_validation_report_package_hash_carrier_fields"
    );
    expect(hash?.value).toBe(repeatHash?.value);
    expect(hash?.value).not.toBe(changedHash?.value);
    expect(hash?.payload_excludes).toBe("validation_report_package_hash_fields");
    expect(carrierExcludesHash?.payload_excludes).toBe("manifest_and_validation_report_package_hash_carrier_fields");
    expect(carrierExcludesHash?.value).toBe(hash?.value);
  });

  it("drops undefined-valued keys before hashing (JSON.stringify semantics, as before)", async () => {
    const withUndefined = await canonicalJsonString({ a: 1, ghost: undefined });
    const without = await canonicalJsonString({ a: 1 });
    expect(withUndefined).toBe(without);
  });
});
