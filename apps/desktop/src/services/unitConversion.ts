// Display-only dual-unit conversion (DEC-018 "si-dual-display").
//
// The reviewed DEC-018 unit catalog is owned by the Rust backend; its numeric
// transforms are not exposed to the frontend (the catalog entry carries only a
// prose `factor_representation`). Rather than parse that prose — or add a new
// backend contract (out of scope for this UI tranche, §10) — this module holds a
// small curated table of EXACT public unit-definition constants (the same
// constants the catalog prose describes) for the units the UI actually shows.
//
// This is DISPLAY ONLY. It never mutates a model value and is never written back
// to storage; the entered value and its unit remain authoritative
// (entered_units_preserved). The converted companion is always labelled "≈".
export type DualUnitDisplay = {
  enteredValue: string;
  enteredUnit: string;
  convertedValue: string | null;
  convertedUnit: string | null;
  /** "5 in (≈ 127 mm)", or just "5 mm" when no companion conversion applies. */
  text: string;
};

type UnitConversion = {
  dimension: string;
  /** Exact multiplicative factor from this unit to its canonical SI unit. */
  toCanonical: number;
  /** The alternate-system unit shown alongside, or null for none. */
  companion: string | null;
};

// Factors are exact public definitions (NIST SP 811 / SI). Canonical SI unit per
// dimension has toCanonical = 1. Companion picks the natural opposite-system unit.
const UNIT_TABLE: Record<string, UnitConversion> = {
  // length — canonical metre
  m: { dimension: "length", toCanonical: 1, companion: "ft" },
  mm: { dimension: "length", toCanonical: 0.001, companion: "in" },
  cm: { dimension: "length", toCanonical: 0.01, companion: "in" },
  in: { dimension: "length", toCanonical: 0.0254, companion: "mm" },
  ft: { dimension: "length", toCanonical: 0.3048, companion: "m" },
  // pressure / stress — canonical pascal
  Pa: { dimension: "pressure", toCanonical: 1, companion: "psi" },
  kPa: { dimension: "pressure", toCanonical: 1000, companion: "psi" },
  MPa: { dimension: "pressure", toCanonical: 1_000_000, companion: "ksi" },
  GPa: { dimension: "pressure", toCanonical: 1_000_000_000, companion: "ksi" },
  bar: { dimension: "pressure", toCanonical: 100_000, companion: "psi" },
  psi: { dimension: "pressure", toCanonical: 6894.757293168361, companion: "MPa" },
  ksi: { dimension: "pressure", toCanonical: 6_894_757.293168361, companion: "MPa" },
  // force — canonical newton
  N: { dimension: "force", toCanonical: 1, companion: "lbf" },
  kN: { dimension: "force", toCanonical: 1000, companion: "kip" },
  lbf: { dimension: "force", toCanonical: 4.4482216152605, companion: "N" },
  kip: { dimension: "force", toCanonical: 4448.2216152605, companion: "kN" },
  // moment — canonical newton-metre
  "N*m": { dimension: "moment", toCanonical: 1, companion: "lbf*ft" },
  "kN*m": { dimension: "moment", toCanonical: 1000, companion: "lbf*ft" },
  "lbf*ft": { dimension: "moment", toCanonical: 1.3558179483314004, companion: "N*m" },
  "lbf*in": { dimension: "moment", toCanonical: 0.1129848290276167, companion: "N*m" },
  // plane angle — canonical radian
  rad: { dimension: "angle", toCanonical: 1, companion: "deg" },
  deg: { dimension: "angle", toCanonical: Math.PI / 180, companion: "rad" },
  // mass — canonical kilogram
  kg: { dimension: "mass", toCanonical: 1, companion: "lb" },
  g: { dimension: "mass", toCanonical: 0.001, companion: "lb" },
  lb: { dimension: "mass", toCanonical: 0.45359237, companion: "kg" },
  lbm: { dimension: "mass", toCanonical: 0.45359237, companion: "kg" }
};

/** Convert an entered value to its companion-system unit (display only). */
export function convertForDisplay(
  value: number,
  unit: string
): { value: number; unit: string } | null {
  if (!Number.isFinite(value)) return null;
  const from = UNIT_TABLE[unit.trim()];
  if (!from || !from.companion) return null;
  const to = UNIT_TABLE[from.companion];
  if (!to || to.dimension !== from.dimension) return null;
  return { value: (value * from.toCanonical) / to.toCanonical, unit: from.companion };
}

/** Format a converted magnitude to ~4 significant figures without trailing noise. */
export function formatConverted(value: number): string {
  if (!Number.isFinite(value)) return "TBD";
  if (value === 0) return "0";
  return String(Number(value.toPrecision(4)));
}

/**
 * Build the dual-unit display string for an entered (value, unit). Non-numeric
 * values, unknown units, or units without a companion render entered-only.
 */
export function dualUnitDisplay(rawValue: string | number, unit: string): DualUnitDisplay {
  const enteredUnit = unit?.trim() ?? "";
  const enteredValue = typeof rawValue === "number" ? String(rawValue) : String(rawValue ?? "").trim();
  const numeric = typeof rawValue === "number" ? rawValue : Number(enteredValue);
  const base: DualUnitDisplay = {
    enteredValue,
    enteredUnit,
    convertedValue: null,
    convertedUnit: null,
    text: enteredUnit ? `${enteredValue} ${enteredUnit}` : enteredValue
  };
  if (enteredValue === "" || !Number.isFinite(numeric)) return base;
  const converted = convertForDisplay(numeric, enteredUnit);
  if (!converted) return base;
  const convertedValue = formatConverted(converted.value);
  return {
    enteredValue,
    enteredUnit,
    convertedValue,
    convertedUnit: converted.unit,
    text: `${enteredValue} ${enteredUnit} (≈ ${convertedValue} ${converted.unit})`
  };
}
