// The Analyze page by removal: one status line in words and the solve controls lead; the
// memory settings and every record of the run sit unchanged inside two closed disclosures.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { loadPreviewModel } from "../../services/previewService";
import type { MechanicsResult, PoolStatus, PreviewModel } from "../../types";
import { numericalHealthView, type NumericalHealthState } from "../results/numericalHealth";
import { BOUNDARY_SENTENCE } from "../results/numericalHealthText";
import { initialSolveJob, requestSolveCancellation, resultTooLargeSolveJob, startSolveJob } from "../workspace/solveJobAudit";
import { RESULT_TOO_LARGE_CODE } from "../../services/previewService";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import { receivedFromText } from "../../test-support/receivedText";
import { SolvePanel } from "./SolvePanel";
import { statusLineHealth } from "./statusLineHealth";

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function solvedResult(): MechanicsResult {
  return JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json"), "utf8")) as MechanicsResult;
}

async function props() {
  const model = await loadPreviewModel();
  return { model, result: null, analysisRun: null, running: false, solveJob: initialSolveJob(),
    solverMode: "sparse_interactive" as const, onRun: vi.fn(), onCancel: vi.fn(), onSolverModeChange: vi.fn() };
}

const RECORD_IDS = [
  "solve-readiness-summary", "readiness-mechanics", "readiness-rule", "readiness-diagnostics", "readiness-professional",
  "solve-job-summary", "solve-job-audit", "solve-job-solver-mode", "solve-job-progress", "solve-job-cancellation",
  "solve-job-resource", "solve-job-binding", "solve-job-unit-policy", "solve-job-boundary"
];

