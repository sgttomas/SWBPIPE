/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.1): once an exact result can
 * be Current, the surfaces that read the Current result receive it too. With an eligible
 * pressure-2 Current result (plain, and with the load/reference-state group), each of them
 * renders, and none carries result values to a file or a handoff:
 * - `ComparisonPanel` shows the preview comparison built from the Current result;
 * - `KnowledgePanel` lists computed records from the result's diagnostics;
 * - `ReviewGeometryPanel` offers a glTF review download that names the result only by
 *   `run_id` and embeds no result value;
 * - the status bar and the stage rail count a qualified Current result;
 * - the native menu state (`workspaceSession.ts`, `syncNativeShellState`) enables the
 *   Results and Review stages from the same qualification.
 * The results reach the actual workspace session through the native solve-job transport
 * replay of committed producer bytes, as in `loadReference.resultsSessionState.test.ts`.
 * C1 (DEL-04-05 `c1-native-custody-plan.md` slice 4): the Current result is held natively; the
 * comparison (a C2 consumer) is built once its full copy is requested, and the panel reads that
 * copy, as the app's comparisons gate passes it; the other surfaces read the native result.
 * Unit transport replay; NOT a native UI qualification witness. All inputs are invented. */
import { act, cleanup, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { getLocalStorageCapability } from "../../services/projectService";
import { hasNativeMechanicsInvocation, loadDesignKnowledge, type PreviewSolverMode } from "../../services/previewService";
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { sourceContract } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { useWorkspaceSession } from "./workspaceSession";
import { isNativeResult } from "../../services/nativeResult";
import type { C2FullResult } from "../results/C2ResultGate";
import { WorkspaceSessionProvider } from "./WorkspaceSessionContext";
import { ComparisonPanel } from "../comparison/ComparisonPanel";
import { KnowledgePanel } from "../knowledge/KnowledgePanel";
import { ReviewGeometryPanel, buildReviewGeometryPacket } from "../review-geometry/ReviewGeometryPanel";
import { ShellStatusBar } from "./shell/ShellStatusBar";
import { StageRail } from "./shell/StageRail";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const liveHooks = new Set<{ unmount(): void }>();
afterEach(() => {
  cleanup();
  for (const hook of liveHooks) { try { hook.unmount(); } catch { /* already unmounted */ } }
  liveHooks.clear();
  invokeMock.mockReset(); window.localStorage.clear(); delete (window as Json).__TAURI_INTERNALS__;
});

const PRESSURE_LR = "fixtures/product_preview/pressure/load_reference";
/** A pressure-2 producer pair: the model the session loads and the bytes the native job returns. */
type Pair = { model: PreviewModel; bytes: string };
/** The committed plain pressure-2 pair of the native replay (`exact_pressure_authoring_model`). */
const plainPair = (mode: PreviewSolverMode): Pair => {
  const pair = nativeMechanicsReplayPair(mode, { profile: "pressure" });
  return { model: pair.model as PreviewModel, bytes: pair.bytes as string };
};
/** A committed pressure-2 load/reference-state pair, with the case metadata, data boundary
 * and diagnostics list the desktop session requires where the request lacks them (invented
 * values; no physics is read from them). */
const loadReferencePair = (name: string) => (mode: PreviewSolverMode): Pair => {
  const model = structuredClone(json(`${PRESSURE_LR}/${name}.request.json`).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  (model as Json).data_boundary ??= { source: "invented" };
  model.diagnostics ??= [];
  return { model, bytes: readFileSync(resolve(root, `${PRESSURE_LR}/${name}-${mode}.raw.json`), "utf8") };
};

/** The actual workspace session solves the pair through the native job transport replay. */
async function currentSession(make: (mode: PreviewSolverMode) => Pair, mode: PreviewSolverMode) {
  const pair = make(mode);
  delete (window as Json).__TAURI_INTERNALS__;
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const shellStates: Json[] = [];
  const transport = createSolveJobTransport((sent) => {
    expect(sent.mode).toBe(mode);
    if (!isDeepStrictEqual(sent.model, pair.model)) throw new Error("REPLAY_REQUEST_MISMATCH");
    return pair.bytes; // the producer's bytes, as captured
  });
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") { shellStates.push(structuredClone(args.state)); return null; }
    if (command === "load_preview_model" && args === undefined) return structuredClone(pair.model);
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  const hook = renderHook(() => useWorkspaceSession());
  liveHooks.add(hook);
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  act(() => hook.result.current.results.setSolverMode(mode));
  await act(async () => { await hook.result.current.results.handleRun(); });
  await waitFor(() => expect(hook.result.current.results.currentSolvedResult).not.toBeNull());
  return { hook, shellStates };
}

/** Perturbs every value outside the kept identifiers: numbers, strings and booleans change,
 * and every array gains one more element (as `loadReferenceOutputRefusal.test.tsx` does). */
function perturbed(value: Json, keep: (path: string[]) => boolean, path: string[] = []): Json {
  if (keep(path)) return value;
  if (typeof value === "number") return value + 1;
  if (typeof value === "string") return `${value}:perturbed`;
  if (typeof value === "boolean") return !value;
  if (Array.isArray(value)) {
    const items = value.map((item, index) => perturbed(item, keep, [...path, String(index)]));
    return [...items, items.length ? structuredClone(items[items.length - 1]) : "perturbed:extra"];
  }
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, perturbed(item, keep, [...path, key])]));
  return value;
}
const keepResultIdentifier = (path: string[]) => path.length === 1 && path[0] === "run_id";
const keepRunIdentifiers = (path: string[]) => path[0] === "analysis_run"
  && ((path.length === 2 && (path[1] === "run_id" || path[1] === "model_state_ref")) || (path.length > 2 && path[1] === "model_state_ref"));
