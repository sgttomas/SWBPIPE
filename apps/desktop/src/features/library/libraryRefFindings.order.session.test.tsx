// The library-reference findings stay those of the latest check of the open
// project (DEL-04-06 REQ-013; W-13 §3.5): a check that answers after a later one
// started, for another project or for the same one, publishes nothing, so the
// later check's warning stands in the store, in the model checks and in the
// Library Manager. The workspace session, the model checks and the panel are
// the product's; the store's two reads (the list and the reference check) are
// held or answered here. Every model, reference and finding is invented.
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import * as libraryImportService from "../../services/libraryImportService";
import type {
  LibraryDeleteRoute,
  LibraryImportValidation,
  LibraryListRoute,
  LibraryRefCheckRoute,
  LibraryRefFinding,
  LibrarySaveRoute,
  LocalLibraryIndexEntry
} from "../../services/libraryImportService";
import type { ModelDocumentSource } from "../../services/modelDocumentFile";
import type { PreviewModel } from "../../types";
import { useWorkspaceSession, type WorkspaceSession } from "../workspace/workspaceSession";
import { LibraryManagerPanel } from "./LibraryManagerPanel";
import {
  beginLibraryRefCheck,
  beginLibraryRefCheckIfActive,
  publishedLibraryRefFindings,
  publishLibraryRefFindings,
  publishLibraryRefFindingsIfCurrent,
  setActiveLibraryRefProject
} from "./libraryRefFindings";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  publishLibraryRefFindings("", []);
});

const PROJECT_A = "project:invented-refs-a";
const PROJECT_B = "project:invented-refs-b";

/** The bundled invented model, as project `projectId`, with one invented library reference on its first material. */
async function referencedModel(projectId: string): Promise<PreviewModel> {
  const model = structuredClone(await previewService.loadPreviewModel());
  model.project = { ...model.project, id: projectId };
  model.materials![0].provenance_record = { source_name: "Invented", library_ref: { library_kind: "material", library_id: "matlib.invented.refs", library_version: "sha256:recorded", record_id: "mat.refs" } };
  return model;
}
function changed(model: PreviewModel): LibraryRefFinding[] {
  return [{ code: "MODEL-LIBRARY-REF-VERSION-CHANGED", severity: "warning", entity_ref: model.materials![0].id, library_kind: "material", library_id: "matlib.invented.refs",
    record_id: "mat.refs", recorded_version: "sha256:recorded", stored_version: "sha256:stored", message: "The stored record has changed." }];
}
function documentSource(document: PreviewModel): ModelDocumentSource {
  return {
    action: "Opening a model document",
    read: async () => ({ outcome: "opened", file_name: "invented.json", byte_count: 1, schema_status: "current", document: structuredClone(document), warnings: [] }),
    opened: () => "Opened the invented model document.",
    operation: "open_model"
  };
}

/** The store's reference check, held for project A's first call and answered at once with `answers` for every other. */
function heldReferenceCheck(answers: (projectId: string) => LibraryRefFinding[]) {
  let release: (findings: LibraryRefFinding[]) => void = () => {};
  const held = new Promise<LibraryRefCheckRoute>((resolve) => { release = (findings) => resolve({ route: "tauri_backend", findings }); });
  let holding = true;
  const calls: string[] = [];
  vi.spyOn(libraryImportService, "listLocalLibraries").mockResolvedValue({ route: "tauri_backend", entries: [] });
  vi.spyOn(libraryImportService, "checkModelLibraryRefs").mockImplementation(async (projectId) => {
    calls.push(projectId);
    if (holding && projectId === PROJECT_A) { holding = false; return held; }
    return { route: "tauri_backend", findings: answers(projectId) };
  });
  return { release, calls, released: held };
}

/** The session with the Library Manager on its model, opened on `model` with its opening check completed. */
async function workspace(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  vi.spyOn(previewService, "startPreviewMechanicsJob").mockRejectedValue(new Error("INVENTED-START-OBSERVED"));
  const session: { current: WorkspaceSession } = { current: undefined as unknown as WorkspaceSession };
  function Harness() {
    session.current = useWorkspaceSession();
    return <LibraryManagerPanel model={session.current.model.model} />;
  }
  render(<Harness />);
  await waitFor(() => expect(session.current.model.model).not.toBeNull());
  await waitFor(() => expect(session.current.chrome.modelCheckRun?.model).toBe(session.current.model.model));
  await waitFor(() => expect(session.current.chrome.modelCheckBusy).toBe(false));
  return session;
}