it("leads after a solve with one status line in words, the solver choice and Run, and keeps both disclosures closed", async () => {
  const base = await props();
  const onOpenIssues = vi.fn();
  const result = solvedResult();
  render(<SolvePanel {...base} result={result} issueCount={30} onOpenIssues={onOpenIssues} />);

  const line = screen.getByTestId("solve-status-line");
  // The fixture's own quality status is sensitive: the line says so, with the boundary sentence.
  const rows = result.results.length;
  expect(rows).toBeGreaterThan(0);
  expect(screen.getByTestId("solve-status-text").textContent).toBe(
    `Solver · Mechanics solved: ${rows} result rows. Rule pack · Rule inputs incomplete. Numerical health: sensitive. Human · Human review required.`
  );
  expect(screen.getByTestId("solve-status-boundary")).toHaveTextContent(BOUNDARY_SENTENCE);
  // No status code in capitals on the line; each registered label keeps its domain, and its
  // token is one press away in place.
  expect(line.textContent).not.toMatch(/[A-Z]{2,}_[A-Z_]+/);
  for (const token of ["MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"]) {
    const label = screen.getByTestId(`solve-status-label-${token}`);
    const toggle = within(label).getByRole("button");
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(toggle);
    expect(within(label).getByTestId(`solve-status-token-${token}`)).toHaveTextContent(token);
    expect(toggle).toHaveAttribute("aria-expanded", "true");
  }
  fireEvent.click(within(line).getByRole("button", { name: "30 issues" }));
  expect(onOpenIssues).toHaveBeenCalledTimes(1);

  const choice = screen.getByRole("group", { name: "Solver mode" });
  const sparse = within(choice).getByRole("button", { name: "Sparse solver" });
  const dense = within(choice).getByRole("button", { name: "Dense solver" });
  expect(sparse).toHaveAttribute("aria-pressed", "true");
  expect(sparse.title).toMatch(/default/);
  expect(dense.title).toMatch(/full stiffness matrix/);
  expect(screen.getByRole("button", { name: "Run" })).toBeEnabled();
  expect(screen.queryByTestId("cancel-mechanics-preview")).toBeNull();

  const memory = screen.getByTestId("solve-memory-settings");
  const record = screen.getByTestId("solve-run-record");
  expect(memory).not.toHaveAttribute("open");
  expect(record).not.toHaveAttribute("open");
  expect(within(memory).getByText("Memory settings").tagName).toBe("SUMMARY");
  expect(within(record).getByText("Run record").tagName).toBe("SUMMARY");
  // The line and the controls come first; the disclosures last, Run record at the foot.
  expect(line.compareDocumentPosition(choice) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  expect(choice.compareDocumentPosition(memory) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  expect(memory.compareDocumentPosition(record) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  for (const id of RECORD_IDS) {
    expect(record).toContainElement(screen.getByTestId(id));
    expect(screen.getByTestId(id)).not.toBeVisible();
  }
  expect(memory).toContainElement(screen.getByTestId("memory-pool-panel"));
  expect(screen.getByTestId("memory-pool-panel")).not.toBeVisible();
});

it("shows Cancel only while a solve runs, and Run reads Running", async () => {
  const base = await props();
  const running = startSolveJob(base.model, { mode: "backend_job", job_id: "backend-solve-job-1", backend_cancellation_token: "token-1", state: "running", cancellation_scope: "worker_process_terminated", resource: null });
  const view = render(<SolvePanel {...base} running solveJob={running} />);
  // A job just started reports queued; the line says so in words.
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent("Waiting in the queue to solve.");
  expect(screen.getByTestId("run-mechanics-preview")).toHaveTextContent("Running");
  expect(screen.getByTestId("run-mechanics-preview")).toBeDisabled();
  const cancel = screen.getByRole("button", { name: "Cancel" });
  expect(cancel).toBeEnabled();
  expect(cancel).not.toHaveAttribute("aria-disabled");
  expect(screen.getByTestId("solve-job-cancellation")).toHaveTextContent("control_visible=true; enabled=true");
  fireEvent.click(cancel);
  expect(base.onCancel).toHaveBeenCalledTimes(1);
  // As the progress panel does (DEL-07-07 AC-002): focus moves to the state line.
  expect(screen.getByTestId("solve-status-line")).toHaveFocus();

  // While the cancellation is pending, Cancel stays in place, disabled, and the record says so.
  view.rerender(<SolvePanel {...base} running solveJob={requestSolveCancellation(running)} />);
  const pending = screen.getByRole("button", { name: "Cancel" });
  expect(pending).toHaveAttribute("aria-disabled", "true");
  fireEvent.click(pending);
  expect(base.onCancel).toHaveBeenCalledTimes(1);
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent("Cancelling the solve.");
  expect(screen.getByTestId("solve-job-cancellation")).toHaveTextContent("control_visible=true; enabled=false; requested=true");

  view.rerender(<SolvePanel {...base} />);
  expect(screen.queryByTestId("cancel-mechanics-preview")).toBeNull();
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent("Not solved yet.");
  expect(screen.getByTestId("solve-job-cancellation")).toHaveTextContent("control_visible=false");
});

it("reaches every record of the run, with its test id, once Run record is opened", async () => {
  render(<SolvePanel {...await props()} />);
  const record = screen.getByTestId("solve-run-record");
  fireEvent.click(within(record).getByText("Run record"));
  expect(record).toHaveAttribute("open");
  for (const id of RECORD_IDS) expect(screen.getByTestId(id)).toBeVisible();
  // The download, its private-values choice, its decisions line and its Details, unchanged.
  expect(screen.getByTestId("solve-job-export-link")).toBeVisible();
  expect(screen.getByTestId("solve-job-export-link-local-private-intent")).toBeVisible();
  expect(screen.getByTestId("solve-job-export-link-redaction-summary")).toHaveTextContent(/decisions=\d+; findings=\d+; blocked=true/);
  expect(within(screen.getByTestId("solve-job-export-link-redaction-details")).getByText("Details")).toBeVisible();
  fireEvent.click(screen.getByTestId("solve-job-export-link-local-private-intent"));
  expect(screen.getByTestId("solve-job-export-link")).toHaveAttribute("href", expect.stringMatching(/^data:application\/json/));
});

it("reaches the rule-pack references and the load/reference states of a run once Run record is opened", async () => {
  const request = receivedFromText<{ model: PreviewModel }>(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure/load_reference/pressure.request.json"), "utf8"));
  const result = receivedFromText<MechanicsResult>(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json"), "utf8"));
  // Invented identities: one rule-pack reference the run recorded.
  const manifest = {
    model_basis: { model_ref: "model:invented", model_payload: { ...request.model, rule_pack_refs: [{ rule_pack_id: "invented-pack", version: "1.0.0" }] } },
    active_rule_packs: [{ rule_pack_id: "invented-pack", version: "1.0.0", checksum_sha256: "a".repeat(64) }]
  } as unknown as CurrentSessionInputManifest;
  render(<SolvePanel {...await props()} model={request.model} result={result} inputManifest={manifest} />);
  const record = screen.getByTestId("solve-run-record");
  for (const id of ["solve-rule-pack-refs", "load-reference-states-block"]) {
    expect(record).toContainElement(screen.getByTestId(id));
    expect(screen.getByTestId(id)).not.toBeVisible();
  }
  fireEvent.click(within(record).getByText("Run record"));
  expect(screen.getByTestId("solve-rule-pack-refs")).toBeVisible();
  expect(screen.getByTestId("solve-rule-pack-refs-recorded")).toHaveTextContent("invented-pack v1.0.0 (sha256 aaaaaaaaaaaa…)");
  expect(screen.getByTestId("load-reference-states-block")).toBeVisible();
  expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary pressure-3");
});

it("says Starting before the job reports, and claims no solve for a result refused for its size", async () => {
  const base = await props();
  // The Solve request's model checks run first: no job has reported yet.
  const view = render(<SolvePanel {...base} running solveJob={initialSolveJob()} />);
  expect(screen.getByTestId("solve-status-text")).toHaveTextContent(/^Starting\.$/);
  expect(screen.getByTestId("solve-status-text")).not.toHaveTextContent(/Solving|Solved/);

  const started = startSolveJob(base.model, { mode: "backend_job", job_id: "backend-solve-job-2", backend_cancellation_token: "token-2", state: "running", cancellation_scope: "worker_process_terminated", resource: null });
  const refused = resultTooLargeSolveJob(started, { code: RESULT_TOO_LARGE_CODE, result_bytes: 300 * 1024 ** 2, limit_bytes: 256 * 1024 ** 2 });
  view.rerender(<SolvePanel {...base} solveJob={refused} />);
  const line = screen.getByTestId("solve-status-text").textContent ?? "";
  expect(line).toMatch(/^Refused: the result is larger than the desktop can show, so none was kept\./);
  expect(line).not.toMatch(/solved/i);
});

it("carries the boundary sentence beside the Run record's clean numerical-health badge", async () => {
  render(<SolvePanel {...await props()} result={withDocument("checks_passed")} numericalHealth={health("checks_passed")} />);
  const record = screen.getByTestId("solve-run-record");
  fireEvent.click(within(record).getByText("Run record"));
  expect(screen.getByTestId("solve-numerical-health-badge").textContent).toBe("Numerical health: solver checks found no problem");
  expect(screen.getByTestId("solve-numerical-health-badge")).toBeVisible();
  const boundary = screen.getByTestId("solve-run-record-health-boundary");
  expect(boundary).toBeVisible();
  expect(boundary).toHaveTextContent(BOUNDARY_SENTENCE);
  expect(screen.getByTestId("solve-readiness-summary")).toContainElement(boundary);
  // "checks passed" appears nowhere on the page, with Run record open.
  expect(screen.getByTestId("solve-panel").textContent).not.toContain("checks passed");
});

it("counts passing load cases as no problem found beside the others in the Run record badge", async () => {
  const mixed = numericalHealthView({ route: "generic", verdict_only: false, combinations: [], cases: [
    { case_id: "a", state: "checks_passed", code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] },
    { case_id: "b", state: "sensitive", code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] }
  ] }, { eligible: false, verdictHold: false });
  render(<SolvePanel {...await props()} result={withDocument("sensitive")} numericalHealth={mixed} />);
  fireEvent.click(within(screen.getByTestId("solve-run-record")).getByText("Run record"));
  expect(screen.getByTestId("solve-numerical-health-badge").textContent).toBe("Numerical health: 1 sensitive, 1 no problem found");
  expect(screen.getByTestId("solve-panel").textContent).not.toContain("checks passed");
});

const GiB = 1024 ** 3;
function poolStatus(overrides: Partial<PoolStatus> = {}): PoolStatus {
  // Mocked native pool readings; no figure here is a reading of any machine.
  return {
    capacity_bytes: 64 * GiB, recommended_pool_bytes: 48 * GiB, pool_bytes: 48 * GiB, pool_above_recommended: false,
    invocation_budget_bytes: 12 * GiB, default_invocation_budget_bytes: 12 * GiB, max_invocation_budget_bytes: 48 * GiB,
    reserved_bytes: 0, reserved_by_purpose: { ingress: 0, worker: 0, result: 0, custody: 0 },
    running_workers: 0, limits: { max_running_workers: 8 }, queued_jobs: 0, running_jobs: 0, retained_results: 0, webview_custody_results: 0,
    headroom: { available_bytes: 20 * GiB, pressure: "normal", sample_age_ms: 120 }, ordinary_route_pricing: "worker_capped", last_refusal: null,
    ...overrides
  };
}

it("keeps the memory inputs working as before once Memory settings is opened, each with the line saying what it limits", async () => {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args?: { poolBytes: number; invocationBudgetBytes: number }) => {
    if (command === "get_memory_pool_status") return poolStatus();
    if (command === "configure_memory_pool") return poolStatus({ pool_bytes: args!.poolBytes, max_invocation_budget_bytes: args!.poolBytes, invocation_budget_bytes: args!.invocationBudgetBytes });
    throw new Error(`unexpected ${command}`);
  });
  render(<SolvePanel {...await props()} />);
  const memory = screen.getByTestId("solve-memory-settings");
  await screen.findByTestId("memory-pool-status");
  expect(screen.getByLabelText("Memory pool (GiB)")).not.toBeVisible();
  fireEvent.click(within(memory).getByText("Memory settings"));
  expect(memory).toHaveAttribute("open");

  const pool = screen.getByLabelText("Memory pool (GiB)");
  const budget = screen.getByLabelText("Memory per solve (GiB)");
  expect(pool).toBeVisible();
  expect(pool).toHaveValue(48);
  expect(budget).toHaveValue(12);
  expect(pool).toHaveAccessibleDescription("The most memory all solves and the app's other work may reserve at once. Recommended for this Mac: 48 GiB.");
  expect(budget).toHaveAccessibleDescription("The per-invocation budget: the most memory one solve may use; a solve that needs more is refused. Default: 12 GiB.");
  fireEvent.change(pool, { target: { value: "24" } });
  fireEvent.change(budget, { target: { value: "8" } });
  fireEvent.click(screen.getByRole("button", { name: "Apply" }));
  expect(await screen.findByTestId("memory-pool-outcome")).toHaveTextContent("Applied: pool 24 GiB; per-invocation budget 8 GiB.");
  expect(invokeMock).toHaveBeenCalledWith("configure_memory_pool", { poolBytes: 24 * GiB, invocationBudgetBytes: 8 * GiB, acknowledgeAboveRecommended: false });
});

// DEL-07-07 REQ-007 / AC-005, DEL-09-05 REQ-004 and DEL-07-05 REQ-023 on the status line,
// read without opening Run record.
const EXCLUDED = /checks passed|verified|certified|\bbound\b|guaranteed|\bvalid\b|approved|compliant|correct|reliable/i;

function health(state: NumericalHealthState) {
  return numericalHealthView({ route: "generic", verdict_only: false, combinations: [], cases: [
    { case_id: "case", state, code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] }
  ] }, { eligible: false, verdictHold: false });
}

