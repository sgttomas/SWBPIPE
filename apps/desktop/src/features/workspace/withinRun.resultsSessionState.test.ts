// "Within this run" in the results cells (W-12 S3c; DEL-14-01 Design §4.3): a choice
// is revalidated against each Current run. Two actual producer outputs are made
// Current in turn (unit transport replay of their bytes; NOT a native UI
// qualification witness): the range-envelope run offers L-100, L-200 and
// U4B-RANGE-MAX; the connected preview run offers only case:six-component-load. A
// choice the second run does not offer is dropped, named, and the default applies;
// it is never compared, never carried and never reset without the notice. Invented
// data only.
import { act, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, buildPreviewComparison, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import type { PreviewModel } from "../../types";
import { useResultsSessionState } from "./resultsSessionState";

afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const RANGE = "combination:U4B-RANGE-MAX";

/** A Current run of `model` whose replayed producer returns `bytes`. */
async function currentRun(model: PreviewModel, bytes: string) {
  const transport = createSolveJobTransport(() => bytes);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation((command: string, args: unknown, options?: unknown) => transport.invoke(command, args, options));
  const source = await runPreviewMechanics(model, "sparse_interactive");
  const manifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:within-run-transport-replay", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
  return { source, manifest, analysis };
}

it("drops a choice the next Current run does not offer, names it, and applies the default", async () => {
  const rangeModel = JSON.parse(readFileSync(resolve(root, "core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json"), "utf8")) as PreviewModel;
  const range = await currentRun(rangeModel, readFileSync(resolve(root, "fixtures/results/preview_physics_range_envelope_sparse.json"), "utf8"));
  const connectedPair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const connected = await currentRun(connectedPair.model, connectedPair.bytes);
  const { result } = renderHook(() => useResultsSessionState());
  const show = (run: Awaited<ReturnType<typeof currentRun>>) => act(() => {
    result.current.setResult(run.source);
    result.current.setInputManifest(run.manifest);
    result.current.setAnalysisRun(run.analysis);
  });
  const pair = () => (result.current.withinRun.status === "ready" ? { ...result.current.withinRun.pair, source: result.current.withinRun.source } : result.current.withinRun.status);

  show(range);
  expect(result.current.currentSolvedResult).toBe(range.source);
  act(() => result.current.setHeldWithinRunSelection({ left: "load:L-200", right: RANGE }));
  expect(pair()).toEqual({ left: "load:L-200", right: RANGE, source: "selected" });
  expect(result.current.droppedWithinRunBases).toBeNull();

  // Retired (an edit or a new solve clears the result): nothing is compared, the choice waits.
  act(() => { result.current.setResult(null); result.current.setAnalysisRun(null); result.current.setInputManifest(null); });
  expect(result.current.withinRun).toEqual({ status: "no_result" });
  expect(result.current.droppedWithinRunBases).toBeNull();

  // The connected run offers neither chosen basis: dropped, named, default applies.
  show(connected);
  expect(result.current.currentSolvedResult).toBe(connected.source);
  expect(result.current.droppedWithinRunBases).toEqual(["load:L-200", RANGE]);
  expect(result.current.withinRun.status).toBe("unavailable"); // it has no combination rows
  // The packages carry the fixed-pair comparison exactly as without a choice.
  const today = buildPreviewComparison({ result: connected.source, analysisRun: connected.analysis });
  expect(JSON.stringify(result.current.comparison)).toBe(JSON.stringify(today));
  expect(result.current.comparisonUnavailable).toBeNull();

  // The dropped choice is not carried back to a run that offers its bases again.
  show(range);
  expect(pair()).toEqual({ left: "load:L-100", right: RANGE, source: "default" });
  expect(result.current.droppedWithinRunBases).toEqual(["load:L-200", RANGE]);
  // The next choice clears the notice.
  act(() => { result.current.setHeldWithinRunSelection({ left: "load:L-200", right: RANGE }); result.current.setDroppedWithinRunBases(null); });
  expect(pair()).toEqual({ left: "load:L-200", right: RANGE, source: "selected" });
  expect(result.current.droppedWithinRunBases).toBeNull();
});
