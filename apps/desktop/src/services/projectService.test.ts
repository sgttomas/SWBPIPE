import { afterEach, describe, expect, it, vi } from "vitest";
import type { LocalProjectEnvelope, PreviewModel } from "../types";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import {
  buildBlankLocalModelDocument,
  createLocalProject,
  findLocalProject,
  evaluateModelDocumentLocal,
  MODEL_SCHEMA_VERSION,
  modelDocumentVersionCheckDiagnostic,
  modelDocumentVersionCheckStatus,
  listLocalProjects,
  openLocalProject,
  saveLocalProject
} from "./projectService";

/** The revision a Replace of this stored project confirms (as the prompt would). */
async function shownRevision(projectId: string): Promise<string> {
  return (await findLocalProject(projectId))!.revision;
}

function sampleModel(schemaVersion: string): PreviewModel {
  return {
    schema_version: schemaVersion,
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {},
    project: { id: "project:doc-migration-test", name: "Doc Migration Test", description: "test", units: { length: "m" } },
    analysis_status: { mechanics: "ready", rule_check: "not_performed", professional_acceptance: "not_provided" },
    nodes: [],
    pipe_segments: [],
    supports: [],
    components: [],
    load_cases: [],
    diagnostics: []
  };
}

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("projectService model-document version evidence (DEL-02-05 CLM-003, browser preview)", () => {
  it("evaluates the current 0.7.0 document as current with the in-document version authority", () => {
    const status = evaluateModelDocumentLocal(sampleModel(MODEL_SCHEMA_VERSION));
    expect(status).toEqual({
      status: "current",
      source_schema_version: "0.7.0",
      target_schema_version: "0.7.0",
      detail: "Model document 0.7.0 is the current version; a pressure contract, when present, and solve eligibility are validated separately."
    });
  });

  // Each version the native evaluation handles (model_document_version.rs
  // `evaluate_model_document`), with its native outcome. 0.1.0 to 0.6.0 are
  // retired (B7b: 0.3.0 and 0.4.0 with the v2 exact contract; 0.5.0 since 0.6.0; 0.6.0 since 0.7.0): refused by name with
  // the re-author diagnostic, never migrated.
  it.each([
    ["0.1.0", "retired_schema", "0.7.0"],
    ["0.2.0", "retired_schema", "0.7.0"],
    ["0.0.9", "retired_schema", "0.7.0"],
    ["0.3.1", "retired_schema", "0.7.0"],
    ["0.3.0", "retired_schema", "0.7.0"],
    ["0.4.0", "retired_schema", "0.7.0"],
    ["0.5.0", "retired_schema", "0.7.0"],
    ["0.5.1", "retired_schema", "0.7.0"],
    ["0.6.0", "retired_schema", "0.7.0"],
    ["0.6.1", "retired_schema", "0.7.0"],
    ["0.7.0", "current", "0.7.0"],
    ["0.7.1", "newer_than_supported", "0.7.0"],
    ["1.0.0", "newer_than_supported", "0.7.0"],
    ["not-semver", "unsupported_schema", "0.7.0"]
  ])("mirrors native for %s: %s, target %s", (version, status, target) => {
    const model = sampleModel(version);
    const before = JSON.stringify(model);
    const evaluated = evaluateModelDocumentLocal(model);
    expect(evaluated).toMatchObject({ status, source_schema_version: version, target_schema_version: target });
    expect(modelDocumentVersionCheckStatus(model)).toBe(status);
    expect(JSON.stringify(model)).toBe(before);
  });

  it.each(["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0"])("refuses the retired %s by name with a re-author diagnostic and never migrates it", async (version) => {
    const evaluated = evaluateModelDocumentLocal(sampleModel(version));
    expect(evaluated.detail).toBe(`MODEL_SCHEMA_VERSION_RETIRED: model document ${version} is retired; re-author it as model document 0.7.0 (a pressure-free model omits pressure_contract). Nothing is migrated.`);
    await expect(createLocalProject(sampleModel(version))).rejects.toThrow(/status=retired_schema; source_schema_version=/);
    await expect(saveLocalProject(sampleModel(version))).rejects.toThrow(/MODEL_SCHEMA_VERSION_RETIRED/);
    expect(await findLocalProject("project:doc-migration-test")).toBeNull();
  });

  it("mirrors native: the explicit exact profile (0.7.0 with 3.0.0/exact_pressure_v3) opens and saves in the browser as authored", async () => {
    const model: PreviewModel = { ...sampleModel("0.7.0"), pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" } } as PreviewModel;
    const before = JSON.stringify(model);
    const status = evaluateModelDocumentLocal(model);
    expect(status.detail).toBe("Model document 0.7.0 is the current version; a pressure contract, when present, and solve eligibility are validated separately.");
    const created = await createLocalProject(model);
    expect(created.model_document_version?.status).toBe("current");
    expect(JSON.stringify(created.model)).toBe(before);
    // Saves over its own stored snapshot (no envelope hash here), confirmed; the
    // replacement guard is covered below (HELP_HUMAN ruling 2026-10-10, #1252).
    await saveLocalProject(model, [], null, null, null, null, null, null, await shownRevision(model.project.id));
    expect(JSON.stringify((await openLocalProject(model.project.id))?.model)).toBe(before);
  });

  it("refuses newer-than-supported and invalid versions without coercion", () => {
    const newer = evaluateModelDocumentLocal(sampleModel("0.7.1"));
    expect(newer.status).toBe("newer_than_supported");
    expect(newer.detail).toContain("no down-migration");

    const invalid = evaluateModelDocumentLocal(sampleModel(" 0.7.0"));
    expect(invalid.status).toBe("unsupported_schema");
  });

  it("classifies version checks and their diagnostics with the governed statuses", () => {
    expect(modelDocumentVersionCheckStatus(sampleModel(MODEL_SCHEMA_VERSION))).toBe("current");
    expect(modelDocumentVersionCheckStatus(sampleModel(MODEL_SCHEMA_VERSION), "retired_schema")).toBe("retired_schema");
    expect(modelDocumentVersionCheckDiagnostic("current")).toBeNull();
    expect(modelDocumentVersionCheckDiagnostic("retired_schema")).toMatchObject({
      code: "PROJECT-VALIDATION-RETIRED-SCHEMA",
      severity: "blocking"
    });
    expect(modelDocumentVersionCheckDiagnostic("newer_than_supported")).toMatchObject({
      code: "PROJECT-VALIDATION-NEWER-THAN-SUPPORTED-SCHEMA",
      severity: "blocking"
    });
    expect(modelDocumentVersionCheckDiagnostic("unsupported_schema")).toMatchObject({
      code: "PROJECT-VALIDATION-UNSUPPORTED-SCHEMA",
      severity: "blocking"
    });
  });

  it("round-trips create, save, and open with current version evidence", async () => {
    const created = await createLocalProject(sampleModel(MODEL_SCHEMA_VERSION));
    expect(created.model_document_version?.status).toBe("current");
    expect(created).not.toHaveProperty("model_migration_ledger");
    expect(created.summary.unit_round_trip_status).toBe("unit_metadata_preserved_in_local_project_envelope");
    expect(created.summary.unit_round_trip_checked_ref_count).toBe(1);
    expect(created.summary.unit_round_trip_signature).toBe("project.units.length=m");

    // Saves over its own stored snapshot (no envelope hash here); the guard is covered below.
    const saved = await saveLocalProject(sampleModel(MODEL_SCHEMA_VERSION), [], null, null, null, null, null, null, await shownRevision("project:doc-migration-test"));
    expect(saved.model_document_version?.status).toBe("current");
    expect(saved.summary.unit_round_trip_status).toBe(created.summary.unit_round_trip_status);
    expect(saved.summary.unit_round_trip_signature).toBe(created.summary.unit_round_trip_signature);

    const opened = await openLocalProject();
    expect(opened?.model_document_version?.status).toBe("current");
    expect(opened?.summary.unit_round_trip_status).toBe(created.summary.unit_round_trip_status);
    expect(opened?.summary.unit_round_trip_signature).toBe(created.summary.unit_round_trip_signature);
  });

  it("builds and persists an explicit blank local authoring document", async () => {
    const blank = buildBlankLocalModelDocument(new Date("2026-06-12T00:00:00Z"));
    expect(blank.schema_version).toBe(MODEL_SCHEMA_VERSION);
    expect(blank.project.id).toBe("project:blank-local-20260612t000000z");
    expect(blank.project.name).toBe("Blank Local Model");
    expect(blank.project.units.temperature).toBe("degC");
    expect(blank.analysis_status.mechanics).toBe("MODEL_INCOMPLETE");
    expect(blank.analysis_status.rule_check).toBe("RULE_INPUTS_INCOMPLETE");
    expect(blank.nodes).toEqual([]);
    expect(blank.pipe_segments).toEqual([]);
    expect(blank.supports).toEqual([]);
    expect(blank.materials).toEqual([]);
    expect(blank.sections).toEqual([]);
    expect(blank.load_cases).toEqual([]);
    expect(blank.combinations).toEqual([]);
    expect(blank.data_boundary.private_data_policy).toContain("not_committed_to_repository");
    expect(blank.diagnostics[0].code).toBe("BLANK_PROJECT_AUTHORING_TARGET");
    expect(blank.diagnostics[0].severity).toBe("blocking");

    const created = await createLocalProject(blank);
    expect(created.summary.project_id).toBe("project:blank-local-20260612t000000z");
    expect(created.summary.project_name).toBe("Blank Local Model");
    expect(created.summary.persisted_mechanics_result_count).toBe(0);
    expect(created.summary.persisted_model_hash_count).toBe(0);
    expect(created.summary.unit_round_trip_status).toBe("unit_metadata_preserved_in_local_project_envelope");
    // W-17 slice (a): seven keys, the SI preset (today's six plus angle).
    expect(blank.project.units).toEqual({ length: "m", force: "N", moment: "N*m", pressure: "Pa", stress: "Pa", temperature: "degC", angle: "rad" });
    expect(created.summary.unit_round_trip_checked_ref_count).toBe(7);
    expect(created.summary.unit_round_trip_signature).toContain("project.units.length=m");
    expect(created.summary.unit_round_trip_signature).toContain("project.units.force=N");
    expect(created.summary.unit_round_trip_signature).toContain("project.units.angle=rad");
    expect(created.model_document_version?.status).toBe("current");

    const opened = await openLocalProject("project:blank-local-20260612t000000z");
    expect(opened?.model.project.id).toBe("project:blank-local-20260612t000000z");
    expect(opened?.model.nodes).toEqual([]);
    expect(opened?.summary.storage_mode).toBe("browser_memory_preview");
    expect(opened?.summary.unit_round_trip_signature).toBe(created.summary.unit_round_trip_signature);
  });

  it("preserves legacy C metadata and thermal quantities through save and reopen", async () => {
    const model = sampleModel(MODEL_SCHEMA_VERSION);
    model.project.units.temperature = "C";
    model.load_cases = [{ id: "load:legacy", label: "Legacy thermal", kind: "thermal", status: "draft", provenance: "legacy test", primitive_loads: [{
      id: "load:legacy-thermal", category: "thermal", target: { type: "element", pipe: "pipe:legacy" },
      direction: "global_x", magnitude: { value: 10, unit: "C" }, dimension: "temperature_interval",
      provenance: "legacy compatibility test"
    }] }];
    const before = JSON.stringify(model);
    // The round trip saves over its own stored snapshot (no envelope hash here);
    // the replacement guard itself is covered below.
    await createLocalProject(model);
    const saved = await saveLocalProject(model, [], null, null, null, null, null, null, await shownRevision(model.project.id));
    const opened = await openLocalProject(model.project.id);
    expect(JSON.stringify(saved.model)).toBe(before);
    expect(JSON.stringify(opened?.model)).toBe(before);
    expect(opened?.summary.unit_round_trip_signature).toContain("project.units.temperature=C");
    expect(opened?.summary.unit_round_trip_signature).toBe(saved.summary.unit_round_trip_signature);
  });

  it("refuses to persist documents with refused schema versions", async () => {
    await expect(createLocalProject(sampleModel("0.9.0"))).rejects.toThrow(/newer_than_supported/);
    await expect(saveLocalProject(sampleModel("not-semver"))).rejects.toThrow(/unsupported_schema/);
  });

  it("keeps every project it is given: saving one id never discards another (HELP_HUMAN 2026-10-10)", async () => {
    const first = sampleModel(MODEL_SCHEMA_VERSION);
    const second = { ...sampleModel(MODEL_SCHEMA_VERSION), project: { ...first.project, id: "project:invented-second", name: "Invented second" } };
    await createLocalProject(first);
    await createLocalProject(second);
    expect((await listLocalProjects()).map((entry) => entry.project_id)).toEqual(["project:invented-second", first.project.id]);
    expect((await openLocalProject(first.project.id))?.model.project.id).toBe(first.project.id);
    expect((await openLocalProject())?.model.project.id).toBe("project:invented-second");
    // A later write to the first makes it the most recent; the second is still stored.
    await saveLocalProject(first, [], null, null, null, null, null, null, await shownRevision(first.project.id));
    expect((await openLocalProject())?.model.project.id).toBe(first.project.id);
    expect(await findLocalProject("project:invented-second")).toMatchObject({ project_name: "Invented second" });
  });

  it("never replaces the stored project without confirmation (HELP_HUMAN ruling 2026-10-10)", async () => {
    const model = sampleModel(MODEL_SCHEMA_VERSION);
    await createLocalProject(model);
    const stored = await findLocalProject(model.project.id);
    expect(stored).toMatchObject({ project_id: model.project.id, project_name: model.project.name });
    await expect(createLocalProject(model)).rejects.toThrow(/^PROJECT-ID-EXISTS: project `/);
    await expect(saveLocalProject(model)).rejects.toThrow(/^PROJECT-ID-EXISTS: project `/);
    await expect(saveLocalProject(model, [], null, null, null, null, null, null, null, "sha256:not-the-stored-base"))
      .rejects.toThrow(/^PROJECT-BASE-MOVED: project `/);
    // The stored snapshot records no envelope hash: no base can match it.
    await expect(saveLocalProject(model, [], null, null, null, null, null, null, null, ""))
      .rejects.toThrow(/^PROJECT-BASE-MOVED: project `/);
    expect(await findLocalProject("project:not-stored")).toBeNull();
    // A Replace confirms the revision the prompt showed; one saved since is refused.
    const shown = await shownRevision(model.project.id);
    await saveLocalProject(model, [], null, null, null, null, null, null, shown);
    await expect(saveLocalProject(model, [], null, null, null, null, null, null, shown))
      .rejects.toThrow(/^PROJECT-BASE-MOVED: .* changed after the replace prompt showed it/);
  });
});

// W-17: model document 0.7.0's schema leaves `project.units` optional, but the
// desktop's open check requires the map (`MODEL-FILE-SHAPE` at `project.units`).
// A stored project without it is refused by name at open, in the browser
// preview and through the native store; a missing key alone still opens.
describe("openLocalProject refuses a document without project.units", () => {
  const REFUSAL = "Model document refused: MODEL-FILE-SHAPE at project.units: project.units is missing (object required).";
  function withoutUnits(id: string): PreviewModel {
    const model = { ...sampleModel(MODEL_SCHEMA_VERSION), project: { ...sampleModel(MODEL_SCHEMA_VERSION).project, id } };
    delete (model.project as { units?: unknown }).units;
    return model;
  }

  it("browser preview: a stored snapshot without the map is refused at open; one missing keys opens", async () => {
    const model = withoutUnits("project:invented-no-units");
    await createLocalProject(model);
    await expect(openLocalProject(model.project.id)).rejects.toThrow(REFUSAL);
    const partial = { ...sampleModel(MODEL_SCHEMA_VERSION), project: { ...sampleModel(MODEL_SCHEMA_VERSION).project, id: "project:invented-length-only" } };
    await createLocalProject(partial);
    expect((await openLocalProject(partial.project.id))?.model.project.units).toEqual({ length: "m" });
  });

  it("native store: an envelope whose model lacks the map is refused at open", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const model = withoutUnits("project:invented-native-no-units");
    invokeMock.mockImplementation(async (command: string) => {
      if (command !== "open_local_project") throw new Error(`unexpected ${command}`);
      return { summary: { project_id: model.project.id }, model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: null, project_envelope_hash: null };
    });
    await expect(openLocalProject(model.project.id)).rejects.toThrow(REFUSAL);
    invokeMock.mockImplementation(async () => ({ summary: { project_id: "project:doc-migration-test" }, model: sampleModel(MODEL_SCHEMA_VERSION), editor_intents: [] }));
    expect((await openLocalProject("project:doc-migration-test"))?.model.project.units).toEqual({ length: "m" });
  });
});
