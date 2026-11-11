// W-12 S2 repair r7 (DEL-14-01 Design §3): the history commands take 256 KiB of
// arguments, so a project id of any length the app keeps up to that limit lists;
// a longer one is refused at ingress, and the History panel shows that named
// refusal (`INGRESS-ARGS-TOO-LARGE`), not a bare "List failed". The native
// budget itself is the Rust test `the_history_commands_reach_long_project_ids_written_by_the_real_writers`.
import { act, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { useModelStateHistory, type ModelStateHistoryDeps } from "./useModelStateHistory";

const HISTORY_ARGS_BUDGET = 256 * 1024;

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

/** The native store's admission of the history commands: a body over the
 * budget is refused before anything runs, with the native error's shape. */
function nativeHistoryStore() {
  invokeMock.mockImplementation((command: string, args: Record<string, unknown>) => {
    const sent = new TextEncoder().encode(JSON.stringify(args)).length;
    if (sent > HISTORY_ARGS_BUDGET) {
      return Promise.reject({
        code: "INVALID_REQUEST",
        message: `INGRESS-ARGS-TOO-LARGE: \`${command}\` takes at most ${HISTORY_ARGS_BUDGET} bytes of arguments; ${sent} were sent.`
      });
    }
    return Promise.resolve([]);
  });
}

/** The hook for a session whose model is project `projectId` (only what the
 * list reads; no run is in the slot). */
function history(projectId: string) {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const model = { project: { id: projectId } };
  const deps = {
    model, currentModel: { current: model }, projectSessionGeneration: 0, projectSessionGenerationRef: { current: 0 },
    runSlot: { result: null, analysisRun: null, inputManifest: null, restored: false }
  } as unknown as ModelStateHistoryDeps;
  return renderHook(() => useModelStateHistory(deps));
}

describe("the History panel's list with long project ids (W-12 S2 r7)", () => {
  it("lists a project whose id is 64 KiB", async () => {
    nativeHistoryStore();
    const projectId = `project:${"p".repeat(64 * 1024)}`;
    const { result } = history(projectId);
    await act(async () => { await result.current.refreshStates(projectId); });
    expect(result.current.states).toEqual([]);
    expect(result.current.historyMessage).not.toMatch(/List failed/);
  });

  it("shows the named ingress refusal for a project id past the 256 KiB budget", async () => {
    nativeHistoryStore();
    const projectId = `project:${"p".repeat(HISTORY_ARGS_BUDGET)}`;
    const { result } = history(projectId);
    await act(async () => { await result.current.refreshStates(projectId); });
    expect(result.current.states).toBeNull();
    expect(result.current.historyMessage).toMatch(/^List failed: .*INGRESS-ARGS-TOO-LARGE: `list_(model_states|analysis_runs)` takes at most 262144 bytes of arguments/);
  });
});
