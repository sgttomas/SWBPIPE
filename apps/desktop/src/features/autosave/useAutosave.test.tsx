// D5 slice 2 (DEL-02-05 Design `autosave-and-backup.md` §2, §3; Agent 0,
// 2026-10-28 items (a) and (b)): the autosave timer, its settings and status
// line, the discard after a landed save and the recovery offer, driven
// through the hook with the native commands mocked at the invoke boundary.
// Invented documents only.
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { LocalProjectIndexEntry, ModelHashEvidence, PreviewModel } from "../../types";
import { AutosaveRecoveryPrompt } from "./AutosaveRecoveryPrompt";
import { AutosaveSettingsControl } from "./AutosaveSettingsControl";
import { AutosaveStatus } from "./AutosaveStatus";
import { formatAutosaveTime } from "./autosaveRecovery";
import { autosaveSlot, type AutosaveEntry, type AutosaveReadOutcome, type AutosaveSettingsRead, type AutosaveWriteOutcome } from "./autosaveService";
import { AUTOSAVE_OFF_NOTICE, useAutosave, type AutosaveDeps, type AutosaveSession } from "./useAutosave";

const DEMO = demoModel as unknown as PreviewModel;
const OTHER_ID = "project:invented-autosave-older";
const SAVED_AT = 1_760_000_000;
const WRITTEN = 1_790_000_000_000;
let DEMO_HASH = "";
let DEMO_TEXT = "";
let DEMO_SLOT = "";
let OTHER_SLOT = "";

beforeAll(async () => {
  await loadWasmEngine();
  DEMO_TEXT = await canonicalJsonString(DEMO);
  DEMO_HASH = `sha256:${await canonicalSha256Hex(DEMO)}`;
  DEMO_SLOT = await autosaveSlot(DEMO.project.id);
  OTHER_SLOT = await autosaveSlot(OTHER_ID);
});

beforeEach(() => {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
});

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function entry(slot: string, writtenMillis: number, hash: string, name = `${slot}-${String(writtenMillis).padStart(13, "0")}-${hash.slice(7)}.json`): AutosaveEntry {
  return { file_name: name, slot, written_unix_millis: writtenMillis, model_hash: hash, byte_count: 1234 };
}

function stored(projectId: string, name: string, updated = SAVED_AT): LocalProjectIndexEntry {
  return { project_id: projectId, project_name: name, storage_mode: "local_sqlite", created_at_unix: updated - 100, updated_at_unix: updated };
}

type NativeState = {
  settings: AutosaveSettingsRead;
  entries: AutosaveEntry[];
  projects: LocalProjectIndexEntry[];
  read: (fileName: string) => AutosaveReadOutcome;
  write: (args: Record<string, unknown>) => Promise<AutosaveWriteOutcome>;
};

function written(millis = WRITTEN): AutosaveWriteOutcome {
  return { outcome: "written", file_name: `${DEMO_SLOT}-${millis}-x.json`, slot: DEMO_SLOT, written_unix_millis: millis, model_hash: DEMO_HASH, byte_count: DEMO_TEXT.length, durability: "durable", warnings: [] };
}

/** The native autosave commands over an in-memory state. */
function native(overrides: Partial<NativeState> = {}): NativeState {
  const state: NativeState = {
    settings: { settings: { autosave_minutes: 5, backup_folder: null, backup_keep: null }, source: "default", status: null },
    entries: [],
    projects: [],
    read: () => ({ outcome: "read", ...entry(DEMO_SLOT, WRITTEN, DEMO_HASH), byte_count: DEMO_TEXT.length, document: structuredClone(DEMO) }),
    write: () => Promise.resolve(written()),
    ...overrides
  };
  invokeMock.mockImplementation((command: string, args: Record<string, unknown> = {}) => {
    switch (command) {
      case "get_autosave_settings": return Promise.resolve(state.settings);
      case "set_autosave_settings": {
        const settings = { ...state.settings.settings, autosave_minutes: args.autosaveMinutes as number };
        state.settings = { settings, source: "file", status: null };
        return Promise.resolve({ outcome: "saved", settings });
      }
      case "write_project_autosave": return state.write(args);
      case "list_project_autosaves": return Promise.resolve({ outcome: "listed", entries: state.entries, not_listed: 0 });
      case "list_local_projects": return Promise.resolve(state.projects);
      case "read_project_autosave": return Promise.resolve(state.read(args.fileName as string));
      case "discard_project_autosave":
        state.entries = state.entries.filter((item) => item.file_name !== args.fileName);
        return Promise.resolve({ outcome: "discarded", file_name: args.fileName });
      default: return Promise.reject(new Error(`Unexpected command ${command}`));
    }
  });
  return state;
}

