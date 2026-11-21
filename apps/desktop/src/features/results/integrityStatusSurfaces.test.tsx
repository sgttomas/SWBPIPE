// Release exit criterion 18 (LIM-026): no desktop surface a person reads says, of the ordinary solve,
// that its checks passed or that a state or row is established, confirmed, verified or bounded, unless
// "Reported by the solver; not verified by SWBPIPE." stands with it: in the same element or its
// immediate sibling. And the note is not put on what another certificate establishes (the
// pressure-source recovery's sentence). One walk over the rendered text of each surface, for each
// fixture round 1 of review named: a linear passing result, the invented demo with confirmed nonlinear
// supports, a mixed pressure-source run, a combination certificate, a stress-free case, the bundled
// reference, a historical saved run, a case whose confirmation quotes its reason, the Design Knowledge
// panel, a held-status comparison and the run comparison. The results are read through the actual
// wasm reader.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { ResultsPanel } from "./ResultsPanel";
import { numericalHealthView, readNumericalHealth } from "./numericalHealth";
import { numericalResultStanding, reportStanding } from "./resultStanding";
import { recoveredCaseIds, SOLVER_REPORT_NOTE, SUPPORT_STATES_LEAD, supportStatesByCase } from "./integrityStatus";
import { ELIGIBLE_RECOVERY_TEXT } from "./numericalHealthText";
import { PhysicsSourceEvidencePanel } from "./PhysicsSourceEvidencePanel";
import { KnownSemanticNotices } from "./KnownSemanticNotices";
import { buildBundledReferenceContext, buildHistoricalRunContext, HistoricalRunPanel } from "./HistoricalRunContext";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { KnowledgePanel } from "../knowledge/KnowledgePanel";
import { RunComparisonView } from "../comparison/RunComparisonView";
import { compareRunInputs } from "../comparison/runComparison";
import type { RunComparisonSide } from "../comparison/runComparisonSides";
import type { RunComparisonResult } from "../history/useModelStateHistory";
import type { BundledMechanicsReference } from "../../services/previewService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";

afterEach(cleanup);

const root = resolve(__dirname, "../../../../..");
const read = <T,>(path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as T;
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const corpus = (path: string, name: string) => {
  const entry = read<{ documents: { name: string; document: MechanicsResult; model: PreviewModel }[] }>(path).documents.find((d) => d.name === name)!;
  return { result: structuredClone(entry.document), model: structuredClone(entry.model) };
};
const LINEAR = { result: "fixtures/results/preview_physics_connected_sparse.json", model: "fixtures/model_operations/precision_connected_ui_model.json" };
const DEMO = { result: "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json", model: "fixtures/product_preview/invented_demo_model.json" };

/** What a person could read as a pass, a bound or an established state, found independently of the
 * predicate's own phrases. A denial directly before the word ("not", "no") is not a claim. */
const CLAIMS = [
  /checks[ _]passed/i, /(?<!(?:not|no) )\bestablished\b/i, /(?<!(?:not|no) )\bconfirmed\b/i, /(?<!(?:not|no) )\bverified\b/i,
  /\bwithin (?:its|their) stated (?:numerical )?bound\b/i, /\bstress-free to within\b/i, /\bbounded by\b/i, /integrity[ _]checked/i
];
/** An element's own text, without its child elements'. */
const ownText = (element: Element) => [...element.childNodes].filter((node) => node.nodeType === node.TEXT_NODE).map((node) => node.textContent).join("");
/** A code shown as a code (a machine field, as the decided words keep it), not a sentence. */
const isCode = (text: string) => /^[\sA-Z0-9_:.()=-]+$/.test(text.trim());
/** The note, or a denial that is the statement itself: the support-state line, which begins "Support
 * states: not confirmed" (its outcome statements deny each confirmation in the reviewed words). */
const governs = (text: string | null | undefined) => !!text && (text.includes(SOLVER_REPORT_NOTE) || /reported by the solver; not verified by SWBPIPE/i.test(text) || text.startsWith(`${SUPPORT_STATES_LEAD}.`));
/** Every element whose own text states a claim has the note in it or in its immediate sibling. Returns
 * the claims found, so a fixture shows it has some. */
function expectEveryClaimNoted(container: HTMLElement, what: string): string[] {
  const text = container.textContent ?? "";
  expect(text, `${what}: inverse-norm`).not.toMatch(/inverse-norm/i);
  expect(text, `${what}: the removed phrase`).not.toMatch(/solver checks passed, not verified/i);
  const found: string[] = [];
  for (const element of container.querySelectorAll("*")) {
    const own = ownText(element);
    if (!own.trim() || isCode(own) || !CLAIMS.some((claim) => claim.test(own))) continue;
    found.push(own);
    const noted = governs(element.textContent) || governs(element.previousElementSibling?.textContent) || governs(element.nextElementSibling?.textContent);
    expect(noted, `${what}: the note with "${own.slice(0, 120)}"`).toBe(true);
  }
  return found;
}

function healthFor(result: MechanicsResult, model: PreviewModel | null, eligibleOverride?: boolean) {
  const standing = numericalResultStanding(result, model ?? undefined);
  const reading = model ? reportStanding(result, model) : null;
  const supportStates = reading && reading.readability.readable && reading.readability.solved ? supportStatesByCase(reading.numerical_standing) : undefined;
  return numericalHealthView(readNumericalHealth(result, standing.findings), { eligible: eligibleOverride ?? standing.eligible, verdictHold: false, supportStates, recoveredCases: recoveredCaseIds(result) })!;
}
const panel = (result: MechanicsResult, health: ReturnType<typeof healthFor> | undefined) =>
  render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);

