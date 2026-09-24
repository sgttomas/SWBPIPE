import { afterEach, describe, expect, it, vi } from "vitest";
import type { UnitCatalog } from "./unitCatalogService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitCatalogEntryForSymbol,
  unitDimensionValidationStatus
} from "./unitCatalogService";

const catalogFixture: UnitCatalog = {
  schema_version: "0.1.0",
  catalog_id: "unit-system:dec-018-si-dual-display",
  decision_basis: "DEC-018",
  calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved",
  entry_count: 5,
  entries: [
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
      factor_representation: "25.4 mm/in, exact by definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:degree_fahrenheit",
      symbol: "degF",
      dimension_id: "temperature",
      canonical: false,
      transform_kind: "affine",
      factor_representation: "5/9 K/degF interval, exact by definition",
      offset_representation: "(459.67 * 5/9) K at 0 degF, exact by definition",
      provenance: "exact_public_definition",
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
      unit_id: "unit:pound_force",
      symbol: "lbf",
      dimension_id: "force",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation:
        "0.45359237 kg/lb * 9.80665 m/s^2 per lbf, conventional standard gravity",
      offset_representation: null,
      provenance: "conventional_public_constant",
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

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("unitCatalogService", () => {
  it("reports an explicit browser-preview unavailable route instead of synthesizing units", async () => {
    const route = await loadUnitCatalog();

    expect(route.route).toBe("unavailable_browser_preview");
    if (route.route !== "unavailable_browser_preview") throw new Error("expected browser-unavailable route");
    expect(route.diagnostic).toContain("UNIT-CATALOG-DESKTOP-ONLY");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("loads the reviewed Tauri unit catalog command in desktop mode", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(catalogFixture);

    const route = await loadUnitCatalog();

    expect(route.route).toBe("tauri_unit_catalog");
    expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog");
    if (route.route !== "tauri_unit_catalog") throw new Error("expected Tauri catalog route");
    expect(route.catalog.decision_basis).toBe("DEC-018");
    expect(route.catalog.storage_convention).toBe("entered_units_preserved");
    expect(route.catalog.boundary.protected_content_included).toBe(false);
    expect(route.catalog.boundary.professional_approval_claimed).toBe(false);
    expect(route.catalog.entries.find((entry) => entry.unit_id === "unit:inch")?.factor_representation).toContain(
      "25.4 mm/in"
    );
    expect(route.catalog.entries.find((entry) => entry.unit_id === "unit:degree_fahrenheit")?.offset_representation)
      .toContain("459.67");
    expect(route.catalog.entries.find((entry) => entry.unit_id === "unit:pound_force")?.provenance).toBe(
      "conventional_public_constant"
    );
    expect(acceptedUnits(route.catalog)).toHaveLength(route.catalog.entry_count);
  });

  it("describes model-unit fields against the reviewed catalog without inventing browser units", () => {
    const tauriRoute = { route: "tauri_unit_catalog" as const, catalog: catalogFixture };

    expect(unitCatalogEntryForSymbol(catalogFixture, "m", "length")?.unit_id).toBe("unit:meter");
    expect(unitCatalogEntryForSymbol(catalogFixture, "Pa", "stress")?.unit_id).toBe("unit:pascal");

    expect(describeUnitBasis(tauriRoute, "m", "length")).toMatchObject({
      label: "m, DEC-018 canonical",
      source: "dec018_catalog_accepted",
      unit_id: "unit:meter",
      review_status: "accepted",
      canonical: true
    });
    expect(describeUnitBasis(tauriRoute, "Pa", "stress")).toMatchObject({
      label: "Pa, DEC-018 canonical",
      source: "dec018_catalog_accepted",
      unit_id: "unit:pascal"
    });
    expect(describeUnitBasis(tauriRoute, "C", "temperature")).toMatchObject({
      label: "C, catalog mismatch",
      source: "dec018_catalog_miss",
      unit_id: null
    });
    expect(
      describeUnitBasis(
        {
          route: "unavailable_browser_preview",
          diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: browser preview does not synthesize a fallback catalog."
        },
        "m",
        "length"
      )
    ).toMatchObject({
      label: "m, model metadata",
      source: "browser_preview_model_metadata",
      unit_id: null
    });
  });

  it("resolves legacy C only for explicit thermal dimensions while preserving the entered token", () => {
    const thermalEntries: UnitCatalog["entries"] = [
      { unit_id: "unit:degree_celsius", symbol: "degC", dimension_id: "temperature", canonical: false,
        transform_kind: "affine", factor_representation: "1 K/degC", offset_representation: "273.15 K",
        provenance: "exact_public_definition", review_status: "accepted" },
      { unit_id: "unit:degree_celsius_interval", symbol: "degC", dimension_id: "temperature_interval", canonical: false,
        transform_kind: "multiplicative", factor_representation: "1 K/degC interval", offset_representation: null,
        provenance: "exact_public_definition", review_status: "accepted" },
      { unit_id: "unit:kelvin", symbol: "K", dimension_id: "temperature", canonical: true,
        transform_kind: "identity", factor_representation: "1 K/K", offset_representation: null,
        provenance: "si_canonical", review_status: "accepted" }
    ];
    const catalog = { ...catalogFixture, entries: [...catalogFixture.entries, ...thermalEntries], entry_count: 8 };
    const before = JSON.stringify(catalog);
    const route = { route: "tauri_unit_catalog" as const, catalog };
    expect(unitCatalogEntryForSymbol(catalog, "C", "temperature")?.unit_id).toBe("unit:degree_celsius");
    expect(unitCatalogEntryForSymbol(catalog, "C", "temperature_interval")?.unit_id).toBe("unit:degree_celsius_interval");
    expect(describeUnitBasis(route, "C", "temperature_interval")).toMatchObject({
      symbol: "C", label: "C (alias of degC), DEC-018 display", source: "dec018_catalog_accepted",
      unit_id: "unit:degree_celsius_interval"
    });
    expect(unitDimensionValidationStatus(route, "C", "temperature")).toBe("dec018_catalog_dimension_match");
    expect(unitDimensionValidationStatus(route, "C", "temperature_interval")).toBe("dec018_catalog_dimension_match");
    for (const dimension of ["length", "force", "pressure", "stress", "linear_stiffness", "thermal_expansion_coefficient", "rotation", "dimensionless"]) {
      expect(unitCatalogEntryForSymbol(catalog, "C", dimension)).toBeNull();
    }
    expect(unitCatalogEntryForSymbol(catalog, "degF", "temperature")?.unit_id).toBe("unit:degree_fahrenheit");
    expect(unitCatalogEntryForSymbol(catalog, "K", "temperature")?.unit_id).toBe("unit:kelvin");
    expect(JSON.stringify(catalog)).toBe(before);
  });

  it("summarizes unit-dimension validation status for catalog and browser routes", () => {
    const tauriRoute = { route: "tauri_unit_catalog" as const, catalog: catalogFixture };
    const browserRoute = {
      route: "unavailable_browser_preview" as const,
      diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: browser preview does not synthesize a fallback catalog."
    };

    expect(unitDimensionValidationStatus(tauriRoute, "Pa", "stress")).toBe(
      "dec018_catalog_dimension_match"
    );
    expect(unitDimensionValidationStatus(tauriRoute, "m", "stress")).toBe(
      "dec018_catalog_dimension_mismatch"
    );
    expect(unitDimensionValidationStatus(browserRoute, "MPa", "stress")).toBe(
      "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview"
    );
    expect(unitDimensionValidationStatus(null, "MPa", "stress")).toBe(
      "catalog_loading_unit_dimension_declared"
    );
    expect(unitDimensionValidationStatus(tauriRoute, "TBD", "stress")).toBe("missing_unit_or_dimension");
    expect(unitDimensionValidationStatus(tauriRoute, "none", "dimensionless")).toBe(
      "not_required_dimensionless"
    );
  });
});
