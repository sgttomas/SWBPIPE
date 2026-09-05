import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitSelector, DisplayUnitsProvider } from "./index";
import { ResultsPanel } from "../results/ResultsPanel";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { KnowledgePanel, physicsRecords } from "../knowledge/KnowledgePanel";
import { ComparisonPanel } from "../comparison/ComparisonPanel";
import { RuleCheckRunPanel } from "../rule-check/RuleCheckRunPanel";
import { buildDiagnosticInterpretation, buildResultInterpretation } from "../results/resultInterpretation";
import type { DesignKnowledge, MechanicsResult, PreviewComparison, PreviewModel } from "../../types";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import { runRuleChecks, type RuleCheckRunResult } from "../../services/ruleCheckService";

vi.mock("../../services/ruleCheckService", async (importOriginal) => ({
  ...await importOriginal<typeof import("../../services/ruleCheckService")>(),
  runRuleChecks: vi.fn()
}));

const sourceResult: MechanicsResult = {
  schema_version: "1", document_kind: "test", run_id: "run:display", model_ref: "model:display",
  status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
  summary: { max_displacement: { value: 0.0254, unit: "m", location_ref: "node:1", result_ref: "disp" } },
  results: [
    { id: "force:i", kind: "element_local_axial_force", value: 10, unit: "N", dimension: "force", entity_ref: "pipe:1", metadata: { component: "axial", coordinate_system: "local", location: "end_i", basis: "test", sign_convention: "signed" } },
    { id: "force:j", kind: "element_local_axial_force", value: -10, unit: "N", dimension: "force", entity_ref: "pipe:1", metadata: { component: "axial", coordinate_system: "local", location: "end_j", basis: "test", sign_convention: "signed" } },
    { id: "disp", kind: "displacement", value: 0.0254, unit: "m", dimension: "length", entity_ref: "node:1" },
    { id: "unknown", kind: "unclassified", value: 9, unit: "N", entity_ref: "pipe:unknown" },
    { id: "temperature:left", kind: "temperature", value: 0, unit: "degC", dimension: "temperature", entity_ref: "pipe:1" },
    { id: "temperature:right", kind: "temperature", value: 10, unit: "degC", dimension: "temperature", entity_ref: "pipe:1" }
  ],
  diagnostics: [{ id: "diagnostic:1", code: "REVIEW", severity: "warning", message: "Keep source 10 N evidence.", affected_refs: ["force:i"] }]
};
const sourceModel = { project: { id: "model:display" }, diagnostics: [] } as unknown as PreviewModel;
const sourceKnowledge: DesignKnowledge = { schema_version: "1", document_kind: "test", knowledge_set_id: "k", model_ref: "model:display", records: [{ id: "authored", kind: "note", title: "Source note", summary: "Keep authored 10 N exactly.", affected_refs: [], provenance: "human", status: "draft" }], diagnostics: [] };

// Fixed service responses test renderer wiring. Rust arithmetic is tested by
// the accepted producer; no renderer implements conversion equations.
function converter() {
  return vi.fn(async (items: DisplayQuantityRequest[]): Promise<DisplayQuantityResult[]> => items.map(item => {
    let value = item.value;
    if (item.to_unit === "lbf") value = item.value < 0 ? -2.248 : 2.248;
    if (item.to_unit === "in") value = 1;
    if (item.to_unit === "degF") value = item.dimension_id === "temperature_interval" ? 18 : item.value === 0 ? 32 : 50;
    if (item.to_unit === "K") value = item.dimension_id === "temperature_interval" ? 10 : item.value === 0 ? 273.15 : 283.15;
    return { id: item.id, status: "converted", value, unit: item.to_unit };
  }));
}
function comparison(): PreviewComparison {
  return {
    schema_version: "1", document_kind: "openpipestress.technical_preview.comparison", deliverable_id: "DEL-14-04", package_id: "PKG-14", scope_items: [], objectives: [], comparison_id: "comparison:1", comparison_kind: "single_run_load_basis_review",
    left: { label: "left", basis_ref: { object_type: "LoadCase", ref: "left" }, model_state_ref: { object_type: "ModelState", ref: "m" }, analysis_run_ref: { object_type: "AnalysisRun", ref: "run:display" }, result_count: 1 },
    right: { label: "right", basis_ref: { object_type: "Combination", ref: "right" }, model_state_ref: { object_type: "ModelState", ref: "m" }, analysis_run_ref: { object_type: "AnalysisRun", ref: "run:display" }, result_count: 1 },
    summary: { comparable_result_pairs: 1, unmatched_left_results: 0, unmatched_right_results: 0, mapping_basis: "explicit", tolerance_status: "not_tolerance_checked", tolerance_profile_ref: "TBD" },
    unit_policy_evidence: { evidence_id: "e", unit_system_ref: { object_type: "UnitSystem", ref: "units" }, storage_convention: "entered_units_preserved", comparison_unit_policy: "explicit", matching_policy: "equal", matched_result_units: ["degC"], unmatched_left_result_count: 0, unmatched_right_result_count: 0, conversion_policy: "none", conversion_performed: false, tolerance_profile_ref: "TBD", tolerance_status: "not_tolerance_checked", decision_basis_refs: [], protected_content_included: false, private_payload_included: false },
    result_deltas: [{ mapping_id: "map:1", left_result_id: "temperature:left", right_result_id: "temperature:right", entity_ref: "pipe:1", result_family: "temperature", component: "temperature", location: "summary", unit: "degC", left_value: 0, right_value: 10, raw_delta: 10, absolute_delta: 10, classification: "not_tolerance_checked", classification_basis: "none" }],
    diagnostics: [], professional_boundary: { human_review_required: true }
  };
}

