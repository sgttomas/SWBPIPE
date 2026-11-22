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
import type { LibraryRefCheckRoute, LibraryRefFinding } from "../../services/libraryImportService";
import type { ModelDocumentSource } from "../../services/modelDocumentFile";
import type { PreviewModel } from "../../types";
import { useWorkspaceSession, type WorkspaceSession } from "../workspace/workspaceSession";
import { LibraryManagerPanel } from "./LibraryManagerPanel";
import {
  beginLibraryRefCheck,
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
