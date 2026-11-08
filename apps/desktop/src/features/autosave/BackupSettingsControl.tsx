import { useEffect, useState } from "react";
import { BACKUP_KEEP_CHOICES } from "./backupService";
import type { AutosaveSession } from "./useAutosave";

export function BackupSettingsControl({ autosave }: {
  autosave: Pick<AutosaveSession, "settings" | "settingBusy" | "configureBackup">;
}) {
  const { settings, settingBusy } = autosave;
  const [intent, setIntent] = useState(false);
  const [keep, setKeep] = useState("all");
  useEffect(() => { setKeep(settings?.backup_keep == null ? "all" : String(settings.backup_keep)); }, [settings?.backup_keep]);
  const disabled = !settings || settingBusy;
  return (
    <fieldset disabled={disabled} aria-describedby="backup-setting-detail">
      <legend>Backups after Save Local Project</legend>
      <p data-testid="backup-folder">{settings?.backup_folder ? `Backup folder: ${settings.backup_folder}` : "Backups are off."}</p>
      <label>
        <span>Keep backups of each project</span>
        <select aria-label="Backups to keep" value={keep} onChange={(event) => setKeep(event.target.value)}>
          <option value="all">All — delete nothing</option>
          {BACKUP_KEEP_CHOICES.map((count) => <option key={count} value={String(count)}>Newest {count}</option>)}
        </select>
      </label>
      <p id="backup-setting-detail">Backups copy the last saved model to a local folder you choose. They do not include queued batches, proposals, review context or solve results. Other software may sync the chosen folder; choose where private models belong. Choosing a limit removes older verified backups of that project only after a new copy verifies and is durable. Changing the limit deletes nothing now.</p>
      <label>
        <input type="checkbox" checked={intent} onChange={(event) => setIntent(event.target.checked)} />
        I intend to copy private project models to the local folder I choose.
      </label>
      <button type="button" data-testid="backup-enable" disabled={disabled || !intent} onClick={() => { void autosave.configureBackup(true, keep === "all" ? null : Number(keep), intent); }}>
        {settings?.backup_folder ? "Choose folder and apply backup settings…" : "Choose folder and enable backups…"}
      </button>
      <button type="button" data-testid="backup-disable" disabled={disabled || !settings?.backup_folder} onClick={() => { void autosave.configureBackup(false, settings?.backup_keep ?? null, false); }}>
        Turn backups off
      </button>
      <p>Turning backups off keeps existing copies. A backup failure leaves your saved project unchanged.</p>
    </fieldset>
  );
}
