/** Issue 519 in the actual workspace session: a saved project's stored review proposal that lacks the
 * members the desktop reads is kept out of the session, named in the open's message, held only as a
 * notice in a ref private to the session, and kept as stored by a save in place (the browser
 * preview's store, read back; the desktop's request carries the keep member and no proposal). A
 * proposal of the session's own replaces it as any save does; a copy does not carry it and says so.
 * A well-formed stored proposal opens and saves exactly as before. All data are invented. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { useWorkspaceSession } from "./workspaceSession";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import * as projectService from "../../services/projectService";
import { createLocalProject, findLocalProject, getLocalStorageCapability, openLocalProject, saveLocalProject, saveLocalProjectKeepingStoredProposal } from "../../services/projectService";
import * as hashService from "../../services/hashService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import * as previewService from "../../services/previewService";
import { loadDesignKnowledge, loadSampleProposal } from "../../services/previewService";
import { REPLACE_REMOVES_STORED_PROPOSAL, STORED_PROPOSAL_NOT_ADMITTED, admitStoredProposal, storedEditorIntentsSentence, storedProposalSentence } from "./storedProposalAdmission";
import type { AgentProposal, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

/** Carried inside the unadmitted proposal's members: it must reach no view, report, export or agent read. */
const MARKER = "MARKER-519-UNSHOWN-PROPOSAL";
const UNADMITTED = { proposal_id: "proposal:invented", summary: `Invented proposal ${MARKER}.`, operations: [{ note: MARKER }], prompt: MARKER };

async function readySession() {
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  return hook;
}

/** A stored project (the browser preview's store) holding `proposal`, its envelope hash over it. */
async function storedWith(base: PreviewModel, id: string, proposal: unknown): Promise<PreviewModel> {
  const model = structuredClone(base);
  model.project.id = id;
  const modelHash = await computeModelHash(model);
  const envelopeHash = await computeProjectEnvelopeHash({ model, editor_intents: [], proposal: proposal as AgentProposal, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: modelHash });
  await createLocalProject(model, [], proposal as AgentProposal, null, null, null, modelHash, envelopeHash);
  return model;
}

/** Every string reachable from `root` (functions and DOM nodes aside) that contains `needle`. */
function reachable(root: unknown, needle: string): string[] {
  const seen = new WeakSet<object>(), found: string[] = [];
  const walk = (value: unknown, path: string, depth: number) => {
    if (typeof value === "string") { if (value.includes(needle)) found.push(path); return; }
    if (typeof value !== "object" || value === null || depth > 40 || value instanceof Node) return;
    if (seen.has(value)) return;
    seen.add(value);
    const entries = value instanceof Map ? [...value.entries()].map(([key, item], index) => [`${index}`, [key, item]] as const)
      : value instanceof Set ? [...value].map((item, index) => [`${index}`, item] as const)
      : Object.getOwnPropertyNames(value).map((key) => { try { return [key, (value as Json)[key]] as const; } catch { return [key, undefined] as const; } });
    for (const [key, item] of entries) walk(item, `${path}.${key}`, depth + 1);
  };
  walk(root, "session", 0);
  return found;
}

/** Open State of a saved state holding the open project's model, as the History panel does. Model states
 * live in the desktop store, so the runtime is native while the state is read (served by the invoke mock). */
