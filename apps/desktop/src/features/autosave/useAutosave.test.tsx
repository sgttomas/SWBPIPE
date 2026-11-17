// D5 slice 2 (Agent 0,
// 2026-10-28 items (a) and (b)): the autosave timer, its settings and status
// line, the discard after a landed save and the recovery offer, driven
// through the hook with the native commands mocked at the invoke boundary.
// Invented documents only.
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
// The canonical text is made by the real engine; a test may hold its result
// back to replace the session while it is outstanding.
const serialization = vi.hoisted(() => ({ gate: null as Promise<void> | null }));
vi.mock("../../services/hashService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/hashService")>();
  return {
    ...actual,
    canonicalJsonString: async (value: unknown) => {
      const text = await actual.canonicalJsonString(value);
      if (serialization.gate) await serialization.gate;
      return text;
    }
  };
});

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { LocalProjectIndexEntry, ModelHashEvidence, PreviewModel } from "../../types";
import { AutosaveRecoveryPrompt } from "./AutosaveRecoveryPrompt";
import { AutosaveSettingsControl } from "./AutosaveSettingsControl";
import { AutosaveStatus } from "./AutosaveStatus";
import { formatAutosaveTime } from "./autosaveRecovery";
import { autosaveSlot, type AutosaveEntry, type AutosaveReadOutcome, type AutosaveSettingsRead, type AutosaveWriteOutcome } from "./autosaveService";
import type { BackupSettingsOutcome, BackupWriteOutcome } from "./backupService";
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
  serialization.gate = null;
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
  configureBackup: (args: Record<string, unknown>) => Promise<BackupSettingsOutcome>;
  backup: (args: Record<string, unknown>) => Promise<BackupWriteOutcome>;
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
    configureBackup: (args) => Promise.resolve({ outcome: "saved", settings: {
      ...state.settings.settings, backup_folder: args.enabled ? "/invented-local-backups" : null,
      backup_keep: args.backupKeep as number | null
    } }),
    backup: () => Promise.resolve({ outcome: "disabled" }),
    ...overrides
  };
  invokeMock.mockImplementation((command: string, args: Record<string, unknown> = {}) => {
    switch (command) {
      case "configure_project_backup": return state.configureBackup(args);
      case "write_project_backup": return state.backup(args);
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

function mount(options: { edited?: boolean; busy?: boolean; blockers?: Array<{ current: boolean }>; working?: { name: string; edited: boolean } | null } = {}) {
  const edited = { current: options.edited ?? true };
  const busy = { current: options.busy ?? false };
  const modelHash = { value: DEMO_HASH } as ModelHashEvidence;
  const basis: AutosaveDeps["saveBasisRef"] = { current: { model: DEMO, modelHash } };
  const holds = { taken: 0, released: 0 };
  const openAutosave = vi.fn(async () => {});
  const generation = { live: 0 };
  const deps: AutosaveDeps = {
    native: true,
    sessionGeneration: 0,
    currentSessionGeneration: () => generation.live,
    modelEditedRef: edited,
    saveBasisRef: basis,
    projectSessionBusy: () => busy.current || (options.blockers ?? []).some((blocker) => blocker.current),
    holdModal: () => { holds.taken += 1; let done = false; return () => { if (!done) { done = true; holds.released += 1; } }; },
    workingModel: () => options.working === undefined ? { name: "Working invented model", edited: true } : options.working,
    openAutosave
  };
  let session: AutosaveSession | null = null;
  const view = render(<Harness deps={deps} expose={(value) => { session = value; }} />);
  return { edited, busy, basis, deps, generation, holds, openAutosave, unmount: view.unmount, session: () => session! };
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

  it("shows a refusal as `Autosave not written: <code>: <message>` and retries it", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "setTimeout", "clearTimeout"] });
    const outcomes: Array<() => Promise<AutosaveWriteOutcome>> = [
      () => Promise.resolve({ outcome: "refused", diagnostics: [{ code: "AUTOSAVE-WRITE-FAILED", path: "", message: "invented disk full (ENOSPC)" }] }),
      () => Promise.reject("AUTOSAVE-WORKER-FAILED: the `write_project_autosave` worker did not complete."),
      () => Promise.resolve(written(WRITTEN + 1000))
    ];
    native({ write: () => outcomes.shift()!() });
    mount();
    await advance(0);
    expect(screen.getByTestId("autosave-interval")).toHaveValue("5");
    await advance(5 * MINUTE);
    expect(status()).toHaveTextContent("Autosave not written: AUTOSAVE-WRITE-FAILED: invented disk full (ENOSPC)");
    // Retried after 5 seconds, then 10 more.
    await advance(5_000);
    expect(status()).toHaveTextContent("Autosave not written: AUTOSAVE-WORKER-FAILED: the `write_project_autosave` worker did not complete.");
    await advance(10_000);
    expect(status()).toHaveTextContent(`Autosaved at ${formatAutosaveTime(WRITTEN + 1000)}.`);
    expect(calls("write_project_autosave")).toHaveLength(3);
  });
});

