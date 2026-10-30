// Export length conversion to millimetres (DEL-02-02 Design `project-unit-selection.md`
// §7, "Export length conversion (C2)"). The owner, 2026-10-26: "units conversions that
// are routinely done in engineering applications are permissible as a solution."
//
// One table, exact defined factors: 1 m = 1000 mm, 1 in = 25.4 mm, 1 ft = 304.8 mm (the
// international inch and foot). These are the four lengths the engine registers
// (`core/units/src/lib.rs`, `UnitId::Meter`, `Millimeter`, `Inch`, `Foot`); the factors
// are written here as millimetres per unit, not derived as 0.0254 x 1000. Each quantity
// converts by its own stored unit. Any other unit, an empty one, `TBD` or a missing one
// has no routine exact conversion here and is refused by name; nothing is assumed. A
// source value that is not finite (NaN, +/-Infinity), or one whose product overflows to
// a non-finite value (e.g. 1e308 in), is refused as out of range.
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
  | { status: "refused"; unitLabel: string }
  | { status: "out_of_range"; unitLabel: string };

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
  const product = value * factor;
  if (!Number.isFinite(value) || !Number.isFinite(product)) return { status: "out_of_range", unitLabel: lengthUnitLabel(unit) };
  return {
    status: "converted",
    value: Number(product.toFixed(6)),
    factor,
    sourceMatchesTarget: unit === EXPORT_LENGTH_TARGET_UNIT
  };
}

/** A source value as named in an out-of-range refusal (`NaN`, `Infinity`, `1e+308`). */
export function lengthValueLabel(value: unknown): string {
  return typeof value === "number" ? String(value) : `(${typeof value})`;
}

/** The text of an out-of-range refusal for one quantity. */
export function outOfRangeLengthText(subject: string, value: number, unit: unknown): string {
  return `${subject} (${lengthValueLabel(value)} ${lengthUnitLabel(unit)}) does not give a finite length in millimetres.`;
}

/** Every node coordinate, by sorted node id and axis, that converts out of range. A
 * unit with no routine conversion is refused once for all coordinates instead. */
export function outOfRangeNodeCoordinates(
  nodes: ReadonlyArray<{ id: string; position: { x: number; y: number; z: number } }>,
  unit: unknown
): Array<{ nodeId: string; axis: "x" | "y" | "z"; text: string }> {
  return nodes
    .slice()
    .sort((left, right) => left.id.localeCompare(right.id))
    .flatMap((node) =>
      (["x", "y", "z"] as const)
        .filter((axis) => lengthToMillimeters(node.position[axis], unit).status === "out_of_range")
        .map((axis) => ({
          nodeId: node.id,
          axis,
          text: outOfRangeLengthText(`Node ${node.id} position ${axis}`, node.position[axis], unit)
        }))
    );
}
