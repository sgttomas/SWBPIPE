import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../types";
import {
  buildBlankLocalModelDocument,
  createLocalProject,
  evaluateModelDocumentLocal,
  modelDocumentVersionCheckDiagnostic,
  modelDocumentVersionCheckStatus,
  openLocalProject,
  saveLocalProject,
  SUPPORTED_MODEL_SCHEMA_VERSION
} from "./projectService";

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

  it("refuses newer-than-supported and invalid versions without coercion", () => {
    const newer = evaluateModelDocumentLocal(sampleModel("0.3.0"));
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
    expect(modelDocumentVersionCheckStatus(sampleModel("0.3.0"))).toBe("newer_than_supported");
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

    const saved = await saveLocalProject(sampleModel(SUPPORTED_MODEL_SCHEMA_VERSION));
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

  it("refuses to persist documents with refused schema versions", async () => {
    await expect(createLocalProject(sampleModel("0.9.0"))).rejects.toThrow(/newer_than_supported/);
    await expect(saveLocalProject(sampleModel("not-semver"))).rejects.toThrow(/unsupported_schema/);
  });
});
