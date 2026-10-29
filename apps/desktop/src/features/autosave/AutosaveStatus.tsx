import type { AutosaveSession } from "./useAutosave";

/** The autosave lines in the toolbar: the notice of item (a), shown for as
 * long as autosave is off (from the settings, so no later status replaces
 * it; Codex r1 P2 on #1718), and the latest status line. */
export function AutosaveStatus({ autosave }: { autosave: Pick<AutosaveSession, "offNotice" | "statusLine"> }) {
  return (
    <>
      {autosave.offNotice ? (
        <p className="shell-autosave-status shell-autosave-off-status" data-testid="autosave-off-status" role="status" title={autosave.offNotice}>{autosave.offNotice}</p>
      ) : null}
      {autosave.statusLine ? (
        <p className="shell-autosave-status" data-testid="autosave-status" role="status" title={autosave.statusLine}>{autosave.statusLine}</p>
      ) : null}
    </>
  );
}
