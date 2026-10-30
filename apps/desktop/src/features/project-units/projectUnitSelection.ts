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

/**
 * How a project unit key reads (§2.4). A key is absent only when the property is
 * missing or `undefined`. A present key that is not a usable symbol (non-string,
 * empty or whitespace-only) is invalid: it is never read as absent, so it never
 * lets a composition stand in for it.
 */
export type ProjectUnitKeyRead =
  | { state: "absent" }
  | { state: "valid"; symbol: string }
  | { state: "invalid"; notice: string };

export function readProjectUnitKey(units: Readonly<Record<string, unknown>>, key: string): ProjectUnitKeyRead {
  if (!Object.hasOwn(units, key) || units[key] === undefined) return { state: "absent" };
  const value = units[key];
  if (typeof value !== "string") return { state: "invalid", notice: `project.units.${key} is not a unit symbol; choose one` };
  if (!value.trim()) return { state: "invalid", notice: `project.units.${key} is empty; choose one` };
  return { state: "valid", symbol: value };
}

// W-17 slice (c) (§7.3, §7.6): acceleration `<length>/s^2` and area `<length>^2`
// are compositions too. They are not in COMPOSED_DEFAULTS (the creation dialog's
// list) until the catalog carries the acceleration units for every length (§7.3).
export type CompositionKind = ComposedDefaultKind | "moment" | "acceleration" | "area";

const COMPOSITIONS: Readonly<Record<CompositionKind, { parts: readonly string[]; compose: (parts: string[]) => string; dimension: string }>> = {
  distributed_force: { parts: ["force", "length"], compose: ([force, length]) => `${force}/${length}`, dimension: "force_per_length" },
  linear_stiffness: { parts: ["force", "length"], compose: ([force, length]) => `${force}/${length}`, dimension: "linear_stiffness" },
  thermal_expansion: { parts: ["temperature"], compose: ([temperature]) => `1/${temperature}`, dimension: "thermal_expansion_coefficient" },
  moment: { parts: ["force", "length"], compose: ([force, length]) => `${force}*${length}`, dimension: "moment" },
  acceleration: { parts: ["length"], compose: ([length]) => `${length}/s^2`, dimension: "acceleration" },
  area: { parts: ["length"], compose: ([length]) => `${length}^2`, dimension: "area" }
};