describe.each([
  { name: "linear passing", ...LINEAR, nonlinear: false },
  { name: "nonlinear confirmed", ...DEMO, nonlinear: true }
])("no unannotated passed or established claim ($name)", ({ result: resultPath, model: modelPath, nonlinear }) => {
  const result = () => read<MechanicsResult>(resultPath);
  const model = () => read<PreviewModel>(modelPath);

  it("the results panel: results line, health header and detail", () => {
    const source = result(), bound = model();
    const standing = numericalResultStanding(source, bound);
    expect(standing.status).toBe("solver_checks_passed");
    expect(standing.integrity_bound).toEqual({ status: "not_verified", reason: "inverse_norm_bound_not_verified" });
    const health = healthFor(source, bound);
    expect(health.cases.every((c) => c.state === "checks_passed")).toBe(true);
    panel(source, health);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: not verified\./);
    for (const c of health.cases) {
      const line = screen.queryByTestId(`numerical-health-case-support-states-${c.caseId}`);
      if (nonlinear) expect(line?.textContent).toMatch(/^Support states: not confirmed\. Not confirmed: /);
    }
    const found = expectEveryClaimNoted(document.body, "results panel");
    // The limitation that states the checks-passed code has the note directly after it.
    const notes = screen.getAllByTestId("result-standing-solver-report-note");
    for (const note of notes) expect(note.parentElement!.className).toBe("result-standing-limitation");
    expect(found.length).toBeGreaterThan(0);
  });

  it("the issues list and the diagnostic detail, the note with the explanation", () => {
    const source = result(), bound = model();
    const integrity = source.diagnostics.filter((d) => d.code === "NUMERICAL_INTEGRITY_CHECKS_PASSED");
    let selected: string | null = null;
    const { rerender } = render(<DiagnosticsPanel model={bound} knowledge={null} result={source} selectedDiagnosticId={null} onSelectDiagnostic={(id) => { selected = id; }} />);
    expect(screen.getAllByTestId("diagnostic-solver-report-note-NUMERICAL_INTEGRITY_CHECKS_PASSED")).toHaveLength(integrity.length);
    fireEvent.click(screen.getAllByTestId("diagnostic-NUMERICAL_INTEGRITY_CHECKS_PASSED")[0]);
    rerender(<DiagnosticsPanel model={bound} knowledge={null} result={source} selectedDiagnosticId={selected} onSelectDiagnostic={() => {}} />);
    expect(screen.getByTestId("selected-diagnostic-explanation").querySelector("dd")!.textContent).toContain(SOLVER_REPORT_NOTE);
    expectNoteWithExplanation();
    const found = expectEveryClaimNoted(document.body, "issues");
    // The linear case's messages state their claim by code only; the nonlinear one's also in words.
    if (nonlinear) expect(found.length).toBeGreaterThan(0);
  });

  it("the run comparison: the standing in its decided words, and the solver's text noted", () => {
    const source = result(), bound = model();
    const standing = numericalResultStanding(source, bound);
    const side = (name: string, diagnostics: MechanicsResult["diagnostics"]): RunComparisonSide => ({
      kind: "recorded", id: `runrec:${name}`, description: name, createdAt: null, state: null, standing, recordedTableRead: null,
      input: { rows: [], standing, semantic_identity: { route: standing.contract, contract_id: "c", contract_sha256: "s" }, context: {}, diagnostics }
    } as unknown as RunComparisonSide);
    const diagnostics = source.diagnostics.map((d) => ({ code: d.code, severity: d.severity ?? null, affected_refs: d.affected_refs ?? [], message: d.message ?? null }));
    const left = side("left", []), right = side("right", diagnostics as unknown as MechanicsResult["diagnostics"]);
    const shown: RunComparisonResult = { projectId: "project", left, right, comparison: compareRunInputs(left.input, right.input) };
    render(<RunComparisonView result={shown} onClose={() => {}} />);
    expect(screen.getByTestId("run-comparison-side-left").textContent).toBe(`Left: left. Its result is numerically eligible (route ${standing.contract}); the ordinary solve's error bound is not verified.`);
    expect(screen.getByTestId("run-comparison-basis")).toHaveTextContent("It is computed only when both runs' results are numerically eligible, both carry the same result semantic identity, and the two units are equal; nothing is converted. Numerically eligible is an admission standing: it asserts no verified error bound.");
    // The comparison shortens a long message, so the claim's words may be cut; the note follows the code.
    expect(screen.getAllByTestId("run-comparison-diagnostic-solver-report-note").length).toBe(source.diagnostics.filter((d) => d.code === "NUMERICAL_INTEGRITY_CHECKS_PASSED").length);
    expectEveryClaimNoted(document.body, "run comparison");
  });
});