// DEL-02-05 REQ-032 cadence witnesses: a tick that cannot write is due, and the
// due save is attempted as soon as its blocker clears, with the model and hash
// current at that moment. Fake time and mocked native commands only: these
// show the hook's scheduling, not wall-clock timing or storage durability.
describe("the due autosave", () => {
  type Variant = { model: PreviewModel; text: string; hash: string };
  const variants: Variant[] = [];
  let replacement: Variant;
  async function variant(model: PreviewModel): Promise<Variant> {
    return { model, text: await canonicalJsonString(model), hash: `sha256:${await canonicalSha256Hex(model)}` };
  }
  beforeAll(async () => {
    for (let index = 0; index < 5; index += 1) {
      const model = structuredClone(DEMO);
      model.project.name = `Invented autosave edit ${index}`;
      variants.push(await variant(model));
    }
    const other = structuredClone(DEMO);
    other.project.id = OTHER_ID;
    other.project.name = "Invented replacement project";
    replacement = await variant(other);
  });

  const SECOND = 1000;
  let clock = 0;
  async function to(time: number) {
    await advance(time - clock);
    clock = time;
  }
  const writes = () => calls("write_project_autosave").map(([, args]) => args);
  const payload = (v: Variant) => ({ canonicalText: v.text, claimedModelHash: v.hash });
  const writtenFor = (args: Record<string, unknown>): AutosaveWriteOutcome => ({
    outcome: "written", file_name: `${DEMO_SLOT}-${WRITTEN}-x.json`, slot: DEMO_SLOT, written_unix_millis: WRITTEN,
    model_hash: args.claimedModelHash as string, byte_count: (args.canonicalText as string).length, durability: "durable", warnings: []
  });
  function show(basis: AutosaveDeps["saveBasisRef"], v: Variant, owned = true) {
    basis.current = { model: v.model, modelHash: owned ? ({ value: v.hash } as ModelHashEvidence) : null };
  }
  async function start(overrides: Partial<NativeState> = {}, options: Parameters<typeof mount>[0] = {}) {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "setTimeout", "clearTimeout"] });
    clock = 0;
    native({ write: (args) => Promise.resolve(writtenFor(args)), ...overrides });
    const view = mount(options);
    show(view.basis, variants[0]);
    await to(0);
    await to(0);
    expect(screen.getByTestId("autosave-interval")).toHaveValue("5");
    return view;
  }

  it("is on by default from the product's own default when the settings cannot be read", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "setTimeout", "clearTimeout"] });
    clock = 0;
    native({ write: (args) => Promise.resolve(writtenFor(args)) });
    const answer = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => command === "get_autosave_settings"
      ? Promise.reject(new Error("invented settings read failure")) : answer(command, args));
    const view = mount();
    show(view.basis, variants[0]);
    await to(0);
    await to(0);
    expect(screen.getByTestId("autosave-interval")).toHaveValue("5");
    expect(status()).toHaveTextContent("autosave uses the default, every 5 minutes");
    await to(5 * MINUTE);
    expect(writes()).toEqual([payload(variants[0])]);
  });

  it("records the current model and its hash at every tick across continuing edits", async () => {
    const { basis } = await start();
    const expected: Array<ReturnType<typeof payload>> = [];
    for (let tick = 1; tick <= 4; tick += 1) {
      await to(tick * 5 * MINUTE - 2 * MINUTE - 30 * SECOND);
      show(basis, variants[tick]);
      await to(tick * 5 * MINUTE);
      expected.push(payload(variants[tick]));
      expect(writes()).toEqual(expected);
    }
  });

  // (1) One coalesced due save, woken by each blocker's release.
  it("a tick missed while the session is busy is written once, with the newest model, as soon as the session is idle", async () => {
    const { basis, busy } = await start();
    await to(4 * MINUTE);
    busy.current = true;
    await to(6 * MINUTE);
    show(basis, variants[1]);
    await to(16 * MINUTE);
    show(basis, variants[2]);
    await to(17 * MINUTE);
    expect(writes()).toEqual([]);
    busy.current = false;
    await to(17 * MINUTE + SECOND);
    // Three missed ticks (5, 10 and 15 minutes) make one write, not three.
    expect(writes()).toEqual([payload(variants[2])]);
    await to(20 * MINUTE - SECOND);
    expect(writes()).toHaveLength(1);
  });

  it("waits through each blocker in turn and writes when the last one is released", async () => {
    const first = { current: false };
    const second = { current: false };
    await start({}, { blockers: [first, second] });
    first.current = true;
    await to(5 * MINUTE + 10 * SECOND);
    second.current = true;
    first.current = false;
    await to(5 * MINUTE + 20 * SECOND);
    expect(writes()).toEqual([]);
    second.current = false;
    await to(5 * MINUTE + 21 * SECOND);
    expect(writes()).toEqual([payload(variants[0])]);
  });

  it("a tick that finds the hash not yet computed for a changed model writes that model once its hash arrives", async () => {
    const { basis } = await start();
    await to(5 * MINUTE - 100);
    show(basis, variants[1], false);
    await to(5 * MINUTE + 100);
    expect(writes()).toEqual([]);
    show(basis, variants[1]);
    await to(5 * MINUTE + SECOND);
    expect(writes()).toEqual([payload(variants[1])]);
  });

  it("ticks missed while a write is in flight make one write, of the newest model, when it settles", async () => {
    const pending: Array<(outcome: AutosaveWriteOutcome) => void> = [];
    const { basis } = await start({ write: (args) => new Promise((resolve) => { pending.push(() => resolve(writtenFor(args))); }) });
    await to(5 * MINUTE);
    expect(writes()).toEqual([payload(variants[0])]);
    await to(7 * MINUTE);
    show(basis, variants[1]);
    await to(16 * MINUTE);
    expect(writes()).toHaveLength(1);
    await act(async () => { pending.shift()!(written()); });
    await to(16 * MINUTE + SECOND);
    // The late completion covered only the model it captured.
    expect(writes()).toEqual([payload(variants[0]), payload(variants[1])]);
  });

  it("a failed write is retried within seconds with the current model, not at the next tick", async () => {
    let refuse = true;
    const { basis } = await start({ write: (args) => Promise.resolve(refuse
      ? { outcome: "refused", diagnostics: [{ code: "AUTOSAVE-WRITE-FAILED", path: "", message: "invented disk full (ENOSPC)" }] }
      : writtenFor(args)) });
    await to(5 * MINUTE);
    expect(writes()).toEqual([payload(variants[0])]);
    refuse = false;
    show(basis, variants[1]);
    await to(5 * MINUTE + 30 * SECOND);
    expect(writes()).toEqual([payload(variants[0]), payload(variants[1])]);
    expect(status()).toHaveTextContent(`Autosaved at ${formatAutosaveTime(WRITTEN)}.`);
  });

  // (2) Identity across awaits: nothing obsolete is kept for a retry.
  it("a retry after Save as new changes the project id in the same session writes the current model, never the earlier one", async () => {
    // Every response is lost: each write's outcome is unknown.
    const { basis, unmount } = await start({ write: () => Promise.reject("AUTOSAVE-WORKER-FAILED: invented lost response.") });
    await to(5 * MINUTE);
    show(basis, replacement);
    await to(5 * MINUTE + 30 * SECOND);
    const [first, ...retries] = writes();
    expect(first).toEqual(payload(variants[0]));
    expect(retries.length).toBeGreaterThanOrEqual(1);
    for (const retry of retries) expect(retry).toEqual(payload(replacement));
    // An unknown outcome removes nothing from the hook's side.
    expect(calls("discard_project_autosave")).toHaveLength(0);
    const before = writes().length;
    unmount();
    await to(20 * MINUTE);
    expect(writes()).toHaveLength(before);
  });

  // (3) A manual save that lands while a due save waits.
  it("after a save that stored an earlier model lands, the due save writes the newer edits and the save's cleanup keeps them", async () => {
    const { basis, busy, session } = await start();
    await to(4 * MINUTE + 50 * SECOND);
    busy.current = true; // Save Local Project of variant 0 is running.
    await to(5 * MINUTE + 5 * SECOND);
    show(basis, variants[1]); // Edited while the save ran.
    const slot = DEMO_SLOT;
    const savedRecord = entry(slot, WRITTEN - 10, variants[0].hash);
    const newerRecord = entry(slot, WRITTEN + 10, variants[1].hash);
    const otherProject = entry(OTHER_SLOT, WRITTEN - 20, variants[0].hash);
    const listing = [savedRecord, newerRecord, otherProject];
    const answer = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => command === "list_project_autosaves"
      ? Promise.resolve({ outcome: "listed", entries: listing, not_listed: 0 }) : answer(command, args));
    let cleanup!: Promise<void>;
    act(() => { cleanup = session().afterProjectSaved(DEMO.project.id, variants[0].hash); });
    busy.current = false;
    await act(async () => { await cleanup; });
    await to(5 * MINUTE + 6 * SECOND);
    expect(writes()).toEqual([payload(variants[1])]);
    expect(calls("discard_project_autosave").map(([, args]) => args.fileName)).toEqual([savedRecord.file_name]);
  });

  it("writes nothing for a due save once a landed save holds exactly the session's model", async () => {
    const { busy, edited, session } = await start();
    busy.current = true;
    await to(5 * MINUTE + 5 * SECOND);
    act(() => { void session().afterProjectSaved(DEMO.project.id, variants[0].hash); });
    // The session's edited flag clears at its next render, after the release.
    busy.current = false;
    await to(5 * MINUTE + 6 * SECOND);
    edited.current = false;
    await to(9 * MINUTE);
    expect(writes()).toEqual([]);
  });

  // (4) The protection clock: delays, retries and failures never move it.
  it("a due save written late does not move the next tick", async () => {
    const { busy } = await start();
    busy.current = true;
    await to(5 * MINUTE + 30 * SECOND);
    busy.current = false;
    await to(5 * MINUTE + 31 * SECOND);
    expect(writes()).toHaveLength(1);
    await to(10 * MINUTE - SECOND);
    expect(writes()).toHaveLength(1);
    await to(10 * MINUTE);
    expect(writes()).toHaveLength(2);
  });

  it("a write that keeps failing is retried on a bounded schedule, never in a tight loop, and removes nothing", async () => {
    await start({ write: () => Promise.resolve({ outcome: "refused", diagnostics: [{ code: "AUTOSAVE-WRITE-FAILED", path: "", message: "invented disk full (ENOSPC)" }] }) });
    await to(5 * MINUTE);
    const attempts: number[] = [];
    let seen = writes().length;
    for (let second = 1; second < 5 * 60; second += 1) {
      await to(5 * MINUTE + second * SECOND);
      if (writes().length !== seen) { attempts.push(second); seen = writes().length; }
    }
    expect(attempts.length).toBeGreaterThanOrEqual(3);
    expect(attempts.length).toBeLessThanOrEqual(10);
    attempts.forEach((second, index) => expect(second - (attempts[index - 1] ?? 0)).toBeGreaterThanOrEqual(5));
    expect(calls("discard_project_autosave")).toHaveLength(0);
    expect(status()).toHaveTextContent("Autosave not written: AUTOSAVE-WRITE-FAILED");
  });

  it("turning autosave off ends a due save", async () => {
    const { basis } = await start();
    show(basis, variants[1], false);
    await to(5 * MINUTE + SECOND);
    await act(async () => { fireEvent.change(screen.getByTestId("autosave-interval"), { target: { value: "0" } }); });
    await to(5 * MINUTE + 2 * SECOND);
    show(basis, variants[1]);
    await to(5 * MINUTE + 10 * SECOND);
    expect(writes()).toEqual([]);
    expect(screen.getByTestId("autosave-off-notice")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
  });

  // Identity across awaits: a session replaced (a new generation, whether or
  // not the project id changes) while an attempt is outstanding. The
  // generation advances at once; the new basis renders a moment later.
  type View = Awaited<ReturnType<typeof start>>;
  function replaceSession(view: View) {
    view.generation.live += 1;
  }
  function renderSession(view: View, v: Variant) {
    view.deps.sessionGeneration = view.generation.live;
    show(view.basis, v);
  }
  const autosaveStatus = () => screen.queryByTestId("autosave-status");

  it("a session replaced while the canonical text is being made: nothing is written or shown for the replaced one", async () => {
    const view = await start();
    let release!: () => void;
    serialization.gate = new Promise((resolve) => { release = resolve; });
    await to(5 * MINUTE);
    replaceSession(view);
    await to(5 * MINUTE + 100);
    renderSession(view, variants[1]); // The same project id, reopened.
    await act(async () => { serialization.gate = null; release(); });
    await to(5 * MINUTE + SECOND);
    expect(writes()).toEqual([]);
    expect(autosaveStatus()).toBeNull();
    // The new session's own first tick writes its model.
    await to(10 * MINUTE);
    expect(writes()).toEqual([payload(variants[1])]);
  });

  it("a session replaced during a write that succeeds: nothing is shown, and the new session's missed tick is written when it settles", async () => {
    const pending: Array<() => void> = [];
    const view = await start({ write: (args) => new Promise((resolve) => { pending.push(() => resolve(writtenFor(args))); }) });
    await to(5 * MINUTE);
    replaceSession(view);
    await to(5 * MINUTE + 100);
    renderSession(view, variants[1]);
    await to(10 * MINUTE); // The new session's tick, with the old write still running.
    expect(writes()).toEqual([payload(variants[0])]);
    await act(async () => { pending.shift()!(); });
    expect(autosaveStatus()).toBeNull();
    await to(10 * MINUTE + SECOND);
    expect(writes()).toEqual([payload(variants[0]), payload(variants[1])]);
    expect(autosaveStatus()).toBeNull();
  });

  it.each([
    ["the same project id", () => variants[1]],
    ["another project id", () => replacement]
  ])("a session replaced during a write that fails (%s): nothing is shown and no retry runs in the new session", async (_, next) => {
    let fail!: () => void;
    let first = true;
    const view = await start({ write: (args) => {
      if (!first) return Promise.resolve(writtenFor(args));
      first = false;
      return new Promise((_resolve, reject) => { fail = () => reject("AUTOSAVE-WORKER-FAILED: invented lost response."); });
    } });
    await to(5 * MINUTE);
    replaceSession(view);
    await to(5 * MINUTE + 100);
    renderSession(view, next());
    await to(6 * MINUTE);
    await act(async () => { fail(); });
    await to(9 * MINUTE);
    expect(writes()).toEqual([payload(variants[0])]);
    expect(autosaveStatus()).toBeNull();
    await to(10 * MINUTE);
    expect(writes()).toEqual([payload(variants[0]), payload(next())]);
  });

  it("a replaced session's failed write neither retries nor delays the new session's missed tick, written when it settles", async () => {
    let fail!: () => void;
    let first = true;
    const view = await start({ write: (args) => {
      if (!first) return Promise.resolve(writtenFor(args));
      first = false;
      return new Promise((_resolve, reject) => { fail = () => reject("AUTOSAVE-WORKER-FAILED: invented lost response."); });
    } });
    await to(5 * MINUTE);
    replaceSession(view);
    await to(5 * MINUTE + 100);
    renderSession(view, variants[1]);
    await to(11 * MINUTE); // The new session's tick at 10 minutes waits for the old write.
    expect(writes()).toEqual([payload(variants[0])]);
    await act(async () => { fail(); });
    expect(autosaveStatus()).toBeNull();
    await to(11 * MINUTE + SECOND);
    expect(writes()).toEqual([payload(variants[0]), payload(variants[1])]);
  });

  it("the new session's retries start afresh: the replaced session's failures and retry do not reach it", async () => {
    const view = await start({ write: () => Promise.resolve({ outcome: "refused", diagnostics: [{ code: "AUTOSAVE-DIRECTORY-UNAVAILABLE", path: "", message: "invented" }] }) });
    await to(5 * MINUTE + 16 * SECOND); // Failed at 0, 5 and 15 s; next retry would be 35 s.
    expect(writes()).toHaveLength(3);
    replaceSession(view);
    await to(5 * MINUTE + 20 * SECOND);
    renderSession(view, variants[1]);
    await to(10 * MINUTE - SECOND);
    expect(writes()).toHaveLength(3);
    await to(10 * MINUTE);
    expect(writes()).toHaveLength(4);
    // Its first retry comes after 5 seconds, not after the replaced session's 40.
    await to(10 * MINUTE + 5 * SECOND);
    expect(writes()).toHaveLength(5);
    expect(writes()[4]).toEqual(payload(variants[1]));
  });

  it("a write outstanding when autosave is turned off, or the window closes, schedules nothing after it", async () => {
    let fail!: () => void;
    native({ write: () => new Promise((_resolve, reject) => { fail = () => reject("AUTOSAVE-WORKER-FAILED: invented lost response."); }) });
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval", "setTimeout", "clearTimeout"] });
    clock = 0;
    let view = mount();
    show(view.basis, variants[0]);
    await to(0);
    await to(5 * MINUTE);
    await act(async () => { fireEvent.change(screen.getByTestId("autosave-interval"), { target: { value: "0" } }); });
    await act(async () => { fail(); });
    await to(30 * MINUTE);
    expect(writes()).toHaveLength(1);
    expect(screen.getByTestId("autosave-off-notice")).toHaveTextContent(AUTOSAVE_OFF_NOTICE);
    cleanup();
    invokeMock.mockReset();
    clock = 0;
    let finish!: () => void;
    native({ write: (args) => new Promise((resolve) => { finish = () => resolve(writtenFor(args)); }) });
    view = mount();
    show(view.basis, variants[0]);
    await to(0);
    await to(5 * MINUTE);
    expect(writes()).toHaveLength(1);
    view.unmount();
    await act(async () => { finish(); });
    await to(30 * MINUTE);
    expect(writes()).toHaveLength(1);
  });

  it("a landed save's cleanup waits for the writes running and due, then lists; a replaced session's cleanup shows nothing", async () => {
    const pending: Array<() => void> = [];
    const view = await start({ write: (args) => new Promise((resolve) => { pending.push(() => resolve(writtenFor(args))); }) });
    await to(5 * MINUTE);
    show(view.basis, variants[1]);
    await to(10 * MINUTE); // Due while the first write runs.
    const answer = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => command === "list_project_autosaves"
      ? Promise.resolve({ outcome: "refused", diagnostics: [{ code: "AUTOSAVE-DIRECTORY-UNAVAILABLE", path: "", message: "invented" }] })
      : answer(command, args));
    let saved!: Promise<void>;
    act(() => { saved = view.session().afterProjectSaved(DEMO.project.id, variants[2].hash); });
    await act(async () => { pending.shift()!(); });
    await to(10 * MINUTE + SECOND);
    // The due write started when the first settled; the listing waits for it.
    expect(writes()).toEqual([payload(variants[0]), payload(variants[1])]);
    expect(calls("list_project_autosaves")).toHaveLength(1); // The start's check only.
    replaceSession(view);
    await act(async () => { pending.shift()!(); await saved; });
    expect(calls("list_project_autosaves")).toHaveLength(2);
    // The first write landed in its own session and said so; the replaced
    // session's cleanup refusal is not shown in the new one.
    expect(autosaveStatus()).toHaveTextContent(`Autosaved at ${formatAutosaveTime(WRITTEN)}.`);
    expect(autosaveStatus()).not.toHaveTextContent("Autosave not removed");
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
      `By this Mac's clock, the autosave is newer: it was written ${formatAutosaveTime(record.written_unix_millis)}, and the stored project \`Stored invented demo\` was last saved ${formatAutosaveTime(SAVED_AT * 1000)}.`
    );
    // The stored project newer than its autosave by the clock: said in the
    // status line, kept, and not offered at start.
    expect(status()).toHaveTextContent(
      `An autosave is kept. By this Mac's clock, the stored project \`Stored invented older\` is newer: it was last saved ${formatAutosaveTime(SAVED_AT * 1000)}, and the autosave was written ${formatAutosaveTime(old.written_unix_millis)}. A clock that was changed can make this order wrong.`
    );
  });

  // Codex r1 P2: a clock set back can make newer work look older. Such a
  // record is never hidden: it is stated as kept, and Recover autosave…
  // offers it with the clock-qualified comparison.
  it("keeps a differing record the clock calls older reachable: stated at start, offered by Recover autosave…", async () => {
    const rolledBack = entry(DEMO_SLOT, (SAVED_AT - 300) * 1000, "sha256:" + "9".repeat(64));
    native({ entries: [rolledBack], projects: [stored(DEMO.project.id, "Stored invented demo")] });
    const { openAutosave } = mount();
    await waitFor(() => expect(status()).toHaveTextContent("An autosave is kept."));
    expect(status()).toHaveTextContent("It is not known to hold the saved project's model. Recover autosave… (in Appearance, or the project panel) offers it.");
    expect(screen.queryByTestId("autosave-recovery-prompt")).toBeNull();
    fireEvent.click(screen.getByTestId("recover-autosave"));
    await offered();
    expect(screen.getByTestId("autosave-recovery-newer")).toHaveTextContent(
      `By this Mac's clock, the stored project \`Stored invented demo\` is newer: it was last saved ${formatAutosaveTime(SAVED_AT * 1000)}, and the autosave was written ${formatAutosaveTime(rolledBack.written_unix_millis)}. A clock that was changed can make this order wrong.`
    );
    fireEvent.click(screen.getByTestId("autosave-recovery-open"));
    await waitFor(() => expect(openAutosave).toHaveBeenCalledWith(rolledBack.file_name, formatAutosaveTime(rolledBack.written_unix_millis)));
  });

  it("Recover autosave… says so when there is nothing to recover", async () => {
    native({ entries: [], projects: projects() });
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    fireEvent.click(screen.getByTestId("recover-autosave"));
    await waitFor(() => expect(status()).toHaveTextContent("No autosave to recover."));
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


describe("opt-in backups", () => {
  it("requires private-copy intent, defaults to All, and sends no folder path or model", async () => {
    native();
    const view = mount();
    const button = await screen.findByTestId("backup-enable");
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute("title", "Check the private-copy statement above to choose a folder.");
    expect(screen.getByTestId("backup-disable")).toHaveAttribute("title", "Backups are already off.");
    expect(screen.getByText("Retention: All — delete nothing. Automatic rotation is unavailable.")).toBeInTheDocument();
    expect(screen.queryByLabelText("Backups to keep")).toBeNull();
    fireEvent.click(screen.getByLabelText("I intend to copy private project models to the local folder I choose and keep all copies."));
    expect(button).toBeEnabled();
    expect(button).not.toHaveAttribute("title");
    fireEvent.click(button);
    await waitFor(() => expect(calls("configure_project_backup").map(([, args]) => args)).toEqual([
      { enabled: true, backupKeep: null, explicitLocalPrivateIntent: true }
    ]));
    await waitFor(() => expect(screen.getByTestId("backup-folder")).toHaveTextContent("/invented-local-backups"));
    expect(view.holds).toEqual({ taken: 1, released: 1 });
    expect(calls("write_project_backup")).toHaveLength(0);
    fireEvent.click(screen.getByTestId("backup-disable"));
    await waitFor(() => expect(screen.getByTestId("backup-folder")).toHaveTextContent("Backups are off."));
    expect(calls("configure_project_backup")[1][1]).toEqual({ enabled: false, backupKeep: null, explicitLocalPrivateIntent: false });
  });

  it("an Enable that commits but loses its response is an unknown outcome; settings refresh and Disable stays available", async () => {
    const state = native({
      configureBackup: async (args) => {
        // Native commits the change, then the response is lost in transport.
        state.settings = { settings: { ...state.settings.settings, backup_folder: args.enabled ? "/invented-local-backups" : null }, source: "file", status: null };
        throw new Error("invented lost IPC response");
      }
    });
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    fireEvent.click(screen.getByLabelText("I intend to copy private project models to the local folder I choose and keep all copies."));
    fireEvent.click(screen.getByTestId("backup-enable"));
    await waitFor(() => expect(screen.getByTestId("backup-status")).toHaveTextContent("Backup settings outcome unknown"));
    expect(screen.getByTestId("backup-status")).not.toHaveTextContent("not changed");
    expect(screen.getByTestId("backup-status")).toHaveTextContent("The saved settings now show backups on, to /invented-local-backups. Turn backups off remains available.");
    expect(screen.getByTestId("backup-folder")).toHaveTextContent("/invented-local-backups");
    expect(screen.getByTestId("backup-disable")).toBeEnabled();
  });

  it("while a lost configuration response leaves the outcome unknown, Disable stays available even if the refresh shows off", async () => {
    let lose = true;
    const state = native({
      configureBackup: async (args) => {
        if (lose) { lose = false; throw new Error("invented lost IPC response"); }
        state.settings = { settings: { ...state.settings.settings, backup_folder: args.enabled ? "/invented-local-backups" : null }, source: "file", status: null };
        return { outcome: "saved", settings: state.settings.settings };
      }
    });
    mount();
    await waitFor(() => expect(screen.getByTestId("autosave-interval")).toHaveValue("5"));
    fireEvent.click(screen.getByLabelText("I intend to copy private project models to the local folder I choose and keep all copies."));
    fireEvent.click(screen.getByTestId("backup-enable"));
    await waitFor(() => expect(screen.getByTestId("backup-status")).toHaveTextContent("Backup settings outcome unknown (Error: invented lost IPC response). The saved settings now show backups off. Turn backups off remains available."));
    const disable = screen.getByTestId("backup-disable");
    await waitFor(() => expect(disable).toBeEnabled());
    expect(disable).not.toHaveAttribute("title");
    fireEvent.click(disable);
    await waitFor(() => expect(screen.getByTestId("backup-status")).toHaveTextContent("Backups are off. Existing copies are kept."));
    expect(calls("configure_project_backup").map(([, args]) => args)).toEqual([
      { enabled: true, backupKeep: null, explicitLocalPrivateIntent: true },
      { enabled: false, backupKeep: null, explicitLocalPrivateIntent: false }
    ]);
    // A definite outcome ends the uncertainty.
    await waitFor(() => expect(screen.getByTestId("backup-disable")).toBeDisabled());
    expect(screen.getByTestId("backup-disable")).toHaveAttribute("title", "Backups are already off.");
  });

  it("canceling the native chooser preserves settings and never starts a backup", async () => {
    native({ configureBackup: async () => ({ outcome: "cancelled" }) });
    const view = mount();
    await waitFor(() => expect(view.session().settings).not.toBeNull());
    await act(async () => { await view.session().configureBackup(true, null, true); });
    expect(view.session().settings?.backup_folder).toBeNull();
    expect(view.holds).toEqual({ taken: 1, released: 1 });
    expect(calls("write_project_backup")).toHaveLength(0);
  });

  it("checks native settings on save even when this window shows backups off, and reports refusal separately", async () => {
    native({ backup: async () => ({ outcome: "refused", diagnostics: [{ code: "BACKUP-FULL", path: "", message: "No space left." }] }) });
    const view = mount();
    await waitFor(() => expect(view.session().settings).not.toBeNull());
    await act(async () => { await view.session().backupProjectSaved(DEMO.project.id, DEMO_HASH, () => true); });
    expect(calls("write_project_backup").map(([, args]) => args)).toEqual([{ projectId: DEMO.project.id, expectedModelHash: DEMO_HASH }]);
    expect(screen.getByTestId("backup-status")).toHaveTextContent("Saved. Backup not written: BACKUP-FULL: No space left.");
    expect(calls("save_local_project")).toHaveLength(0);
  });

  it("a stale session cannot inherit the pending backup's completion status", async () => {
    let finish!: (value: BackupWriteOutcome) => void;
    native({ backup: () => new Promise((resolve) => { finish = resolve; }) });
    const view = mount();
    await waitFor(() => expect(view.session().settings).not.toBeNull());
    let current = true;
    let pending!: Promise<void>;
    act(() => { pending = view.session().backupProjectSaved(DEMO.project.id, DEMO_HASH, () => current); });
    current = false;
    await act(async () => { finish({ outcome: "refused", diagnostics: [{ code: "BACKUP-STALE", path: "", message: "Kept." }] }); await pending; });
    expect(screen.queryByTestId("backup-status")).toBeNull();
  });

  it("reports a created unverified copy distinctly and treats a lost response as unknown", async () => {
    const state = native({ backup: async () => ({ outcome: "unverified", file_name: "invented.swbpipe-backup.json", model_hash: DEMO_HASH, creation: "created", diagnostics: [{ code: "BACKUP-DURABILITY-UNVERIFIED", path: "", message: "Copy kept; sync failed." }] }) });
    const view = mount();
    await waitFor(() => expect(view.session().settings).not.toBeNull());
    await act(async () => { await view.session().backupProjectSaved(DEMO.project.id, DEMO_HASH, () => true); });
    expect(screen.getByTestId("backup-status")).toHaveTextContent("Saved. Backup copy not verified: invented.swbpipe-backup.json");
    state.backup = async () => { throw new Error("invented IPC response loss"); };
    await act(async () => { await view.session().backupProjectSaved(DEMO.project.id, DEMO_HASH, () => true); });
    expect(screen.getByTestId("backup-status")).toHaveTextContent("Saved. Backup outcome unknown:");
  });

  it("preserves and explains a preexisting finite preference without treating it as active retention", async () => {
    native({ settings: { settings: { autosave_minutes: 5, backup_folder: "/invented-local-backups", backup_keep: 3 }, source: "file", status: null } });
    mount();
    expect(await screen.findByTestId("backup-retention-unavailable")).toHaveTextContent("saved limit of 3 is unavailable and remains unchanged");
    expect(screen.queryByLabelText("Backups to keep")).toBeNull();
    expect(calls("configure_project_backup")).toHaveLength(0);
    expect(calls("write_project_backup")).toHaveLength(0);
    expect(calls("discard_project_autosave")).toHaveLength(0);
  });
});