function withDocument(status: string, mechanics = "MECHANICS_SOLVED"): MechanicsResult {
  const result = solvedResult();
  return { ...result, status: { ...result.status, mechanics }, numerical_quality: { ...result.numerical_quality!, status: status as never } };
}

function lineOf() {
  return screen.getByTestId("solve-status-text").textContent ?? "";
}

it("on a clean solved result states the outcome, the rule status and human review, and no numerical-health verdict", async () => {
  render(<SolvePanel {...await props()} result={withDocument("checks_passed")} numericalHealth={health("checks_passed")} />);
  expect(lineOf()).toBe(`Solver · Mechanics solved: ${solvedResult().results.length} result rows. Rule pack · Rule inputs incomplete. Human · Human review required.`);
  expect(lineOf()).not.toMatch(/Numerical health/);
  // "checks passed" appears nowhere on the page, the closed Run record included.
  expect(screen.getByTestId("solve-panel").textContent).not.toContain("checks passed");
  expect(screen.queryByTestId("solve-status-boundary")).toBeNull();
  expect(screen.getByTestId("solve-status-line")).toBeVisible();
});

it.each([
  ["sensitive", "sensitive", "Numerical health: sensitive."],
  ["unresolved", "quality_unresolved", "Numerical health: quality unresolved."],
  ["failed", "solve_failed", "Numerical health: solve failed."],
  ["not_assessed", "not_assessed", "Numerical health: not assessed."],
  ["not_assessed", null, "Numerical health: not assessed."]
] as const)("on a solved result whose health is %s (%s) shows its verdict, the boundary sentence and human review without a press", async (document, gated, words) => {
  render(<SolvePanel {...await props()} result={withDocument(document)} numericalHealth={gated ? health(gated) : null} />);
  expect(lineOf()).toContain(words);
  expect(lineOf()).toMatch(/Human · Human review required\.$/);
  expect(lineOf()).not.toMatch(EXCLUDED);
  expect(screen.getByTestId("solve-status-line")).toBeVisible();
  expect(screen.getByTestId("solve-status-boundary")).toBeVisible();
  expect(screen.getByTestId("solve-status-boundary")).toHaveTextContent(BOUNDARY_SENTENCE);
  expect(screen.getByTestId("solve-run-record")).not.toHaveAttribute("open");
});

