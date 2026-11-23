// The automatic model checks in the workspace session (DEL-04-06 REQ-013): replacing a
// checked project checks the new model once; a library-reference publication after a
// completed check (a warning that arrives late, a Library Manager refresh that clears it)
// is listed on that run without checking the engine again; and a check the session runs
// by itself leaves keyboard focus in an editor, where a check the person asks for moves
// it to the outcome. The model checks are the actual wasm engine's; the model is the
// bundled invented model, and every library reference and finding is invented.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import type { LibraryRefFinding } from "../../services/libraryImportService";
import type { PreviewModel } from "../../types";
import { publishLibraryRefFindings } from "../library/libraryRefFindings";
import { useWorkspaceSession, type WorkspaceSession } from "../workspace/workspaceSession";
import * as modelChecks from "./modelChecks";
import { ModelChecksPanel } from "./ModelChecksPanel";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  publishLibraryRefFindings("", []);
});

const realRunModelChecks = modelChecks.runModelChecks;

/** The bundled invented model with one invented library reference on its first material. */
async function referencedModel(): Promise<PreviewModel> {
  const model = structuredClone(await previewService.loadPreviewModel());
  model.materials![0].provenance_record = { source_name: "Invented", library_ref: { library_kind: "material", library_id: "matlib.invented.refs", library_version: "sha256:recorded", record_id: "mat.refs" } };
  return model;
}
function changed(model: PreviewModel): LibraryRefFinding[] {
  return [{ code: "MODEL-LIBRARY-REF-VERSION-CHANGED", severity: "warning", entity_ref: model.materials![0].id, library_kind: "material", library_id: "matlib.invented.refs",
    record_id: "mat.refs", recorded_version: "sha256:recorded", stored_version: "sha256:stored", message: "The stored record has changed." }];
}

/** A session on `model` whose opening check has completed for it. */
async function checkedSession(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  vi.spyOn(previewService, "startPreviewMechanicsJob").mockRejectedValue(new Error("INVENTED-START-OBSERVED"));
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.chrome.modelCheckRun?.model).toBe(hook.result.current.model.model));
  await waitFor(() => expect(hook.result.current.chrome.modelCheckBusy).toBe(false));
  return hook;
}

const library = (session: WorkspaceSession) => session.chrome.modelCheckRun!.findings.filter((finding) => finding.group === "library").map((finding) => finding.diagnostic.code);

it("replacing a checked project checks the replacement's model once", async () => {
  const { result } = await checkedSession(await referencedModel());
  const first = result.current.model.model!;
  const spy = vi.spyOn(modelChecks, "runModelChecks");
  await act(async () => { await result.current.project.handleCreateBlankProject(); });
  await waitFor(() => expect(result.current.model.model).not.toBe(first));
  await waitFor(() => expect(result.current.chrome.modelCheckRun?.model).toBe(result.current.model.model));
  await waitFor(() => expect(result.current.chrome.modelCheckBusy).toBe(false));
  expect(spy).toHaveBeenCalledTimes(1);
  expect(spy.mock.calls[0][0]).toBe(result.current.model.model);
});

it("lists a library-reference warning published after the completed check, and its clear, on that run without checking again", async () => {
  const model = await referencedModel();
  const { result } = await checkedSession(model);
  const opened = result.current.chrome.modelCheckRun!;
  expect(opened.solve.kind).not.toBe("unavailable");
  expect(library(result.current)).toEqual([]);
  const gate = result.current.results.solveGate.kind;
  expect(gate).not.toBe("unknown");
  const spy = vi.spyOn(modelChecks, "runModelChecks");
  const engineFindings = opened.findings.filter((finding) => finding.group !== "library");

  // The store's check of the references answers after the engine's check completed.
  act(() => { publishLibraryRefFindings(model.project.id, changed(result.current.model.model!)); });
  const warned = result.current.chrome.modelCheckRun!;
  expect(warned).not.toBe(opened);
  expect(warned.model).toBe(opened.model);
  expect(warned.solve).toBe(opened.solve);
  expect(warned.findings.filter((finding) => finding.group !== "library")).toEqual(engineFindings);
  expect(library(result.current)).toEqual(["MODEL-LIBRARY-REF-VERSION-CHANGED"]);
  expect(warned.findings.find((finding) => finding.group === "library")!.entity).toEqual({ type: "material", id: model.materials![0].id });
  expect(warned.focusSummary).toBe(false);

  // A Library Manager refresh clears it.
  act(() => { publishLibraryRefFindings(model.project.id, []); });
  expect(library(result.current)).toEqual([]);
  expect(result.current.chrome.modelCheckRun!.findings.filter((finding) => finding.group !== "library")).toEqual(engineFindings);
  expect(result.current.results.solveGate.kind).toBe(gate);
  expect(spy).not.toHaveBeenCalled();
  expect(result.current.chrome.modelCheckBusy).toBe(false);
});

it("leaves focus in an editor when the session's own check completes, and moves it to the outcome of a check the person asks for", async () => {
  const model = await referencedModel();
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  let release: () => void = () => {};
  const held = new Promise<void>((resolve) => { release = resolve; });
  vi.spyOn(modelChecks, "runModelChecks").mockImplementationOnce(async (...args) => { await held; return realRunModelChecks(...args); });
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    const shown = current.model.model;
    return (
      <>
        <input aria-label="Invented editor field" />
        {shown ? <ModelChecksPanel model={shown} run={current.chrome.modelCheckRun} busy={current.chrome.modelCheckBusy}
          onRun={() => { void current.chrome.handleRunModelChecks(); }} onSelectEntity={() => {}} /> : null}
      </>
    );
  }
  render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  await waitFor(() => expect(current.chrome.modelCheckBusy).toBe(true));
  const editor = screen.getByRole("textbox", { name: "Invented editor field" });
  act(() => { editor.focus(); });
  expect(document.activeElement).toBe(editor);
  await act(async () => { release(); await held; });
  await waitFor(() => expect(current.chrome.modelCheckRun?.model).toBe(current.model.model));
  expect(current.chrome.modelCheckRun!.focusSummary).toBe(false);
  expect(document.activeElement).toBe(editor);

  // Check Model: the person's run moves focus to its outcome.
  await act(async () => { fireEvent.click(screen.getByTestId("model-checks-run")); });
  await waitFor(() => expect(current.chrome.modelCheckBusy).toBe(false));
  await waitFor(() => expect(document.activeElement).toBe(screen.getByTestId("model-checks-summary")));
  expect(current.chrome.modelCheckRun!.focusSummary).toBe(true);
});