const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command);
const status = () => screen.getByTestId("autosave-status");

function Harness({ deps, expose }: { deps: AutosaveDeps; expose: (session: AutosaveSession) => void }) {
  const autosave = useAutosave(deps);
  expose(autosave);
  return (
    <div>
      <button type="button" data-testid="workspace-control">Workspace</button>
      <AutosaveSettingsControl autosave={autosave} />
      <AutosaveStatus autosave={autosave} />
      <AutosaveRecoveryPrompt prompt={autosave.recoveryPrompt} />
    </div>
  );
}

function mount(options: { edited?: boolean; busy?: boolean; working?: { name: string; edited: boolean } | null } = {}) {
  const edited = { current: options.edited ?? true };
  const busy = { current: options.busy ?? false };
  const modelHash = { value: DEMO_HASH } as ModelHashEvidence;
  const holds = { taken: 0, released: 0 };
  const openAutosave = vi.fn(async () => {});
  const deps: AutosaveDeps = {
    native: true,
    modelEditedRef: edited,
    saveBasisRef: { current: { model: DEMO, modelHash } },
    projectSessionBusy: () => busy.current,
    holdModal: () => { holds.taken += 1; let done = false; return () => { if (!done) { done = true; holds.released += 1; } }; },
    workingModel: () => options.working === undefined ? { name: "Working invented model", edited: true } : options.working,
    openAutosave
  };
  let session: AutosaveSession | null = null;
  render(<Harness deps={deps} expose={(value) => { session = value; }} />);
  return { edited, busy, holds, openAutosave, session: () => session! };
}

async function advance(ms: number) {
  await act(async () => { await vi.advanceTimersByTimeAsync(ms); });
}

const MINUTE = 60_000;

