// The Project page's body against a fake of the session: the project's one line, the
// recover line only when an autosave is found, the saved projects, Model History's disabled
// actions with their reasons as titles, and the storage record behind a closed disclosure.
// The whole window in a saved project is `App.projectPage.test.tsx`. All data are invented.
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { LocalProjectIndexEntry, LocalProjectSummary } from "../../types";
import { RUN_RECORD_NO_RUN } from "../../services/analysisRunService";
import { WorkspaceSessionProvider } from "../workspace/WorkspaceSessionContext";
import type { WorkspaceSession } from "../workspace/workspaceSession";

const listed = vi.hoisted(() => ({ projects: [] as unknown[], autosaves: null as unknown }));
vi.mock("../../services/projectService", async (importOriginal) => ({
  ...await importOriginal<typeof import("../../services/projectService")>(),
  listLocalProjects: vi.fn(async () => listed.projects)
}));
vi.mock("../autosave/autosaveService", async (importOriginal) => ({
  ...await importOriginal<typeof import("../autosave/autosaveService")>(),
  listProjectAutosaves: vi.fn(async () => listed.autosaves)
}));

import { autosaveSlot } from "../autosave/autosaveService";
import { formatAutosaveTime } from "../autosave/autosaveRecovery";
import { ProjectPage, projectSavedLine } from "./ProjectPage";

const PROJECT = "project:invented-page";
const NAME = "Invented Page Loop";
const SAVED_AT = Date.UTC(2026, 10, 21, 9, 30, 0);
const OTHER: LocalProjectIndexEntry = { project_id: "project:invented-other", project_name: "Invented Other Loop", storage_mode: "local_sqlite", created_at_unix: 1_790_000_000, updated_at_unix: 1_790_000_100 };
const THIS: LocalProjectIndexEntry = { project_id: PROJECT, project_name: NAME, storage_mode: "local_sqlite", created_at_unix: 1_790_000_000, updated_at_unix: SAVED_AT / 1000 };
const SUMMARY = { project_id: PROJECT, project_name: NAME } as LocalProjectSummary;

afterEach(() => {
  listed.projects = [];
  listed.autosaves = null;
});

function historySlice(overrides: Record<string, unknown> = {}) {
  return {
    states: [], runs: [], unreadable: null, runCheck: null, recordRunUnavailableReason: RUN_RECORD_NO_RUN, recordRunState: null,
    historyMessage: "", historyOperation: "", historyBusy: false, basisStateId: null, refusedRuns: {}, currentRunUnavailableReason: "Solve the model to compare the current run.",
    stateComparison: null, runComparison: null, refusedStateExport: null,
    refreshStates: vi.fn(), compareStates: vi.fn(), compareRuns: vi.fn(), closeStateComparison: vi.fn(), closeRunComparison: vi.fn(), exportRefusedState: vi.fn(),
    ...overrides
  };
}

function renderPage({ native = false, summary = SUMMARY as LocalProjectSummary | null, edited = false } = {}) {
  const recoverAutosave = vi.fn(async () => undefined);
  const handleOpenProject = vi.fn(async () => undefined);
  const sessionWith = (recordsRevision: number) => ({
    model: { model: { project: { id: PROJECT, name: NAME } }, projectSessionGeneration: 1 },
    chrome: { activeSection: "project" },
    project: {
      projectSummary: summary, modelEdited: edited, projectOperation: "save", projectBusy: false,
      handleOpenProject, history: historySlice(),
      autosave: { settings: native ? { autosave_minutes: 5 } : null, statusLine: null, recordsRevision, recoverAutosave }
    }
  }) as unknown as WorkspaceSession;
  const page = (recordsRevision: number) => (
    <WorkspaceSessionProvider session={sessionWith(recordsRevision)}>
      <ProjectPage storageRecord={<><section data-testid="project-storage-panel" /><section data-testid="project-validation-panel" /></>} />
    </WorkspaceSessionProvider>
  );
  const view = render(page(0));
  /** The autosave records changed (a write, a discard, a save's cleanup finishing). */
  const recordsChanged = (recordsRevision: number) => view.rerender(page(recordsRevision));
  return { recoverAutosave, handleOpenProject, recordsChanged };
}

