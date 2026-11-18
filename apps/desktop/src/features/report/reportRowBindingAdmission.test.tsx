/** The report's admission, standing and refusals are what they were when the Current builder
 * searched the analysis record's result references from the start for every row
 * (DEL-08-01 REQ-017, REQ-024; `../result-export/resultRefLookup.ts`).
 *
 * Each maintained exact-route session below goes through the production report build step
 * (`assembleRenderedReport`) and the production package request (`buildReportPackageRequest`)
 * twice from equal inputs: once with the maintained lookup and once with that search put in
 * its place. The two outcomes must be equal: the whole report input with the standing record
 * it states, the whole package request, or the named refusal. The standing and the refusal
 * each case is for are then asserted by name.
 *
 * The sessions are one numerically eligible result and the two REQ-024 witnesses that are
 * solved but not numerically eligible (`fixtures/reports/invented/report_standing_witnesses/`).
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. Job ids are invented; case labels and project names are added
 * where a committed request has none. Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
const lookup = vi.hoisted(() => ({ searchFromStart: false, calls: 0 }));
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
vi.mock("../result-export/resultRefLookup", async importOriginal => {
  const actual = await importOriginal<typeof import("../result-export/resultRefLookup")>();
  return {
    firstMatchResultRefLookup: <T extends { result_ref: { ref: unknown } }>(refs: readonly T[]) => {
      lookup.calls += 1;
      return lookup.searchFromStart ? (id: unknown) => refs.find(x => x.result_ref.ref === id) : actual.firstMatchResultRefLookup(refs);
    },
  };
});
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { numericalResultStanding, reportStanding } from "../results/resultStanding";
import { assembleRenderedReport } from "./renderedReportAssembly";
import { buildReportPackageRequest } from "./reportPackageRequest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => {
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
  lookup.searchFromStart = false;
});
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = (path: string) => JSON.parse(text(path));
const WITNESSES = "fixtures/reports/invented/report_standing_witnesses";

type Shape = { name: string; eligible: boolean; rawPath: string; requestPath: string };
const SHAPES: readonly Shape[] = [
  { name: "pressure_2_straight", eligible: true, rawPath: "fixtures/results/pressure_connected_mechanics_sparse.json", requestPath: "core/product_physics/tests/fixtures/exact_pressure_connected_request.json" },
  { name: "pressure_3_gap_unresolved", eligible: false, rawPath: `${WITNESSES}/pressure_3_gap_unresolved-sparse_interactive.raw.json`, requestPath: "validation/references/rt_a3_s6/requests/c1_gap.json" },
  { name: "pressure_3_coupled_friction_sensitive", eligible: false, rawPath: `${WITNESSES}/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json`, requestPath: `${WITNESSES}/coupled_friction.request.json` }
];

type Session = { model: PreviewModel; result: MechanicsResult; analysisRun: Json; inputManifest: CurrentSessionInputManifestEvidence };

/** One native-capture solve of the committed raw bytes for the shape's request model. */
async function solve(item: Shape): Promise<Session> {
  const model = structuredClone(json(item.requestPath).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: `Invented row-binding witness ${item.name}` });
  (window as Json).__TAURI_INTERNALS__ = {};
  const rawText = text(item.rawPath);
  const transport = createSolveJobTransport(() => rawText, { jobIdPrefix: `invented-row-binding-${item.name}-job` });
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

/** The record with its result references edited; the model, the registered result object
 * and the manifest are the session's own. */
type Edit = { name: string; apply: (refs: Json[]) => void };
const EDITS: readonly Edit[] = [
  { name: "a duplicate reference", apply: refs => { refs[refs.length - 1] = structuredClone(refs[0]); } },
  { name: "a missing reference", apply: refs => { refs[refs.length >> 1].result_ref.ref = "result:absent"; } }
];
const edited = (session: Session, edit: Edit): Session => {
  const analysisRun = structuredClone(session.analysisRun);
  edit.apply(analysisRun.analysis_run.result_refs);
  return { ...session, analysisRun };
};