const decodeJsonDataHref = (href: string): Json => {
  const comma = href.indexOf(",");
  const header = href.slice(0, comma), body = href.slice(comma + 1);
  return JSON.parse(header.endsWith(";base64") ? new TextDecoder().decode(Uint8Array.from(atob(body), c => c.charCodeAt(0))) : decodeURIComponent(body));
};

const CASES = [
  ["plain", plainPair],
  ["load/reference-state (connected)", loadReferencePair("connected")],
  ["load/reference-state (pressure)", loadReferencePair("pressure")],
] as const;
const MODES: PreviewSolverMode[] = ["sparse_interactive", "dense_scrutiny"];

describe.each(CASES)("an eligible pressure-2 %s Current result reaches the Current-receiving surfaces (§4.1)", (_label, make) => {
  it.each(MODES)("%s: each renders, and none carries result values to a file or a handoff", async (mode) => {
    const { hook, shellStates } = await currentSession(make, mode);
    const { model } = hook.result.current.model;
    const result = hook.result.current.results.currentSolvedResult!;
    expect(isNativeResult(result)).toBe(true);
    // Nothing is compared until the comparison's full copy is requested.
    expect(hook.result.current.results.comparison).toBeNull();
    let full: C2FullResult | null = null;
    await act(async () => { full = await hook.result.current.results.loadC2Full(result); });
    const copy = full!.result;
    const session = hook.result.current;
    const { analysisRun, comparison } = session.results;
    expect(sourceContract(result)).toBe("pressure");
    expect(hasNativeMechanicsInvocation(result, model!, mode)).toBe(true);
    expect(numericalResultStanding(result, model).eligible).toBe(true);
    expect(analysisRun?.schema_version).toBe("0.3.0");
    const hasStates = Array.isArray((result.contract_evidence as Json)?.load_reference_states);
    expect(hasStates).toBe(_label !== "plain");

    // ComparisonPanel: the preview comparison built from the Current result; no download.
    expect(comparison).not.toBeNull();
    const comparisonView = render(<ComparisonPanel comparison={comparison} result={copy} onSelectResult={() => {}} />);
    expect(within(comparisonView.container).getByTestId("comparison-panel")).toBeTruthy();
    expect(within(comparisonView.container).getByTestId("comparison-summary")).toBeTruthy();
    expect(comparisonView.container.querySelectorAll("a[download], a[href^='data:']")).toHaveLength(0);
    comparisonView.unmount();

    // KnowledgePanel: computed records from the result's diagnostics; no download.
    const knowledgeView = render(<KnowledgePanel knowledge={session.model.knowledge} result={result} />);
    expect(within(knowledgeView.container).getByTestId("knowledge-panel")).toBeTruthy();
    expect(knowledgeView.container.querySelectorAll("a[download], a[href^='data:']")).toHaveLength(0);
    knowledgeView.unmount();

    // ReviewGeometryPanel: one glTF review download, which names the result by run_id only.
    const reviewView = render(<ReviewGeometryPanel model={model!} result={result} analysisRun={analysisRun} />);
    const link = within(reviewView.container).getByTestId("review-geometry-export-link");
    expect(reviewView.container.querySelectorAll("[data-testid$='-export-link']")).toHaveLength(1);
    const packet = buildReviewGeometryPacket({ model: model!, result, analysisRun });
    // The link exposes its data href only after the user's explicit local intent; when
    // exposed, it is exactly this packet.
    const href = link.getAttribute("href") ?? "";
    if (href.startsWith("data:")) expect(decodeJsonDataHref(href)).toStrictEqual(JSON.parse(JSON.stringify(packet)));
    const text = JSON.stringify(packet);
    expect(packet.result_ref).toStrictEqual(expect.objectContaining({ ref: result.run_id }));
    for (const row of copy.results) expect(text, row.id).not.toContain(row.id);
    expect(text).not.toContain("load_reference_states");
    // Invariant under every result value but run_id, and every record value but its identifiers:
    // the full copy and full record, perturbed, give the native handle's packet.
    const changed = buildReviewGeometryPacket({ model: model!, result: perturbed(copy, keepResultIdentifier) as MechanicsResult, analysisRun: perturbed(full!.analysisRun, keepRunIdentifiers) as AnalysisRunEnvelope });
    expect(JSON.stringify(changed)).toBe(text);
    reviewView.unmount();

    // The status bar and the stage rail count the qualified Current result.
    render(
      <WorkspaceSessionProvider session={hook.result.current}>
        <StageRail issueCount={0} />
        <ShellStatusBar issueCount={0} />
      </WorkspaceSessionProvider>
    );
    expect(screen.getByTestId("rail-stage-results").getAttribute("aria-disabled")).toBeNull();
    expect(screen.getByTestId("rail-stage-review").getAttribute("aria-disabled")).toBeNull();
    expect(screen.queryByTestId("rail-caption-results")).toBeNull(); // no "Needs recompute"
    expect(screen.getByTestId("status-pill-mechanics")).toBeTruthy();
    expect(screen.getByTestId("status-chips").textContent).not.toContain("Needs recompute");
    expect(screen.getByTestId("workspace-status-bar").querySelectorAll("a[download]")).toHaveLength(0);
    expect(screen.getByTestId("stage-rail").querySelectorAll("a[download]")).toHaveLength(0);

    // The native menu state: the Results and Review stages enabled by the same qualification.
    await waitFor(() => expect(shellStates.at(-1)).toMatchObject({ resultsStageEnabled: true, reviewStageEnabled: true }));
    expect(JSON.stringify(shellStates.at(-1))).not.toContain(result.run_id);
  }, 30000);
});
