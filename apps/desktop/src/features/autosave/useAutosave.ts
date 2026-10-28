// The desktop side of autosave (D5 slice 2; DEL-02-05 Design
// `autosave-and-backup.md` §1–§3): the settings, the timer, its status line,
// the discard after a landed save and the recovery offer. Called only by the
// session (`useWorkspaceSession`), which owns the model, the guard and the
// modal hold.
//
// - Settings: read from the native file at start; on by default, every 5
//   minutes. Turning autosave off shows a one-line notice that a crash can
//   lose unsaved work (Agent 0, 2026-10-28, item (a)).
// - Timer: one interval. A tick writes only while the model has unsaved edits
//   (`modelEdited`), its hash is owned and no project operation or prompt owns
//   the session. At most one autosave is in flight; a tick that finds one
//   running is skipped. A refusal keeps the previous record (native) and the
//   next tick retries.
// - After a save lands: that project's records written up to then, and any
//   holding the saved model, are discarded; the store now holds the model.
// - Recovery: at start, and after a stored project opens, records are paired
//   with stored projects by slot and offered one at a time in a modal dialog.
//   Nothing is applied automatically; Open goes through the session's
//   unsaved-changes guard and adopts the record as Open Model Document does.
import { useEffect, useRef, useState } from "react";
import type { MutableRefObject } from "react";
import { canonicalJsonString } from "../../services/hashService";
import { nativeFailureText } from "../../services/nativeIngress";
import { listLocalProjects } from "../../services/projectService";
import { modelDocumentShapeDiagnostics, modelDocumentSlotValidation } from "../model-file/modelDocumentShape";
import type { ModelHashEvidence, PreviewModel } from "../../types";
import {
  AUTOSAVE_NOT_INCLUDED,
  classifyRecords,
  keptOlderText,
  formatAutosaveTime,
  shortHash,
  whichIsNewerText,
  type RecoveryCandidate,
  type StoredProjectInfo
} from "./autosaveRecovery";
import {
  autosaveSlot,
  DEFAULT_AUTOSAVE_MINUTES,
  DEFAULT_AUTOSAVE_SETTINGS,
  diagnosticsText,
  discardProjectAutosave,
  getAutosaveSettings,
  listProjectAutosaves,
  readProjectAutosave,
  setAutosaveSettings,
  writeProjectAutosave,
  type AutosaveSettings
} from "./autosaveService";

/** Item (a): the notice shown while autosave is off. */
export const AUTOSAVE_OFF_NOTICE = "Autosave is off: a crash can lose unsaved work.";

export type AutosaveRecoveryChoice = "open" | "keep" | "discard" | "discard-confirmed" | "discard-cancelled";

/** The recovery dialog's content (`AutosaveRecoveryPrompt.tsx`). */
export type AutosaveRecoveryPromptState = {
  fileName: string;
  /** What the record is, when it verified; null when it is unreadable. */
  record: { projectName: string; projectId: string; writtenAt: string; byteCount: number; shortHash: string } | null;
  /** `code: message` of an unreadable record. */
  unreadable: string | null;
  /** Item (b): which is newer. */
  newer: string;
  /** What Open would replace: the working model now open. */
  replaces: string;
  /** The stored project, which stays unchanged. */
  stored: string;
  notIncluded: string;
  /** Discard asks once more. */
  confirmingDiscard: boolean;
  /** Where focus was when the dialog opened; it returns there on close. */
  opener: HTMLElement | null;
  choose: (choice: AutosaveRecoveryChoice) => void;
};

export type AutosaveDeps = {
  /** The desktop app (native commands); off in the browser preview. */
  native: boolean;
  /** The session's `modelEdited`, read at each tick. */
  modelEditedRef: MutableRefObject<boolean>;
  /** The session's model and its hash, only while that hash is owned. */
  saveBasisRef: MutableRefObject<{ model: PreviewModel | null; modelHash: ModelHashEvidence | null }>;
  /** True while a project operation or prompt owns the session. */
  projectSessionBusy: () => boolean;
  /** Takes the shared modal hold (`modalHolds.ts`). */
  holdModal: () => () => void;
  /** The working model now open, for "what it would replace". */
  workingModel: () => { name: string; edited: boolean } | null;
  /** Runs the guard, then reads the record verified and adopts it. */
  openAutosave: (fileName: string, writtenAt: string) => Promise<void>;
};

