// RT-R6 (DEL-08-04 Design `csv-export.md` §3): the display rule is split into an exact step and a
// rounding step. The composed form must stay byte-identical to the rule before the split, which
// is pinned here verbatim as `legacyDisplayNumber`; the shared parity vectors are checked by
// `reportQuantityDisplay.parity.test.ts` and the Rust producer by `quantity_display_parity.rs`.
import { describe, expect, it } from "vitest";
import vectors from "../../../../../fixtures/reports/invented/report_quantity_display_parity.json";
import {
  canonicalDecimal,
  displayNumber,
  displayQuantity,
  displayQuantityParts,
  exactDecimalText,
  exactDisplayQuantity,
  roundedDisplayText
} from "./reportQuantityDisplay";

// The rule before the split (git show of the previous `reportQuantityDisplay.ts`), unchanged.
function legacyDisplayNumber(canonical: string, shift: number): string {
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
  if (digits.length > 6) {
    const roundUp = digits[6] >= "5";
    const kept = digits.slice(0, 6).split("");
    if (roundUp) {
      let index = 6;
      for (;;) {
        if (index === 0) {
          // Every kept digit was 9: the significand becomes 1 and the decimal point moves one place.
          kept.unshift("1");
          kept.length = 6;
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

function legacyParts(value: number, unit: string) {
  const scaled = unit === "Pa";
  return { number: legacyDisplayNumber(String(value), scaled ? 6 : 0), unit: scaled ? "MPa" : unit };
}

function samples(): number[] {
  const values = [0, -0, 1, -1, 0.1, 0.5, 999999.5, 9999995, 0.0000999999500, 5e-324, -5e-324, Number.MAX_VALUE, -Number.MAX_VALUE,
    Number.MIN_VALUE, 1e21, 1e-7, 123456789012345680000, 1.2345649999, 1.2345650001, 2 ** 53, 0.30000000000000004];
  let seed = 0x9e3779b9;
  const next = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 2 ** 32; };
  for (let i = 0; i < 4000; i += 1) {
    const mantissa = (next() - 0.5) * 2;
    const exponent = Math.floor(next() * 640) - 320;
    values.push(mantissa * 10 ** exponent, Math.round(mantissa * 1e9) / 1e3);
  }
  for (const vector of vectors.vectors) values.push(vector.value);
  return values.filter(Number.isFinite);
}

describe("display rule split (exact step, then rounding step)", () => {
  it("composes byte-identically to the rule before the split, for every unit form", () => {
    for (const value of samples()) {
      for (const unit of ["Pa", "MPa", "mm", "N", "", "N*m", "rad", "kg"]) {
        expect(displayQuantityParts(value, unit), `${value} ${unit}`).toEqual(legacyParts(value, unit));
      }
      for (const shift of [0, 6]) expect(displayNumber(String(value), shift)).toBe(legacyDisplayNumber(String(value), shift));
    }
    for (const vector of vectors.vectors) expect(displayQuantity(vector.value, vector.unit)).toBe(vector.display);
  });

  it("the exact step is a decimal shift for Pa and the identity otherwise, with no rounding", () => {
    expect(exactDisplayQuantity(123456789.123, "Pa")).toEqual({ sign: "", digits: "123456789123", point: 3, unit: "MPa" });
    expect(exactDecimalText(exactDisplayQuantity(123456789.123, "Pa"))).toBe("123.456789123");
    expect(exactDecimalText(exactDisplayQuantity(5e-324, "Pa"))).toBe("5e-330");
    expect(exactDecimalText(exactDisplayQuantity(1.7976931348623157e308, "Pa"))).toBe("1.7976931348623157e+302");
    expect(exactDecimalText(exactDisplayQuantity(-0, "Pa"))).toBe("-0");
    expect(exactDecimalText(exactDisplayQuantity(0.1, "mm"))).toBe("0.1");
    expect(exactDisplayQuantity(0.1, "mm").unit).toBe("mm");
    for (const value of samples()) {
      for (const unit of ["mm", "N", "", "MPa"]) {
        // The identity: the exact text is the canonical text itself.
        expect(exactDecimalText(exactDisplayQuantity(value, unit))).toBe(Object.is(value, -0) ? "-0" : String(value));
      }
      // The shift: the exact text carries every digit of the canonical text, six places left.
      const shifted = exactDisplayQuantity(value, "Pa");
      const source = canonicalDecimal(Object.is(value, -0) ? "-0" : String(value), 0);
      expect({ ...shifted, unit: undefined }).toEqual({ ...source, point: source.digits === "" ? 0 : source.point - 6, unit: undefined });
      expect(roundedDisplayText(shifted)).toBe(displayQuantityParts(value, "Pa").number);
    }
  });
});
