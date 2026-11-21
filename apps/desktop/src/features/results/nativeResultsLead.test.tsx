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
import { buildNativeHistoricalRunContext, HistoricalRunPanel } from "./HistoricalRunContext";

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

/** The saved project opened natively and shown by the historical panel's native results panel. */
async function shownNatively(model: PreviewModel, source: MechanicsResult): Promise<string> {
  const payload = { model, mechanics_result: source, analysis_run: null, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
  envelope = { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never), summary: { project_id: (model as Json).project?.id ?? "", message: "Opened the invented stored project." } };
  (window as Json).__TAURI_INTERNALS__ = {};
  const context = (await buildNativeHistoricalRunContext((await openLocalProjectNative(null))!))!;
  expect(isNativeResult(context.mechanicsResult!)).toBe(true);
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

  it("the solved demo reads exactly as before", async () => {
    const standing = await shownNatively(json<PreviewModel>(DEMO_MODEL), json<MechanicsResult>(DEMO_RESULT));
    expect(standing).toContain("Numerical health: solver checks found no problem. ");
  });
});
