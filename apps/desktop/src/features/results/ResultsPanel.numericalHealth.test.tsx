// DEL-07-05 numerical health §3, §4 and §7: the Results panel's health header, the
// output sentence, the per-case detail, row badges, located-cause navigation and the
// solver-diagnostic open. The indicator is a hand-written fixture of the Rust
// `numerical_health_json` output shape, presented through `numericalHealthView`.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ResultsPanel } from "./ResultsPanel";
import { numericalQualityStatusText, recoveredCaseIds } from "./integrityStatus";
import { numericalHealthView, parseNumericalHealth, type NumericalHealthView } from "./numericalHealth";
import { BOUNDARY_SENTENCE } from "./numericalHealthText";
import { resolveLocatedCauseSelection } from "./resultInterpretation";
import type { MechanicsResult, PreviewModel } from "../../types";
import unicodeSparse from "../../../../../fixtures/results/preview_physics_unicode_ids_sparse.json";

afterEach(cleanup);

function view(raw: unknown, options: Parameters<typeof numericalHealthView>[1]): NumericalHealthView {
  const health = parseNumericalHealth(raw);
  expect(health).not.toBeNull();
  return numericalHealthView(health, options)!;
}
const genericResult = () => structuredClone(unicodeSparse) as unknown as MechanicsResult;
const SENSITIVE = {
  case_id: "load:é", state: "sensitive", code: "NUMERICAL_INTEGRITY_SENSITIVE", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:é",
  rcond: 1e-11, digits: 11, conditioning_sensitive: true,
  causes: [
    { kind: "formation", diagnostic_id: "diagnostic:numerical-integrity:load:é", locations: [{ kind: "node_dof", node: "node:α", dof: "UX" }, { kind: "node_dof", node: "node:missing", dof: "RZ" }] },
    { kind: "curved_bend_short", diagnostic_id: "diagnostic:bend", locations: [{ kind: "ref", ref: "pipe:α-β" }, { kind: "text", text: "member end forces" }] }
  ]
};
const PASSED = { case_id: "load:𝔫", state: "checks_passed", code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:𝔫", rcond: 0.01, digits: 2, conditioning_sensitive: false, causes: [] };
const genericHealth = (verdictOnly: boolean) => ({
  route: "generic", verdict_only: verdictOnly, cases: [SENSITIVE, PASSED],
  combinations: [{ combination_id: "combination:Σ", state: "sensitive", cases: ["load:é", "load:𝔫"], withheld: false }]
});

describe("ResultsPanel numerical health", () => {
  it("shows the generic header with counts, the output sentence under the hold, and the per-case detail", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(true), { eligible: false, verdictHold: true })} />);
    expect(screen.getByTestId("numerical-health-header-line").textContent).toBe(
      "Numerical health: 1 sensitive, 1 no problem found. Rule checks, the result JSON and stress-neutral exports and the report package do not use this run while 1 load case(s) are sensitive, quality unresolved, solve failed or not assessed; results tables, CSV export and, for an exact-route result, the rendered report carry each case's standing."
    );
    expect(screen.getByTestId("numerical-health-header")).toHaveTextContent(/^Numerical health: 1 sensitive, 1 no problem found$/);
    expect(screen.getByTestId("numerical-health-header")).toHaveAttribute("data-badge", "red");
    expect(screen.getByTestId("numerical-health-case-text-load:é").textContent).toContain("Its values are kept for inspection.");
    expect(screen.getByTestId("numerical-health-case-conditioning-load:é").textContent).toContain("roughly 11.0 of the about 16 decimal digits");
    expect(screen.getByTestId("numerical-health-case-badge-load:𝔫")).toHaveTextContent("Numerical health: solver checks found no problem");
    expect(screen.getByTestId("numerical-health-combination-badge-combination:Σ")).toHaveTextContent("sensitive");
  });

  it("adds no output sentence without the verdict-only hold", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(false), { eligible: false, verdictHold: false })} />);
    expect(screen.getByTestId("numerical-health-header-line").textContent).toBe("Numerical health: 1 sensitive, 1 no problem found");
    expect(screen.queryByTestId("numerical-health-output-sentence")).toBeNull();
  });

  it("puts each generic row's load case or combination badge on the row, and none on a row whose case the solver's checks found no problem with", () => {
    const result = genericResult();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(true), { eligible: false, verdictHold: true })} />);
    const rows = result.results.slice(0, 50);
    for (const row of rows) {
      const badge = screen.queryByTestId(`result-row-health-${row.id}`);
      if (!row.basis_ref) { expect(badge).toBeNull(); continue; }
      if (row.basis_ref.ref_id === "load:𝔫") expect(badge).toBeNull();
      else expect(badge?.textContent).toBe("sensitive");
    }
  });

  it("gives a withheld combination's rows no badge", () => {
    const raw = genericHealth(true) as { combinations: { withheld: boolean; state: string | null; cases: string[] }[] };
    Object.assign(raw.combinations[0], { withheld: true, state: null, cases: [] });
    const result = genericResult();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={view(raw, { eligible: false, verdictHold: true })} />);
    expect(screen.queryByTestId("numerical-health-combination-badge-combination:Σ")).toBeNull();
    expect(screen.queryByTestId("numerical-health-combination-combination:Σ")).toBeNull();
    const combinationRow = result.results.slice(0, 50).find((row) => row.basis_ref?.ref_type === "combination");
    if (combinationRow) expect(screen.queryByTestId(`result-row-health-${combinationRow.id}`)).toBeNull();
  });

  it("lists located causes, navigates the resolvable ones and opens the solver diagnostic", () => {
    const model = { nodes: [{ id: "node:α" }], pipe_segments: [{ id: "pipe:α-β" }], supports: [], components: [], load_cases: [] } as unknown as PreviewModel;
    const onSelect = vi.fn(), onShow = vi.fn();
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(true), { eligible: false, verdictHold: true })}
      resolveCauseLocation={(location) => resolveLocatedCauseSelection(model, location)}
      onSelectCauseLocation={onSelect} onShowDiagnostic={onShow} />);
    const list = within(screen.getByTestId("numerical-health-case-locations-load:é"));
    expect(list.getByText("node node:α, UX").tagName).toBe("BUTTON");
    expect(list.getByText("node node:missing, RZ").tagName).toBe("SPAN");
    expect(list.getByText("pipe:α-β").tagName).toBe("BUTTON");
    expect(list.getByText("member end forces").tagName).toBe("SPAN");
    fireEvent.click(list.getByText("node node:α, UX"));
    expect(onSelect).toHaveBeenLastCalledWith({ type: "node", id: "node:α" });
    fireEvent.click(list.getByText("pipe:α-β"));
    expect(onSelect).toHaveBeenLastCalledWith({ type: "pipe", id: "pipe:α-β" });
    fireEvent.click(screen.getByTestId("numerical-health-show-diagnostic-load:é"));
    expect(onShow).toHaveBeenCalledWith("diagnostic:numerical-integrity:load:é");
  });

  it("offers no solver-diagnostic link without an integrity diagnostic", () => {
    const raw = genericHealth(false);
    (raw.cases[0] as { integrity_diagnostic_id: string | null }).integrity_diagnostic_id = null;
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(raw, { eligible: false, verdictHold: false })} onShowDiagnostic={() => {}} />);
    expect(screen.queryByTestId("numerical-health-show-diagnostic-load:é")).toBeNull();
    expect(screen.getByTestId("numerical-health-show-diagnostic-load:𝔫")).toHaveTextContent("Show solver diagnostic");
  });

  it("prefixes the pressure-source header, keeps the recovery text, adds the eligible wording and places no row badges", () => {
    const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"), "utf8")) as MechanicsResult;
    const health = view({ route: "pressure_source", verdict_only: false, cases: [{ ...SENSITIVE, case_id: "case", integrity_diagnostic_id: "diagnostic:numerical-integrity:case", causes: [] }], combinations: [] },
      { eligible: true, verdictHold: false, recoveredCases: recoveredCaseIds(result) });
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={health} />);
    expect(screen.getByTestId("numerical-health-header").textContent).toBe("Ordinary solve — Numerical health: 1 sensitive");
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Ordinary solve: sensitive\. Producer recovery receipt: qualified\. /);
    expect(screen.queryByTestId("numerical-health-output-sentence")).toBeNull();
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).toContain(
      "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
    );
    expect(screen.getByTestId("numerical-health-case-text-case").textContent).not.toContain("Its values are kept for inspection.");
    expect(screen.queryAllByTestId(/^result-row-health-/)).toHaveLength(0);
    expect(screen.getByTestId("physics-source-evidence")).toBeTruthy();
  });

  it("reads not assessed, never the raw status, when the indicator is absent or its route disagrees", () => {
    // The fixture's own status is checks_passed; with no gated health the line fails closed.
    expect(genericResult().numerical_quality!.status).toBe("checks_passed");
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: not assessed\. /);
    expect(screen.queryByTestId("numerical-health-detail")).toBeNull();
    cleanup();
    const pressureSource = view({ route: "pressure_source", verdict_only: false, cases: [PASSED], combinations: [] }, { eligible: true, verdictHold: false });
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={pressureSource} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: not assessed\. /);
  });

  it("reads a solved result whose gated health agrees with its status exactly as before", () => {
    const passed = view({ route: "generic", verdict_only: false, cases: [PASSED], combinations: [] }, { eligible: true, verdictHold: false });
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={passed} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).toMatch(/^Numerical health: solver checks found no problem\. /);
  });
});

