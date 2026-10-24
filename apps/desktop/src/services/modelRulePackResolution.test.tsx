/** The model's rule-pack references resolved against the local store at solve time
 * (FR-CMP-002; DEL-14-01 Design `desktop-states-runs-comparison.md` §3). All rule-pack
 * data are invented. */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestArgs } from "./inputManifestService";
import {
  manifestRulePackLines,
  resolveModelRulePacks,
  RULE_PACK_REF_CHECKSUM_MISMATCH,
  RULE_PACK_REF_UNRESOLVED
} from "./modelRulePackResolution";
import { RULE_PACK_BACKEND_DIAGNOSTIC, type RulePackOpenRoute } from "./rulePackService";
import { loadPreviewModel } from "./previewService";
import { RulePackRunReferences } from "../features/rule-packs/RulePackRunReferences";
import type { PreviewModel, RulePackReference } from "../types";
import { hex, inventedEnvelope, inventedReference } from "../test/inventedRulePacks";

afterEach(cleanup);

async function modelWith(references: RulePackReference[]): Promise<PreviewModel> {
  const model = await loadPreviewModel();
  return { ...model, rule_pack_refs: references };
}

function solver(): CurrentSessionInputManifestArgs["solver"] {
  return { solver_name: "invented-solver", solver_version: "0.0.1", solver_build_ref: "invented-solver@0.0.1", solver_mode: "sparse_interactive", settings: { sparse_evidence_lane: true } };
}

