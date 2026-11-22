// The Issues drawer's items, second review round: excerpts only where they are the producer's
// text and no report takes the solver report note (R1); every opened row is its own report (R2);
// the grouping key (R3); a requested diagnostic shown once (R4); a class decided after every
// report is read (R5); counts at each item's severity, and missing data apart (R9). Invented
// models and diagnostics only.
import { fireEvent, render, screen, within } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import type { DesignKnowledge, Diagnostic, MechanicsResult, PreviewModel } from "../../types";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { countItemsBySeverity, diagnosticsListOf, groupIssues, issueSummary, type IssueItem } from "./issueItems";
import { IssueItemView, LONG_MESSAGE_CHARACTERS, NO_OWN_DETAIL_REASON } from "./IssueItemView";
import { IssuesHome } from "../../App";
import { loadPreviewModel } from "../../services/previewService";
import type { ModelCheckFinding } from "./modelChecks";
import { ModelChecksPanel } from "./ModelChecksPanel";
import { SOLVER_MESSAGES_HEADING } from "../results/integrityStatus";
import missingDataStartModel from "../../../../../fixtures/embedded_agent/use_cases/missing-data/start_model.json";

const model = {
  project: { id: "project:invented-round-2" },
  nodes: [], pipe_segments: [], components: [],
  load_cases: [{ id: "load:LC1", label: "Invented operating" }, { id: "load:LC2", label: "Invented sustained" }],
  materials: [{ id: "material:M1", label: "Invented steel" }],
  supports: [{ id: "support:H-1" }],
  diagnostics: []
} as unknown as PreviewModel;
const report = (overrides: Partial<Diagnostic> & Pick<Diagnostic, "code">): Diagnostic => ({
  severity: "warning", message: `invented ${overrides.code} text on support:H-1`, affected_refs: ["support:H-1"], ...overrides
});
const checked = (diagnostic: Diagnostic, entity: ModelCheckFinding["entity"] = { type: "support", id: "support:H-1" }): ModelCheckFinding =>
  ({ key: diagnostic.id ?? diagnostic.code, group: "solve", diagnostic, entity });
const run = (findings: ModelCheckFinding[]) => ({
  model, solve: { kind: "checked" as const }, findings,
  notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 }
});
const renderList = (items: IssueItem[], onSelect = vi.fn()) =>
  render(<DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={onSelect} items={items} />);
function openReports(group: HTMLElement): HTMLElement {
  const disclosure = within(group).getByTestId("issue-group-members");
  expect(disclosure.hidden).toBe(true);
  fireEvent.click(within(group).getByTestId("issue-group-members-toggle"));
  expect(disclosure.hidden).toBe(false);
  return disclosure;
}

