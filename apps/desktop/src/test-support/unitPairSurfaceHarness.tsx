/** Issue 382 surface regressions: renders a real desktop surface bound to the real workspace
 * session exactly as App.tsx binds it, so an operation the surface produces is queued, applied
 * by the session's own route and run by the operation applier in the Wasm engine built from the
 * candidate source. Nothing in the edit path is mocked; only the starting model (an invented
 * one) and, where a surface needs it, the unit catalog the native application supplies are
 * provided. `storedDocument` reads the result back through the project store: the session saves
 * the project and the saved document is reopened. */
import { act, render, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { expect, vi } from "vitest";
import * as previewService from "../services/previewService";
import { openLocalProject } from "../services/projectService";
import * as workspaceSession from "../features/workspace/workspaceSession";
import { useWorkspaceSession, type WorkspaceSession } from "../features/workspace/workspaceSession";
import type { PreviewModel } from "../types";

export type SessionRef = { current: WorkspaceSession };

export async function renderSurface(model: PreviewModel, surface: (session: WorkspaceSession) => ReactNode) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(model));
  const ref = {} as SessionRef;
  function Harness() {
    const session = useWorkspaceSession();
    ref.current = session;
    return session.model.model ? <>{surface(session)}</> : null;
  }
  const view = render(<Harness />);
  await waitFor(() => expect(ref.current?.model.model).not.toBeNull());
  return { session: ref, view };
}

/** For a whole-App test: the App's own workspace session, captured by a call-through wrapper of
 * `useWorkspaceSession` (the real hook runs unchanged), so the stored document can be read back.
 * Call before rendering <App />. */
export function captureAppSession(): SessionRef {
  const ref = {} as SessionRef;
  const real = workspaceSession.useWorkspaceSession;
  vi.spyOn(workspaceSession, "useWorkspaceSession").mockImplementation(() => {
    const session = real();
    ref.current = session;
    return session;
  });
  return ref;
}

/** The session's current document after saving it and reopening the saved project. */
export async function storedDocument(session: SessionRef): Promise<PreviewModel> {
  await act(async () => { await session.current.project.handleCreateProject(); });
  const saved = await openLocalProject(session.current.model.model!.project.id);
  expect(saved, "the project was saved and reopens").not.toBeNull();
  return saved!.model;
}

/** Apply the one queued batch through the session's Review Apply route (App.tsx `onApply`). */
export async function applyQueuedBatch(session: SessionRef) {
  await waitFor(() => expect(session.current.operations.queuedBatches).toHaveLength(1));
  const entry = session.current.operations.queuedBatches[0];
  await act(async () => { await session.current.operations.handleRunOperationBatch(entry, true); });
  return entry;
}

/** The diagnostics codes the engine returned for a batch that was not applied. */
export function refusalCodes(session: SessionRef, key: string): string[] {
  const outcome = session.current.operations.batchOutcomes[key];
  expect(outcome, "the engine returned an outcome for the batch").toBeDefined();
  expect(outcome.validation.application_status).not.toBe("applied_to_session_model");
  return [...outcome.diagnostics, ...outcome.operation_outcomes.flatMap((step) => step.diagnostics)].map((diagnostic) => diagnostic.code);
}