const library = (session: WorkspaceSession) => session.chrome.modelCheckRun!.findings.filter((finding) => finding.group === "library").map((finding) => finding.diagnostic.code);

async function expectWarningStands(session: { current: WorkspaceSession }, projectId: string) {
  const model = session.current.model.model!;
  expect(model.project.id).toBe(projectId);
  expect(publishedLibraryRefFindings().projectId).toBe(projectId);
  expect(publishedLibraryRefFindings().findings.map((finding) => finding.code)).toEqual(["MODEL-LIBRARY-REF-VERSION-CHANGED"]);
  expect(library(session.current)).toEqual(["MODEL-LIBRARY-REF-VERSION-CHANGED"]);
  expect(screen.getByTestId(`library-ref-finding-${model.materials![0].id}`).textContent).toContain("MODEL-LIBRARY-REF-VERSION-CHANGED (warning)");
}

it("a Library Manager refresh of project A that answers after project B opened leaves B's warning", async () => {
  const a = await referencedModel(PROJECT_A);
  const b = await referencedModel(PROJECT_B);
  const check = heldReferenceCheck((projectId) => (projectId === PROJECT_B ? changed(b) : []));
  const session = await workspace(a);

  // The refresh of A lists the store and holds on its reference check.
  await act(async () => { screen.getByTestId("library-refresh-list").click(); });
  await waitFor(() => expect(check.calls).toEqual([PROJECT_A]));

  // B opens; its own check publishes its warning.
  await act(async () => { await session.current.project.handleOpenModelDocument(documentSource(b)); });
  await waitFor(() => expect(session.current.chrome.modelCheckRun?.model).toBe(session.current.model.model));
  await waitFor(() => expect(session.current.chrome.modelCheckBusy).toBe(false));
  expect(check.calls).toEqual([PROJECT_A, PROJECT_B]);
  await expectWarningStands(session, PROJECT_B);

  // A's check answers now, with nothing to report for A.
  await act(async () => { check.release([]); await check.released; });
  await expectWarningStands(session, PROJECT_B);
  expect(screen.getByTestId("library-list-status").textContent).toBe("Local library list not refreshed for this project yet.");
});

it("a Library Manager refresh that answers after the project's own later check leaves that check's warning", async () => {
  const a = await referencedModel(PROJECT_A);
  const check = heldReferenceCheck(() => changed(a));
  const session = await workspace(a);

  await act(async () => { screen.getByTestId("library-refresh-list").click(); });
  await waitFor(() => expect(check.calls).toEqual([PROJECT_A]));

  // The same project's document opens again; its check, started later, answers first.
  await act(async () => { await session.current.project.handleOpenModelDocument(documentSource(a)); });
  await waitFor(() => expect(session.current.chrome.modelCheckRun?.model).toBe(session.current.model.model));
  await waitFor(() => expect(session.current.chrome.modelCheckBusy).toBe(false));
  expect(check.calls).toEqual([PROJECT_A, PROJECT_A]);
  await expectWarningStands(session, PROJECT_A);

  // The earlier check answers last, with nothing to report.
  await act(async () => { check.release([]); await check.released; });
  await expectWarningStands(session, PROJECT_A);
});