describe("R1: an opened group's rows mark what tells them apart, only where it is the producer's own text", () => {
  it("the hanger's six reports (three loads from the model checks and the solve): each opened row marks its load within its message; the collapsed item adds no excerpt line", () => {
    const code = "INVENTED_HANGER_LOAD_NOT_PRELOAD";
    const load = (name: string, id: string) => report({ id, code, message: `invented hanger support:H-1: its ${name} load (hanger.${name}_load) is not applied as preload` });
    const loads = ["installed", "cold", "hot"];
    const items = groupIssues({
      model,
      modelChecks: loads.map((name) => checked(load(name, `check-${name}`))),
      diagnostics: loads.map((name) => load(name, `solve-${name}`)),
      diagnosticOrigins: ["solve", "solve", "solve"]
    });
    expect(items).toHaveLength(1);
    render(<ModelChecksPanel model={model} run={run(loads.map((name) => checked(load(name, `check-${name}`))))} items={items} onRun={() => {}} onSelectEntity={() => {}} onSelectDiagnostic={() => {}} />);
    const group = screen.getByTestId(`issue-group-${code}`);
    expect(within(group).queryByTestId("issue-group-also")).toBeNull();
    const rows = within(openReports(group)).getAllByTestId(`issue-member-${code}`);
    expect(rows).toHaveLength(6);
    // Each opened row: its first line is the severity's mark and its message, marking what tells it apart.
    for (const row of rows) expect(row.firstElementChild).toHaveClass("issue-first-line");
    const messages = rows.map((row) => row.querySelector(".issue-message") as HTMLElement);
    expect(messages.map((message) => within(message).getByTestId("issue-member-difference").textContent)).toEqual([
      "installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)",
      "installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)"
    ]);
    // The excerpt is cut from the producer's message, which is shown whole.
    expect(messages[1]).toHaveTextContent("invented hanger support:H-1: its cold load (hanger.cold_load) is not applied as preload");
  });

  it("the same six where the solve's reports are shown as supplied, unverified text and one message is long: each row still marks only its load", () => {
    const code = "INVENTED_HANGER_LOAD_NOT_PRELOAD";
    const rest = "; results treat the hanger as a linear spring from zero load, whose force is its stiffness times the displacement at its degree of freedom, and the entered load remains review evidence only";
    const load = (name: string, id: string) => report({ id, code, message: `invented variable spring hanger support:H-1: its ${name} load (hanger.${name}_load) is not applied as preload${rest}` });
    const loads = ["installed", "cold", "hot"];
    const solve = loads.map((name) => load(name, `solve-${name}`));
    const wrapped = solve.map((diagnostic) => ({ ...diagnostic, message: `Supplied diagnostic text, unverified (not a combination certificate statement): "${diagnostic.message}"` }));
    expect(load("installed", "x").message.length).toBeGreaterThan(LONG_MESSAGE_CHARACTERS);
    const checks = loads.map((name) => checked(load(name, `check-${name}`)));
    const items = groupIssues({ model, modelChecks: checks, diagnostics: solve, shownDiagnostics: wrapped, diagnosticOrigins: ["solve", "solve", "solve"] });
    expect(items).toHaveLength(1);
    expect(items[0].differences).toEqual(["installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)"]);
    render(<ModelChecksPanel model={model} run={run(checks)} items={items} onRun={() => {}} onSelectEntity={() => {}} onSelectDiagnostic={() => {}} />);
    const rows = within(openReports(screen.getByTestId(`issue-group-${code}`))).getAllByTestId(`issue-member-${code}`);
    expect(rows).toHaveLength(6);
    // Every row, the long one and the three shown as supplied text included, marks its load and nothing more.
    expect(rows.map((row) => within(row.querySelector(".issue-message") as HTMLElement).getByTestId("issue-member-difference").textContent)).toEqual([
      "installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)",
      "installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)"
    ]);
    expect(rows[3].querySelector(".issue-message")).toHaveTextContent(/^Supplied diagnostic text, unverified/);
  });

  it("an item whose only source of one distinct message is supplied, unverified text shows no excerpt", () => {
    const code = "INVENTED_SUPPLIED";
    const own = report({ id: "own", code, message: "invented first load is not applied" });
    const supplied = report({ id: "sup", code, message: "invented second load is not applied" });
    const items = groupIssues({
      model,
      diagnostics: [own, supplied],
      shownDiagnostics: [own, { ...supplied, message: `Supplied diagnostic text, unverified: "${supplied.message}"` }],
      diagnosticOrigins: ["model", "solve"]
    });
    expect(items[0].differences).toEqual([]);
    renderList(items);
    const group = screen.getByTestId(`issue-group-${code}`);
    expect(within(group).queryByTestId("issue-group-also")).toBeNull();
    openReports(group);
    expect(within(group).queryAllByTestId("issue-member-difference")).toHaveLength(0);
    expect(within(group).getByText(/^Supplied diagnostic text, unverified/)).toBeInTheDocument();
  });
});

describe("R2: every opened row is that report", () => {
  it("two reports alike but for their load case read differently, and each opens its own detail or says why it cannot", () => {
    const code = "INVENTED_CASE_LOAD";
    const forCase = (lc: string, id?: string) => report({ ...(id ? { id } : {}), code, message: "invented load not applied on support:H-1", affected_refs: ["support:H-1", lc] });
    // Without identifiers: each row names its load case and offers no detail, saying why on the row.
    renderList(groupIssues({ model, diagnostics: [forCase("load:LC1"), forCase("load:LC2")] }));
    let group = screen.getByTestId(`issue-group-${code}`);
    let rows = within(openReports(group)).getAllByTestId(`issue-member-${code}`);
    expect(rows.map((row) => within(row).getByTestId("issue-member-refs").textContent)).toEqual([
      "Invented operating (load:LC1)", "Invented sustained (load:LC2)"
    ]);
    for (const row of rows) {
      expect(row).toHaveAttribute("aria-disabled", "true");
      expect(row).toHaveAttribute("title", NO_OWN_DETAIL_REASON);
      expect(within(row).getByTestId("issue-member-no-detail")).not.toHaveTextContent(/Remediation/);
      expect(within(row).getByTestId("issue-member-not-supplied")).toHaveTextContent(/^Not supplied: class, source, remediation, provenance$/);
    }
    document.body.innerHTML = "";
    // With identifiers: each row opens its own detail.
    const onSelect = vi.fn();
    renderList(groupIssues({ model, diagnostics: [forCase("load:LC1", "a"), forCase("load:LC2", "b")] }), onSelect);
    group = screen.getByTestId(`issue-group-${code}`);
    rows = within(openReports(group)).getAllByTestId(`issue-member-${code}`);
    fireEvent.click(rows[1]);
    fireEvent.click(rows[0]);
    expect(onSelect.mock.calls).toEqual([["b"], ["a"]]);
  });

  it("a model-check report keeps its shortened-text mark inside a grouped item", () => {
    const code = "INVENTED_SHORTENED";
    const long = { ...report({ id: "c", code }), truncated: true } as Diagnostic;
    const items = groupIssues({ model, modelChecks: [checked(long)], diagnostics: [report({ id: "s", code })], diagnosticOrigins: ["solve"] });
    render(<ModelChecksPanel model={model} run={run([checked(long)])} items={items} onRun={() => {}} onSelectEntity={() => {}} />);
    const rows = within(openReports(screen.getByTestId(`issue-group-${code}`))).getAllByTestId(`issue-member-${code}`);
    expect(within(rows[0]).getByTestId("issue-member-truncated")).toHaveTextContent("Text shortened in the bounded report.");
    expect(within(rows[1]).queryByTestId("issue-member-truncated")).toBeNull();
  });
});

