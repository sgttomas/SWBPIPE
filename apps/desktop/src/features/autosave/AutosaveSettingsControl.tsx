import { AUTOSAVE_MINUTE_CHOICES } from "./autosaveService";
import { AUTOSAVE_DESKTOP_ONLY, AUTOSAVE_OFF_NOTICE, type AutosaveSession } from "./useAutosave";

/** The autosave interval, beside the theme and density in the toolbar's
 * Appearance popover (the existing preferences UI). Off, or every 1, 2, 5, 10,
 * 15 or 30 minutes; on by default, every 5 minutes. While it is off, a
 * one-line notice says that a crash can lose unsaved work (Agent 0,
 * 2026-10-28, item (a)). Recover autosave… offers every autosave not proven
 * to be its stored project's model (Design §3). In the browser preview there are no native settings,
 * so the control is disabled and says so. */
export function AutosaveSettingsControl({ autosave }: { autosave: Pick<AutosaveSession, "settings" | "settingBusy" | "chooseInterval" | "recoverAutosave"> }) {
  const { settings } = autosave;
  const off = settings?.autosave_minutes === 0;
  return (
    <>
      <label>
        <span>Autosave</span>
        <select
          aria-label="Autosave interval"
          aria-describedby="autosave-setting-detail"
          data-testid="autosave-interval"
          value={settings ? String(settings.autosave_minutes) : ""}
          disabled={!settings || autosave.settingBusy}
          onChange={(event) => { void autosave.chooseInterval(Number(event.target.value)); }}
        >
          {settings ? null : <option value="">Desktop app only</option>}
          <option value="0">Off</option>
          {AUTOSAVE_MINUTE_CHOICES.map((minutes) => (
            <option key={minutes} value={String(minutes)}>Every {minutes} {minutes === 1 ? "minute" : "minutes"}</option>
          ))}
        </select>
      </label>
      {off ? <p className="shell-autosave-off" role="status" data-testid="autosave-off-notice">{AUTOSAVE_OFF_NOTICE}</p> : null}
      <button type="button" data-testid="recover-autosave" disabled={!settings} title={settings ? "Offer each autosave not shown to be saved" : AUTOSAVE_DESKTOP_ONLY} onClick={() => { void autosave.recoverAutosave(); }}>
        Recover autosave…
      </button>
      <p className="shell-autosave-detail" id="autosave-setting-detail">
        Autosave writes only while there are unsaved edits. Autosaves stay on this computer. Saving the project removes only the autosaves that match the saved model.
      </p>
    </>
  );
}