it("publishes a check's answer only while it is the latest check of the active project", () => {
  const finding = (projectId: string): LibraryRefFinding[] => [{ code: "MODEL-LIBRARY-REF-UNRESOLVED", severity: "warning", entity_ref: "material:invented",
    library_kind: "material", library_id: "matlib.invented", recorded_version: "v1", stored_version: null, message: `Unresolved in ${projectId}.` }];

  // Across projects: A's check starts, B opens and its check answers first; A's answer is dropped.
  setActiveLibraryRefProject(PROJECT_A);
  const forA = beginLibraryRefCheck(PROJECT_A);
  setActiveLibraryRefProject(PROJECT_B);
  const forB = beginLibraryRefCheck(PROJECT_B);
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_B, forB, finding(PROJECT_B))).toBe(true);
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_A, forA, [])).toBe(false);
  expect(publishedLibraryRefFindings()).toEqual({ projectId: PROJECT_B, findings: finding(PROJECT_B) });

  // Within one project: the second check answers first; the first one's answer is dropped.
  const first = beginLibraryRefCheck(PROJECT_B);
  const second = beginLibraryRefCheck(PROJECT_B);
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_B, second, finding("second"))).toBe(true);
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_B, first, [])).toBe(false);
  expect(publishedLibraryRefFindings().findings).toEqual(finding("second"));

  // A project change alone supersedes the check in flight.
  const pending = beginLibraryRefCheck(PROJECT_B);
  setActiveLibraryRefProject(PROJECT_A);
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_B, pending, [])).toBe(false);
  expect(publishedLibraryRefFindings().findings).toEqual(finding("second"));

  // A direct publication supersedes a check in flight too.
  const overtaken = beginLibraryRefCheck(PROJECT_A);
  publishLibraryRefFindings(PROJECT_A, finding(PROJECT_A));
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_A, overtaken, [])).toBe(false);
  expect(publishedLibraryRefFindings()).toEqual({ projectId: PROJECT_A, findings: finding(PROJECT_A) });
});

/** A promise and the function that settles it. */
function deferred<T>() {
  let resolve: (value: T) => void = () => {};
  const promise = new Promise<T>((settle) => { resolve = settle; });
  return { promise, resolve };
}

const ENTRY: LocalLibraryIndexEntry = { project_id: PROJECT_A, library_kind: "material", library_id: "matlib.invented.listed", library_name: "Invented listed",
  privacy_class: "private_user_supplied", storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 1 };
const ACCEPTED: LibraryImportValidation = { document_kind: "library_import", outcome: "PRIVATE_LOCAL_ONLY", library_kind: "material", intended_visibility: "private",
  accepted: true, has_blocking_findings: false, findings: [], diagnostics: [], professional_boundary_notice: "Invented notice." };

/** Opens `model` through the session without waiting for the open to finish. */
function startOpen(session: { current: WorkspaceSession }, model: PreviewModel): Promise<void> {
  let opening: Promise<unknown> = Promise.resolve();
  act(() => { opening = session.current.project.handleOpenModelDocument(documentSource(model)); });
  return opening.then(() => undefined);
}

async function openCompletely(session: { current: WorkspaceSession }, model: PreviewModel) {
  await act(async () => { await session.current.project.handleOpenModelDocument(documentSource(model)); });
  await waitFor(() => expect(session.current.chrome.modelCheckRun?.model).toBe(session.current.model.model));
  await waitFor(() => expect(session.current.chrome.modelCheckBusy).toBe(false));
  expect(session.current.model.model!.project.id).toBe(model.project.id);
}

const refreshButton = () => screen.getByTestId("library-refresh-list") as HTMLButtonElement;

