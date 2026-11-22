// The Project page's body: what a person came for first (the project and whether it is
// saved, an autosave to recover when there is one, the saved projects, the model's
// history), then the storage record behind one closed disclosure. The page's New, Open…
// and Save buttons are in its header (App.tsx); the File menu keeps the complete list.
import { useEffect, useRef, useState, type ReactNode } from "react";
import { FolderOpen } from "lucide-react";
import { listLocalProjects } from "../../services/projectService";
import { nativeFailureText } from "../../services/nativeIngress";
import type { LocalProjectIndexEntry, LocalProjectSummary } from "../../types";
import { autosaveSlot, listProjectAutosaves, type AutosaveEntry } from "../autosave/autosaveService";
import { formatAutosaveTime } from "../autosave/autosaveRecovery";
import { HistoryPanel } from "../history/HistoryPanel";
import { useSessionChrome, useSessionModel, useSessionProject } from "../workspace/WorkspaceSessionContext";

/** The project's one line: whether it is bound to a stored project, when that stored revision
 * was saved, and whether the working model changed since. The binding is the session's stored
 * summary (set only by a create, open or save that committed to that project); the time is the
 * stored revision's own, as the saved-projects list reads it from the store, so the line and the
 * project's row agree and a refused or failed save never moves it. */
export function projectSavedLine({ projectId, projectSummary, modelEdited, savedProjects }: {
  projectId: string;
  projectSummary: LocalProjectSummary | null;
  modelEdited: boolean;
  savedProjects: readonly LocalProjectIndexEntry[] | null;
}): string {
  if (projectSummary?.project_id !== projectId) return "Not saved as a project.";
  const listed = savedProjects?.find((entry) => entry.project_id === projectId);
  const saved = listed ? `Saved ${formatAutosaveTime(listed.updated_at_unix * 1000)}.` : "Saved.";
  return `${saved} ${modelEdited ? "Changed since." : "No changes since."}`;
}

/** The stated reason of a control disabled while a project operation owns the session. */
const BUSY_REASON = "Another project operation or prompt is in progress.";

type Listing<T> = { items: T[] | null; failure: string | null };

/** The saved projects, listed when the page opens and again after each project operation
 * lands, while the page is open. In the desktop app this is one read of the local store's project index; in the
 * browser preview, of the session's own memory. */
function useSavedProjects(active: boolean, projectOperation: string, projectBusy: boolean, generation: number): Listing<LocalProjectIndexEntry> {
  const [listing, setListing] = useState<Listing<LocalProjectIndexEntry>>({ items: null, failure: null });
  const request = useRef(0);
  useEffect(() => {
    const current = ++request.current;
    if (!active || projectBusy) return;
    listLocalProjects().then(
      (items: unknown) => {
        if (current !== request.current) return;
        setListing(Array.isArray(items)
          ? { items: items.filter((entry): entry is LocalProjectIndexEntry => typeof entry?.project_id === "string"), failure: null }
          : { items: null, failure: "the reply is not a list of projects" });
      },
      (error) => { if (current === request.current) setListing({ items: null, failure: nativeFailureText(error) }); }
    );
  }, [active, projectOperation, projectBusy, generation]);
  return listing;
}

type Recoverable = { newest: AutosaveEntry; name: string | null; count: number };

/** The autosaves Recover would offer (every record found; DEL-02-05 Design
 * `autosave-and-backup.md` §3), read from the autosave folder's listing when the page opens
 * and after each project operation or autosave status change. Desktop only. */