async function openStateOfTheOpenProject(result: { current: ReturnType<typeof useWorkspaceSession> }) {
  const saved = structuredClone(result.current.model.model!);
  const project = saved.project.id;
  const summary = {
    project_id: project, state_id: `state:${"5".padStart(32, "0")}`, state_kind: "design_snapshot", created_at: "2026-11-21T10:00:01.000Z",
    parent_state_id: null, model_hash: (await computeModelHash(saved))!.value, record_hash: "r", labels_updated_at: "2026-11-21T10:00:01.000Z",
    child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "Invented state", tags: [], notes: [], assumptions: []
  };
  const wasNative = Boolean((window as Json).__TAURI_INTERNALS__);
  const previous = invokeMock.getMockImplementation();
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    switch (command) {
      case "list_model_states": return [summary];
      case "list_analysis_runs": return [];
      case "check_model_library_refs": return [];
      case "read_model_state": return { state: summary, record: {}, model: structuredClone(saved), model_document_version: { status: "current" } };
    }
    return previous ? previous(command, args) : null;
  });
  await act(async () => { await result.current.project.history.refreshStates(project); });
  let opened!: Promise<boolean>;
  act(() => { opened = result.current.project.history.openState(summary.state_id); });
  await waitFor(() => expect(result.current.project.history.historyOperation).toMatch(/^open_state/));
  await act(async () => { await opened; });
  expect(result.current.project.history.historyOperation).toBe("open_state");
  if (!wasNative) delete (window as Json).__TAURI_INTERNALS__;
  if (previous) invokeMock.mockImplementation(previous);
}

/** A second writer saves the stored project (as another window would), holding `proposal`. */
async function anotherWriterSaves(projectId: string, proposal: unknown) {
  const existing = (await findLocalProject(projectId))!;
  const stored = (await openLocalProject(projectId))!;
  const envelopeHash = await computeProjectEnvelopeHash({ model: stored.model, editor_intents: [], proposal: proposal as never, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: stored.model_hash });
  await saveLocalProject(stored.model, [], proposal as never, null, null, null, stored.model_hash, envelopeHash, existing.revision);
}

const notice = admitStoredProposal(UNADMITTED);
if (notice.admitted) throw new Error("the invented proposal must not be admitted");
const OPEN_SENTENCE = storedProposalSentence(notice.notice, "open");

