import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { LibraryManagerPanel } from "./LibraryManagerPanel";
import {
  LIBRARY_IMPORT_BACKEND_DIAGNOSTIC,
  buildInventedLibraryImportTemplate,
  type LibraryImportValidation,
  type LocalLibrarySaveResult
} from "../../services/libraryImportService";
import type { UnitCatalog } from "../../services/unitCatalogService";
import type { PreviewModel } from "../../types";

// Phase C3 GUI slice (TP-C3-LIBGUI-001). jsdom has no Tauri runtime, so the
// browser-preview tests pin the honest desktop-only seam: import documents stay
// in memory and every backend action reports the explicit unavailable
// diagnostic instead of a synthesized fallback. The desktop-mode tests set
// __TAURI_INTERNALS__ and mock invoke to exercise the §13.5 findings display
// and the DEC-036 refuse-to-store surfacing the real backend produces.

const modelStub = {
  project: { id: "project:invented-library-test", name: "Invented Library Test Project" }
} as unknown as PreviewModel;

const unitCatalogFixture: UnitCatalog = {
  schema_version: "0.1.0",
  catalog_id: "unit-system:dec-018-si-dual-display",
  decision_basis: "DEC-018",
  calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved",
  entry_count: 12,
  entries: [
    {
      unit_id: "unit:newton_per_meter",
      symbol: "N/m",
      dimension_id: "force_per_length",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 N/m, SI canonical linear-stiffness basis",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:pound_force_per_inch",
      symbol: "lbf/in",
      dimension_id: "force_per_length",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "175.12683524647636 N/m per lbf/in",
      offset_representation: null,
      provenance: "conventional_public_constant",
      review_status: "accepted"
    },
    {
      unit_id: "unit:pascal",
      symbol: "Pa",
      dimension_id: "pressure",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 Pa/Pa, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:megapascal",
      symbol: "MPa",
      dimension_id: "pressure",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "1000000 Pa/MPa, exact SI prefix definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:kilogram_per_cubic_meter",
      symbol: "kg/m^3",
      dimension_id: "density",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 (kg/m^3)/(kg/m^3), SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:per_degree_celsius",
      symbol: "1/degC",
      dimension_id: "thermal_expansion_coefficient",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "1 (1/degC)/(1/K), exact Celsius interval definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:meter",
      symbol: "m",
      dimension_id: "length",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 m/m, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:inch",
      symbol: "in",
      dimension_id: "length",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "0.0254 m/in, exact public definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:square_meter",
      symbol: "m^2",
      dimension_id: "area",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 m^2/m^2, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:square_inch",
      symbol: "in^2",
      dimension_id: "area",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "0.00064516 m^2/in^2, exact public definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:meter_fourth",
      symbol: "m^4",
      dimension_id: "second_moment_area",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 m^4/m^4, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:kilogram_per_meter",
      symbol: "kg/m",
      dimension_id: "mass_per_length",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 (kg/m)/(kg/m), SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    }
  ],
  boundary: {
    source: "core/units open_pipe_stress_units catalog",
    protected_content_included: false,
    private_project_data_included: false,
    professional_approval_claimed: false,
    code_compliance_claimed: false
  }
};

