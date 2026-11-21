// R12: no two numbers on screen read as a disagreement. On one fixture with repeats, model-check
// items, a solve report and a blocking missing-data entry, every count the drawer, the line's
// Details and the Analyze row show names its subject: "items" for what the Issues drawer lists
// (a code repeated on one object is one item), "reports" for the diagnostics before grouping, and
// missing data apart. The parts add up to the Issues total. Invented diagnostics.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { expect, it } from "vitest";
import type { AnalysisRunEnvelope, Diagnostic, MechanicsResult } from "../../types";
import { loadPreviewModel } from "../../services/previewService";
import { countMissingDataBlockers, MissingDataBlockingPanel } from "../missing-data/MissingDataBlockingPanel";
import { SolvePanel } from "../solve/SolvePanel";
import { SolveProgressPanel } from "../solve/SolveProgressPanel";
import { completeSolveJob, initialSolveJob, progressSolveJob, startSolveJob } from "../workspace/solveJobAudit";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { issueSummary, itemsAt } from "./issueItems";
import type { ModelCheckFinding } from "./modelChecks";
import { ModelChecksPanel } from "./ModelChecksPanel";

it("every count names what it counts, and the parts add up to the Issues total", async () => {
  const base = await loadPreviewModel();
  const support = base.supports[0].id;
  const repeat = (load: string): Diagnostic => ({ id: `diag:invented-${load}`, code: "INVENTED_REPEAT", severity: "warning", message: `invented ${load} load on ${support}`, affected_refs: [support] });
  const model = { ...base, diagnostics: [repeat("installed"), repeat("cold"), repeat("hot")] };
  const solveReport: Diagnostic = { id: "diag:invented-solve", code: "INVENTED_SOLVE_REPORT", severity: "info", message: "invented solve text", affected_refs: [support] };
  const checkFinding = (code: string): ModelCheckFinding => ({ key: code, group: "solve", entity: { type: "support", id: support }, diagnostic: { id: code, code, severity: "warning", message: `invented ${code}`, affected_refs: [support] } });
  const findings = [checkFinding("INVENTED_CHECK_A"), checkFinding("INVENTED_CHECK_B")];
  const missingInputs = countMissingDataBlockers({ model, result: null });
  expect(missingInputs).toBeGreaterThan(0);
  const summary = issueSummary({
    model,
    modelChecks: findings,
    diagnostics: [...model.diagnostics, solveReport],
    diagnosticOrigins: ["model", "model", "model", "solve"],
    missingInputs
  });
  const underChecks = itemsAt(summary, "model-checks");
  const underDiagnostics = itemsAt(summary, "diagnostics");
  // 2 check items + (1 repeated item + 1 solve item) + the blocking missing-data entries.
  expect(underChecks).toHaveLength(2);
  expect(underDiagnostics).toHaveLength(2);
  expect(summary.total).toBe(underChecks.length + underDiagnostics.length + missingInputs);

  // The Analyze row: items, of what, and missing data in its own words.
  render(<SolvePanel analysisRun={null} model={model} result={null} running={false} solveJob={initialSolveJob()} solverMode="sparse_interactive"
    diagnosticItems={summary.items} onRun={() => {}} onSolverModeChange={() => {}} />);
  expect(screen.getByTestId("readiness-diagnostics")).toHaveTextContent(
    `2 items from the model and the solve; 1 warning; 0 blocking or error. Missing data, listed apart in Issues: ${missingInputs} blocking`
  );
  cleanup();

  // The Diagnostics list: items shown, and the reports they gather where that differs.
  render(<DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} items={underDiagnostics} allItems={summary.items} />);
  expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent(/^2 of 2 items shown \(4 reports\)$/);
  expect(screen.getByTestId("diagnostics-item-count")).toHaveAccessibleName("2 items");
  cleanup();

  // Model checks: items at their severity.
  const run = { model, solve: { kind: "checked" as const }, findings, notShown: { solve: 0, slots: 0, library: 0 }, notShownBlocking: 0, notShownBlockingByGroup: { solve: 0, slots: 0, library: 0 } };
  render(<ModelChecksPanel model={model} run={run} items={underChecks} onRun={() => {}} onSelectEntity={() => {}} />);
  expect(screen.getByTestId("model-checks-summary")).toHaveTextContent(/^2 items: 0 blocking or error, 2 warnings, 0 info\.$/);
  expect(screen.getByTestId("model-checks-item-count")).toHaveAccessibleName("2 items");
  cleanup();

  // Missing data: the blocking entries the total counts, in its heading.
  render(<MissingDataBlockingPanel model={model} result={null} />);
  expect(screen.getByTestId("missing-data-blocking-count")).toHaveAccessibleName(`${missingInputs} blocking ${missingInputs === 1 ? "entry" : "entries"}`);
  cleanup();

  // The line's Details: the job's own counts, as reports.
  const job = progressSolveJob(startSolveJob(model, { mode: "backend_job", job_id: "invented-count-job", backend_cancellation_token: "t", state: "queued", cancellation_scope: "worker_process_terminated" } as never),
    { job_id: "invented-count-job", state: "running", queue_reason: null, cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "worker_process_terminated", result_available: false, result_bytes: null, error_message: null, resource_refusal: null } as never);
  const done = completeSolveJob(job, { results: [{}, {}], diagnostics: [solveReport, solveReport, solveReport, solveReport, solveReport] } as unknown as MechanicsResult,
    { analysis_run: { run_id: "analysis-run:invented-counts", analysis_status: ["MECHANICS_SOLVED"] } } as unknown as AnalysisRunEnvelope);
  const view = render(<SolveProgressPanel solveJob={initialSolveJob()} running={false} onCancel={() => {}} />);
  view.rerender(<SolveProgressPanel solveJob={initialSolveJob()} running onCancel={() => {}} />);
  view.rerender(<SolveProgressPanel solveJob={done} running={false} onCancel={() => {}} />);
  fireEvent.click(screen.getByTestId("solve-progress-details-toggle"));
  expect(screen.getByTestId("solve-progress-details")).not.toHaveAttribute("hidden");
  expect(screen.getByTestId("solve-progress-completed-counts")).toHaveTextContent(/^The job reported 2 result rows and 5 diagnostic reports\.$/);
});
