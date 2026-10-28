// The autosave commands (D5 slice 2; DEL-02-05 Design `autosave-and-backup.md`
// §2, §3 and §6): the native host's `autosave_commands.rs`, each sent as a raw
// body through `invokeWithIngress`. Nothing here decides anything: the hook
// (`useAutosave.ts`) and the pure rules (`autosaveRecovery.ts`) do.
import { invokeWithIngress } from "../../services/nativeIngress";

export type AutosaveDiagnostic = { code: string; path: string; message: string };

export type AutosaveEntry = {
  file_name: string;
  slot: string;
  written_unix_millis: number;
  /** `sha256:<hex>`. */
  model_hash: string;
  byte_count: number;
};

export type AutosaveWriteOutcome =
  | {
      outcome: "written";
      file_name: string;
      slot: string;
      written_unix_millis: number;
      model_hash: string;
      byte_count: number;
      durability: string;
      warnings: AutosaveDiagnostic[];
    }
  | { outcome: "unchanged"; file_name: string; slot: string; model_hash: string }
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

export type AutosaveListOutcome =
  | { outcome: "listed"; entries: AutosaveEntry[]; not_listed: number }
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

export type AutosaveReadOutcome =
  | (AutosaveEntry & { outcome: "read"; document: unknown })
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

export type AutosaveDiscardOutcome =
  | { outcome: "discarded"; file_name: string }
  | { outcome: "not_found"; file_name: string }
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

export type AutosaveSettings = {
  /** 0 is off. */
  autosave_minutes: number;
  /** Slice 3 (backups); null until then. */
  backup_folder: string | null;
  backup_keep: number | null;
};

export type AutosaveSettingsRead = {
  settings: AutosaveSettings;
  source: "file" | "default";
  /** Why a settings file present was not used. */
  status: AutosaveDiagnostic | null;
};

export type AutosaveSettingsWriteOutcome =
  | { outcome: "saved"; settings: AutosaveSettings }
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

/** On by default, every 5 minutes (Agent 0, 2026-10-28, row RT-U3). */
export const DEFAULT_AUTOSAVE_MINUTES = 5;
/** The interval choices; off is 0. */
export const AUTOSAVE_MINUTE_CHOICES = [1, 2, 5, 10, 15, 30] as const;
export const DEFAULT_AUTOSAVE_SETTINGS: AutosaveSettings = { autosave_minutes: DEFAULT_AUTOSAVE_MINUTES, backup_folder: null, backup_keep: null };

export function writeProjectAutosave(canonicalText: string, claimedModelHash: string): Promise<AutosaveWriteOutcome> {
  return invokeWithIngress<AutosaveWriteOutcome>("write_project_autosave", { canonicalText, claimedModelHash });
}

export function listProjectAutosaves(): Promise<AutosaveListOutcome> {
  return invokeWithIngress<AutosaveListOutcome>("list_project_autosaves", {});
}

export function readProjectAutosave(fileName: string): Promise<AutosaveReadOutcome> {
  return invokeWithIngress<AutosaveReadOutcome>("read_project_autosave", { fileName });
}

export function discardProjectAutosave(fileName: string): Promise<AutosaveDiscardOutcome> {
  return invokeWithIngress<AutosaveDiscardOutcome>("discard_project_autosave", { fileName });
}

export function getAutosaveSettings(): Promise<AutosaveSettingsRead> {
  return invokeWithIngress<AutosaveSettingsRead>("get_autosave_settings", {});
}

export function setAutosaveSettings(autosaveMinutes: number): Promise<AutosaveSettingsWriteOutcome> {
  return invokeWithIngress<AutosaveSettingsWriteOutcome>("set_autosave_settings", { autosaveMinutes });
}

/** A record's slot for a project id: the first 32 hex digits of the SHA-256
 * of its UTF-8 bytes (`autosave.rs` `autosave_slot`). */
export async function autosaveSlot(projectId: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(projectId));
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("").slice(0, 32);
}

/** `code: message` of each diagnostic, joined. */
export function diagnosticsText(diagnostics: AutosaveDiagnostic[]): string {
  if (diagnostics.length === 0) return "AUTOSAVE-REFUSED: no reason was given";
  return diagnostics.map((item) => `${item.code}: ${item.message}`).join(" | ");
}
