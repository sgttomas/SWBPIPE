// DEL-08-06 AC-002 (VER-002): the desktop assembler gives the reference assembler's section
// set for the same non-empty invented comparison records. The vector is written by the
// reference engine (`tools/fixtures/state_comparison_handoff_comparison_parity.py`); each
// case is compared in RFC 8785 form, the bytes the product hashes. The order and the
// de-duplication key reproduce the reference's canonical text (`canonical_texts`).
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { canonicalJsonString } from "../../services/hashService";
import { buildComparisonSectionSet, pythonJson } from "./comparisonSections";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const VECTOR = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/reports/invented/state_comparison_handoff_comparison_parity.json"), "utf8")) as Json;

describe("DEL-08-06 comparison sections: the desktop assembler and the reference (AC-002)", () => {
  it("covers the cases the criterion and the brief name", () => {
    expect(VECTOR.cases.map((item: Json) => item.name)).toEqual([
      "state_classifications", "run_reasons_and_unmatched", "delta_without_dimension", "authority_term_in_identifier",
      "both_kinds", "both_kinds_permuted", "number_forms", "sparse_state_record"
    ]);
    const byName = (name: string) => VECTOR.cases.find((item: Json) => item.name === name).expected;
    const classifications = new Set(byName("state_classifications").sections.comparison_sections[0].changed_entities.map((row: Json) => row.classification));
    expect(classifications).toEqual(new Set(["added", "removed", "changed", "mapped_changed", "unresolved"]));
    const notCompared = byName("run_reasons_and_unmatched").sections.comparison_sections[0].run_context.not_compared;
    expect(notCompared.flatMap((pair: Json) => pair.reasons)).toEqual([
      "semantic_identity_differs", "left_run_needs_recompute", "right_run_needs_recompute", "family_not_compared", "units_differ", "value_not_numeric", "dimension_missing"
    ]);
    expect(notCompared.some((pair: Json) => "left_magnitude" in pair || "normalized_delta" in pair)).toBe(false);
    expect(byName("delta_without_dimension").diagnostics.map((item: Json) => item.code).sort()).toEqual(["SCH-COMPARISON-DELTA-UNIT-MISSING", "SCH-NUMERIC-UNIT-METADATA-MISSING"]);
    expect(byName("authority_term_in_identifier").diagnostics.every((item: Json) => item.code === "SCH-AUTHORITY-CLAIM-REJECTED" && item.severity === "blocking")).toBe(true);
    expect(byName("both_kinds_permuted")).toEqual(byName("both_kinds"));
  });

  for (const vectorCase of VECTOR.cases as Json[]) {
    it(`gives the reference's set, byte for byte in RFC 8785 form: ${vectorCase.name}`, async () => {
      const desktop = buildComparisonSectionSet(structuredClone(vectorCase.inputs));
      expect(desktop).toEqual(vectorCase.expected);
      expect(await canonicalJsonString(desktop)).toBe(await canonicalJsonString(vectorCase.expected));
    });
  }

  it("orders by the reference's canonical text for strings and non-integral numbers", () => {
    for (const [value, text] of VECTOR.canonical_texts as Array<[unknown, string]>) expect(pythonJson(value), String(text)).toBe(text);
  });

  it("refuses a state, run or handoff record: the vector holds comparison records only", () => {
    const inputs = structuredClone(VECTOR.cases[0].inputs);
    inputs.model_states = [{}];
    expect(() => buildComparisonSectionSet(inputs)).toThrow("DESKTOP-PARITY-FIXTURE-SCOPE");
  });
});
