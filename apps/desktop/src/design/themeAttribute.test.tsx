import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "../App";

// tokens.css scopes its theme blocks on :root[data-theme]. The shell keeps its own data-theme
// (the resolved preference); the document root has to carry the same value, or the tokens would
// follow the operating system while the shell follows the engineer's explicit choice.

afterEach(() => {
  cleanup();
  window.localStorage.clear();
});

describe("design tokens follow the product's resolved theme", () => {
  it("keeps data-theme on the document root equal to the shell's, for every preference, and releases it on unmount", async () => {
    const root = document.documentElement;
    const before = root.getAttribute("data-theme");
    const view = render(<App />);
    const shell = await screen.findByTestId("desktop-preview-shell");

    expect(shell.getAttribute("data-theme-preference")).toBe("system");
    expect(["light", "dark"]).toContain(shell.getAttribute("data-theme"));
    expect(root.getAttribute("data-theme")).toBe(shell.getAttribute("data-theme"));

    const select = screen.getByLabelText("Appearance theme");
    for (const preference of ["dark", "light", "system"] as const) {
      fireEvent.change(select, { target: { value: preference } });
      await waitFor(() => expect(shell.getAttribute("data-theme-preference")).toBe(preference));
      if (preference !== "system") expect(shell.getAttribute("data-theme")).toBe(preference);
      expect(root.getAttribute("data-theme")).toBe(shell.getAttribute("data-theme"));
    }

    view.unmount();
    expect(root.getAttribute("data-theme")).toBe(before);
  });
});
