/** A render error in one page or pane of the window (issue 519: the Project page's storage
 * panel read a member a stored proposal did not have, and the whole window emptied). The
 * failing part shows a plain message in its place and everything outside it keeps working.
 * The throw is made here by replacing a panel's module in this test; the product has no
 * switch for it. The thrown text is the one the issue met; all data are invented. */
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import type { ComponentProps } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const throwing = vi.hoisted(() => ({ storagePanel: false, inspector: false, rail: false }));
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

vi.mock("./features/project-storage/ProjectStorageAuditPanel", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./features/project-storage/ProjectStorageAuditPanel")>();
  return {
    ...actual,
    ProjectStorageAuditPanel: (props: ComponentProps<typeof actual.ProjectStorageAuditPanel>) => {
      if (throwing.storagePanel) throw new TypeError("Cannot read properties of undefined (reading 'application_status')");
      return <actual.ProjectStorageAuditPanel {...props} />;
    }
  };
});
vi.mock("./features/model-tree/PropertyInspector", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./features/model-tree/PropertyInspector")>();
  return {
    ...actual,
    PropertyInspector: (props: ComponentProps<typeof actual.PropertyInspector>) => {
      if (throwing.inspector) throw new TypeError("Cannot read properties of undefined (reading 'kind')");
      return <actual.PropertyInspector {...props} />;
    }
  };
});
vi.mock("./features/workspace/shell/StageRail", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./features/workspace/shell/StageRail")>();
  return {
    ...actual,
    StageRail: (props: ComponentProps<typeof actual.StageRail>) => {
      if (throwing.rail) throw new TypeError("Cannot read properties of undefined (reading 'stage')");
      return <actual.StageRail {...props} />;
    }
  };
});

import { App } from "./App";
import { WindowErrorBoundary } from "./features/workspace/shell/PartErrorBoundary";

beforeEach(() => {
  // The boundary writes each failure once; React reports it too. Neither is under test here.
  vi.spyOn(console, "error").mockImplementation(() => undefined);
});

afterEach(() => {
  throwing.storagePanel = false;
  throwing.inspector = false;
  throwing.rail = false;
  vi.restoreAllMocks();
  invokeMock.mockReset();
  window.localStorage.clear();
});

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

async function renderWindow() {
  Object.defineProperty(window, "innerWidth", { configurable: true, value: 1440 });
  const view = render(<WindowErrorBoundary><App /></WindowErrorBoundary>);
  await screen.findByTestId("desktop-preview-shell");
  return view;
}

function expectChromeStanding() {
  expect(screen.getByTestId("workspace-toolbar")).toBeInTheDocument();
  expect(screen.getByTestId("stage-rail")).toBeInTheDocument();
  expect(screen.getByTestId("workspace-status-bar")).toBeInTheDocument();
  expect(screen.queryByTestId("window-error")).toBeNull();
}

