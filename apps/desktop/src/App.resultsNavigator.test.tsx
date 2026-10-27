// A1 (DEL-07-05 Design `results-navigator.md` §3, §6): in the app, a navigator step selects the
// current row's entity through the session's selection, which the viewport, the tree and the
// inspector draw from, and changes no model: nothing is edited, queued or put on the undo stack.
// The solve is a unit transport replay of captured producer bytes for the invented preview model,
// NOT a native UI qualification witness.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge } from "./services/previewService";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "./test/nativeMechanicsReplay";

afterEach(() => {
  // Unmount first: the session releases its delivered native result on unmount.
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function installPreviewReplay() {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const mechanics = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return Promise.resolve(storage);
    if (command === "load_design_knowledge") return Promise.resolve(knowledge);
    if (command === "sync_native_shell_state") return Promise.resolve(null);
    if (["load_preview_model", "start_preview_mechanics_job", "poll_preview_mechanics_job", "take_preview_mechanics_job_result", "dispose_preview_mechanics_result", "cancel_preview_mechanics_job"].includes(command)) return mechanics.invoke(command, args, options);
    return Promise.reject(new Error(`NATIVE_APP_REPLAY_COMMAND_UNSUPPORTED: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return pair;
}

function openSection(section: "solve" | "results") {
  const host = screen.getByTestId(`workspace-section-${section}`);
  // The replay sets the native runtime, so the View menu is the native one: send its command.
  if (host.classList.contains("inactive")) {
    act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: `view.section.${section}` })); });
  }
  return screen.getByTestId(`workspace-section-${section}`);
}

describe("the results navigator in the app", () => {
  it("moves the session's selection with each step and leaves the model, the queue and the undo stack alone", async () => {
    const pair = await installPreviewReplay();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    fireEvent.click(within(openSection("solve")).getByTestId("run-mechanics-preview"));
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    const results = within(openSection("results")).getByTestId("results-panel");
    const readout = screen.getByTestId("command-selection-readout");
    const before = readout.textContent;
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();

    // The Displacement table, then its first node: the session selects that node.
    while (!within(results).getByTestId("results-navigator-kind-position").textContent!.startsWith("Displacement")) {
      fireEvent.click(within(results).getByTestId("results-navigator-kind-next"));
    }
    fireEvent.click(within(results).getByTestId("results-navigator-entity-next"));
    const first = within(results).getByTestId("results-navigator-entity-position").textContent!.split(" (")[0];
    expect(pair.model.nodes.map((node) => node.id)).toContain(first);
    await waitFor(() => expect(readout).toHaveTextContent(`Selected node: ${first}; 0 queued`));
    expect(readout.textContent).not.toBe(before);

    // The key chord from the current row: the next node is selected, and focus follows its row.
    const current = within(results).getAllByRole("row").find((row) => row.getAttribute("aria-selected") === "true")!;
    fireEvent.keyDown(current, { key: "ArrowRight", altKey: true });
    const second = within(results).getByTestId("results-navigator-entity-position").textContent!.split(" (")[0];
    expect(second).not.toBe(first);
    await waitFor(() => expect(readout).toHaveTextContent(`Selected node: ${second}; 0 queued`));
    expect(document.activeElement?.getAttribute("aria-selected")).toBe("true");

    // A case step keeps the node; the selection stays on it.
    fireEvent.keyDown(document.activeElement!, { key: "ArrowDown", altKey: true });
    expect(within(results).getByTestId("results-navigator-case-position")).not.toHaveTextContent(/^All cases/);
    expect(readout).toHaveTextContent(`Selected node: ${second}; 0 queued`);

    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    expect(screen.queryByTestId("project-edited")).toBeNull();
  }, 60_000);
});
