// The recovery offer's rules (DEL-02-05 Design `autosave-and-backup.md` §3),
// pure so the tests can state them. A record is paired with the stored project
// whose id has the record's slot. It is offered when there is no such project
// (a model never saved), or when it was written after that project's last save
// and its model hash differs from the stored model's (when that hash is
// known). Before anything is offered, the record and the stored document are
// compared by their written and last-saved times, and the text says which is
// newer (Agent 0, 2026-10-28, item (b)): neither is ever silently preferred.
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
      return `The autosave is newer: it was written ${written}, after the stored project \`${stored.project_name}\` was last saved ${saved}.`;
    case "same-second":
      return `The autosave was written ${written}, within the same second as the stored project's last save (${saved}), so the times cannot tell which is newer.`;
    case "stored-newer":
      return `The stored project \`${stored.project_name}\` is newer: it was last saved ${saved}, after its autosave was written ${written}. The autosave is not offered and is kept.`;
    default:
      return `The autosave (written ${written}) holds the same model as the stored project \`${stored.project_name}\`.`;
  }
}

/** What a record holds, and what it does not (Design §1, §3). */
export const AUTOSAVE_NOT_INCLUDED = "Queued batches, proposals, review context and solve results are not in an autosave; a recovered model needs to be solved again.";

/** The first 12 hex digits of a `sha256:` hash. */
export function shortHash(hash: string): string {
  return hash.replace(/^sha256:/, "").slice(0, 12);
}
