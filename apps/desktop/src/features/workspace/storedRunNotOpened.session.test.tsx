/** #547: the session says when the open left the project's stored run in the store unopened, so the
 * Project Storage Audit's snapshot line does not call the open's counts "persisted". Through the
 * actual workspace session, with the native job transport replay of committed producer bytes, the
 * native store's actual refused-open reply (`native_reopen_replies.json`, re-pointed at the replayed
 * model) and a fake of the native store's other commands (unit transport replay; NOT a native UI
 * witness). The flag follows the summary the open returned: a save in place succeeds only with a new
 * result, which replaces the stored run (`refuseDroppingUnopenedRun`; the store's
 * `refuse_dropping_withheld_run`), so the save's summary states what the store then holds. All data
 * are invented. */
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { createNativePersistenceFake } from "../../test/nativePersistenceFake";
import { NATIVE_SAVE_PROJECT_COMMAND } from "../../services/nativePersistence";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { ProjectStorageAuditPanel, STORED_RUN_NOT_OPENED } from "../project-storage/ProjectStorageAuditPanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const REPLIES = JSON.parse(readFileSync(resolve(__dirname, "../../test-support/native-reopen/native_reopen_replies.json"), "utf8"));
const REFUSED = REPLIES.open_refused.reply.value;

