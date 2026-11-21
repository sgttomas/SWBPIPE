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
import { presentedResultDiagnostics } from "./suppliedDiagnostics";
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
  /\bwithin (?:its|their) stated (?:numerical )?bound\b/i, /\bstress-free to within\b/i, /\bbounded by\b/i, /integrity[ _]checked/i,
  /\bquality: Passed\b/
];
/** A code shown as a code (a machine field, as the decided words keep it), not a sentence. */
const isCode = (text: string) => /^[\sA-Z0-9_:.()=-]+$/.test(text.trim());
/** The note in a text, or the support-state line's own denial of the confirmation in the reviewed words. */
const NOTE_IN_TEXT = /reported by the solver; not verified by SWBPIPE/gi;
/** The end of the last claim in a text; -1 where it states none. */
function lastClaimEnd(text: string): number {
  let end = -1;
  for (const claim of CLAIMS) for (const match of text.matchAll(new RegExp(claim.source, claim.flags.includes("g") ? claim.flags : `${claim.flags}g`))) end = Math.max(end, match.index! + match[0].length);
  return end;
}
/** A node that is the note itself: an element whose whole text is the note. */
const isNoteElement = (node: Node | null): node is Element => !!node && node.nodeType === Node.ELEMENT_NODE && (node as Element).textContent?.trim() === SOLVER_REPORT_NOTE;
/** The next sibling node that is not blank text or closing punctuation (a parenthesis around a quoted message). */
function nextNode(node: Node): Node | null {
  let next = node.nextSibling;
  while (next && next.nodeType === Node.TEXT_NODE && /^[\s).,;:]*$/.test(next.textContent ?? "")) next = next.nextSibling;
  return next;
}
/** Every text that states a claim has its own note: in the same text after its last claim, or as the
 * node that immediately follows it and is the note itself (where the text is the last of its element,
 * the element's next sibling). A note serves one claim-bearing text only, so a note anywhere else in
 * the subtree, or one already used by the text before, satisfies nothing. A text that is the
 * support-state line's denial ("Support states: not confirmed. …") governs itself. Returns the claims
 * found, so a fixture shows it has some. */
function expectEveryClaimNoted(container: HTMLElement, what: string): string[] {
  const text = container.textContent ?? "";
  expect(text, `${what}: inverse-norm`).not.toMatch(/inverse-norm/i);
  expect(text, `${what}: the removed phrase`).not.toMatch(/solver checks passed, not verified/i);
  const found: string[] = [];
  const used = new Set<Element>();
  const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT);
  for (let node = walker.nextNode(); node; node = walker.nextNode()) {
    const own = node.textContent ?? "";
    if (!own.trim() || isCode(own)) continue;
    const end = lastClaimEnd(own);
    if (end < 0) continue;
    // The note's own text and the support-state line's denial are not claims to be noted.
    if (node.parentElement && isNoteElement(node.parentElement)) continue;
    if (own.trimStart().startsWith(`${SUPPORT_STATES_LEAD}.`)) continue;
    found.push(own);
    if ([...own.slice(end).matchAll(NOTE_IN_TEXT)].length > 0) continue;
    let following = nextNode(node);
    // The text ends its element: the element's own next sibling.
    if (!following && node.parentElement && node.parentElement !== container) following = nextNode(node.parentElement);
    const noted = isNoteElement(following) && !used.has(following);
    if (noted) used.add(following as Element);
    expect(noted, `${what}: its own note with "${own.slice(0, 120)}"`).toBe(true);
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

describe("a quoted structural report that passed (round 2)", () => {
  const CAP = () => read<{ document: MechanicsResult; request: { model: PreviewModel } }>("fixtures/results/pressure_v3_cap_remainder_halfway_force.json");
  const witnesses = () => {
    const cap = CAP();
    const operands = corpus("fixtures/results/preview_physics_nonlinear_operands_corpus.json", "v1_gap_closes");
    return [
      { name: "a sensitive case (case:P)", result: cap.document, model: cap.request.model, code: "NUMERICAL_INTEGRITY_SENSITIVE", id: "diagnostic:numerical-integrity:case:P" },
      { name: "an unqualified recovery basis (case:S)", result: operands.result, model: operands.model, code: "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", id: "diagnostic:numerical-integrity:case:S" }
    ];
  };

  it.each([0, 1])("witness %i: the note on the Issues list, the diagnostic detail and the Design Knowledge panel", (index) => {
    const { result, model: bound, code, id } = witnesses()[index];
    const model = { ...bound, diagnostics: bound.diagnostics ?? [] } as PreviewModel;
    const diagnostic = result.diagnostics.find((d) => d.id === id)!;
    expect(diagnostic.code).toBe(code);
    expect(diagnostic.message).toContain("quality: Passed");
    let selected: string | null = null;
    const { rerender } = render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={null} onSelectDiagnostic={(value) => { selected = value; }} />);
    const items = screen.getAllByTestId(`diagnostic-${code}`);
    const item = items.find((element) => element.textContent?.includes(diagnostic.message))!;
    expect(item.querySelector(`[data-testid='diagnostic-solver-report-note-${code}']`)).toHaveTextContent(SOLVER_REPORT_NOTE);
    fireEvent.click(item);
    expect(selected).toBe(id);
    rerender(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={selected} onSelectDiagnostic={() => {}} />);
    expectNoteWithExplanation();
    expect(expectEveryClaimNoted(document.body, "issues").some((t) => t.includes("quality: Passed"))).toBe(true);
    cleanup();
    render(<KnowledgePanel knowledge={null} result={result} />);
    // The panel shows the presented message (a result with combinations quotes it as supplied text).
    const presented = presentedResultDiagnostics(result).find((d) => d.id === id)!;
    expect(presented.message).toContain("quality: Passed");
    expect(document.body.textContent).toContain(`${presented.message} ${SOLVER_REPORT_NOTE}`);
    expect(expectEveryClaimNoted(document.body, "design knowledge").some((t) => t.includes("quality: Passed"))).toBe(true);
  });
});