describe("the indicator never reads better than the document's own status", () => {
  const passedOnly = { route: "generic", verdict_only: false, cases: [PASSED], combinations: [] };
  it("a status worse than every case: the header, its short form and its badge state the status; each case keeps its own", () => {
    const shown = view(passedOnly, { eligible: false, verdictHold: false, documentStatus: "sensitive" } as never);
    expect(shown.headline).toBe("Numerical health: sensitive");
    expect(shown.shortHeadline).toBe("Numerical health: sensitive");
    expect(shown.badge).toBe("red");
    expect(shown.cases[0].headline).toBe("Numerical health: solver checks found no problem");
    const pressureSource = view({ ...passedOnly, route: "pressure_source" }, { eligible: true, verdictHold: false, documentStatus: "failed" } as never);
    expect(pressureSource.headline).toBe("Ordinary solve — Numerical health: solve failed");
    // A token the indicator has no state for counts as not assessed: worse than checks passed.
    expect(view(passedOnly, { eligible: false, verdictHold: false, documentStatus: "invented" } as never).headline).toBe("Numerical health: not assessed");
  });
  it("a status no worse than the cases changes nothing", () => {
    expect(view(passedOnly, { eligible: false, verdictHold: false, documentStatus: "checks_passed" } as never).headline).toBe("Numerical health: solver checks found no problem");
    const mixed = view(genericHealth(false), { eligible: false, verdictHold: false, documentStatus: "checks_passed" } as never);
    expect(mixed.headline).toBe("Numerical health: 1 sensitive, 1 no problem found");
    expect(mixed.shortHeadline).toBe("Numerical health: sensitive");
  });
});

