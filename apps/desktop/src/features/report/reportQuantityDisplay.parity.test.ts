import { describe, expect, it } from "vitest";
import vectors from "../../../../../fixtures/reports/invented/report_quantity_display_parity.json";
import type { MechanicsResult } from "../../types";
import { displaysScaledValue } from "./renderableReportInput";
import { displayNumber, displayQuantity } from "./reportQuantityDisplay";

// The report's display rule (DEL-08-01 Design `report-package-current-results.md` §2.3 item 6
// and its display resolution). The Rust producer's rule is checked against the same vectors by
// `core/reporting/report_package/tests/quantity_display_parity.rs`.
describe("report display rule parity", () => {
  it("matches the shared vectors", () => {
    expect(vectors.vectors.length).toBeGreaterThanOrEqual(40);
    for (const vector of vectors.vectors) {
      expect(vector.reason.length, `${vector.value} ${vector.unit}`).toBeGreaterThan(0);
      expect(displayQuantity(vector.value, vector.unit), `${vector.value} ${vector.unit}`).toBe(vector.display);
    }
  });

  it("shifts and rounds the text, not the binary value", () => {
    expect(displayNumber("5e-324", 6)).toBe("5e-330");
    expect(displayNumber("1.7976931348623157e+308", 6)).toBe("1.79769e+302");
    expect(displayNumber("-0.0000999999500", 0)).toBe("-0.0001");
    expect(displayNumber("0", 6)).toBe("0");
    expect(displayNumber("-0", 0)).toBe("0");
    expect(displayNumber("999999.5", 6)).toBe("1");
    expect(displayQuantity(-0, "Pa")).toBe("0 MPa");
  });

  // `conversion_performed` (§2.3 item 5): true only when a displayed value is stored in Pa.
  it("states a conversion only when a displayed value is stored in Pa", () => {
    const row = (unit: string) => ({ id: "r", kind: "k", value: 1, unit });
    const source = (results: unknown[], id?: string, summary?: unknown) =>
      ({ results, producer: id ? { semantic_contract_id: id } : undefined, summary }) as unknown as MechanicsResult;
    expect(displaysScaledValue(source([row("MPa"), row("N")]))).toBe(false);
    expect(displaysScaledValue(source([row("MPa"), row("Pa")]))).toBe(true);
    const headline = (unit: string) => ({ value: 1, unit, location_ref: "l", result_ref: "r" });
    const summary = (nominal: unknown, reduced: unknown) => ({
      max_open_formula_stress_nominal_section: nominal,
      max_open_formula_stress_reduced_section: reduced
    });
    const pressure = "openpipestress.result_semantics/0.3.0/pressure-2";
    const pressureSource = "openpipestress.result_semantics/0.3.0/pressure-source-1";
    const preview = "openpipestress.result_semantics/0.3.0/preview-physics-2";
    expect(displaysScaledValue(source([row("MPa")], pressure, summary(headline("Pa"), headline("Pa"))))).toBe(true);
    expect(displaysScaledValue(source([row("MPa")], pressureSource, summary(null, headline("Pa"))))).toBe(true);
    expect(displaysScaledValue(source([row("MPa")], pressure, summary(null, null)))).toBe(false);
    expect(displaysScaledValue(source([row("MPa")], pressure, summary(headline("MPa"), headline("MPa"))))).toBe(false);
    expect(displaysScaledValue(source([row("MPa")], preview, summary(headline("Pa"), headline("Pa"))))).toBe(false);
  });
});
