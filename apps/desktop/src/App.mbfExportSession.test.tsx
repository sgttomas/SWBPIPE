import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { StrictMode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import {
  cancelMbfExport,
  MBF_EXPORT_CANCELLED,
  retireMbfExportSession,
  reviewMbfExport,
  saveMbfExport,
  type MbfCancelReceipt
} from "./features/caepipe-mbf/mbfExportCancel";
import type { MbfExportArgs, MbfReviewArgs } from "./features/caepipe-mbf/mbfExport";

// DEL-17-04 Design `desktop-mbf-export-import.md` §3.9.5 (slice 4b.2a): File > Export MBF…'s review and save belong
// to the project session that started them. Retiring that session (a new generation, a project replaced by one with
// the same id, unmount, React Strict Mode's re-run) retires the export through its one barrier
// (`mbfExportCancel.ts`): runs started so far are cancelled natively (`cancel_mbf_export`), a start not yet sent is
// never sent, and a start issued during a retirement reaches native only after the retirement is acknowledged, with
// that retirement's epoch as its dispatch ticket (`expectedEpoch`).
afterEach(async () => {
  // Leave the barrier settled and acknowledged for the next test.
  invokeMock.mockImplementation((command: string) => (command === "cancel_mbf_export" ? Promise.resolve(nothing()) : Promise.reject(new Error(command))));
  await retireMbfExportSession();
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

// The mocked native retirement epoch: each acknowledged cancel moves it on (the barrier's module, and so its last
// acknowledged epoch, lasts across this file's tests).
let nativeEpoch = 0;
const nothing = (): MbfCancelReceipt => ({ cancelled: 0, too_late: 0, epoch: ++nativeEpoch });
const REVIEW_ARGS: MbfReviewArgs = { canonicalText: "{}", claimedModelHash: "sha256:0", settingsJson: "{}" };
const SAVE_ARGS = { ...REVIEW_ARGS, suggestedName: "swbpipe-model.mbf", reviewed: { mbfSha256: "0", sidecarSha256: "0" } } as unknown as MbfExportArgs;
const REVIEWED = { outcome: "reviewed", review: {} };
const SAVED = { outcome: "saved" };

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}

/** A native host whose `cancel_mbf_export` deliveries wait until the test answers them, in order. */
function nativeHost() {
  const commands: string[] = [];
  const cancels: Array<ReturnType<typeof deferred<MbfCancelReceipt>>> = [];
  const starts: Array<{ command: string; ticket: unknown; reply: ReturnType<typeof deferred<unknown>> }> = [];
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    commands.push(command);
    if (command === "cancel_mbf_export") {
      const answer = deferred<MbfCancelReceipt>();
      cancels.push(answer);
      return answer.promise;
    }
    if (command === "review_mbf_export" || command === "save_mbf_export") {
      const reply = deferred<unknown>();
      starts.push({ command, ticket: args?.expectedEpoch, reply });
      return reply.promise;
    }
    return Promise.reject(new Error(`Unexpected command ${command}`));
  });
  const sent = () => commands.filter((command) => command === "review_mbf_export" || command === "save_mbf_export");
  const answerCancels = async () => {
    for (const answer of cancels.splice(0)) answer.resolve(nothing());
    await act(async () => {});
  };
  return { commands, cancels, starts, sent, answerCancels };
}

async function renderedWithProject(strict = false) {
  const view = render(strict ? <StrictMode><App /></StrictMode> : <App />);
  await screen.findByTestId("desktop-preview-shell");
  act(() => nativeMenuCommand("file.save-local"));
  await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Saved local browser-preview project snapshot"));
  return view;
}

/** Replaces the session with the same project (the browser preview's local store holds it). */
async function replaceWithSameId(host: ReturnType<typeof nativeHost>) {
  const before = host.cancels.length;
  act(() => nativeMenuCommand("file.open-local"));
  await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Opened local browser-preview project snapshot."));
  await waitFor(() => expect(host.cancels.length).toBe(before + 1));
}

function cancelledCode(outcome: { outcome: string; diagnostics?: Array<{ code: string }> }) {
  return outcome.outcome === "refused" ? outcome.diagnostics?.map((item) => item.code) : outcome.outcome;
}

