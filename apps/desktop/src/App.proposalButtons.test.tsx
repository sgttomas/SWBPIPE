// The two sample proposal buttons ("Generate review proposal" in Details, the Agent tab's "Propose")
// read a native result's full copy before they build the proposal. Each press ends in one line at
// the button pressed: the copy's size refusal in the result gates' sentence (the size and the limit),
// another failure, or what the press made, including the same proposal as the one shown. While a
// press works, the button says so and neither button starts another. The solve is a unit transport
// replay of captured producer bytes for the invented preview model; the refusal is the native
// command's reply, as the gates' own app test reaches it. NOT a native UI qualification witness.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import { getLocalStorageCapability } from "./services/projectService";
import { describeFullResultRefused, FullResultRefused, RESULT_DETAIL_WEBVIEW_LIMIT, RESULT_DETAIL_WEBVIEW_LIMIT_BYTES } from "./services/nativeResult";
import { PROPOSAL_FEATURE } from "./features/agent-proposals/useProposalPress";
import { loadDesignKnowledge } from "./services/previewService";
import { createNativeMechanicsReplay, REPLAY_COMMANDS } from "./test/nativeMechanicsReplay";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const FULL_COPY = "read_preview_mechanics_job_rows_all";
const ABOVE = RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1;

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
  return mechanics;
}
/** The full copy answered by `answer` instead of the replay. */
function answerFullCopy(answer: (forward: () => Promise<unknown>) => Promise<unknown>) {
  const forward = invokeMock.getMockImplementation()!;
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => command === FULL_COPY
    ? answer(() => forward(command, args, options) as Promise<unknown>)
    : forward(command, args, options));
}
function refuseFullCopy() {
  answerFullCopy(() => Promise.reject({ code: "INVALID_REQUEST", message: `RESULT-DETAIL-WEBVIEW-LIMIT (result_bytes=${ABOVE}, limit_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES}): the result is above the bytes the webview can hold whole; nothing was copied, and the result stays held natively.` }));
}

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}
async function settle() {
  await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
}
async function solve() {
  nativeMenuCommand("view.section.solve");
  const run = within(screen.getByTestId("workspace-section-solve")).getByTestId("run-mechanics-preview");
  await waitFor(() => expect(run).toBeEnabled());
  fireEvent.click(run);
  await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
  await settle();
}
/** Opens the Operation Apply page if it is not open (choosing an open page again closes it), then the tab. */
function operations(tab: "details" | "agent") {
  if (screen.getByTestId("workspace-section-operations").classList.contains("inactive")) nativeMenuCommand("view.section.operations");
  const section = screen.getByTestId("workspace-section-operations");
  fireEvent.click(within(section).getByTestId(`operation-tab-${tab}`));
  return section;
}
const generateButton = () => within(screen.getByTestId("agent-proposal-panel")).getByTestId("generate-review-proposal");
const proposeButton = () => within(screen.getByTestId("agent-workbench-panel")).getByTestId("agent-generate-proposal");
const panelLine = () => within(screen.getByTestId("agent-proposal-panel")).findByTestId("generate-review-proposal-outcome", undefined, { timeout: 15_000 });
const workbenchLine = () => within(screen.getByTestId("agent-workbench-panel")).findByTestId("agent-generate-proposal-outcome", undefined, { timeout: 15_000 });

/** The words the result gates' own source gives for this refusal (the result's size and the limit),
 * read from that source rather than restated, with no error class name and no key=value form. */
function expectGateRefusal(line: HTMLElement) {
  const refusal = new FullResultRefused(RESULT_DETAIL_WEBVIEW_LIMIT, ABOVE, RESULT_DETAIL_WEBVIEW_LIMIT_BYTES, "refused");
  expect(line).toHaveAttribute("role", "status");
  expect(line.textContent).toBe(describeFullResultRefused(refusal, PROPOSAL_FEATURE));
  expect(line.textContent).toContain(String(ABOVE));
  expect(line.textContent).toContain(String(RESULT_DETAIL_WEBVIEW_LIMIT_BYTES));
  expect(line.textContent).not.toContain("FullResultRefused");
  expect(line.textContent).not.toMatch(/\w+=\S/);
}