/** The composed symbol from the project keys, or null when a part is absent or invalid. */
export function composedDefaultSymbol(kind: CompositionKind, units: Readonly<Record<string, unknown>>): string | null {
  const { parts, compose } = COMPOSITIONS[kind];
  const symbols: string[] = [];
  for (const part of parts) {
    const read = readProjectUnitKey(units, part);
    if (read.state !== "valid") return null;
    symbols.push(read.symbol);
  }
  return compose(symbols);
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

/**
 * The composed default of a field (§2.4). An absent part gives TBD (no unit is
 * invented). A present but invalid part gives no default, with its notice;
 * nothing is composed from it. Otherwise the composition, checked against the
 * catalog.
 */
export function composedDefault(route: UnitCatalogRoute | null, kind: CompositionKind, units: Readonly<Record<string, unknown>>): CheckedComposedDefault {
  const { parts, dimension } = COMPOSITIONS[kind];
  const reads = parts.map((part) => readProjectUnitKey(units, part));
  const invalid = reads.find((read) => read.state === "invalid");
  if (invalid?.state === "invalid") return { symbol: "", notice: invalid.notice };
  if (reads.some((read) => read.state === "absent")) return { symbol: "TBD", notice: null };
  return checkComposedDefault(route, composedDefaultSymbol(kind, units)!, dimension);
}

/**
 * A field whose default is a declared key and only without the key a
 * composition: the moment (`project.units.moment`, else `<force>*<length>`) and
 * the inspector's linear stiffness (`linear_stiffness`, else `<force>/<length>`).
 * A present key is never replaced by a composition. A present but invalid key
 * gives no default, with a notice naming it. With `checkDeclared`, a declared
 * symbol the loaded catalog does not list for the dimension is invalid too.
 */
export function declaredOrComposedDefault(
  route: UnitCatalogRoute | null,
  key: string,
  kind: CompositionKind,
  units: Readonly<Record<string, unknown>>,
  checkDeclared: boolean
): CheckedComposedDefault {
  const read = readProjectUnitKey(units, key);
  if (read.state === "absent") return composedDefault(route, kind, units);
  if (read.state === "invalid") return { symbol: "", notice: read.notice };
  const catalog = checkDeclared ? loadedUnitCatalog(route) : null;
  const dimension = COMPOSITIONS[kind].dimension;
  if (catalog && !canonicalCatalogEntry(catalog, read.symbol, dimension)) {
    return { symbol: "", notice: `project.units.${key} (${read.symbol}) is not a catalog unit for ${dimension}; choose one` };
  }
  return { symbol: read.symbol, notice: null };
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

// --- Slice (c): editor field defaults (§7.2, §7.3, §7.6) ------------------------

/**
 * The project temperature as a new field's starting unit (§7.2): the stored
 * symbol, a legacy `C` given as `degC`, or "" (no unit) when the key is absent,
 * empty, whitespace-only or not a string. Shared with the thermal primitive load,
 * which shows "" as its own "TBD".
 */
export function projectTemperatureDefault(units: Readonly<Record<string, unknown>>): string {
  const read = readProjectUnitKey(units, "temperature");
  if (read.state !== "valid") return "";
  return read.symbol === "C" ? "degC" : read.symbol;
}

/** A field's composed starting unit: the symbol ("" when there is none) and why
 * there is none, either the absent key it is composed from or a notice. */
export type FieldUnitDefault = { symbol: string; notice: string | null; missingKey: string | null };

function firstAbsentPart(parts: readonly string[], units: Readonly<Record<string, unknown>>): string | null {
  return parts.find((part) => readProjectUnitKey(units, part).state === "absent") ?? null;
}

function fieldDefaultFrom(checked: CheckedComposedDefault, missingKey: () => string | null): FieldUnitDefault {
  if (checked.symbol === "TBD") return { symbol: "", notice: null, missingKey: missingKey() };
  return { symbol: checked.symbol, notice: checked.notice, missingKey: null };
}

/** The composed default of an editor field (§2.4, §7.3, §7.6). */
export function fieldComposedDefault(route: UnitCatalogRoute | null, kind: CompositionKind, units: Readonly<Record<string, unknown>>): FieldUnitDefault {
  return fieldDefaultFrom(composedDefault(route, kind, units), () => firstAbsentPart(COMPOSITIONS[kind].parts, units));
}

/** Linear stiffness: a declared `linear_stiffness` key, else `<force>/<length>` (§2.4; the support draft's default). */
export function linearStiffnessFieldDefault(route: UnitCatalogRoute | null, units: Readonly<Record<string, unknown>>): FieldUnitDefault {
  return fieldDefaultFrom(
    declaredOrComposedDefault(route, "linear_stiffness", "linear_stiffness", units, false),
    () => firstAbsentPart(COMPOSITIONS.linear_stiffness.parts, units)
  );
}

/**
 * Rotational stiffness `<moment>/<angle>` (§7.6), with the moment read as
 * declared or composed (§2.4). An invalid part gives its notice; an absent one is
 * named; the composed symbol is checked against the catalog.
 */
export function rotationalStiffnessFieldDefault(route: UnitCatalogRoute | null, units: Readonly<Record<string, unknown>>): FieldUnitDefault {
  const moment = declaredOrComposedDefault(route, "moment", "moment", units, true);
  const angle = readProjectUnitKey(units, "angle");
  if (moment.notice) return { symbol: "", notice: moment.notice, missingKey: null };
  if (angle.state === "invalid") return { symbol: "", notice: angle.notice, missingKey: null };
  if (moment.symbol === "TBD") return { symbol: "", notice: null, missingKey: firstAbsentPart(COMPOSITIONS.moment.parts, units) };
  if (angle.state === "absent") return { symbol: "", notice: null, missingKey: "angle" };
  const checked = checkComposedDefault(route, `${moment.symbol}/${angle.symbol}`, "rotational_stiffness");
  return { symbol: checked.symbol, notice: checked.notice, missingKey: null };
}
