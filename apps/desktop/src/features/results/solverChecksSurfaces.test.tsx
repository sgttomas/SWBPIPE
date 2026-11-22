// Release exit criterion 18 (LIM-026): what each desktop surface says of the solver's checks. The
// results line and the health indicator say what the checks found; the support-state line stands on
// fresh, bundled and reopened results; the pressure-source recovery's sentence is on its recovered case
// only; the run comparison and its standing read as an admission standing; and each surface that lists
// the solver's own messages carries one heading sentence above them. The results are read through the
// actual wasm reader, from committed fixtures.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { ResultsPanel } from "./ResultsPanel";
import { numericalHealthView, readNumericalHealth } from "./numericalHealth";
import { numericalResultStanding, reportStanding } from "./resultStanding";
import { CONFIRMATION_STATEMENTS, recoveredCaseIds, SOLVER_MESSAGES_HEADING, supportStatesByCase } from "./integrityStatus";
import { ELIGIBLE_RECOVERY_TEXT } from "./numericalHealthText";
import { PhysicsSourceEvidencePanel } from "./PhysicsSourceEvidencePanel";
import { buildBundledReferenceContext, buildHistoricalRunContext, HistoricalRunPanel } from "./HistoricalRunContext";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { KnowledgePanel } from "../knowledge/KnowledgePanel";
import { MissingDataBlockingPanel } from "../missing-data/MissingDataBlockingPanel";
import { RunComparisonView } from "../comparison/RunComparisonView";
import { compareRunInputs } from "../comparison/runComparison";
import type { RunComparisonSide } from "../comparison/runComparisonSides";
import type { RunComparisonResult } from "../history/useModelStateHistory";
import type { BundledMechanicsReference } from "../../services/previewService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import type { DesignKnowledge, LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { RULE_PACK_REF_UNRESOLVED } from "../../services/rulePackRefDiagnostics";

afterEach(cleanup);

const root = resolve(__dirname, "../../../../..");
const read = <T,>(path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as T;
const corpus = (path: string, name: string) => {
  const entry = read<{ documents: { name: string; document: MechanicsResult; model: PreviewModel }[] }>(path).documents.find((d) => d.name === name)!;
  return { result: structuredClone(entry.document), model: structuredClone(entry.model) };
};
const LINEAR = { result: "fixtures/results/preview_physics_connected_sparse.json", model: "fixtures/model_operations/precision_connected_ui_model.json" };
const DEMO = { result: "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json", model: "fixtures/product_preview/invented_demo_model.json" };
const ESTABLISHED = CONFIRMATION_STATEMENTS.solver_reported_established;

function healthFor(result: MechanicsResult, model: PreviewModel | null, eligibleOverride?: boolean) {
  const standing = numericalResultStanding(result, model ?? undefined);
  const reading = model ? reportStanding(result, model) : null;
  const supportStates = reading && reading.readability.readable && reading.readability.solved ? supportStatesByCase(reading.numerical_standing) : undefined;
  return numericalHealthView(readNumericalHealth(result, standing.findings), { eligible: eligibleOverride ?? standing.eligible, verdictHold: false, supportStates, recoveredCases: recoveredCaseIds(result) })!;
}
const panel = (result: MechanicsResult, health: ReturnType<typeof healthFor> | undefined) =>
  render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);

describe("the results line takes the gated health's state", () => {
  it("an unsolved document whose own status says checks_passed reads not assessed, never no problem", () => {
    const source = read<MechanicsResult>(DEMO.result), bound = read<PreviewModel>(DEMO.model);
    source.status = { ...source.status, mechanics: "MODEL_INCOMPLETE" };
    expect(source.numerical_quality!.status).toBe("checks_passed");
    const health = healthFor(source, bound);
    expect(health.cases.length).toBeGreaterThan(0);
    expect(health.cases.every((c) => c.state === "not_assessed")).toBe(true);
    panel(source, health);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: not assessed\. /);
    expect(screen.getByTestId("numerical-result-standing").textContent).not.toContain("no problem");
    cleanup();
    // The panel's own reading of the same document (no health handed in) reads the same.
    panel(source, undefined);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: not assessed\. /);
  });
});

