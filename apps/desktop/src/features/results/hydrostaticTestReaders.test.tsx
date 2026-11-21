/** DEL-05-01 REQ-014: the desktop's intake of a solved hydrostatic test case. The two documents of
 * `fixtures/hydrostatic_test/four_span_reader_models.json` (the four-span model taken through its journey, and its
 * load and reference-state twin with an ordinary case beside the held test case) are solved under the native-capture
 * test seams with the raw bytes the producer wrote for them
 * (`fixtures/results/hydrostatic_test_*-sparse_interactive.raw.json`, written with the models by the applier's
 * journey test). Through the functions the results panel and the report take their verdict from, each is read on the
 * pressure route: the panel's standing (`numericalResultStanding`), the report's (`reportStanding`, readable and
 * solved, with a standing record) and the web view's admission of it for the rendered report
 * (`admitExactReportSource`, purpose `report-render`). No standing finding
 * concerns the test case, and the mark reaches the session unchanged.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI qualification witness.
 * Standard claim fence applies (claims taxonomy per DEC-081). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { PreviewModel } from "../../types";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { numericalResultStanding, reportStanding } from "./resultStanding";
import { readerDispatch } from "./resultReader";
import { admitExactReportSource } from "../report/exactReportAdmission";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => {
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const MODELS = JSON.parse(text("fixtures/hydrostatic_test/four_span_reader_models.json")).models;
const DOCUMENTS = [
  { name: "journey", raw: "fixtures/results/hydrostatic_test_journey-sparse_interactive.raw.json" },
  { name: "load_state_twin", raw: "fixtures/results/hydrostatic_test_load_state_twin-sparse_interactive.raw.json" }
] as const;
const ABOUT_THE_TEST = (finding: string) => /TEST|CASE_SHAPE|CONSTANT_EFFORT/.test(finding);

async function solve(name: string, rawText: string) {
  const model = structuredClone(MODELS[name]) as PreviewModel;
  (window as Json).__TAURI_INTERNALS__ = {};
  const transport = createSolveJobTransport(() => rawText, { jobIdPrefix: `invented-hydrostatic-${name}-job` });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model, "sparse_interactive");
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: `${result.producer!.component_name}@${result.producer!.component_version}`, solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun };
}

describe("a solved hydrostatic test case through the desktop's readers", () => {
  for (const document of DOCUMENTS) {
    it(`reads ${document.name} on the pressure route, with no finding about the test case`, async () => {
      const { model, result, inputManifest, analysisRun } = await solve(document.name, text(document.raw));
      expect(readerDispatch(result).route).toBe("pressure");
      const marked = (result as Json).contract_evidence.exact_cases.filter((c: Json) => c.test_condition !== undefined);
      expect(marked.map((c: Json) => c.load_case_id)).toEqual(["HT-4S-TEST"]);
      expect(marked[0].test_condition.held_supports).toBe("springs_and_constant_effort");

      const standing = numericalResultStanding(result, model);
      expect(standing.contract).toBe("pressure");
      expect(standing.findings.filter(ABOUT_THE_TEST)).toEqual([]);

      const report = reportStanding(result, model);
      expect(report.readability).toEqual({ readable: true, solved: true, unreadable: [] });
      expect(report.standing_refusal).toBeNull();
      expect(report.numerical_standing).not.toBeNull();
      expect(report.standing.findings.filter(ABOUT_THE_TEST)).toEqual([]);

      const admission = await admitExactReportSource({ model, result, analysisRun, inputManifest, purpose: "report-render" });
      expect(admission.binding.semantic_contract_id).toBe((result as Json).producer.semantic_contract_id);
    });
  }
});
