// DEL-08-06 AC-002 (VER-002): the desktop assembler gives the reference assembler's section
// set for the same non-empty invented comparison records. The vector is written by the
// reference engine (`tools/fixtures/state_comparison_handoff_comparison_parity.py`); each
// case is compared in RFC 8785 form, the bytes the product hashes. The order and the
// de-duplication key reproduce the reference's canonical text (`canonical_texts`).
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { canonicalJsonString } from "../../services/hashService";
import { buildComparisonSectionSet, buildCurrentModelStateSection, pythonJson, STATUS_MEMBERS, STATUS_VOCABULARIES } from "./comparisonSections";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const VECTOR = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/reports/invented/state_comparison_handoff_comparison_parity.json"), "utf8")) as Json;

describe("DEL-08-06 comparison sections: the desktop assembler and the reference (AC-002)", () => {
  it("covers the cases the criterion and the brief name", () => {
    expect(VECTOR.cases.map((item: Json) => item.name)).toEqual([
      "state_classifications", "run_reasons_and_unmatched", "delta_without_dimension", "status_vocabulary_state_comparison",
      "status_vocabulary_run_comparison", "former_scan_terms_carried", "both_kinds", "both_kinds_permuted", "number_forms",
      "result_difference_bound", "sparse_state_record"
    ]);
    const byName = (name: string) => VECTOR.cases.find((item: Json) => item.name === name).expected;
    const classifications = new Set(byName("state_classifications").sections.comparison_sections[0].changed_entities.map((row: Json) => row.classification));
    expect(classifications).toEqual(new Set(["added", "removed", "changed", "mapped_changed", "unresolved"]));
    const notCompared = byName("run_reasons_and_unmatched").sections.comparison_sections[0].pairs_not_compared_listed.flatMap((group: Json) => group.rows);
    expect(new Set(notCompared.flatMap((pair: Json) => pair.reasons))).toEqual(new Set([
      "semantic_identity_differs", "left_run_needs_recompute", "right_run_needs_recompute", "family_not_compared", "units_differ", "value_not_numeric", "dimension_missing"
    ]));
    expect(notCompared.some((pair: Json) => "left_magnitude" in pair || "normalized_delta" in pair)).toBe(false);
    const bound = byName("result_difference_bound").sections.comparison_sections[0];
    expect(bound.largest_differences.every((group: Json) => group.differences.length <= 10 && group.differences.every((entry: Json) => entry.normalized_delta !== 0))).toBe(true);
    expect(bound.summary_coverage.differences_not_listed).toBeGreaterThan(0);
    expect(byName("delta_without_dimension").diagnostics.map((item: Json) => item.code).sort()).toEqual(["SCH-COMPARISON-DELTA-UNIT-MISSING", "SCH-NUMERIC-UNIT-METADATA-MISSING"]);
    // A value outside its member's vocabulary: a blocking diagnostic per value, the value carried nowhere.
    for (const name of ["status_vocabulary_state_comparison", "status_vocabulary_run_comparison"]) {
      const set = byName(name);
      const outside = set.diagnostics.filter((item: Json) => item.code === "SCH-STATUS-OUTSIDE-VOCABULARY");
      expect(outside.length, name).toBeGreaterThan(0);
      expect(outside.every((item: Json) => item.severity === "blocking" && item.affected_refs.some((ref: Json) => ref.object_type === "Vocabulary"))).toBe(true);
      const text = JSON.stringify(set);
      for (const value of ["approved_by_engineer", "certified_match", "within_code_limits", "accepted_run", "\"middle\""]) expect(text, `${name} ${value}`).not.toContain(value);
    }
    // Former scan terms in identifiers, field paths, stored values, an author's note and the
    // committed solver diagnostic: carried unchanged, with no diagnostic.
    const former = byName("former_scan_terms_carried");
    expect(former.diagnostics).toEqual([]);
    const formerText = JSON.stringify(former);
    for (const carried of ["node:SEAL-PUMP-1", "provenance.contributor_certification", "Compliant pump, engineering acceptance pending", "No certified inertia", "does not establish engineering acceptance"]) {
      expect(formerText, carried).toContain(carried);
    }
    expect(formerText).not.toContain("omitted_prohibited");
    for (const section of former.sections.comparison_sections) {
      for (const entry of section.unmatched_classifications) expect(entry).not.toHaveProperty("message");
    }
    expect(byName("both_kinds_permuted")).toEqual(byName("both_kinds"));
  });

  for (const vectorCase of VECTOR.cases as Json[]) {
    it(`gives the reference's set, byte for byte in RFC 8785 form: ${vectorCase.name}`, async () => {
      const desktop = buildComparisonSectionSet(structuredClone(vectorCase.inputs));
      expect(desktop).toEqual(vectorCase.expected);
      expect(await canonicalJsonString(desktop)).toBe(await canonicalJsonString(vectorCase.expected));
    });
  }

  for (const stateCase of VECTOR.current_state_cases as Json[]) {
    it(`gives the reference's current-state section, byte for byte: ${stateCase.name}`, async () => {
      const section = buildCurrentModelStateSection(structuredClone(stateCase.record));
      expect(section).toEqual(stateCase.expected);
      expect(await canonicalJsonString(section)).toBe(await canonicalJsonString(stateCase.expected));
      // The document's labels are its author's, carried as written; the software's statuses beside them.
      expect(section.authored_analysis_status).toEqual(stateCase.record.authored_analysis_status);
      expect(section.analysis_status).toEqual(["HUMAN_REVIEW_REQUIRED"]);
      expect(section.software_standing).toMatchObject({ derived: false });
    });
  }

  it("names the same vocabularies and status-bearing members as the reference", () => {
    expect(Object.fromEntries(Object.entries(STATUS_VOCABULARIES).map(([name, values]) => [name, [...values].sort()]))).toEqual(VECTOR.status_vocabularies);
    for (const [kind, members] of Object.entries(STATUS_MEMBERS)) expect(members.map((member) => [...member])).toEqual(VECTOR.status_members[kind]);
  });

  it("orders by the reference's canonical text for strings and non-integral numbers", () => {
    for (const [value, text] of VECTOR.canonical_texts as Array<[unknown, string]>) expect(pythonJson(value), String(text)).toBe(text);
  });

  it("refuses a state, run or handoff record: the vector holds comparison records only", () => {
    const inputs = structuredClone(VECTOR.cases[0].inputs);
    inputs.model_states = [{}];
    expect(() => buildComparisonSectionSet(inputs)).toThrow("DESKTOP-PARITY-FIXTURE-SCOPE");
  });
});
