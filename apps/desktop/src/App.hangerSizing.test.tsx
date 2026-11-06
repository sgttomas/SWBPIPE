import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";

// DEL-04-11 increment 3c part 2: the hanger-sizing form and its table are mounted beside the imported-hanger
// selection, on the Supports tool page. The mounted form makes no choice and sizes nothing until the person asks; its
// mount sends at most a keyed retirement (`cancel_hanger_sizing`), never a sizing request.
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
});

it("mounts the hanger-sizing form beside the hanger selection, with no pre-set choice and no sizing sent", async () => {
  render(<App />);
  await screen.findByTestId("workspace-toolbar");
  fireEvent.click(screen.getByTestId("toolkit-entry"));
  fireEvent.click(screen.getByTestId("toolkit-properties.hanger-library"));
  const selection = document.getElementById("hanger-selection")!;
  const page = selection.parentElement!;
  expect(page.hidden).toBe(false);
  const form = within(page).getByTestId("hanger-sizing-form");
  expect(form.previousElementSibling).toBe(selection);
  for (const testId of ["hanger-sizing-library", "hanger-sizing-weight-case", "hanger-sizing-design-case", "hanger-sizing-solver-mode"]) {
    expect((within(form).getByTestId(testId) as HTMLSelectElement).value, testId).toBe("");
  }
  expect(within(form).queryByTestId("hanger-sizing-panel")).toBeNull();
  expect(invokeMock.mock.calls.filter(([command]) => command === "size_hangers")).toEqual([]);
  for (const [, args] of invokeMock.mock.calls.filter(([command]) => command === "cancel_hanger_sizing")) {
    expect(Object.keys(args as Record<string, unknown>)).toEqual(["retirementId"]);
  }
});
