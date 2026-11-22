// Release exit criterion 18 (LIM-026): the native results panel's results line takes its state from
// the gated health, as the browser panel does (`solverChecksSurfaces.test.tsx`). An unsolved document
// whose own status says checks_passed reads "not assessed", never "no problem"; the solved demo reads
// exactly as before. The saved project is opened natively (`openLocalProjectNative`), and the handle's
// reads are the actual commands' replies (`nativeReplayStore.ts`, `native_replay_tests.rs`). Unit
// transport replay; NOT native UI qualification. Committed fixtures only.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import type { MechanicsResult, PreviewModel } from "../../types";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { openLocalProjectNative } from "../../services/nativeReopen";
import { isNativeResult, resetNativeResultsForTests } from "../../services/nativeResult";
import { resetSessionResultCustodyForTests } from "../../services/previewService";
import { resetNativeReopenEmulation } from "../../test/nativeReopenEmulation";
import { buildNativeHistoricalRunContext, HistoricalRunPanel, type HistoricalRunContext } from "./HistoricalRunContext";
import { ResultsPanel } from "./ResultsPanel";
import { BOUNDARY_SENTENCE } from "./numericalHealthText";

const root = resolve(__dirname, "../../../../../");
const json = <T,>(path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as T;
const DEMO_MODEL = "fixtures/product_preview/invented_demo_model.json";
const DEMO_RESULT = "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";

let envelope: Json = null;
beforeEach(() => {
  resetSessionResultCustodyForTests();
  resetNativeResultsForTests();
  resetNativeReopenEmulation();
  invokeMock.mockImplementation(async (command: string) => {
    if (command === "open_local_project") return structuredClone(envelope);
    throw new Error(`UNEXPECTED_COMMAND: ${command}`);
  });
});
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

/** The saved project opened natively. */
async function openedNatively(model: PreviewModel, source: MechanicsResult): Promise<HistoricalRunContext> {
  const payload = { model, mechanics_result: source, analysis_run: null, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
  envelope = { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never), summary: { project_id: (model as Json).project?.id ?? "", message: "Opened the invented stored project." } };
  (window as Json).__TAURI_INTERNALS__ = {};
  const context = (await buildNativeHistoricalRunContext((await openLocalProjectNative(null))!))!;
  expect(isNativeResult(context.mechanicsResult!)).toBe(true);
  return context;
}
/** The saved project opened natively and shown by the historical panel's native results panel. */
async function shownNatively(model: PreviewModel, source: MechanicsResult): Promise<string> {
  const context = await openedNatively(model, source);
  render(<HistoricalRunPanel context={context} />);
  const standing = await screen.findByTestId("numerical-result-standing");
  await waitFor(() => expect(screen.getByTestId("historical-run-context").textContent).not.toContain("Reading"));
  return standing.textContent ?? "";
}

describe("the native results line takes the gated health's state", () => {
  it("an unsolved document whose own status says checks_passed reads not assessed, never no problem", async () => {
    const source = json<MechanicsResult>(DEMO_RESULT);
    source.status = { ...source.status, mechanics: "MODEL_INCOMPLETE" };
    expect(source.numerical_quality!.status).toBe("checks_passed");
    const standing = await shownNatively(json<PreviewModel>(DEMO_MODEL), source);
    expect(standing).toContain("Numerical health: not assessed. ");
    expect(standing).not.toContain("no problem");
  });

  it("a solved document whose status is sensitive while every case passed reads sensitive on the line and the header; each case keeps its own", async () => {
    const source = json<MechanicsResult>(DEMO_RESULT);
    source.numerical_quality = { ...source.numerical_quality!, status: "sensitive" };
    const standing = await shownNatively(json<PreviewModel>(DEMO_MODEL), source);
    expect(standing).toContain("Numerical health: sensitive. ");
    expect(screen.getByTestId("numerical-health-header").textContent).toBe("Numerical health: sensitive");
    const caseBadges = screen.getAllByTestId(/^numerical-health-case-badge-/);
    expect(caseBadges.length).toBeGreaterThan(0);
    for (const badge of caseBadges) expect(badge.textContent).toBe("Numerical health: solver checks found no problem");
  });

  it("with no indicator, the line that stands in its place carries the boundary sentence once", async () => {
    const context = await openedNatively(json<PreviewModel>(DEMO_MODEL), json<MechanicsResult>(DEMO_RESULT));
    render(<ResultsPanel result={context.mechanicsResult} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} numericalHealth={null} />);
    expect((await screen.findByTestId("numerical-result-standing")).textContent).toContain("Numerical health: not assessed. ");
    expect(screen.queryByTestId("numerical-health-detail")).toBeNull();
    expect(screen.getAllByTestId("numerical-health-boundary").map((n) => n.textContent)).toEqual([BOUNDARY_SENTENCE]);
    expect(document.body.textContent!.split(BOUNDARY_SENTENCE).length - 1).toBe(1);
  });

  it("the solved demo reads exactly as before", async () => {
    const standing = await shownNatively(json<PreviewModel>(DEMO_MODEL), json<MechanicsResult>(DEMO_RESULT));
    expect(standing).toContain("Numerical health: solver checks found no problem. ");
  });
});
