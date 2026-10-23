import {
  PROJECT_UNIT_KEYS,
  SI_PROJECT_UNITS,
  US_CUSTOMARY_PROJECT_UNITS,
  type ProjectUnitKey,
  type ProjectUnitMap
} from "../../services/projectService";
import {
  unitEntryMatchesDimension,
  type UnitCatalog,
  type UnitCatalogEntry,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";

// W-17 slice (a): project units chosen at creation (DEL-02-02 Design
// `project-unit-selection.md` §2.1-2.4, §3). `project.units` is the one record;
// a preset is UI shorthand and is never stored.

/** The catalog dimension each project unit key is checked against. */
export const PROJECT_UNIT_DIMENSIONS: Readonly<Record<ProjectUnitKey, string>> = Object.freeze({
  length: "length",
  force: "force",
  moment: "moment",
  pressure: "pressure",
  stress: "stress",
  temperature: "temperature",
  angle: "angle"
});

export const PROJECT_UNIT_LABELS: Readonly<Record<ProjectUnitKey, string>> = Object.freeze({
  length: "Length",
  force: "Force",
  moment: "Moment",
  pressure: "Pressure",
  stress: "Stress",
  temperature: "Temperature",
  angle: "Angle"
});

export type ProjectUnitPresetId = "si" | "us_customary" | "custom";

export const PROJECT_UNIT_PRESETS: ReadonlyArray<{ id: Exclude<ProjectUnitPresetId, "custom">; label: string; units: Readonly<ProjectUnitMap> }> = [
  { id: "si", label: "SI", units: SI_PROJECT_UNITS },
  { id: "us_customary", label: "US customary", units: US_CUSTOMARY_PROJECT_UNITS }
];

export function projectUnitPresetFor(units: Readonly<ProjectUnitMap>): ProjectUnitPresetId {
  const preset = PROJECT_UNIT_PRESETS.find((candidate) => PROJECT_UNIT_KEYS.every((key) => candidate.units[key] === units[key]));
  return preset?.id ?? "custom";
}

/** The loaded catalog, or null when the route carries none (browser preview, loading, or a malformed reply). */
export function loadedUnitCatalog(route: UnitCatalogRoute | null): UnitCatalog | null {
  return route?.route === "tauri_unit_catalog" && Array.isArray(route.catalog?.entries) ? route.catalog : null;
}

/**
 * The accepted catalog entry whose `symbol` is exactly `symbol` for the
 * dimension. Aliases are not resolved: the legacy `C` matches no entry, so a
 * new selection cannot use it (§2.2). Existing documents keep it.
 */
export function canonicalCatalogEntry(catalog: UnitCatalog, symbol: string, dimensionId: string): UnitCatalogEntry | null {
  return catalog.entries.find((entry) =>
    entry.symbol === symbol &&
    entry.review_status === "accepted" &&
    unitEntryMatchesDimension(entry, dimensionId)
  ) ?? null;
}

/** The symbols a Custom choice may take for a key: accepted catalog entries of its dimension, canonical first. */
export function projectUnitChoices(catalog: UnitCatalog, key: ProjectUnitKey): string[] {
  const dimension = PROJECT_UNIT_DIMENSIONS[key];
  const symbols = catalog.entries
    .filter((entry) => entry.review_status === "accepted" && unitEntryMatchesDimension(entry, dimension))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical))
    .map((entry) => entry.symbol);
  return [...new Set(symbols)];
}

export type ProjectUnitProblem = { key: ProjectUnitKey; symbol: string; message: string };

/**
 * Why a unit map cannot create a project, if it cannot. With the catalog
 * loaded, each key must be a canonical catalog symbol for its dimension.
 * Without it (browser preview, or still loading), only the preset symbols are
 * known to be canonical, so anything else is refused rather than guessed.
 */
