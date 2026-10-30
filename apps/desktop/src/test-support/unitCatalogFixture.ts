import type { UnitCatalog, UnitCatalogEntry } from "../services/unitCatalogService";

// The DEC-018 catalog entries the project-unit tests read, as the desktop's
// get_unit_catalog command returns them (core/units `CATALOG`). Symbols and
// dimensions only; projectUnitSelection.test.ts checks each against the core
// unit engine, so the fixture cannot list a unit the core lacks.
export const CATALOG_FIXTURE_ROWS: ReadonlyArray<readonly [symbol: string, dimension: string, canonical: boolean]> = [
  ["m", "length", true], ["mm", "length", false], ["in", "length", false], ["ft", "length", false],
  ["N", "force", true], ["kN", "force", false], ["lbf", "force", false],
  ["N*m", "moment", true], ["kN*m", "moment", false], ["lbf*in", "moment", false], ["lbf*ft", "moment", false],
  ["Pa", "pressure", true], ["kPa", "pressure", false], ["MPa", "pressure", false], ["GPa", "pressure", false],
  ["bar", "pressure", false], ["psi", "pressure", false], ["ksi", "pressure", false],
  ["K", "temperature", true], ["degC", "temperature", false], ["degF", "temperature", false], ["degR", "temperature", false],
  ["K", "temperature_interval", true], ["degC", "temperature_interval", false], ["degF", "temperature_interval", false], ["degR", "temperature_interval", false],
  ["rad", "angle", true], ["deg", "angle", false],
  ["N/m", "force_per_length", true], ["kN/m", "force_per_length", false], ["N/mm", "force_per_length", false],
  ["kN/mm", "force_per_length", false], ["lbf/ft", "force_per_length", false], ["lbf/in", "force_per_length", false],
  ["N/m", "linear_stiffness", true],
  ["N*m/rad", "rotational_stiffness", true],
  ["m^2", "area", true], ["mm^2", "area", false], ["in^2", "area", false],
  ["m/s^2", "acceleration", true],
  ["1/K", "thermal_expansion_coefficient", true], ["1/degC", "thermal_expansion_coefficient", false], ["1/degF", "thermal_expansion_coefficient", false]
];

export function unitCatalogFixture(): UnitCatalog {
  const entries: UnitCatalogEntry[] = CATALOG_FIXTURE_ROWS.map(([symbol, dimension, canonical]) => ({
    unit_id: `unit:fixture:${dimension}:${symbol}`,
    symbol,
    dimension_id: dimension,
    canonical,
    transform_kind: canonical ? "identity" : "multiplicative",
    factor_representation: "fixture",
    offset_representation: null,
    provenance: canonical ? "si_canonical" : "exact_public_definition",
    review_status: "accepted"
  }));
  return {
    schema_version: "0.1.0",
    catalog_id: "unit-system:dec-018-si-dual-display",
    decision_basis: "DEC-018",
    calculation_basis: "si_canonical",
    storage_convention: "entered_units_preserved",
    entry_count: entries.length,
    entries,
    boundary: {
      source: "test fixture mirroring core/units open_pipe_stress_units catalog",
      protected_content_included: false,
      private_project_data_included: false,
      professional_approval_claimed: false,
      code_compliance_claimed: false
    }
  };
}