function useRecoverableAutosave(active: boolean, native: boolean, refreshKey: string, savedProjects: readonly LocalProjectIndexEntry[] | null): Recoverable | null {
  const [recoverable, setRecoverable] = useState<Recoverable | null>(null);
  const request = useRef(0);
  useEffect(() => {
    const current = ++request.current;
    if (!native || !active) { if (!native) setRecoverable(null); return; }
    void (async () => {
      try {
        const listed = await listProjectAutosaves();
        if (current !== request.current) return;
        if (listed?.outcome !== "listed" || !Array.isArray(listed.entries) || listed.entries.length === 0) { setRecoverable(null); return; }
        const newest = [...listed.entries].sort((a, b) => b.written_unix_millis - a.written_unix_millis)[0];
        let name: string | null = null;
        for (const project of savedProjects ?? []) {
          if (await autosaveSlot(project.project_id) === newest.slot) { name = project.project_name; break; }
        }
        if (current === request.current) setRecoverable({ newest, name, count: listed.entries.length });
      } catch {
        if (current === request.current) setRecoverable(null);
      }
    })();
  }, [active, native, refreshKey, savedProjects]);
  return recoverable;
}

export function ProjectPage({ storageRecord }: { storageRecord: ReactNode }) {
  const { model } = useSessionModel();
  const {
    projectSummary, modelEdited, projectOperation, projectBusy, handleOpenProject, autosave
  } = useSessionProject();
  const { projectSessionGeneration: generation } = useSessionModel();
  // The page stays mounted, dormant, once opened; it reads the store only while it is the open page.
  const active = useSessionChrome().activeSection === "project";
  const saved = useSavedProjects(active, projectOperation, projectBusy, generation);
  // The autosave folder's own revision moves when a record is written or removed (the cleanup
  // after a save finishes after the save does), so Recover leaves when its last record does.
  const recoverable = useRecoverableAutosave(active, Boolean(autosave.settings), `${projectOperation}:${projectBusy}:${autosave.statusLine ?? ""}:${autosave.recordsRevision ?? 0}`, saved.items);
  if (!model) return null;
  const projectId = model.project.id;
  return (
    <div className="project-page" data-testid="project-page">
      <section className="project-page-head" aria-label="This project">
        <h3 data-testid="project-page-name">{model.project.name}</h3>
        <p data-testid="project-page-saved-line">
          {projectSavedLine({ projectId, projectSummary, modelEdited, savedProjects: saved.items })}
        </p>
        {recoverable ? (
          <p className="project-page-recover" data-testid="project-page-recover-line">
            <span>
              Autosave{recoverable.name ? ` of ${recoverable.name}` : ""} written {formatAutosaveTime(recoverable.newest.written_unix_millis)}
              {recoverable.count > 1 ? `, and ${recoverable.count - 1} more` : ""}.
            </span>
            <button type="button" data-testid="project-recover-autosave" disabled={projectBusy} title={projectBusy ? BUSY_REASON : undefined} onClick={() => void autosave.recoverAutosave()}>
              Recover…
            </button>
          </p>
        ) : null}
      </section>
      <section className="project-page-saved" aria-label="Saved projects" data-testid="project-saved-list">
        <h4>Saved projects</h4>
        {saved.failure ? (
          <p className="muted" role="note" data-testid="project-saved-list-failure">Saved projects not listed: {saved.failure}</p>
        ) : saved.items === null ? null : saved.items.length === 0 ? (
          <p className="muted" data-testid="project-saved-list-empty">None yet.</p>
        ) : (
          <ul>
            {saved.items.map((entry) => (
              <li key={entry.project_id}>
                <button
                  type="button"
                  data-testid={`project-saved-open-${entry.project_id}`}
                  disabled={projectBusy}
                  title={projectBusy ? BUSY_REASON : undefined}
                  onClick={() => void handleOpenProject(entry.project_id)}
                >
                  <FolderOpen size={14} aria-hidden="true" />
                  <span className="project-saved-name">{entry.project_name}</span>
                  <span className="muted">
                    {" "}saved {formatAutosaveTime(entry.updated_at_unix * 1000)}{entry.project_id === projectSummary?.project_id ? " · open now" : ""}
                  </span>
                </button>
              </li>
            ))}
          </ul>
        )}
      </section>
      <HistoryPanel />
      <details className="project-storage-record" data-testid="project-storage-record">
        <summary>Storage record</summary>
        <div className="project-storage-record-body">{storageRecord}</div>
      </details>
    </div>
  );
}