export function projectUnitSelectionProblems(route: UnitCatalogRoute | null, units: Readonly<Record<string, unknown>>): ProjectUnitProblem[] {
  const problems: ProjectUnitProblem[] = [];
  for (const key of PROJECT_UNIT_KEYS) {
    const raw = units[key];
    const symbol = typeof raw === "string" ? raw : "";
    const dimension = PROJECT_UNIT_DIMENSIONS[key];
    if (!symbol) {
      problems.push({ key, symbol, message: `Choose a ${dimension} unit.` });
      continue;
    }
    const catalog = loadedUnitCatalog(route);
    if (catalog) {
      if (!canonicalCatalogEntry(catalog, symbol, dimension)) {
        problems.push({
          key,
          symbol,
          message: symbol === "C"
            ? `C is a legacy spelling; a new project uses the catalog symbol degC.`
            : `${symbol} is not a catalog unit for ${dimension}.`
        });
      }
      continue;
    }
    if (!PROJECT_UNIT_PRESETS.some((preset) => preset.units[key] === symbol)) {
      problems.push({ key, symbol, message: `${symbol} cannot be checked without the desktop unit catalog; use a preset.` });
    }
  }
  return problems;
}

// --- Composed defaults (§2.4) -------------------------------------------------

export type ComposedDefaultKind = "distributed_force" | "linear_stiffness" | "thermal_expansion";

export const COMPOSED_DEFAULTS: ReadonlyArray<{ kind: ComposedDefaultKind; label: string; dimension: string }> = [
  { kind: "distributed_force", label: "Distributed force", dimension: "force_per_length" },
  { kind: "linear_stiffness", label: "Linear stiffness", dimension: "linear_stiffness" },
  { kind: "thermal_expansion", label: "Thermal expansion", dimension: "thermal_expansion_coefficient" }
];

function presentString(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value : null;
}

/** The composed symbol from the project keys, or null when a part is not declared. */
export function composedDefaultSymbol(kind: ComposedDefaultKind, units: Readonly<Record<string, unknown>>): string | null {
  if (kind === "thermal_expansion") {
    const temperature = presentString(units.temperature);
    return temperature ? `1/${temperature}` : null;
  }
  const force = presentString(units.force);
  const length = presentString(units.length);
  return force && length ? `${force}/${length}` : null;
}

export function composedDefaultNotice(symbol: string): string {
  return `${symbol} is not a catalog unit; choose one`;
}

export type CheckedComposedDefault = {
  /** The default to start the field with; "" leaves it unselected. */
  symbol: string;
  /** Why the field starts unselected, when it does. */
  notice: string | null;
};

/**
 * A composed default checked against the loaded catalog. A composition the
 * catalog lacks gives no default and no substitute (§2.4). While the catalog
 * is loading, or where it is not exposed (browser preview), the composition is
 * kept unchecked as before; the core's own unit validation still applies.
 */
export function checkComposedDefault(route: UnitCatalogRoute | null, symbol: string, dimensionId: string): CheckedComposedDefault {
  const catalog = loadedUnitCatalog(route);
  if (!catalog || !symbol || symbol === "TBD") return { symbol, notice: null };
  return canonicalCatalogEntry(catalog, symbol, dimensionId)
    ? { symbol, notice: null }
    : { symbol: "", notice: composedDefaultNotice(symbol) };
}

export type ComposedDefaultSummary = {
  kind: ComposedDefaultKind;
  label: string;
  symbol: string;
  /** true or false against the loaded catalog; null when it cannot be checked. */
  inCatalog: boolean | null;
};

/** The composed defaults a unit map gives, each marked against the catalog (the creation dialog's notice). */
export function composedDefaultsFor(route: UnitCatalogRoute | null, units: Readonly<Record<string, unknown>>): ComposedDefaultSummary[] {
  const catalog = loadedUnitCatalog(route);
  return COMPOSED_DEFAULTS.flatMap(({ kind, label, dimension }) => {
    const symbol = composedDefaultSymbol(kind, units);
    if (!symbol) return [];
    const inCatalog = catalog ? canonicalCatalogEntry(catalog, symbol, dimension) !== null : null;
    return [{ kind, label, symbol, inCatalog }];
  });
}