describe("the autosave setting", () => {
  it("is on by default, every 5 minutes, and writes the session's canonical text on that interval", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    native();
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    expect(screen.queryByTestId("autosave-off-notice")).toBeNull();
    await advance(5 * MINUTE - 1);
    expect(calls("write_project_autosave")).toHaveLength(0);
    await advance(1);
    await waitFor(() => expect(calls("write_project_autosave")).toHaveLength(1));
    expect(calls("write_project_autosave")[0][1]).toEqual({ canonicalText: DEMO_TEXT, claimedModelHash: DEMO_HASH });
    await waitFor(() => expect(status()).toHaveTextContent(`Autosaved at ${formatAutosaveTime(WRITTEN)}.`));
  });

  it("(a) turning it off shows the one-line notice that a crash can lose unsaved work, and stops writing", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    native();
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    fireEvent.change(screen.getByTestId("autosave-interval"), { target: { value: "0" } });
    await waitFor(() => expect(screen.getByTestId("autosave-off-notice")).toHaveTextContent(AUTOSAVE_OFF_NOTICE));
    expect(AUTOSAVE_OFF_NOTICE).toBe("Autosave is off: a crash can lose unsaved work.");
    expect(screen.getByTestId("autosave-off-status")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
    expect(calls("set_autosave_settings").map(([, args]) => args)).toEqual([{ autosaveMinutes: 0 }]);
    await advance(60 * MINUTE);
    expect(calls("write_project_autosave")).toHaveLength(0);
    // Back on: the notice goes and the chosen interval applies.
    fireEvent.change(screen.getByTestId("autosave-interval"), { target: { value: "1" } });
    await waitFor(() => expect(screen.queryByTestId("autosave-off-notice")).toBeNull());
    expect(screen.queryByTestId("autosave-off-status")).toBeNull();
    await advance(MINUTE);
    await waitFor(() => expect(calls("write_project_autosave")).toHaveLength(1));
  });

  // Codex r1 P2: the notice of item (a) stays for as long as autosave is off,
  // whatever status line comes later (Keep for later, a write that lands after
  // autosave was turned off).
  it("(a) keeps the off notice while autosave is off, through later status lines", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    const record = entry(DEMO_SLOT, (SAVED_AT + 3600) * 1000, "sha256:" + "d".repeat(64));
    let finish: (outcome: AutosaveWriteOutcome) => void = () => {};
    const state = native({
      settings: { settings: { autosave_minutes: 0, backup_folder: null, backup_keep: null }, source: "file", status: null },
      entries: [record],
      projects: [stored(DEMO.project.id, "Stored invented demo")],
      write: () => new Promise((resolve) => { finish = resolve; })
    });
    mount();
    expect(await screen.findByTestId("autosave-off-status")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
    fireEvent.click(await screen.findByTestId("autosave-recovery-keep"));
    await waitFor(() => expect(status()).toHaveTextContent("Autosave kept for later"));
    expect(screen.getByTestId("autosave-off-status")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
    // On, a write starts; off again while it runs; it lands after.
    state.entries = [];
    fireEvent.change(screen.getByTestId("autosave-interval"), { target: { value: "1" } });
    await waitFor(() => expect(screen.queryByTestId("autosave-off-status")).toBeNull());
    await advance(MINUTE);
    await waitFor(() => expect(calls("write_project_autosave")).toHaveLength(1));
    fireEvent.change(screen.getByTestId("autosave-interval"), { target: { value: "0" } });
    expect(await screen.findByTestId("autosave-off-status")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
    await act(async () => { finish(written()); });
    await waitFor(() => expect(status()).toHaveTextContent(`Autosaved at ${formatAutosaveTime(WRITTEN)}.`));
    expect(screen.getByTestId("autosave-off-status")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
  });

  it("a settings file that could not be used reads as the default, with its status line", async () => {
    native({ settings: { settings: { autosave_minutes: 5, backup_folder: null, backup_keep: null }, source: "default", status: { code: "AUTOSAVE-SETTINGS-VERSION-UNKNOWN", path: "", message: "The autosave settings file has version 2, which this app does not read; autosave uses the default, every 5 minutes." } } });
    mount();
    await waitFor(() => expect(status()).toHaveTextContent("Autosave settings: AUTOSAVE-SETTINGS-VERSION-UNKNOWN: The autosave settings file has version 2"));
    expect(screen.getByTestId("autosave-interval")).toHaveValue("5");
  });
});

describe("the autosave timer", () => {
  it("writes only while the model has unsaved edits, only on the interval, and never while the session is busy", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    native();
    const { edited, busy } = mount({ edited: false });
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    await advance(15 * MINUTE);
    expect(calls("write_project_autosave")).toHaveLength(0);
    edited.current = true;
    // An edit does not write at once: the next tick (20 minutes) does.
    await advance(5 * MINUTE - 1);
    expect(calls("write_project_autosave")).toHaveLength(0);
    await advance(1);
    await waitFor(() => expect(calls("write_project_autosave")).toHaveLength(1));
    busy.current = true;
    await advance(5 * MINUTE);
    expect(calls("write_project_autosave")).toHaveLength(1);
    busy.current = false;
    edited.current = false;
    await advance(5 * MINUTE);
    expect(calls("write_project_autosave")).toHaveLength(1);
  });

  it("keeps at most one autosave in flight: a tick that finds one running is skipped", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    let finish: (outcome: AutosaveWriteOutcome) => void = () => {};
    native({ write: () => new Promise((resolve) => { finish = resolve; }) });
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    await advance(5 * MINUTE);
    await waitFor(() => expect(calls("write_project_autosave")).toHaveLength(1));
    await advance(10 * MINUTE);
    expect(calls("write_project_autosave")).toHaveLength(1);
    await act(async () => { finish(written()); });
    await advance(5 * MINUTE);
    await waitFor(() => expect(calls("write_project_autosave")).toHaveLength(2));
  });

  it("shows a refusal as `Autosave not written: <code>: <message>` and retries at the next tick", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    const outcomes: Array<() => Promise<AutosaveWriteOutcome>> = [
      () => Promise.resolve({ outcome: "refused", diagnostics: [{ code: "AUTOSAVE-WRITE-FAILED", path: "", message: "invented disk full (ENOSPC)" }] }),
      () => Promise.reject("AUTOSAVE-WORKER-FAILED: the `write_project_autosave` worker did not complete."),
      () => Promise.resolve(written(WRITTEN + 1000))
    ];
    native({ write: () => outcomes.shift()!() });
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    await advance(5 * MINUTE);
    await waitFor(() => expect(status()).toHaveTextContent("Autosave not written: AUTOSAVE-WRITE-FAILED: invented disk full (ENOSPC)"));
    await advance(5 * MINUTE);
    await waitFor(() => expect(status()).toHaveTextContent("Autosave not written: AUTOSAVE-WORKER-FAILED: the `write_project_autosave` worker did not complete."));
    await advance(5 * MINUTE);
    await waitFor(() => expect(status()).toHaveTextContent(`Autosaved at ${formatAutosaveTime(WRITTEN + 1000)}.`));
    expect(calls("write_project_autosave")).toHaveLength(3);
  });
});

