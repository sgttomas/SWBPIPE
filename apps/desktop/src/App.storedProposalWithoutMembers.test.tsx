/** Issue 519, the whole window: a saved project whose stored review proposal lacks the members the
 * desktop reads, opened by File > Open Local Project… on the native host's open reply (the committed
 * reply of `native_reopen_replies.json`, with no stored run and the proposal replaced), then
 * View > Project and the operations section's agent tab. Before admission at open, the Project page's
 * storage panel read `proposal.validation.application_status` of the persistence corpus's invented
 * proposal (`{ proposal_id, summary, operations }`), threw, and the window emptied. Now the proposal
 * is kept out of the session and the open says so by name; nothing of it reaches the window. All
 * data are invented. */
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { App } from "./App";
import { createSolveJobTransport, nativeMechanicsReplayPair } from "./test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "./services/previewService";
import { STORED_PROPOSAL_NOT_ADMITTED } from "./features/workspace/storedProposalAdmission";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const MARKER = "MARKER-519-UNSHOWN-PROPOSAL";

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

/** The committed open reply with no stored run, holding `proposal`. */
function openReply(proposal: unknown): Json {
  const reply = JSON.parse(readFileSync(resolve(__dirname, "test-support/native-reopen/native_reopen_replies.json"), "utf8")).open_refused.reply.value;
  reply.stored_run = { state: "none" };
  reply.proposal = proposal;
  return reply;
}

/** Renders the window, opens the project the host answers with `reply`, then View > Project. */
async function openThenProjectPage(reply: Json) {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport(() => bytes, { jobIdPrefix: "unit-stored-proposal" });
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
  await waitFor(() => expect(screen.getByTestId("local-project-message")).toHaveTextContent(reply.summary.message));
  act(() => nativeMenuCommand("view.section.project"));
  return container;
}

const CASES: Array<[string, unknown, string]> = [
  // The persistence corpus's invented proposal: issue 519's reproduction (no validation member).
  ["has no validation member", { proposal_id: "proposal:invented", summary: `Invented ${MARKER}.`, operations: [] }, "missing prompt, rationale, assumptions, operation, validation, audit_boundary and 1 more"],
  ["has no audit_boundary member", { ...wellFormed(), audit_boundary: undefined }, "missing audit_boundary"],
  ["has a member of another type", { ...wellFormed(), validation: `not an object ${MARKER}` }, "wrong type validation"],
  ["is not an object", [`${MARKER}`], "wrong type the proposal itself"]
];

function wellFormed(): Json {
  return {
    proposal_id: "proposal:invented-review", prompt: `Invented ${MARKER}.`, rationale: "Invented.", assumptions: [],
    operation: { operation_id: "op:invented", operation_kind: "attach_design_knowledge", operation_status: "draft_user_review_required", affected_entity_ids: [], changes: [] },
    validation: { application_status: "not_applied", diff_preview_status: "generated" },
    audit_boundary: { requires_user_acceptance: true, mutates_accepted_model_state: false, acceptance_recorded_as_review_only: true },
    professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_approval_claim: false }
  };
}

describe("the window after opening a saved project whose stored proposal lacks the members it reads", () => {
  for (const [what, proposal, members] of CASES) {
    it(`shows the Project page, the validation panel and the agent pane when the stored proposal ${what}`, async () => {
      const reply = openReply(JSON.parse(JSON.stringify(proposal)));
      const container = await openThenProjectPage(reply);
      await waitFor(() => expect(screen.getByTestId("workspace-section-project")).not.toHaveClass("inactive"));
      expect(screen.getByTestId("project-storage-panel")).toBeInTheDocument();
      expect(screen.getByTestId("project-validation-panel")).toBeInTheDocument();
      const message = screen.getByTestId("local-project-message");
      expect(message).toHaveTextContent(`A review proposal stored in this project was not opened, because this version cannot read it.`);
      expect(message).toHaveTextContent(`(${STORED_PROPOSAL_NOT_ADMITTED}:`);
      expect(message).toHaveTextContent(members);
      act(() => nativeMenuCommand("view.section.operations"));
      act(() => screen.getByTestId("operation-tab-agent").click());
      expect(await screen.findByTestId("agent-proposal-summary")).toHaveTextContent("No active proposal");
      expect(container.childElementCount).toBeGreaterThan(0);
      expect(document.body.innerHTML).not.toContain(MARKER);
    });
  }

  it("still adopts a well-formed stored proposal", async () => {
    const reply = openReply(wellFormed());
    await openThenProjectPage(reply);
    await waitFor(() => expect(screen.getByTestId("workspace-section-project")).not.toHaveClass("inactive"));
    expect(screen.getByTestId("local-project-message")).not.toHaveTextContent(STORED_PROPOSAL_NOT_ADMITTED);
    act(() => nativeMenuCommand("view.section.operations"));
    act(() => screen.getByTestId("operation-tab-agent").click());
    expect(await screen.findByTestId("agent-proposal-summary")).toHaveTextContent("proposal:invented-review");
  });
});