describe("the results stage never reads better than the document's own status", () => {
  it("a solved document whose status is sensitive while every case passed reads sensitive on the line and the header; each case keeps its own", () => {
    const source = read<MechanicsResult>(DEMO.result), bound = read<PreviewModel>(DEMO.model);
    source.numerical_quality = { ...source.numerical_quality!, status: "sensitive" };
    expect(numericalResultStanding(source, bound).findings).toContain("NUMERICAL_AGGREGATE_CONTRADICTION");
    // The panel's own reading (no health handed in), as a reopened or bundled result is shown.
    panel(source, undefined);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: sensitive\. /);
    expect(screen.getByTestId("numerical-health-header").textContent).toBe("Numerical health: sensitive");
    const caseBadges = screen.getAllByTestId(/^numerical-health-case-badge-/);
    expect(caseBadges.length).toBeGreaterThan(0);
    for (const badge of caseBadges) expect(badge.textContent).toBe("Numerical health: solver checks found no problem");
  });
});

describe.each([
  { name: "linear", ...LINEAR, nonlinear: false },
  { name: "nonlinear, confirmation reported established", ...DEMO, nonlinear: true }
])("a result the solver's checks found no problem with ($name)", ({ result: resultPath, model: modelPath, nonlinear }) => {
  const result = () => read<MechanicsResult>(resultPath);
  const model = () => read<PreviewModel>(modelPath);

  it("the results line and the indicator say what the checks found; no row carries a mark", () => {
    const source = result(), bound = model();
    const standing = numericalResultStanding(source, bound);
    expect(standing.status).toBe("solver_checks_passed");
    expect(standing.integrity_bound).toEqual({ status: "not_verified", reason: "inverse_norm_bound_not_verified" });
    const health = healthFor(source, bound);
    expect(health.cases.every((c) => c.state === "checks_passed")).toBe(true);
    panel(source, health);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: solver checks found no problem\. /);
    expect(screen.getByTestId("numerical-health-header").textContent).toBe("Numerical health: solver checks found no problem");
    for (const c of health.cases) {
      expect(screen.getByTestId(`numerical-health-case-text-${c.caseId}`).textContent).toContain(`The solver's residual and conditioning checks found no problem with ${c.caseId}.`);
      const line = screen.queryByTestId(`numerical-health-case-support-states-${c.caseId}`);
      expect(line?.textContent ?? null).toBe(nonlinear ? ESTABLISHED : null);
    }
    expect(document.querySelectorAll("[data-testid^='result-row-health-']")).toHaveLength(0);
  });

  it("the run comparison states the standing in its words", () => {
    const source = result(), bound = model();
    const standing = numericalResultStanding(source, bound);
    const side = (name: string, diagnostics: MechanicsResult["diagnostics"]): RunComparisonSide => ({
      kind: "recorded", id: `runrec:${name}`, description: name, createdAt: null, state: null, standing, recordedTableRead: null,
      input: { rows: [], standing, semantic_identity: { route: standing.contract, contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics }
    } as unknown as RunComparisonSide);
    const left = side("left", []), right = side("right", []);
    render(<RunComparisonView result={{ projectId: "project", left, right, comparison: compareRunInputs(left.input, right.input) }} onClose={() => {}} />);
    expect(screen.getByTestId("run-comparison-side-left").textContent).toBe(`Left: left. Its result is numerically eligible (route ${standing.contract}).`);
    const basis = screen.getByTestId("run-comparison-basis").textContent!.replace(/\s+/g, " ");
    expect(basis).toContain("It is computed only when both runs' results are numerically eligible, both carry the same result semantic identity, and the two units are equal; nothing is converted. Numerically eligible is an admission standing: it asserts no verified error bound.");
    expect(basis.split("Numerically eligible is an admission standing").length - 1).toBe(1);
  });
});

