/** DEL-16-03, Commit (A1): the editors' binder is out of the context. The application is rendered as it runs; the
 * session the root receives and the value its context provider is given are captured, and every value reachable
 * from the provider's value (own enumerable members, to any depth, cycles guarded) is walked. The context hooks
 * (`useWorkspaceSessionContext`, `useSessionOperations` and the other slices) return that value or a member of it, so
 * the walk covers what they return. Found by identity, not by name: none of the binder, its two functions, or a
 * callback bound by them. DOM nodes and React's own bookkeeping members on them are not walked: they reach the
 * component tree, not what the hooks return. */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { ReactNode } from "react";
import * as previewService from "../../services/previewService";
import { reconnectModel } from "../../test-support/reconnectModel";
import type { WorkspaceSessionProvider } from "./WorkspaceSessionContext";
import type { WorkspaceSession, EditorBinding } from "./workspaceSession";

const captured = vi.hoisted(() => ({ provided: [] as unknown[], sessions: [] as unknown[], bound: [] as unknown[] }));
vi.mock("./WorkspaceSessionContext", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./WorkspaceSessionContext")>();
  return { ...actual, WorkspaceSessionProvider: (props: { session: WorkspaceSession; children: ReactNode }) => { captured.provided.push(props.session); return actual.WorkspaceSessionProvider(props); } };
});
vi.mock("./workspaceSession", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./workspaceSession")>();
  return { ...actual, useWorkspaceSession: () => {
    const session = actual.useWorkspaceSession();
    captured.sessions.push(session);
    // A callback bound as the root will bind one, so the walk also looks for a bound callback.
    if (captured.bound.length === 0) {
      captured.bound.push(session.editorBinding.bindCommit({ name: "probe", role: "gui_editor", input: { members: ["value"], check: () => null }, build: () => { throw new Error("never built"); } }));
      const dialog = session.editorBinding.bindDialog({ name: "probe-dialog", role: "gui_editor", input: { members: ["value"], check: () => null }, build: () => { throw new Error("never built"); } });
      captured.bound.push(dialog, dialog.preview, dialog.commit);
    }
    return session;
  } };
});
import { App } from "../../App";

// Type level: the provider's session, and the session type the context hooks return, have no binder member; a change
// that widens either fails the type check here.
type ProvidedSession = Parameters<typeof WorkspaceSessionProvider>[0]["session"];
const providerHasNoBinder: "editorBinding" extends keyof ProvidedSession ? false : true = true;
const sessionTypeHasNoBinder: "editorBinding" extends keyof WorkspaceSession ? false : true = true;
const binderIsTyped: keyof EditorBinding = "bindCommit";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

function reachable(root: unknown): Set<unknown> {
  const seen = new Set<unknown>();
  const pending: unknown[] = [root];
  while (pending.length) {
    const value = pending.pop();
    if (value === null || (typeof value !== "object" && typeof value !== "function") || seen.has(value)) continue;
    if (typeof Node !== "undefined" && value instanceof Node) continue;
    seen.add(value);
    for (const key of Object.keys(value as object)) {
      if (key.startsWith("__react")) continue;
      let member: unknown;
      try { member = (value as Record<string, unknown>)[key]; } catch { continue; }
      pending.push(member);
    }
  }
  return seen;
}

it("nothing reachable from what the context provider is given holds the binder, its functions or a bound callback", async () => {
  expect([providerHasNoBinder, sessionTypeHasNoBinder, binderIsTyped]).toEqual([true, true, "bindCommit"]);
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(reconnectModel());
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  const session = captured.sessions.at(-1) as { editorBinding: EditorBinding };
  const binder = session.editorBinding;
  const sought = [binder, binder.bindCommit, binder.bindDialog, ...captured.bound];
  expect(captured.provided.length).toBeGreaterThan(0);
  for (const provided of captured.provided) {
    expect(Object.hasOwn(provided as object, "editorBinding")).toBe(false);
    const found = reachable(provided);
    expect(found.size).toBeGreaterThan(100);
    expect(sought.filter((entry) => found.has(entry))).toEqual([]);
  }
  // The root received the binder, and it is the same across renders.
  expect(new Set(captured.sessions.map((each) => (each as { editorBinding: unknown }).editorBinding)).size).toBe(1);
});
