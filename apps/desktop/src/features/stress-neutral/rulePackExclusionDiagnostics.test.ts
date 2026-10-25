/** The run record's rule-pack exclusion diagnostics reach the stress-neutral package (#1602's
 * follow-up): a model reference the solve left out of the input manifest's `active_rule_packs`
 * (unresolved, or a checksum mismatch) appears once among the package's diagnostics, after the
 * package's own, as the record names it, and the validator holds the package to the record.
 * The solve is the native job transport replay of committed producer bytes (unit transport
 * replay; NOT a native UI qualification). All rule-pack data are invented. */
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { hex, inventedEnvelope, inventedReference } from "../../test/inventedRulePacks";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { resolveModelRulePacks } from "../../services/modelRulePackResolution";
import { modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED, rulePackRefDiagnostic } from "../../services/rulePackRefDiagnostics";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "./StressNeutralExportPanel";
import type { RulePackReference } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

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
const RULE_PACK_CODES: string[] = [RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH];

/** The preview pair's sparse solve as the session runs and records it (#1602): its model given
 * `references`, resolved against the invented store; `edit` may change the producer's result. */
async function solvedWith(references: RulePackReference[] | null, edit?: (source: Json) => void) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model);
  if (references) model.rule_pack_refs = references;
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    if (!edit) return pair.bytes;
    const source = JSON.parse(pair.bytes);
    edit(source);
    return JSON.stringify(source);
  });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model);
  const resolution = await resolveModelRulePacks(model, async (_projectId, id) => ({ route: "tauri_backend", envelope: STORE.get(id) ?? null }));
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "unit-transport-replay", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: resolution.active_rule_packs,
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest, rulePackExclusions: resolution.excluded });
  return { model, result, analysisRun, resolution };
}

const ruleRefDiagnostics = (packet: Json) => (packet.diagnostics as Json[]).filter((item) => RULE_PACK_CODES.includes(item.code));

async function rawSha(text: string) { const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)); return [...new Uint8Array(digest)].map(x => x.toString(16).padStart(2, "0")).join(""); }
/** Recomputes every member, manifest and package checksum after a deliberate tamper (the recipe
 * of `retainedPrecisionStressNeutral.test.tsx`), so that only the relational check can refuse. */