let sequence = 0;
async function session() {
  const { model, bytes } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-stored-run-not-opened:${++sequence}` });
  const saved: Json[] = [];
  let row = 0;
  const envelopeOf = (request: Json, message: string): LocalProjectEnvelope => ({
    summary: { ...REFUSED.summary, project_id: request.model.project.id, project_name: request.model.project.name, message,
      persisted_mechanics_result_count: request.mechanics_result ? 1 : 0, persisted_analysis_run_count: request.analysis_run ? 1 : 0,
      persisted_analysis_run_ref: request.analysis_run?.analysis_run?.run_id ?? "not_persisted" },
    model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: request.mechanics_result ?? null,
    analysis_run: request.analysis_run ?? null, model_hash: request.model_hash,
    // A stored revision's token, as the store's claim carries it (a digest of the invented row).
    project_envelope_hash: { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "project_envelope_payload", payload_excludes: "storage_summary_and_envelope_hash_carrier_fields",
      payload_ref: request.model.project.id, value: `sha256:${createHash("sha256").update(`row#${++row}`).digest("hex")}`, hash_status: "computed_local_preview" }, model_document_version: null
  }) as unknown as LocalProjectEnvelope;
  const fake = createNativePersistenceFake({
    heldBytes: transport.heldBytes,
    heldIntake: transport.heldIntake,
    save: ({ request }) => {
      saved.push(request);
      return envelopeOf(request, "Saved current project model snapshot without external file copies.");
    }
  });
  /** What `open_local_project_in_native_custody` returns next. */
  const native: { opened: Json | null } = { opened: null };
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
      case "open_local_project_in_native_custody": return structuredClone(native.opened);
      case "create_local_project": return envelopeOf({ model: args.model, mechanics_result: args.mechanicsResult, analysis_run: args.analysisRun, model_hash: args.modelHash }, "Created local SQLite project snapshot without external file copies.");
      case NATIVE_SAVE_PROJECT_COMMAND: return fake.invoke(command, args);
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    return null;
  }
  render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
  /** The store's refused-open reply for the replayed model, with `stored_run` and members as given. */
  const reply = (storedRun: Json, extra: Json = {}, id = model.project.id) => ({
    ...structuredClone(REFUSED),
    model: { ...structuredClone(model), project: { ...structuredClone(model.project), id } },
    summary: { ...REFUSED.summary, project_id: id, project_name: model.project.name },
    stored_run: storedRun,
    ...extra
  });
  return { state: () => current, native, saved, reply, model: model as PreviewModel };
}

async function open(state: () => WorkspaceSession, native: { opened: Json | null }, reply: Json) {
  native.opened = reply;
  await act(async () => { await state().project.handleOpenProject(reply.model.project.id); });
  expect(state().model.model?.project.id).toBe(reply.model.project.id);
}

/** The Project snapshot line as the Project page shows it for the session. */
function snapshotLine(state: () => WorkspaceSession): string {
  const { project, model } = state();
  const { unmount } = render(
    <ProjectStorageAuditPanel model={model.model!} storageCapability={project.storageCapability} projectSummary={project.projectSummary} projectIndex={null}
      projectMessage={project.projectMessage} projectOperation={project.projectOperation} editorIntents={[]} proposal={null} storedRunNotOpened={project.storedRunNotOpened} />
  );
  const line = screen.getByTestId("project-storage-snapshot").textContent ?? "";
  unmount();
  return line;
}

const UNTRUE = /persisted_mechanics_results|persisted_analysis_runs|persisted_analysis_run_ref/;

describe("a stored run the open left unopened (#547)", () => {
  it("(a) refused for size: the session says so and the snapshot line counts nothing as persisted", async () => {
    const { state, native, reply } = await session();
    await open(state, native, reply(REFUSED.stored_run));
    expect(state().project.projectMessage).toContain("The stored run is kept unchanged");
    expect(state().project.storedRunNotOpened).toBe(true);
    const line = snapshotLine(state);
    expect(line).toContain(STORED_RUN_NOT_OPENED);
    expect(line).not.toMatch(UNTRUE);
  });

  it("(b) withheld for size: the same", async () => {
    // The member as the store's envelope names it; the session reads it whichever reply carries it.
    const { state, native, reply } = await session();
    const withheld = { code: "STORED-RESULT-TOO-LARGE", result_bytes: 70_000_000, analysis_run_bytes: 1_000, limit_bytes: 67_108_864 };
    await open(state, native, reply({ state: "none" }, { stored_result_withheld: withheld }));
    expect(state().project.storedRunNotOpened).toBe(true);
    expect(snapshotLine(state)).toContain(STORED_RUN_NOT_OPENED);
  });

  it("(c) an ordinary open: false, and the line keeps its counts", async () => {
    const { state, native, reply } = await session();
    await open(state, native, reply({ state: "none" }));
    expect(state().project.storedRunNotOpened).toBe(false);
    const line = snapshotLine(state);
    expect(line).toContain("persisted_mechanics_results=0; persisted_analysis_runs=0; persisted_analysis_run_ref=not_persisted");
    expect(line).not.toContain(STORED_RUN_NOT_OPENED);
  });

  it("(d) a save in place without a new result is refused and the flag stays; a save with a new solve replaces the stored run and the flag clears", async () => {
    const { state, native, saved, reply } = await session();
    await open(state, native, reply(REFUSED.stored_run));
    await act(async () => { await state().project.handleSaveProject(); });
    expect(saved).toHaveLength(0);
    expect(state().project.projectMessage).toContain("STORED-RESULT-RESTORE-REFUSED: Not saved.");
    expect(state().project.storedRunNotOpened).toBe(true);
    await act(async () => { await state().results.handleRun(); });
    await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
    await act(async () => { await state().project.handleSaveProject(); });
    await waitFor(() => expect(saved).toHaveLength(1));
    // The save wrote the new solve's result and record over the stored run.
    expect(saved[0].mechanics_result).not.toBeNull();
    expect(saved[0].analysis_run).not.toBeNull();
    await waitFor(() => expect(state().project.storedRunNotOpened).toBe(false));
    expect(snapshotLine(state)).toContain("persisted_mechanics_results=1; persisted_analysis_runs=1");
  });

  it("(e) a new project clears it", async () => {
    const { state, native, reply } = await session();
    await open(state, native, reply(REFUSED.stored_run));
    expect(state().project.storedRunNotOpened).toBe(true);
    await act(async () => { await state().project.handleCreateBlankProject(); });
    await waitFor(() => expect(state().project.projectOperation, state().project.projectMessage).toBe("create_blank"));
    expect(state().project.storedRunNotOpened).toBe(false);
  });

  it("(e) opening another project clears it", async () => {
    const { state, native, reply } = await session();
    await open(state, native, reply(REFUSED.stored_run));
    expect(state().project.storedRunNotOpened).toBe(true);
    await open(state, native, reply({ state: "none" }, {}, "project:invented-other"));
    expect(state().project.storedRunNotOpened).toBe(false);
  });
});