describe("MBF export session retirement", () => {
  it("retires the MBF export natively at each project session change and at unmount, with no model check in progress", async () => {
    const commands: string[] = [];
    invokeMock.mockImplementation((command: string) => {
      commands.push(command);
      if (command === "cancel_mbf_export") return Promise.resolve(nothing());
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    const { unmount } = render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const message = () => screen.getByTestId("local-project-message");
    const cancels = () => commands.filter((command) => command === "cancel_mbf_export").length;
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Saved local browser-preview project snapshot"));
    const before = cancels();
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Opened local browser-preview project snapshot."));
    await waitFor(() => expect(cancels()).toBe(before + 1));
    // The same project id again: a new session all the same, so the export is retired again.
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(cancels()).toBe(before + 2));
    // No model check was in progress, and none was cancelled: the MBF retirement does not depend on one.
    expect(commands).not.toContain("check_preview_model");
    expect(commands).not.toContain("cancel_model_checks");
    unmount();
    await waitFor(() => expect(cancels()).toBe(before + 3));
  });

  it("returns the native receipt, and never throws when the host does not answer", async () => {
    invokeMock.mockResolvedValueOnce({ cancelled: 1, too_late: 0, epoch: 7 });
    await expect(cancelMbfExport()).resolves.toEqual({ cancelled: 1, too_late: 0, epoch: 7 });
    expect(invokeMock).toHaveBeenLastCalledWith("cancel_mbf_export");
    // A reply without a retirement epoch is not an acknowledgement.
    invokeMock.mockResolvedValueOnce({ cancelled: 1, too_late: 0 });
    await expect(cancelMbfExport()).resolves.toBeNull();
    invokeMock.mockRejectedValueOnce(new Error("no native host"));
    await expect(cancelMbfExport()).resolves.toBeNull();
    invokeMock.mockImplementationOnce(() => { throw new Error("no native host"); });
    await expect(cancelMbfExport()).resolves.toBeNull();
  });

  it("sends a start issued during a delayed retirement only after it is acknowledged, and the retirement does not cancel it", async () => {
    const host = nativeHost();
    await renderedWithProject();
    await host.answerCancels();
    await replaceWithSameId(host);
    // The new session's starts, issued at once while the old session's cancel is still on its way.
    const review = reviewMbfExport(REVIEW_ARGS);
    const save = saveMbfExport(SAVE_ARGS);
    await act(async () => {});
    expect(host.sent()).toEqual([]);
    await host.answerCancels();
    await waitFor(() => expect(host.sent()).toEqual(["review_mbf_export", "save_mbf_export"]));
    // Every cancel was delivered before either start, so neither is among the runs it cancelled, and both carry the
    // epoch of the retirement they waited for.
    expect(host.commands.lastIndexOf("cancel_mbf_export")).toBeLessThan(host.commands.indexOf("review_mbf_export"));
    expect(host.starts.map(({ ticket }) => ticket)).toEqual([nativeEpoch, nativeEpoch]);
    host.starts[0].reply.resolve(REVIEWED);
    host.starts[1].reply.resolve(SAVED);
    const [reviewed, saved] = await Promise.all([review, save]);
    expect(reviewed.outcome).toEqual(REVIEWED);
    expect(saved.outcome).toEqual(SAVED);
    expect(reviewed.current() && saved.current()).toBe(true);
  });

  it("never sends an old start that was waiting on a retirement when its session is replaced by one with the same id", async () => {
    const host = nativeHost();
    await renderedWithProject();
    await host.answerCancels();
    await replaceWithSameId(host);
    // Old starts (this session's) wait on the retirement still in progress; the session is then replaced again.
    const review = reviewMbfExport(REVIEW_ARGS);
    const save = saveMbfExport(SAVE_ARGS);
    await replaceWithSameId(host);
    await host.answerCancels();
    expect(host.sent()).toEqual([]);
    const [reviewed, saved] = await Promise.all([review, save]);
    expect(cancelledCode(reviewed.outcome)).toEqual([MBF_EXPORT_CANCELLED]);
    expect(cancelledCode(saved.outcome)).toEqual([MBF_EXPORT_CANCELLED]);
    expect(reviewed.current() || saved.current()).toBe(false);
  });

  it("never sends an old start whose caller's own session check fails, though no retirement has been requested since", async () => {
    const host = nativeHost();
    await renderedWithProject();
    await host.answerCancels();
    await replaceWithSameId(host);
    let callerCurrent = true;
    const review = reviewMbfExport(REVIEW_ARGS, () => callerCurrent);
    const save = saveMbfExport(SAVE_ARGS, () => callerCurrent);
    callerCurrent = false;
    await host.answerCancels();
    expect(host.sent()).toEqual([]);
    expect(cancelledCode((await review).outcome)).toEqual([MBF_EXPORT_CANCELLED]);
    expect(cancelledCode((await save).outcome)).toEqual([MBF_EXPORT_CANCELLED]);
  });

  it("never sends an old start that was waiting on a retirement when the workspace unmounts", async () => {
    const host = nativeHost();
    const { unmount } = await renderedWithProject();
    await host.answerCancels();
    await replaceWithSameId(host);
    const review = reviewMbfExport(REVIEW_ARGS);
    const save = saveMbfExport(SAVE_ARGS);
    const before = host.cancels.length;
    unmount();
    await waitFor(() => expect(host.cancels.length).toBe(before + 1));
    await host.answerCancels();
    expect(host.sent()).toEqual([]);
    expect(cancelledCode((await review).outcome)).toEqual([MBF_EXPORT_CANCELLED]);
    expect(cancelledCode((await save).outcome)).toEqual([MBF_EXPORT_CANCELLED]);
  });

  it("under React Strict Mode, never sends a start from before the double effect and sends one from after it once", async () => {
    const host = nativeHost();
    // A start of an earlier workspace, waiting on that workspace's retirement.
    void retireMbfExportSession();
    const old = reviewMbfExport(REVIEW_ARGS);
    const oldSave = saveMbfExport(SAVE_ARGS);
    render(<StrictMode><App /></StrictMode>);
    await screen.findByTestId("desktop-preview-shell");
    // Strict Mode ran the workspace's effect, its cleanup (a retirement) and the effect again.
    expect(host.cancels.length).toBeGreaterThanOrEqual(2);
    const fresh = reviewMbfExport(REVIEW_ARGS);
    await act(async () => {});
    expect(host.sent()).toEqual([]);
    await host.answerCancels();
    // Only the start from after the double effect is sent, with the latest epoch.
    await waitFor(() => expect(host.sent()).toEqual(["review_mbf_export"]));
    expect(host.starts[0].ticket).toBe(nativeEpoch);
    expect(cancelledCode((await old).outcome)).toEqual([MBF_EXPORT_CANCELLED]);
    expect(cancelledCode((await oldSave).outcome)).toEqual([MBF_EXPORT_CANCELLED]);
    host.starts[0].reply.resolve(REVIEWED);
    const reviewed = await fresh;
    expect(reviewed.outcome).toEqual(REVIEWED);
    expect(reviewed.current()).toBe(true);
  });

  it("tags a delayed old completion so the new session discards it", async () => {
    const host = nativeHost();
    await renderedWithProject();
    await host.answerCancels();
    const old = reviewMbfExport(REVIEW_ARGS);
    await waitFor(() => expect(host.sent()).toEqual(["review_mbf_export"]));
    await replaceWithSameId(host);
    const fresh = reviewMbfExport(REVIEW_ARGS);
    await host.answerCancels();
    await waitFor(() => expect(host.sent()).toEqual(["review_mbf_export", "review_mbf_export"]));
    // The old start kept its ticket; the new one carries the epoch of the retirement between them.
    expect(host.starts[1].ticket).toBe(nativeEpoch);
    expect(host.starts[0].ticket).toBeLessThan(nativeEpoch);
    // The old session's reply arrives after the new session's start: it is not current, the new one is.
    host.starts[1].reply.resolve(REVIEWED);
    host.starts[0].reply.resolve(REVIEWED);
    const [late, current] = await Promise.all([old, fresh]);
    expect(late.current()).toBe(false);
    expect(current.current()).toBe(true);
    expect(late.generation).toBeLessThan(current.generation);
  });
});