type Outcome = { made: string; reachedRowBinding: boolean } | { refused: string; reachedRowBinding: boolean };
async function outcomeOf(searchFromStart: boolean, act: () => Promise<unknown>): Promise<Outcome> {
  lookup.searchFromStart = searchFromStart;
  const before = lookup.calls;
  try { return { made: JSON.stringify(await act()), reachedRowBinding: lookup.calls > before }; }
  catch (error) { return { refused: (error as Error).message, reachedRowBinding: lookup.calls > before }; }
}
/** The same act under the search and under the maintained lookup; the outcomes are equal. */
async function bothWays(act: () => Promise<unknown>, why: string): Promise<Outcome> {
  const searched = await outcomeOf(true, act), maintained = await outcomeOf(false, act);
  expect(maintained, why).toStrictEqual(searched);
  return maintained;
}
/** The report build step's whole output: the report input (with the standing record it
 * states) and the controlled payload the native host is sent. */
const render = ({ model, result, analysisRun, inputManifest }: Session) => async () => {
  const assembly = await assembleRenderedReport({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null, libraryRefFindings: [] });
  return { input: assembly.input, controlled: assembly.controlled.payload };
};
const packageRequest = ({ model, result, analysisRun, inputManifest }: Session) => () =>
  buildReportPackageRequest({ model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null });

const ROW_BINDING = "CURRENT_ROW_BINDING_MISMATCH";
const NEEDS_RECOMPUTE = "CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE";

describe("report admission, standing and refusals against the search the row binding replaced", () => {
  it.each(SHAPES)("$name: the rendered report states the same standing and the package gives the same answer", async item => {
    const session = await solve(item);
    const { model, result } = session;
    expect(numericalResultStanding(result, model).eligible).toBe(item.eligible);
    const reading = reportStanding(result, model);
    expect(reading.readability).toEqual({ readable: true, solved: true, unreadable: [] });
    expect(reading.numerical_standing).not.toBeNull();

    const rendered = await bothWays(render(session), "render");
    expect("made" in rendered && rendered.reachedRowBinding, JSON.stringify(rendered).slice(0, 300)).toBe(true);
    const input = JSON.parse((rendered as { made: string }).made).input;
    // The standing shown is the one the reader derives for this result and model.
    expect(input.numerical_standing).toEqual(reading.numerical_standing);
    expect(JSON.stringify(input.numerical_standing.cases.map((c: Json) => [c.case_id, c.health, c.integrity_code, c.confirmation.outcome])))
      .toBe(JSON.stringify((reading.numerical_standing as Json).cases.map((c: Json) => [c.case_id, c.health, c.integrity_code, c.confirmation.outcome])));

    const packaged = await bothWays(packageRequest(session), "package");
    if (item.eligible) expect("made" in packaged && packaged.reachedRowBinding).toBe(true);
    // A result that is not numerically eligible is refused for its standing, before any row is bound.
    else expect(packaged).toStrictEqual({ refused: NEEDS_RECOMPUTE, reachedRowBinding: false });
  }, 240_000);

  it.each(SHAPES.flatMap(item => EDITS.map(edit => ({ item, edit, title: `${item.name}, ${edit.name}` }))))("$title: the same named refusal", async ({ item, edit }) => {
    const session = edited(await solve(item), edit);
    const rendered = await bothWays(render(session), "render");
    expect(rendered).toStrictEqual({ refused: ROW_BINDING, reachedRowBinding: true });
    const packaged = await bothWays(packageRequest(session), "package");
    // The standing refusal comes first for a result that is not numerically eligible.
    expect(packaged).toStrictEqual(item.eligible ? { refused: ROW_BINDING, reachedRowBinding: true } : { refused: NEEDS_RECOMPUTE, reachedRowBinding: false });
  }, 240_000);
});
