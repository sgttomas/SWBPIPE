import { useState } from "react";
import type { AutosaveSession } from "./useAutosave";

const BACKUP_DESKTOP_ONLY = "Backups need the desktop app.";
const BACKUP_SETTING_BUSY = "Waiting for the current setting change to finish.";
const BACKUP_INTENT_REQUIRED = "Check the private-copy statement above to choose a folder.";
const BACKUP_ALREADY_OFF = "Backups are already off.";

export function BackupSettingsControl({ autosave }: {
  autosave: Pick<AutosaveSession, "settings" | "settingBusy" | "backupSettingsUncertain" | "configureBackup">;
}) {
  const { settings, settingBusy, backupSettingsUncertain } = autosave;
  // An unknown configuration outcome may have enabled backups: Disable stays available.
  const mayBeOn = Boolean(settings?.backup_folder) || backupSettingsUncertain;
  const [intent, setIntent] = useState(false);
  const disabled = !settings || settingBusy;
  const finite = settings?.backup_keep != null;
  // A disabled control states why (dead-control audit TP-APP-R2-UXSHELL-001).
  const blocked = !settings ? BACKUP_DESKTOP_ONLY : settingBusy ? BACKUP_SETTING_BUSY : null;
  const enableReason = blocked ?? (intent ? undefined : BACKUP_INTENT_REQUIRED);
  const disableReason = blocked ?? (mayBeOn ? undefined : BACKUP_ALREADY_OFF);
  return (
    <fieldset disabled={disabled} aria-describedby="backup-setting-detail">
      <legend>Backups after Save Local Project</legend>
      <p data-testid="backup-folder">{settings?.backup_folder ? `Backup folder: ${settings.backup_folder}` : "Backups are off."}</p>
      <p>Retention: All — delete nothing. Automatic rotation is unavailable.</p>
      {finite ? <p role="status" data-testid="backup-retention-unavailable">The saved limit of {settings?.backup_keep} is unavailable and remains unchanged. Choose a folder again to explicitly enable All. No backup uses that finite setting.</p> : null}
      <p id="backup-setting-detail">Backups copy the last saved model. They do not include queued batches, proposals, review context or solve results. All copies are kept, including temporary, partial and unverified files, so storage grows. Other software may sync the chosen folder; choose where private models belong. The selected volume must confirm support for this app's atomic clone route before enabling; unsupported or unknown support is refused. Each backup still checks the actual write and durability.</p>
      <label>
        <input type="checkbox" checked={intent} onChange={(event) => setIntent(event.target.checked)} />
        I intend to copy private project models to the local folder I choose and keep all copies.
      </label>
      <button type="button" data-testid="backup-enable" disabled={disabled || !intent} title={enableReason} onClick={() => { void autosave.configureBackup(true, null, intent); }}>
        Choose folder and enable All backups…
      </button>
      <button type="button" data-testid="backup-disable" disabled={disabled || !mayBeOn} title={disableReason} onClick={() => { void autosave.configureBackup(false, null, false); }}>
        Turn backups off
      </button>
      <p>Turning backups off keeps every copy. A backup failure leaves your saved project unchanged.</p>
    </fieldset>
  );
}
