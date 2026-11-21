/** The Project page after a saved project whose stored run was refused at open (C1 slice 6,
 * `stored_custody`): the whole window, with the native host's refused-open reply supplied
 * (`test-support/native-reopen/native_reopen_replies.json`, `open_refused`: the actual command's
 * reply on a real store, written by `stored_custody_tests.rs`), opened by File > Open Local
 * Project…, then View > Project, where the refusal is shown. All data are invented. */
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { App } from "./App";
import { createSolveJobTransport, nativeMechanicsReplayPair } from "./test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "./services/previewService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

/** The committed refused open's reply. */
function refusedReply(): Json {
  const reply = JSON.parse(readFileSync(resolve(__dirname, "test-support/native-reopen/native_reopen_replies.json"), "utf8")).open_refused.reply.value;
  expect(reply.stored_run.state).toBe("refused");
  return reply;
}

/** Renders the window, opens the project the host answers with `reply`, then View > Project. */
async function openThenProjectPage(reply: Json) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: "unit-refused-project-page" });
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": if (args === undefined) return structuredClone(model); break;
      case "find_local_project": return null;
      case "list_model_states": return [];
      case "list_analysis_runs": return [];
      case "open_local_project_in_native_custody": return structuredClone(reply);
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  const { container } = render(<App />);
  await screen.findByTestId("workspace-toolbar");
  act(() => nativeMenuCommand("file.open-local"));
  await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent("STORED-RESULT-RESTORE-REFUSED"));
  act(() => nativeMenuCommand("view.section.project"));
  return container;
}

describe("the Project page after a saved project whose stored run was refused", () => {
  it("shows the Project section and the refusal when the project holds no proposal", async () => {
    const reply = refusedReply();
    expect(reply.proposal).toBeNull();
    const container = await openThenProjectPage(reply);
    await waitFor(() => expect(screen.getByTestId("workspace-section-project")).not.toHaveClass("inactive"));
    expect(screen.getByTestId("project-storage-panel")).toBeInTheDocument();
    expect(screen.getByTestId("project-validation-panel")).toBeInTheDocument();
    expect(screen.getByTestId("local-project-message")).toHaveTextContent(reply.stored_run.message);
    expect(container.childElementCount).toBeGreaterThan(0);
  });
});
