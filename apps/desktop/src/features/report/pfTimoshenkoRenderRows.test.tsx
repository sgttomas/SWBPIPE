/** DEL-08-01 REQ-024 and DEL-05-03 Design `pf-timoshenko.md` §4: the rendered report's request for
 * a Current pressure-free Timoshenko result carries exactly its source rows' projection, which its
 * native render requires (`report_render_exact.rs`, `rederive_pf_timoshenko`, refusing
 * `result_rows` otherwise), and states each load case's numerical health only through the standing
 * record, which that render derives again. The generic `numerical-health:<case>` rows are lines the
 * web view writes and no boundary derives again; the pressure-free Euler–Bernoulli report, which
 * carries no standing record, keeps them. The panel is mounted as App mounts it for a Current
 * result, with the numerical health App passes it (`App.tsx`, `numericalHealth={currentSolvedResult
 * ? numericalHealth : null}`). The native refusal of an appended row, and the rendered health by
 * load case, are in `apps/desktop/src-tauri/src/report_render_pf_timoshenko_tests.rs`.
 *
 * The Timoshenko result is the product's own committed envelope for its frozen request
 * (`core/product_physics/tests/pf_timoshenko_envelopes.rs` keeps it current); the Euler–Bernoulli
 * one is the replay pair's. Each is solved through the unit solve-job transport, and the render is
 * captured, not run. Unit transport replay through mocked IPC; NOT a native UI qualification
 * witness. Invented ids. */
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { numericalHealthView, readNumericalHealth } from "../results/numericalHealth";
import { numericalResultStanding } from "../results/resultStanding";
import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import { RenderedReportPanel } from "./RenderedReportPanel";
import { reportResultRows } from "./reportResultRows";
import productStraight from "../../../../../fixtures/results/preview_physics_timoshenko_straight_sparse.json?raw";
import straightRequest from "../../../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const SOLVER = { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} };
const HEALTH_ROW = "numerical-health:";

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});

/** An unblocked outcome for the captured render (the render itself is not run here). */
function renderedOutcome() {
  return {
    html: "<!DOCTYPE html>\n<html><body>invented rendered report</body></html>",
    sha256_hex: "a".repeat(64),
    export_blocked: false,
    blocking_reasons: [],
    report_validation_diagnostics: [],
    section_validation_diagnostics: [],
    pre_render_findings: [],
    post_render_findings: [],
    derived_print_html: "<div>DERIVED VIEW aaaa</div>"
  };
}

/** A Current session over `bytes` for `model`, its record and manifest, and the numerical health
 * App passes the rendered report panel for a Current result. */
async function current(model: PreviewModel, bytes: string, jobIdPrefix: string) {
  (window as Json).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport(() => bytes, { jobIdPrefix });
  const renders: Json[] = [];
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "render_calculation_report") { renders.push(args); return renderedOutcome(); }
    if (command === "retire_rendered_report") return null;
    return transport.invoke(command, args, options);
  });
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: SOLVER, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  const standing = numericalResultStanding(result, model);
  expect(standing.eligible).toBe(true);
  const numericalHealth = numericalHealthView(readNumericalHealth(result as MechanicsResult, standing.findings), { eligible: standing.eligible, verdictHold: false });
  return { model, result, inputManifest, analysisRun, numericalHealth, renders };
}

/** The render request the panel sends when the person presses Render. The rendering is the
 * local-private one: without that stated intent the route is blocked before any request. */
async function renderedInput(session: Awaited<ReturnType<typeof current>>): Promise<Json> {
  const { model, result, analysisRun, inputManifest, numericalHealth, renders } = session;
  render(<RenderedReportPanel model={model} result={result} numericalHealth={numericalHealth} analysisRun={analysisRun} projectSummary={null} inputManifest={inputManifest} />);
  fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
  fireEvent.click(screen.getByTestId("rendered-report-render"));
  await waitFor(() => expect(renders).toHaveLength(1));
  return (renders[0] as Json).input;
}

it("an eligible pressure-free Timoshenko result's render request carries its source rows' projection and no numerical-health row", async () => {
  const model = structuredClone((straightRequest as Json).model) as Json;
  model.project.name = model.project.id;
  for (const loadCase of model.load_cases) loadCase.status = "preview_only";
  const session = await current(model as PreviewModel, productStraight, "invented-pf-timoshenko-render-rows-job");
  expect(session.result.producer?.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
  // App passes this health for a Current result; on the preview route it reads generic.
  expect(session.numericalHealth?.route).toBe("generic");
  expect(session.numericalHealth!.cases.length).toBeGreaterThan(0);
  const input = await renderedInput(session);
  const rows = input.result_rows as Json[];
  expect(rows.filter((row) => String(row.row_id).startsWith(HEALTH_ROW)).map((row) => row.row_id)).toEqual([]);
  expect(rows).toEqual(reportResultRows(session.result));
  // Each load case's health reaches the report through the standing record the native render
  // derives again, one entry per load case.
  expect(input.numerical_standing.cases).toHaveLength(session.numericalHealth!.cases.length);
});

it("an eligible pressure-free Euler–Bernoulli result's render request keeps its numerical-health rows", async () => {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const session = await current(structuredClone(pair.model) as PreviewModel, pair.bytes, "invented-pf-euler-bernoulli-render-rows-job");
  expect(session.result.producer?.semantic_contract_id).toBe(PREVIEW_PHYSICS_CONTRACT_ID);
  expect(session.numericalHealth?.route).toBe("generic");
  const input = await renderedInput(session);
  const rows = input.result_rows as Json[];
  const projection = reportResultRows(session.result);
  expect(rows.slice(0, projection.length)).toEqual(projection);
  expect(rows.slice(projection.length).map((row) => row.row_id)).toEqual(session.numericalHealth!.cases.map((item) => `${HEALTH_ROW}${item.caseId}`));
  expect("numerical_standing" in input).toBe(false);
});
