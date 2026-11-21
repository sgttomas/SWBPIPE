// The status bar's selection is the ordered selection's primary: with nothing selected it names
// nothing, never the inspector's fallback project; a project a person selected is named.
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { WorkspaceSessionProvider } from "../WorkspaceSessionContext";
import { entityKey } from "../selectionState";
import { ShellStatusBar } from "./ShellStatusBar";
import type { WorkspaceSession } from "../workspaceSession";
import type { EntityRef } from "../../../types";

const project: EntityRef = { type: "project", id: "project:demo" };

function session(fallback: EntityRef, ordered: readonly EntityRef[]): WorkspaceSession {
  const orderedKeys = ordered.map(entityKey);
  return {
    model: { model: null },
    results: { result: null, currentSolvedResult: null, numericalHealthHold: null, historicalRun: null, solveJob: { state: "idle" }, ruleCheckAggregate: null },
    selection: { selection: fallback, orderedSelection: { orderedKeys, primaryKey: orderedKeys.at(-1) ?? null } },
    chrome: { activeSection: null, stageSurface: null, issuesDrawerOpen: false, setIssuesDrawerOpen() {} }
  } as unknown as WorkspaceSession;
}

function statusSelection(fallback: EntityRef, ordered: readonly EntityRef[]) {
  render(<WorkspaceSessionProvider session={session(fallback, ordered)}><ShellStatusBar issueCount={0} /></WorkspaceSessionProvider>);
  return screen.getByTestId("status-selection");
}

describe("the status bar's selection", () => {
  it("names nothing when nothing is selected, though the inspector falls back to the project", () => {
    const element = statusSelection(project, []);
    expect(element).toBeEmptyDOMElement();
    expect(element).not.toHaveAttribute("title");
  });

  it("names the primary of the ordered selection", () => {
    const element = statusSelection(project, [{ type: "node", id: "node:A" }, { type: "pipe", id: "pipe:B" }]);
    expect(element.textContent).toBe("pipe: pipe:B · 2 selected");
    expect(element).toHaveAttribute("title", "pipe: pipe:B · 2 selected");
  });

  it("names a project a person selected", () => {
    expect(statusSelection(project, [project]).textContent).toBe("project: project:demo");
  });
});
