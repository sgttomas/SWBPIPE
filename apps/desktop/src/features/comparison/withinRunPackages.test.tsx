// W-12 S3c packages (Agent 0, option A): the report, handoff, export-review and
// design-workspace documents carry the within-run comparison the session hands
// them, and the structured unavailable record when the builder refused the chosen
// pair. Without that record each document is byte-identical to before: the key is
// absent and `comparison_ref` reads as it did. Invented data only.
import rangeEnvelopeResult from "../../../../../fixtures/results/preview_physics_range_envelope_sparse.json";
import rangeEnvelopeModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json";
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildAnalysisRunPreview, buildPreviewComparison } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildHandoffPackage } from "../handoff/HandoffPanel";
import { buildStateComparisonHandoffSections } from "../report/stateComparisonHandoffSections";
import { reportComparisonFields, reportComparisonLine } from "../report/ReportPanel";
import { ExportReviewPanel } from "../export-review/ExportReviewPanel";
import { DesignWorkspacePanel } from "../design-workspace/DesignWorkspacePanel";
import { buildWithinRunComparison } from "./withinRunComparison";

afterEach(cleanup);
const RANGE = "combination:U4B-RANGE-MAX";

async function inputs() {
  const model = structuredClone(rangeEnvelopeModel) as unknown as PreviewModel;
  const result = structuredClone(rangeEnvelopeResult) as unknown as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "fixture", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  const fixed = buildPreviewComparison({ result, analysisRun });
  // The producer's own range-envelope rows: the unchanged builder refuses them.
  const unavailable = buildWithinRunComparison(result, analysisRun, { left: "load:L-200", right: RANGE }).unavailable!;
  expect(unavailable).not.toBeNull();
  return { model, result, analysisRun, fixed, unavailable };
}

describe("without an unavailable record, every document is unchanged", () => {
  it("the preview handoff summary and the report package's comparison records", async () => {
    const { model, result, analysisRun, fixed } = await inputs();
    for (const comparison of [fixed, null]) {
      const base = { model, knowledge: null, result, analysisRun, comparison, editorIntents: [], proposal: null, selectedReviewTarget: null };
      expect(JSON.stringify(buildHandoffPackage({ ...base, comparisonUnavailable: null }))).toBe(JSON.stringify(buildHandoffPackage(base)));
      const sections = { model, result, analysisRun, comparison, modelHash: "sha256:invented" };
      expect(JSON.stringify(buildStateComparisonHandoffSections({ ...sections, comparisonUnavailable: null }))).toBe(JSON.stringify(buildStateComparisonHandoffSections(sections)));
      expect(JSON.stringify(buildHandoffPackage(base))).not.toContain("comparison_unavailable");
    }
    // The report's comparison members: the same keys, in the same order, as before.
    const { fixed: today } = await inputs();
    expect(Object.keys(reportComparisonFields(today, null))).toEqual(["comparison_ref", "comparison_summary", "comparison_top_deltas", "comparison_professional_boundary"]);
    expect(reportComparisonFields(today, null).comparison_summary).toMatchObject({ left_basis_ref: "load:L-100", right_basis_ref: "combination:C-OPER-ALT" });
    expect(reportComparisonLine(null, null)).toBeNull();
    // The fixed pair's packaged document names both bases.
    const summary = buildHandoffPackage({ model, knowledge: null, result, analysisRun, comparison: fixed, editorIntents: [], proposal: null, selectedReviewTarget: null });
    expect(summary.comparison_ref).toBe("comparison:run:preview-linear-static-001:load-L-100-to-combination-C-OPER-ALT");
  });
});

describe("with an unavailable record, the documents carry it with the chosen bases", () => {
  it("the preview handoff summary", async () => {
    const { model, result, analysisRun, unavailable } = await inputs();
    const packet = buildHandoffPackage({ model, knowledge: null, result, analysisRun, comparison: null, comparisonUnavailable: unavailable, editorIntents: [], proposal: null, selectedReviewTarget: null }) as Record<string, unknown>;
    expect(packet.comparison_ref).toBe(unavailable.comparison_id);
    expect(packet.comparison_unavailable).toEqual(unavailable);
    expect(packet.comparison_summary).toBeNull();
    expect(unavailable).toMatchObject({ left_basis_ref: "load:L-200", right_basis_ref: RANGE, reason_code: "ANALYSIS-RUN-RESULT-DIMENSION-UNDECLARED" });
  });

  it("the report package's comparison records", async () => {
    const { model, result, analysisRun, unavailable } = await inputs();
    const record = buildStateComparisonHandoffSections({ model, result, analysisRun, comparison: null, comparisonUnavailable: unavailable, modelHash: "sha256:invented" }) as Record<string, unknown>;
    expect(record.comparison_unavailable).toEqual(unavailable);
    expect((record.sections as { comparison_sections: unknown[] }).comparison_sections).toEqual([]);
  });

  it("the report packet, the export review and the design workspace", async () => {
    const { model, result, analysisRun, unavailable } = await inputs();
    // The report: its Comparison line and its export packet's comparison members.
    expect(reportComparisonLine(null, unavailable)).toBe(`${unavailable.comparison_id}; not included: ${unavailable.reason}`);
    expect(reportComparisonFields(null, unavailable)).toEqual({ comparison_ref: unavailable.comparison_id, comparison_unavailable: unavailable, comparison_summary: null, comparison_top_deltas: [], comparison_professional_boundary: null });
    const review = render(<ExportReviewPanel model={model} knowledge={null} result={result} analysisRun={analysisRun} comparison={null} comparisonUnavailable={unavailable} editorIntents={[]} projectOperation="invented" projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />);
    expect(review.container.textContent).toContain(unavailable.comparison_id);
    cleanup();
    const workspace = render(<DesignWorkspacePanel model={model} knowledge={null} result={result} analysisRun={analysisRun} comparison={null} comparisonUnavailable={unavailable} editorIntents={[]} proposal={null} selectedReviewTarget={null} />);
    expect(workspace.container.textContent).toContain(`comparison=${unavailable.comparison_id}`);
  });
});