describe("a stored proposal without its members, on the browser preview's store", () => {
  it("opens without it, names it, keeps it only in the store, and a save in place keeps it as stored", async () => {
    const { result } = await readySession();
    const model = await storedWith(result.current.model.model!, "project:issue-519-kept", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    expect(result.current.model.model?.project.id).toBe(model.project.id);
    expect(result.current.results.proposal).toBeNull();
    expect(result.current.project.projectMessage).toContain(OPEN_SENTENCE);
    expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match" });
    // Nothing the session hands any consumer holds the stored proposal.
    expect(reachable(result.current, MARKER)).toEqual([]);

    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(result.current.project.projectOperation).toBe("save");
    expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "kept"));
    expect(result.current.results.proposal).toBeNull();
    expect(reachable(result.current, MARKER)).toEqual([]);
    // The store holds it as it was, and its envelope hash agrees with it.
    const stored = await openLocalProject(model.project.id);
    expect(stored!.proposal).toEqual(UNADMITTED);
    expect(stored!.project_envelope_hash).toEqual(await computeProjectEnvelopeHash({
      model: stored!.model, editor_intents: stored!.editor_intents, proposal: UNADMITTED as never, selected_review_target: null,
      mechanics_result: null, analysis_run: null, model_hash: stored!.model_hash
    }));
    expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match", verification_source: "save" });
    // Opened again: the same sentence, the hash verified.
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    expect(result.current.project.projectMessage).toContain(OPEN_SENTENCE);
    expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match", verification_source: "open" });
  });

  it("is replaced by a proposal of the session's own, as any save does", async () => {
    const { result } = await readySession();
    const model = await storedWith(result.current.model.model!, "project:issue-519-own", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    await act(async () => { await result.current.results.handleProposal(); });
    const own = result.current.results.proposal!;
    expect(own).not.toBeNull();
    const keeping = vi.spyOn(projectService, "saveLocalProjectKeepingStoredProposal");
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(keeping).not.toHaveBeenCalled();
    expect(result.current.project.projectMessage).not.toContain(STORED_PROPOSAL_NOT_ADMITTED);
    expect((await openLocalProject(model.project.id))!.proposal).toEqual(own);
  });

  it("is not carried by a copy, and a Replace of the whole project says it is gone; each says so", async () => {
    const { result } = await readySession();
    const model = await storedWith(result.current.model.model!, "project:issue-519-copy", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    // A save in place after another writer moved the stored project: Save as a new project.
    const existing = (await findLocalProject(model.project.id))!;
    const stored = (await openLocalProject(model.project.id))!;
    await projectService.saveLocalProject(stored.model, [], UNADMITTED as never, null, null, null, stored.model_hash,
      { ...stored.project_envelope_hash!, value: "sha256:moved-by-another-writer" }, existing.revision);
    let saving!: Promise<void>;
    act(() => { saving = result.current.project.handleSaveProject(); });
    await waitFor(() => expect(result.current.project.replaceProjectPrompt).not.toBeNull());
    act(() => result.current.project.replaceProjectPrompt!.choose("save-as-new"));
    await act(async () => { await saving; });
    const copyId = result.current.model.model!.project.id;
    expect(copyId).toMatch(new RegExp(`^${model.project.id}:copy-`));
    expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "not-carried"));
    expect((await openLocalProject(copyId))!.proposal).toBeNull();
    expect((await openLocalProject(model.project.id))!.proposal).toEqual(UNADMITTED);

    // New Local Project over the opened project, Replace chosen: the whole stored project is replaced.
    const other = await storedWith(result.current.model.model!, "project:issue-519-replace", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(other.project.id); });
    let creating!: Promise<void>;
    act(() => { creating = result.current.project.handleCreateProject(); });
    await waitFor(() => expect(result.current.project.replaceProjectPrompt).not.toBeNull());
    act(() => result.current.project.replaceProjectPrompt!.choose("replace"));
    await act(async () => { await creating; });
    expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "replaced"));
    expect((await openLocalProject(other.project.id))!.proposal).toBeNull();
  });

  it("keeps it on the next save after Open State of the same project (issue 519 review, repair 1)", async () => {
    const { result } = await readySession();
    const model = await storedWith(result.current.model.model!, "project:issue-519-open-state", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    await openStateOfTheOpenProject(result);
    const keeping = vi.spyOn(projectService, "saveLocalProjectKeepingStoredProposal");
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(result.current.project.projectOperation).toBe("save");
    expect(keeping).toHaveBeenCalledTimes(1);
    expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "kept"));
    expect((await openLocalProject(model.project.id))!.proposal).toEqual(UNADMITTED);
    expect(reachable(result.current, MARKER)).toEqual([]);
  });

  for (const [what, written] of [["cleared it", null], ["wrote a different one", { ...UNADMITTED, proposal_id: "proposal:other-writer" }]] as const) {
    it(`is not kept by a Replace after another writer ${what}: main's Replace, the replaced sentence, the notice gone`, async () => {
      const { result } = await readySession();
      const model = await storedWith(result.current.model.model!, `project:issue-519-replace-${written ? "other" : "cleared"}`, UNADMITTED);
      await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
      await anotherWriterSaves(model.project.id, written);
      const keeping = vi.spyOn(projectService, "saveLocalProjectKeepingStoredProposal");
      const saving = vi.spyOn(projectService, "saveLocalProject");
      let pending!: Promise<void>;
      act(() => { pending = result.current.project.handleSaveProject(); });
      await waitFor(() => expect(result.current.project.replaceProjectPrompt).not.toBeNull());
      // The prompt says, before the choice, what a Replace does to the stored proposal.
      expect(result.current.project.replaceProjectPrompt!.note).toBe(REPLACE_REMOVES_STORED_PROPOSAL);
      act(() => result.current.project.replaceProjectPrompt!.choose("replace"));
      await act(async () => { await pending; });
      expect(result.current.project.projectOperation).toBe("save");
      // The first write asked to keep it and was refused as moved; the Replace is main's save.
      expect(keeping).toHaveBeenCalledTimes(1);
      expect(saving).toHaveBeenCalledTimes(1);
      const stored = (await openLocalProject(model.project.id))!;
      expect(stored.proposal).toBeNull();
      expect(stored.project_envelope_hash).toEqual(await computeProjectEnvelopeHash({
        model: stored.model, editor_intents: stored.editor_intents, proposal: null, selected_review_target: null,
        mechanics_result: null, analysis_run: null, model_hash: stored.model_hash
      }));
      expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "replaced"));
      // The notice is gone: the next save says nothing of a stored proposal and keeps none.
      await act(async () => { await result.current.project.handleSaveProject(); });
      expect(result.current.project.projectOperation).toBe("save");
      expect(result.current.project.projectMessage).not.toContain(STORED_PROPOSAL_NOT_ADMITTED);
      expect(keeping).toHaveBeenCalledTimes(1);
    });
  }

  it("is not carried by New Local Project then Save as a new project, and the copy's next save says nothing of it", async () => {
    const { result } = await readySession();
    const model = await storedWith(result.current.model.model!, "project:issue-519-create-copy", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    let creating!: Promise<void>;
    act(() => { creating = result.current.project.handleCreateProject(); });
    await waitFor(() => expect(result.current.project.replaceProjectPrompt).not.toBeNull());
    expect(result.current.project.replaceProjectPrompt!.note).toBe(REPLACE_REMOVES_STORED_PROPOSAL);
    act(() => result.current.project.replaceProjectPrompt!.choose("save-as-new"));
    await act(async () => { await creating; });
    const copyId = result.current.model.model!.project.id;
    expect(copyId).toMatch(new RegExp(`^${model.project.id}:copy-`));
    expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "not-carried"));
    const keeping = vi.spyOn(projectService, "saveLocalProjectKeepingStoredProposal");
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(result.current.project.projectOperation).toBe("save");
    expect(result.current.project.projectMessage).not.toContain(STORED_PROPOSAL_NOT_ADMITTED);
    expect(keeping).not.toHaveBeenCalled();
    expect((await openLocalProject(copyId))!.proposal).toBeNull();
    expect((await openLocalProject(model.project.id))!.proposal).toEqual(UNADMITTED);
  });

  /** Generate paused until `release`; the late result is the sample proposal. */
  async function pausedGenerate() {
    const sample = await loadSampleProposal(null, null);
    let release!: () => void;
    const paused = new Promise<void>((resolve) => { release = resolve; });
    vi.spyOn(previewService, "loadSampleProposal").mockImplementationOnce(async () => { await paused; return sample; });
    return { release };
  }

  it("does not adopt a Generate result that completes after another project was opened (issue 519 review, repair 5)", async () => {
    const { result } = await readySession();
    const first = await storedWith(result.current.model.model!, "project:issue-519-generate-first", null);
    const second = await storedWith(result.current.model.model!, "project:issue-519-generate-second", UNADMITTED);
    await act(async () => { await result.current.project.handleOpenProject(first.project.id); });
    const { release } = await pausedGenerate();
    let generating!: Promise<void>;
    act(() => { generating = result.current.results.handleProposal(); });
    await act(async () => { await result.current.project.handleOpenProject(second.project.id); });
    release();
    await act(async () => { await generating; });
    expect(result.current.results.proposal).toBeNull();
    // So the second project's save keeps its stored proposal.
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(result.current.project.projectMessage).toContain(storedProposalSentence(notice.notice, "kept"));
    expect((await openLocalProject(second.project.id))!.proposal).toEqual(UNADMITTED);
  });

  it("does not adopt a Generate result that completes after a model edit", async () => {
    const { result } = await readySession();
    const model = await storedWith(result.current.model.model!, "project:issue-519-generate-edit", null);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    const { release } = await pausedGenerate();
    let generating!: Promise<void>;
    act(() => { generating = result.current.results.handleProposal(); });
    const opened = result.current.model.model!;
    const target = opened.nodes[0];
    const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => opened.project.units.length, value: () => String(target.position.y) };
    const edit = buildGridOperationIntent({ column, model: opened, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: 1, value: String(target.position.y + 1.5), interaction: "cell" });
    await act(async () => { expect(await result.current.operations.handleApplyIntent(edit)).toBe(true); });
    expect(result.current.model.model).not.toBe(opened);
    release();
    await act(async () => { await generating; });
    expect(result.current.results.proposal).toBeNull();
  });

  for (const value of [0, "", false]) {
    it(`keeps a stored proposal of ${JSON.stringify(value)} as stored, counts it as none, and verifies its hash`, async () => {
      const { result } = await readySession();
      const model = await storedWith(result.current.model.model!, `project:issue-519-falsy-${typeof value}`, value);
      expect((await openLocalProject(model.project.id))!.proposal).toBe(value);
      await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
      expect(result.current.results.proposal).toBeNull();
      expect(result.current.project.projectMessage).toContain(STORED_PROPOSAL_NOT_ADMITTED);
      expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match" });
      await act(async () => { await result.current.project.handleSaveProject(); });
      expect(result.current.project.projectOperation).toBe("save");
      expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match", verification_source: "save" });
      const stored = (await openLocalProject(model.project.id))!;
      expect(stored.proposal).toBe(value);
      expect(stored.summary.proposal_count).toBe(0);
      await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
      expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match", verification_source: "open" });
    });
  }

  it("opens and saves a well-formed stored proposal exactly as before", async () => {
    const { result } = await readySession();
    const proposal = await loadSampleProposal(null, null);
    expect(admitStoredProposal(proposal).admitted).toBe(true);
    const model = await storedWith(result.current.model.model!, "project:issue-519-admitted", proposal);
    const opened = (await openLocalProject(model.project.id))!;
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    expect(result.current.results.proposal).toEqual(opened.proposal);
    expect(result.current.project.projectMessage).not.toContain(STORED_PROPOSAL_NOT_ADMITTED);
    const saving = vi.spyOn(projectService, "saveLocalProject");
    const keeping = vi.spyOn(projectService, "saveLocalProjectKeepingStoredProposal");
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(keeping).not.toHaveBeenCalled();
    expect(saving).toHaveBeenCalledTimes(1);
    const envelopeHash = await computeProjectEnvelopeHash({ model: opened.model, editor_intents: [], proposal: opened.proposal, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: opened.model_hash });
    // The request main's session sends for this store: its proposal, the hash over it, the base it opened.
    expect(saving.mock.calls[0]).toEqual([opened.model, [], opened.proposal, null, null, null, opened.model_hash, envelopeHash, null, opened.project_envelope_hash!.value]);
    expect((await openLocalProject(model.project.id))!.proposal).toEqual(proposal);
  });
});