describe("after a save lands", () => {
  // Codex r1 P1: only a record holding the saved model's hash is proven to be
  // in the store. A record with another hash is kept, whatever its time: an
  // older one the user kept for later, and one another window wrote just
  // before or after the save.
  it("discards only that project's autosaves holding the saved model, never one with another hash", async () => {
    const state = native();
    const { session } = mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    const now = Date.now();
    const keptForLater = entry(DEMO_SLOT, 1_000_000_000_000, "sha256:" + "a".repeat(64));
    const otherWindowBeforeCutoff = entry(DEMO_SLOT, now - 1, "sha256:" + "b".repeat(64));
    const otherWindowAfter = entry(DEMO_SLOT, now + 3_600_000, "sha256:" + "c".repeat(64));
    const sameOlder = entry(DEMO_SLOT, 1_000_000_000_001, DEMO_HASH);
    const sameNewer = entry(DEMO_SLOT, now + 3_600_001, DEMO_HASH);
    const otherProject = entry(OTHER_SLOT, 1_000_000_000_000, DEMO_HASH);
    state.entries = [sameNewer, otherWindowAfter, otherWindowBeforeCutoff, sameOlder, keptForLater, otherProject];
    await act(async () => { await session().afterProjectSaved(DEMO.project.id, DEMO_HASH); });
    expect(calls("discard_project_autosave").map(([, args]) => args.fileName).sort()).toEqual([sameNewer.file_name, sameOlder.file_name].sort());
    expect(state.entries.map((item) => item.file_name)).toEqual([otherWindowAfter.file_name, otherWindowBeforeCutoff.file_name, keptForLater.file_name, otherProject.file_name]);
  });
});

