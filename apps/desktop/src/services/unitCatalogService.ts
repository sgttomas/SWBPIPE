import { invoke } from "@tauri-apps/api/core";

export type UnitCatalogEntry = {
  unit_id: string;
  symbol: string;
  dimension_id: string;
  canonical: boolean;
  transform_kind: "identity" | "multiplicative" | "affine";
  factor_representation: string;
  offset_representation: string | null;
  provenance:
    | "si_canonical"
    | "exact_public_definition"
    | "conventional_public_constant"
    | "project_governed_decision";
  review_status: "accepted" | "pending" | "rejected" | "quarantined" | "TBD";
};

export type UnitCatalogBoundary = {
  source: string;
  protected_content_included: boolean;
  private_project_data_included: boolean;
  professional_approval_claimed: boolean;
  code_compliance_claimed: boolean;
};

export type UnitCatalog = {
  schema_version: string;
  catalog_id: string;
  decision_basis: string;
  calculation_basis: "si_canonical";
  storage_convention: "entered_units_preserved";
  entry_count: number;
  entries: UnitCatalogEntry[];
  boundary: UnitCatalogBoundary;
};

export type UnitCatalogRoute =
  | { route: "tauri_unit_catalog"; catalog: UnitCatalog }
  | { route: "unavailable_browser_preview"; diagnostic: string };

export type UnitBasisDisplay = {
  symbol: string;
  dimension_id: string;
  label: string;
  detail: string;
  source:
    | "catalog_loading"
    | "browser_preview_model_metadata"
    | "dec018_catalog_accepted"
    | "dec018_catalog_unreviewed"
    | "dec018_catalog_miss";
  unit_id: string | null;
  review_status: UnitCatalogEntry["review_status"] | null;
  canonical: boolean | null;
};

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function loadUnitCatalog(): Promise<UnitCatalogRoute> {
  if (!isTauriRuntime()) {
    return {
      route: "unavailable_browser_preview",
      diagnostic:
        "UNIT-CATALOG-DESKTOP-ONLY: the reviewed DEC-018 unit catalog is exposed through the " +
        "desktop (Tauri) backend command get_unit_catalog; browser preview mode does not " +
        "synthesize a fallback catalog."
    };
  }
  const catalog = await invoke<UnitCatalog>("get_unit_catalog");
  return { route: "tauri_unit_catalog", catalog };
}

export function acceptedUnits(catalog: UnitCatalog): UnitCatalogEntry[] {
  return catalog.entries.filter((entry) => entry.review_status === "accepted");
}

export function unitCatalogEntryForSymbol(
  catalog: UnitCatalog,
  symbol: string,
  dimensionId: string
): UnitCatalogEntry | null {
  const normalizedSymbol = normalizeSymbol(symbol);
  const normalizedDimension = normalizeDimension(dimensionId);
  return (
    catalog.entries.find(
      (entry) =>
        entry.symbol === normalizedSymbol &&
        entry.review_status === "accepted" &&
        dimensionMatches(entry.dimension_id, normalizedDimension)
    ) ??
    catalog.entries.find(
      (entry) => entry.symbol === normalizedSymbol && dimensionMatches(entry.dimension_id, normalizedDimension)
    ) ??
    null
  );
}

export function unitEntryMatchesDimension(entry: UnitCatalogEntry, dimensionId: string): boolean {
  return dimensionMatches(entry.dimension_id, normalizeDimension(dimensionId));
}

export function describeUnitBasis(
  route: UnitCatalogRoute | null,
  symbol: string,
  dimensionId: string
): UnitBasisDisplay {
  const normalizedSymbol = normalizeSymbol(symbol);
  const normalizedDimension = normalizeDimension(dimensionId);
  if (!route) {
    return {
      symbol: normalizedSymbol,
      dimension_id: normalizedDimension,
      label: `${normalizedSymbol}, catalog loading`,
      detail: "DEC-018 unit catalog has not finished loading.",
      source: "catalog_loading",
      unit_id: null,
      review_status: null,
      canonical: null
    };
  }
  if (route.route === "unavailable_browser_preview") {
    return {
      symbol: normalizedSymbol,
      dimension_id: normalizedDimension,
      label: `${normalizedSymbol}, model metadata`,
      detail: route.diagnostic,
      source: "browser_preview_model_metadata",
      unit_id: null,
      review_status: null,
      canonical: null
    };
  }

  const entry = unitCatalogEntryForSymbol(route.catalog, normalizedSymbol, normalizedDimension);
  if (!entry) {
    return {
      symbol: normalizedSymbol,
      dimension_id: normalizedDimension,
      label: `${normalizedSymbol}, catalog mismatch`,
      detail: `No DEC-018 catalog entry matches symbol=${normalizedSymbol} and dimension=${normalizedDimension}.`,
      source: "dec018_catalog_miss",
      unit_id: null,
      review_status: null,
      canonical: null
    };
  }

  const accepted = entry.review_status === "accepted";
  const basis = entry.canonical ? "DEC-018 canonical" : "DEC-018 display";
  return {
    symbol: normalizedSymbol,
    dimension_id: normalizedDimension,
    label: `${entry.symbol}, ${accepted ? basis : `DEC-018 ${entry.review_status}`}`,
    detail: `${entry.unit_id}; dimension=${entry.dimension_id}; provenance=${entry.provenance}; ${entry.factor_representation}`,
    source: accepted ? "dec018_catalog_accepted" : "dec018_catalog_unreviewed",
    unit_id: entry.unit_id,
    review_status: entry.review_status,
    canonical: entry.canonical
  };
}

function normalizeSymbol(symbol: string): string {
  return symbol.trim() || "TBD";
}

function normalizeDimension(dimensionId: string): string {
  return dimensionId.trim() || "TBD";
}

function dimensionMatches(entryDimension: string, requestedDimension: string): boolean {
  if (entryDimension === requestedDimension) return true;
  return equivalentDimensions(requestedDimension).includes(entryDimension);
}

function equivalentDimensions(dimensionId: string): string[] {
  switch (dimensionId) {
    case "displacement":
      return ["length"];
    case "stress":
      return ["pressure"];
    case "rotation":
      return ["angle"];
    case "slope":
      return ["dimensionless"];
    case "linear_stiffness":
      return ["force_per_length"];
    case "volume_per_length":
      return ["area"];
    default:
      return [];
  }
}