describe("the browser preview store's keeping save (issue 519 review, repair 2)", () => {
  it("is refused as moved, writing nothing, when another write lands while its hash is computed", async () => {
    const hook = await readySession();
    const model = await storedWith(hook.result.current.model.model!, "project:issue-519-interleaved", UNADMITTED);
    const opened = (await openLocalProject(model.project.id))!;
    const base = opened.project_envelope_hash!.value;
    const other = { ...UNADMITTED, proposal_id: "proposal:second-writer" };
    const otherHash = await computeProjectEnvelopeHash({ model: opened.model, editor_intents: [], proposal: other as never, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: opened.model_hash });
    const real = hashService.computeProjectEnvelopeHash;
    let release!: () => void, entered!: () => void;
    const paused = new Promise<void>((resolve) => { release = resolve; });
    const hashing = new Promise<void>((resolve) => { entered = resolve; });
    vi.spyOn(hashService, "computeProjectEnvelopeHash").mockImplementationOnce(async (payload) => { entered(); await paused; return real(payload); });
    const keeping = saveLocalProjectKeepingStoredProposal(opened.model, [], null, null, null, opened.model_hash, null, base);
    await hashing;
    // The second writer saves in place from the same base while the first save's hash is paused.
    await saveLocalProject(opened.model, [], other as never, null, null, null, opened.model_hash, otherHash, null, base);
    release();
    await expect(keeping).rejects.toThrow(/^PROJECT-BASE-MOVED: /);
    const stored = (await openLocalProject(model.project.id))!;
    expect(stored.proposal).toEqual(other);
    expect(stored.project_envelope_hash).toEqual(otherHash);
  });
});

