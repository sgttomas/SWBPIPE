/** The model's rule-pack references resolved against the local store at solve time
 * (FR-CMP-002; DEL-14-01 Design `desktop-states-runs-comparison.md` §3): resolved packs
 * enter the input manifest's `active_rule_packs`; excluded references become diagnostics
 * of the run's analysis record, after the producer's. All rule-pack data are invented. */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestArgs } from "./inputManifestService";
import { recordRulePackExclusions, resolveModelRulePacks, RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED, runRulePackLines } from "./modelRulePackResolution";
import { rulePackRefDiagnostic, rulePackRefDiagnosticEntries, RULE_PACK_REF_DIAGNOSTIC_SOURCE } from "./rulePackRefDiagnostics";
import { ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH, analysisRecordProjection, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "./hashService";
import { RULE_PACK_BACKEND_DIAGNOSTIC, type RulePackOpenRoute } from "./rulePackService";
import { buildAnalysisRunPreview, loadPreviewModel } from "./previewService";
import { RulePackRunReferences } from "../features/rule-packs/RulePackRunReferences";
import { nativeMechanicsReplayPair } from "../test/nativeMechanicsReplay";
import { hex, inventedEnvelope, inventedReference } from "../test/inventedRulePacks";
import type { AnalysisRunEnvelope, PreviewModel, RulePackReference } from "../types";

afterEach(cleanup);

async function modelWith(references: RulePackReference[]): Promise<PreviewModel> {
  const model = await loadPreviewModel();
  return { ...model, rule_pack_refs: references };
}

function solver(): CurrentSessionInputManifestArgs["solver"] {
  return { solver_name: "invented-solver", solver_version: "0.0.1", solver_build_ref: "invented-solver@0.0.1", solver_mode: "sparse_interactive", settings: { sparse_evidence_lane: true } };
}

/** The manifest members before this change (1.0.0): the solve adds none. */
const MANIFEST_MEMBERS = ["active_rule_packs", "document_kind", "external_assets", "identity_policy", "load_basis", "model_basis", "replay_boundary", "schema_version", "solver_basis", "unit_basis"];

/** The preview pair's sparse solve as the session records it, its model given `references`
 * resolved against `store` (the result does not depend on rule_pack_refs). */
async function solvedWith(references: RulePackReference[] | null, store: Map<string, ReturnType<typeof inventedEnvelope>> = new Map()) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model);
  if (references) model.rule_pack_refs = references;
  const resolution = await resolveModelRulePacks(model, async (_projectId, id) => ({ route: "tauri_backend", envelope: store.get(id) ?? null }));
  const manifest = await buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: pair.source.producer!.component_name, solver_version: pair.source.producer!.component_version,
      solver_build_ref: `${pair.source.producer!.component_name}@${pair.source.producer!.component_version}`, solver_mode: pair.mode,
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true }
    },
    active_rule_packs: resolution.active_rule_packs,
    external_assets: []
  });
  const record = await buildAnalysisRunPreview(pair.source, { inputManifest: manifest, rulePackExclusions: resolution.excluded });
  return { pair, model, resolution, manifest, record };
}

const rehash = async (record: AnalysisRunEnvelope) => {
  record.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(record));
  return record;
};

