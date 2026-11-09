import { useState } from "react";
import type { AutosaveSession } from "./useAutosave";

export function BackupSettingsControl({ autosave }: {
  autosave: Pick<AutosaveSession, "settings" | "settingBusy" | "configureBackup">;
}) {
  const { settings, settingBusy } = autosave;
  const [intent, setIntent] = useState(false);
  const disabled = !settings || settingBusy;
  const finite = settings?.backup_keep != null;
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
      <button type="button" data-testid="backup-enable" disabled={disabled || !intent} onClick={() => { void autosave.configureBackup(true, null, intent); }}>
        Choose folder and enable All backups…
      </button>
      <button type="button" data-testid="backup-disable" disabled={disabled || !settings?.backup_folder} onClick={() => { void autosave.configureBackup(false, null, false); }}>
        Turn backups off
      </button>
      <p>Turning backups off keeps every copy. A backup failure leaves your saved project unchanged.</p>
    </fieldset>
  );
}