describe("R3: the grouping key", () => {
  it("a report that names no object is never grouped", () => {
    const items = groupIssues({ model, diagnostics: [report({ code: "INVENTED_NOWHERE", affected_refs: [] }), report({ code: "INVENTED_NOWHERE", affected_refs: [""] }), report({ code: "INVENTED_NOWHERE", affected_refs: undefined })] });
    expect(items).toHaveLength(3);
  });

  it("keys an object by the entity the model resolves it to (materials included), and an unresolved reference by its exact text", () => {
    // A material the model checks resolve: their finding and a diagnostic naming it are one item.
    const material = groupIssues({
      model,
      modelChecks: [checked(report({ code: "INVENTED_MATERIAL", affected_refs: ["material:M1"] }), { type: "material", id: "material:M1" })],
      diagnostics: [report({ code: "INVENTED_MATERIAL", affected_refs: ["material:M1"] })]
    });
    expect(material.map((item) => item.members.length)).toEqual([2]);
    // "H-1" names no entity here; "support:H-1" does: two objects.
    const spelled = groupIssues({ model, diagnostics: [report({ code: "INVENTED_SPELLING", affected_refs: ["H-1"] }), report({ code: "INVENTED_SPELLING", affected_refs: ["support:H-1"] })] });
    expect(spelled.map((item) => item.object)).toEqual(["H-1", "support:H-1"]);
    // The same unresolved text twice is one object.
    const text = groupIssues({ model, diagnostics: [report({ code: "INVENTED_TEXT", affected_refs: ["somewhere"] }), report({ code: "INVENTED_TEXT", affected_refs: ["somewhere"] })] });
    expect(text).toHaveLength(1);
  });
});

describe("R4: a requested diagnostic", () => {
  it("is shown once, led by that report, with a true sentence, and pressing it selects that report", () => {
    const code = "INVENTED_REQUESTED";
    const fromChecks = report({ id: "check", code, severity: "blocking", message: "invented check text on support:H-1" });
    const solved = report({ id: "diag:requested", code, message: "invented solve text on support:H-1" });
    const items = groupIssues({ model, modelChecks: [checked(fromChecks)], diagnostics: [solved], diagnosticOrigins: ["solve"] });
    const onSelect = vi.fn();
    function Drawer() {
      const [pinned, setPinned] = useState<string | null>(null);
      const elsewhere = (item: IssueItem) => pinned !== null && item.members.some((member) => member.diagnostic.id === pinned);
      return (
        <>
          <ModelChecksPanel model={model} run={run([checked(fromChecks)])} items={items.filter((item) => item.home === "model-checks")} shownElsewhere={elsewhere} onRun={() => {}} onSelectEntity={() => {}} />
          <DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={onSelect}
            items={items.filter((item) => item.home === "diagnostics")} allItems={items} onPinnedChange={setPinned}
            filterRequest={{ diagnosticId: "diag:requested", sequence: 1 }} />
        </>
      );
    }
    render(<Drawer />);
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("Showing the requested report: 1 item, otherwise listed under Model checks.");
    expect(screen.getByTestId("diagnostic-filter-summary").textContent).not.toMatch(/of 0/);
    // Once: the Model checks list says where it is instead of showing it again.
    expect(screen.getAllByTestId(`issue-group-${code}`)).toHaveLength(1);
    expect(screen.getByTestId("issue-shown-under-diagnostics")).toHaveTextContent(`${code} on support:H-1: shown under Diagnostics below, as requested.`);
    const row = screen.getByTestId(`diagnostic-${code}`);
    expect(row.querySelector(".issue-message")).toHaveTextContent(solved.message);
    fireEvent.click(row);
    expect(onSelect).toHaveBeenCalledWith("diag:requested");
  });
});

