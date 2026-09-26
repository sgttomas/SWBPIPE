/** No lossy projection of 0.4.0 models or load/reference-state results anywhere
 * they pass through the desktop (T1 WP2): session clone, browser persistence,
 * input manifest, native IPC capture, the wasm operation route and saved
 * results. Absent keys stay absent, explicit nulls stay null, and a key the
 * desktop does not model survives byte-exactly. All values are invented. */
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, EditorOperationIntent, MechanicsResult, PreviewModel } from "../types";
import { clonePreviewModel } from "../features/workspace/sessionModel";
import { createLocalProject, evaluateModelDocumentLocal, LOAD_REFERENCE_MODEL_SCHEMA_VERSION, modelDocumentVersionCheckStatus, openLocalProject, saveLocalProject } from "./projectService";
import { buildCurrentSessionInputManifest } from "./inputManifestService";
import { runPreviewMechanics, hasNativeMechanicsInvocation, retainedNativeMechanicsInvocation } from "./previewService";
import { applyModelOperation } from "./operationService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";

afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const text = (value: unknown) => JSON.stringify(value);

/** The committed connected request model, completed with the case metadata the
 * desktop needs, plus an explicit null and a key the desktop does not model. */
function model040(): PreviewModel {
  const model = structuredClone(json("fixtures/product_preview/load_reference/connected.request.json").model) as PreviewModel & Record<string, unknown>;
  for (const loadCase of model.load_cases) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  Object.assign(model, { data_boundary: { source: "invented" }, diagnostics: [] });
  (model.reference_configurations![0] as Record<string, unknown>).label = null;
  (model.load_cases[1].analysis_state as Record<string, unknown>).unmodeled_future_key = { nested: [1, "two", null] };
  (model.materials![0] as Record<string, unknown>).unmodeled_material_key = "kept";
  return model;
}
const loadStateKeys = (model: PreviewModel) => text({ references: model.reference_configurations, laws: model.materials?.map(m => m.expansion_laws), states: model.load_cases.map(c => c.analysis_state) });

describe("0.4.0 documents in the browser persistence mirror", () => {
  it("mirrors native: 0.4.0 is current and retained as authored; 0.4.1 and 0.3.0 keep their prior refusals", () => {
    const status = evaluateModelDocumentLocal(model040());
    expect(status).toMatchObject({ status: "current", source_schema_version: "0.4.0", target_schema_version: LOAD_REFERENCE_MODEL_SCHEMA_VERSION, product_schema_migration_status: "current", persistence_state: "stored_document_current", applied_migration_ids: [] });
    expect(status.detail).toBe("Explicit model 0.4.0 is retained without migration or upgrade; load/reference-state records, pressure-profile inputs and solve eligibility are validated separately.");
    expect(modelDocumentVersionCheckStatus(model040())).toBe("current");
    expect(evaluateModelDocumentLocal({ ...model040(), schema_version: "0.4.1" }).status).toBe("newer_than_supported");
    expect(evaluateModelDocumentLocal({ ...model040(), schema_version: "0.3.0" }).status).toBe("newer_than_supported");
  });

  it("round-trips the model, the raw result and its AnalysisRun byte-exactly through create, save and open", async () => {
    const model = model040(), before = text(model);
    const created = await createLocalProject(model);
    expect(text(created.model)).toBe(before);
    expect(created.model_document_migration?.status).toBe("current");
    const raw = json("fixtures/product_preview/load_reference_source/mixed-sparse_interactive.raw.json") as MechanicsResult;
    const run = json("fixtures/results/load_reference_source_mixed_sparse.analysis_run.json") as AnalysisRunEnvelope;
    await saveLocalProject(model, [], null, null, raw, run);
    const opened = await openLocalProject(model.project.id);
    expect(text(opened!.model)).toBe(before);
    expect(loadStateKeys(opened!.model)).toBe(loadStateKeys(model));
    expect((opened!.model.reference_configurations![0] as Record<string, unknown>).label).toBeNull();
    expect(text(opened!.mechanics_result)).toBe(text(raw));
    expect(text(opened!.analysis_run)).toBe(text(run));
    expect(await canonicalSha256HexCheckedV1(opened!.mechanics_result)).toBe(run.analysis_run.hashes.find(h => h.payload_scope === "received_result")!.value);
    expect(text(model)).toBe(before);
  });
});