it.each([
  ["document checks passed, gated sensitive", withDocument("checks_passed"), "sensitive", "Numerical health: sensitive."],
  ["document checks passed, no gated reading", withDocument("checks_passed"), null, "Numerical health: not assessed."],
  ["document sensitive, gated checks passed", withDocument("sensitive"), "checks_passed", "Numerical health: sensitive."],
  ["document unresolved, gated checks passed", withDocument("unresolved"), "checks_passed", "Numerical health: quality unresolved."],
  ["mechanics not solved, document and gated checks passed", withDocument("checks_passed", "MODEL_INCOMPLETE"), "checks_passed", "Numerical health: not assessed."],
  ["mechanics not solved, document sensitive, no gated reading", withDocument("sensitive", "MODEL_INCOMPLETE"), null, "Numerical health: not assessed."],
  ["mechanics not solved, document sensitive, gated not assessed", withDocument("sensitive", "MODEL_INCOMPLETE"), "not_assessed", "Numerical health: not assessed."],
  ["mechanics not solved, document failed", withDocument("failed", "MODEL_INCOMPLETE"), "not_assessed", "Numerical health: solve failed."],
  ["mechanics not solved, gated quality unresolved", withDocument("checks_passed", "MODEL_INCOMPLETE"), "quality_unresolved", "Numerical health: quality unresolved."],
  ["document failed, gated checks passed", withDocument("failed"), "checks_passed", "Numerical health: solve failed."]
] as const)("takes the worse of the gated health and the document status: %s", async (_label, result, gated, words) => {
  render(<SolvePanel {...await props()} result={result} numericalHealth={gated ? health(gated) : null} />);
  expect(lineOf()).toContain(words);
  // An unsolved model never reads sensitive or checks passed.
  if (result.status.mechanics !== "MECHANICS_SOLVED") expect(lineOf()).not.toMatch(/sensitive|checks passed/);
  expect(lineOf()).not.toMatch(EXCLUDED);
  expect(screen.getByTestId("solve-status-boundary")).toHaveTextContent(BOUNDARY_SENTENCE);
});