describe("resolving the model's rule-pack references at solve time", () => {
  it("enters each resolved pack in the model's order, with the stored pack's version, checksum and notice; the manifest keeps its members", async () => {
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
      excluded: []
    });
    const manifest = await buildCurrentSessionInputManifest({ model, solver: solver(), active_rule_packs: resolution.active_rule_packs, external_assets: [] });
    // The model's order (the manifest no longer sorts the packs); no new member.
    expect(manifest.manifest.active_rule_packs.map((pack) => pack.rule_pack_id)).toEqual(["invented-pack-b", "invented-pack-a"]);
    expect(Object.keys(manifest.manifest).sort()).toEqual(MANIFEST_MEMBERS);
  });

  it("excludes an unresolved and a checksum-mismatched reference; each becomes a record diagnostic after the producer's, under the record hash", async () => {
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const store = new Map([
      ["invented-pack-held", inventedEnvelope(project, "invented-pack-held", hex("1"))],
      // Edited and re-stamped since the model referenced it.
      ["invented-pack-changed", inventedEnvelope(project, "invented-pack-changed", hex("9"))],
      // The stored stamp is the reference's, but the document no longer recomputes to it.
      ["invented-pack-unverified", inventedEnvelope(project, "invented-pack-unverified", hex("4"), { matchStatus: "mismatch_review_required" })]
    ]);
    const { pair, resolution, manifest, record } = await solvedWith([
      inventedReference("invented-pack-held", hex("1")),
      inventedReference("invented-pack-missing", hex("2")),
      inventedReference("invented-pack-changed", hex("3")),
      inventedReference("invented-pack-unverified", hex("4"))
    ], store);
    expect(resolution.active_rule_packs.map((pack) => pack.rule_pack_id)).toEqual(["invented-pack-held"]);
    expect(resolution.excluded).toEqual([
      { rule_pack_id: "invented-pack-missing", rule_pack_version: "1.0.0", code: RULE_PACK_REF_UNRESOLVED },
      { rule_pack_id: "invented-pack-changed", rule_pack_version: "1.0.0", code: RULE_PACK_REF_CHECKSUM_MISMATCH },
      { rule_pack_id: "invented-pack-unverified", rule_pack_version: "1.0.0", code: RULE_PACK_REF_CHECKSUM_MISMATCH }
    ]);
    expect(Object.keys(manifest.manifest).sort()).toEqual(MANIFEST_MEMBERS);
    const producer = pair.source.diagnostics.map((source_annotation) => ({ source_annotation }));
    expect(record.analysis_run.diagnostics!.slice(0, producer.length)).toEqual(producer);
    expect(record.analysis_run.diagnostics!.slice(producer.length)).toEqual([
      { source_annotation: {
        id: "diagnostic:rule-pack-ref:invented-pack-missing@1.0.0", code: RULE_PACK_REF_UNRESOLVED, severity: "warning",
        message: "Rule pack invented-pack-missing@1.0.0 is referenced by the model, but the local rule-pack store held no such pack for this project at solve time; it is not among this run's rule-pack references.",
        source: RULE_PACK_REF_DIAGNOSTIC_SOURCE, affected_refs: ["invented-pack-missing@1.0.0"] } },
      { source_annotation: {
        id: "diagnostic:rule-pack-ref:invented-pack-changed@1.0.0", code: RULE_PACK_REF_CHECKSUM_MISMATCH, severity: "warning",
        message: "Rule pack invented-pack-changed@1.0.0 is referenced by the model, but the stored pack's checksum was not the reference's at solve time; it is not among this run's rule-pack references.",
        source: RULE_PACK_REF_DIAGNOSTIC_SOURCE, affected_refs: ["invented-pack-changed@1.0.0"] } },
      { source_annotation: rulePackRefDiagnostic(RULE_PACK_REF_CHECKSUM_MISMATCH, "invented-pack-unverified@1.0.0") }
    ]);
    // The transient record names no rule packs until Record Run.
    expect(record.analysis_run.rule_pack_refs).toEqual([]);
    expect(await verifyAnalysisRunRecord(record)).toBe("match");
    await expect(validateAnalysisRunV03(record, pair.source, record.analysis_run.load_basis_refs)).resolves.toBeUndefined();
    // Read back for a rule-check revision of the record.
    expect(recordRulePackExclusions(record, manifest.manifest, pair.source.diagnostics.length)).toEqual(resolution.excluded);
    const lines = runRulePackLines(manifest.manifest, record)!;
    expect(lines.recorded).toEqual([`invented-pack-held v1.0.0 (sha256 ${hex("1").slice(0, 12)}…)`]);
    expect(lines.diagnostics.map((line) => line.split(" ")[0])).toEqual([RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_CHECKSUM_MISMATCH]);
  });

  it("the builder refuses exclusions that are not exactly the references the manifest leaves out", async () => {
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const { pair, resolution, manifest } = await solvedWith([inventedReference("invented-pack-held", hex("1")), inventedReference("invented-pack-missing", hex("2"))],
      new Map([["invented-pack-held", inventedEnvelope(project, "invented-pack-held", hex("1"))]]));
    await expect(buildAnalysisRunPreview(pair.source, { inputManifest: manifest })).rejects.toThrow(ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH);
    await expect(buildAnalysisRunPreview(pair.source, { inputManifest: manifest, rulePackExclusions: [...resolution.excluded, { rule_pack_id: "invented-pack-held", rule_pack_version: "1.0.0", code: RULE_PACK_REF_UNRESOLVED }] }))
      .rejects.toThrow(ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH);
  });

  it("the validator refuses a rule-pack diagnostic that is not exactly the one its code and identity determine, or one placed among the producer's", async () => {
    const { pair, record } = await solvedWith([inventedReference("invented-pack-missing", hex("2"))]);
    const producerCount = pair.source.diagnostics.length;
    const reworded = structuredClone(record);
    (reworded.analysis_run.diagnostics![producerCount].source_annotation as { message: string }).message = "Rule pack invented-pack-missing@1.0.0 was applied.";
    const recoded = structuredClone(record);
    (recoded.analysis_run.diagnostics![producerCount].source_annotation as { code: string }).code = "MODEL-RULE-PACK-REF-INVENTED";
    const extra = structuredClone(record);
    (extra.analysis_run.diagnostics![producerCount].source_annotation as Record<string, unknown>).note = "extra";
    const reordered = structuredClone(record);
    reordered.analysis_run.diagnostics!.unshift(reordered.analysis_run.diagnostics!.pop()!);
    const doubled = structuredClone(record);
    doubled.analysis_run.diagnostics!.push(structuredClone(doubled.analysis_run.diagnostics![producerCount]));
    for (const forged of [reworded, recoded, extra, reordered, doubled]) {
      await rehash(forged);
      await expect(validateAnalysisRunV03(forged, pair.source, forged.analysis_run.load_basis_refs)).rejects.toThrow("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH");
    }
    // The record hash covers them: an unrehashed edit is a checksum mismatch.
    const unhashed = structuredClone(record);
    unhashed.analysis_run.diagnostics!.pop();
    expect(await verifyAnalysisRunRecord(unhashed)).toBe("mismatch");
  });

  it("a long identity is shown bounded in the message and kept whole in affected_refs", () => {
    const identity = `${"x".repeat(200)}@1.0.0`;
    const annotation = rulePackRefDiagnostic(RULE_PACK_REF_UNRESOLVED, identity);
    expect(annotation.affected_refs).toEqual([identity]);
    expect(annotation.message).toContain(`${"x".repeat(160)}…`);
    expect(annotation.message).not.toContain("x".repeat(161));
    expect(rulePackRefDiagnosticEntries([{ source_annotation: annotation }])).toEqual([{ code: RULE_PACK_REF_UNRESOLVED, identity }]);
  });

  it("a different stored version under the same checksum, or a stored pack without a usable notice, does not enter", async () => {
    const model = await modelWith([inventedReference("invented-pack-v", hex("c")), inventedReference("invented-pack-n", hex("d"))]);
    const store = new Map([
      ["invented-pack-v", inventedEnvelope(model.project.id, "invented-pack-v", hex("c"), { version: "1.0.1" })],
      ["invented-pack-n", inventedEnvelope(model.project.id, "invented-pack-n", hex("d"), { notice: "  " })]
    ]);
    const resolution = await resolveModelRulePacks(model, async (_projectId, id) => ({ route: "tauri_backend", envelope: store.get(id) ?? null }));
    expect(resolution.active_rule_packs).toEqual([]);
    expect(resolution.excluded.map((item) => item.code)).toEqual([RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED]);
  });

  it("a store that fails leaves the reference unresolved", async () => {
    const model = await modelWith([inventedReference("invented-pack-a", hex("a"))]);
    const resolution = await resolveModelRulePacks(model, async () => { throw new Error("invented store failure"); });
    expect(resolution).toEqual({ active_rule_packs: [], excluded: [{ rule_pack_id: "invented-pack-a", rule_pack_version: "1.0.0", code: RULE_PACK_REF_UNRESOLVED }] });
  });

  it("the browser preview has no rule-pack store: every reference stays unresolved", async () => {
    expect("__TAURI_INTERNALS__" in window).toBe(false);
    const model = await modelWith([inventedReference("invented-pack-a", hex("a"))]);
    // The real store route: the browser preview answers unavailable_browser_preview.
    expect(await resolveModelRulePacks(model)).toEqual({ active_rule_packs: [], excluded: [{ rule_pack_id: "invented-pack-a", rule_pack_version: "1.0.0", code: RULE_PACK_REF_UNRESOLVED }] });
    expect(RULE_PACK_BACKEND_DIAGNOSTIC).toMatch(/^RULE-PACK-BACKEND-DESKTOP-ONLY/);
  });

  it("a model without references makes no store call, and its manifest and record are those built without any rule-pack input", async () => {
    const model = await loadPreviewModel();
    expect(model.rule_pack_refs ?? []).toEqual([]);
    let opened = 0;
    expect(await resolveModelRulePacks(model, async () => { opened += 1; return { route: "tauri_backend", envelope: null }; })).toEqual({ active_rule_packs: [], excluded: [] });
    expect(opened).toBe(0);
    const { pair, manifest, record } = await solvedWith(null);
    const plain = await buildAnalysisRunPreview(pair.source, { inputManifest: manifest });
    expect(await canonicalJsonString(record)).toBe(await canonicalJsonString(plain));
    expect(record.analysis_run.diagnostics).toEqual(pair.source.diagnostics.map((source_annotation) => ({ source_annotation })));
    expect(runRulePackLines(manifest.manifest, record)).toBeNull();
  });
});