describe("R5: a class decided after every report is read", () => {
  const classed = (id: string, cls: string, severity: Diagnostic["severity"] = "warning") => report({ id, code: "INVENTED_CLASSES", diagnostic_class: cls, severity });
  const classless = report({ id: "none", code: "INVENTED_CLASSES", severity: "blocking" });

  it("gives the same items, sections and counts in either arrival order of two classes around a classless blocking report", () => {
    const shape = (items: IssueItem[]) => items.map((item) => ({ ids: item.members.map((member) => member.diagnostic.id).sort(), severity: item.severity, home: item.home })).sort((a, b) => a.ids.join().localeCompare(b.ids.join()));
    const one = groupIssues({ model, diagnostics: [classed("a", "ASSUMPTION_WARNING"), classless, classed("b", "PROVENANCE_WARNING")] });
    const two = groupIssues({ model, diagnostics: [classed("b", "PROVENANCE_WARNING"), classless, classed("a", "ASSUMPTION_WARNING")] });
    expect(shape(one)).toEqual(shape(two));
    expect(shape(one)).toEqual([
      { ids: ["a"], severity: "warning", home: "diagnostics" },
      { ids: ["b"], severity: "warning", home: "diagnostics" },
      { ids: ["none"], severity: "blocking", home: "diagnostics" }
    ]);
    expect(countItemsBySeverity(one)).toEqual(countItemsBySeverity(two));
    renderList(one);
    const own = screen.getAllByTestId("diagnostic-INVENTED_CLASSES").map((row) => row.closest(".issue-entry") as HTMLElement)
      .find((entry) => within(entry).getByTestId("issue-meta").textContent?.includes("blocking"))!;
    // Formed because the class was ambiguous: it shows no class, and says nothing of one at rest.
    expect(within(own).queryByTestId("issue-class")).toBeNull();
    expect(own.textContent).not.toMatch(/not supplied/);
  });

  it("joins a classless report to its code's one stated class and leads with the most severe report, saying nothing of the lead's missing class", () => {
    const warning = classed("w", "ASSUMPTION_WARNING", "warning");
    const error = report({ id: "e", code: "INVENTED_CLASSES", severity: "error", message: "invented error text on support:H-1" });
    const items = groupIssues({ model, diagnostics: [warning, error] });
    expect(items).toHaveLength(1);
    expect(items[0].lead.diagnostic.id).toBe("e");
    renderList(items);
    const row = screen.getByTestId("diagnostic-INVENTED_CLASSES");
    expect(row.querySelector(".issue-message")).toHaveTextContent("invented error text on support:H-1");
    const entry = row.closest(".issue-entry") as HTMLElement;
    expect(within(entry).getAllByTestId("issue-class")[0]).toHaveTextContent(/^class ASSUMPTION_WARNING$/);
    expect(within(entry).getAllByTestId("issue-meta")[0].textContent).not.toMatch(/not supplied/);
  });

  it("a blank class is no class", () => {
    const items = groupIssues({ model, diagnostics: [report({ id: "x", code: "INVENTED_BLANK", diagnostic_class: "  " }), report({ id: "y", code: "INVENTED_BLANK", class: "" } as Diagnostic)] });
    expect(items).toHaveLength(1);
    renderList(items);
    expect(within(screen.getByTestId("diagnostic-INVENTED_BLANK")).queryByTestId("issue-class")).toBeNull();
  });
});

