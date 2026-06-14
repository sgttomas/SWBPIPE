import { afterEach, describe, expect, it, vi } from "vitest";
import type { LibraryImportValidation, LocalLibrarySaveResult } from "./libraryImportService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import {
  LIBRARY_IMPORT_BACKEND_DIAGNOSTIC,
  classifyLibraryReference,
  deleteLocalLibrary,
  indexLibraryRecordsSlots,
  listLocalLibraries,
  openLocalLibrary,
  partitionLibraryImportFindings,
  saveLocalLibrary,
  validateLibraryImport
} from "./libraryImportService";

const acceptedFixture: LibraryImportValidation = {
  document_kind: "openpipestress.library_import.validation",
  outcome: "PRIVATE_LOCAL_ONLY",
  library_kind: "material",
  intended_visibility: "private",
  accepted: true,
  has_blocking_findings: false,
  findings: [],
  diagnostics: [],
  professional_boundary_notice:
    "Library-import validation reports software findings only over an already-parsed import payload."
};

const blockedFixture: LibraryImportValidation = {
  document_kind: "openpipestress.library_import.validation",
  outcome: "QUARANTINE",
  library_kind: "component",
  intended_visibility: "public",
  accepted: false,
  has_blocking_findings: true,
  findings: [
    {
      code: "IMPORT_PROTECTED_CONTENT_SUSPECTED",
      severity: "quarantine",
      path: "component_records[0]",
      message: "Import metadata indicates suspected protected content.",
      remediation: "Quarantine metadata and request human/legal review; do not publish values."
    },
    {
      code: "IMPORT_LIBRARY_METADATA_MISSING",
      severity: "blocking",
      path: "component_library",
      message: "Library metadata object is missing.",
      remediation: "Provide library metadata with provenance before import."
    },
    {
      code: "IMPORT_REVIEW_REQUIRED",
      severity: "review_required",
      path: "component_records[0]",
      message: "Public import requires an accepted review disposition.",
      remediation: "Record maintainer review before accepting public data."
    }
  ],
  diagnostics: [
    {
      code: "IMPORT_PROTECTED_CONTENT_SUSPECTED",
      severity: "quarantine",
      class: "import_boundary",
      source: "core.library_import.library_import_document"
    }
  ],
  professional_boundary_notice:
    "Library-import validation reports software findings only over an already-parsed import payload."
};

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("libraryImportService", () => {
  it("reports an explicit browser-preview unavailable route instead of synthesizing a validator", async () => {
    const route = await validateLibraryImport({}, "material", "private");

    expect(route.route).toBe("unavailable_browser_preview");
    if (route.route !== "unavailable_browser_preview") throw new Error("expected browser-unavailable route");
    expect(route.diagnostic).toBe(LIBRARY_IMPORT_BACKEND_DIAGNOSTIC);
    expect(route.diagnostic).toContain("LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("routes validation through the Tauri command with camelCase args in desktop mode", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(acceptedFixture);

    const payload = { material_library: {}, material_records: [] };
    const route = await validateLibraryImport(payload, "material", "private");

    expect(invokeMock).toHaveBeenCalledWith("validate_library_import", {
      payload,
      libraryKind: "material",
      intendedVisibility: "private"
    });
    expect(route.route).toBe("tauri_backend");
    if (route.route !== "tauri_backend") throw new Error("expected Tauri backend route");
    expect(route.validation.outcome).toBe("PRIVATE_LOCAL_ONLY");
    expect(route.validation.accepted).toBe(true);
    expect(route.validation.has_blocking_findings).toBe(false);
    expect(route.validation.professional_boundary_notice).toContain("software findings only");
  });

  it("propagates an unsupported-token rejection from the backend command", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockRejectedValue("unsupported library_kind: widget");

    await expect(
      validateLibraryImport({}, "widget" as unknown as "material", "private")
    ).rejects.toBe("unsupported library_kind: widget");
  });

  it("partitions findings along the PRD §13.5 blocking-vs-advisory axis", () => {
    const { blocking, advisory } = partitionLibraryImportFindings(blockedFixture);

    expect(blocking.map((finding) => finding.code)).toEqual([
      "IMPORT_PROTECTED_CONTENT_SUSPECTED",
      "IMPORT_LIBRARY_METADATA_MISSING"
    ]);
    expect(advisory.map((finding) => finding.code)).toEqual(["IMPORT_REVIEW_REQUIRED"]);
    expect(partitionLibraryImportFindings(acceptedFixture)).toEqual({ blocking: [], advisory: [] });
  });

  const storedSaveFixture: LocalLibrarySaveResult = {
    project_id: "project:test-local",
    library_kind: "material",
    library_id: "matlib.invented.alpha",
    stored: true,
    storage_mode: "local_sqlite",
    created_at_unix: 100,
    updated_at_unix: 100,
    document: { material_library: { library_id: "matlib.invented.alpha" }, material_records: [] },
    validation: acceptedFixture,
    message: "Stored library document to the local SQLite store only."
  };

  it("reports the unavailable route for persistence in browser preview without calling invoke", async () => {
    const route = await saveLocalLibrary("project:test-local", "material", {});

    expect(route.route).toBe("unavailable_browser_preview");
    if (route.route !== "unavailable_browser_preview") throw new Error("expected unavailable route");
    expect(route.diagnostic).toBe(LIBRARY_IMPORT_BACKEND_DIAGNOSTIC);
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("routes save through the Tauri command and surfaces the stored flag", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(storedSaveFixture);
    const document = { material_library: { library_id: "matlib.invented.alpha" }, material_records: [] };

    const route = await saveLocalLibrary("project:test-local", "material", document);

    expect(invokeMock).toHaveBeenCalledWith("save_local_library", {
      projectId: "project:test-local",
      libraryKind: "material",
      document
    });
    expect(route.route).toBe("tauri_backend");
    if (route.route !== "tauri_backend") throw new Error("expected Tauri backend route");
    expect(route.result.stored).toBe(true);
    expect(route.result.library_id).toBe("matlib.invented.alpha");
  });

  it("passes a refused (not-stored) save result through unchanged", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const refused: LocalLibrarySaveResult = {
      ...storedSaveFixture,
      stored: false,
      created_at_unix: null,
      updated_at_unix: null,
      validation: blockedFixture,
      message: "Import not stored: validation findings block this library."
    };
    invokeMock.mockResolvedValue(refused);

    const route = await saveLocalLibrary("project:test-local", "component", {});

    expect(route.route).toBe("tauri_backend");
    if (route.route !== "tauri_backend") throw new Error("expected Tauri backend route");
    expect(route.result.stored).toBe(false);
    expect(route.result.created_at_unix).toBeNull();
    expect(route.result.validation.has_blocking_findings).toBe(true);
  });

  it("routes open/list/delete through their Tauri commands with camelCase args", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};

    invokeMock.mockResolvedValueOnce(null);
    const openRoute = await openLocalLibrary("project:test-local", "material", "matlib.invented.alpha");
    expect(invokeMock).toHaveBeenLastCalledWith("open_local_library", {
      projectId: "project:test-local",
      libraryKind: "material",
      libraryId: "matlib.invented.alpha"
    });
    expect(openRoute.route === "tauri_backend" && openRoute.envelope).toBeNull();

    invokeMock.mockResolvedValueOnce([
      {
        project_id: "project:test-local",
        library_kind: "material",
        library_id: "matlib.invented.alpha",
        library_name: "Invented private material library",
        privacy_class: "private_user_data",
        storage_mode: "local_sqlite",
        created_at_unix: 100,
        updated_at_unix: 100
      }
    ]);
    const listRoute = await listLocalLibraries("project:test-local");
    expect(invokeMock).toHaveBeenLastCalledWith("list_local_libraries", {
      projectId: "project:test-local"
    });
    if (listRoute.route !== "tauri_backend") throw new Error("expected Tauri backend route");
    expect(listRoute.entries).toHaveLength(1);
    expect(listRoute.entries[0].library_id).toBe("matlib.invented.alpha");

    invokeMock.mockResolvedValueOnce({
      project_id: "project:test-local",
      library_kind: "material",
      library_id: "matlib.invented.alpha",
      deleted: true,
      message: "Deleted library document from the local SQLite store."
    });
    const deleteRoute = await deleteLocalLibrary("project:test-local", "material", "matlib.invented.alpha");
    expect(invokeMock).toHaveBeenLastCalledWith("delete_local_library", {
      projectId: "project:test-local",
      libraryKind: "material",
      libraryId: "matlib.invented.alpha"
    });
    expect(deleteRoute.route === "tauri_backend" && deleteRoute.receipt.deleted).toBe(true);
  });
});