describe("the recovery offer", () => {
  const newerRecord = () => entry(DEMO_SLOT, (SAVED_AT + 3600) * 1000, "sha256:" + "d".repeat(64));
  const olderRecord = () => entry(OTHER_SLOT, (SAVED_AT - 3600) * 1000, "sha256:" + "e".repeat(64));
  const projects = () => [stored(DEMO.project.id, "Stored invented demo"), stored(OTHER_ID, "Stored invented older")];

  async function offered() {
    const prompt = await screen.findByTestId("autosave-recovery-prompt");
    expect(prompt).toHaveAttribute("role", "alertdialog");
    expect(prompt).toHaveAttribute("aria-modal", "true");
    return prompt;
  }

  it("is shown only for a record newer than its project's last save whose model differs", async () => {
    const record = newerRecord();
    native({ entries: [record, olderRecord()], projects: projects() });
    const { session } = mount();
    await offered();
    fireEvent.click(screen.getByTestId("autosave-recovery-keep"));
    await waitFor(() => expect(screen.queryByTestId("autosave-recovery-prompt")).toBeNull());
    // The older record was not offered: one read, one prompt.
    expect(calls("read_project_autosave").map(([, args]) => args.fileName)).toEqual([record.file_name]);
    // Opening the stored project whose model is the record's: nothing to offer.
    await act(async () => { await session().afterStoredProjectOpened(DEMO.project.id, record.model_hash); });
    expect(screen.queryByTestId("autosave-recovery-prompt")).toBeNull();
    expect(calls("read_project_autosave")).toHaveLength(1);
    // Opening it with another stored model: the newer record is offered again.
    act(() => { void session().afterStoredProjectOpened(DEMO.project.id, "sha256:" + "f".repeat(64)); });
    await offered();
    expect(calls("read_project_autosave")).toHaveLength(2);
  });

  it("(b) says which is newer, in both directions, before offering anything", async () => {
    const record = newerRecord();
    const old = olderRecord();
    native({ entries: [record, old], projects: projects() });
    mount();
    await offered();
    expect(screen.getByTestId("autosave-recovery-newer")).toHaveTextContent(
      `The autosave is newer: it was written ${formatAutosaveTime(record.written_unix_millis)}, after the stored project \`Stored invented demo\` was last saved ${formatAutosaveTime(SAVED_AT * 1000)}.`
    );
    // The stored project newer than its autosave: said in the status line, not offered.
    expect(status()).toHaveTextContent(
      `The stored project \`Stored invented older\` is newer: it was last saved ${formatAutosaveTime(SAVED_AT * 1000)}, after its autosave was written ${formatAutosaveTime(old.written_unix_millis)}. The autosave is not offered and is kept.`
    );
  });

  it("says when the times cannot tell which is newer, and when there is no stored project", async () => {
    const sameSecond = entry(DEMO_SLOT, SAVED_AT * 1000 + 400, "sha256:" + "1".repeat(64));
    native({ entries: [sameSecond], projects: [stored(DEMO.project.id, "Stored invented demo")] });
    mount();
    await offered();
    expect(screen.getByTestId("autosave-recovery-newer")).toHaveTextContent("within the same second as the stored project's last save");
    cleanup();
    invokeMock.mockReset();
    native({ entries: [newerRecord()], projects: [] });
    mount();
    await offered();
    expect(screen.getByTestId("autosave-recovery-newer")).toHaveTextContent("No stored project has this model's project id, so there is no saved version to compare");
    expect(screen.getByTestId("autosave-recovery-stored")).toHaveTextContent("No stored project is changed");
  });

  it("shows what the record is, what it would replace, and that the stored project stays unchanged", async () => {
    const record = newerRecord();
    native({ entries: [record], projects: projects(), read: () => ({ outcome: "read", ...record, byte_count: 4321, document: structuredClone(DEMO) }) });
    mount();
    await offered();
    expect(screen.getByTestId("autosave-recovery-what")).toHaveTextContent(
      `Autosave of ${DEMO.project.name} (${DEMO.project.id}), written ${formatAutosaveTime(record.written_unix_millis)}, 4321 bytes, model hash dddddddddddd.`
    );
    expect(screen.getByTestId("autosave-recovery-what")).toHaveTextContent("Queued batches, proposals, review context and solve results are not in an autosave");
    expect(screen.getByTestId("autosave-recovery-replaces")).toHaveTextContent("Open autosave replaces the working model now open, `Working invented model`, which has unsaved edits");
    expect(screen.getByTestId("autosave-recovery-stored")).toHaveTextContent(
      `The stored project \`Stored invented demo\`, last saved ${formatAutosaveTime(SAVED_AT * 1000)}, stays unchanged until you save and confirm the replace prompt.`
    );
  });

  it("never applies a record by itself: Keep for later is the focused default, holds the window and keeps the record", async () => {
    native({ entries: [newerRecord()], projects: projects() });
    const opener = () => screen.getByTestId("workspace-control");
    const { openAutosave, holds } = mount();
    act(() => opener().focus());
    await offered();
    expect(screen.getByTestId("autosave-recovery-keep")).toHaveFocus();
    expect(screen.getByTestId("autosave-recovery-keep")).toHaveTextContent("Keep for later");
    expect(holds).toEqual({ taken: 1, released: 0 });
    expect(openAutosave).not.toHaveBeenCalled();
    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(screen.queryByTestId("autosave-recovery-prompt")).toBeNull());
    expect(holds).toEqual({ taken: 1, released: 1 });
    expect(openAutosave).not.toHaveBeenCalled();
    expect(calls("discard_project_autosave")).toHaveLength(0);
    expect(status()).toHaveTextContent("Autosave kept for later");
    await waitFor(() => expect(opener()).toHaveFocus());
  });

  it("Open autosave hands the record to the session's open, which runs the unsaved-changes guard", async () => {
    const record = newerRecord();
    native({ entries: [record], projects: projects() });
    const { openAutosave, holds } = mount();
    await offered();
    fireEvent.click(screen.getByTestId("autosave-recovery-open"));
    await waitFor(() => expect(openAutosave).toHaveBeenCalledWith(record.file_name, formatAutosaveTime(record.written_unix_millis)));
    // The hold is released before the session's guard can ask.
    expect(holds).toEqual({ taken: 1, released: 1 });
  });

  it("Discard asks once more, with Cancel the default, and only then removes the record", async () => {
    const record = newerRecord();
    native({ entries: [record], projects: projects() });
    mount();
    await offered();
    fireEvent.click(screen.getByTestId("autosave-recovery-discard"));
    expect(await screen.findByTestId("autosave-discard-detail")).toHaveTextContent("cannot be recovered afterwards");
    expect(screen.getByTestId("autosave-discard-cancel")).toHaveFocus();
    expect(calls("discard_project_autosave")).toHaveLength(0);
    fireEvent.click(screen.getByTestId("autosave-discard-cancel"));
    await waitFor(() => expect(screen.getByTestId("autosave-recovery-keep")).toHaveFocus());
    expect(calls("discard_project_autosave")).toHaveLength(0);
    fireEvent.click(screen.getByTestId("autosave-recovery-discard"));
    fireEvent.click(await screen.findByTestId("autosave-discard-confirm"));
    await waitFor(() => expect(calls("discard_project_autosave").map(([, args]) => args)).toEqual([{ fileName: record.file_name }]));
    await waitFor(() => expect(status()).toHaveTextContent(`Autosave written ${formatAutosaveTime(record.written_unix_millis)} discarded.`));
    expect(screen.queryByTestId("autosave-recovery-prompt")).toBeNull();
  });

  it("shows a record that fails verification as unreadable, with its code, and offers Discard only", async () => {
    const record = newerRecord();
    native({
      entries: [record],
      projects: projects(),
      read: () => ({ outcome: "refused", diagnostics: [{ code: "AUTOSAVE-INTEGRITY", path: "", message: "The autosave's bytes hash to sha256:0000, not the sha256:dddd its name records." }] })
    });
    const { openAutosave, session } = mount();
    await offered();
    expect(screen.getByTestId("autosave-recovery-unreadable")).toHaveTextContent("could not be read: AUTOSAVE-INTEGRITY: The autosave's bytes hash to sha256:0000");
    expect(screen.queryByTestId("autosave-recovery-open")).toBeNull();
    expect(screen.queryByTestId("autosave-recovery-replaces")).toBeNull();
    expect(screen.getByTestId("autosave-recovery-keep")).toHaveTextContent("Close");
    // Open is not a choice for it, even if asked for.
    act(() => session().recoveryPrompt!.choose("open"));
    expect(screen.getByTestId("autosave-recovery-prompt")).toBeInTheDocument();
    expect(openAutosave).not.toHaveBeenCalled();
    fireEvent.click(screen.getByTestId("autosave-recovery-discard"));
    fireEvent.click(await screen.findByTestId("autosave-discard-confirm"));
    await waitFor(() => expect(calls("discard_project_autosave")).toHaveLength(1));
  });
});
