import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { cancelMbfExport } from "./features/caepipe-mbf/mbfExportCancel";

// DEL-17-04 Design `desktop-mbf-export-import.md` §3.9.5 (slice 4b.2a): File > Export MBF…'s review and save in
// progress belong to the project session that started them. Retiring that session cancels them natively
// (`cancel_mbf_export`), independently of any model check (a session with no check in progress, and a project
// replaced by one with the same id, included), and so does unmounting the workspace.
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

describe("MBF export session retirement", () => {
  it("cancels the MBF export natively at each project session change and at unmount, with no model check in progress", async () => {
    const commands: string[] = [];
    invokeMock.mockImplementation((command: string) => {
      commands.push(command);
      if (command === "cancel_mbf_export") return Promise.resolve({ cancelled: 0, too_late: 0 });
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    const { unmount } = render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const message = () => screen.getByTestId("local-project-message");
    const cancels = () => commands.filter((command) => command === "cancel_mbf_export").length;
    // The browser preview's local store holds the project, so each open below replaces the session with the same id.
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Saved local browser-preview project snapshot"));
    const before = cancels();
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Opened local browser-preview project snapshot."));
    await waitFor(() => expect(cancels()).toBe(before + 1));
    // The same project id again: a new session all the same, so its run is cancelled again.
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(cancels()).toBe(before + 2));
    // No model check was in progress, and none was cancelled: the MBF cancel does not depend on one.
    expect(commands).not.toContain("check_preview_model");
    expect(commands).not.toContain("cancel_model_checks");
    unmount();
    await waitFor(() => expect(cancels()).toBe(before + 3));
  });

  it("returns the native receipt, and never throws when the host does not answer", async () => {
    invokeMock.mockResolvedValueOnce({ cancelled: 1, too_late: 0 });
    await expect(cancelMbfExport()).resolves.toEqual({ cancelled: 1, too_late: 0 });
    expect(invokeMock).toHaveBeenLastCalledWith("cancel_mbf_export");
    invokeMock.mockRejectedValueOnce(new Error("no native host"));
    await expect(cancelMbfExport()).resolves.toBeNull();
    invokeMock.mockImplementationOnce(() => { throw new Error("no native host"); });
    await expect(cancelMbfExport()).resolves.toBeNull();
  });
});
