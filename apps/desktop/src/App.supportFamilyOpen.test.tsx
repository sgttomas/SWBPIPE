// RT-S1/RT-U1: an existing model whose six-DOF support declares no `family`. Opening it, selecting
// that support and working the "New support" form short of queuing leave the document unchanged:
// no path adds `family` (the form's anchor suggestion is written only by the user's own queue). The
// key-results panel's supports line then says what is true of that model: no support is declared
// an anchor or equipment connection. The solve is a unit transport replay of captured producer
// bytes for the invented precision preview model, NOT a native UI qualification witness.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { KEY_SUPPORTS_NONE } from "./features/results/keyResults";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge } from "./services/previewService";
import { SOLVE_JOB_COMMANDS, createSolveJobTransport, nativeMechanicsReplayPair } from "./test/nativeMechanicsReplay";
import type { PreviewModel } from "./types";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}

/** The captured preview model with its one support's `family` removed: restrained in all six DOFs, declared nothing. */
function familyLessDocument(): PreviewModel {
  const document = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model;
  expect(document.supports).toHaveLength(1);
  delete (document.supports[0] as { family?: unknown }).family;
  return document;
}

describe("an opened model whose six-DOF support declares no family", () => {
  it("stays unchanged short of the user's queue, and key results say no support is declared an anchor", async () => {
    const opened = familyLessDocument();
    const support = opened.supports[0];
    expect(["UX", "UY", "UZ", "RX", "RY", "RZ"].every((dof) => support.restraints.includes(dof))).toBe(true);
    expect("family" in support).toBe(false);
    const storage = await getLocalStorageCapability();
    const knowledge = await loadDesignKnowledge();
    const solved: unknown[] = [];
    // The solve start refuses any model other than the opened document, so an added `family` fails the solve.
    const transport = createSolveJobTransport((request) => {
      if (!isDeepStrictEqual(request.model, opened)) throw new Error("SOLVE_MODEL_IS_NOT_THE_OPENED_DOCUMENT");
      solved.push(request.model);
      return nativeMechanicsReplayPair(request.mode as "sparse_interactive", { profile: "preview" }).bytes;
    });
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
      if (command === "get_local_storage_capability") return Promise.resolve(storage);
      if (command === "load_design_knowledge") return Promise.resolve(knowledge);
      if (command === "sync_native_shell_state") return Promise.resolve(null);
      if (command === "load_preview_model") return Promise.resolve(nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model);
      if (command === "open_model_document_file") {
        return Promise.resolve({ outcome: "opened", file_name: "family-less.json", byte_count: 1234, schema_status: "current", document: structuredClone(opened) });
      }
      if (SOLVE_JOB_COMMANDS.includes(command)) return transport.invoke(command, args, options);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    render(<App />);
    await screen.findByTestId("workspace-toolbar");

    nativeMenuCommand("file.open-model");
    await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("Opened model document family-less.json"));

    // Select the support: its configuration form shows the absent family as preserved, not as a suggestion.
    const tree = screen.getByLabelText("Model tree");
    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), { target: { value: support.id } });
    fireEvent.click(within(tree).getByTestId(`tree-row-support-${encodeURIComponent(support.id)}`));
    expect(document.querySelector('[aria-label="Support family"]')).toHaveTextContent("Not provided (preserved)");
    // Work the New support form up to the anchor suggestion, without queuing it.
    for (const dof of ["RX", "RY", "RZ"]) fireEvent.click(screen.getByTestId(`create-support-restraint-${dof}`));
    expect(screen.getByTestId("create-support-family-suggestion")).toHaveTextContent("Suggested from restraints: anchor");
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    expect(screen.queryByTestId("project-edited")).toBeNull();

    const solveSection = screen.getByTestId("workspace-section-solve");
    if (solveSection.classList.contains("inactive")) nativeMenuCommand("view.section.solve");
    const run = within(screen.getByTestId("workspace-section-solve")).getByTestId("run-mechanics-preview");
    await waitFor(() => expect(run).toBeEnabled());
    fireEvent.click(run);
    await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
    expect(solved).toEqual([opened]);
    expect("family" in (solved[0] as PreviewModel).supports[0]).toBe(false);

    const panel = await screen.findByTestId("key-results-panel");
    expect(KEY_SUPPORTS_NONE).toBe("No support is declared as an anchor or equipment connection.");
    expect(within(panel).getByTestId("key-results-support-reason")).toHaveTextContent(KEY_SUPPORTS_NONE);
    // True for this model: no support declares the anchor family or an equipment boundary.
    expect(opened.supports.some((item) => item.family === "anchor" || item.boundary_association)).toBe(false);
    expect(JSON.stringify(opened)).not.toContain("equipment_interfaces");
  }, 90_000);
});
