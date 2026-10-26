import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import inventedSparse from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import inventedModel from "../../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json";
import pressureSourceN05 from "../../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json";
import pressureConnected from "../../../../../fixtures/results/pressure_connected_mechanics_sparse.json";
import pressureLoadReference from "../../../../../fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json";
import pressureSourceLoadReference from "../../../../../fixtures/product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json";
import { PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL, surfaceRouteRefusal } from "./outputPolicy";
import { ResultsPanel } from "./ResultsPanel";
import { KnownSemanticNotices } from "./KnownSemanticNotices";
import { ComparisonPanel } from "../comparison/ComparisonPanel";
import { LocalFeaHandoffPanel } from "../local-fea-handoff/LocalFeaHandoffPanel";
import { ResultExportPanel } from "../result-export/ResultExportPanel";
import { StressNeutralExportPanel } from "../stress-neutral/StressNeutralExportPanel";
import { COMBINATION_GATE_REASONS, N_HEADLINE, N_HEADLINE_WITHHELD, N_INTENSIFIED, isFreshSemanticResult, knownSemanticNotices } from "./knownSemanticLimitations";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { formatComponentStressModifierSummary } from "../report/ReportPanel";
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
  // B4/D1 (b): N-SB and N-SB-MIXED went with source-blocks-1; a composite shows neither.
  it("Results notices: no source-blocks notice on a composite", () => {
    render(<KnownSemanticNotices result={clone(pressureSourceN05)} testIdPrefix="results" />);
    expect(screen.queryByTestId("results-notice-source-blocks-summary")).toBeNull();
    expect(screen.queryByTestId("results-notice-source-blocks-mixed")).toBeNull();
  });
  it("Comparison shows the gate reason where combination rows would appear", () => {
    render(<ComparisonPanel comparison={null} result={clone(inventedSparse)} onSelectResult={() => {}} />);
    expect(screen.getByTestId("comparison-notice-combination-gate:combination:C-OPER-ALT").textContent).toContain(COMBINATION_GATE_REASONS.NONLINEAR_COMBINATION_REQUIRES_SOLVE);
    expect(screen.getByTestId("comparison-notice-headline-label").textContent).toContain(N_HEADLINE);
  });
  it("Local FEA handoff and the export panels show the headline label", () => {
    render(<LocalFeaHandoffPanel model={model} result={clone(inventedSparse)} analysisRun={null} />);
    expect(screen.getByTestId("local-fea-handoff-notice-headline-label").textContent).toContain(N_HEADLINE);
    render(<ResultExportPanel model={model} result={clone(inventedSparse)} analysisRun={null} />);
    expect(screen.getByTestId("result-export-notice-headline-label").textContent).toContain(N_HEADLINE);
    render(<StressNeutralExportPanel model={model} result={clone(inventedSparse)} analysisRun={null} />);
    expect(screen.getByTestId("stress-neutral-notice-headline-label").textContent).toContain(N_HEADLINE);
  });
  it("report package follows the output policy's report-package cell (W-18)", () => {
    // DEL-08-01 Design `report-package-current-results.md` §2.1: the pressure-free route is
    // ungated, so the builder's own checks apply.
    expect(reportPackageUnavailableReason(clone(inventedSparse))).toBeNull();
    // Owner ruling 10A: pressure-2 and the composite (pressure-source-1), with or without
    // load/reference states, keep their routes' refusal (PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE).
    for (const [exact, reason] of [[pressureConnected, PRESSURE_2_OUTPUT_REFUSAL], [pressureLoadReference, PRESSURE_2_OUTPUT_REFUSAL], [pressureSourceN05, PRESSURE_SOURCE_OUTPUT_REFUSAL], [pressureSourceLoadReference, PRESSURE_SOURCE_OUTPUT_REFUSAL]] as const) {
      expect(isFreshSemanticResult(clone(exact))).toBe(true);
      expect(reportPackageUnavailableReason(clone(exact))).toBe(reason);
      expect(reason.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE: ")).toBe(true);
      expect(surfaceRouteRefusal(clone(exact), "report-package")).toBe(reason);
    }
  });
  it("adds no notice for the exact routes on the opened output panels (§5.3)", () => {
    for (const exact of [pressureConnected, pressureLoadReference, pressureSourceN05, pressureSourceLoadReference]) {
      expect(knownSemanticNotices(clone(exact))).toEqual([]);
      for (const prefix of ["result-export", "stress-neutral", "rule-check"]) {
        const { container, unmount } = render(<KnownSemanticNotices result={clone(exact)} testIdPrefix={prefix} />);
        expect(container.innerHTML).toBe("");
        unmount();
      }
    }
  });
  it("report packet SIF section never presents SIF×k rows as stress", () => {
    // The component refs of SIF×k review rows (component_user_stress_multiplier_review),
    // supplied directly: no current route publishes them. The formatter reads only
    // the records and the source's intensified rows (none without a source).
    const records = ["component:C-110", "component:C-110", "component:C-120"].map(component_ref => ({ component_ref }));
    const historical = formatComponentStressModifierSummary(records, null);
    expect(historical).toContain(`${records.length} retired SIF×k review rows (not a stress)`);
    expect(historical).not.toMatch(/MPa|units=/);
    expect(formatComponentStressModifierSummary([], clone(inventedSparse))).toContain(`8 intensified measure rows (${N_INTENSIFIED})`);
  });
});