describe("shared display preference in result renderers", () => {
  it("converts results, endpoint pairs, diagnostic links and knowledge summaries while source evidence remains exact", async () => {
    const result = structuredClone(sourceResult);
    const model = structuredClone(sourceModel);
    const knowledge = structuredClone(sourceKnowledge);
    const source = JSON.stringify({ model, result, knowledge });
    const interpretation = () => JSON.stringify({ result: buildResultInterpretation({ result, resultId: "force:i", knowledge, analysisRun: null }), diagnostic: buildDiagnosticInterpretation({ model, result, knowledge, diagnosticId: "diagnostic:1" }), records: physicsRecords(result) });
    const evidence = interpretation();
    const convert = converter();
    render(<DisplayUnitsProvider converter={convert}><DisplayUnitSelector />
      <ResultsPanel result={result} knowledge={knowledge} analysisRun={null} selectedResultId="force:i" onSelectResult={() => {}} />
      <DiagnosticsPanel model={model} knowledge={knowledge} result={result} selectedDiagnosticId="diagnostic:1" onSelectDiagnostic={() => {}} />
      <KnowledgePanel knowledge={knowledge} result={result} />
    </DisplayUnitsProvider>);
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "US" } });
    await waitFor(() => expect(screen.getByTestId("selected-diagnostic-linked-results")).toHaveTextContent("2.248 lbf"));
    expect(screen.getByTestId("result-row-force:i")).toHaveTextContent("2.248 lbf");
    expect(screen.getByTestId("result-row-dual-force:i")).toHaveTextContent("Entered: 10 N");
    expect(screen.getByTestId("result-detail-panel")).toHaveTextContent("2.248 lbf");
    expect(screen.getByTestId("result-detail-panel")).toHaveTextContent("-2.248 lbf");
    expect(screen.getByTestId("knowledge-record-knowledge:computed-max-displacement")).toHaveTextContent("1 in");
    expect(screen.getByTestId("knowledge-record-knowledge:computed-axial-force")).toHaveTextContent("2.248 lbf");
    expect(screen.getByTestId("result-row-unknown")).toHaveTextContent("9 N");
    expect(screen.getByTestId("result-row-unknown")).toHaveTextContent("no US catalog target for unknown");
    expect(screen.getByTestId("knowledge-record-authored")).toHaveTextContent("Keep authored 10 N exactly.");
    expect(convert.mock.calls.flatMap(call => call[0])).not.toEqual(expect.arrayContaining([expect.objectContaining({value:9})]));
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "SI" } });
    await waitFor(() => expect(screen.getByTestId("result-row-force:i").querySelector('[data-display-status="converted"]')).toHaveTextContent("10 N"));
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "entered" } });
    expect(screen.getByTestId("knowledge-record-knowledge:computed-max-displacement")).toHaveTextContent("0.0254 m");
    expect(JSON.stringify({model,result,knowledge})).toBe(source);
    expect(interpretation()).toBe(evidence);
  });

  it("converts comparison absolute temperatures and interval deltas separately, with honest stale-source fallback", async () => {
    const compared = comparison();
    const before = JSON.stringify(compared);
    const convert = converter();
    const view = (result: MechanicsResult) => <DisplayUnitsProvider initialPreference="US" converter={convert}><DisplayUnitSelector /><ComparisonPanel comparison={compared} result={result} onSelectResult={() => {}} /></DisplayUnitsProvider>;
    const rendered = render(view(sourceResult));
    await waitFor(() => expect(screen.getByTestId("comparison-delta-temperature:right")).toHaveTextContent("18 degF"));
    expect(screen.getByTestId("comparison-reference-temperature:right")).toHaveTextContent("32 degF");
    expect(screen.getByTestId("comparison-target-temperature:right")).toHaveTextContent("50 degF");
    expect(convert.mock.calls.flatMap(call=>call[0])).toEqual(expect.arrayContaining([expect.objectContaining({value:10, dimension_id:"temperature_interval", to_unit:"degF"}), expect.objectContaining({value:10, dimension_id:"temperature", to_unit:"degF"})]));
    fireEvent.change(screen.getByLabelText("Display units"), { target: { value: "SI" } });
    await waitFor(() => expect(screen.getByTestId("comparison-reference-temperature:right")).toHaveTextContent("273.15 K"));
    expect(screen.getByTestId("comparison-delta-temperature:right")).toHaveTextContent("10 K");
    const stale = structuredClone(sourceResult);
    stale.results.find(row=>row.id === "temperature:right")!.value = 20;
    rendered.rerender(view(stale));
    expect(screen.getByTestId("comparison-delta-temperature:right")).toHaveTextContent("no SI catalog target for unknown");
    expect(JSON.stringify(compared)).toBe(before);
  });

  it("converts rule-check picker and reference preview values without changing selections or pack JSON", async () => {
    const result = structuredClone(sourceResult);
    const before = JSON.stringify(result);
    const packText = JSON.stringify({ metadata: {rule_pack_id:"display:pack"}, required_inputs: [
      {input_id:"manual", name:"Manual force", source_kind:"solver_result", quantity_intent:{dimension:"force",unit_ref:"N"}},
      {input_id:"authored", name:"Authored force", source_kind:"solver_result", quantity_intent:{dimension:"force",unit_ref:"N"}, solver_result_ref:{result_id:"force:i"}}
    ] });
    render(<DisplayUnitsProvider converter={converter()}><DisplayUnitSelector /><RuleCheckRunPanel model={sourceModel} result={result} /></DisplayUnitsProvider>);
    fireEvent.change(screen.getByTestId("rule-check-pack-json"), {target:{value:packText}});
    fireEvent.change(screen.getByTestId("rule-check-solver-select-manual"), {target:{value:"force:i"}});
    fireEvent.click(screen.getByTestId("rule-check-solver-preview-authored"));
    fireEvent.change(screen.getByLabelText("Display units"), {target:{value:"US"}});
    await waitFor(() => expect(screen.getByTestId("rule-check-solver-browse-authored")).toHaveTextContent("2.248 lbf"));
    const picker = screen.getByTestId("rule-check-solver-select-manual");
    expect(within(picker).getByRole("option", {name:/force:i/})).toHaveTextContent("2.248 lbf");
    expect(within(picker).getByRole("option", {name:/unclassified/})).toHaveTextContent("no US catalog target for unknown");
    expect(picker).toHaveValue("force:i");
    expect(screen.getByTestId("rule-check-pack-json")).toHaveValue(packText);
    expect(JSON.stringify(result)).toBe(before);
  });

  it("converts typed rule outcomes and retains unsupported catalog references as entered evidence", async () => {
    const outcome: RuleCheckRunResult = {
      document_kind: "test", rule_pack_id: "p", grammar_version: "1", aggregate_status: "USER_RULE_CHECKED", professional_boundary_notice: "Source result evidence.",
      checks: [{ check_id: "c", status: "USER_RULE_CHECKED", computed_value: {value:10, unit_ref:"N", dimension:"force"}, limit_value: {value:12, unit_ref:"private:unit", dimension:"force"}, acceptability_relation:"lte", bound_inputs:[], completeness_findings:[], evaluator_findings:[], diagnostic_codes:[] }]
    };
    const before = JSON.stringify(outcome);
    vi.mocked(runRuleChecks).mockResolvedValueOnce({ route:"tauri_backend", result:outcome });
    const convert = converter();
    convert.mockImplementation(async items => items.map(item => item.from_unit === "private:unit"
      ? {id:item.id,status:"unavailable",message:"Catalog unit unavailable."}
      : {id:item.id,status:"converted",value:2.248,unit:item.to_unit}));
    render(<DisplayUnitsProvider initialPreference="US" converter={convert}><RuleCheckRunPanel model={sourceModel} result={sourceResult} /></DisplayUnitsProvider>);
    const packText = '{"metadata":{"rule_pack_id":"p"}}';
    fireEvent.change(screen.getByTestId("rule-check-pack-json"), {target:{value:packText}});
    fireEvent.click(screen.getByTestId("rule-check-run"));
    await waitFor(() => expect(screen.getByTestId("rule-check-outcome-c")).toHaveTextContent("computed=2.248 lbf"));
    expect(screen.getByTestId("rule-check-outcome-c")).toHaveTextContent("limit=12 private:unit");
    expect(screen.getByTestId("rule-check-outcome-c")).toHaveTextContent("Entered value shown: Catalog unit unavailable.");
    expect(screen.getByTestId("rule-check-pack-json")).toHaveValue(packText);
    expect(JSON.stringify(outcome)).toBe(before);
  });
});