describe("0.4.0 documents through session state, manifests, IPC and operations", () => {
  it("the undo checkpoint clone keeps every key and explicit null", () => {
    const model = model040();
    expect(text(clonePreviewModel(model))).toBe(text(model));
  });

  it("the input manifest carries the whole model and its cases unprojected", async () => {
    const model = model040();
    const manifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "test:invented", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
    expect(text(manifest.manifest.model_basis.model_payload)).toBe(text(model));
    expect(text(manifest.manifest.load_basis.load_cases)).toBe(text(model.load_cases));
  });

  it("native IPC receives the exact model and the received result is preserved verbatim", async () => {
    const model = model040(), raw = json("fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json") as MechanicsResult;
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    let sent: unknown = null;
    invokeMock.mockImplementation(async (_command: string, args: { model: unknown }) => { sent = args.model; return structuredClone(raw); });
    const source = await runPreviewMechanics(model, "sparse_interactive");
    expect(text(sent)).toBe(text(model));
    expect(text(source)).toBe(text(raw));
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(true);
    expect(checkedJsonText(retainedNativeMechanicsInvocation(source, model)!.request.model)).toBe(checkedJsonText(model));
  });

  it("an unrelated wasm operation returns the model with every load/reference-state record unchanged", async () => {
    const model = model040();
    const intent = {
      audit_boundary: { direct_model_mutation_allowed: false, mutates_accepted_model_state: false, mutation_route: "structured_operations_only", requires_user_acceptance: true },
      author_type: "user",
      change: { after: "invented_updated_status", before: "invented_test_status", change_id: "change:t1-round-trip", change_kind: "update_load", dimension: "dimensionless", field_label: "status", field_path: "status", source_note: "invented_t1_round_trip", unit: "none" },
      operation_id: "op:t1-round-trip", operation_kind: "modify", operation_status: "proposed",
      professional_boundary: { human_review_required: true, software_makes_approval_claim: false, software_makes_authentication_claim: false, software_makes_certification_claim: false, software_makes_compliance_claim: false, software_makes_sealing_claim: false },
      rationale: "invented T1 round-trip check",
      target: { object_type: "Load", ref: "case:cold" },
      validation: { application_status: "not_applied", constraint_validation: "not_run", diff_preview_status: "not_generated", schema_validation: "not_run", unit_validation: "not_run" },
    } as unknown as EditorOperationIntent;
    const outcome = await applyModelOperation(model, intent, null);
    expect(outcome.validation.application_status).toBe("applied_to_session_model");
    const applied = outcome.applied_model!;
    expect(applied.load_cases[0].status).toBe("invented_updated_status");
    // The Rust applier returns serde_json maps with sorted keys (as for every
    // model); content, absent keys and explicit nulls are unchanged.
    const canonical = async (m: PreviewModel) => canonicalSha256HexCheckedV1({ references: m.reference_configurations, laws: m.materials?.map(x => x.expansion_laws ?? "absent"), states: m.load_cases.map(c => c.analysis_state ?? "absent") });
    expect(await canonical(applied)).toBe(await canonical(model));
    expect(applied.reference_configurations).toEqual(model.reference_configurations);
    expect(applied.load_cases.map(c => c.analysis_state)).toEqual(model.load_cases.map(c => c.analysis_state));
    expect(Object.hasOwn(applied.reference_configurations![0], "label")).toBe(true);
    expect(applied.schema_version).toBe("0.4.0");
    expect((applied.materials![0] as Record<string, unknown>).unmodeled_material_key).toBe("kept");
  });
});

describe("saved load/reference-state results keep the raw hash and the resolved evidence", () => {
  it.each([
    ["fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json", false],
    ["fixtures/product_preview/load_reference_source/eigen_motion-dense_scrutiny.raw.json", true],
  ] as const)("%s", async (path, joined) => {
    const raw = json(path) as MechanicsResult, model = json(path.replace(/-(sparse_interactive|dense_scrutiny)\.raw\.json$/, ".request.json")).model as PreviewModel;
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: raw.model_ref }, solver_basis: { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "test:invented" } } };
    const record = await buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model));
    expect(record.analysis_run.hashes.find(h => h.payload_scope === "received_result")!.value).toBe(await canonicalSha256HexCheckedV1(raw));
    if (joined) expect(text(record.analysis_run.contract_evidence)).toBe(text(raw.contract_evidence));
    else expect(Object.hasOwn(record.analysis_run, "contract_evidence")).toBe(false);
  });
});
