import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import inventedSparse from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import inventedModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import precisionSparse from "../../../../../fixtures/results/precision_connected_ui_mechanics_sparse.json";
import multicase from "../../../../../fixtures/product_preview/source_blocks/multicase-sparse_interactive.raw.json";
import sourceBlocksN05 from "../../../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json";
import { ResultsPanel } from "./ResultsPanel";
import { KnownSemanticNotices } from "./KnownSemanticNotices";
import { ComparisonPanel } from "../comparison/ComparisonPanel";
import { LocalFeaHandoffPanel } from "../local-fea-handoff/LocalFeaHandoffPanel";
import { ResultExportPanel } from "../result-export/ResultExportPanel";
import { StressNeutralExportPanel } from "../stress-neutral/StressNeutralExportPanel";
import { COMBINATION_GATE_REASONS, N_HEADLINE, N_HEADLINE_WITHHELD, N_INTENSIFIED, N_P1, N_REPORT, N_SB } from "./knownSemanticLimitations";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { formatComponentStressModifierSummary } from "../report/ReportPanel";
import inventedPrecision from "../../../../../fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json";
import type { MechanicsResult, PreviewModel } from "../../types";

const clone = (value: unknown) => structuredClone(value) as MechanicsResult;
const model = inventedModel as unknown as PreviewModel;

describe("text-only T0R UI (DESIGN §7)", () => {
  it("Results: headline label, intensified label, gate reason, and new kinds listed with their units", () => {
    const result = clone(inventedSparse);
    const before = JSON.stringify(result);
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    expect(screen.getByTestId("results-notice-headline-label").textContent).toContain(N_HEADLINE);
    expect(screen.getByTestId("results-notice-intensified-label").textContent).toContain(N_INTENSIFIED);
    expect(screen.getByTestId("results-notice-combination-gate:combination:C-OPER-ALT").textContent).toContain(COMBINATION_GATE_REASONS.NONLINEAR_COMBINATION_REQUIRES_SOLVE);
    const support = result.results.find(r => r.kind === "support_reaction_component_v2" && r.metadata?.component === "Mz")!;
    fireEvent.change(screen.getByTestId("result-filter-input"), { target: { value: support.id } });
    expect(screen.getByTestId(`result-row-dual-${support.id}`).textContent).toBe(`Entered: ${support.value} N*m`);
    const intensified = result.results.find(r => r.kind === "component_equal_factor_intensified_bending_stress_v1")!;
    fireEvent.change(screen.getByTestId("result-filter-input"), { target: { value: intensified.id } });
    expect(screen.getByTestId(`result-row-dual-${intensified.id}`).textContent).toBe(`Entered: ${intensified.value} Pa`);
    expect(screen.getByTestId(`result-row-label-${intensified.id}`).textContent).toContain(N_INTENSIFIED);
    expect(JSON.stringify(result)).toBe(before);
  });
  // R2 SF-2: split into one render per test (each renders a single result).
  it("Results: withheld reason when the stress headline is null", () => {
    const withheld = clone(inventedSparse); withheld.summary.max_open_formula_stress = null;
    render(<ResultsPanel result={withheld} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    expect(screen.getByTestId("results-notice-headline-withheld").textContent).toBe(N_HEADLINE_WITHHELD);
  });
  it("Results: N-P1 on precision-1", () => {
    render(<ResultsPanel result={clone(precisionSparse)} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    expect(screen.getByTestId("results-notice-precision-1").textContent).toBe(N_P1);
  });
  // The ResultsPanel placement of the notice component is covered by the tests
  // above; rendering the full row table of a source-blocks-1 result costs several
  // seconds per render in jsdom, so N-SB is asserted on the component itself.
  it("Results notices: N-SB on all-selected source-blocks-1", () => {
    render(<KnownSemanticNotices result={clone(sourceBlocksN05)} testIdPrefix="results" />);
    expect(screen.getByTestId("results-notice-source-blocks-summary").textContent).toBe(N_SB);
  });
  it("Comparison shows the gate reason where combination rows would appear", () => {
    render(<ComparisonPanel comparison={null} result={clone(inventedSparse)} onSelectResult={() => {}} />);
    expect(screen.getByTestId("comparison-notice-combination-gate:combination:C-OPER-ALT").textContent).toContain(COMBINATION_GATE_REASONS.NONLINEAR_COMBINATION_REQUIRES_SOLVE);
    expect(screen.getByTestId("comparison-notice-headline-label").textContent).toContain(N_HEADLINE);
  });
  it("Local FEA handoff and the export panels show the headline label; N-SB appears in the export UI only", () => {
    render(<LocalFeaHandoffPanel model={model} result={clone(inventedSparse)} analysisRun={null} />);
    expect(screen.getByTestId("local-fea-handoff-notice-headline-label").textContent).toContain(N_HEADLINE);
    render(<ResultExportPanel model={model} result={clone(multicase)} analysisRun={null} />);
    expect(screen.getByTestId("result-export-notice-source-blocks-summary").textContent).toBe(N_SB);
    render(<StressNeutralExportPanel model={model} result={clone(multicase)} analysisRun={null} />);
    expect(screen.getByTestId("stress-neutral-notice-source-blocks-summary").textContent).toBe(N_SB);
  });
  it("report package is unavailable for every fresh identity (N-REPORT) and for precision-1 (N-P1)", () => {
    expect(reportPackageUnavailableReason(clone(inventedSparse))).toBe(`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE: ${N_REPORT}`);
    expect(reportPackageUnavailableReason(clone(multicase))).toContain(N_REPORT);
    expect(reportPackageUnavailableReason(clone(precisionSparse))).toBe(`REPORT-PACKAGE-PRECISION-1-HISTORICAL: ${N_P1}`);
  });
  it("report packet SIF section never presents SIF×k rows as stress", () => {
    const precision = clone(inventedPrecision);
    const records = precision.results.filter(r => r.kind === "component_user_stress_multiplier_review").map(r => ({ component_ref: r.entity_ref }));
    const historical = formatComponentStressModifierSummary(records, precision);
    expect(historical).toContain(`${records.length} historical precision-1 SIF×k review rows (retired; not a stress)`);
    expect(historical).not.toMatch(/MPa|units=/);
    expect(formatComponentStressModifierSummary([], clone(inventedSparse))).toContain(`8 intensified measure rows (${N_INTENSIFIED})`);
  });
});
