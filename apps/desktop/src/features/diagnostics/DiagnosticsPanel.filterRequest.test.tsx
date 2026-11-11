// DEL-07-05 numerical health §3: "Show solver diagnostic" opens the Diagnostics panel
// filtered to exactly that diagnostic, once. The request is cleared when the drawer
// closes and when the result or the run changes; a later ordinary opening is unfiltered.
import { act, fireEvent, render, renderHook, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { useDiagnosticFilterRequest } from "./diagnosticFilterRequest";
import type { MechanicsResult, PreviewModel } from "../../types";

const model = { diagnostics: [] } as unknown as PreviewModel;
const result = { run_id: "run:invented", summary: {}, results: [], diagnostics: [
  { id: "diagnostic:numerical-integrity:LC1", code: "NUMERICAL_INTEGRITY_SENSITIVE", severity: "warning", message: "a", affected_refs: ["LC1"] },
  { id: "diagnostic:numerical-integrity:LC10", code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", severity: "info", message: "b", affected_refs: ["LC10"] },
  { id: "diagnostic:other", code: "OTHER", severity: "info", message: "c", affected_refs: [] }
] } as unknown as MechanicsResult;
const panel = (filterRequest: { diagnosticId: string; sequence: number } | null) => (
  <DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={null} filterRequest={filterRequest} onSelectDiagnostic={() => {}} />
);

describe("DiagnosticsPanel filter request", () => {
  it("shows exactly the requested diagnostic (…:LC1 does not match …:LC10)", () => {
    render(panel({ diagnosticId: "diagnostic:numerical-integrity:LC1", sequence: 1 }));
    expect(screen.getByTestId("diagnostic-filter-input")).toHaveValue("diagnostic:numerical-integrity:LC1");
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("1 of 3 diagnostics visible");
    expect(screen.getByTestId("diagnostic-NUMERICAL_INTEGRITY_SENSITIVE")).toBeInTheDocument();
    expect(screen.queryByTestId("diagnostic-NUMERICAL_INTEGRITY_CHECKS_PASSED")).toBeNull();
    // Typing returns to the ordinary substring filter.
    fireEvent.change(screen.getByTestId("diagnostic-filter-input"), { target: { value: "numerical-integrity" } });
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("2 of 3 diagnostics visible");
  });

  it("applies a later request again and clears its filter when the request is cleared", () => {
    const view = render(panel({ diagnosticId: "diagnostic:numerical-integrity:LC1", sequence: 1 }));
    fireEvent.change(screen.getByTestId("diagnostic-filter-input"), { target: { value: "" } });
    view.rerender(panel({ diagnosticId: "diagnostic:numerical-integrity:LC1", sequence: 2 }));
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("1 of 3 diagnostics visible");
    view.rerender(panel(null));
    expect(screen.getByTestId("diagnostic-filter-input")).toHaveValue("");
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("3 of 3 diagnostics visible");
  });

  it("opens unfiltered when the drawer remounts after the request was consumed", () => {
    const first = render(panel({ diagnosticId: "diagnostic:numerical-integrity:LC1", sequence: 1 }));
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("1 of 3 diagnostics visible");
    first.unmount();
    render(panel(null));
    expect(screen.getByTestId("diagnostic-filter-input")).toHaveValue("");
    expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("3 of 3 diagnostics visible");
  });
});

describe("useDiagnosticFilterRequest", () => {
  it("is cleared when the drawer closes, and when the result or the run changes", () => {
    const run: unknown = { id: 1 }, nextRun: unknown = { id: 2 }, nextResult: unknown = { id: "r2" };
    const hook = renderHook((props: { issuesDrawerOpen: boolean; result: unknown; analysisRun: unknown }) => useDiagnosticFilterRequest(props),
      { initialProps: { issuesDrawerOpen: false, result: result as unknown, analysisRun: run } });
    act(() => hook.result.current.requestDiagnosticFilter("diagnostic:numerical-integrity:LC1"));
    hook.rerender({ issuesDrawerOpen: true, result, analysisRun: run });
    expect(hook.result.current.filterRequest).toMatchObject({ diagnosticId: "diagnostic:numerical-integrity:LC1" });
    // Drawer closed: consumed, so a later ordinary opening has no request.
    hook.rerender({ issuesDrawerOpen: false, result, analysisRun: run });
    expect(hook.result.current.filterRequest).toBeNull();
    hook.rerender({ issuesDrawerOpen: true, result, analysisRun: run });
    expect(hook.result.current.filterRequest).toBeNull();
    // A run change clears it.
    act(() => hook.result.current.requestDiagnosticFilter("diagnostic:numerical-integrity:LC1"));
    expect(hook.result.current.filterRequest).not.toBeNull();
    hook.rerender({ issuesDrawerOpen: true, result, analysisRun: nextRun });
    expect(hook.result.current.filterRequest).toBeNull();
    // A result change clears it.
    act(() => hook.result.current.requestDiagnosticFilter("diagnostic:numerical-integrity:LC1"));
    hook.rerender({ issuesDrawerOpen: true, result: nextResult, analysisRun: nextRun });
    expect(hook.result.current.filterRequest).toBeNull();
  });

  it("gives each request a new sequence", () => {
    const hook = renderHook(() => useDiagnosticFilterRequest({ issuesDrawerOpen: true, result: null, analysisRun: null }));
    act(() => hook.result.current.requestDiagnosticFilter("a"));
    const first = hook.result.current.filterRequest!.sequence;
    act(() => hook.result.current.requestDiagnosticFilter("a"));
    expect(hook.result.current.filterRequest!.sequence).toBeGreaterThan(first);
  });
});