describe("R9: Model checks count items at each item's own severity", () => {
  it("a blocking solve report joined to a model-check warning counts as blocking in the section's summary", () => {
    const code = "INVENTED_JOINED_BLOCKING";
    const warning = report({ id: "c", code });
    const items = groupIssues({ model, modelChecks: [checked(warning)], diagnostics: [report({ id: "s", code, severity: "blocking" })], diagnosticOrigins: ["solve"] });
    render(<ModelChecksPanel model={model} run={run([checked(warning)])} items={items} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(screen.getByTestId("model-checks-summary")).toHaveTextContent(
      "1 item: 1 blocking or error, 0 warnings, 0 info. They gather 2 reports: a report repeated on one object is one item."
    );
  });
});

describe("the row, final: the severity's mark and the message, one muted line beneath, nothing else at rest", () => {
  const meta = (entry: HTMLElement) => within(entry.closest(".issue-entry") as HTMLElement).getAllByTestId("issue-meta")[0];

  it("a single report: its first line is the mark and the message; beneath it the severity's word, the code and the source; no note", () => {
    const code = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
    renderList(groupIssues({ model, diagnostics: [report({ id: "one", code, severity: "info", message: "invented checks passed on support:H-1" })] }));
    const row = screen.getByTestId(`diagnostic-${code}`);
    expect([...row.children].map((child) => child.className)).toEqual(["issue-first-line"]);
    expect(row.querySelector(".issue-first-line svg")).toHaveAttribute("aria-hidden", "true");
    expect(row.querySelector(".issue-message")).toHaveTextContent(/^invented checks passed on support:H-1$/);
    expect(meta(row)).toHaveTextContent(/^support:H-1 · info · NUMERICAL_INTEGRITY_CHECKS_PASSED · from the model$/);
  });

  it("a grouped item: its count of reports is the control on its muted line; each opened row has the same form", () => {
    const code = "INVENTED_CASES";
    const forCase = (id: string, lc: string) => report({ id, code, message: `invented check for ${lc} on support:H-1` });
    renderList(groupIssues({ model, diagnostics: [forCase("a", "LC1"), forCase("b", "LC2")] }));
    const group = screen.getByTestId(`issue-group-${code}`);
    const line = screen.getByTestId(`diagnostic-${code}`);
    expect(line.querySelector(".issue-message")).toHaveTextContent("invented check for LC1 on support:H-1");
    const muted = meta(line);
    expect(muted).toHaveTextContent(/^support:H-1 · warning · INVENTED_CASES · from the model · 2 reports$/);
    expect(within(muted).getByTestId("issue-group-members-toggle")).toHaveAttribute("aria-expanded", "false");
    for (const row of within(openReports(group)).getAllByTestId(`issue-member-${code}`)) {
      expect(row.firstElementChild).toHaveClass("issue-first-line");
      expect(row.querySelector(".issue-meta")).toHaveTextContent(/^support:H-1 · warning · INVENTED_CASES · from the model$/);
    }
    expect(within(muted).getByTestId("issue-group-members-toggle")).toHaveAttribute("aria-expanded", "true");
  });

  it("a long message (a checks-passed entry carrying a debug dump of some 43,500 characters) shows its beginning, cut at a word, and Show all; opened, the whole text, unchanged", () => {
    const code = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
    const words = Array.from({ length: 9000 }, (_, index) => `w${index % 997}`);
    let dump = "invented checks passed; debug: " + words.join(" ");
    dump = dump.slice(0, 43_500);
    renderList(groupIssues({ model, diagnostics: [report({ id: "long", code, severity: "info", message: dump }), report({ id: "next", code: "INVENTED_NEXT", message: "invented next issue" })] }));
    const row = screen.getByTestId(`diagnostic-${code}`);
    const shown = row.querySelector(".issue-message")!.textContent!;
    expect(shown.length).toBeLessThanOrEqual(LONG_MESSAGE_CHARACTERS);
    expect(shown.endsWith("…")).toBe(true);
    const beginning = shown.slice(0, -1);
    expect(dump.startsWith(beginning)).toBe(true);
    // Cut at a word: the next character of the message is a space.
    expect(dump[beginning.length]).toMatch(/\s/);
    const showAll = within(meta(row)).getByTestId("issue-show-all");
    expect(showAll).toHaveTextContent("Show all (43,500 characters)");
    expect(screen.queryByTestId("issue-message-full")).toBeNull();
    fireEvent.click(showAll);
    expect(screen.getByTestId("issue-message-full").textContent).toBe(dump);
    expect(showAll).toHaveTextContent("Show less");
    // The whole text stands in its own bounded box, outside the row's button; the next issue is still listed.
    expect(row.contains(screen.getByTestId("issue-message-full"))).toBe(false);
    expect(screen.getByTestId("issue-message-full")).toHaveClass("issue-message-full");
    expect(screen.getByTestId("diagnostic-INVENTED_NEXT")).toBeInTheDocument();
  });

  it("a message at or under the limit is shown whole, with no Show all", () => {
    const text = "x".repeat(LONG_MESSAGE_CHARACTERS);
    renderList(groupIssues({ model, diagnostics: [report({ id: "edge", code: "INVENTED_EDGE", message: text })] }));
    expect(screen.getByTestId("diagnostic-INVENTED_EDGE").querySelector(".issue-message")).toHaveTextContent(text);
    expect(screen.queryByTestId("issue-show-all")).toBeNull();
  });
});

describe("every row names its object (the missing-data use case's model)", () => {
  it("two findings of one code and message on two pipes read differently at rest, each led by its pipe's name, and each opens its own report", () => {
    const useCaseModel = missingDataStartModel as unknown as PreviewModel;
    const findings = useCaseModel.pipe_segments.map((pipe): Diagnostic => ({
      id: `invented:${pipe.id}`, code: "EQUIVALENT_STATIC_INPUT_MISSING", severity: "blocking",
      message: "invented: seismic generation requires material_density on every pipe segment", affected_refs: [pipe.id, "case:md-seismic"]
    }));
    const onSelect = vi.fn();
    render(<DiagnosticsPanel model={useCaseModel} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={onSelect}
      items={groupIssues({ model: useCaseModel, diagnostics: findings, diagnosticOrigins: ["solve", "solve"] })} />);
    const rows = screen.getAllByTestId("diagnostic-EQUIVALENT_STATIC_INPUT_MISSING");
    expect(rows).toHaveLength(2);
    const lines = rows.map((row) => within(row.closest(".issue-entry") as HTMLElement).getByTestId("issue-meta").textContent);
    expect(lines).toEqual([
      expect.stringMatching(/^Inboard span \(pipe:md-inboard\); Invented seismic static-equivalent \(case:md-seismic\) · blocking · EQUIVALENT_STATIC_INPUT_MISSING · /),
      expect.stringMatching(/^Outboard span \(pipe:md-outboard\); Invented seismic static-equivalent \(case:md-seismic\) · blocking · EQUIVALENT_STATIC_INPUT_MISSING · /)
    ]);
    fireEvent.click(rows[1]);
    expect(onSelect).toHaveBeenCalledWith("invented:pipe:md-outboard");
  });
});

describe("the solver's messages: the result's diagnostics, and only those, bring the list's one heading", () => {
  const own: Diagnostic = { id: "own", code: "INVENTED_MODEL_OWN", severity: "warning", message: "invented model finding", affected_refs: ["support:H-1"] };
  const known: Diagnostic = { id: "known", code: "INVENTED_KNOWLEDGE", severity: "info", message: "invented knowledge finding", affected_refs: [] };
  const solved = (id: string): Diagnostic => ({ id, code: `INVENTED_SOLVE_${id.toUpperCase()}`, severity: "warning", message: `invented solve finding ${id}`, affected_refs: [] });
  const result = { diagnostics: [solved("a"), solved("b")], results: [], contract_evidence: {}, summary: {} } as unknown as MechanicsResult;
  const knowledge = { diagnostics: [known] } as unknown as DesignKnowledge;
  const withOwn = { ...model, diagnostics: [own] } as PreviewModel;

  it("gives the origin solve to every result diagnostic and to nothing else", () => {
    const { diagnostics, diagnosticOrigins } = diagnosticsListOf(withOwn, knowledge, result, { projectId: null, findings: [] } as never);
    const solveIds = diagnostics.filter((_, index) => diagnosticOrigins[index] === "solve").map((item) => item.id);
    expect(solveIds).toEqual(["a", "b"]);
    expect(diagnostics.filter((_, index) => diagnosticOrigins[index] !== "solve").map((item) => item.id)).toEqual(expect.arrayContaining(["own", "known"]));
  });

  it("stands once, at the head of the list, while some listed item holds one, and goes when a filter hides them all", () => {
    render(<DiagnosticsPanel model={withOwn} knowledge={knowledge} result={result} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    const headings = screen.getAllByTestId("diagnostic-list-solver-messages-heading");
    expect(headings).toHaveLength(1);
    expect(headings[0].parentElement?.firstElementChild).toBe(headings[0]);
    expect(headings[0]).toHaveTextContent(SOLVER_MESSAGES_HEADING);
    fireEvent.change(screen.getByTestId("diagnostic-filter-input"), { target: { value: "INVENTED_MODEL_OWN" } });
    expect(screen.queryByTestId("diagnostic-list-solver-messages-heading")).toBeNull();
  });
});


describe("a one-report item whose report opens no detail of its own opens its fields on its row", () => {
  const twoSupports = { ...model, supports: [{ id: "support:H-1", label: "Invented hanger one" }, { id: "support:H-2", label: "Invented hanger two" }] } as unknown as PreviewModel;
  const fieldsOf = (entry: HTMLElement) => {
    const toggle = within(entry).getByTestId("issue-fields-toggle");
    const fields = within(entry).getByTestId("issue-fields");
    expect(fields.hidden).toBe(true);
    expect(toggle).toHaveTextContent(/^Details$/);
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    expect(fields.hidden).toBe(false);
    return fields;
  };

  it("two diagnostics with no identifier sharing a code on two objects: each opens its own remediation, provenance, Not supplied line and object", () => {
    const code = "INVENTED_SHARED_CODE";
    const first: Diagnostic = { code, severity: "warning", message: "invented finding on the first hanger", affected_refs: ["support:H-1"], remediation: "Invented remediation one.", provenance: "invented provenance one" };
    const second: Diagnostic = { code, severity: "warning", message: "invented finding on the second hanger", affected_refs: ["support:H-2"], source: "invented_source" };
    const items = groupIssues({ model: twoSupports, diagnostics: [first, second] });
    expect(items).toHaveLength(2);
    const onSelectEntity = vi.fn();
    render(<DiagnosticsPanel model={twoSupports} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} onSelectEntity={onSelectEntity} items={items} />);
    const entries = screen.getAllByTestId(`diagnostic-${code}`).map((row) => row.closest(".issue-entry") as HTMLElement);
    expect(entries).toHaveLength(2);
    // Neither row opens a detail of its own: two reports share the code and neither has an identifier.
    for (const entry of entries) expect(within(entry).getByTestId(`diagnostic-${code}`)).toHaveAttribute("aria-disabled", "true");

    const one = fieldsOf(entries[0]);
    expect(one).toHaveTextContent(NO_OWN_DETAIL_REASON);
    expect(one).toHaveTextContent("Remediation: Invented remediation one.");
    expect(one).toHaveTextContent("Provenance: invented provenance one");
    expect(within(one).getByTestId("issue-member-not-supplied")).toHaveTextContent(/^Not supplied: class, source$/);
    fireEvent.click(within(one).getByTestId("issue-fields-select"));
    expect(onSelectEntity).toHaveBeenLastCalledWith(expect.objectContaining({ id: "support:H-1" }));

    const two = fieldsOf(entries[1]);
    expect(two).toHaveTextContent("Source: invented_source");
    expect(two).not.toHaveTextContent("Remediation:");
    expect(within(two).getByTestId("issue-member-not-supplied")).toHaveTextContent(/^Not supplied: class, remediation, provenance$/);
    fireEvent.click(within(two).getByTestId("issue-fields-select"));
    expect(onSelectEntity).toHaveBeenLastCalledWith(expect.objectContaining({ id: "support:H-2" }));
  });

  it("a diagnostic that opens its own detail has no fields control: its detail shows them", () => {
    const own: Diagnostic = { id: "own", code: "INVENTED_OWN_DETAIL", severity: "warning", message: "invented finding with its own detail", affected_refs: ["support:H-1"] };
    render(<DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} items={groupIssues({ model, diagnostics: [own] })} />);
    expect(screen.queryByTestId("issue-fields-toggle")).toBeNull();
  });

  it("a one-report Model checks item opens its remediation and Not supplied line; its row already selects its object", () => {
    const finding = checked(report({ id: "check-one", code: "INVENTED_CHECK_ONLY", remediation: "Invented check remediation." }));
    const items = groupIssues({ model, modelChecks: [finding] });
    const onSelectEntity = vi.fn();
    render(<ModelChecksPanel model={model} run={run([finding])} items={items} onRun={() => {}} onSelectEntity={onSelectEntity} />);
    const entry = screen.getByTestId("model-check-INVENTED_CHECK_ONLY").closest(".issue-entry") as HTMLElement;
    const fields = fieldsOf(entry);
    expect(fields).toHaveTextContent("Remediation: Invented check remediation.");
    expect(fields).not.toHaveTextContent(NO_OWN_DETAIL_REASON);
    expect(within(fields).getByTestId("issue-member-not-supplied")).toHaveTextContent(/^Not supplied: class, source, provenance$/);
    expect(within(fields).queryByTestId("issue-fields-select")).toBeNull();
    fireEvent.click(screen.getByTestId("model-check-INVENTED_CHECK_ONLY"));
    expect(onSelectEntity).toHaveBeenCalledWith({ type: "support", id: "support:H-1" });
  });

  it("a one-report Operation diagnostics item opens its fields, its Not supplied line and its object to select", () => {
    const operation: Diagnostic = { id: "op-one", code: "INVENTED_OPERATION", severity: "warning", message: "invented operation finding", affected_refs: ["support:H-1"], provenance: "invented operation provenance" };
    const [item] = groupIssues({ model, operations: [operation] });
    const onSelectEntity = vi.fn();
    render(<IssueItemView item={item} onSelectEntity={onSelectEntity} main={(body) => <article className="issue-row diagnostic">{body}</article>} />);
    const fields = fieldsOf(screen.getByTestId("issue-meta").closest(".issue-entry") as HTMLElement);
    expect(fields).toHaveTextContent("Provenance: invented operation provenance");
    expect(fields).not.toHaveTextContent(NO_OWN_DETAIL_REASON);
    expect(within(fields).getByTestId("issue-member-not-supplied")).toHaveTextContent(/^Not supplied: class, source, remediation$/);
    fireEvent.click(within(fields).getByTestId("issue-fields-select"));
    expect(onSelectEntity).toHaveBeenCalledWith(expect.objectContaining({ id: "support:H-1" }));
  });
});

