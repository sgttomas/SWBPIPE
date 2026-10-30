import { describe, expect, it } from "vitest";
import { convertForDisplay, dualUnitDisplay, formatConverted } from "./unitConversion";

describe("unitConversion (display-only DEC-018 dual units)", () => {
  it("converts US length to SI and back using exact constants", () => {
    expect(convertForDisplay(1, "in")).toEqual({ value: 25.4, unit: "mm" });
    expect(convertForDisplay(1, "ft")).toEqual({ value: 0.3048, unit: "m" });
    const mmToIn = convertForDisplay(25.4, "mm");
    expect(mmToIn?.unit).toBe("in");
    expect(mmToIn?.value).toBeCloseTo(1, 9);
  });

  it("converts stress/pressure pairs", () => {
    const psiToMPa = convertForDisplay(1000, "psi");
    expect(psiToMPa?.unit).toBe("MPa");
    expect(psiToMPa?.value).toBeCloseTo(6.894757, 5);
    const mpaToKsi = convertForDisplay(1, "MPa");
    expect(mpaToKsi?.unit).toBe("ksi");
    expect(mpaToKsi?.value).toBeCloseTo(0.1450377, 6);
  });

  it("converts force, moment, and angle", () => {
    expect(convertForDisplay(1, "lbf")?.value).toBeCloseTo(4.4482216, 6);
    expect(convertForDisplay(1, "N*m")?.unit).toBe("lbf*ft");
    const deg = convertForDisplay(180, "deg");
    expect(deg?.unit).toBe("rad");
    expect(deg?.value).toBeCloseTo(Math.PI, 9);
  });

  it("renders entered-only for unknown units or non-numeric input", () => {
    expect(dualUnitDisplay("TBD", "mm").text).toBe("TBD mm");
    expect(dualUnitDisplay(5, "widgets").text).toBe("5 widgets");
    expect(dualUnitDisplay("not_present", "in").convertedValue).toBeNull();
  });

  it("builds the (≈ converted) display text", () => {
    expect(dualUnitDisplay(5, "in").text).toBe("5 in (≈ 127 mm)");
    expect(dualUnitDisplay(2, "ft").text).toBe("2 ft (≈ 0.6096 m)");
  });

  it("formats converted magnitudes to ~4 significant figures", () => {
    expect(formatConverted(127)).toBe("127");
    expect(formatConverted(0.6096)).toBe("0.6096");
    expect(formatConverted(0)).toBe("0");
  });
});
