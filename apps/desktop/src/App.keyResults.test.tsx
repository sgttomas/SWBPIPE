// A6 (DEL-07-05 Design `key-results-panel.md` §3, §6, §7): in the app, the key-results panel opens
// by itself on the session's first solve without taking focus from the field being edited, opens on
// demand afterwards, and its actions set the existing view state (the viewport's deformation case,
// the Results section, the Model stage) with no model edit, queued operation or undo entry. The
// solve is a unit transport replay of captured producer bytes for the invented preview model, NOT a
// native UI qualification witness.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge } from "./services/previewService";
import { createNativeMechanicsReplay, REPLAY_COMMANDS } from "./test/nativeMechanicsReplay";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function installPreviewReplay() {
  const storage = await getLocalStorageCapability();
  const knowledge = await loadDesignKnowledge();
  const mechanics = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return Promise.resolve(storage);
    if (command === "load_design_knowledge") return Promise.resolve(knowledge);
    if (command === "sync_native_shell_state") return Promise.resolve(null);
    if (REPLAY_COMMANDS.includes(command)) return mechanics.invoke(command, args, options);
    return Promise.reject(new Error(`NATIVE_APP_REPLAY_COMMAND_UNSUPPORTED: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}

async function solve() {
  const solveSection = screen.getByTestId("workspace-section-solve");
  if (solveSection.classList.contains("inactive")) nativeMenuCommand("view.section.solve");
  const run = within(screen.getByTestId("workspace-section-solve")).getByTestId("run-mechanics-preview");
  await waitFor(() => expect(run).toBeEnabled());
  fireEvent.click(run);
  return run;
}

describe("the key-results panel in the app", () => {
  it("opens by itself on the first solve, keeps focus in the field being edited, and opens only on demand after", async () => {
    await installPreviewReplay();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    expect(screen.queryByTestId("key-results-panel")).toBeNull();
    await solve();
    // The user is typing in a field while the solve runs.
    fireEvent.click(screen.getByRole("button", { name: "Find modeling commands" }));
    const field = screen.getByRole("searchbox", { name: "Find a tool" });
    field.focus();
    fireEvent.change(field, { target: { value: "pip" } });
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    const panel = await screen.findByTestId("key-results-panel");
    expect(panel).toHaveAttribute("data-state", "available");
    expect(document.activeElement).toBe(field);
    expect(field).toHaveValue("pip");
    fireEvent.keyDown(field, { key: "Escape" });

    // Escape from inside the panel closes it; a second solve does not reopen it.
    within(panel).getByTestId("key-results-results-table").focus();
    fireEvent.keyDown(document.activeElement!, { key: "Escape" });
    expect(screen.queryByTestId("key-results-panel")).toBeNull();
    expect(document.activeElement).toHaveAttribute("aria-current", "page");
    await solve();
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
    expect(screen.queryByTestId("key-results-panel")).toBeNull();

    // On demand, from the Results section's button.
    nativeMenuCommand("view.section.results");
    const opener = within(screen.getByTestId("workspace-section-results")).getByTestId("key-results-open");
    opener.focus();
    fireEvent.click(opener);
    expect(opener).toHaveAttribute("aria-pressed", "true");
    expect(document.activeElement).toBe(opener);
    expect(screen.getByTestId("key-results-panel")).toBeInTheDocument();
  }, 90_000);

  it("sets the deformation case, the Results section and the Model stage, and edits nothing", async () => {
    await installPreviewReplay();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    const panel = await screen.findByTestId("key-results-panel");
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    // Close the Solve page first: jsdom lays nothing out, so the shell's page-close focus rule would
    // otherwise treat every control as invisible and move focus to the rail.
    nativeMenuCommand("view.close-panels");

    // Deflected shape, by key: the viewport's deformation case is the panel's chosen case.
    const caseKey = (within(panel).getByTestId("key-results-deflection-case") as HTMLSelectElement).value;
    expect(caseKey).not.toBe("");
    expect(screen.getByTestId("viewport-deflection-case")).toHaveValue("");
    within(panel).getByTestId("key-results-deflection").focus();
    fireEvent.keyDown(document.activeElement!, { key: "d" });
    expect(screen.getByTestId("viewport-deflection-case")).toHaveValue(caseKey);

    // The results table: the Results stage, with the panel still open and focus kept in it.
    expect(screen.getByTestId("workspace-section-results").className).toContain("inactive");
    within(panel).getByTestId("key-results-results-table").focus();
    fireEvent.click(document.activeElement!);
    expect(screen.getByTestId("workspace-section-results").className).not.toContain("inactive");
    expect(screen.getByTestId("rail-stage-results")).toHaveAttribute("aria-current", "page");
    expect(panel.contains(document.activeElement)).toBe(true);

    // Back to editing: the Model stage, the panel closed, and focus on the Model stage's rail button.
    fireEvent.keyDown(within(panel).getByTestId("key-results-edit"), { key: "e" });
    expect(screen.queryByTestId("key-results-panel")).toBeNull();
    expect(screen.getByTestId("rail-stage-model")).toHaveAttribute("aria-current", "page");
    expect(document.activeElement).toBe(screen.getByTestId("rail-stage-model"));

    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    expect(screen.queryByTestId("project-edited")).toBeNull();
  }, 90_000);

  it("through the app: R says the pack is absent and colours nothing, S shows the anchor in A2, C says the linear result has no support state", async () => {
    await installPreviewReplay();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    const panel = await screen.findByTestId("key-results-panel");
    nativeMenuCommand("view.close-panels");

    within(panel).getByTestId("key-results-ratio").focus();
    fireEvent.keyDown(document.activeElement!, { key: "r" });
    expect(within(panel).getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "absent");
    expect(within(panel).getByTestId("key-results-status")).toHaveTextContent("No rule pack");
    expect(screen.getByTestId("toggle-viewport-result-colour")).toHaveAttribute("aria-pressed", "false");

    fireEvent.keyDown(document.activeElement!, { key: "c" });
    expect(within(panel).getByTestId("key-results-status")).toHaveTextContent("no nonlinear support state");

    fireEvent.keyDown(document.activeElement!, { key: "s" });
    expect(screen.getByTestId("status-selection")).toHaveTextContent("support: support:fixture-root");
    expect(screen.getByTestId("rail-stage-results")).toHaveAttribute("aria-current", "page");
    expect(within(screen.getByTestId("workspace-section-results")).getByTestId("support-load-select")).toHaveValue("support:fixture-root");
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
  }, 90_000);

  it("reopens after every solve once the user asks for it, for this session", async () => {
    await installPreviewReplay();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    fireEvent.click(await screen.findByTestId("key-results-every-solve"));
    fireEvent.click(screen.getByTestId("key-results-close"));
    expect(screen.queryByTestId("key-results-panel")).toBeNull();
    await solve();
    await waitFor(() => expect(screen.getByTestId("key-results-panel")).toBeInTheDocument(), { timeout: 30_000 });
    expect(screen.getByTestId("key-results-every-solve")).toBeChecked();
  }, 90_000);
});

describe("the key-results panel from the browser menu", () => {
  it("opens and closes from View > Key Results, and says there is no result yet", async () => {
    render(<App />);
    await screen.findByTestId("app-menu-bar");
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.key-results"));
    const panel = screen.getByTestId("key-results-panel");
    expect(panel).toHaveAttribute("data-state", "no_result");
    expect(within(panel).getByTestId("key-results-deflection")).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.key-results"));
    expect(screen.queryByTestId("key-results-panel")).toBeNull();
  });
});