// Phase C3 resolution-preview helpers (TP-C3-LIBREFPICKER-001). Pure traversal
// mirroring the desktop resolver's per-kind dispatch
// (src-tauri extract_library_slot_value / find_library_slot_value).
describe("indexLibraryRecordsSlots", () => {
  it("indexes material records by material_id and allowable slots by allowable_id", () => {
    const document = {
      material_records: [
        {
          material_id: "mat:a",
          allowables: [{ allowable_id: "allow:Sh", value: {} }, { allowable_id: "allow:Sc" }]
        },
        { material_id: "mat:b", allowables: [] }
      ]
    };
    expect(indexLibraryRecordsSlots(document, "material")).toEqual([
      { record_id: "mat:a", slot_ids: ["allow:Sh", "allow:Sc"] },
      { record_id: "mat:b", slot_ids: [] }
    ]);
  });

  it("gathers section slot ids across dimensions and properties in scan order", () => {
    const document = {
      section_records: [
        {
          section_id: "sec:a",
          dimensions: [{ dimension_id: "dim:od" }],
          properties: [{ property_id: "prop:area" }, { property_id: "prop:i" }]
        }
      ]
    };
    expect(indexLibraryRecordsSlots(document, "section")).toEqual([
      { record_id: "sec:a", slot_ids: ["dim:od", "prop:area", "prop:i"] }
    ]);
  });

  it("indexes component records by component_id and field slots by field_id", () => {
    const document = {
      component_records: [{ component_id: "comp:a", fields: [{ field_id: "field:k" }] }]
    };
    expect(indexLibraryRecordsSlots(document, "component")).toEqual([
      { record_id: "comp:a", slot_ids: ["field:k"] }
    ]);
  });

  it("skips records and slots without a string id, de-duplicates slot ids, and tolerates non-array/empty shapes", () => {
    const document = {
      material_records: [
        { allowables: [{ allowable_id: "allow:Sh" }] }, // no material_id -> skipped
        {
          material_id: "mat:a",
          allowables: [{ allowable_id: "allow:Sh" }, { allowable_id: "allow:Sh" }, { value: {} }]
        }
      ]
    };
    expect(indexLibraryRecordsSlots(document, "material")).toEqual([
      { record_id: "mat:a", slot_ids: ["allow:Sh"] }
    ]);
    expect(indexLibraryRecordsSlots(null, "material")).toEqual([]);
    expect(indexLibraryRecordsSlots({ material_records: "nope" }, "material")).toEqual([]);
    expect(indexLibraryRecordsSlots({}, "section")).toEqual([]);
  });
});

describe("classifyLibraryReference", () => {
  const records = [{ record_id: "mat:a", slot_ids: ["allow:Sh", "allow:Sc"] }];

  it("resolves when both the record and the slot are present", () => {
    expect(classifyLibraryReference(records, "mat:a", "allow:Sh")).toBe("resolves");
  });

  it("reports record_missing when no record matches", () => {
    expect(classifyLibraryReference(records, "mat:z", "allow:Sh")).toBe("record_missing");
  });

  it("reports slot_missing when the record exists but the slot does not", () => {
    expect(classifyLibraryReference(records, "mat:a", "allow:zz")).toBe("slot_missing");
  });
});