/** The detail's note is inside the Explanation line, not a line of its own. */
function expectNoteWithExplanation() {
  const note = screen.getByTestId("selected-diagnostic-solver-report-note");
  expect(note.closest("[data-testid='selected-diagnostic-explanation']")).not.toBeNull();
}

describe("the surfaces round 1 found", () => {
  it("a mixed pressure-source run: the recovery's sentence on the recovered case only, the ordinary attempt in the desktop's words", () => {
    const source = read<MechanicsResult>("fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json");
    // The recovery's sentence follows an eligible standing (a registered invocation on the live path).
    const health = healthFor(source, null, true);
    expect(health.route).toBe("pressure_source");
    panel(source, health);
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).toContain(ELIGIBLE_RECOVERY_TEXT);
    expect(screen.getByTestId("numerical-health-case-text-case:ordinary-pressure").textContent).not.toContain(ELIGIBLE_RECOVERY_TEXT);
    // Another certificate's sentence takes no note.
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).not.toContain(SOLVER_REPORT_NOTE);
    cleanup();
    render(<PhysicsSourceEvidencePanel result={source} />);
    expect(screen.getByTestId("physics-source-case-case:ordinary-pressure").textContent).toContain("Ordinary case attempt: not verified.");
    expectEveryClaimNoted(document.body, "physics-source evidence");
  });

  it("a combination certificate's diagnostic", () => {
    const { result, model: corpusModel } = corpus("fixtures/results/combination_certificate_corpus.json", "pf_cantilever");
    const model = { ...corpusModel, diagnostics: corpusModel.diagnostics ?? [] } as PreviewModel;
    expect(result.diagnostics.some((d) => d.code === "COMBINATION_ROWS_WITHIN_STATED_BOUND")).toBe(true);
    render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    expect(screen.getAllByTestId("diagnostic-solver-report-note-COMBINATION_ROWS_WITHIN_STATED_BOUND").length).toBeGreaterThan(0);
    expect(expectEveryClaimNoted(document.body, "combination certificate").some((t) => /within their stated bound/.test(t))).toBe(true);
  });

  it("a stress-free case's diagnostic", () => {
    const source = read<MechanicsResult>(LINEAR.result), bound = read<PreviewModel>(LINEAR.model);
    // The solver's sentence (formation_guard.rs `stress_free_notice`); no committed result publishes one.
    source.diagnostics.push({ id: "diagnostic:stress-free-within-numerical-precision:case", code: "STRESS_FREE_WITHIN_NUMERICAL_PRECISION", severity: "info", affected_refs: ["case"],
      message: "Load case case: every element-local member force row and support force row is bounded by 1.2e-12 N and every element-local member moment row and support moment row by 3.4e-12 N*m at 128-bit precision; the case is stress-free to within numerical precision. The standing remains NUMERICAL_INTEGRITY_SENSITIVE." } as never);
    render(<DiagnosticsPanel model={bound} knowledge={null} result={source} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    expect(screen.getByTestId("diagnostic-solver-report-note-STRESS_FREE_WITHIN_NUMERICAL_PRECISION")).toHaveTextContent(SOLVER_REPORT_NOTE);
    expect(expectEveryClaimNoted(document.body, "stress-free").some((t) => /stress-free to within/.test(t))).toBe(true);
  });

  it("the bundled reference with nonlinear supports shows the support-state line", () => {
    const reference = { standing: "reference_only", model: read<PreviewModel>(DEMO.model), source: read<MechanicsResult>(DEMO.result), provenance: { notice: "Bundled reference." } } as unknown as BundledMechanicsReference;
    render(<HistoricalRunPanel context={buildBundledReferenceContext(reference)} />);
    expect(screen.getAllByText(/^Support states: not confirmed\. Not confirmed: /).length).toBeGreaterThan(0);
    expectEveryClaimNoted(document.body, "bundled reference");
  });

  it("a historical saved run with nonlinear supports shows the support-state line", async () => {
    const model = read<PreviewModel>(DEMO.model), mechanics_result = read<MechanicsResult>(DEMO.result);
    const model_hash = await computeModelHash(model);
    const payload = { model, mechanics_result, analysis_run: null, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
    const envelope = { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never) } as unknown as LocalProjectEnvelope;
    const context = (await buildHistoricalRunContext(envelope))!;
    render(<HistoricalRunPanel context={context} />);
    await waitFor(() => expect(screen.getAllByText(/^Support states: not confirmed\. Not confirmed: /).length).toBeGreaterThan(0));
    expectEveryClaimNoted(document.body, "historical saved run");
  });

  it("a confirmation that quotes its reason shows the reason directly beneath", () => {
    const { result, model } = corpus("fixtures/results/preview_physics_held_status_corpus.json", "h15c_zero_limit");
    const health = healthFor(result, model);
    const quoted = health.cases.find((c) => c.supportStates?.text.includes("quoted below"));
    expect(quoted, "a not_established_detail_quoted case").toBeDefined();
    panel(result, health);
    const statement = screen.getByTestId(`numerical-health-case-support-states-${quoted!.caseId}`);
    const detail = statement.nextElementSibling!;
    expect(detail.getAttribute("data-testid")).toBe(`numerical-health-case-support-states-detail-${quoted!.caseId}`);
    expect(detail.textContent).toContain(quoted!.supportStates!.detail!);
    expect(quoted!.supportStates!.detail!.length).toBeGreaterThan(0);
    expectEveryClaimNoted(document.body, "quoted reason");
  });

  it("the Design Knowledge panel copies the solver's warning with the note", () => {
    const source = read<{ envelope: MechanicsResult }>("fixtures/results/rt_a3_s6_c1_correction_unavailable.json").envelope;
    expect(source.diagnostics.some((d) => d.severity === "warning" && d.message.includes("checks passed"))).toBe(true);
    render(<KnowledgePanel knowledge={null} result={source} />);
    expect(expectEveryClaimNoted(document.body, "design knowledge").some((t) => /checks passed/.test(t))).toBe(true);
  });

  it("a held-status comparison's established outcome, in the known-limitation notices", () => {
    const { result } = corpus("fixtures/results/preview_physics_held_status_corpus.json", "h4_kept_explicit");
    render(<KnownSemanticNotices result={result} testIdPrefix="notices" />);
    expect(expectEveryClaimNoted(document.body, "held-status notice").some((t) => /Established within its stated numerical bound/.test(t))).toBe(true);
  });
});

void (null as Json);