describe("resolving the model's rule-pack references at solve time", () => {
  it("enters each resolved pack in the model's order, with the stored pack's version, checksum and notice", async () => {
    const model = await modelWith([inventedReference("invented-pack-b", hex("b")), inventedReference("invented-pack-a", hex("a"), "2.0.0")]);
    const opened: string[] = [];
    const store = new Map([
      ["invented-pack-a", inventedEnvelope(model.project.id, "invented-pack-a", hex("a"), { version: "2.0.0" })],
      ["invented-pack-b", inventedEnvelope(model.project.id, "invented-pack-b", hex("b"))]
    ]);
    const resolution = await resolveModelRulePacks(model, async (projectId, id): Promise<RulePackOpenRoute> => {
      opened.push(`${projectId}/${id}`);
      return { route: "tauri_backend", envelope: store.get(id) ?? null };
    });
    expect(opened).toEqual([`${model.project.id}/invented-pack-b`, `${model.project.id}/invented-pack-a`]);
    expect(resolution).toEqual({
      active_rule_packs: [
        { rule_pack_id: "invented-pack-b", version: "1.0.0", checksum_sha256: hex("b"), source_notice: "Invented notice for invented-pack-b; not an engineering basis." },
        { rule_pack_id: "invented-pack-a", version: "2.0.0", checksum_sha256: hex("a"), source_notice: "Invented notice for invented-pack-a; not an engineering basis." }
      ],
      excluded_rule_pack_refs: []
    });
    // The manifest keeps the model's order (it no longer sorts the packs).
    const manifest = await buildCurrentSessionInputManifest({ model, solver: solver(), ...resolution, external_assets: [] });
    expect(manifest.manifest.active_rule_packs.map((pack) => pack.rule_pack_id)).toEqual(["invented-pack-b", "invented-pack-a"]);
    expect(Object.hasOwn(manifest.manifest, "excluded_rule_pack_refs")).toBe(false);
  });

  it("excludes an unresolved and a checksum-mismatched reference, each with its diagnostic, and records them in the manifest", async () => {
    const model = await modelWith([
      inventedReference("invented-pack-held", hex("1")),
      inventedReference("invented-pack-missing", hex("2")),
      inventedReference("invented-pack-changed", hex("3")),
      inventedReference("invented-pack-unverified", hex("4")),
      inventedReference("invented-pack-failing-store", hex("5"))
    ]);
    const store = new Map([
      ["invented-pack-held", inventedEnvelope(model.project.id, "invented-pack-held", hex("1"))],
      // Edited and re-stamped since the model referenced it.
      ["invented-pack-changed", inventedEnvelope(model.project.id, "invented-pack-changed", hex("9"))],
      // The stored stamp is the reference's, but the document no longer recomputes to it.
      ["invented-pack-unverified", inventedEnvelope(model.project.id, "invented-pack-unverified", hex("4"), { matchStatus: "mismatch_review_required" })]
    ]);
    const resolution = await resolveModelRulePacks(model, async (_projectId, id) => {
      if (id === "invented-pack-failing-store") throw new Error("invented store failure");
      return { route: "tauri_backend", envelope: store.get(id) ?? null };
    });
    expect(resolution.active_rule_packs.map((pack) => pack.rule_pack_id)).toEqual(["invented-pack-held"]);
    expect(resolution.excluded_rule_pack_refs).toEqual([
      { rule_pack_id: "invented-pack-missing", rule_pack_version: "1.0.0", checksum_sha256: hex("2"), code: RULE_PACK_REF_UNRESOLVED },
      { rule_pack_id: "invented-pack-changed", rule_pack_version: "1.0.0", checksum_sha256: hex("3"), code: RULE_PACK_REF_CHECKSUM_MISMATCH },
      { rule_pack_id: "invented-pack-unverified", rule_pack_version: "1.0.0", checksum_sha256: hex("4"), code: RULE_PACK_REF_CHECKSUM_MISMATCH },
      { rule_pack_id: "invented-pack-failing-store", rule_pack_version: "1.0.0", checksum_sha256: hex("5"), code: RULE_PACK_REF_UNRESOLVED }
    ]);
    const evidence = await buildCurrentSessionInputManifest({ model, solver: solver(), ...resolution, external_assets: [] });
    expect(evidence.manifest.excluded_rule_pack_refs).toEqual(resolution.excluded_rule_pack_refs);
    const lines = manifestRulePackLines(evidence.manifest);
    expect(lines.recorded).toEqual([`invented-pack-held v1.0.0 (sha256 ${hex("1").slice(0, 12)}…)`]);
    expect(lines.diagnostics).toHaveLength(4);
    expect(lines.diagnostics[0]).toBe(`${RULE_PACK_REF_UNRESOLVED} (warning): invented-pack-missing v1.0.0 is referenced by the model, but the local rule-pack store held no such pack for this project at solve time; it is not among this run's rule-pack references.`);
    expect(lines.diagnostics[1]).toBe(`${RULE_PACK_REF_CHECKSUM_MISMATCH} (warning): invented-pack-changed v1.0.0 is referenced by the model, but the stored pack's checksum is not the model reference's sha256 ${hex("3").slice(0, 12)}…; it is not among this run's rule-pack references.`);
    // A changed exclusion is a different manifest.
    const fewer = await buildCurrentSessionInputManifest({ model, solver: solver(), ...resolution, excluded_rule_pack_refs: resolution.excluded_rule_pack_refs.slice(1), external_assets: [] });
    expect(fewer.manifest_sha256).not.toBe(evidence.manifest_sha256);
  });

  it("a different stored version under the same checksum, or a stored pack without a usable notice, does not enter", async () => {
    const model = await modelWith([inventedReference("invented-pack-v", hex("c")), inventedReference("invented-pack-n", hex("d"))]);
    const store = new Map([
      ["invented-pack-v", inventedEnvelope(model.project.id, "invented-pack-v", hex("c"), { version: "1.0.1" })],
      ["invented-pack-n", inventedEnvelope(model.project.id, "invented-pack-n", hex("d"), { notice: "  " })]
    ]);
    const resolution = await resolveModelRulePacks(model, async (_projectId, id) => ({ route: "tauri_backend", envelope: store.get(id) ?? null }));
    expect(resolution.active_rule_packs).toEqual([]);
    expect(resolution.excluded_rule_pack_refs.map((item) => item.code)).toEqual([RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED]);
  });

  it("the browser preview has no rule-pack store: every reference stays unresolved with the diagnostic", async () => {
    expect("__TAURI_INTERNALS__" in window).toBe(false);
    const model = await modelWith([inventedReference("invented-pack-a", hex("a"))]);
    // The real store route: the browser preview answers unavailable_browser_preview.
    const resolution = await resolveModelRulePacks(model);
    expect(resolution).toEqual({ active_rule_packs: [], excluded_rule_pack_refs: [{ rule_pack_id: "invented-pack-a", rule_pack_version: "1.0.0", checksum_sha256: hex("a"), code: RULE_PACK_REF_UNRESOLVED }] });
    expect(RULE_PACK_BACKEND_DIAGNOSTIC).toMatch(/^RULE-PACK-BACKEND-DESKTOP-ONLY/);
  });

  it("a model without references resolves to nothing and its manifest has no exclusion member", async () => {
    const model = await loadPreviewModel();
    expect(model.rule_pack_refs ?? []).toEqual([]);
    let opened = 0;
    expect(await resolveModelRulePacks(model, async () => { opened += 1; return { route: "tauri_backend", envelope: null }; })).toEqual({ active_rule_packs: [], excluded_rule_pack_refs: [] });
    expect(opened).toBe(0);
    const withEmpty = await buildCurrentSessionInputManifest({ model, solver: solver(), active_rule_packs: [], excluded_rule_pack_refs: [], external_assets: [] });
    const without = await buildCurrentSessionInputManifest({ model, solver: solver(), active_rule_packs: [], external_assets: [] });
    expect(withEmpty.canonical_bytes).toBe(without.canonical_bytes);
    expect(withEmpty.canonical_bytes).not.toContain("excluded_rule_pack_refs");
  });

  it("the manifest refuses a malformed exclusion or one that repeats a recorded pack", async () => {
    const model = await modelWith([inventedReference("invented-pack-a", hex("a"))]);
    const pack = { rule_pack_id: "invented-pack-a", version: "1.0.0", checksum_sha256: hex("a"), source_notice: "Invented." };
    await expect(buildCurrentSessionInputManifest({ model, solver: solver(), active_rule_packs: [pack], excluded_rule_pack_refs: [{ rule_pack_id: "invented-pack-a", rule_pack_version: "1.0.0", checksum_sha256: hex("a"), code: RULE_PACK_REF_UNRESOLVED }], external_assets: [] }))
      .rejects.toThrow("INPUT-MANIFEST-RULE-PACKS-INCOMPLETE");
    await expect(buildCurrentSessionInputManifest({ model, solver: solver(), active_rule_packs: [], excluded_rule_pack_refs: [{ rule_pack_id: "invented-pack-a", rule_pack_version: "1.0.0", checksum_sha256: hex("a"), code: "INVENTED-CODE" as typeof RULE_PACK_REF_UNRESOLVED }], external_assets: [] }))
      .rejects.toThrow("INPUT-MANIFEST-RULE-PACKS-INCOMPLETE");
  });
});

