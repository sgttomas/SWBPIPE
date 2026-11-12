// DEL-08-06 AC-009: the desktop's section set with the saved-state and run comparisons a
// person included. Without any, the set is byte for byte what it was; with them, their
// sections are the reference assembler's (`comparisonSections.ts`) beside the within-run
// comparison, in canonical order, with the set-level rules they touch. Invented data only.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import rangeEnvelopeResult from "../../../../../fixtures/results/preview_physics_range_envelope_sparse.json";
import rangeEnvelopeModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json";
import { describe, expect, it } from "vitest";
import { buildAnalysisRunPreview, buildPreviewComparison } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildComparisonSectionSet } from "./comparisonSections";
import { buildStateComparisonHandoffSections } from "./stateComparisonHandoffSections";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const VECTOR = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/reports/invented/state_comparison_handoff_comparison_parity.json"), "utf8")) as Json;
const caseInputs = (name: string) => structuredClone(VECTOR.cases.find((item: Json) => item.name === name).inputs);

async function inputs() {
  const model = structuredClone(rangeEnvelopeModel) as unknown as PreviewModel;
  const result = structuredClone(rangeEnvelopeResult) as unknown as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, analysisRun, fixed: buildPreviewComparison({ result, analysisRun }), modelHash: "0".repeat(64) };
}

describe("the desktop section set with included comparisons", () => {
  it("is byte for byte unchanged without any", async () => {
    const { model, result, analysisRun, fixed, modelHash } = await inputs();
    for (const comparison of [fixed, null]) {
      const base = { model, result, analysisRun, comparison, modelHash };
      const today = JSON.stringify(buildStateComparisonHandoffSections(base));
      expect(JSON.stringify(buildStateComparisonHandoffSections({ ...base, sessionComparisons: null }))).toBe(today);
      expect(JSON.stringify(buildStateComparisonHandoffSections({ ...base, sessionComparisons: { model_state_comparisons: [], analysis_run_comparisons: [] } }))).toBe(today);
    }
  });

  it("holds the reference's sections of the included records beside the within-run comparison", async () => {
    const { model, result, analysisRun, fixed, modelHash } = await inputs();
    const both = caseInputs("both_kinds");
    const sessionComparisons = { model_state_comparisons: both.model_state_comparisons, analysis_run_comparisons: both.analysis_run_comparisons };
    const reference = buildComparisonSectionSet(both) as Json;
    for (const comparison of [null, fixed]) {
      const set = buildStateComparisonHandoffSections({ model, result, analysisRun, comparison, modelHash, sessionComparisons }) as Json;
      const sections = set.sections.comparison_sections as Json[];
      expect(sections.filter((section) => section.section_kind === "model_state_comparison").length).toBe(2);
      const recordSections = sections.filter((section) => section.hash_refs !== undefined);
      expect(recordSections).toEqual(reference.sections.comparison_sections);
      expect(sections.length).toBe(comparison ? 5 : 4);
      expect(set.diagnostics).toEqual([]);
      // The state and run sections and the envelope are as without comparisons.
      const without = buildStateComparisonHandoffSections({ model, result, analysisRun, comparison, modelHash }) as Json;
      expect(set.sections.state_run_sections).toEqual(without.sections.state_run_sections);
      expect(set.section_set_id).toBe(without.section_set_id);
      expect(set.sow_024_coverage.warnings).toBe("represented_by_warning_and_diagnostic_lists");
      // Deterministic, whatever order the records come in.
      const permuted = { model_state_comparisons: [...sessionComparisons.model_state_comparisons].reverse(), analysis_run_comparisons: [...sessionComparisons.analysis_run_comparisons].reverse() };
      expect(JSON.stringify(buildStateComparisonHandoffSections({ model, result, analysisRun, comparison, modelHash, sessionComparisons: permuted }))).toBe(JSON.stringify(set));
    }
  });

  it("carries a delta without unit metadata with its blocking diagnostics, never as a bare number", async () => {
    const { model, result, analysisRun, modelHash } = await inputs();
    const noDimension = caseInputs("delta_without_dimension");
    const set = buildStateComparisonHandoffSections({ model, result, analysisRun, comparison: null, modelHash, sessionComparisons: { model_state_comparisons: [], analysis_run_comparisons: noDimension.analysis_run_comparisons } }) as Json;
    expect(set.diagnostics.map((item: Json) => [item.code, item.severity]).sort()).toEqual([
      ["SCH-COMPARISON-DELTA-UNIT-MISSING", "blocking"],
      ["SCH-NUMERIC-UNIT-METADATA-MISSING", "blocking"]
    ]);
  });
});