describe("the surface walk's own guard", () => {
  const dom = (html: string) => { const host = document.createElement("div"); host.innerHTML = html; document.body.append(host); return host; };
  const LIMITATION = "Member actions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered.";

  it("accepts a note in the claim's own text after it, or as the next node that is the note itself", () => {
    expect(expectEveryClaimNoted(dom(`<p>${LIMITATION} ${SOLVER_REPORT_NOTE}</p>`), "own text")).toHaveLength(1);
    cleanup(); document.body.innerHTML = "";
    expect(expectEveryClaimNoted(dom(`<span class="lim">${LIMITATION} <span>${SOLVER_REPORT_NOTE} </span></span><span class="lim">${LIMITATION} <span>${SOLVER_REPORT_NOTE} </span></span>`), "two noted")).toHaveLength(2);
    document.body.innerHTML = "";
    expect(expectEveryClaimNoted(dom(`<button><p>quality: Passed</p><p>${SOLVER_REPORT_NOTE}</p></button>`), "sibling")).toHaveLength(1);
    document.body.innerHTML = "";
  });

  it("fails two adjacent limitations of which only the first is noted", () => {
    expect(() => expectEveryClaimNoted(dom(`<span class="lim">${LIMITATION} <span>${SOLVER_REPORT_NOTE} </span></span><span class="lim">${LIMITATION} </span>`), "control")).toThrow(/its own note/);
    document.body.innerHTML = "";
    // In one text: a marked limitation does not cover the next.
    expect(() => expectEveryClaimNoted(dom(`<p>${LIMITATION} ${SOLVER_REPORT_NOTE} ${LIMITATION}</p>`), "control")).toThrow(/its own note/);
    document.body.innerHTML = "";
  });

  it("fails a note in a distant descendant, or one before the claim", () => {
    expect(() => expectEveryClaimNoted(dom(`<div>${LIMITATION}<div><p>Other words. <span>${SOLVER_REPORT_NOTE}</span></p></div></div>`), "control")).toThrow(/its own note/);
    document.body.innerHTML = "";
    expect(() => expectEveryClaimNoted(dom(`<div><section><p>${LIMITATION}</p></section></div><p>${SOLVER_REPORT_NOTE}</p>`), "control")).toThrow(/its own note/);
    document.body.innerHTML = "";
    expect(() => expectEveryClaimNoted(dom(`<p>${SOLVER_REPORT_NOTE} ${LIMITATION}</p>`), "control")).toThrow(/its own note/);
    document.body.innerHTML = "";
  });
});

void (null as Json);