describe("the rule-pack references shown with a run", () => {
  it("lists the recorded packs and each diagnostic, and claims no check, application or pass", async () => {
    const model = await modelWith([inventedReference("invented-pack-held", hex("1")), inventedReference("invented-pack-missing", hex("2"))]);
    const evidence = await buildCurrentSessionInputManifest({
      model, solver: solver(), external_assets: [],
      active_rule_packs: [{ rule_pack_id: "invented-pack-held", version: "1.0.0", checksum_sha256: hex("1"), source_notice: "Invented." }],
      excluded_rule_pack_refs: [{ rule_pack_id: "invented-pack-missing", rule_pack_version: "1.0.0", checksum_sha256: hex("2"), code: RULE_PACK_REF_UNRESOLVED }]
    });
    render(<RulePackRunReferences manifest={evidence.manifest} testId="refs" />);
    const block = screen.getByTestId("refs");
    expect(screen.getByTestId("refs-recorded").textContent).toContain("invented-pack-held v1.0.0");
    expect(screen.getByTestId("refs-diagnostic").textContent).toMatch(/^MODEL-RULE-PACK-REF-UNRESOLVED \(warning\): invented-pack-missing/);
    expect(block.textContent).toContain("does not read, apply or check a rule pack");
    expect(block.textContent).not.toMatch(/\b(passed|applied to|was checked|compliant|approved|accepted)\b/i);
  });

  it("shows nothing for a run whose model referenced no pack", async () => {
    const model = await loadPreviewModel();
    const evidence = await buildCurrentSessionInputManifest({ model, solver: solver(), active_rule_packs: [], external_assets: [] });
    render(<RulePackRunReferences manifest={evidence.manifest} testId="refs" />);
    expect(screen.queryByTestId("refs")).toBeNull();
  });
});
