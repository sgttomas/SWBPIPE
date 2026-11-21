// The Analyze page's diagnostics row counts the Issues drawer's items that hold a model or
// solve report, whatever list the drawer files them under, at each item's own (worst)
// severity. Invented models and diagnostics only.
import { cleanup, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { Diagnostic, PreviewModel } from "../../types";
import { groupIssues } from "../diagnostics/issueItems";
import type { ModelCheckFinding } from "../diagnostics/modelChecks";
import { loadPreviewModel } from "../../services/previewService";
import { initialSolveJob } from "../workspace/solveJobAudit";
import { SolvePanel } from "./SolvePanel";

const report = (id: string, code: string, severity: Diagnostic["severity"], ref: string): Diagnostic => ({
  id, code, severity, message: `invented ${code} text on ${ref}`, affected_refs: [ref]
});
const checked = (diagnostic: Diagnostic): ModelCheckFinding => ({ key: diagnostic.id ?? diagnostic.code, group: "solve", diagnostic, entity: null });

async function renderRow(items: ReturnType<typeof groupIssues>, model: PreviewModel) {
  render(<SolvePanel analysisRun={null} model={model} result={null} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive"
    diagnosticItems={items} onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
  return screen.getByTestId("readiness-diagnostics");
}

describe("the Analyze page's diagnostics row", () => {
  it("counts a solve diagnostic the model checks also found, says how many are listed under Model checks, and takes its tone from it", async () => {
    const model = await loadPreviewModel();
    const solved = report("s1", "INVENTED_SHARED", "blocking", "support:H-1");
    const items = groupIssues({
      modelChecks: [checked({ ...solved, id: "c1", severity: "warning" }), checked(report("c2", "INVENTED_CHECK_ONLY", "error", "support:H-2"))],
      diagnostics: [solved, report("m1", "INVENTED_MODEL", "warning", "support:H-3"), report("k1", "INVENTED_KNOWLEDGE", "error", "support:H-4")],
      diagnosticOrigins: ["solve", "model", "design knowledge"]
    });
    expect(items.map((item) => [item.code, item.home, item.severity])).toEqual([
      ["INVENTED_SHARED", "model-checks", "blocking"],
      ["INVENTED_CHECK_ONLY", "model-checks", "error"],
      ["INVENTED_MODEL", "diagnostics", "warning"],
      ["INVENTED_KNOWLEDGE", "diagnostics", "error"]
    ]);
    const row = await renderRow(items, model);
    expect(row).toHaveTextContent("Model and solve diagnostics2 diagnostics (1 listed under Model checks); 1 warning; 1 blocking/error");
    expect(row).toHaveClass("blocking");
  });

  it("gives no parenthesis where none is listed under Model checks, and reads clear on a clear model", async () => {
    const model = await loadPreviewModel();
    const warned = await renderRow(groupIssues({ diagnostics: [report("m1", "INVENTED_MODEL", "warning", "support:H-3")], diagnosticOrigins: ["model"] }), model);
    expect(warned).toHaveTextContent(/1 diagnostic; 1 warning; 0 blocking\/error$/);
    expect(warned).toHaveClass("warning");
    cleanup();
    const clear = await renderRow([], model);
    expect(clear).toHaveTextContent(/0 diagnostics; 0 warnings; 0 blocking\/error$/);
    expect(clear).toHaveClass("ok");
  });
});
