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
