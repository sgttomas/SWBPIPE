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

// Codex review r2 of #1665 (its probes, kept): asynchronous native checks; the
// latest request wins, and a new project retires a check in progress.
function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}
it("review round 2: a later async check wins; an old completion cannot clear its busy state", async () => {
const releases: Array<(value:unknown)=>void>=[];
invokeMock.mockImplementation((command:string)=>{
if(command==="sync_native_shell_state") return Promise.resolve(undefined);
if(command==="dispose_all_preview_mechanics_results")return Promise.resolve({released_results:0,released_bytes:0,abandoned_jobs:0});
if(command==="check_preview_model")return new Promise(resolve=>releases.push(resolve));
throw new Error(`Unexpected command ${command}`);
});
render(<App />);await screen.findByTestId("desktop-preview-shell");
(window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
act(()=>nativeMenuCommand("analyze.check-model"));
await waitFor(()=>expect(releases).toHaveLength(1));
expect(screen.getByTestId("model-checks-run")).toHaveAttribute("aria-disabled","true");
act(()=>nativeMenuCommand("analyze.check-model"));
await waitFor(()=>expect(releases).toHaveLength(2));
const report=(code:string)=>({checked:true,diagnostics:[{id:code,code,severity:"warning",message:code,affected_refs:[]}],diagnostics_not_shown:0,slot_findings:[],slot_findings_not_shown:0});
await act(async()=>releases[0](report("REVIEW_OLD")));
expect(screen.getByTestId("model-checks-run")).toHaveAttribute("aria-disabled","true");
expect(screen.queryByTestId("model-check-REVIEW_OLD")).toBeNull();
await act(async()=>releases[1](report("REVIEW_NEW")));
expect(screen.getByTestId("model-checks-run")).toHaveAttribute("aria-disabled","false");
expect(screen.getByTestId("model-check-REVIEW_NEW")).toBeInTheDocument();
expect(screen.queryByTestId("model-check-REVIEW_OLD")).toBeNull();
// The reverse completion order also leaves the later request's report in place.
act(()=>nativeMenuCommand("analyze.check-model"));
act(()=>nativeMenuCommand("analyze.check-model"));
await waitFor(()=>expect(releases).toHaveLength(4));
await act(async()=>releases[3](report("REVIEW_LATEST")));
await act(async()=>releases[2](report("REVIEW_OVERTAKEN")));
expect(screen.getByTestId("model-check-REVIEW_LATEST")).toBeInTheDocument();
expect(screen.queryByTestId("model-check-REVIEW_OVERTAKEN")).toBeNull();
});
it("review round 2: a new project retires the previous model-check busy state", async()=>{
let release:(v:unknown)=>void=()=>{};let called=false;
invokeMock.mockImplementation((command:string)=>{
if(command==="sync_native_shell_state")return Promise.resolve(undefined);
if(command==="dispose_all_preview_mechanics_results")return Promise.resolve({released_results:0,released_bytes:0,abandoned_jobs:0});
if(command==="check_preview_model"){called=true;return new Promise(resolve=>{release=resolve;});}
throw new Error(`Unexpected command ${command}`);
});
render(<App/>);await screen.findByTestId("desktop-preview-shell");
(window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
act(()=>nativeMenuCommand("analyze.check-model"));await waitFor(()=>expect(called).toBe(true));
expect(screen.getByTestId("model-checks-run")).toHaveAttribute("aria-disabled","true");
delete (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__;
act(()=>nativeMenuCommand("file.new-blank"));
fireEvent.click(await screen.findByTestId("project-units-create"));
await waitFor(()=>expect(screen.getByTestId("toolbar-project-name")).toHaveTextContent("Blank Local Model"));
fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
const busy=screen.getByTestId("model-checks-run").getAttribute("aria-disabled");
await act(async()=>release({checked:true,diagnostics:[{id:"OLD_PROJECT",code:"OLD_PROJECT",severity:"warning",message:"Old project",affected_refs:[]}],diagnostics_not_shown:0,slot_findings:[],slot_findings_not_shown:0}));
expect(screen.queryByTestId("model-check-OLD_PROJECT")).toBeNull();
expect(busy).toBe("false");
});