describe("a page that cannot be shown", () => {
  it("shows the Project page's message in its place and leaves the window working", async () => {
    throwing.storagePanel = true;
    await renderWindow();
    act(() => nativeMenuCommand("view.section.project"));

    const project = screen.getByTestId("workspace-section-project");
    await waitFor(() => expect(project).not.toHaveClass("inactive"));
    const message = within(project).getByRole("alert");
    expect(message).toHaveTextContent(/^The Project page could not be shown\.$/);
    expect(within(project).getByTestId("part-error-details")).not.toHaveAttribute("open");
    expect(within(project).getByTestId("part-error-details")).toHaveTextContent("reading 'application_status'");
    expect(within(project).queryByTestId("project-storage-panel")).toBeNull();
    expectChromeStanding();
    // The page's own header controls (outside the failed content) still stand.
    expect(screen.getByTestId("open-local-project")).toBeInTheDocument();

    // Another page opens from the rail and shows its content.
    fireEvent.click(screen.getByTestId("rail-page-libraries"));
    await waitFor(() => expect(screen.getByTestId("workspace-section-libraries")).not.toHaveClass("inactive"));
    expect(within(screen.getByTestId("workspace-section-libraries")).getByTestId("library-manager-panel")).toBeInTheDocument();

    // A menu command opens another page.
    act(() => nativeMenuCommand("view.section.exports"));
    await waitFor(() => expect(screen.getByTestId("workspace-section-exports")).not.toHaveClass("inactive"));
    expect(screen.getByTestId("export-list")).toBeInTheDocument();

    // Coming back shows the message again.
    act(() => nativeMenuCommand("view.section.project"));
    await waitFor(() => expect(project).not.toHaveClass("inactive"));
    expect(within(project).getByRole("alert")).toHaveTextContent("The Project page could not be shown.");

    // Try again while the panel still throws: the message again, no loop.
    fireEvent.click(within(project).getByRole("button", { name: "Try again" }));
    expect(within(project).getByRole("alert")).toHaveTextContent("The Project page could not be shown.");

    // Once the panel no longer throws, Try again shows the page.
    throwing.storagePanel = false;
    fireEvent.click(within(project).getByRole("button", { name: "Try again" }));
    await waitFor(() => expect(within(project).getByTestId("project-storage-panel")).toBeInTheDocument());
    expect(within(project).queryByRole("alert")).toBeNull();
    expectChromeStanding();
  });

  it("shows the Model stage from a failed page", async () => {
    throwing.storagePanel = true;
    await renderWindow();
    act(() => nativeMenuCommand("view.section.project"));
    const project = screen.getByTestId("workspace-section-project");
    await waitFor(() => expect(within(project).getByRole("alert")).toBeInTheDocument());

    fireEvent.click(within(project).getByRole("button", { name: "Show the Model stage" }));
    await waitFor(() => expect(project).toHaveClass("inactive"));
    expect(screen.getByTestId("workspace-dock")).toHaveClass("collapsed");
    expect(screen.getByTestId("rail-stage-model")).toHaveAttribute("aria-pressed", "true");
  });
});

describe("a pane that cannot be shown", () => {
  it("shows the inspector's message in its place and leaves the window working", async () => {
    throwing.inspector = true;
    await renderWindow();

    const inspector = document.getElementById("shell-inspector")!;
    const message = within(inspector).getByRole("alert");
    expect(message).toHaveTextContent(/^The inspector could not be shown\.$/);
    expect(within(inspector).queryByRole("button", { name: "Show the Model stage" })).toBeNull();
    expect(within(inspector).getByTestId("part-error-details")).not.toHaveAttribute("open");
    expect(screen.queryByTestId("property-inspector")).toBeNull();
    expectChromeStanding();
    expect(screen.getByTestId("shell-tree-host")).toBeInTheDocument();

    act(() => nativeMenuCommand("view.section.exports"));
    await waitFor(() => expect(screen.getByTestId("workspace-section-exports")).not.toHaveClass("inactive"));
    expect(screen.getByTestId("export-list")).toBeInTheDocument();

    throwing.inspector = false;
    fireEvent.click(within(inspector).getByRole("button", { name: "Try again" }));
    await waitFor(() => expect(screen.getByTestId("property-inspector")).toBeInTheDocument());
    expect(within(inspector).queryByRole("alert")).toBeNull();
  });
});

describe("a failure outside every page and pane", () => {
  it("leaves the window's message and not an empty document", async () => {
    throwing.rail = true;
    const { container } = render(<WindowErrorBoundary><App /></WindowErrorBoundary>);
    await screen.findByTestId("window-error");
    expect(screen.getByRole("alert")).toHaveTextContent(/^The window could not be shown\.$/);
    expect(screen.getByRole("button", { name: "Reload the window" })).toBeInTheDocument();
    expect(screen.getByText("Changes that were not saved are lost on reload.")).toBeInTheDocument();
    expect(screen.getByTestId("part-error-details")).toHaveTextContent("reading 'stage'");
    expect(container.childElementCount).toBeGreaterThan(0);
  });
});