describe("the pressure-source route", () => {
  it("the recovery's sentence on the recovered case only; the ordinary companion reads as the ordinary solve's", () => {
    const source = read<MechanicsResult>("fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json");
    // The recovery's sentence follows an eligible standing (a registered invocation on the live path).
    const health = healthFor(source, null, true);
    expect(health.route).toBe("pressure_source");
    panel(source, health);
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).toContain(ELIGIBLE_RECOVERY_TEXT);
    expect(screen.getByTestId("numerical-health-case-text-case:ordinary-pressure").textContent).not.toContain(ELIGIBLE_RECOVERY_TEXT);
    cleanup();
    render(<PhysicsSourceEvidencePanel result={source} />);
    expect(screen.getByTestId("physics-source-case-case:ordinary-pressure").textContent).toContain("Ordinary case attempt: solver checks found no problem.");
  });
});

describe("the support-state line beyond a fresh solve", () => {
  it("on the bundled reference with nonlinear supports", () => {
    const reference = { standing: "reference_only", model: read<PreviewModel>(DEMO.model), source: read<MechanicsResult>(DEMO.result), provenance: { notice: "Bundled reference." } } as unknown as BundledMechanicsReference;
    render(<HistoricalRunPanel context={buildBundledReferenceContext(reference)} />);
    expect(screen.getAllByText(ESTABLISHED).length).toBeGreaterThan(0);
  });

  it("on a reopened saved run with nonlinear supports", async () => {
    const model = read<PreviewModel>(DEMO.model), mechanics_result = read<MechanicsResult>(DEMO.result);
    const model_hash = await computeModelHash(model);
    const payload = { model, mechanics_result, analysis_run: null, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
    const envelope = { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never) } as unknown as LocalProjectEnvelope;
    render(<HistoricalRunPanel context={(await buildHistoricalRunContext(envelope))!} />);
    await waitFor(() => expect(screen.getAllByText(ESTABLISHED).length).toBeGreaterThan(0));
  });

  it("a confirmation that quotes its reason shows the reason directly beneath", () => {
    const { result, model } = corpus("fixtures/results/preview_physics_held_status_corpus.json", "h15c_zero_limit");
    const health = healthFor(result, model);
    const quoted = health.cases.find((c) => c.supportStates?.text.includes("quoted below"));
    expect(quoted, "a not_established_detail_quoted case").toBeDefined();
    panel(result, health);
    const statement = screen.getByTestId(`numerical-health-case-support-states-${quoted!.caseId}`);
    expect(statement.textContent).toBe(CONFIRMATION_STATEMENTS.not_established_detail_quoted);
    const detail = statement.nextElementSibling!;
    expect(detail.getAttribute("data-testid")).toBe(`numerical-health-case-support-states-detail-${quoted!.caseId}`);
    expect(quoted!.supportStates!.detail!.length).toBeGreaterThan(0);
    expect(detail.textContent).toBe(quoted!.supportStates!.detail!);
  });
});