describe("stored editor intents that are not a list", () => {
  it("are not adopted, the open says so, and a save in place succeeds", async () => {
    const { result } = await readySession();
    const model = structuredClone(result.current.model.model!);
    model.project.id = "project:issue-519-intents";
    const intents = { damaged: MARKER } as never;
    const modelHash = await computeModelHash(model);
    const envelopeHash = await computeProjectEnvelopeHash({ model, editor_intents: intents, proposal: null, selected_review_target: null, mechanics_result: null, analysis_run: null, model_hash: modelHash });
    await createLocalProject(model, intents, null, null, null, null, modelHash, envelopeHash);
    await act(async () => { await result.current.project.handleOpenProject(model.project.id); });
    expect(result.current.model.model?.project.id).toBe(model.project.id);
    expect(result.current.operations.retainedReviewContext).toEqual([]);
    expect(result.current.project.projectMessage).toContain(storedEditorIntentsSentence());
    expect(result.current.project.projectEnvelopeHashIntegrity).toMatchObject({ integrity_status: "verified_match" });
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(result.current.project.projectOperation).toBe("save");
    expect((await openLocalProject(model.project.id))!.editor_intents).toEqual([]);
  });
});

describe("a stored proposal without its members, on the desktop store", () => {
  /** The native open's committed reply with no stored run and `proposal`; the save's request is captured. */
  async function openedNatively(proposal: unknown) {
    const reply = JSON.parse(readFileSync(resolve(__dirname, "../../test-support/native-reopen/native_reopen_replies.json"), "utf8")).open_refused.reply.value;
    reply.stored_run = { state: "none" };
    reply.proposal = proposal;
    const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
    const saves: Json[] = [];
    invokeMock.mockImplementation(async (command: string, args: Json) => {
      switch (command) {
        case "get_local_storage_capability": return storage;
        case "load_design_knowledge": return knowledge;
        case "sync_native_shell_state": return null;
        case "check_model_library_refs": return [];
        case "find_local_project": return null;
        case "list_model_states": return [];
        case "list_analysis_runs": return [];
        case "open_local_project_in_native_custody": return structuredClone(reply);
        case "save_local_project": saves.push(args.request); throw new Error("INVENTED-STOP: the request was captured");
      }
      return null;
    });
    const { result } = await readySession();
    (window as Json).__TAURI_INTERNALS__ = {};
    await act(async () => { await result.current.project.handleOpenProject(reply.model.project.id); });
    expect(result.current.model.model?.project.id).toBe(reply.model.project.id);
    await act(async () => { await result.current.project.handleSaveProject(); });
    return { result, reply, saves };
  }

  it("asks the store to keep the stored proposal, sending no proposal and no envelope hash", async () => {
    const { result, reply, saves } = await openedNatively(UNADMITTED);
    expect(saves).toHaveLength(1);
    expect(saves[0].keep_stored_proposal).toBe(true);
    expect("proposal" in saves[0]).toBe(false);
    expect("project_envelope_hash" in saves[0]).toBe(false);
    expect(saves[0].expected_base).toBe(reply.project_envelope_hash.value);
    expect(JSON.stringify(saves[0])).not.toContain(MARKER);
    expect(result.current.project.projectMessage).toContain("INVENTED-STOP");
  });

  it("asks the store to keep it on the save after Open State of the same project", async () => {
    const { result, reply, saves } = await openedNatively(UNADMITTED);
    expect(saves).toHaveLength(1);
    await openStateOfTheOpenProject(result);
    await act(async () => { await result.current.project.handleSaveProject(); });
    expect(saves).toHaveLength(2);
    expect(saves[1].keep_stored_proposal).toBe(true);
    expect("proposal" in saves[1]).toBe(false);
    expect(saves[1].expected_base).toBe(reply.project_envelope_hash.value);
  });

  it("sends a well-formed stored proposal as before, without the keep member", async () => {
    const proposal = await loadSampleProposal(null, null);
    const { saves } = await openedNatively(proposal);
    expect(saves).toHaveLength(1);
    expect("keep_stored_proposal" in saves[0]).toBe(false);
    expect(saves[0].proposal).toEqual(proposal);
  });
});
