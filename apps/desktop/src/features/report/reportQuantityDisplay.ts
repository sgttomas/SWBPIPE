/** The report's display rule for result-row values and exact stress headlines (DEL-08-01
 * Design `report-package-current-results.md` §2.3 item 6 and the display resolution beside
 * it; `exact-route-report-sections.md` §3.3).
 *
 * Display only: stored values and units, the package's machine-readable members, hashes and
 * identifiers never change. The rule starts from the value's canonical text (ECMAScript
 * Number-to-String, `String(value)`) and works on its decimal digits as text, with no
 * floating-point arithmetic:
 * - a value stored in `Pa` is shifted six decimal places and labelled `MPa` (1 MPa = 10^6 Pa, exact);
 * - the decimal significand is rounded to six significant digits, half away from zero;
 * - the result is printed with the ECMAScript Number-to-String layout (plain decimal for
 *   decimal-point positions in (-6, 21], exponent form with an explicit sign otherwise; trailing
 *   zeros dropped); 0 and -0 print "0".
 *
 * The report package's Rust producer applies the identical rule
 * (`report_package::quantity_display`); both are checked against
 * `fixtures/reports/invented/report_quantity_display_parity.json`. This module is the report's
 * own and is independent of the session's display units (`features/display-units`). */

/** The stored unit that is displayed scaled. */
export const PASCAL = "Pa";
/** The unit a `Pa` value is displayed in. */
export const MEGAPASCAL = "MPa";
/** Displayed values keep this many significant digits. */
export const SIGNIFICANT_DIGITS = 6;
const PASCAL_TO_MEGAPASCAL_SHIFT = 6;

/** The displayed text `<display value> <display unit>` of a stored value and its stored unit. */
export function displayQuantity(value: number, unit: string): string {
  const parts = displayQuantityParts(value, unit);
  return `${parts.number} ${parts.unit}`;
}

/** The same rule's number text and display unit, apart (a table may state the unit once in
 * its column header). `displayQuantity` is `${number} ${unit}` of these. It is the rounding
 * step applied to the exact step: `roundedDisplayText(exactDisplayQuantity(value, unit))`. */
export function displayQuantityParts(value: number, unit: string): { number: string; unit: string } {
  const exact = exactDisplayQuantity(value, unit);
  return { number: roundedDisplayText(exact), unit: exact.unit };
}

/** A decimal held as text: `sign 0.<digits> x 10^point` (ECMA-262 Number::toString's s, k and
 * n). `digits` has no leading or trailing zero and is "" for zero. */
export type DisplayDecimal = Readonly<{ sign: "" | "-"; digits: string; point: number }>;
/** The exact step's output: the stored value in its display unit, exact and unrounded. */
export type ExactDisplayQuantity = DisplayDecimal & Readonly<{ unit: string }>;

/** The exact step of the display rule (RT-R6; DEL-08-04 Design `csv-export.md` §3). The rule
 * performs one conversion: a value stored in `Pa` is shown in `MPa`, a decimal shift of six
 * places (1 MPa = 10^6 Pa, exact). Every other unit is shown as stored, the identity. The step
 * works on the value's canonical text (`String(value)`) as decimal digits, with no
 * floating-point arithmetic, so its result is exact for every conversion the rule performs. */
export function exactDisplayQuantity(value: number, unit: string): ExactDisplayQuantity {
  const scaled = unit === PASCAL;
  // `String(-0)` is "0": negative zero keeps its sign here, so the step loses nothing.
  const canonical = Object.is(value, -0) ? "-0" : String(value);
  return { ...canonicalDecimal(canonical, scaled ? PASCAL_TO_MEGAPASCAL_SHIFT : 0), unit: scaled ? MEGAPASCAL : unit };
}

/** Whether a value stored in `unit` is displayed scaled (`conversion_performed`). */
export function isDisplayedScaled(unit: unknown): boolean {
  return unit === PASCAL;
}

/** The display rule on a canonical number text, after shifting the decimal point `shift`
 * places to the left. `canonical` is ECMAScript Number-to-String output. */
export function displayNumber(canonical: string, shift: number): string {
  return roundedDisplayText(canonicalDecimal(canonical, shift));
}

/** A canonical number text as an exact decimal, its point shifted `shift` places left. */
export function canonicalDecimal(canonical: string, shift: number): DisplayDecimal {
  const match = /^(-?)(\d+)(?:\.(\d+))?(?:[eE]([+-]?\d+))?$/.exec(canonical);
  if (match === null) throw new Error(`not a canonical number text: ${canonical}`);
  const [, sign, integer, fraction = "", exponentText = "0"] = match;
  // value = 0.<digits> x 10^point (ECMA-262 Number::toString's s, k and n).
  const all = integer + fraction;
  let leading = 0;
  while (leading < all.length && all[leading] === "0") leading += 1;
  const digits = all.slice(leading).replace(/0+$/, "");
  if (digits === "") return { sign: sign as "" | "-", digits: "", point: 0 };
  return { sign: sign as "" | "-", digits, point: integer.length + Number.parseInt(exponentText, 10) - leading - shift };
}

/** The rounding step: six significant digits, half away from zero, in the ECMAScript
 * Number-to-String layout; zero of either sign prints "0". */
export function roundedDisplayText(decimal: DisplayDecimal): string {
  const { sign } = decimal;
  let { digits, point } = decimal;
  if (digits === "") return "0";
  if (digits.length > SIGNIFICANT_DIGITS) {
    const roundUp = digits[SIGNIFICANT_DIGITS] >= "5";
    const kept = digits.slice(0, SIGNIFICANT_DIGITS).split("");
    if (roundUp) {
      let index = SIGNIFICANT_DIGITS;
      for (;;) {
        if (index === 0) {
          // Every kept digit was 9: the significand becomes 1 and the decimal point moves one place.
          kept.unshift("1");
          kept.length = SIGNIFICANT_DIGITS;
          point += 1;
          break;
        }
        index -= 1;
        if (kept[index] === "9") {
          kept[index] = "0";
        } else {
          kept[index] = String.fromCharCode(kept[index].charCodeAt(0) + 1);
          break;
        }
      }
    }
    digits = kept.join("").replace(/0+$/, "");
  }
  return layoutDecimal(sign, digits, point);
}

/** The exact step's text, unrounded, in the same layout; zero keeps its sign ("-0"). */
export function exactDecimalText(decimal: DisplayDecimal): string {
  return decimal.digits === "" ? `${decimal.sign}0` : layoutDecimal(decimal.sign, decimal.digits, decimal.point);
}

/** The ECMAScript Number-to-String layout of `sign 0.<digits> x 10^point`: plain decimal for
 * decimal-point positions in (-6, 21], exponent form with an explicit sign otherwise. */
function layoutDecimal(sign: string, digits: string, point: number): string {
  const k = digits.length;
  let text = sign;
  if (k <= point && point <= 21) {
    text += digits + "0".repeat(point - k);
  } else if (0 < point && point <= 21) {
    text += `${digits.slice(0, point)}.${digits.slice(point)}`;
  } else if (-6 < point && point <= 0) {
    text += `0.${"0".repeat(-point)}${digits}`;
  } else {
    const exponent = point - 1;
    text += `${digits[0]}${k > 1 ? `.${digits.slice(1)}` : ""}e${exponent >= 0 ? "+" : "-"}${Math.abs(exponent)}`;
  }
  return text;
}