describe("the boundary sentence where no indicator is shown", () => {
  it("is carried once by the line that stands in the indicator's place, and once by the indicator's detail where it is shown", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null} />);
    expect(screen.queryByTestId("numerical-health-detail")).toBeNull();
    expect(screen.getAllByTestId("numerical-health-boundary").map((n) => n.textContent)).toEqual([BOUNDARY_SENTENCE]);
    expect(document.body.textContent!.split(BOUNDARY_SENTENCE).length - 1).toBe(1);
    cleanup();
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(false), { eligible: false, verdictHold: false })} />);
    expect(screen.getAllByTestId("numerical-health-boundary")).toHaveLength(1);
    expect(document.body.textContent!.split(BOUNDARY_SENTENCE).length - 1).toBe(1);
  });
});

describe("the standing line beside the health header", () => {
  // Native-smoke expected.json pins this line's prefix (owner-held G10 observation): its text is
  // today's wherever the gated health agrees with the status, and the header is a separate element
  // above it. Its state words are the worse of the document's status and the gated health's: the
  // generic fixture's status says checks_passed while its hand-written health has a sensitive case,
  // so the line says sensitive; with no health the gated side is not assessed, so the generic line
  // says not assessed and the pressure-source line, whose status is sensitive, says sensitive.
  const original = (result: MechanicsResult, state: string) => sourceContractText(result, state);
  function sourceContractText(result: MechanicsResult, state: string): string {
    return result.source_block_recovery
      ? `Ordinary solve: ${state}. Producer recovery receipt: ${(result.source_block_recovery as { body: { status: string } }).body.status}. ${result.formulation_basis!.limitations.join(" ")} Current use is checked separately against the actual invocation and complete result evidence. Numerical checks do not establish engineering correctness.`
      : `Numerical health: ${state}. ${result.formulation_basis!.limitations.join(" ")} Numerical checks do not establish engineering correctness.`;
  }
  it.each([
    ["generic", () => genericResult(), () => view(genericHealth(true), { eligible: false, verdictHold: true })],
    ["pressure-source", () => JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"), "utf8")) as MechanicsResult,
      () => view({ route: "pressure_source", verdict_only: false, cases: [{ ...SENSITIVE, case_id: "case", causes: [] }], combinations: [] }, { eligible: true, verdictHold: false })]
  ] as const)("keeps the %s standing text, in the gated state's words, with the header above it", (name, makeResult, makeHealth) => {
    const result = makeResult();
    expect(numericalQualityStatusText(result.numerical_quality!.status)).toBe(name === "generic" ? "solver checks found no problem" : "sensitive");
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={makeHealth()} />);
    const standing = screen.getByTestId("numerical-result-standing");
    expect(standing.textContent).toBe(original(result, "sensitive"));
    const header = screen.getByTestId("numerical-health-header-line");
    expect(standing).not.toContainElement(screen.getByTestId("numerical-health-header"));
    // Header, then the per-case detail (its causes stay above the long standing text), then the unchanged standing line.
    expect(header.nextElementSibling).toBe(screen.getByTestId("numerical-health-detail"));
    expect(screen.getByTestId("numerical-health-detail").nextElementSibling).toBe(standing);
    cleanup();
    // And the same text with no indicator at all.
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).toBe(original(result, name === "generic" ? "not assessed" : "sensitive"));
    expect(screen.queryByTestId("numerical-health-header-line")).toBeNull();
    expect(screen.queryByTestId("numerical-health-detail")).toBeNull();
  });
});

