// DEL-17-04 Design `desktop-mbf-export-import.md` §3.1 (4b.2b's follow-up, "Output gate removed"): the model batch
// file export carries no result data, so no result the session holds gates it. With the same model and settings, File >
// Export MBF… and the Exports section's Model batch file panel send the same native review and save (the canonical
// text, its hash, the settings, the suggested name, the reviewed digests, the route evidence and the ticket's shape)
// with no result, beside a preview-physics-2 (PF) result, an exact (pressure-2) result, a retained-precision route and a
// held result (an exact result held natively whose full copy the webview also holds), and the export sends no result
// read: no native result page, no full copy, no take.
// These run the real App: its menu, its Exports section, its solve and the dialog, against a mocked native host whose
// solves are unit transport replays of committed producer bytes (NOT a native UI qualification witness). The retained
// route cannot be delivered into an App session here (no product caller delivers a successor, and the native replay
// records no reply for its bytes), so that case solves the exact pair and reports its result's route as
// `retained_physics` to every reader in the App (`routeAs`). Invented data only.
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import reviewCases from "../../../fixtures/caepipe_mbf/invented/desktop_export/review_cases.json";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
/** The retained case's route double: a result whose run id is `runId` reads as `route` everywhere in the App. */
const routeAs = vi.hoisted(() => ({ runId: null as string | null, route: null as string | null }));
vi.mock("./features/results/resultReader", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./features/results/resultReader")>();
  return {
    ...actual,
    sourceContract: (source: Parameters<typeof actual.sourceContract>[0]) =>
      routeAs.runId !== null && source && (source as { run_id?: unknown }).run_id === routeAs.runId
        ? routeAs.route as ReturnType<typeof actual.sourceContract>
        : actual.sourceContract(source)
  };
});

import { App } from "./App";
import { RETIREMENT_ID, retireMbfExportSession, type MbfCancelReceipt, type MbfRetirementId } from "./features/caepipe-mbf/mbfExportCancel";
import type { MbfExportSaveOutcome, MbfReviewPayload } from "./features/caepipe-mbf/mbfExport";
import { getLocalStorageCapability } from "./services/projectService";
import { loadDesignKnowledge } from "./services/previewService";
import { createSolveJobTransport, DISPOSE_ALL_COMMAND, nativeMechanicsReplayPair, NATIVE_RESULT_COMMANDS, SOLVE_JOB_COMMANDS } from "./test/nativeMechanicsReplay";
import type { PreviewModel } from "./types";

type Args = Record<string, unknown>;
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const CASES = reviewCases.cases as Record<string, { review_text: string }>;
const REVIEW = JSON.parse(CASES["degc-default"].review_text) as MbfReviewPayload;
const SAVED: MbfExportSaveOutcome = {
  outcome: "saved",
  mbf: { file_name: "swbpipe-model.mbf", byte_count: 900, sha256: "a".repeat(64), replaced_existing: false, durability: "durable" },
  identity_file: { file_name: "swbpipe-model.swbpipe-ids.json", byte_count: 2000, sha256: "b".repeat(64), replaced_existing: false, durability: "durable" },
  durability: "durable",
  path_containment: "best_effort_non_adversarial",
  limitation: "invented limitation",
  warnings: []
};
/** The commands the export itself may send, and the session's shell sync. */
const EXPORT_COMMANDS = ["review_mbf_export", "save_mbf_export", "cancel_mbf_export", "check_model_library_refs", "sync_native_shell_state"];
/** Every command that reads a result, or a page or copy of one: the solve job's (take, full copy and its release,
 * dispose) and the native result commands (guarded rows, semantics, navigator, source recovery, verdict, summary and
 * record preview), and the restored record's and native custody's full reads. */
const RESULT_READ_COMMANDS = [
  ...SOLVE_JOB_COMMANDS.filter((command) => command !== "start_preview_mechanics_job" && command !== "poll_preview_mechanics_job" && command !== "cancel_preview_mechanics_job"),
  ...NATIVE_RESULT_COMMANDS,
  "read_restored_result_record_all",
  "read_analysis_run_in_native_custody"
];

let nativeEpoch = 0;
const receipt = (args: Args): MbfCancelReceipt => ({ cancelled: 0, too_late: 0, epoch: ++nativeEpoch, retirement_id: args[RETIREMENT_ID] as MbfRetirementId });

