// File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.9.5, slice 4b.2a): the native
// `cancel_mbf_export`, which cancels the export's review and save in progress. The workspace session calls it when it
// retires a project session (a new session generation, including a project replaced by one with the same id) and when
// it unmounts, whether or not a model check is in progress; slice 4b.2b's dialog calls it when it closes. A waiting
// run stops waiting, a running worker is killed and its grant released, and a save before its commit writes nothing;
// a save past its commit completes and is reported too late.
import { invoke } from "@tauri-apps/api/core";

/** What the cancel did: the runs it stopped (nothing written) and the saves past their commit (not interrupted). */
export type MbfCancelReceipt = { cancelled: number; too_late: number };

/** Cancels the MBF export's runs in progress. Best effort; never throws: `null` when the native host did not answer
 * (the browser preview has none, and the page-load reset cancels natively too). */
export async function cancelMbfExport(): Promise<MbfCancelReceipt | null> {
  try {
    return await invoke<MbfCancelReceipt>("cancel_mbf_export");
  } catch {
    return null;
  }
}
