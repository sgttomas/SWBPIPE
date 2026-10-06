/** RV101 reviewer differential (scratch; placed only in the reviewer's archive copies).
 * The same file runs in the base copy (c1bfc460fc) and the candidate copy (2033260c57).
 * For every committed MechanicsResult-shaped fixture in RV101_MANIFEST it records, as sha256 of
 * canonical JSON or as the error text, the outcome of:
 *  - the shared refusal (loadReferenceOutputRefusal);
 *  - the pure projection deriveResultDocument (both origin scopes) and validateResultDocument;
 *  - the Current path through mocked IPC: runPreviewMechanics registration, buildCurrentResultExport;
 *  - stress-neutral: buildStressNeutralExportPacket with the registered result and a V0.3 preview
 *    AnalysisRun, and with an unregistered clone and a V0.2 AnalysisRun; then
 *    validateStressNeutralExportPacket with and without the source.
 * Uses only symbols present in both revisions. Writes JSONL to RV101_OUT and bytes to RV101_BYTES. */
import { afterEach, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../types";
import { canonicalJsonString } from "../services/hashService";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import { buildAnalysisRunPreview, bindSourceResultDimensions, runPreviewMechanics, type PreviewSolverMode } from "../services/previewService";
import { buildAnalysisRunV02, modelLoadBasisRefs } from "../services/analysisRunCompatibility";
import { buildCurrentResultExport, deriveResultDocument, validateResultDocument, resultDigest, ref, derivativeProvenance } from "../features/result-export/resultExportAdapter";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "../features/stress-neutral/StressNeutralExportPanel";
import { loadReferenceOutputRefusal } from "../features/results/loadReferenceOutputAvailability";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => { invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const P = resolve(__dirname, "../../../../");
const OUT = process.env.RV101_OUT!, BYTES = process.env.RV101_BYTES!;
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
function load(path: string, keys: string[]): Json { let d = JSON.parse(readFileSync(resolve(P, path), "utf8")); for (const k of keys) d = d[k]; return d; }
function record(id: string, step: string, outcome: { text?: string; error?: unknown; value?: unknown }) {
  const line: Json = { id, step };
  if (outcome.error !== undefined) { line.error = outcome.error instanceof Error ? outcome.error.message : String(outcome.error); if (process.env.RV101_STACK && outcome.error instanceof Error) line.stack = (outcome.error.stack ?? "").split("\n").slice(0, 8).join(" | "); }
  else if (outcome.text !== undefined) { line.sha256 = sha(outcome.text); line.bytes = outcome.text.length; writeFileSync(resolve(BYTES, sha(id + "\u0000" + step) + ".json"), outcome.text); }
  else line.value = outcome.value;
  appendFileSync(OUT, JSON.stringify(line) + "\n");
}
async function attempt(id: string, step: string, f: () => Promise<unknown> | unknown, asText = true): Promise<unknown> {
  try { const v = await f(); record(id, step, asText ? { text: v === undefined ? "undefined" : await canonicalJsonString(v) } : { value: v ?? null }); return v; }
  catch (error) { record(id, step, { error }); return undefined; }
}
/** RV101's own copy of the base-revision builder's base and origin (c1bfc460fc resultExportAdapter.ts),
 * with fixed stand-ins; identical in both copies, so only the projection under test can differ. */
async function baseAndOrigin(model: PreviewModel, result: MechanicsResult, enriched: boolean) {
  const scope = enriched ? "received_current_legacy_enriched_carrier" : "received_current_dimension_absent_carrier";
  const origin = { origin_id: "source-origin:rv101", origin_class: enriched ? "received_current_qualified_legacy_enriched" : "received_current_dimension_absent", qualification_ref: ref("current_manifest", "rv101:manifest"), authentic_producer_available: false, received_carrier_checksum: { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: scope, payload_ref: ref("received_current_carrier", result.run_id), value: await resultDigest(result) }, original_producer_checksum: null, origin_limit: "RV101 differential stand-in origin; not a qualified Current carrier", actual_model_ref: ref("model_payload", model.project.id), mechanics_run_ref: ref("mechanics_run", result.run_id), request_model_ref: null, request_run_ref: null, request_alias_disclosure: null };
  let basis: Json[] = []; try { basis = modelLoadBasisRefs(model) as Json[]; } catch { basis = []; }
  const provenance = derivativeProvenance;
  const base = { schema_version: "0.2.0", deliverable_id: "DEL-08-04", package_id: "PKG-08", scope_item: "SOW-046", objectives: ["OBJ-007", "OBJ-009"], export_format_status: { baseline_format: "schema_first_json_result_envelope", additional_formats: "TBD", public_transport_protocol: "TBD", local_fea_package_format: "TBD", external_adapter_formats: "TBD" }, result_envelope: { schema_version: "0.2.0", envelope_id: `result-envelope:${result.run_id}`, model_ref: ref("model_payload", model.project.id), run_ref: ref("analysis_run", result.run_id), solver_version: { solver_name: "rv101", solver_version: "0", solver_build_ref: "rv101:stand-in" }, unit_system_ref: ref("unit_system", `${model.project.id}:units`), load_basis_refs: basis.map((x: Json) => ref(x.object_type, x.ref)), result_sets: [{ set_id: `result-set:${result.run_id}:mechanics`, set_type: "mechanics", basis_ref: ref("analysis_run", result.run_id), values: [] }], diagnostics: (result.diagnostics ?? []).map((x: Json) => ({ code: x.code, class: "ASSUMPTION_WARNING", severity: x.severity === "error" ? "blocking" : x.severity, source: ref("source", x.source ?? "local_preview"), affected_object: ref("preview_entity", x.affected_refs?.[0] ?? result.model_ref), message: x.message, remediation: "Review source model and preview limitations.", provenance })), provenance, reproducibility: { model_hash: null, run_hashes: [], audit_manifest_ref: ref("audit_manifest", "rv101:manifest"), deterministic_ordering: true }, analysis_status: ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED"], professional_boundary: { human_review_required: true }, downstream_use: { review: true, regression_comparison: true, report_consumption: true, headless_automation: true, governed_downstream_tooling: true, additional_export_formats: "TBD" } } };
  return { base, origin };
}

it("RV101 differential over committed MechanicsResult fixtures", async () => {
  mkdirSync(BYTES, { recursive: true });
  const manifest: Json[] = JSON.parse(readFileSync(process.env.RV101_MANIFEST!, "utf8"));
  for (const e of manifest) {
    delete (window as Json).__TAURI_INTERNALS__; invokeMock.mockReset();
    try { await oneEntry(e); } finally { delete (window as Json).__TAURI_INTERNALS__; invokeMock.mockReset(); }
  }
  expect(manifest.length).toBeGreaterThan(0);
}, 3_600_000);

async function oneEntry(e: Json) {
  {
    const id: string = e.id;
    const raw: MechanicsResult = load(e.result_path, e.result_keys);
    const model: PreviewModel = load(e.model_path, e.model_keys);
    const mode = e.mode as PreviewSolverMode;
    await attempt(id, "shared_refusal", () => loadReferenceOutputRefusal(structuredClone(raw)), false);
    // The pure projection, as received and with bound dimensions (the enriched origin scope).
    for (const [variant, make] of [["plain", (r: MechanicsResult) => r], ["bound", (r: MechanicsResult) => bindSourceResultDimensions(r)]] as const) {
      let src: MechanicsResult; try { src = make(structuredClone(raw)); } catch (error) { record(id, `derive_${variant}`, { error }); continue; }
      const enriched = src.results.every(row => Object.hasOwn(row, "dimension"));
      let bo: { base: Json; origin: Json }; try { bo = await baseAndOrigin(model, src, enriched); } catch (error) { record(id, `derive_${variant}`, { error }); continue; }
      const before = await canonicalJsonString(src);
      const doc = await attempt(id, `derive_${variant}`, () => deriveResultDocument(structuredClone(bo.base), structuredClone(model), src, structuredClone(bo.origin)));
      record(id, `derive_${variant}_input_unchanged`, { value: (await canonicalJsonString(src)) === before });
      if (doc) await attempt(id, `validate_${variant}`, () => validateResultDocument(doc as Json, src), false);
    }
    // The Current path through mocked IPC (the product's own capture and registration).
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command: string) => { if (command !== "run_preview_mechanics_with_solver_mode") throw new Error("RV101 mock: unexpected command " + command); return structuredClone(raw); });
    let received: MechanicsResult | undefined;
    try { received = await runPreviewMechanics(structuredClone(model), mode); } catch (error) { record(id, "ipc", { error }); }
    if (!received) return;
    let manifestEvidence: Json, run03: Json, run02: Json;
    try { manifestEvidence = await buildCurrentSessionInputManifest({ model, solver: { solver_name: received.producer?.component_name ?? "fixture", solver_version: received.producer?.component_version ?? "1", solver_build_ref: "rv101:fixture", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] } as Json); } catch (error) { record(id, "manifest", { error }); return; }
    try { run03 = await buildAnalysisRunPreview(received, { inputManifest: manifestEvidence }); record(id, "analysis_run_preview", { text: await canonicalJsonString(run03) }); } catch (error) { record(id, "analysis_run_preview", { error }); }
    try { run02 = await buildAnalysisRunV02(structuredClone(raw), manifestEvidence); record(id, "analysis_run_v02", { text: await canonicalJsonString(run02) }); } catch (error) { record(id, "analysis_run_v02", { error }); }
    if (run03) await attempt(id, "current_result_export", () => buildCurrentResultExport({ model, result: received!, analysisRun: run03, inputManifest: manifestEvidence }));
    for (const [variant, result, run] of [["registered_preview", received, run03], ["registered_v02", received, run02], ["clone_v02", structuredClone(raw), run02], ["clone_preview", structuredClone(raw), run03]] as const) {
      if (!run) continue;
      const packet = await attempt(id, `sn_${variant}`, () => buildStressNeutralExportPacket({ model, result: result as MechanicsResult, analysisRun: run }));
      if (packet) {
        const p = packet as Json;
        await attempt(id, `sn_${variant}_validate_with_source`, () => validateStressNeutralExportPacket(structuredClone(p), p.schema_version === "0.3.0" ? result as MechanicsResult : undefined, p.schema_version === "0.3.0" ? run : undefined, p.schema_version === "0.3.0" ? modelLoadBasisRefs(model) : undefined), false);
        await attempt(id, `sn_${variant}_validate_header_only`, () => validateStressNeutralExportPacket(structuredClone(p)), false);
      }
    }
    delete (window as Json).__TAURI_INTERNALS__;
  }
}