async function rehash(packet: Json) {
  const payloads: Json = { "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure, "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report, "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics };
  const non = packet.manifest.checksums.filter((x: Json) => x.payload_ref.ref !== "manifest.json");
  for (const c of non) c.value = c.payload_ref.ref.endsWith(".csv") ? await rawSha(payloads[c.payload_ref.ref]) : await canonicalSha256HexCheckedV1(payloads[c.payload_ref.ref]);
  const seed = { manifest_id: packet.manifest.manifest_id, source_result_ref: packet.source_result_ref, source_run_ref: packet.source_run_ref, source_model_ref: packet.source_model_ref, received_source_checksums: packet.received_source_checksums, unresolved_assumption_refs: packet.unresolved_assumption_refs, reproducibility_refs: packet.reproducibility_refs, export_profile_ref: packet.manifest.export_profile_ref, boundary_notes: packet.manifest.boundary_notes, member_checksums: non, diagnostics: packet.diagnostics };
  const m = packet.manifest.checksums.find((x: Json) => x.payload_ref.ref === "manifest.json"); m.value = await canonicalSha256HexCheckedV1(seed);
  const by = new Map(packet.manifest.checksums.map((x: Json) => [x.payload_ref.ref, x]));
  for (const member of packet.manifest.package_members) member.checksum = structuredClone(by.get(member.filename));
  const projection = structuredClone(packet); delete projection.package_checksum;
  packet.package_checksum.value = await canonicalSha256HexCheckedV1(projection);
  return packet;
}

describe("the stress-neutral package carries the run record's rule-pack exclusion diagnostics", () => {
  it("names each excluded reference once, after the package's own diagnostics; the validator holds it to the record", async () => {
    const session = await solvedWith([HELD, MISSING, CHANGED]);
    expect(session.resolution.excluded.map((item) => item.code)).toEqual(RULE_PACK_CODES);
    const packet = await buildStressNeutralExportPacket(session);
    const expected = [[RULE_PACK_REF_UNRESOLVED, "invented-pack-missing@1.0.0"], [RULE_PACK_REF_CHECKSUM_MISMATCH, "invented-pack-changed@1.0.0"]].map(([code, identity]) => {
      const recorded = rulePackRefDiagnostic(code as typeof RULE_PACK_REF_UNRESOLVED, identity);
      expect(session.analysisRun.analysis_run.diagnostics).toContainEqual({ source_annotation: recorded });
      return {
        code, class: "rule_pack_reference", severity: "warning",
        source: { object_type: "AnalysisRun", ref: session.result.run_id },
        affected_object: { object_type: "RulePack", ref: identity },
        message: recorded.message,
        remediation: "Review the excluded rule-pack reference before downstream use; it is not among this run's rule-pack references.",
        provenance: packet.provenance
      };
    });
    expect(ruleRefDiagnostics(packet)).toStrictEqual(expected);
    expect(packet.diagnostics.slice(-2)).toStrictEqual(expected);
    // Warnings: they do not block the package.
    expect(packet.validation_ready).toBe(packet.diagnostics.every((item: Json) => item.severity !== "blocking"));
    const basis = modelLoadBasisRefs(session.model);
    await expect(validateStressNeutralExportPacket(packet, session.result, session.analysisRun, basis)).resolves.toBeUndefined();
    // A package that drops, repeats or alters one no longer matches its record.
    for (const tamper of [
      (p: Json) => { p.diagnostics.pop(); },
      (p: Json) => { p.diagnostics.push(structuredClone(p.diagnostics.at(-1))); },
      (p: Json) => { p.diagnostics.at(-1).affected_object.ref = "invented-pack-held@1.0.0"; }
    ]) {
      const changed = structuredClone(packet);
      tamper(changed);
      await expect(validateStressNeutralExportPacket(changed, session.result, session.analysisRun, basis)).rejects.toThrow("SN-RULE-PACK-REF-DIAGNOSTICS-MISMATCH");
    }
  }, 120_000);

  it("the validator finds the exclusion diagnostics by their codes, whatever their class, and expects them from the record alone", async () => {
    const session = await solvedWith([HELD, MISSING, CHANGED]);
    const packet = await buildStressNeutralExportPacket(session);
    const basis = modelLoadBasisRefs(session.model);
    const unresolved = (p: Json) => (p.diagnostics as Json[]).find((item) => item.code === RULE_PACK_REF_UNRESOLVED);
    // The restamp alone changes nothing the validator refuses.
    await expect(validateStressNeutralExportPacket(await rehash(structuredClone(packet)), session.result, session.analysisRun, basis)).resolves.toBeUndefined();
    for (const tamper of [
      // (a) An exclusion relabelled to another class, its message and severity altered: it must
      // not count as one of the package's own diagnostics and so suppress the record's.
      (p: Json) => { Object.assign(unresolved(p), { class: "comparison", severity: "info", message: "Invented altered message." }); },
      // (b) An extra reserved-code diagnostic under another class, naming a reference the record
      // does not exclude.
      (p: Json) => {
        const extra = { ...structuredClone(unresolved(p)), class: "stress_neutral_export", affected_object: { object_type: "RulePack", ref: "invented-pack-held@1.0.0" } };
        p.diagnostics.splice(0, 0, extra);
      }
    ]) {
      const changed = structuredClone(packet);
      tamper(changed);
      await expect(validateStressNeutralExportPacket(await rehash(changed), session.result, session.analysisRun, basis)).rejects.toThrow("SN-RULE-PACK-REF-DIAGNOSTICS-MISMATCH");
    }
  }, 120_000);

  it("the exclusion diagnostics' provenance is the desktop builder's, not the package's top-level provenance", async () => {
    const session = await solvedWith([HELD, MISSING, CHANGED]);
    const packet = await buildStressNeutralExportPacket(session);
    const basis = modelLoadBasisRefs(session.model);
    const substituted = { ...structuredClone(packet.provenance), source_name: "Invented substituted exporter" };
    const withholding = (item: Json) => typeof item.code === "string" && (item.code.startsWith("SN-UNIT-WITNESS-WITHHELD") || item.code === "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE");
    // A coordinated substitution: the top-level provenance and the exclusion diagnostics' (and
    // any witness-withholding finding's, which are bound to the top level), restamped.
    const coordinated = structuredClone(packet);
    coordinated.provenance = structuredClone(substituted);
    for (const item of coordinated.diagnostics as Json[]) if (RULE_PACK_CODES.includes(item.code) || withholding(item)) item.provenance = structuredClone(substituted);
    await expect(validateStressNeutralExportPacket(await rehash(coordinated), session.result, session.analysisRun, basis)).rejects.toThrow("SN-RULE-PACK-REF-DIAGNOSTICS-MISMATCH");
    // Another exporter's top-level provenance (the interop shape the validator admits) leaves
    // the builder's exclusion diagnostics, which only the desktop builder emits, as they are.
    const interop = structuredClone(packet);
    interop.provenance = structuredClone(substituted);
    for (const item of interop.diagnostics as Json[]) if (withholding(item)) item.provenance = structuredClone(substituted);
    await expect(validateStressNeutralExportPacket(await rehash(interop), session.result, session.analysisRun, basis)).resolves.toBeUndefined();
  }, 120_000);

  it("a producer diagnostic naming the same exclusion neither suppresses nor repeats the package's one", async () => {
    // The duplicate case the other outputs test cannot arise here: the package carries none of the
    // producer's diagnostics, and none of its own (comparison, witness-withholding) has a rule-pack
    // code, so nothing already in the package can name an exclusion. What can happen is a producer
    // diagnostic naming it, which must neither suppress nor repeat the record's projection.
    const carried = { id: "invented-producer-diagnostic", code: RULE_PACK_REF_UNRESOLVED, severity: "warning", message: "Invented producer diagnostic.", source: "invented", affected_refs: ["invented-pack-missing@1.0.0"] };
    const session = await solvedWith([HELD, MISSING, CHANGED], (source) => { source.diagnostics.push(carried); });
    expect(session.result.diagnostics).toContainEqual(carried);
    const packet = await buildStressNeutralExportPacket(session);
    expect(ruleRefDiagnostics(packet).map((item: Json) => [item.code, item.affected_object.ref])).toStrictEqual([
      [RULE_PACK_REF_UNRESOLVED, "invented-pack-missing@1.0.0"], [RULE_PACK_REF_CHECKSUM_MISMATCH, "invented-pack-changed@1.0.0"]
    ]);
  }, 120_000);

  it("without exclusions the package's diagnostics are its own alone", async () => {
    for (const references of [null, [HELD]]) {
      const session = await solvedWith(references);
      expect(session.resolution.excluded).toEqual([]);
      const packet = await buildStressNeutralExportPacket(session);
      expect(ruleRefDiagnostics(packet)).toEqual([]);
      expect(packet.diagnostics.some((item: Json) => item.class === "rule_pack_reference")).toBe(false);
      // The diagnostics equal those of the package of the same solve without references.
      const plain = await buildStressNeutralExportPacket(await solvedWith(null));
      expect(await canonicalJsonString(packet.diagnostics)).toBe(await canonicalJsonString(plain.diagnostics));
    }
  }, 120_000);
});