describe("the rule-pack references shown with a run", () => {
  it("lists the recorded packs and each diagnostic from the record, and claims no check, application or pass", async () => {
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const { manifest, record } = await solvedWith([inventedReference("invented-pack-held", hex("1")), inventedReference("invented-pack-missing", hex("2"))],
      new Map([["invented-pack-held", inventedEnvelope(project, "invented-pack-held", hex("1"))]]));
    render(<RulePackRunReferences manifest={manifest.manifest} record={record} testId="refs" />);
    const block = screen.getByTestId("refs");
    expect(screen.getByTestId("refs-recorded").textContent).toContain("invented-pack-held v1.0.0");
    expect(screen.getByTestId("refs-diagnostic").textContent).toMatch(/^MODEL-RULE-PACK-REF-UNRESOLVED \(warning\): Rule pack invented-pack-missing@1\.0\.0/);
    expect(block.textContent).toContain("does not read, apply or check a rule pack");
    expect(block.textContent).not.toMatch(/\b(passed|applied to|was checked|compliant|approved|accepted)\b/i);
  });

  it("shows nothing for a run whose model referenced no pack", async () => {
    const { manifest, record } = await solvedWith(null);
    render(<RulePackRunReferences manifest={manifest.manifest} record={record} testId="refs" />);
    expect(screen.queryByTestId("refs")).toBeNull();
  });
});