afterEach(async () => {
  invokeMock.mockImplementation((command: string, args?: Args) =>
    command === "cancel_mbf_export" ? Promise.resolve(receipt(args ?? {})) : Promise.reject(new Error(command)));
  await retireMbfExportSession();
  routeAs.runId = null;
  routeAs.route = null;
  invokeMock.mockReset();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

type Context = { model: PreviewModel; bytes: string };
const exact = (): Context => { const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" }); return { model: pair.model, bytes: pair.bytes }; };
const pf = (): Context => { const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }); return { model: pair.model, bytes: pair.bytes }; };

/** The mocked native host: the shell's commands, the export's three, and the solve's replay of `context.bytes`. */
async function nativeHost(context: Context) {
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport(() => context.bytes);
  const commands: Array<{ command: string; args: Args }> = [];
  invokeMock.mockImplementation(async (command: string, raw?: unknown, options?: unknown) => {
    const args = (raw && typeof raw === "object" && !ArrayBuffer.isView(raw) ? raw : {}) as Args;
    commands.push({ command, args });
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "load_preview_model": return structuredClone(context.model);
      case "check_model_library_refs": return [];
      case "cancel_mbf_export": return receipt(args);
      case "review_mbf_export": return { outcome: "reviewed", review: REVIEW };
      case "save_mbf_export": return SAVED;
      default:
        if (SOLVE_JOB_COMMANDS.includes(command) || command === DISPOSE_ALL_COMMAND) return transport.invoke(command, raw, options);
        throw new Error(`Unexpected command ${command}`);
    }
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  return { commands, transport };
}
type Host = Awaited<ReturnType<typeof nativeHost>>;

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}
function openSection(id: string) {
  const section = screen.queryByTestId(`workspace-section-${id}`);
  if (!section || section.classList.contains("inactive")) nativeMenuCommand(`view.section.${id}`);
  return screen.getByTestId(`workspace-section-${id}`);
}
const settle = () => act(async () => { await new Promise((resolve) => setTimeout(resolve, 100)); });
/** Waits until the App sends nothing for 500 ms: the other panels' own reads (the Exports section's result packets,
 * for example) finish before the export's commands are counted. */
async function quiet(host: Host) {
  for (let attempt = 0; attempt < 120; attempt += 1) {
    const sent = host.commands.length;
    await act(async () => { await new Promise((resolve) => setTimeout(resolve, 500)); });
    if (host.commands.length === sent) return;
  }
  throw new Error("the App did not settle");
}

/** Chooses Export MBF… until its dialog opens (it is refused by name until the session's model hash is shown). */
async function untilOpen(choose: () => void) {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    choose();
    try {
      await waitFor(() => expect(screen.getByTestId("mbf-export-dialog")).toBeInTheDocument(), { timeout: 500 });
      return screen.getByTestId("mbf-export-dialog");
    } catch {
      // Not open yet: choose again.
    }
  }
  throw new Error(`Export MBF… did not open: ${screen.queryByTestId("local-project-message")?.textContent ?? ""}`);
}
const ENTRIES = {
  menu: () => untilOpen(() => nativeMenuCommand("file.export-mbf")),
  panel: async () => {
    const panel = within(screen.getByTestId("workspace-section-exports")).getByLabelText("Model batch file (.mbf) export");
    expect(within(panel).queryByTestId("caepipe-mbf-load-reference-output-unavailable")).toBeNull();
    return untilOpen(() => fireEvent.click(within(panel).getByTestId("caepipe-mbf-export")));
  }
} as const;

/** One export from `entry`: review, the explicit choice, save, close. The review and save arguments with the ticket
 * replaced by its type (each open retires the one before, so the epoch moves on), and every command the export sent. */
async function exportOnce(host: Host, entry: keyof typeof ENTRIES) {
  if (entry === "panel") openSection("exports");
  await quiet(host);
  const from = host.commands.length;
  const opened = await ENTRIES[entry]();
  await within(opened).findByTestId("mbf-export-profile", {}, { timeout: 20000 });
  fireEvent.click(within(opened).getByTestId("mbf-export-intent"));
  fireEvent.click(within(opened).getByTestId("mbf-export-save"));
  await waitFor(() => expect(within(opened).getByTestId("mbf-export-status")).toHaveTextContent("Saved swbpipe-model.mbf"));
  fireEvent.click(within(opened).getByTestId("mbf-export-cancel"));
  await waitFor(() => expect(screen.queryByTestId("mbf-export-dialog")).toBeNull());
  await settle();
  const sent = host.commands.slice(from);
  const one = (command: string) => {
    const calls = sent.filter((item) => item.command === command);
    expect(calls, command).toHaveLength(1);
    const { expectedEpoch, ...rest } = calls[0].args;
    expect(typeof expectedEpoch, `${command} ticket`).toBe("number");
    return { ...rest, expectedEpoch: typeof expectedEpoch } as Args;
  };
  return { review: one("review_mbf_export"), save: one("save_mbf_export"), commands: sent.map((item) => item.command) };
}