it.each(["save", "delete"] as const)("a Library Manager %s in project A that settles while B's open check is pending leaves B's warning", async (action) => {
  const a = await referencedModel(PROJECT_A);
  const b = await referencedModel(PROJECT_B);
  const bCheck = deferred<LibraryRefCheckRoute>();
  const calls: string[] = [];
  let holdB = true;
  vi.spyOn(libraryImportService, "listLocalLibraries").mockImplementation(async (projectId) =>
    ({ route: "tauri_backend", entries: projectId === PROJECT_A ? [ENTRY] : [] }));
  vi.spyOn(libraryImportService, "checkModelLibraryRefs").mockImplementation(async (projectId) => {
    calls.push(projectId);
    if (projectId === PROJECT_B && holdB) { holdB = false; return bCheck.promise; }
    return { route: "tauri_backend", findings: projectId === PROJECT_B ? changed(b) : [] };
  });
  const held = deferred<LibrarySaveRoute | LibraryDeleteRoute>();
  const save = vi.spyOn(libraryImportService, "saveLocalLibrary").mockImplementation(() => held.promise as Promise<LibrarySaveRoute>);
  const remove = vi.spyOn(libraryImportService, "deleteLocalLibrary").mockImplementation(() => held.promise as Promise<LibraryDeleteRoute>);
  const session = await workspace(a);

  // A's action starts and holds on the store.
  if (action === "save") {
    await act(async () => { screen.getByTestId("library-load-template").click(); });
    await act(async () => { screen.getByTestId("library-save").click(); });
    expect(save).toHaveBeenCalledTimes(1);
  } else {
    await act(async () => { refreshButton().click(); });
    await waitFor(() => expect(screen.getByTestId(`library-delete-${ENTRY.library_id}`)).toBeTruthy());
    await act(async () => { screen.getByTestId(`library-delete-${ENTRY.library_id}`).click(); });
    expect(remove).toHaveBeenCalledTimes(1);
  }

  // B opens; its open check holds.
  const opening = startOpen(session, b);
  await waitFor(() => expect(calls[calls.length - 1]).toBe(PROJECT_B));

  // A's action settles while B's check is pending.
  await act(async () => {
    held.resolve(action === "save"
      ? { route: "tauri_backend", result: { project_id: PROJECT_A, library_kind: "material", library_id: "matlib.invented.saved", stored: true, storage_mode: "local_sqlite",
        created_at_unix: 1, updated_at_unix: 1, document: {}, validation: ACCEPTED, message: "Stored." } }
      : { route: "tauri_backend", receipt: { project_id: PROJECT_A, library_kind: "material", library_id: ENTRY.library_id, deleted: true, message: "Deleted." } });
    await held.promise;
  });

  // B's check answers with its warning.
  await act(async () => { bCheck.resolve({ route: "tauri_backend", findings: changed(b) }); await opening; });
  await waitFor(() => expect(session.current.chrome.modelCheckRun?.model).toBe(session.current.model.model));
  await waitFor(() => expect(session.current.chrome.modelCheckBusy).toBe(false));
  await expectWarningStands(session, PROJECT_B);
});

it.each(["refresh", "save"] as const)("project B's Library Manager controls are enabled at once while A's %s is pending, and A's settlement leaves B's busy state alone", async (action) => {
  const a = await referencedModel(PROJECT_A);
  const b = await referencedModel(PROJECT_B);
  const aList = deferred<LibraryListRoute>();
  const bList = deferred<LibraryListRoute>();
  const aSave = deferred<LibrarySaveRoute>();
  vi.spyOn(libraryImportService, "checkModelLibraryRefs").mockResolvedValue({ route: "tauri_backend", findings: [] });
  vi.spyOn(libraryImportService, "listLocalLibraries").mockImplementation((projectId) => (projectId === PROJECT_A ? aList.promise : bList.promise));
  vi.spyOn(libraryImportService, "saveLocalLibrary").mockImplementation(() => aSave.promise);
  const session = await workspace(a);

  if (action === "save") {
    await act(async () => { screen.getByTestId("library-load-template").click(); });
    await act(async () => { screen.getByTestId("library-save").click(); });
  } else {
    await act(async () => { refreshButton().click(); });
  }
  expect(refreshButton().disabled).toBe(true);

  await openCompletely(session, b);
  expect(refreshButton().disabled).toBe(false);

  // B's own refresh is in flight when A's action settles: B stays busy until its own answers.
  await act(async () => { refreshButton().click(); });
  expect(refreshButton().disabled).toBe(true);
  await act(async () => {
    if (action === "save") {
      aSave.resolve({ route: "tauri_backend", result: { project_id: PROJECT_A, library_kind: "material", library_id: "matlib.invented.saved", stored: true,
        storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 1, document: {}, validation: ACCEPTED, message: "Stored." } });
      await aSave.promise;
    } else {
      aList.resolve({ route: "tauri_backend", entries: [ENTRY] });
      await aList.promise;
    }
  });
  expect(refreshButton().disabled).toBe(true);
  expect(screen.getByTestId("library-list-status").textContent).toBe("Local library list not refreshed for this project yet.");
  expect(screen.getByTestId("library-action-status").textContent).not.toContain("Stored.");
  await act(async () => { bList.resolve({ route: "tauri_backend", entries: [] }); await bList.promise; });
  await waitFor(() => expect(refreshButton().disabled).toBe(false));
  expect(screen.getByTestId("library-list-status").textContent).toBe(`Local store listed 0 library(ies) for ${PROJECT_B}.`);
});

