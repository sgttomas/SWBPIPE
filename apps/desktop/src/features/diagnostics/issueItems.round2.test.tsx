// The Issues drawer's items, second review round: excerpts only where they are the producer's
// text and no report takes the solver report note (R1); every opened row is its own report (R2);
// the grouping key (R3); a requested diagnostic shown once (R4); a class decided after every
// report is read (R5); counts at each item's severity, and missing data apart (R9). Invented
// models and diagnostics only.
import { fireEvent, render, screen, within } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import type { Diagnostic, PreviewModel } from "../../types";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { countItemsBySeverity, groupIssues, type IssueItem } from "./issueItems";
import { NO_OWN_DETAIL_REASON } from "./IssueItemView";
import type { ModelCheckFinding } from "./modelChecks";
import { ModelChecksPanel } from "./ModelChecksPanel";

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
  const disclosure = within(group).getByTestId("issue-group-members") as HTMLDetailsElement;
  expect(disclosure.open).toBe(false);
  fireEvent.click(within(disclosure).getByTestId("issue-group-members-toggle"));
  expect(disclosure.open).toBe(true);
  return disclosure;
}
const noteIds = () => screen.queryAllByTestId(/^diagnostic-solver-report-note-/);

describe("R1: excerpts of messages appear only where they are the producer's text and no report takes the note", () => {
  it("an item whose later report alone takes the note shows no excerpt anywhere, and that report its note", () => {
    const code = "NUMERICAL_INTEGRITY_SENSITIVE";
    const plain = report({ id: "p", code, message: "invented case LC1 on support:H-1: its prior outcome was none" });
    const quoting = report({ id: "q", code, message: "invented case LC2 on support:H-1: its prior outcome was NUMERICAL_INTEGRITY_CHECKS_PASSED" });
    const items = groupIssues({ model, diagnostics: [plain, quoting] });
    expect(items).toHaveLength(1);
    expect(items[0].differences).toEqual([]);
    renderList(items);
    const group = screen.getByTestId(`issue-group-${code}`);
    expect(within(group).queryByTestId("issue-group-also")).toBeNull();
    const rows = within(openReports(group)).getAllByTestId(`issue-member-${code}`);
    expect(within(group).queryAllByTestId("issue-member-difference")).toHaveLength(0);
    expect(within(rows[1]).getByText(quoting.message).previousElementSibling).toHaveAttribute("data-testid", `diagnostic-solver-report-note-${code}`);
    expect(within(rows[0]).queryByTestId(`diagnostic-solver-report-note-${code}`)).toBeNull();
  });

  it("the hanger's six reports (three loads from the model checks and the solve) show the excerpts and no note anywhere", () => {
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
    expect(within(group).getByTestId("issue-group-also")).toHaveTextContent(
      "Reported for each of: installed load (hanger.installed_load); cold load (hanger.cold_load); hot load (hanger.hot_load)."
    );
    const rows = within(openReports(group)).getAllByTestId(`issue-member-${code}`);
    expect(rows).toHaveLength(6);
    // S6: each opened row reads its eyebrow first and its message last, the message marking what tells it apart.
    for (const row of rows) expect(row.firstElementChild).toHaveClass("issue-meta");
    const messages = rows.map((row) => row.lastElementChild?.lastElementChild as HTMLElement);
    for (const message of messages) expect(message).toHaveClass("issue-message");
    expect(messages.map((message) => within(message).getByTestId("issue-member-difference").textContent)).toEqual([
      "installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)",
      "installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)"
    ]);
    // The excerpt is cut from the producer's message, which is shown whole.
    expect(messages[1]).toHaveTextContent("invented hanger support:H-1: its cold load (hanger.cold_load) is not applied as preload");
    expect(noteIds()).toHaveLength(0);
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
      "Also about: Invented operating (load:LC1)", "Also about: Invented sustained (load:LC2)"
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
    const own = screen.getAllByTestId("diagnostic-INVENTED_CLASSES").find((row) => row.textContent?.includes("blocking"))!;
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
    expect(within(row).getByTestId("issue-class")).toHaveTextContent(/^class ASSUMPTION_WARNING$/);
    expect(row.textContent).not.toMatch(/not supplied/);
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

describe("S6: every entry reads eyebrow, what it also names, the note where it takes one, then the message last", () => {
  const code = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
  const entryParts = (entry: HTMLElement) => {
    const meta = entry.firstElementChild as HTMLElement;
    const block = entry.lastElementChild as HTMLElement;
    return { meta, block, message: block.lastElementChild as HTMLElement, note: block.firstElementChild as HTMLElement };
  };
  it("a single report: severity first in the eyebrow, the note directly before the message, nothing after it", () => {
    renderList(groupIssues({ model, diagnostics: [report({ id: "one", code, severity: "info", message: "invented checks passed on support:H-1", affected_refs: ["support:H-1", "load:LC1"] })] }));
    const row = screen.getByTestId(`diagnostic-${code}`);
    const { meta, block, message, note } = entryParts(row);
    expect(meta).toHaveTextContent(/^info · NUMERICAL_INTEGRITY_CHECKS_PASSED · from /);
    expect(within(row).getByTestId("issue-member-refs")).toHaveTextContent("Also about: Invented operating (load:LC1)");
    expect(within(row).getByTestId("issue-member-refs").compareDocumentPosition(block) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(note).toHaveAttribute("data-testid", `diagnostic-solver-report-note-${code}`);
    expect(message).toHaveTextContent(/^invented checks passed on support:H-1$/);
    expect(message.nextElementSibling).toBeNull();
  });
  it("a grouped item: its count stands in the eyebrow, the lead's note and message last; each opened row the same", () => {
    const forCase = (id: string, lc: string) => report({ id, code, severity: "info", message: `invented checks passed for ${lc} on support:H-1` });
    renderList(groupIssues({ model, diagnostics: [forCase("a", "LC1"), forCase("b", "LC2")] }));
    const group = screen.getByTestId(`issue-group-${code}`);
    const line = screen.getByTestId(`diagnostic-${code}`);
    const collapsed = entryParts(line);
    expect(within(collapsed.meta).getByTestId("issue-group-count")).toHaveTextContent("2 reports");
    expect(collapsed.meta).toHaveTextContent(/^info · NUMERICAL_INTEGRITY_CHECKS_PASSED · /);
    expect(collapsed.note).toHaveAttribute("data-testid", `diagnostic-solver-report-note-${code}`);
    expect(collapsed.message).toHaveTextContent("invented checks passed for LC1 on support:H-1");
    expect(collapsed.message.nextElementSibling).toBeNull();
    for (const row of within(openReports(group)).getAllByTestId(`issue-member-${code}`)) {
      const parts = entryParts(row);
      expect(parts.meta).toHaveTextContent(/^info · NUMERICAL_INTEGRITY_CHECKS_PASSED · from /);
      expect(parts.note).toHaveAttribute("data-testid", `diagnostic-solver-report-note-${code}`);
      expect(parts.message.nextElementSibling).toBeNull();
    }
  });
});