/** Solves the session's model (the replay answers `context.bytes`) and waits until the result is shown. */
async function solve(host: Host) {
  const section = openSection("solve");
  await act(async () => { fireEvent.click(within(section).getByTestId("run-mechanics-preview")); });
  await waitFor(() => expect(host.transport.nativeHeld()).toHaveLength(1), { timeout: 20000 });
  openSection("results");
  await screen.findByTestId("numerical-result-standing", {}, { timeout: 20000 });
  await settle();
}

/** The export with no result, then (after `prepare`) beside the session's result, from both entry points. */
async function invariant(context: Context, prepare: (host: Host) => Promise<void>) {
  const host = await nativeHost(context);
  render(<App />);
  await screen.findByTestId(/^tree-row-project-/, {}, { timeout: 20000 });
  const none = { menu: await exportOnce(host, "menu"), panel: await exportOnce(host, "panel") };
  await prepare(host);
  const beside = { menu: await exportOnce(host, "menu"), panel: await exportOnce(host, "panel") };
  // The same native request from both entry points, with no result and beside it.
  for (const run of [none.panel, beside.menu, beside.panel]) {
    expect(run.review).toStrictEqual(none.menu.review);
    expect(run.save).toStrictEqual(none.menu.save);
  }
  expect(none.menu.review.canonicalText).toBe(none.menu.save.canonicalText);
  expect(none.menu.save.reviewed).toStrictEqual({ mbfSha256: REVIEW.mbf_sha256, sidecarSha256: REVIEW.sidecar_sha256 });
  // No result read: the export sends only its own commands, whatever the session holds.
  for (const run of [none.menu, none.panel, beside.menu, beside.panel]) {
    expect(run.commands.filter((command) => !EXPORT_COMMANDS.includes(command))).toStrictEqual([]);
    for (const command of RESULT_READ_COMMANDS) expect(run.commands.filter((sent) => sent === command), command).toHaveLength(0);
  }
  return { host, request: none.menu };
}

describe("Export MBF… has no result-output gate", () => {
  it("sends the same review and save beside a PF (preview-physics-2) result as with none", async () => {
    await invariant(pf(), async (host) => { await solve(host); });
  }, 120000);

  it("sends the same review and save beside an exact (pressure-2) result as with none", async () => {
    const { request } = await invariant(exact(), async (host) => {
      await solve(host);
      expect(within(openSection("results")).getByTestId("numerical-result-standing")).toHaveTextContent("3.0.0/exact_pressure_v3");
    });
    expect(JSON.parse(request.review.canonicalText as string).project.id).toBe(exact().model.project.id);
  }, 120000);

  it("sends the same review and save beside a result whose route reads as retained-precision as with none", async () => {
    await invariant(exact(), async (host) => {
      await solve(host);
      const runId = host.transport.nativeHeld()[0];
      expect(runId).toBeTruthy();
      // The double reads the session's result (its run id from the replayed bytes) as `retained_physics`.
      routeAs.runId = (JSON.parse(exact().bytes) as { run_id: string }).run_id;
      routeAs.route = "retained_physics";
      // Re-render the workspace on the double: a section change re-reads every panel's route.
      openSection("solve");
      openSection("exports");
      await settle();
    });
  }, 120000);

  it("sends the same review and save beside a held result (held natively, its full copy held in the webview) as with none", async () => {
    await invariant(exact(), async (host) => {
      await solve(host);
      // C2: the Results section loads the native result's full copy on request; the webview then holds it whole.
      const results = openSection("results");
      fireEvent.click(await within(results).findByTestId("c2-result-load"));
      await waitFor(() => expect(host.commands.filter((item) => item.command === "read_preview_mechanics_job_rows_all")).toHaveLength(1), { timeout: 20000 });
      expect(host.transport.nativeHeld()).toHaveLength(1);
    });
  }, 120000);
});