describe("the sample proposal buttons", () => {
  it("each show the full copy's size refusal at the button, with the result's size and the limit", async () => {
    await installPreviewReplay();
    refuseFullCopy();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();

    operations("details");
    fireEvent.click(generateButton());
    expectGateRefusal(await panelLine());
    expect(screen.queryByTestId("proposal-body")).toBeNull();
    expect(generateButton()).toBeEnabled();

    operations("agent");
    fireEvent.click(proposeButton());
    expectGateRefusal(await workbenchLine());
    expect(screen.getByTestId("agent-proposal-summary")).toHaveTextContent("No active proposal");
    // The line is at the button pressed, not at the other.
    expect(within(screen.getByTestId("agent-proposal-panel")).queryByTestId("generate-review-proposal-outcome")).toBeNull();
  }, 90_000);

  it("says a proposal was made, and that Propose with that proposal shown made the same one", async () => {
    await installPreviewReplay();
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();

    operations("details");
    fireEvent.click(generateButton());
    expect(await panelLine()).toHaveTextContent("Review proposal made from this result.");
    expect(screen.getByTestId("proposal-body")).toBeInTheDocument();

    operations("agent");
    expect(screen.getByTestId("agent-proposal-summary")).toHaveTextContent("proposal:physics-diagnostic-review");
    fireEvent.click(proposeButton());
    const line = await workbenchLine();
    expect(line).toHaveAttribute("role", "status");
    expect(line).toHaveTextContent("Unchanged: the proposal made is the same as the one shown.");
    expect(screen.getByTestId("agent-proposal-summary")).toHaveTextContent("proposal:physics-diagnostic-review");
  }, 90_000);

  it("while a press works, says so at the button and starts no second press from either button", async () => {
    await installPreviewReplay();
    let release!: () => void;
    const held = new Promise<void>((resolve) => { release = resolve; });
    answerFullCopy((forward) => held.then(forward));
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();
    const copies = () => invokeMock.mock.calls.filter(([command]) => command === FULL_COPY).length;

    operations("agent");
    fireEvent.click(proposeButton());
    await waitFor(() => expect(proposeButton()).toHaveTextContent("Proposing…"));
    expect(proposeButton()).toBeDisabled();
    expect(proposeButton()).toHaveAttribute("aria-busy", "true");
    expect(proposeButton()).toHaveAccessibleName("Proposing…");
    expect(generateButton()).toBeDisabled();
    expect(generateButton()).toHaveAttribute("title", "A review proposal is being made.");
    const asked = copies();
    expect(asked).toBeGreaterThan(0);
    fireEvent.click(proposeButton());
    fireEvent.click(generateButton());
    await settle();
    expect(copies()).toBe(asked);

    await act(async () => { release(); });
    expect(await workbenchLine()).toHaveTextContent("Review proposal made from this result.");
    expect(proposeButton()).toHaveTextContent("Propose");
    expect(proposeButton()).toBeEnabled();
    expect(proposeButton()).not.toHaveAttribute("aria-busy");
  }, 90_000);

  it("ends another failure of the full copy in a line at the button, not silence", async () => {
    await installPreviewReplay();
    answerFullCopy(() => Promise.reject(new Error("the native full copy could not be read")));
    render(<App />);
    await screen.findByTestId("workspace-toolbar");
    await solve();

    operations("details");
    fireEvent.click(generateButton());
    const line = await panelLine();
    expect(line).toHaveAttribute("role", "alert");
    expect(line).toHaveTextContent("The review proposal could not be made: the native full copy could not be read");
    expect(line.textContent).not.toContain("Error:");
    expect(generateButton()).toHaveTextContent("Generate review proposal");
    expect(generateButton()).toBeEnabled();
  }, 90_000);
});
