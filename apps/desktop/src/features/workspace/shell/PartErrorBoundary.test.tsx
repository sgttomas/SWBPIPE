import { fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { PartErrorBoundary, WindowErrorBoundary } from "./PartErrorBoundary";

// A render error in one part shows a plain message in that part's place. The thrown
// value and the part are invented; nothing here is a product hook.

afterEach(() => vi.restoreAllMocks());

const control = { throwing: true, mounts: 0 };

function Part() {
  const [mountedAs] = useState(() => ++control.mounts);
  if (control.throwing) throw new TypeError("Cannot read properties of undefined (reading 'application_status')");
  return <p data-testid="part-content">Part content, mount {mountedAs}</p>;
}

function quietConsole() {
  return vi.spyOn(console, "error").mockImplementation(() => undefined);
}

describe("PartErrorBoundary", () => {
  it("names the part, keeps Details closed with the error's text, and Try again mounts the part afresh", () => {
    const consoleError = quietConsole();
    control.throwing = true;
    control.mounts = 0;
    render(<><PartErrorBoundary subject="The Project page"><Part /></PartErrorBoundary><button type="button">Outside</button></>);

    const alert = screen.getByRole("alert");
    expect(alert).toHaveTextContent(/^The Project page could not be shown\.$/);
    const details = screen.getByTestId("part-error-details");
    expect(details).not.toHaveAttribute("open");
    expect(details).toHaveTextContent("TypeError: Cannot read properties of undefined (reading 'application_status')");
    expect(screen.getByRole("button", { name: "Try again" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Show the Model stage" })).toBeNull();
    expect(screen.queryByText(/crash|reopen|lost|intact/i)).toBeNull();
    expect(document.activeElement).toBe(document.body);
    expect(consoleError.mock.calls.filter(([first]) => first === "The Project page could not be shown.")).toHaveLength(1);
    expect(screen.getByRole("button", { name: "Outside" })).toBeInTheDocument();

    // A second throw shows the message again and does not loop.
    const mountsBefore = control.mounts;
    fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(screen.getByRole("alert")).toHaveTextContent("The Project page could not be shown.");
    expect(control.mounts).toBeGreaterThan(mountsBefore);

    control.throwing = false;
    fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(screen.queryByRole("alert")).toBeNull();
    expect(screen.getByTestId("part-content")).toBeInTheDocument();
  });

  it("tries a failed part again when the session key changes, and leaves a working part mounted", () => {
    quietConsole();
    control.throwing = true;
    control.mounts = 0;
    const view = render(<PartErrorBoundary subject="The inspector" resetKey={1}><Part /></PartErrorBoundary>);
    expect(screen.getByRole("alert")).toHaveTextContent("The inspector could not be shown.");

    view.rerender(<PartErrorBoundary subject="The inspector" resetKey={1}><Part /></PartErrorBoundary>);
    expect(screen.getByRole("alert")).toBeInTheDocument();

    control.throwing = false;
    view.rerender(<PartErrorBoundary subject="The inspector" resetKey={2}><Part /></PartErrorBoundary>);
    expect(screen.queryByRole("alert")).toBeNull();
    const mounted = screen.getByTestId("part-content").textContent;

    // A working part is not remounted by a new session; only a failed one is tried again.
    view.rerender(<PartErrorBoundary subject="The inspector" resetKey={3}><Part /></PartErrorBoundary>);
    expect(screen.getByTestId("part-content").textContent).toBe(mounted);

    control.throwing = true;
    view.rerender(<PartErrorBoundary subject="The inspector" resetKey={4}><Part key="again" /></PartErrorBoundary>);
    expect(screen.getByRole("alert")).toHaveTextContent("The inspector could not be shown.");
  });
});

describe("WindowErrorBoundary", () => {
  it("leaves words and one way on in the window when everything below it fails", () => {
    const consoleError = quietConsole();
    control.throwing = true;
    const { container } = render(<WindowErrorBoundary><Part /></WindowErrorBoundary>);
    expect(screen.getByRole("alert")).toHaveTextContent(/^The window could not be shown\.$/);
    expect(screen.getAllByRole("button").map((button) => button.textContent)).toEqual(["Reload the window"]);
    expect(screen.getByRole("button", { name: "Reload the window" })).toHaveAccessibleDescription("Changes that were not saved are lost on reload.");
    expect(screen.getByTestId("part-error-details")).not.toHaveAttribute("open");
    expect(screen.getByTestId("part-error-details")).toHaveTextContent("TypeError: Cannot read properties of undefined");
    expect(container.childElementCount).toBeGreaterThan(0);
    expect(consoleError.mock.calls.filter(([first]) => first === "The window could not be shown.")).toHaveLength(1);
  });
});
