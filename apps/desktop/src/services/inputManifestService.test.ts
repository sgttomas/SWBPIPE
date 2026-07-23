import { describe, expect, it } from "vitest";
import { loadPreviewModel } from "./previewService";
import {
  buildCurrentSessionInputManifest,
  verifyCurrentSessionInputManifest,
  type CurrentSessionInputManifestArgs,
} from "./inputManifestService";

async function manifestArgs(): Promise<CurrentSessionInputManifestArgs> {
  return {
    model: await loadPreviewModel(),
    solver: {
      solver_name: "open_pipe_stress_product_physics",
      solver_version: "0.1.0",
      solver_build_ref: "open_pipe_stress_product_physics@0.1.0",
      solver_mode: "sparse_interactive",
      settings: {
        nonlinear_iteration_policy:
          "DEC-046-CV-B-product-preview-active-set-count-v1",
        sparse_evidence_lane: true,
      },
    },
    active_rule_packs: [],
    external_assets: [],
  };
}

describe("current-session input manifest", () => {
  it("is deterministic and covers the exact declared input bases without a package-member or replay claim", async () => {
    const args = await manifestArgs();
    const first = await buildCurrentSessionInputManifest(args);
    const reordered = await buildCurrentSessionInputManifest({
      ...args,
      solver: {
        ...args.solver,
        settings: {
          sparse_evidence_lane: true,
          nonlinear_iteration_policy:
            "DEC-046-CV-B-product-preview-active-set-count-v1",
        },
      },
    });

    expect(first).toEqual(reordered);
    expect(first.manifest_sha256).toMatch(/^[0-9a-f]{64}$/);
    expect(first.manifest_ref).toEqual({
      object_type: "InputManifest",
      ref: `input-manifest:project-invented-loop-01:${first.manifest_sha256}`,
    });
    expect(first.canonical_bytes).toBe(
      JSON.stringify(JSON.parse(first.canonical_bytes)),
    );
    expect(first.manifest.model_basis.model_payload).toEqual(args.model);
    expect(first.manifest.unit_basis.project_units).toEqual(
      args.model.project.units,
    );
    expect(first.manifest.solver_basis).toEqual(args.solver);
    expect(first.manifest.load_basis.load_cases).toEqual(
      args.model.load_cases,
    );
    expect(first.manifest.load_basis.combinations).toEqual(
      args.model.combinations,
    );
    expect(first.manifest.active_rule_packs).toEqual([]);
    expect(first.manifest.external_assets).toEqual([]);
    expect(first.manifest.replay_boundary).toEqual({
      included_as_package_member: false,
      portable_replay_claimed: false,
      current_session_ref_hash_integrity_only: true,
    });
    await expect(verifyCurrentSessionInputManifest(first)).resolves.toBeUndefined();
  });

  it("changes the hash and ref when any declared input basis changes", async () => {
    const args = await manifestArgs();
    const original = await buildCurrentSessionInputManifest(args);
    const changedArgs = structuredClone(args);
    changedArgs.model.load_cases[0].label = "Changed current-session load label";
    const changed = await buildCurrentSessionInputManifest(changedArgs);

    expect(changed.manifest_sha256).not.toBe(original.manifest_sha256);
    expect(changed.manifest_ref.ref).not.toBe(original.manifest_ref.ref);
  });

  it("blocks transformed manifest evidence until its hash and ref are recomputed", async () => {
    const evidence = await buildCurrentSessionInputManifest(
      await manifestArgs(),
    );
    const transformed = structuredClone(evidence);
    transformed.manifest.solver_basis.settings.sparse_evidence_lane = false;

    await expect(
      verifyCurrentSessionInputManifest(transformed),
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
  });

  it("rejects wrong prefix and wrong model tokens even when the digest remains valid", async () => {
    const evidence = await buildCurrentSessionInputManifest(
      await manifestArgs(),
    );
    const wrongPrefix = structuredClone(evidence);
    wrongPrefix.manifest_ref.ref =
      `result-envelope:project-invented-loop-01:${evidence.manifest_sha256}`;
    await expect(
      verifyCurrentSessionInputManifest(wrongPrefix),
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");

    const wrongModel = structuredClone(evidence);
    wrongModel.manifest_ref.ref =
      `input-manifest:project-different:${evidence.manifest_sha256}`;
    await expect(
      verifyCurrentSessionInputManifest(wrongModel),
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
  });

  it("blocks missing or incomplete model, units, solver, load, rule-pack, and asset inputs", async () => {
    const args = await manifestArgs();
    const missingModel = structuredClone(args);
    missingModel.model.project.id = "";
    await expect(
      buildCurrentSessionInputManifest(missingModel),
    ).rejects.toThrow("INPUT-MANIFEST-MODEL-INCOMPLETE");

    const missingUnits = structuredClone(args);
    missingUnits.model.project.units = {};
    await expect(
      buildCurrentSessionInputManifest(missingUnits),
    ).rejects.toThrow("INPUT-MANIFEST-UNITS-INCOMPLETE");

    const missingSolver = structuredClone(args);
    missingSolver.solver.solver_build_ref = "";
    await expect(
      buildCurrentSessionInputManifest(missingSolver),
    ).rejects.toThrow("INPUT-MANIFEST-SOLVER-INCOMPLETE");

    const missingLoads = structuredClone(args);
    missingLoads.model.load_cases = [];
    await expect(
      buildCurrentSessionInputManifest(missingLoads),
    ).rejects.toThrow("INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE");

    const badRulePack = structuredClone(args);
    badRulePack.active_rule_packs = [
      {
        rule_pack_id: "rule-pack:test",
        version: "1",
        checksum_sha256: "ABC",
        source_notice: "invented test pack",
      },
    ];
    await expect(
      buildCurrentSessionInputManifest(badRulePack),
    ).rejects.toThrow("INPUT-MANIFEST-RULE-PACKS-INCOMPLETE");

    const badAsset = structuredClone(args);
    badAsset.external_assets = [
      {
        asset_ref: "asset:test",
        media_type: "",
        checksum_sha256: "0".repeat(64),
      },
    ];
    await expect(
      buildCurrentSessionInputManifest(badAsset),
    ).rejects.toThrow("INPUT-MANIFEST-ASSETS-INCOMPLETE");
  });
});
