import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { IssueItemView } from "./IssueItemView";
import type { Diagnostic, PreviewModel } from "../../types";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { countItemsBySeverity, groupIssues, issueSummary, messageDifferences, type IssueItem } from "./issueItems";
import type { ModelCheckFinding } from "./modelChecks";
import { ModelChecksPanel } from "./ModelChecksPanel";

// The Issues count's rule: an item is one finding code on one object, whoever
// reported it and however many loads repeat it. Invented diagnostics only.
const hanger = (load: string, severity: Diagnostic["severity"] = "warning"): Diagnostic => ({
  id: `diag:invented-hanger-${load}`,
  code: "INVENTED_HANGER_LOAD_NOT_PRELOAD",
  severity,
  message: `invented hanger support:H-1: its ${load} load (hanger.${load}_load) is not applied as preload; review only`,
  affected_refs: ["support:H-1"],
  source: "invented_test"
});
const other: Diagnostic = { id: "diag:invented-other", code: "INVENTED_OTHER", severity: "info", message: "invented other text", affected_refs: [] };
const model = {
  project: { id: "project:invented-issue-items" },
  nodes: [], pipe_segments: [], components: [], load_cases: [],
  supports: [{ id: "support:H-1" }],
  diagnostics: []
} as unknown as PreviewModel;
const finding = (diagnostic: Diagnostic): ModelCheckFinding => ({ key: diagnostic.id ?? diagnostic.code, group: "solve", diagnostic, entity: { type: "support", id: "support:H-1" } });