describe("the per-case detail disclosure", () => {
  it("is summarised as Numerical health by load case, open with a non-passing case and collapsed when all pass", () => {
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view(genericHealth(false), { eligible: false, verdictHold: false })} />);
    const open = screen.getByTestId("numerical-health-detail");
    expect(open.tagName).toBe("DETAILS");
    expect(open.querySelector("summary")?.textContent).toBe("Numerical health by load case");
    expect(open).toHaveAttribute("open");
    cleanup();
    render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
      numericalHealth={view({ route: "generic", verdict_only: false, cases: [PASSED, { ...PASSED, case_id: "load:é", integrity_diagnostic_id: "diagnostic:numerical-integrity:load:é" }], combinations: [] }, { eligible: true, verdictHold: false })} />);
    expect(screen.getByTestId("numerical-health-detail")).not.toHaveAttribute("open");
  });
});

describe("the boundary sentence", () => {
  const BOUNDARY = "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right.";
  const count = () => (screen.getByTestId("numerical-health-detail").textContent ?? "").split(BOUNDARY).length - 1;
  it("is carried once by every detail, whatever its cases' states (REQ-023, AC-008)", () => {
    const notAssessed = { case_id: "load:é", state: "not_assessed", code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] };
    const failed = { ...notAssessed, case_id: "load:𝔫", state: "solve_failed" };
    for (const [label, cases] of [
      ["a blocked document, every case suppressed or unassessed", [notAssessed]],
      ["a blocked document with a preserved failure", [notAssessed, failed]],
      ["a passing and a sensitive case", [PASSED, SENSITIVE]],
      ["passing only", [PASSED]]
    ] as const) {
      cleanup();
      render(<ResultsPanel result={genericResult()} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}}
        numericalHealth={view({ route: "generic", verdict_only: false, cases, combinations: [] }, { eligible: false, verdictHold: false })} />);
      expect(count(), label).toBe(1);
      expect(screen.getByTestId("numerical-health-boundary").textContent).toBe(BOUNDARY);
    }
  });
});

describe("resolveLocatedCauseSelection", () => {
  const model = { nodes: [{ id: "N1" }], pipe_segments: [{ id: "P1" }], supports: [{ id: "S1" }], components: [], load_cases: [{ id: "L1" }] } as unknown as PreviewModel;
  it("maps a row label to its node and keeps the direct-id path for references", () => {
    expect(resolveLocatedCauseSelection(model, { kind: "node_dof", node: "N1", dof: "UX" })).toEqual({ type: "node", id: "N1" });
    expect(resolveLocatedCauseSelection(model, { kind: "node_dof", node: "P1", dof: "UX" })).toBeNull();
    expect(resolveLocatedCauseSelection(model, { kind: "ref", ref: "S1" })).toEqual({ type: "support", id: "S1" });
    expect(resolveLocatedCauseSelection(model, { kind: "ref", ref: "L1" })).toEqual({ type: "load", id: "L1" });
    expect(resolveLocatedCauseSelection(model, { kind: "ref", ref: "unknown" })).toBeNull();
    expect(resolveLocatedCauseSelection(model, { kind: "text" })).toBeNull();
  });
});
