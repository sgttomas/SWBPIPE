import { afterEach, describe, expect, it, vi } from "vitest";
import type { LocalProjectEnvelope, PreviewModel } from "../types";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import {
  buildBlankLocalModelDocument,
  createLocalProject,
  findLocalProject,
  evaluateModelDocumentLocal,
  EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION,
  LOAD_REFERENCE_MODEL_SCHEMA_VERSION,
  modelDocumentVersionCheckDiagnostic,
  modelDocumentVersionCheckStatus,
  listLocalProjects,
  openLocalProject,
  saveLocalProject,
  SUPPORTED_MODEL_SCHEMA_VERSION
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

describe("projectService model-document migration evidence (DEC-019, browser preview)", () => {
  it("evaluates current documents as current with the in-document version authority", () => {
    const status = evaluateModelDocumentLocal(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION));
    expect(status.status).toBe("current");
    expect(status.source_schema_version).toBe(SUPPORTED_MODEL_SCHEMA_VERSION);
    expect(status.target_schema_version).toBe(SUPPORTED_MODEL_SCHEMA_VERSION);
    expect(status.migration_framework).toBe("application_service_separate_db_and_product_schema");
    expect(status.applied_migration_ids).toEqual([]);
  });

  it("migrates 0.1.0 documents in memory through the mirrored DEC-033 no-op chain entry", () => {
    const status = evaluateModelDocumentLocal(sampleModel("0.1.0"));
    expect(status.status).toBe("migrated");
    expect(status.source_schema_version).toBe("0.1.0");
    expect(status.target_schema_version).toBe(SUPPORTED_MODEL_SCHEMA_VERSION);
    expect(status.applied_migration_ids).toEqual([
      "model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop"
    ]);
  });

  it("accepts 0.1.0 documents with migrated evidence while keeping stored snapshot bytes unchanged", async () => {
    const created = await createLocalProject(sampleModel("0.1.0"));
    expect(created.model_document_migration?.status).toBe("migrated");
    expect(created.model_document_migration?.applied_migration_ids).toEqual([
      "model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop"
    ]);
    expect(created.model_document_migration?.target_schema_version).toBe(SUPPORTED_MODEL_SCHEMA_VERSION);
    // No migration ledger exists in the browser preview, so stored bytes are
    // never rewritten; the migration remains in-memory evidence.
    expect(created.model.schema_version).toBe("0.1.0");
    expect(created.model_document_migration?.detail).toContain("stored bytes unchanged");

    const opened = await openLocalProject("project:doc-migration-test");
    expect(opened?.model.schema_version).toBe("0.1.0");
    expect(opened?.model_document_migration?.status).toBe("migrated");
  });

  // Each version the native evaluation handles (model_document_migration.rs
  // `evaluate_model_document` with the published chain), with its native outcome.
  it.each([
    ["0.1.0", "migrated", SUPPORTED_MODEL_SCHEMA_VERSION, "in_memory_only_not_yet_saved", ["model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop"]],
    ["0.2.0", "current", SUPPORTED_MODEL_SCHEMA_VERSION, "stored_document_current", []],
    ["0.3.0", "current", EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION, "stored_document_current", []],
    ["0.4.0", "current", LOAD_REFERENCE_MODEL_SCHEMA_VERSION, "stored_document_current", []]
  ])("mirrors native for %s: %s, target %s", (version, status, target, persistence, applied) => {
    const model = sampleModel(version);
    const before = JSON.stringify(model);
    const evaluated = evaluateModelDocumentLocal(model);
    expect(evaluated).toMatchObject({
      status,
      product_schema_migration_status: status,
      source_schema_version: version,
      target_schema_version: target,
      persistence_state: persistence,
      applied_migration_ids: applied
    });
    expect(modelDocumentVersionCheckStatus(model)).toBe(status === "current" ? "current" : "stale");
    expect(JSON.stringify(model)).toBe(before);
  });

  it("mirrors native: the explicit 0.3.0 profile opens and saves in the browser as authored", async () => {
    const model = sampleModel(EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION);
    const status = evaluateModelDocumentLocal(model);
    expect(status.detail).toBe("Explicit model 0.3.0 is retained without migration; pressure-profile inputs and solve eligibility are validated separately.");
    const created = await createLocalProject(model);
    expect(created.model_document_migration?.status).toBe("current");
    expect(created.model.schema_version).toBe("0.3.0");
    // Saves over its own stored snapshot (no envelope hash here), confirmed; the
    // replacement guard is covered below (HELP_HUMAN ruling 2026-10-10, #1252).
    await saveLocalProject(model, [], null, null, null, null, null, null, null, await shownRevision(model.project.id));
    expect((await openLocalProject(model.project.id))?.model.schema_version).toBe("0.3.0");
  });

  it.each(["0.2.1", "0.3.1", "0.4.1", "0.5.0", "1.0.0"])("mirrors native: %s is newer than supported", (version) => {
    const evaluated = evaluateModelDocumentLocal(sampleModel(version));
    expect(evaluated.status).toBe("newer_than_supported");
    expect(evaluated.detail).toBe(`Model document schema_version ${version} is newer than the legacy migration target 0.2.0 and does not match the separately supported explicit 0.3.0 or 0.4.0; refusing to open for editing (no down-migration).`);
  });

  it("refuses newer-than-supported and invalid versions without coercion", () => {
    const newer = evaluateModelDocumentLocal(sampleModel("0.3.1"));
    expect(newer.status).toBe("newer_than_supported");
    expect(newer.detail).toContain("no down-migration");

    const invalid = evaluateModelDocumentLocal(sampleModel("not-semver"));
    expect(invalid.status).toBe("unsupported_schema");

    const older = evaluateModelDocumentLocal(sampleModel("0.0.1"));
    expect(older.status).toBe("unsupported_schema");
    expect(older.detail).toContain("No migration path");
  });

  it("classifies current, stale, unsupported, newer, and failed version checks with governed statuses", () => {
    expect(modelDocumentVersionCheckStatus(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION))).toBe("current");
    expect(modelDocumentVersionCheckStatus(sampleModel("0.1.0"))).toBe("stale");
    expect(modelDocumentVersionCheckStatus(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION), "stale")).toBe("stale");
    expect(modelDocumentVersionCheckStatus(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION), "migration_needed")).toBe("stale");
    expect(modelDocumentVersionCheckStatus(sampleModel("0.0.1"))).toBe("unsupported_schema");
    expect(modelDocumentVersionCheckStatus(sampleModel("0.3.1"))).toBe("newer_than_supported");
    expect(modelDocumentVersionCheckStatus(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION), "failed")).toBe("failed");

    expect(modelDocumentVersionCheckDiagnostic("current")).toBeNull();
    expect(modelDocumentVersionCheckDiagnostic("stale")).toMatchObject({
      code: "PROJECT-VALIDATION-STALE-SCHEMA",
      severity: "warning"
    });
    expect(modelDocumentVersionCheckDiagnostic("failed")).toMatchObject({
      code: "PROJECT-VALIDATION-SCHEMA-MIGRATION-FAILED",
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

  it("round-trips create, save, and open with current migration evidence and an empty ledger", async () => {
    const created = await createLocalProject(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION));
    expect(created.model_document_migration?.status).toBe("current");
    expect(created.model_migration_ledger).toEqual([]);
    expect(created.summary.unit_round_trip_status).toBe("unit_metadata_preserved_in_local_project_envelope");
    expect(created.summary.unit_round_trip_checked_ref_count).toBe(1);
    expect(created.summary.unit_round_trip_signature).toBe("project.units.length=m");

    // Saves over its own stored snapshot (no envelope hash here); the guard is covered below.
    const saved = await saveLocalProject(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION), [], null, null, null, null, null, null, null, await shownRevision("project:doc-migration-test"));
    expect(saved.model_document_migration?.status).toBe("current");
    expect(saved.summary.unit_round_trip_status).toBe(created.summary.unit_round_trip_status);
    expect(saved.summary.unit_round_trip_signature).toBe(created.summary.unit_round_trip_signature);

    const opened = await openLocalProject();
    expect(opened?.model_document_migration?.status).toBe("current");
    expect(opened?.model_migration_ledger).toEqual([]);
    expect(opened?.summary.unit_round_trip_status).toBe(created.summary.unit_round_trip_status);
    expect(opened?.summary.unit_round_trip_signature).toBe(created.summary.unit_round_trip_signature);
  });

  it("builds and persists an explicit blank local authoring document", async () => {
    const blank = buildBlankLocalModelDocument(new Date("2026-06-12T00:00:00Z"));
    expect(blank.schema_version).toBe(SUPPORTED_MODEL_SCHEMA_VERSION);
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
    expect(created.summary.unit_round_trip_checked_ref_count).toBe(6);
    expect(created.summary.unit_round_trip_signature).toContain("project.units.length=m");
    expect(created.summary.unit_round_trip_signature).toContain("project.units.force=N");
    expect(created.model_document_migration?.status).toBe("current");

    const opened = await openLocalProject("project:blank-local-20260612t000000z");
    expect(opened?.model.project.id).toBe("project:blank-local-20260612t000000z");
    expect(opened?.model.nodes).toEqual([]);
    expect(opened?.summary.storage_mode).toBe("browser_memory_preview");
    expect(opened?.summary.unit_round_trip_signature).toBe(created.summary.unit_round_trip_signature);
  });

  it("preserves legacy C metadata and thermal quantities through save and reopen", async () => {
    const model = sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION);
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
    const saved = await saveLocalProject(model, [], null, null, null, null, null, null, null, await shownRevision(model.project.id));
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
    const first = sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION);
    const second = { ...sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION), project: { ...first.project, id: "project:invented-second", name: "Invented second" } };
    await createLocalProject(first);
    await createLocalProject(second);
    expect((await listLocalProjects()).map((entry) => entry.project_id)).toEqual(["project:invented-second", first.project.id]);
    expect((await openLocalProject(first.project.id))?.model.project.id).toBe(first.project.id);
    expect((await openLocalProject())?.model.project.id).toBe("project:invented-second");
    // A later write to the first makes it the most recent; the second is still stored.
    await saveLocalProject(first, [], null, null, null, null, null, null, null, await shownRevision(first.project.id));
    expect((await openLocalProject())?.model.project.id).toBe(first.project.id);
    expect(await findLocalProject("project:invented-second")).toMatchObject({ project_name: "Invented second" });
  });

  it("never replaces the stored project without confirmation (HELP_HUMAN ruling 2026-10-10)", async () => {
    const model = sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION);
    await createLocalProject(model);
    const stored = await findLocalProject(model.project.id);
    expect(stored).toMatchObject({ project_id: model.project.id, project_name: model.project.name });
    await expect(createLocalProject(model)).rejects.toThrow(/^PROJECT-ID-EXISTS: project `/);
    await expect(saveLocalProject(model)).rejects.toThrow(/^PROJECT-ID-EXISTS: project `/);
    await expect(saveLocalProject(model, [], null, null, null, null, null, null, null, null, "sha256:not-the-stored-base"))
      .rejects.toThrow(/^PROJECT-BASE-MOVED: project `/);
    // The stored snapshot records no envelope hash: no base can match it.
    await expect(saveLocalProject(model, [], null, null, null, null, null, null, null, null, ""))
      .rejects.toThrow(/^PROJECT-BASE-MOVED: project `/);
    expect(await findLocalProject("project:not-stored")).toBeNull();
    // A Replace confirms the revision the prompt showed; one saved since is refused.
    const shown = await shownRevision(model.project.id);
    await saveLocalProject(model, [], null, null, null, null, null, null, null, shown);
    await expect(saveLocalProject(model, [], null, null, null, null, null, null, null, shown))
      .rejects.toThrow(/^PROJECT-BASE-MOVED: .* changed after the replace prompt showed it/);
  });

  it("round-trips old ledger bytes and optional transition hash evidence without projecting raw claims", async () => {
    const model = sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION);
    const oldRecord = {
      record_kind: "model_document_migration_ledger_record",
      recorded_at_unix: 10,
      source_schema_version: "0.1.0",
      target_schema_version: "0.2.0",
      applied_migration_ids: ["legacy-entry"],
      migration_framework: "application_service_separate_db_and_product_schema",
      pre_migration_model_hash: "received-old-pre",
      post_migration_model_hash: "received-old-post",
      trigger: "migrate_in_memory_on_open_persisted_on_save",
      destructive_rewrite: false,
      professional_boundary: { human_review_required: true },
    };
    const newRecord = {
      ...oldRecord,
      recorded_at_unix: 11,
      hash_evidence: {
        schema: "model_migration_hash_evidence_v1" as const,
        source_payload_basis: "stored_pre_open_migration_model" as const,
        received: { model_hash: "malformed raw claim", project_envelope_hash: ["raw", "claim"] },
        prior_stored: { model_hash: { algorithm: "sha512", extra: true }, project_envelope_hash: null },
        computed: {
          pre_migration_model_hash: `sha256:${"1".repeat(64)}`,
          post_migration_model_hash: `sha256:${"2".repeat(64)}`,
          post_migration_project_envelope_hash: `sha256:${"3".repeat(64)}`,
        },
        received_claim_verification: "not_asserted" as const,
      },
    };
    const envelope = {
      model,
      model_migration_ledger: [oldRecord, newRecord],
    } as unknown as LocalProjectEnvelope;
    invokeMock.mockResolvedValue(envelope);
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const opened = await openLocalProject(model.project.id);
    expect(invokeMock).toHaveBeenCalledWith("open_local_project", { projectId: model.project.id });
    expect(opened?.model_migration_ledger).toEqual([oldRecord, newRecord]);
    expect(JSON.stringify(opened?.model_migration_ledger[0])).toBe(JSON.stringify(oldRecord));
    expect(opened?.model_migration_ledger[1].hash_evidence?.received.model_hash).toBe("malformed raw claim");
  });
});