it("shows invalid input and an incomplete model as their own outcomes, each with human review, without a press", async () => {
  const base = await props();
  const view = render(<SolvePanel {...base} solveGate={{ kind: "withheld", model: base.model, findings: [], diagnostics: [], notListedBlocking: 0, refusal: "invented refusal text", laterCheckNotCompleted: null }} />);
  expect(lineOf()).toBe("Not solved: the solve cannot read this model. Human · Human review required.");
  expect(screen.getByTestId("solve-status-line")).toBeVisible();
  view.unmount();

  const incomplete = { ...base.model, analysis_status: { ...base.model.analysis_status, mechanics: "MODEL_INCOMPLETE" } };
  render(<SolvePanel {...base} model={incomplete} />);
  // The registered label keeps its exact form and its domain.
  expect(lineOf()).toBe("Not solved: Solver · Model incomplete. Human · Human review required.");
  expect(screen.getByTestId("solve-status-line")).toBeVisible();
  expect(screen.queryByTestId("solve-status-boundary")).toBeNull();
});

it("states a private rule check's recorded status on the line, not the solve-time rule status", async () => {
  const base = await props();
  const view = render(<SolvePanel {...base} result={withDocument("checks_passed")} numericalHealth={health("checks_passed")} ruleCheckAggregate="USER_RULE_CHECKED" />);
  expect(lineOf()).toContain("Rule pack · User-rule checked.");
  expect(lineOf()).not.toContain("Rule inputs incomplete");
  view.rerender(<SolvePanel {...base} result={withDocument("checks_passed")} numericalHealth={health("checks_passed")} ruleCheckAggregate="USER_RULE_FAILED" />);
  expect(lineOf()).toContain("Rule pack · User-rule failed.");
  view.rerender(<SolvePanel {...base} result={withDocument("checks_passed")} numericalHealth={health("checks_passed")} ruleCheckAggregate={null} />);
  expect(lineOf()).toContain("Rule pack · Rule inputs incomplete.");
});

it("reads a native handle's health from its gated reading alone", () => {
  // A native handle carries no document status; its gated reading stands.
  expect(statusLineHealth({ mechanics: "MECHANICS_SOLVED", documentStatus: "sensitive", native: true }, health("checks_passed"))).toBe("checks_passed");
  expect(statusLineHealth({ mechanics: "MECHANICS_SOLVED", native: true }, health("sensitive"))).toBe("sensitive");
  expect(statusLineHealth({ mechanics: "MECHANICS_SOLVED", native: true }, null)).toBe("not_assessed");
  expect(statusLineHealth({ mechanics: "MODEL_INCOMPLETE", native: true }, health("sensitive"))).toBe("not_assessed");
  expect(statusLineHealth({ mechanics: "MODEL_INCOMPLETE", native: true }, health("solve_failed"))).toBe("solve_failed");
  // The same document status does count on a full document.
  expect(statusLineHealth({ mechanics: "MECHANICS_SOLVED", documentStatus: "sensitive", native: false }, health("checks_passed"))).toBe("sensitive");
});