const blockedValidation: LibraryImportValidation = {
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
  diagnostics: [],
  professional_boundary_notice:
    "Library-import validation reports software findings only over an already-parsed import payload."
};

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("LibraryManagerPanel (browser preview seam)", () => {
  it("scopes the manager to the loaded project and stays honest without one", () => {
    render(<LibraryManagerPanel model={null} />);
    expect(screen.getByTestId("library-scope-status").textContent).toContain(
      "create or open a local project first"
    );
    cleanup();

    render(<LibraryManagerPanel model={modelStub} />);
    expect(screen.getByTestId("library-scope-status").textContent).toContain(
      "project:invented-library-test"
    );
    expect(screen.getByTestId("library-scope-status").textContent).toContain("local SQLite only");
  });

  it("loads an invented private-by-default sample for the selected kind and supports discard", () => {
    render(<LibraryManagerPanel model={modelStub} />);
    const textarea = screen.getByTestId("library-draft-json") as HTMLTextAreaElement;
    expect(textarea.disabled).toBe(true);
    expect(screen.getByTestId("library-validate")).toHaveProperty("disabled", true);

    fireEvent.click(screen.getByTestId("library-load-template"));
    expect(textarea.disabled).toBe(false);
    const material = JSON.parse(textarea.value) as Record<string, unknown>;
    expect(material.material_library).toBeTruthy();
    const materialLibrary = material.material_library as Record<string, unknown>;
    expect(materialLibrary.privacy_class).toBe("private_user_data");
    const provenance = materialLibrary.provenance as Record<string, unknown>;
    expect(provenance.redistribution_status).toBe("private_only");
    expect(screen.getByTestId("library-action-status").textContent).toContain("private_user_data");

    // Switching kind then reloading yields the matching library shape.
    fireEvent.change(screen.getByTestId("library-kind-select"), { target: { value: "component" } });
    fireEvent.click(screen.getByTestId("library-load-template"));
    const component = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    expect(component.component_library).toBeTruthy();
    expect(component.component_records).toEqual([]);

    fireEvent.click(screen.getByTestId("library-discard-draft"));
    expect((screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value).toBe("");
    expect(screen.getByTestId("library-action-status").textContent).toContain("discarded");
  });

  it("reports the explicit desktop-only diagnostic for backend actions in browser preview", async () => {
    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("library-load-template"));

    fireEvent.click(screen.getByTestId("library-validate"));
    await waitFor(() =>
      expect(screen.getByTestId("library-action-status").textContent).toContain(
        "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY"
      )
    );

    fireEvent.click(screen.getByTestId("library-refresh-list"));
    await waitFor(() =>
      expect(screen.getByTestId("library-list-status").textContent).toContain(
        "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY"
      )
    );
    expect(screen.getByTestId("library-list-status").textContent).toBe(
      LIBRARY_IMPORT_BACKEND_DIAGNOSTIC
    );
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("blocks save with an honest reason when no project is loaded", async () => {
    render(<LibraryManagerPanel model={null} />);
    fireEvent.click(screen.getByTestId("library-load-template"));
    fireEvent.click(screen.getByTestId("library-save"));
    await waitFor(() =>
      expect(screen.getByTestId("library-action-status").textContent).toContain(
        "create or open a local project first"
      )
    );
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("reports invalid draft JSON honestly instead of acting on it", async () => {
    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("library-load-template"));
    const textarea = screen.getByTestId("library-draft-json") as HTMLTextAreaElement;
    fireEvent.change(textarea, { target: { value: "{not json" } });
    fireEvent.click(screen.getByTestId("library-validate"));
    await waitFor(() =>
      expect(screen.getByTestId("library-action-status").textContent).toContain(
        "LIBRARY-IMPORT-DRAFT-JSON-INVALID"
      )
    );
  });

  it("keeps the private-data and professional boundaries visible", () => {
    render(<LibraryManagerPanel model={modelStub} />);
    const note = screen.getByTestId("library-boundary-note").textContent ?? "";
    expect(note).toContain("never committed to the repository");
    expect(note).toContain("DEC-036");
    expect(note).toContain(
      "Acceptance and professional judgment remain with the responsible engineer",
    );
  });

  it("drafts component field units without synthesizing a browser catalog", async () => {
    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.change(screen.getByTestId("library-kind-select"), { target: { value: "component" } });
    fireEvent.click(screen.getByTestId("library-load-template"));

    expect(screen.getByTestId("component-field-unit-helper")).toBeTruthy();
    await waitFor(() =>
      expect(screen.getByTestId("component-field-unit-basis").textContent).toContain(
        "browser preview mode does not synthesize a fallback catalog"
      )
    );
    expect((screen.getByTestId("component-field-unit") as HTMLSelectElement).value).toBe("N/m");

    fireEvent.change(screen.getByTestId("component-field-value"), { target: { value: "12.5" } });
    fireEvent.click(screen.getByTestId("component-field-apply-draft"));

    const document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    const records = document.component_records as Array<Record<string, unknown>>;
    const field = (records[0].fields as Array<Record<string, unknown>>)[0];
    expect(field.field_kind).toBe("linear_stiffness");
    expect(field.public_repository_value_policy).toBe("private_user_supplied_only");
    const value = field.value as Record<string, unknown>;
    expect(value).toMatchObject({
      magnitude: 12.5,
      unit: "N/m",
      dimension: "linear_stiffness",
      value_status: "private_user_supplied"
    });
    expect(screen.getByTestId("library-action-status").textContent).toContain(
      "Component field draft updated"
    );
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("drafts material property units without synthesizing a browser catalog", async () => {
    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("library-load-template"));

    expect(screen.getByTestId("material-property-unit-helper")).toBeTruthy();
    await waitFor(() =>
      expect(screen.getByTestId("material-property-unit-basis").textContent).toContain(
        "browser preview mode does not synthesize a fallback catalog"
      )
    );
    expect((screen.getByTestId("material-property-unit") as HTMLSelectElement).value).toBe(
      "unit:kilogram_per_cubic_meter"
    );

    fireEvent.change(screen.getByTestId("material-property-kind"), {
      target: { value: "elastic_modulus" }
    });
    expect((screen.getByTestId("material-property-unit") as HTMLSelectElement).value).toBe(
      "unit:pascal"
    );
    fireEvent.change(screen.getByTestId("material-property-value"), {
      target: { value: "210000000000" }
    });
    fireEvent.click(screen.getByTestId("material-property-apply-draft"));

    const document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    const records = document.material_records as Array<Record<string, unknown>>;
    const property = (records[0].properties as Array<Record<string, unknown>>)[0];
    expect(property).toMatchObject({
      property_kind: "elastic_modulus",
      value_status: "private_user_supplied",
      required_for: "mechanics_solve"
    });
    const value = property.value as Record<string, unknown>;
    expect(value).toMatchObject({
      magnitude: 210000000000,
      dimension_id: "stress",
      quantity_kind: "unit_bearing",
      unit_required: true,
      missing_unit_behavior: "diagnostic_blocking"
    });
    expect(value.unit_ref).toMatchObject({ ref_type: "Unit", ref_id: "unit:pascal" });
    expect(screen.getByTestId("library-action-status").textContent).toContain(
      "Material property draft updated"
    );
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("drafts section quantity units without synthesizing a browser catalog", async () => {
    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.change(screen.getByTestId("library-kind-select"), { target: { value: "section" } });
    fireEvent.click(screen.getByTestId("library-load-template"));

    expect(screen.getByTestId("section-quantity-unit-helper")).toBeTruthy();
    await waitFor(() =>
      expect(screen.getByTestId("section-quantity-unit-basis").textContent).toContain(
        "browser preview mode does not synthesize a fallback catalog"
      )
    );
    expect((screen.getByTestId("section-quantity-unit") as HTMLSelectElement).value).toBe("m");

    fireEvent.change(screen.getByTestId("section-quantity-value"), { target: { value: "0.168" } });
    fireEvent.click(screen.getByTestId("section-quantity-apply-draft"));

    let document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    let records = document.section_records as Array<Record<string, unknown>>;
    const dimension = (records[0].dimensions as Array<Record<string, unknown>>)[0];
    expect(dimension).toMatchObject({
      dimension_kind: "outside_diameter",
      value_status: "private_user_supplied",
      required_for: "section_property_calculation"
    });
    expect(dimension.value).toMatchObject({
      magnitude: 0.168,
      unit: "m",
      dimension: "length",
      value_status: "private_user_supplied"
    });

    fireEvent.change(screen.getByTestId("section-quantity-kind"), {
      target: { value: "cross_section_area" }
    });
    expect((screen.getByTestId("section-quantity-unit") as HTMLSelectElement).value).toBe("m^2");
    fireEvent.change(screen.getByTestId("section-quantity-value"), { target: { value: "0.004" } });
    fireEvent.click(screen.getByTestId("section-quantity-apply-draft"));

    document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    records = document.section_records as Array<Record<string, unknown>>;
    const property = (records[0].properties as Array<Record<string, unknown>>)[0];
    expect(property).toMatchObject({
      property_kind: "cross_section_area",
      value_status: "private_user_supplied",
      calculation_status: "not_calculated",
      required_for: "mechanics_solve"
    });
    expect(property.value).toMatchObject({
      magnitude: 0.004,
      unit: "m^2",
      dimension: "area",
      value_status: "private_user_supplied"
    });
    expect(screen.getByTestId("library-action-status").textContent).toContain(
      "Section quantity draft updated"
    );
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

describe("LibraryManagerPanel (desktop backend, mocked invoke)", () => {
  it("filters material property unit choices through the DEC-018 desktop catalog", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.resolve(unitCatalogFixture);
      return Promise.reject(new Error(`unexpected command ${command}`));
    });

    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("library-load-template"));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    fireEvent.change(screen.getByTestId("material-property-kind"), {
      target: { value: "elastic_modulus" }
    });
    const unitSelect = screen.getByTestId("material-property-unit") as HTMLSelectElement;
    await waitFor(() =>
      expect(Array.from(unitSelect.options).map((option) => option.value)).toContain("unit:megapascal")
    );
    expect(Array.from(unitSelect.options).map((option) => option.value)).not.toContain("unit:meter");

    fireEvent.change(unitSelect, { target: { value: "unit:megapascal" } });
    fireEvent.change(screen.getByTestId("material-property-value"), { target: { value: "210000" } });
    fireEvent.click(screen.getByTestId("material-property-apply-draft"));

    const document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    const records = document.material_records as Array<Record<string, unknown>>;
    const property = (records[0].properties as Array<Record<string, unknown>>)[0];
    const value = property.value as Record<string, unknown>;
    expect(value.unit_ref).toMatchObject({ ref_type: "Unit", ref_id: "unit:megapascal" });
    expect(value.dimension_id).toBe("stress");
    expect(screen.getByTestId("material-property-unit-basis").textContent).toContain("unit:megapascal");
  });

  it("filters section quantity unit choices through the DEC-018 desktop catalog", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.resolve(unitCatalogFixture);
      return Promise.reject(new Error(`unexpected command ${command}`));
    });

    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.change(screen.getByTestId("library-kind-select"), { target: { value: "section" } });
    fireEvent.click(screen.getByTestId("library-load-template"));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    fireEvent.change(screen.getByTestId("section-quantity-kind"), {
      target: { value: "cross_section_area" }
    });
    const unitSelect = screen.getByTestId("section-quantity-unit") as HTMLSelectElement;
    await waitFor(() =>
      expect(Array.from(unitSelect.options).map((option) => option.value)).toContain("in^2")
    );
    expect(Array.from(unitSelect.options).map((option) => option.value)).not.toContain("m");

    fireEvent.change(unitSelect, { target: { value: "in^2" } });
    fireEvent.change(screen.getByTestId("section-quantity-value"), { target: { value: "6.2" } });
    fireEvent.click(screen.getByTestId("section-quantity-apply-draft"));

    const document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    const records = document.section_records as Array<Record<string, unknown>>;
    const property = (records[0].properties as Array<Record<string, unknown>>)[0];
    const value = property.value as Record<string, unknown>;
    expect(value.unit).toBe("in^2");
    expect(value.dimension).toBe("area");
    expect(screen.getByTestId("section-quantity-unit-basis").textContent).toContain("unit:square_inch");
  });

  it("filters component field unit choices through the DEC-018 desktop catalog", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.resolve(unitCatalogFixture);
      return Promise.reject(new Error(`unexpected command ${command}`));
    });

    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.change(screen.getByTestId("library-kind-select"), { target: { value: "component" } });
    fireEvent.click(screen.getByTestId("library-load-template"));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    const unitSelect = screen.getByTestId("component-field-unit") as HTMLSelectElement;
    await waitFor(() =>
      expect(Array.from(unitSelect.options).map((option) => option.value)).toContain("lbf/in")
    );
    expect(Array.from(unitSelect.options).map((option) => option.value)).not.toContain("m");

    fireEvent.change(unitSelect, { target: { value: "lbf/in" } });
    fireEvent.change(screen.getByTestId("component-field-value"), { target: { value: "8" } });
    fireEvent.click(screen.getByTestId("component-field-apply-draft"));

    const document = JSON.parse(
      (screen.getByTestId("library-draft-json") as HTMLTextAreaElement).value
    ) as Record<string, unknown>;
    const records = document.component_records as Array<Record<string, unknown>>;
    const field = (records[0].fields as Array<Record<string, unknown>>)[0];
    const value = field.value as Record<string, unknown>;
    expect(value.unit).toBe("lbf/in");
    expect(value.dimension).toBe("linear_stiffness");
    expect(screen.getByTestId("component-field-unit-basis").textContent).toContain(
      "unit:pound_force_per_inch"
    );
  });

  it("splits validation findings along the PRD §13.5 blocking-vs-advisory axis", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.resolve(unitCatalogFixture);
      if (command === "validate_library_import") return Promise.resolve(blockedValidation);
      return Promise.reject(new Error(`unexpected command ${command}`));
    });

    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("library-load-template"));
    fireEvent.click(screen.getByTestId("library-validate"));

    await waitFor(() => expect(screen.getByTestId("library-validation")).toBeTruthy());
    expect(screen.getByTestId("library-validation-summary").textContent).toContain(
      "outcome=QUARANTINE"
    );
    expect(screen.getByTestId("library-validation-summary").textContent).toContain("accepted=false");

    const blocking = screen.getByTestId("library-findings-blocking");
    expect(within(blocking).getByText("IMPORT_PROTECTED_CONTENT_SUSPECTED")).toBeTruthy();
    expect(within(blocking).getByText("IMPORT_LIBRARY_METADATA_MISSING")).toBeTruthy();
    expect(within(blocking).queryByText("IMPORT_REVIEW_REQUIRED")).toBeNull();

    const advisory = screen.getByTestId("library-findings-advisory");
    expect(within(advisory).getByText("IMPORT_REVIEW_REQUIRED")).toBeTruthy();
    expect(within(advisory).queryByText("IMPORT_PROTECTED_CONTENT_SUSPECTED")).toBeNull();
  });

  it("surfaces the DEC-036 refuse-to-store outcome when a blocked import is not stored", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const refused: LocalLibrarySaveResult = {
      project_id: "project:invented-library-test",
      library_kind: "component",
      library_id: "complib.invented.local_draft",
      stored: false,
      storage_mode: "local_sqlite",
      created_at_unix: null,
      updated_at_unix: null,
      document: {},
      validation: blockedValidation,
      message: "Import not stored: validation findings block this library."
    };
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.resolve(unitCatalogFixture);
      if (command === "save_local_library") return Promise.resolve(refused);
      if (command === "list_local_libraries") return Promise.resolve([]);
      return Promise.reject(new Error(`unexpected command ${command}`));
    });

    render(<LibraryManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("library-load-template"));
    fireEvent.click(screen.getByTestId("library-save"));

    await waitFor(() =>
      expect(screen.getByTestId("library-action-status").textContent).toContain("stored=false")
    );
    expect(screen.getByTestId("library-action-status").textContent).toContain(
      "DEC-036 refuse-to-store"
    );
    // The blocked validation rides through to the §13.5 display.
    expect(screen.getByTestId("library-validation-summary").textContent).toContain(
      "blocking_findings=true"
    );
  });
});

describe("buildInventedLibraryImportTemplate", () => {
  it("builds a private-by-default, provenance-complete document per kind", () => {
    for (const kind of ["material", "section", "component"] as const) {
      const document = buildInventedLibraryImportTemplate(kind);
      const libraryKey = `${kind}_library`;
      const recordsKey = `${kind}_records`;
      const library = document[libraryKey] as Record<string, unknown>;
      expect(library).toBeTruthy();
      expect(library.privacy_class).toBe("private_user_data");
      expect(document[recordsKey]).toEqual([]);
      const provenance = library.provenance as Record<string, unknown>;
      for (const field of [
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status"
      ]) {
        expect(provenance[field]).toBeTruthy();
      }
      expect(provenance.redistribution_status).toBe("private_only");
    }
  });
});
