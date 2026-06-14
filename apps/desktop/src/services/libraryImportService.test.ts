import { afterEach, describe, expect, it, vi } from "vitest";
import type { LibraryImportValidation } from "./libraryImportService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import {
  LIBRARY_IMPORT_BACKEND_DIAGNOSTIC,
  partitionLibraryImportFindings,
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
});