describe("the drawer: the solver-messages heading stands above every solver message, wherever its item is listed (LIM-026)", () => {
  const heads = (testId: string) => screen.getAllByTestId(testId);
  const before = (a: Element, b: Element) => (a.compareDocumentPosition(b) & Node.DOCUMENT_POSITION_FOLLOWING) !== 0;

  it("a solver message joined to a model-check finding: one heading, above the Model checks list, and none again in Diagnostics", async () => {
    const base = await loadPreviewModel();
    const support = base.supports[0].id;
    const shared: Diagnostic = { id: "invented-joined", code: "INVENTED_JOINED", severity: "warning", message: "invented joined finding", affected_refs: [support] };
    const finding: ModelCheckFinding = { key: "check-joined", group: "solve", entity: { type: "support", id: support }, diagnostic: { ...shared, id: "check-joined" } };
    const another: Diagnostic = { id: "invented-listed", code: "INVENTED_LISTED", severity: "info", message: "invented listed solve finding", affected_refs: [support] };
    const issues = issueSummary({ model: base, modelChecks: [finding], diagnostics: [shared, another], diagnosticOrigins: ["solve", "solve"], missingInputs: 0 });
    render(<IssuesHome issues={issues} model={base} knowledge={null} result={null} selectedDiagnosticId={null} onClose={() => {}} onSelectDiagnostic={() => {}} modelCheckRun={{ ...run([finding]), model: base }} />);
    const headings = heads("diagnostic-list-solver-messages-heading");
    expect(headings).toHaveLength(1);
    expect(headings[0]).toHaveTextContent(SOLVER_MESSAGES_HEADING);
    // Above the Model checks list and every solver message, in Model checks and in Diagnostics.
    expect(before(headings[0], screen.getByTestId("model-checks-panel"))).toBe(true);
    expect(before(headings[0], screen.getByTestId("issue-group-INVENTED_JOINED"))).toBe(true);
    expect(before(headings[0], screen.getByTestId("diagnostic-INVENTED_LISTED"))).toBe(true);
    expect(within(screen.getByTestId("diagnostics-panel")).queryByTestId("diagnostic-list-solver-messages-heading")).toBeNull();
  });

  it("every solver message moved under Model checks: the heading still stands, above them", async () => {
    const base = await loadPreviewModel();
    const support = base.supports[0].id;
    const shared: Diagnostic = { id: "invented-only", code: "INVENTED_ONLY_JOINED", severity: "warning", message: "invented only joined finding", affected_refs: [support] };
    const finding: ModelCheckFinding = { key: "check-only", group: "solve", entity: { type: "support", id: support }, diagnostic: { ...shared, id: "check-only" } };
    const issues = issueSummary({ model: base, modelChecks: [finding], diagnostics: [shared], diagnosticOrigins: ["solve"], missingInputs: 0 });
    render(<IssuesHome issues={issues} model={base} knowledge={null} result={null} selectedDiagnosticId={null} onClose={() => {}} onSelectDiagnostic={() => {}} modelCheckRun={{ ...run([finding]), model: base }} />);
    const headings = heads("diagnostic-list-solver-messages-heading");
    expect(headings).toHaveLength(1);
    expect(before(headings[0], screen.getByTestId("issue-group-INVENTED_ONLY_JOINED"))).toBe(true);
  });

  it("solver messages only under Diagnostics: the Diagnostics list heads itself, once", async () => {
    const base = await loadPreviewModel();
    const solveOnly: Diagnostic = { id: "invented-solve-only", code: "INVENTED_SOLVE_ONLY", severity: "info", message: "invented solve-only finding", affected_refs: [base.supports[0].id] };
    const issues = issueSummary({ model: base, diagnostics: [solveOnly], diagnosticOrigins: ["solve"], missingInputs: 0 });
    render(<IssuesHome issues={issues} model={base} knowledge={null} result={null} selectedDiagnosticId={null} onClose={() => {}} onSelectDiagnostic={() => {}} />);
    const headings = heads("diagnostic-list-solver-messages-heading");
    expect(headings).toHaveLength(1);
    expect(within(screen.getByTestId("diagnostics-panel")).getByTestId("diagnostic-list-solver-messages-heading")).toBe(headings[0]);
  });
});