describe("issue items", () => {
  it("makes one item of a support's installed, cold and hot load reports, keeping each report", () => {
    const items = groupIssues({ model, diagnostics: [hanger("installed"), hanger("cold"), hanger("hot"), other] });
    expect(items).toHaveLength(2);
    const [item] = items;
    expect(item.code).toBe("INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(item.object).toBe("support:H-1");
    expect(item.entity).toEqual({ type: "support", id: "support:H-1" });
    expect(item.members.map((member) => member.diagnostic.message)).toEqual([hanger("installed"), hanger("cold"), hanger("hot")].map((d) => d.message));
    expect(item.differences).toEqual(["installed load (hanger.installed_load)", "cold load (hanger.cold_load)", "hot load (hanger.hot_load)"]);
  });

  it("joins the same finding from the model checks and the solve into one item, listed with the checks, at its most severe", () => {
    const items = groupIssues({
      model,
      modelChecks: [finding(hanger("installed")), finding(hanger("cold"))],
      diagnostics: [hanger("installed"), hanger("cold", "error"), other]
    });
    expect(items.map((item) => [item.code, item.home, item.severity, item.members.length])).toEqual([
      ["INVENTED_HANGER_LOAD_NOT_PRELOAD", "model-checks", "error", 4],
      ["INVENTED_OTHER", "diagnostics", "info", 1]
    ]);
    expect(items[0].differences).toEqual(["installed load (hanger.installed_load)", "cold load (hanger.cold_load)"]);
  });

  it("keeps different objects and different codes apart, and counts missing inputs with the items", () => {
    const elsewhere = { ...hanger("installed"), id: "diag:invented-elsewhere", affected_refs: ["support:H-2"] };
    const summary = issueSummary({ model, diagnostics: [hanger("installed"), elsewhere, other], missingInputs: 2 });
    expect(summary.items).toHaveLength(3);
    expect(summary.reports).toBe(3);
    expect(summary.total).toBe(5);
  });

  it("finds no differences in one message", () => {
    expect(messageDifferences(["a b c", "a b c"])).toEqual([]);
    expect(messageDifferences(["a x c", "a y z c"])).toEqual(["x", "y z"]);
  });
});

describe("the drawer's lists show items", () => {
  it("the Diagnostics list shows one hanger item with each load, and every report inside its open disclosure", () => {
    const onSelect = vi.fn();
    const withHanger = { ...model, diagnostics: [hanger("installed"), hanger("cold"), hanger("hot"), other] } as PreviewModel;
    render(<DiagnosticsPanel model={withHanger} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={onSelect} />);
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("2 of 2 items shown (4 reports)");
    expect(screen.getByTestId("diagnostic-severity-warning")).toHaveTextContent("1 Warnings");
    expect(screen.getAllByTestId("diagnostic-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toHaveLength(1);
    const group = screen.getByTestId("issue-group-INVENTED_HANGER_LOAD_NOT_PRELOAD");
    // At rest the item shows its lead's message and its muted line only; no excerpt line.
    expect(within(group).queryByTestId("issue-group-also")).toBeNull();
    const disclosure = within(group).getByTestId("issue-group-members");
    expect(disclosure).not.toBeVisible();
    fireEvent.click(within(group).getByRole("button", { name: "All 3 reports for support:H-1" }));
    expect(disclosure).toBeVisible();
    const members = within(disclosure).getAllByTestId("issue-member-INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(members.map((member) => member.textContent)).toEqual([
      expect.stringContaining("hanger.installed_load"), expect.stringContaining("hanger.cold_load"), expect.stringContaining("hanger.hot_load")
    ]);
    fireEvent.click(members[1]);
    expect(onSelect).toHaveBeenCalledWith("diag:invented-hanger-cold");
  });

  it("the Model checks list counts items beside the findings they gather", () => {
    const run = {
      model, solve: { kind: "checked" as const }, findings: [finding(hanger("installed")), finding(hanger("cold")), finding(hanger("hot"))],
      notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 }
    };
    render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(screen.getByTestId("model-checks-summary")).toHaveTextContent(
      "1 item: 0 blocking or error, 1 warning, 0 info. They gather 3 reports: a report repeated on one object is one item."
    );
    expect(screen.getByRole("list", { name: "Model check findings" })).toHaveTextContent("Solve input, restraint and topology checks 1 item from 3 reports");
    expect(screen.getAllByTestId("model-check-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toHaveLength(1);
  });
});

describe("classes stay apart (DEL-07-04 REQ-001)", () => {
  it("never makes one item of two classes; a report without a class joins its code on its object only where one class is stated", () => {
    const classed = (cls: string, id: string): Diagnostic => ({ ...hanger("installed"), id, diagnostic_class: cls });
    // Two classes stated: each classless report is its own item.
    const two = groupIssues({ model, diagnostics: [hanger("cold"), classed("ASSUMPTION_WARNING", "a"), classed("PROVENANCE_WARNING", "b"), hanger("hot")] });
    expect(two.map((item) => item.members.map((member) => member.diagnostic.id))).toEqual([
      ["diag:invented-hanger-cold"], ["a"], ["b"], ["diag:invented-hanger-hot"]
    ]);
    // One class stated: the classless reports join it, whatever the order.
    const one = groupIssues({ model, diagnostics: [hanger("cold"), classed("ASSUMPTION_WARNING", "a"), hanger("hot")] });
    expect(one.map((item) => item.members.map((member) => member.diagnostic.id))).toEqual([["diag:invented-hanger-cold", "a", "diag:invented-hanger-hot"]]);
  });
});

// Opens an item's reports by a press on its count of reports, as a person does, and fails where
// the press does not open them.
function openReports(group: HTMLElement): HTMLElement {
  const disclosure = within(group).getByTestId("issue-group-members");
  expect(disclosure.hidden).toBe(true);
  fireEvent.click(within(group).getByTestId("issue-group-members-toggle"));
  expect(disclosure.hidden).toBe(false);
  return disclosure;
}
const memberRows = (disclosure: HTMLElement, code: string) => within(disclosure).getAllByTestId(`issue-member-${code}`);
/** Each opened row's supplied class, or null where its producer gave none (nothing is shown for it). */
const classesOf = (rows: HTMLElement[]) => rows.map((row) => within(row).queryByTestId("issue-member-class")?.textContent ?? null);
/** Each opened row's "Not supplied" line, or null where every field was supplied. */
const notSuppliedOf = (rows: HTMLElement[]) => rows.map((row) => within(row).queryByTestId("issue-member-not-supplied")?.textContent ?? null);
const diagnostic = (id: string, code: string, severity: Diagnostic["severity"], message: string, cls?: string): Diagnostic => ({
  id, code, severity, message, affected_refs: ["support:H-1"], ...(cls ? { diagnostic_class: cls } : {})
});
const renderDiagnostics = (diagnostics: Diagnostic[], onSelect = vi.fn()) =>
  render(<DiagnosticsPanel model={{ ...model, diagnostics } as PreviewModel} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={onSelect} />);

describe("every item and every report shows its class where one is supplied, and nothing at rest where none is (DEL-07-04 REQ-001, REQ-004)", () => {
  it("shows the supplied class on a one-report item, and nothing for a class its producer did not supply", () => {
    renderDiagnostics([
      diagnostic("c1", "INVENTED_CLASSED", "warning", "invented classed text", "ASSUMPTION_WARNING"),
      { ...diagnostic("c2", "INVENTED_UNCLASSED", "info", "invented unclassed text"), affected_refs: [] },
      { ...diagnostic("c3", "INVENTED_OLD_CLASS", "info", "invented class-field text"), affected_refs: [], class: "PROVENANCE_WARNING" } as Diagnostic
    ]);
    expect(within(screen.getByTestId("diagnostic-INVENTED_CLASSED")).getByTestId("issue-class")).toHaveTextContent(/^class ASSUMPTION_WARNING$/);
    expect(within(screen.getByTestId("diagnostic-INVENTED_UNCLASSED")).queryByTestId("issue-class")).toBeNull();
    expect(screen.getByTestId("diagnostic-INVENTED_UNCLASSED").textContent).not.toMatch(/not supplied/);
    expect(within(screen.getByTestId("diagnostic-INVENTED_OLD_CLASS")).getByTestId("issue-class")).toHaveTextContent(/^class PROVENANCE_WARNING$/);
  });

  it("shows a classed item's class once on the item; an opened classless row shows no class and names it on its Not supplied line", () => {
    renderDiagnostics([
      hanger("cold"),
      { ...hanger("installed"), id: "classed-installed", diagnostic_class: "ASSUMPTION_WARNING" },
      hanger("hot")
    ]);
    const group = screen.getByTestId("issue-group-INVENTED_HANGER_LOAD_NOT_PRELOAD");
    const main = screen.getByTestId("diagnostic-INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(within(main).getAllByTestId("issue-class").map((node) => node.textContent)).toEqual(["class ASSUMPTION_WARNING"]);
    const rows = memberRows(openReports(group), "INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(classesOf(rows)).toEqual([null, "class ASSUMPTION_WARNING", null]);
    expect(notSuppliedOf(rows)).toEqual(["Not supplied: class, remediation, provenance", "Not supplied: remediation, provenance", "Not supplied: class, remediation, provenance"]);
    // At rest, the item says nothing of a missing class.
    expect(main.textContent).not.toMatch(/not supplied/);
  });

  it("shows the class on the Model checks list's rows and on an operation's item", () => {
    const run = {
      model, solve: { kind: "checked" as const }, findings: [finding({ ...hanger("installed"), diagnostic_class: "ASSUMPTION_WARNING" })],
      notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 }
    };
    render(<ModelChecksPanel model={model} run={run} onRun={() => {}} onSelectEntity={() => {}} />);
    expect(within(screen.getByTestId("model-check-INVENTED_HANGER_LOAD_NOT_PRELOAD")).getByTestId("issue-class")).toHaveTextContent(/^class ASSUMPTION_WARNING$/);
    const [operationItem] = groupIssues({ model, operations: [other] });
    render(<IssueItemView item={operationItem} main={(body) => <article data-testid="operation-item">{body}</article>} />);
    expect(within(screen.getByTestId("operation-item")).queryByTestId("issue-class")).toBeNull();
    expect(screen.getByTestId("operation-item").textContent).not.toMatch(/not supplied/);
  });
});

describe("a report gathered into another list's item opens its detail there", () => {
  const checksRun = (findings: ModelCheckFinding[]) => ({
    model, solve: { kind: "checked" as const }, findings,
    notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 }
  });

  it("a solve diagnostic inside a Model checks item opens its detail, and the row shows the item's worst severity", () => {
    const checked = finding(hanger("installed"));
    const solved = { ...hanger("installed", "error"), id: "diag:invented-solve-installed" };
    const items = groupIssues({ model, modelChecks: [checked], diagnostics: [solved], diagnosticOrigins: ["solve"] });
    const onSelect = vi.fn();
    render(<ModelChecksPanel model={model} run={checksRun([checked])} items={items} onRun={() => {}} onSelectEntity={() => {}} onSelectDiagnostic={onSelect} />);
    const row = screen.getByTestId("model-check-INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(row).toHaveClass("error");
    // The row's first line is the severity's mark and the message; the item's severity word and
    // its code lead the muted line beneath it.
    expect(row.firstElementChild).toHaveClass("issue-first-line");
    expect(row.querySelector(".issue-message")).toHaveTextContent(hanger("installed").message);
    expect(within(row.closest(".issue-entry") as HTMLElement).getByTestId("issue-meta")).toHaveTextContent(/^error · INVENTED_HANGER_LOAD_NOT_PRELOAD · /);
    const rows = memberRows(openReports(screen.getByTestId("issue-group-INVENTED_HANGER_LOAD_NOT_PRELOAD")), "INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(rows.map((member) => member.querySelector(".issue-meta")?.textContent)).toEqual([expect.stringMatching(/^warning · INVENTED_HANGER_LOAD_NOT_PRELOAD · from the model checks/), expect.stringMatching(/^error · INVENTED_HANGER_LOAD_NOT_PRELOAD · from the solve/)]);
    const solveRow = rows[1];
    expect(solveRow).toHaveAttribute("aria-disabled", "false");
    fireEvent.click(solveRow);
    expect(onSelect).toHaveBeenCalledWith("diag:invented-solve-installed");
  });

  it("the Diagnostics list's filter request reaches a diagnostic the drawer lists under Model checks", () => {
    const checked = finding(hanger("installed"));
    const solved = { ...hanger("installed"), id: "diag:invented-solve-installed" };
    const all = groupIssues({ model, modelChecks: [checked], diagnostics: [solved, other], diagnosticOrigins: ["solve", "model"] });
    const own = all.filter((item) => item.home === "diagnostics");
    expect(own.map((item) => item.code)).toEqual(["INVENTED_OTHER"]);
    render(
      <DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}}
        items={own} allItems={all} filterRequest={{ diagnosticId: "diag:invented-solve-installed", sequence: 1 }} />
    );
    expect(screen.getByTestId("diagnostic-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toBeInTheDocument();
    expect(screen.queryByTestId("diagnostic-INVENTED_OTHER")).toBeNull();
  });
});

describe("an item that holds several reports keeps each one readable", () => {
  it("one code on one object for two load cases: the closed item says two, the open one gives each case's action", () => {
    const forCase = (id: string, lc: string) =>
      diagnostic(id, "INVENTED_CASE_LOAD_UNSET", "warning", `invented support:H-1 has no load for load case ${lc}; enter the ${lc} load on the support`);
    renderDiagnostics([forCase("lc1", "LC-1"), forCase("lc2", "LC-2")]);
    const group = screen.getByTestId("issue-group-INVENTED_CASE_LOAD_UNSET");
    expect(within(group).getByTestId("issue-group-count")).toHaveTextContent(/^2 reports$/);
    expect(within(group).getByTestId("issue-group-members-toggle")).toHaveAccessibleName("All 2 reports for support:H-1");
    const rows = memberRows(openReports(group), "INVENTED_CASE_LOAD_UNSET");
    expect(rows).toHaveLength(2);
    expect(rows[0]).toHaveTextContent("enter the LC-1 load on the support");
    expect(rows[1]).toHaveTextContent("enter the LC-2 load on the support");
    expect(rows.map((row) => within(row).getByTestId("issue-member-difference").textContent)).toEqual([
      "LC-1; enter the LC-1", "LC-2; enter the LC-2"
    ]);
    expect(classesOf(rows)).toEqual([null, null]);
    expect(notSuppliedOf(rows)).toEqual(["Not supplied: class, source, remediation, provenance", "Not supplied: class, source, remediation, provenance"]);
  });

  it("one code on one object for two missing fields: the closed item says two, the open one gives each field's action", () => {
    const forField = (id: string, field: string) =>
      diagnostic(id, "INVENTED_FIELD_MISSING", "blocking", `invented support:H-1 is missing ${field}; supply ${field} before solving`, "MISSING_INPUT");
    renderDiagnostics([forField("f1", "support.stiffness"), forField("f2", "support.gap")]);
    const group = screen.getByTestId("issue-group-INVENTED_FIELD_MISSING");
    expect(within(group).getByTestId("issue-group-count")).toHaveTextContent(/^2 reports$/);
    const rows = memberRows(openReports(group), "INVENTED_FIELD_MISSING");
    expect(rows).toHaveLength(2);
    expect(rows[0]).toHaveTextContent("supply support.stiffness before solving");
    expect(rows[1]).toHaveTextContent("supply support.gap before solving");
    expect(rows.map((row) => within(row).getByTestId("issue-member-difference").textContent)).toEqual([
      "support.stiffness; supply support.stiffness", "support.gap; supply support.gap"
    ]);
    expect(classesOf(rows)).toEqual(["class MISSING_INPUT", "class MISSING_INPUT"]);
  });
});

describe("no report that joins an item lowers it", () => {
  it("a warning item joined by a blocking report becomes blocking, in its row and in the drawer's headings", () => {
    renderDiagnostics([hanger("installed"), hanger("cold", "blocking")]);
    expect(screen.getByTestId("diagnostic-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toHaveClass("blocking");
    expect(screen.getByTestId("issue-group-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toHaveClass("blocking");
    expect(screen.getByTestId("diagnostic-severity-blocking")).toHaveTextContent("1 Blocking");
    expect(screen.getByTestId("diagnostic-severity-warning")).toHaveTextContent("0 Warnings");
  });

  it("a blocking item joined by an info report with no class stays blocking; that report's opened row shows no class and names it as not supplied", () => {
    const blocking = diagnostic("b1", "INVENTED_JOINED", "blocking", "invented blocking text on support:H-1", "MISSING_INPUT");
    const info = diagnostic("i1", "INVENTED_JOINED", "info", "invented info text on support:H-1");
    const items: IssueItem[] = groupIssues({ model, diagnostics: [blocking, info] });
    expect(items.map((item) => [item.severity, item.members.length])).toEqual([["blocking", 2]]);
    expect(countItemsBySeverity(items)).toEqual({ blocking: 1, error: 0, warning: 0, info: 0 });
    renderDiagnostics([blocking, info]);
    expect(screen.getByTestId("diagnostic-INVENTED_JOINED")).toHaveClass("blocking");
    expect(within(screen.getByTestId("diagnostic-INVENTED_JOINED")).getByTestId("issue-class")).toHaveTextContent(/^class MISSING_INPUT$/);
    expect(screen.getByTestId("diagnostic-severity-blocking")).toHaveTextContent("1 Blocking");
    expect(screen.getByTestId("diagnostic-severity-info")).toHaveTextContent("0 Info");
    const rows = memberRows(openReports(screen.getByTestId("issue-group-INVENTED_JOINED")), "INVENTED_JOINED");
    expect(rows.map((row) => row.querySelector(".issue-meta")?.textContent)).toEqual([
      expect.stringMatching(/^blocking · INVENTED_JOINED · class MISSING_INPUT · from the model$/), expect.stringMatching(/^info · INVENTED_JOINED · from the model$/)
    ]);
    expect(notSuppliedOf(rows)).toEqual(["Not supplied: source, remediation, provenance", "Not supplied: class, source, remediation, provenance"]);
  });
});