describe("the Project page", () => {
  it("leads with the project, its saved projects and its history; the storage record is last and closed", async () => {
    listed.projects = [THIS, OTHER];
    const { handleOpenProject } = renderPage();
    expect(screen.getByTestId("project-page-name")).toHaveTextContent(NAME);
    // The line's time is the store's latest, the same the project's row shows, named apart from this window's edits.
    await waitFor(() => expect(screen.getByTestId("project-page-saved-line")).toHaveTextContent(`Saved project; no edits in this window. Latest in store: ${formatAutosaveTime(SAVED_AT)}.`));

    const saved = screen.getByTestId("project-saved-list");
    const other = await within(saved).findByTestId(`project-saved-open-${OTHER.project_id}`);
    expect(other).toHaveTextContent(`${OTHER.project_name} saved ${formatAutosaveTime(OTHER.updated_at_unix * 1000)}`);
    expect(within(saved).getByTestId(`project-saved-open-${PROJECT}`)).toHaveTextContent("open now");
    fireEvent.click(other);
    expect(handleOpenProject).toHaveBeenCalledWith(OTHER.project_id);
    // No project:… identifier on the page outside the storage record.
    expect(screen.getByTestId("project-page")).not.toHaveTextContent("project:");

    const history = screen.getByTestId("model-history-panel");
    const record = screen.getByTestId("project-storage-record") as HTMLDetailsElement;
    expect(record.tagName).toBe("DETAILS");
    expect(record.open).toBe(false);
    expect(within(record).getByText("Storage record", { selector: "summary" })).toBeInTheDocument();
    expect(saved.compareDocumentPosition(history) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(history.compareDocumentPosition(record) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(record.nextElementSibling).toBeNull();
    fireEvent.click(within(record).getByText("Storage record"));
    expect(record.open).toBe(true);
    expect(within(record).getByTestId("project-storage-panel")).toBeInTheDocument();
    expect(within(record).getByTestId("project-validation-panel")).toBeInTheDocument();
  });

  it("says a model is not saved as a project, and states a saved one's edits in this window apart from the store's latest time", () => {
    expect(projectSavedLine({ projectId: PROJECT, projectSummary: null, modelEdited: true, savedProjects: [THIS] })).toBe("Not saved as a project.");
    expect(projectSavedLine({ projectId: PROJECT, projectSummary: SUMMARY, modelEdited: true, savedProjects: [THIS, OTHER] }))
      .toBe(`Saved project; edited in this window. Latest in store: ${formatAutosaveTime(SAVED_AT)}.`);
    expect(projectSavedLine({ projectId: PROJECT, projectSummary: SUMMARY, modelEdited: false, savedProjects: [THIS] }))
      .toBe(`Saved project; no edits in this window. Latest in store: ${formatAutosaveTime(SAVED_AT)}.`);
    // Before the saved projects are read, the line names no time rather than another one.
    expect(projectSavedLine({ projectId: PROJECT, projectSummary: SUMMARY, modelEdited: false, savedProjects: null })).toBe("Saved project; no edits in this window.");
    expect(projectSavedLine({ projectId: PROJECT, projectSummary: SUMMARY, modelEdited: true, savedProjects: [OTHER] })).toBe("Saved project; edited in this window.");
  });

  it("marks a row open only when the session is bound to that stored project", async () => {
    // A model file or a recovered autosave with a stored project's id is not that stored project.
    listed.projects = [THIS];
    renderPage({ summary: null });
    const row = await screen.findByTestId(`project-saved-open-${PROJECT}`);
    expect(row).not.toHaveTextContent("open now");
    expect(screen.getByTestId("project-page-saved-line")).toHaveTextContent(/^Not saved as a project\.$/);
  });

  it("shows no Recover when there is nothing to recover", async () => {
    listed.projects = [THIS];
    listed.autosaves = { outcome: "listed", entries: [] };
    renderPage({ native: true });
    await screen.findByTestId(`project-saved-open-${PROJECT}`);
    expect(screen.queryByTestId("project-recover-autosave")).toBeNull();
    expect(screen.queryByTestId("project-page-recover-line")).toBeNull();
  });

  it("takes Recover away when a save's cleanup removes the last autosave after the save has landed", async () => {
    listed.projects = [THIS];
    listed.autosaves = { outcome: "listed", entries: [
      { file_name: "a.json", slot: await autosaveSlot(PROJECT), written_unix_millis: SAVED_AT - 60_000, model_hash: `sha256:${"b".repeat(64)}`, byte_count: 10 }
    ] };
    const { recordsChanged } = renderPage({ native: true });
    // The page read the folder when the save landed: the record was still there.
    await screen.findByTestId("project-page-recover-line");
    // The cleanup finishes later and removes it; nothing else about the session changes.
    listed.autosaves = { outcome: "listed", entries: [] };
    recordsChanged(1);
    await waitFor(() => expect(screen.queryByTestId("project-page-recover-line")).toBeNull());
    expect(screen.queryByTestId("project-recover-autosave")).toBeNull();
  });

  it("shows Recover with what and when when an autosave is found", async () => {
    const written = Date.UTC(2026, 10, 21, 10, 5, 0);
    listed.projects = [THIS];
    listed.autosaves = { outcome: "listed", entries: [
      { file_name: "a.json", slot: await autosaveSlot(PROJECT), written_unix_millis: written, model_hash: `sha256:${"b".repeat(64)}`, byte_count: 10 },
      { file_name: "b.json", slot: "0".repeat(32), written_unix_millis: written - 60_000, model_hash: `sha256:${"c".repeat(64)}`, byte_count: 10 }
    ] };
    const { recoverAutosave } = renderPage({ native: true });
    const line = await screen.findByTestId("project-page-recover-line");
    await waitFor(() => expect(line).toHaveTextContent(`Autosave of ${NAME} written ${formatAutosaveTime(written)}, and 1 more.`));
    fireEvent.click(within(line).getByRole("button", { name: "Recover…" }));
    expect(recoverAutosave).toHaveBeenCalledTimes(1);
  });

  it("gives each unavailable Model History action its reason as its title, in plain words", async () => {
    renderPage();
    const history = screen.getByTestId("model-history-panel");
    const record = within(history).getByTestId("record-run-open");
    expect(record).toBeDisabled();
    expect(record).toHaveAttribute("title", "Solve the model to record a run.");
    for (const id of ["model-state-save-open", "record-run-open", "model-state-compare", "model-history-refresh"]) {
      const control = within(history).getByTestId(id);
      if (!(control as HTMLButtonElement).disabled) continue;
      expect(control.getAttribute("title")).toMatch(/^[A-Z][a-z]/);
    }
    // No line of refusal codes after the buttons.
    expect(history).not.toHaveTextContent(/RUN-RECORD-/);
  });
});