describe("the line's Rule inputs missing opens the drawer at the rule-input entries", () => {
  it("brings the first rule-input entry into view and gives it focus, not the Missing data panel as a whole", async () => {
    const base = await loadPreviewModel();
    const model = { ...base, analysis_status: { ...base.analysis_status, mechanics: "MODEL_INCOMPLETE", rule_check: "RULE_INPUTS_INCOMPLETE" } };
    const issues = issueSummary({ model, missingInputs: 0 });
    const onTargetShown = vi.fn();
    render(<IssuesHome issues={issues} model={model} knowledge={null} result={null} selectedDiagnosticId={null} onClose={() => {}} onSelectDiagnostic={() => {}}
      target={{ kind: "missing-data", sequence: 1 }} onTargetShown={onTargetShown} />);
    const focused = document.activeElement as HTMLElement;
    expect(focused).toHaveAttribute("data-warning-class", "RULE_CHECK_BLOCKING");
    // The first of the rule-input entries the line's count counts; the mechanics entry, which also
    // blocks the rule check, is not where the control leads.
    expect(focused).toBe(screen.getByTestId("missing-data-panel").querySelector('[data-warning-class="RULE_CHECK_BLOCKING"]'));
    expect(focused).not.toBe(screen.getByTestId("missing-data-warning-solve-required-physical-inputs"));
    expect(onTargetShown).toHaveBeenCalledTimes(1);
  });
});