describe("the heading above the solver's own messages", () => {
  const linear = () => ({ result: read<MechanicsResult>(LINEAR.result), model: read<PreviewModel>(LINEAR.model) });
  const once = (testId: string) => {
    const shown = screen.getAllByTestId(testId);
    expect(shown).toHaveLength(1);
    expect(shown[0].textContent).toBe(SOLVER_MESSAGES_HEADING);
    expect(document.body.textContent!.split(SOLVER_MESSAGES_HEADING).length - 1).toBe(1);
  };

  it("the diagnostics list: once where it lists a solver diagnostic, absent where it lists none", () => {
    const { result, model } = linear();
    expect(result.diagnostics.length).toBeGreaterThan(0);
    const { rerender } = render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    once("diagnostic-list-solver-messages-heading");
    rerender(<DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    expect(screen.queryByTestId("diagnostic-list-solver-messages-heading")).toBeNull();
  });

  it("the diagnostic detail: once where the selected diagnostic is the solver's, absent for the model's own", () => {
    const { result, model: bound } = linear();
    const own = { id: "diagnostic:model-own", code: "MODEL_OWN_FINDING", severity: "warning", message: "A finding of the model's own.", affected_refs: [] };
    const model = { ...bound, diagnostics: [...(bound.diagnostics ?? []), own] } as PreviewModel;
    let selected: string | null = null;
    const { rerender } = render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={null} onSelectDiagnostic={(id) => { selected = id; }} />);
    expect(screen.queryByTestId("diagnostic-detail-solver-messages-heading")).toBeNull();
    fireEvent.click(screen.getAllByTestId(`diagnostic-${result.diagnostics[0].code}`)[0]);
    rerender(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={selected} onSelectDiagnostic={() => {}} />);
    expect(screen.getAllByTestId("diagnostic-detail-solver-messages-heading")).toHaveLength(1);
    rerender(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={own.id} onSelectDiagnostic={() => {}} />);
    expect(screen.getByTestId("selected-diagnostic-id").textContent).toContain(own.id);
    expect(screen.queryByTestId("diagnostic-detail-solver-messages-heading")).toBeNull();
  });

  it("the run comparison's diagnostics: once where it lists any, absent where none", () => {
    const { result, model } = linear();
    const standing = numericalResultStanding(result, model);
    const side = (name: string, diagnostics: unknown[]) => ({
      kind: "recorded", id: `runrec:${name}`, description: name, createdAt: null, state: null, standing, recordedTableRead: null,
      input: { rows: [], standing, semantic_identity: { route: standing.contract, contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics }
    } as unknown as RunComparisonSide);
    const shown = (left: RunComparisonSide, right: RunComparisonSide): RunComparisonResult => ({ projectId: "project", left, right, comparison: compareRunInputs(left.input, right.input) });
    const diagnostics = result.diagnostics.map((d) => ({ code: d.code, severity: d.severity ?? null, affected_refs: d.affected_refs ?? [], message: d.message ?? null }));
    const { rerender } = render(<RunComparisonView result={shown(side("left", []), side("right", diagnostics))} onClose={() => {}} />);
    once("run-comparison-solver-messages-heading");
    rerender(<RunComparisonView result={shown(side("left", []), side("right", []))} onClose={() => {}} />);
    expect(screen.queryByTestId("run-comparison-solver-messages-heading")).toBeNull();
  });

  it("the Design Knowledge panel: once, after the person's own records and directly before the first record made from a solver message; absent where none", () => {
    const source = read<{ envelope: MechanicsResult }>("fixtures/results/rt_a3_s6_c1_correction_unavailable.json").envelope;
    const knowledge = read<DesignKnowledge>("fixtures/product_preview/invented_design_knowledge.json");
    // The person's records are listed for the model the knowledge is bound to (`designKnowledgeRecords`).
    const bound = read<PreviewModel>(DEMO.model);
    expect(knowledge.records.length).toBeGreaterThan(0);
    const solverMessage = source.diagnostics.find((d) => d.severity === "warning" || d.severity === "blocking")!;
    expect(solverMessage).toBeDefined();
    const { rerender } = render(<KnowledgePanel knowledge={knowledge} model={bound} result={source} />);
    once("knowledge-solver-messages-heading");
    const heading = screen.getByTestId("knowledge-solver-messages-heading");
    // Every record the person keeps comes before it; the next element is the first solver-made record.
    for (const record of knowledge.records) {
      expect(screen.getByTestId(`knowledge-record-${record.id}`).compareDocumentPosition(heading) & Node.DOCUMENT_POSITION_FOLLOWING, record.id).toBeTruthy();
    }
    expect(heading.nextElementSibling?.getAttribute("data-testid")).toBe(`knowledge-record-knowledge:${solverMessage.id ?? solverMessage.code}`);
    rerender(<KnowledgePanel knowledge={knowledge} model={bound} result={{ ...source, diagnostics: [] }} />);
    expect(screen.queryByTestId("knowledge-solver-messages-heading")).toBeNull();
  });

  it("the Missing data panel: once at the head of its list where a listed warning is the result's own message", () => {
    const source = read<MechanicsResult>(DEMO.result), model = read<PreviewModel>(DEMO.model);
    const solverMessage = source.diagnostics.find((d) => d.code.startsWith("NONLINEAR_"))!;
    expect(solverMessage).toBeDefined();
    render(<MissingDataBlockingPanel model={model} result={source} />);
    once("missing-data-solver-messages-heading");
    const list = screen.getByTestId("missing-data-warnings");
    expect(list.firstElementChild).toBe(screen.getByTestId("missing-data-solver-messages-heading"));
    expect(list.querySelectorAll("[data-testid^='missing-data-warning-nonlinear']").length).toBeGreaterThan(0);
  });

  it("the run comparison: absent where the only listed difference is the desktop's own rule-pack warning", () => {
    const { result, model } = linear();
    const standing = numericalResultStanding(result, model);
    const side = (name: string, diagnostics: unknown[]) => ({
      kind: "recorded", id: `runrec:${name}`, description: name, createdAt: null, state: null, standing, recordedTableRead: null,
      input: { rows: [], standing, semantic_identity: { route: standing.contract, contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics }
    } as unknown as RunComparisonSide);
    const shown = (left: RunComparisonSide, right: RunComparisonSide): RunComparisonResult => ({ projectId: "project", left, right, comparison: compareRunInputs(left.input, right.input) });
    const desktopOwn = { code: RULE_PACK_REF_UNRESOLVED, severity: "warning", affected_refs: ["rule-pack:invented"], message: "The rule pack reference did not resolve." };
    const { rerender } = render(<RunComparisonView result={shown(side("left", []), side("right", [desktopOwn]))} onClose={() => {}} />);
    expect(screen.getByTestId("run-comparison-diagnostics").textContent).toContain(RULE_PACK_REF_UNRESOLVED);
    expect(screen.queryByTestId("run-comparison-solver-messages-heading")).toBeNull();
    const solver = result.diagnostics.map((d) => ({ code: d.code, severity: d.severity ?? null, affected_refs: d.affected_refs ?? [], message: d.message ?? null }))[0];
    rerender(<RunComparisonView result={shown(side("left", []), side("right", [desktopOwn, solver]))} onClose={() => {}} />);
    once("run-comparison-solver-messages-heading");
  });

  it("the Missing data panel: absent where a model warning shares its id with a result diagnostic that is not listed", () => {
    const source = read<MechanicsResult>(DEMO.result), bound = read<PreviewModel>(DEMO.model);
    const shared = "diagnostic:invented:shared";
    source.diagnostics = [...source.diagnostics.filter((d) => !d.code.startsWith("NONLINEAR_")), { id: shared, code: "INVENTED_RESULT_NOTE", severity: "info", message: "A result note.", affected_refs: [] } as never];
    const model = { ...bound, diagnostics: [...(bound.diagnostics ?? []), { id: shared, code: "NONLINEAR_INVENTED_MODEL_WARNING", severity: "warning", message: "A model warning.", affected_refs: [] }] } as PreviewModel;
    render(<MissingDataBlockingPanel model={model} result={source} />);
    expect(screen.getByTestId("missing-data-warnings").textContent).toContain("A model warning.");
    expect(screen.queryByTestId("missing-data-solver-messages-heading")).toBeNull();
  });

  it("the diagnostic detail: absent where the entry it shows is the model's, though a result diagnostic shares its id", () => {
    const { result, model: bound } = linear();
    const id = result.diagnostics[0].id ?? result.diagnostics[0].code;
    const own = { id, code: "MODEL_OWN_FINDING", severity: "warning", message: "A finding of the model's own.", affected_refs: [] };
    const model = { ...bound, diagnostics: [...(bound.diagnostics ?? []), own] } as PreviewModel;
    render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={id} onSelectDiagnostic={() => {}} />);
    expect(screen.getByTestId("diagnostic-detail-panel").textContent).toContain("MODEL_OWN_FINDING");
    expect(screen.queryByTestId("diagnostic-detail-solver-messages-heading")).toBeNull();
  });

  it("the Missing data panel: absent where no listed warning is the result's own message", () => {
    const source = read<MechanicsResult>(DEMO.result), model = read<PreviewModel>(DEMO.model);
    source.diagnostics = source.diagnostics.filter((d) => !d.code.startsWith("NONLINEAR_"));
    render(<MissingDataBlockingPanel model={model} result={source} />);
    expect(screen.getByTestId("missing-data-warnings")).toBeTruthy();
    expect(screen.queryByTestId("missing-data-solver-messages-heading")).toBeNull();
  });
});