it("a refresh of project A that answers after A was left for B and reopened changes nothing in the reopened A", async () => {
  const a = await referencedModel(PROJECT_A);
  const b = await referencedModel(PROJECT_B);
  const firstA = deferred<LibraryListRoute>();
  let holding = true;
  vi.spyOn(libraryImportService, "checkModelLibraryRefs").mockResolvedValue({ route: "tauri_backend", findings: [] });
  vi.spyOn(libraryImportService, "listLocalLibraries").mockImplementation(async (projectId) => {
    if (projectId === PROJECT_A && holding) { holding = false; return firstA.promise; }
    return { route: "tauri_backend", entries: [] };
  });
  const session = await workspace(a);

  await act(async () => { refreshButton().click(); });
  await openCompletely(session, b);
  await openCompletely(session, a);

  await act(async () => { firstA.resolve({ route: "tauri_backend", entries: [ENTRY] }); await firstA.promise; });
  expect(screen.queryByTestId(`library-entry-${ENTRY.library_id}`)).toBeNull();
  expect(screen.getByTestId("library-list-status").textContent).toBe("Local library list not refreshed for this project yet.");
  expect(refreshButton().disabled).toBe(false);
});

it("a check begun only if its project is active neither changes the active project nor supersedes its check", () => {
  const finding: LibraryRefFinding[] = [{ code: "MODEL-LIBRARY-REF-UNRESOLVED", severity: "warning", entity_ref: "material:invented",
    library_kind: "material", library_id: "matlib.invented", recorded_version: "v1", stored_version: null, message: "Unresolved in B." }];
  setActiveLibraryRefProject(PROJECT_B);
  const forB = beginLibraryRefCheck(PROJECT_B);
  expect(beginLibraryRefCheckIfActive(PROJECT_A)).toBeNull();
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_B, forB, finding)).toBe(true);
  expect(publishedLibraryRefFindings()).toEqual({ projectId: PROJECT_B, findings: finding });

  // A project change alone, before any check: the other project still begins nothing.
  setActiveLibraryRefProject(PROJECT_B);
  expect(beginLibraryRefCheckIfActive(PROJECT_A)).toBeNull();
  const later = beginLibraryRefCheckIfActive(PROJECT_B);
  expect(later).not.toBeNull();
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_B, later!, [])).toBe(true);

  // With no active project yet, the check begins and makes its project active.
  publishLibraryRefFindings("", []);
  const first = beginLibraryRefCheckIfActive(PROJECT_A);
  expect(first).not.toBeNull();
  expect(publishLibraryRefFindingsIfCurrent(PROJECT_A, first!, finding)).toBe(true);
});

it("after Save as a new project, a Library Manager refresh checks the copy and publishes its warning", async () => {
  const a = await referencedModel(PROJECT_A);
  vi.spyOn(libraryImportService, "listLocalLibraries").mockResolvedValue({ route: "tauri_backend", entries: [] });
  const checks = vi.spyOn(libraryImportService, "checkModelLibraryRefs").mockImplementation(async (projectId) => {
    const copy = session.current.model.model!;
    return { route: "tauri_backend", findings: projectId === PROJECT_A ? [] : changed(copy) };
  });
  const session = await workspace(a);

  // A is stored; storing it again asks, and Save as a new project adopts the copy under an unused id.
  await act(async () => { await session.current.project.handleCreateProject(); });
  let creating: Promise<unknown> = Promise.resolve();
  act(() => { creating = session.current.project.handleCreateProject(); });
  await waitFor(() => expect(session.current.project.replaceProjectPrompt).not.toBeNull());
  await act(async () => { session.current.project.replaceProjectPrompt!.choose("save-as-new"); await creating; });
  const copyId = session.current.model.model!.project.id;
  expect(copyId).toMatch(new RegExp(`^${PROJECT_A}:copy-`));

  await act(async () => { refreshButton().click(); });
  await waitFor(() => expect(checks.mock.calls.map(([projectId]) => projectId)).toContain(copyId));
  await waitFor(() => expect(refreshButton().disabled).toBe(false));
  await expectWarningStands(session, copyId);
});
