/** DEL-08-01 REQ-024's connecting path, desktop end: two exact-route results that are
 * solved but not numerically eligible (`fixtures/reports/invented/report_standing_witnesses/`,
 * written by `core/product_physics/tests/report_standing_witnesses.rs`) go through the
 * production report build step (`assembleRenderedReport`) under the native-capture test seams.
 *
 * - The rendered report admits them (readable and solved), with the standing record the
 *   one Rust reader derives, under the historical vocabularies.
 * - The report package still refuses them (DEL-08-01 REQ-024's present account; LIM-016).
 * - The render arguments the desktop would send the native host are committed beside the
 *   results (`<name>.render.json`; stale fails; `OPENPIPESTRESS_WRITE_REPORT_STANDING_RENDER_INPUTS=<dir>`
 *   writes them), and the native host's tests render them.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. Job ids are invented; case labels and project names are added
 * where a committed request has none. Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { numericalResultStanding } from "../results/resultStanding";
import { readerReportStanding } from "../results/resultReader";
import { assembleRenderedReport } from "./renderedReportAssembly";
import { buildReportPackageRequest } from "./reportPackageRequest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => {
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = (path: string) => JSON.parse(text(path));
const DIR = "fixtures/reports/invented/report_standing_witnesses";
const GENERATOR = "apps/desktop/src/features/report/reportStandingWitness.test.tsx";
const WRITE = "OPENPIPESTRESS_WRITE_REPORT_STANDING_RENDER_INPUTS";

type Shape = { name: string; rawPath: string; requestPath: string };
const SHAPES: readonly Shape[] = [
  { name: "pressure_3_gap_unresolved", rawPath: `${DIR}/pressure_3_gap_unresolved-sparse_interactive.raw.json`, requestPath: "validation/references/rt_a3_s6/requests/c1_gap.json" },
  { name: "pressure_3_coupled_friction_sensitive", rawPath: `${DIR}/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json`, requestPath: `${DIR}/coupled_friction.request.json` }
];

type Session = { model: PreviewModel; result: MechanicsResult; analysisRun: Json; inputManifest: CurrentSessionInputManifestEvidence; intakeText: string };

/** One native-capture solve of the committed raw bytes for the shape's request model. */
async function solve(item: Shape): Promise<Session> {
  const model = structuredClone(json(item.requestPath).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: `Invented report-standing witness ${item.name}` });
  (window as Json).__TAURI_INTERNALS__ = {};
  let intakeText = "";
  const rawText = text(item.rawPath);
  const transport = createSolveJobTransport((sent) => { intakeText = sent.text; return rawText; }, { jobIdPrefix: `invented-standing-${item.name}-job` });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model, "sparse_interactive");
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: `${result.producer!.component_name}@${result.producer!.component_version}`, solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun, intakeText };
}

describe("report standing witnesses (DEL-08-01 REQ-024)", () => {
  it("renders each witness with its standing record, the package still refuses, and the render arguments are current", async () => {
    const outDir = process.env[WRITE];
    for (const item of SHAPES) {
      const session = await solve(item);
      const { model, result, analysisRun, inputManifest } = session;
      expect(numericalResultStanding(result, model).eligible, item.name).toBe(false);
      const reading = readerReportStanding(result, model);
      expect(reading.readability, item.name).toEqual({ readable: true, solved: true, unreadable: [] });
      const assembly = await assembleRenderedReport({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null });
      const input = assembly.input as Json;
      expect(input.numerical_standing, item.name).toEqual(reading.numerical_standing);
      const cases = input.numerical_standing.cases as Json[];
      if (item.name === "pressure_3_gap_unresolved") {
        expect(cases.map((c) => [c.case_id, c.health, c.integrity_code, c.confirmation.outcome])).toEqual([
          ["load:PRESSURE", "quality_unresolved", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "no_confirmation_record"],
          ["load:VACUUM", "quality_unresolved", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "no_confirmation_record"]
        ]);
      } else {
        expect(cases.map((c) => [c.case_id, c.health, c.integrity_code, c.confirmation.outcome, c.confirmation.status])).toEqual([
          ["load:RS-SUS", "sensitive", "NUMERICAL_INTEGRITY_SENSITIVE", "solver_reported_established", "conditional"]
        ]);
      }
      // The package still requires a numerically eligible result (REQ-024's present account).
      await expect(buildReportPackageRequest({ model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null }), item.name)
        .rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
      const submission = assembly.exactSubmission!();
      expect(submission.exactResultText).toBe(text(item.rawPath));
      const content = `${JSON.stringify({
        schema_version: "1.0.0",
        generator: GENERATOR,
        description: "DEL-08-01 REQ-024 connecting-path witness: the render arguments the desktop's report build step sends the native host for an exact-route result that is solved but not numerically eligible (the delivered result bytes are the committed raw file). Job ids are invented; unit transport replay of committed producer bytes, not a native UI qualification witness.",
        shape: item.name,
        raw_path: item.rawPath,
        request_path: item.requestPath,
        solver_mode: "sparse_interactive",
        exact_intake_text: submission.exactIntakeText,
        input_manifest_text: submission.inputManifestText,
        input: assembly.controlled.payload
      })}\n`;
      const name = `${item.name}.render.json`;
      if (outDir) {
        mkdirSync(outDir, { recursive: true });
        writeFileSync(resolve(outDir, name), content);
        continue;
      }
      expect(text(`${DIR}/${name}`) === content, `${DIR}/${name} is stale; regenerate it with ${WRITE}=<dir>`).toBe(true);
    }
  }, 120_000);
});
