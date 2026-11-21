import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { Diagnostic, PreviewModel } from "../../types";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { groupIssues, issueSummary, messageDifferences } from "./issueItems";
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
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("2 of 2 items shown (4 diagnostics)");
    expect(screen.getByTestId("diagnostic-severity-warning")).toHaveTextContent("1 Warnings");
    expect(screen.getAllByTestId("diagnostic-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toHaveLength(1);
    const group = screen.getByTestId("issue-group-INVENTED_HANGER_LOAD_NOT_PRELOAD");
    expect(within(group).getByTestId("issue-group-also")).toHaveTextContent(
      "Reported for each of: installed load (hanger.installed_load); cold load (hanger.cold_load); hot load (hanger.hot_load)."
    );
    const disclosure = within(group).getByTestId("issue-group-members") as HTMLDetailsElement;
    expect(disclosure.open).toBe(false);
    fireEvent.click(within(disclosure).getByText(/All 3 reports for support:H-1/));
    disclosure.open = true;
    expect(disclosure.open).toBe(true);
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
      "3 findings: 0 blocking or error, 3 warnings, 0 info. Listed as 1 item: a finding repeated on one object is one item."
    );
    expect(screen.getByRole("list", { name: "Model check findings" })).toHaveTextContent("Solve input, restraint and topology checks 1 item from 3 findings");
    expect(screen.getAllByTestId("model-check-INVENTED_HANGER_LOAD_NOT_PRELOAD")).toHaveLength(1);
  });
});

describe("classes stay apart (DEL-07-04 REQ-001)", () => {
  it("never makes one item of two classes, and lets a report without a class join its code on its object", () => {
    const classed = (cls: string, id: string): Diagnostic => ({ ...hanger("installed"), id, diagnostic_class: cls });
    const items = groupIssues({ model, diagnostics: [hanger("cold"), classed("ASSUMPTION_WARNING", "a"), classed("PROVENANCE_WARNING", "b"), hanger("hot")] });
    expect(items.map((item) => item.members.map((member) => member.diagnostic.id))).toEqual([
      ["diag:invented-hanger-cold", "a", "diag:invented-hanger-hot"],
      ["b"]
    ]);
  });
});
