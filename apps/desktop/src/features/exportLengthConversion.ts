// Export length conversion to millimetres (DEL-02-02 Design `project-unit-selection.md`
// §7, "Export length conversion (C2)"). The owner, 2026-10-26: "units conversions that
// are routinely done in engineering applications are permissible as a solution."
//
// One table, exact defined factors: 1 m = 1000 mm, 1 in = 25.4 mm, 1 ft = 304.8 mm (the
// international inch and foot). These are the four lengths the engine registers
// (`core/units/src/lib.rs`, `UnitId::Meter`, `Millimeter`, `Inch`, `Foot`); the factors
// are written here as millimetres per unit, not derived as 0.0254 x 1000. Each quantity
// converts by its own stored unit. Any other unit, an empty one, `TBD` or a missing one
// has no routine exact conversion here and is refused by name; nothing is assumed.
//
// The value keeps the exports' existing six-decimal normalisation,
// `Number((value * factor).toFixed(6))`, so metre and millimetre output is unchanged.

export const EXPORT_LENGTH_TARGET_UNIT = "mm";

const MILLIMETERS_PER_UNIT: Readonly<Record<string, number>> = Object.freeze({
  m: 1000,
  mm: 1,
  in: 25.4,
  ft: 304.8
});

export type LengthToMillimeters =
  | { status: "converted"; value: number; factor: number; sourceMatchesTarget: boolean }
  | { status: "refused"; unitLabel: string };

/** The stored unit as named in a refusal: the symbol quoted, or `(missing)`. */
export function lengthUnitLabel(unit: unknown): string {
  return typeof unit === "string" && unit.length > 0 ? `"${unit}"` : "(missing)";
}

/** The exact millimetres-per-unit factor, or null where no routine exact conversion applies. */
export function millimetersPerLengthUnit(unit: unknown): number | null {
  return typeof unit === "string" && Object.hasOwn(MILLIMETERS_PER_UNIT, unit) ? MILLIMETERS_PER_UNIT[unit] : null;
}

export function lengthToMillimeters(value: number, unit: unknown): LengthToMillimeters {
  const factor = millimetersPerLengthUnit(unit);
  if (factor === null) return { status: "refused", unitLabel: lengthUnitLabel(unit) };
  return {
    status: "converted",
    value: Number((value * factor).toFixed(6)),
    factor,
    sourceMatchesTarget: unit === EXPORT_LENGTH_TARGET_UNIT
  };
}
