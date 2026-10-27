import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";
import * as previewService from "./services/previewService";
import type { PreviewModel } from "./types";

// DEL-04-06 Design `on-demand-model-checks.md`: Analyze → Check Model (⇧⌘M)
// runs the model's existing checks in the full shell, lists their findings in
// the Issues drawer, and a chosen finding selects its entity. The demo gains
// an invented second line that no element joins to it.
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function demoWithSecondLine(): Promise<PreviewModel> {
  const model = JSON.parse(JSON.stringify(await previewService.loadPreviewModel())) as PreviewModel;
  model.nodes.push(
    { ...model.nodes[0], id: "node:checks-c", position: { x: 40, y: 40, z: 0 } },
    { ...model.nodes[0], id: "node:checks-d", position: { x: 41, y: 40, z: 0 } }
  );
  model.pipe_segments.push({ ...model.pipe_segments[0], id: "pipe:checks-c-d", from: "node:checks-c", to: "node:checks-d" });
  model.supports.push({ ...model.supports[0], id: "support:checks-c", node: "node:checks-c" });
  return model;
}

async function renderShell() {
  const model = await demoWithSecondLine();
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  return model;
}

it("lists Check Model in the Analyze menu with its keyboard equivalent", async () => {
  await renderShell();
  fireEvent.click(screen.getByTestId("menu-analyze"));
  const item = screen.getByTestId("menu-item-analyze.check-model");
  expect(item).toHaveTextContent("Check Model⇧⌘M");
  expect(item).toHaveAttribute("aria-keyshortcuts", "Shift+Meta+M Shift+Control+M");
  fireEvent.click(item);
  expect(await screen.findByTestId("model-checks-panel")).toBeInTheDocument();
});

it("runs the checks on ⇧⌘M, lists the topology warning, selects its entity, and changes nothing in the model", async () => {
  const model = await renderShell();
  expect(screen.queryByTestId("issues-home")).toBeNull();
  const firstNode = model.nodes[0].id;
  fireEvent.click(screen.getByTestId("menu-edit"));
  const undoBefore = (screen.getByTestId("menu-item-edit.undo") as HTMLButtonElement).disabled;
  fireEvent.click(screen.getByTestId("menu-edit"));

  act(() => { fireEvent.keyDown(window, { key: "M", metaKey: true, shiftKey: true }); });
  const panel = await screen.findByTestId("model-checks-panel");
  expect(within(screen.getByTestId("issues-home")).getByTestId("model-checks-panel")).toBe(panel);
  const summary = within(panel).getByTestId("model-checks-summary");
  await waitFor(() => expect(document.activeElement).toBe(summary));
  expect(summary).toHaveTextContent(/findings?:/);
  const finding = within(panel).getByTestId("model-check-MODEL_TOPOLOGY_DISCONNECTED");
  expect(finding).toHaveTextContent("MODEL_TOPOLOGY_DISCONNECTED warning");
  expect(finding).toHaveTextContent("(from node:checks-c; 2 nodes; supported)");
  expect(within(panel).queryByTestId("model-checks-stale")).toBeNull();

  expect(screen.getByTestId("status-selection")).not.toHaveTextContent(`node: ${firstNode}`);
  fireEvent.click(finding);
  await waitFor(() => expect(screen.getByTestId("status-selection")).toHaveTextContent(`node: ${firstNode}`));
  // The checks changed nothing: no model edit to undo, and the run still describes the current model.
  fireEvent.click(screen.getByTestId("menu-edit"));
  expect((screen.getByTestId("menu-item-edit.undo") as HTMLButtonElement).disabled).toBe(undoBefore);
  expect(within(panel).queryByTestId("model-checks-stale")).toBeNull();
});

it("runs from the native menu's command too", async () => {
  await renderShell();
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: "analyze.check-model" })); });
  const panel = await screen.findByTestId("model-checks-panel");
  expect(within(panel).getByTestId("model-check-MODEL_TOPOLOGY_DISCONNECTED")).toBeInTheDocument();
});
