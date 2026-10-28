// The recovery offer's rules (DEL-02-05 Design `autosave-and-backup.md` §3),
// pure so the tests can state them. A record is paired with the stored project
// whose id has the record's slot. Every record whose model is not proven to be
// the stored model stays recoverable. At start and at an open it is offered
// (modal) when there is no such project (a model never saved), or when, by
// this Mac's clock, it was written after that project's last save; one the
// clock calls older is stated in a status line and offered by Recover
// Autosave…, since a clock that was set back can misorder them (Codex r1 on
// #1718). The texts compare the written and last-saved times "by this Mac's
// clock" and say which is newer (Agent 0, 2026-10-28, item (b)): neither is
// ever silently preferred.
import type { AutosaveEntry } from "./autosaveService";

/** A stored project, as the project index lists it. */
export type StoredProjectInfo = {
  project_id: string;
  project_name: string;
  /** The last save, in Unix seconds (the store's resolution). */
  updated_at_unix: number;
  /** The stored model's hash (`sha256:<hex>`), when the session knows it. */
  model_hash: string | null;
};

/** How a record stands against its stored project.
 * - `no-stored`: no stored project has its slot;
 * - `autosave-newer`: written in a later second than the last save;
 * - `same-second`: written within the second of the last save, so the times
 *   cannot tell which is newer (offered, and said so);
 * - `stored-newer`: written before the last save (not offered; said so);
 * - `same-model`: the stored model is this record's model (nothing to offer). */
export type RecordStanding = "no-stored" | "autosave-newer" | "same-second" | "stored-newer" | "same-model";

export type RecoveryCandidate = {
  entry: AutosaveEntry;
  stored: StoredProjectInfo | null;
  standing: RecordStanding;
};

export function recordStanding(entry: AutosaveEntry, stored: StoredProjectInfo | null): RecordStanding {
  if (!stored) return "no-stored";
  if (stored.model_hash !== null && stored.model_hash === entry.model_hash) return "same-model";
  const savedFrom = stored.updated_at_unix * 1000;
  if (entry.written_unix_millis < savedFrom) return "stored-newer";
  if (entry.written_unix_millis < savedFrom + 1000) return "same-second";
  return "autosave-newer";
}

/** The records to offer (newest first, as listed) and the records the stored
 * project is newer than (said in a status line, never offered). */
export function classifyRecords(
  entries: readonly AutosaveEntry[],
  storedBySlot: ReadonlyMap<string, StoredProjectInfo>
): { offers: RecoveryCandidate[]; olderThanStored: RecoveryCandidate[] } {
  const offers: RecoveryCandidate[] = [];
  const olderThanStored: RecoveryCandidate[] = [];
  for (const entry of entries) {
    const stored = storedBySlot.get(entry.slot) ?? null;
    const standing = recordStanding(entry, stored);
    if (standing === "stored-newer") olderThanStored.push({ entry, stored, standing });
    else if (standing !== "same-model") offers.push({ entry, stored, standing });
  }
  return { offers, olderThanStored };
}

function pad(value: number): string {
  return String(value).padStart(2, "0");
}

/** A local date and time, `YYYY-MM-DD HH:MM:SS`. */
export function formatAutosaveTime(unixMillis: number): string {
  const at = new Date(unixMillis);
  return `${at.getFullYear()}-${pad(at.getMonth() + 1)}-${pad(at.getDate())} ${pad(at.getHours())}:${pad(at.getMinutes())}:${pad(at.getSeconds())}`;
}

/** Item (b): which is newer, by the written and last-saved times. */
export function whichIsNewerText(candidate: RecoveryCandidate): string {
  const written = formatAutosaveTime(candidate.entry.written_unix_millis);
  const stored = candidate.stored;
  if (!stored) return `No stored project has this model's project id, so there is no saved version to compare: the autosave (written ${written}) is the only copy found.`;
  const saved = formatAutosaveTime(stored.updated_at_unix * 1000);
  switch (candidate.standing) {
    case "autosave-newer":
      return `By this Mac's clock, the autosave is newer: it was written ${written}, and the stored project \`${stored.project_name}\` was last saved ${saved}.`;
    case "same-second":
      return `By this Mac's clock, the autosave was written ${written}, within the same second as the stored project's last save (${saved}), so the times cannot tell which is newer.`;
    case "stored-newer":
      return `By this Mac's clock, the stored project \`${stored.project_name}\` is newer: it was last saved ${saved}, and the autosave was written ${written}. A clock that was changed can make this order wrong.`;
    default:
      return `The autosave (written ${written}) holds the same model as the stored project \`${stored.project_name}\`.`;
  }
}

/** Where Recover Autosave… is, for the status lines. */
export const RECOVER_AUTOSAVE_HINT = "Recover autosave… (in Appearance, or the project panel) offers it";

/** The status line for records the clock calls older than their stored
 * project: they are kept, stated, and reachable through Recover Autosave…. */
export function keptOlderText(candidates: readonly RecoveryCandidate[]): string {
  const first = candidates[0];
  const differs = first.stored?.model_hash ? "Its model differs from the saved project's." : "It is not known to hold the saved project's model.";
  const more = candidates.length > 1 ? ` ${candidates.length - 1} more such autosaves are kept too.` : "";
  return `An autosave is kept. ${whichIsNewerText(first)} ${differs} ${RECOVER_AUTOSAVE_HINT}.${more}`;
}

/** What a record holds, and what it does not (Design §1, §3). */
export const AUTOSAVE_NOT_INCLUDED = "Queued batches, proposals, review context and solve results are not in an autosave; a recovered model needs to be solved again.";

/** The first 12 hex digits of a `sha256:` hash. */
export function shortHash(hash: string): string {
  return hash.replace(/^sha256:/, "").slice(0, 12);
}
