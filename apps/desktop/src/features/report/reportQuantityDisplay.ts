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
  const scaled = unit === PASCAL;
  return `${displayNumber(String(value), scaled ? PASCAL_TO_MEGAPASCAL_SHIFT : 0)} ${scaled ? MEGAPASCAL : unit}`;
}

/** Whether a value stored in `unit` is displayed scaled (`conversion_performed`). */
export function isDisplayedScaled(unit: unknown): boolean {
  return unit === PASCAL;
}

/** The display rule on a canonical number text, after shifting the decimal point `shift`
 * places to the left. `canonical` is ECMAScript Number-to-String output. */
export function displayNumber(canonical: string, shift: number): string {
  const match = /^(-?)(\d+)(?:\.(\d+))?(?:[eE]([+-]?\d+))?$/.exec(canonical);
  if (match === null) throw new Error(`not a canonical number text: ${canonical}`);
  const [, sign, integer, fraction = "", exponentText = "0"] = match;
  // value = 0.<digits> x 10^point (ECMA-262 Number::toString's s, k and n).
  const all = integer + fraction;
  let leading = 0;
  while (leading < all.length && all[leading] === "0") leading += 1;
  let digits = all.slice(leading).replace(/0+$/, "");
  if (digits === "") return "0";
  let point = integer.length + Number.parseInt(exponentText, 10) - leading - shift;
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
