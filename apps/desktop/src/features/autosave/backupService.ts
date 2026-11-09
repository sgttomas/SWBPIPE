import { invokeWithIngress } from "../../services/nativeIngress";
import type { AutosaveDiagnostic, AutosaveSettings } from "./autosaveService";

export type BackupSettingsOutcome =
  | { outcome: "saved"; settings: AutosaveSettings }
  | { outcome: "cancelled" }
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

export type BackupWriteOutcome =
  | { outcome: "written"; file_name: string; model_hash: string; durability: string; warnings: AutosaveDiagnostic[] }
  | { outcome: "unchanged"; file_name: string; model_hash: string; warnings: AutosaveDiagnostic[] }
  | { outcome: "unverified"; file_name: string; model_hash: string; creation: "created" | "existing" | "unknown"; diagnostics: AutosaveDiagnostic[] }
  | { outcome: "disabled" }
  | { outcome: "refused"; diagnostics: AutosaveDiagnostic[] };

/** Only the native chooser supplies the folder. Intent covers copying private
 * project models to that local folder; no model or path is sent by the webview. */
export function configureProjectBackup(enabled: boolean, backupKeep: number | null, explicitLocalPrivateIntent: boolean): Promise<BackupSettingsOutcome> {
  return invokeWithIngress<BackupSettingsOutcome>("configure_project_backup", { enabled, backupKeep, explicitLocalPrivateIntent });
}

/** Called after a verified Save Local Project response. The native worker reads
 * the stored canonical model and refuses if its full hash no longer matches. */
export function writeProjectBackup(projectId: string, expectedModelHash: string): Promise<BackupWriteOutcome> {
  return invokeWithIngress<BackupWriteOutcome>("write_project_backup", { projectId, expectedModelHash });
}
