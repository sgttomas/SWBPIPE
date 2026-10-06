/** I75 scratch probe (never committed): writes TS successor result documents for schema validation. */
import { expect, it, vi } from "vitest";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalJsonString, canonicalSha256Hex, checkedJsonText } from "../../services/hashService";
import { runPreviewMechanics } from "../../services/previewService";
import { registerRetainedPrecision } from "../results/retainedPrecisionStanding";
import { buildCurrentResultExport, currentReceivedOrigin, currentResultDocumentBase, deriveResultDocument } from "./resultExportAdapter";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const json = (p: string) => JSON.parse(readFileSync(resolve(root, p), "utf8"));
async function manifestFor(model: PreviewModel, source: MechanicsResult, mode: string) {
  const manifest: Json = { schema_version: "1.0.0", document_kind: "openpipestress.current_session_input_manifest", model_basis: { model_ref: model.project.id, model_payload: structuredClone(model) }, unit_basis: { project_units: Object.fromEntries(Object.entries(model.project.units ?? {}).sort(([a], [b]) => a.localeCompare(b))) }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:probe", solver_mode: mode, settings: {} }, load_basis: { load_cases: structuredClone(model.load_cases), combinations: structuredClone(model.combinations ?? []) }, active_rule_packs: [], external_assets: [], identity_policy: { canonicalization: "rfc8785_jcs", hash_algorithm: "sha256", canonical_bytes_scope: "entire_input_manifest_object" }, replay_boundary: { included_as_package_member: false, portable_replay_claimed: false, current_session_ref_hash_integrity_only: true } };
  const digest = await canonicalSha256Hex(manifest);
  const token = model.project.id.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "current-session";
  return { manifest, manifest_ref: { object_type: "InputManifest", ref: `input-manifest:${token}:${digest}` }, manifest_sha256: digest, canonical_bytes: await canonicalJsonString(manifest) } as Json;
}
it("writes successor result documents", async () => {
  const out = process.env.I75_OUT!; mkdirSync(out, { recursive: true });
  for (const mode of ["sparse_interactive", "dense_scrutiny"] as const) {
    const doc = json(`fixtures/results/retained_precision_milestone_successor_${mode}.json`);
    const model = doc.invocation.request.model as PreviewModel;
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async () => structuredClone(doc.source));
    const received = await runPreviewMechanics(model, mode);
    const inputManifest = await manifestFor(model, received, mode);
    const analysisRun = await buildAnalysisRunV03(received, inputManifest, undefined, modelLoadBasisRefs(model));
    writeFileSync(resolve(out, `rd_current_${mode}.json`), await canonicalJsonString(await buildCurrentResultExport({ model, result: received, analysisRun, inputManifest })));
  }
  const corpus = json("fixtures/results/retained_precision_cases.json");
  const entry = corpus.cases.find((c: Json) => c.id === "two_case_synthetic");
  const source = structuredClone(entry.source), model = structuredClone(entry.invocation.request.model);
  const captured = checkedJsonText(model);
  await registerRetainedPrecision(source, structuredClone(entry.invocation), (m) => checkedJsonText(m) === captured);
  const run: Json = { run_id: source.run_id, load_basis_refs: modelLoadBasisRefs(model), hashes: [], analysis_status: ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"], professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false } };
  const base = currentResultDocumentBase(model, source, run, "test:probe", { solver_name: source.producer.component_name, solver_version: source.producer.component_version, solver_build_ref: "test:probe" });
  const origin = await currentReceivedOrigin(model, source, "test:probe", false, "Test-built origin (probe); not a qualified Current received carrier.");
  writeFileSync(resolve(out, "rd_two_case_synthetic.json"), await canonicalJsonString(await deriveResultDocument(base, model, source, origin)));
  expect(true).toBe(true);
});
