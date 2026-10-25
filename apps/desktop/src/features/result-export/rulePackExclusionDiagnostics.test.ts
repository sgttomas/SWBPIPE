/** The run record's rule-pack exclusion diagnostics reach the result-export document (#1602's
 * follow-up): a model reference the solve left out of the input manifest's `active_rule_packs`
 * (unresolved, or a checksum mismatch) appears once among the document's diagnostics, after the
 * source's, as the record names it. The solve is the native job transport replay of committed
 * producer bytes (unit transport replay; NOT a native UI qualification). All rule-pack data are
 * invented. */
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { hex, inventedEnvelope, inventedReference } from "../../test/inventedRulePacks";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { resolveModelRulePacks } from "../../services/modelRulePackResolution";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED, rulePackRefDiagnostic } from "../../services/rulePackRefDiagnostics";
import { canonicalJsonString } from "../../services/hashService";
import { buildCurrentResultExport, currentResultDocumentBase, derivativeProvenance, validateResultDocument, type JsonObject } from "./resultExportAdapter";
import type { MechanicsResult, RulePackReference } from "../../types";

afterEach(() => {
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const HELD = inventedReference("invented-pack-held", hex("1"));
const MISSING = inventedReference("invented-pack-missing", hex("2"));
const CHANGED = inventedReference("invented-pack-changed", hex("3"));
const PROJECT = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
const STORE = new Map([
  [HELD.rule_pack_id, inventedEnvelope(PROJECT, HELD.rule_pack_id, hex("1"))],
  [CHANGED.rule_pack_id, inventedEnvelope(PROJECT, CHANGED.rule_pack_id, hex("4"))]
]);

/** The preview pair's sparse solve as the session runs and records it (#1602): its model given
 * `references`, resolved against the invented store; `edit` may change the producer's result. */
async function solvedWith(references: RulePackReference[] | null, edit?: (source: MechanicsResult) => void) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model);
  if (references) model.rule_pack_refs = references;
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    if (!edit) return pair.bytes;
    const source = JSON.parse(pair.bytes) as MechanicsResult;
    edit(source);
    return JSON.stringify(source);
  });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model);
  const resolution = await resolveModelRulePacks(model, async (_projectId, id) => ({ route: "tauri_backend", envelope: STORE.get(id) ?? null }));
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: resolution.active_rule_packs,
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest, rulePackExclusions: resolution.excluded });
  return { model, result, inputManifest, analysisRun, resolution };
}

const withCode = (doc: JsonObject, code: string) => (doc.result_envelope.diagnostics as JsonObject[]).filter((item) => item.code === code);

describe("the result-export document carries the run record's rule-pack exclusion diagnostics", () => {
  it("names each excluded reference once, after the source's diagnostics, as the record does", async () => {
    const session = await solvedWith([HELD, MISSING, CHANGED]);
    expect(session.resolution.excluded.map((item) => item.code)).toEqual([RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH]);
    const doc = await buildCurrentResultExport(session);
    const diagnostics = doc.result_envelope.diagnostics as JsonObject[];
    expect(diagnostics).toHaveLength(session.result.diagnostics.length + 2);
    for (const [code, identity] of [[RULE_PACK_REF_UNRESOLVED, "invented-pack-missing@1.0.0"], [RULE_PACK_REF_CHECKSUM_MISMATCH, "invented-pack-changed@1.0.0"]] as const) {
      const recorded = rulePackRefDiagnostic(code, identity);
      expect(session.analysisRun.analysis_run.diagnostics).toContainEqual({ source_annotation: recorded });
      expect(withCode(doc, code)).toStrictEqual([{
        code, class: "ASSUMPTION_WARNING", severity: "warning",
        source: { ref_type: "desktop_component", ref_id: recorded.source },
        affected_object: { ref_type: "rule_pack_ref", ref_id: identity },
        message: recorded.message,
        remediation: "Review the excluded rule-pack reference before relying on this exported result envelope; it is not among this run's rule-pack references.",
        provenance: derivativeProvenance
      }]);
    }
    // The source's diagnostics come first, unchanged.
    expect(diagnostics.slice(0, session.result.diagnostics.length).map((item) => item.affected_object.ref_id)).toEqual(session.result.diagnostics.map((item) => item.id));
    await expect(validateResultDocument(doc, session.result)).resolves.toBeUndefined();
  }, 120_000);

  it("adds no second diagnostic for an exclusion a source diagnostic already carries", async () => {
    // A producer that itself names the unresolved reference (same code, same reference).
    const carried = { id: "invented-producer-diagnostic", code: RULE_PACK_REF_UNRESOLVED, severity: "warning" as const, message: "Invented producer diagnostic.", source: "invented", affected_refs: ["invented-pack-missing@1.0.0"] };
    const session = await solvedWith([HELD, MISSING, CHANGED], (source) => { source.diagnostics.push(carried); });
    expect(session.result.diagnostics).toContainEqual(carried);
    // The record still names both exclusions after the producer's diagnostics.
    expect(session.analysisRun.analysis_run.diagnostics!.slice(session.result.diagnostics.length).map((item) => item.source_annotation.code)).toEqual([RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH]);
    const doc = await buildCurrentResultExport(session);
    const unresolved = withCode(doc, RULE_PACK_REF_UNRESOLVED);
    expect(unresolved).toHaveLength(1);
    expect(unresolved[0].affected_object).toStrictEqual({ ref_type: "preview_diagnostic", ref_id: carried.id });
    expect(withCode(doc, RULE_PACK_REF_CHECKSUM_MISMATCH)).toHaveLength(1);
    expect(doc.result_envelope.diagnostics).toHaveLength(session.result.diagnostics.length + 1);
  }, 120_000);

  it("without exclusions the document's diagnostics are the source's alone, byte for byte", async () => {
    for (const references of [null, [HELD]]) {
      const session = await solvedWith(references);
      expect(session.resolution.excluded).toEqual([]);
      const doc = await buildCurrentResultExport(session);
      const { diagnostics: _recorded, ...withoutDiagnostics } = session.analysisRun.analysis_run;
      const sourceOnly = currentResultDocumentBase(session.model, session.result, withoutDiagnostics, session.inputManifest.manifest_ref.ref, session.inputManifest.manifest.solver_basis);
      expect(await canonicalJsonString(doc.result_envelope.diagnostics)).toBe(await canonicalJsonString(sourceOnly.result_envelope.diagnostics));
      expect(doc.result_envelope.diagnostics).toHaveLength(session.result.diagnostics.length);
    }
  }, 120_000);
});