type Scope = { projectId: string; storedHash: string | null } | null;
/** `automatic` (start, an open): modal offers for records the clock calls
 * newer, a status line for the rest. `all` (Recover Autosave…): every record
 * not proven to be its stored project's model is offered. */
type CheckMode = "automatic" | "all";

const IDLE_POLL_MS = 250;

export function useAutosave(deps: AutosaveDeps) {
  const depsRef = useRef(deps);
  depsRef.current = deps;
  const [settings, setSettings] = useState<AutosaveSettings | null>(null);
  const [statusLine, setStatusLine] = useState<string | null>(null);
  const [recoveryPrompt, setRecoveryPrompt] = useState<AutosaveRecoveryPromptState | null>(null);
  const [settingBusy, setSettingBusy] = useState(false);
  const inFlight = useRef<Promise<void> | null>(null);
  const checks = useRef<Promise<void>>(Promise.resolve());
  const mounted = useRef(true);

  // Settings, then the recovery check at start (the desktop runtime is fixed
  // for a session; the tests switch it on after an edit in the browser engine).
  useEffect(() => {
    mounted.current = true;
    if (!deps.native) return () => { mounted.current = false; };
    void (async () => {
      try {
        const read = await getAutosaveSettings();
        if (!mounted.current) return;
        if (typeof read?.settings?.autosave_minutes !== "number") throw new Error("AUTOSAVE-SETTINGS-RESPONSE: the settings response has no interval");
        setSettings(read.settings);
        if (read.status) setStatusLine(`Autosave settings: ${read.status.code}: ${read.status.message}`);
      } catch (error) {
        if (!mounted.current) return;
        setSettings(DEFAULT_AUTOSAVE_SETTINGS);
        setStatusLine(`Autosave settings could not be read (${nativeFailureText(error)}); autosave uses the default, every ${DEFAULT_AUTOSAVE_MINUTES} minutes.`);
      }
    })();
    scheduleRecoveryCheck(null);
    return () => { mounted.current = false; };
  }, [deps.native]);

  const minutes = settings?.autosave_minutes ?? 0;
  useEffect(() => {
    if (!deps.native || minutes <= 0) return;
    const timer = window.setInterval(() => { void tick(); }, minutes * 60_000);
    return () => window.clearInterval(timer);
  }, [deps.native, minutes]);

  /** One tick of the timer. */
  async function tick(): Promise<void> {
    // One in flight: a tick that finds one running is skipped.
    if (inFlight.current) return;
    const current = depsRef.current;
    if (!current.modelEditedRef.current || current.projectSessionBusy()) return;
    const { model, modelHash } = current.saveBasisRef.current;
    if (!model || !modelHash) return;
    const run = (async () => {
      try {
        const text = await canonicalJsonString(model);
        const outcome = await writeProjectAutosave(text, modelHash.value);
        if (!mounted.current) return;
        if (outcome.outcome === "written") {
          const warning = outcome.warnings.length > 0 ? ` ${diagnosticsText(outcome.warnings)}` : "";
          setStatusLine(`Autosaved at ${formatAutosaveTime(outcome.written_unix_millis)}.${warning}`);
        } else if (outcome.outcome === "refused") {
          setStatusLine(`Autosave not written: ${diagnosticsText(outcome.diagnostics)}`);
        }
      } catch (error) {
        if (mounted.current) setStatusLine(`Autosave not written: ${nativeFailureText(error)}`);
      }
    })();
    inFlight.current = run;
    try {
      await run;
    } finally {
      inFlight.current = null;
    }
  }

  /** The interval control: 0 is off. */
  async function chooseInterval(next: number): Promise<void> {
    if (!depsRef.current.native || settingBusy) return;
    setSettingBusy(true);
    try {
      const outcome = await setAutosaveSettings(next);
      if (!mounted.current) return;
      if (outcome.outcome === "saved") {
        setSettings(outcome.settings);
        // Off: the notice of item (a) is the persistent `offNotice`, not a status line.
        setStatusLine(outcome.settings.autosave_minutes === 0 ? null : `Autosave every ${outcome.settings.autosave_minutes} minutes while there are unsaved edits.`);
      } else {
        setStatusLine(`Autosave setting not changed: ${diagnosticsText(outcome.diagnostics)}`);
      }
    } catch (error) {
      if (mounted.current) setStatusLine(`Autosave setting not changed: ${nativeFailureText(error)}`);
    } finally {
      if (mounted.current) setSettingBusy(false);
    }
  }

  /** Save Local Project landed for `projectId` with `savedHash`: the store now
   * holds that model, so the project's records holding exactly that model are
   * discarded. A record with any other hash is never removed by a save: only
   * its hash proves its work is in the store, and no time can (another window
   * may write newer edits after the save; the user may have kept it for
   * later). A running autosave finishes first, so its record is checked too. */
  async function afterProjectSaved(projectId: string, savedHash: string | null): Promise<void> {
    if (!depsRef.current.native || savedHash === null) return;
    try {
      await inFlight.current;
      const slot = await autosaveSlot(projectId);
      const listed = await listProjectAutosaves();
      if (listed?.outcome !== "listed") {
        if (mounted.current) setStatusLine(`Autosave not removed after the save: ${listed?.outcome === "refused" ? diagnosticsText(listed.diagnostics) : "AUTOSAVE-LIST-RESPONSE: the listing response is not a listing"}`);
        return;
      }
      for (const entry of listed.entries) {
        if (entry.slot !== slot || entry.model_hash !== savedHash) continue;
        const discarded = await discardProjectAutosave(entry.file_name);
        if (discarded.outcome === "refused" && mounted.current) setStatusLine(`Autosave not removed after the save: ${diagnosticsText(discarded.diagnostics)}`);
      }
    } catch (error) {
      if (mounted.current) setStatusLine(`Autosave not removed after the save: ${nativeFailureText(error)}`);
    }
  }

  /** A stored project opened: offer its records (§3). */
  function afterStoredProjectOpened(projectId: string, storedHash: string | null): Promise<void> {
    return scheduleRecoveryCheck({ projectId, storedHash });
  }

  /** Recover Autosave…: offers every record not proven saved, one at a time. */
  function recoverAutosave(): Promise<void> {
    return scheduleRecoveryCheck(null, "all");
  }

  function scheduleRecoveryCheck(scope: Scope, mode: CheckMode = "automatic"): Promise<void> {
    if (!depsRef.current.native) return Promise.resolve();
    checks.current = checks.current.then(() => recoveryCheck(scope, mode)).catch((error) => {
      if (mounted.current) setStatusLine(`Autosave recovery unavailable: ${nativeFailureText(error)}`);
    });
    return checks.current;
  }

  async function recoveryCheck(scope: Scope, mode: CheckMode): Promise<void> {
    const listed = await listProjectAutosaves();
    if (!mounted.current) return;
    if (listed?.outcome !== "listed") {
      setStatusLine(`Autosave recovery unavailable: ${listed?.outcome === "refused" ? diagnosticsText(listed.diagnostics) : "AUTOSAVE-LIST-RESPONSE: the listing response is not a listing"}`);
      return;
    }
    let entries = listed.entries;
    if (scope) {
      const slot = await autosaveSlot(scope.projectId);
      entries = entries.filter((entry) => entry.slot === slot);
    }
    if (entries.length === 0) {
      if (mode === "all") setStatusLine("No autosave to recover.");
      return;
    }
    const storedBySlot = new Map<string, StoredProjectInfo>();
    for (const project of await listLocalProjects()) {
      storedBySlot.set(await autosaveSlot(project.project_id), {
        project_id: project.project_id,
        project_name: project.project_name,
        updated_at_unix: project.updated_at_unix,
        model_hash: scope && project.project_id === scope.projectId ? scope.storedHash : null
      });
    }
    const { offers, olderThanStored } = classifyRecords(entries, storedBySlot);
    // Recover Autosave… offers every record not proven saved, newest first.
    const offered = mode === "all"
      ? [...offers, ...olderThanStored].sort((a, b) => b.entry.written_unix_millis - a.entry.written_unix_millis)
      : offers;
    if (mode === "all" && offered.length === 0) {
      setStatusLine("No autosave to recover: each autosave found holds its stored project's model.");
      return;
    }
    if (mode === "automatic" && olderThanStored.length > 0 && mounted.current) setStatusLine(keptOlderText(olderThanStored));
    for (const candidate of offered) {
      if (!mounted.current) return;
      // After Open, the working model is the record: nothing else is offered now.
      if ((await offerRecord(candidate)) === "open") return;
    }
  }

  /** Waits until no project operation or prompt owns the session. */
  async function untilIdle(): Promise<void> {
    while (mounted.current && depsRef.current.projectSessionBusy()) {
      await new Promise((resolve) => setTimeout(resolve, IDLE_POLL_MS));
    }
  }

  async function offerRecord(candidate: RecoveryCandidate): Promise<"open" | "keep" | "discard"> {
    const { entry, stored } = candidate;
    const writtenAt = formatAutosaveTime(entry.written_unix_millis);
    let record: AutosaveRecoveryPromptState["record"] = null;
    let unreadable: string | null = null;
    try {
      const read = await readProjectAutosave(entry.file_name);
      if (read?.outcome === "read") {
        // The open path's webview checks: the shape check and the validator.
        const shape = modelDocumentShapeDiagnostics(read.document);
        const blocking = shape.length > 0 ? shape : modelDocumentSlotValidation(read.document).blocking;
        if (blocking.length > 0) {
          unreadable = diagnosticsText(blocking);
        } else {
          const project = (read.document as PreviewModel).project;
          record = { projectName: project.name, projectId: project.id, writtenAt, byteCount: read.byte_count, shortHash: shortHash(read.model_hash) };
        }
      } else {
        unreadable = read?.outcome === "refused" ? diagnosticsText(read.diagnostics) : "AUTOSAVE-READ-RESPONSE: the read response is not a record";
      }
    } catch (error) {
      unreadable = nativeFailureText(error);
    }
    await untilIdle();
    if (!mounted.current) return "keep";
    const working = depsRef.current.workingModel();
    const replaces = working
      ? `Open autosave replaces the working model now open, \`${working.name}\`${working.edited ? ", which has unsaved edits (you are asked about them first)" : ", which has no unsaved edits"}.`
      : "Open autosave opens it as the working model.";
    const storedText = stored
      ? `The stored project \`${stored.project_name}\`, last saved ${formatAutosaveTime(stored.updated_at_unix * 1000)}, stays unchanged until you save and confirm the replace prompt.`
      : "No stored project is changed: Save Local Project saves the recovered model as a local project.";
    const choice = await new Promise<"open" | "keep" | "discard">((resolve) => {
      const release = depsRef.current.holdModal();
      // Captured before the workspace turns inert, which drops its focus.
      const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      const show = (confirmingDiscard: boolean) => setRecoveryPrompt({
        fileName: entry.file_name,
        record,
        unreadable,
        newer: whichIsNewerText(candidate),
        replaces,
        stored: storedText,
        notIncluded: AUTOSAVE_NOT_INCLUDED,
        confirmingDiscard,
        opener,
        choose: (picked) => {
          if (picked === "discard") return show(true);
          if (picked === "discard-cancelled") return show(false);
          // Open is offered only for a record that verified.
          if (picked === "open" && !record) return;
          release();
          setRecoveryPrompt(null);
          resolve(picked === "discard-confirmed" ? "discard" : picked);
        }
      });
      show(false);
    });
    if (choice === "keep") {
      setStatusLine(`Autosave kept for later (written ${writtenAt}); it is offered again at the next start or open of its project.`);
    } else if (choice === "discard") {
      try {
        const discarded = await discardProjectAutosave(entry.file_name);
        if (!mounted.current) return choice;
        setStatusLine(discarded.outcome === "refused"
          ? `Autosave not discarded: ${diagnosticsText(discarded.diagnostics)}`
          : `Autosave written ${writtenAt} discarded.`);
      } catch (error) {
        if (mounted.current) setStatusLine(`Autosave not discarded: ${nativeFailureText(error)}`);
      }
    } else {
      await depsRef.current.openAutosave(entry.file_name, writtenAt);
    }
    return choice;
  }

  return {
    /** Null until the native settings are read (and always in the browser preview). */
    settings,
    /** Item (a): shown for as long as autosave is off, apart from the status line. */
    offNotice: settings?.autosave_minutes === 0 ? AUTOSAVE_OFF_NOTICE : null,
    settingBusy,
    statusLine,
    recoveryPrompt,
    chooseInterval,
    afterProjectSaved,
    afterStoredProjectOpened,
    recoverAutosave
  };
}

export type AutosaveSession = ReturnType<typeof useAutosave>;
