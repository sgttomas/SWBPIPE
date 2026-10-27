// B9: the viewport's entity context menu. A right-click on an entity (its label target, or the
// canvas where a press released in place opens it and a right-drag still pans), and Shift+F10
// or the ContextMenu key on a focused label or on the canvas. Each item runs the toolbar's own
// command. Uses the bundled invented reference model; jsdom has no WebGL, so the canvas picks
// nothing and the canvas menu is the primary selection's.
import { createEvent, fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { loadBundledMechanicsReference } from "../../services/previewService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { buildDeleteSupportIntent } from "../model-tree/removalIntents";
import { entityKey, singletonSelection, type EntityKey } from "../workspace/selectionState";
import { PipeViewport } from "./PipeViewport";

function mount(model: PreviewModel, selection: EntityRef) {
  const onSelect = vi.fn((ref: EntityRef) => singletonSelection(ref));
  const onHiddenKeysChange = vi.fn<(keys: ReadonlySet<EntityKey>) => void>();
  const onIsolationSelectionChange = vi.fn<(keys: ReadonlySet<EntityKey> | null) => void>();
  const onQueueIntent = vi.fn<(intent: EditorOperationIntent) => void>();
  render(
    <DisplayUnitsProvider>
      <PipeViewport model={model} result={null} selection={selection} selectionState={{ ...singletonSelection(selection), focusKey: entityKey(selection) }}
        onSelect={onSelect} onHiddenKeysChange={onHiddenKeysChange} onIsolationSelectionChange={onIsolationSelectionChange} onQueueIntent={onQueueIntent} />
    </DisplayUnitsProvider>
  );
  return { onSelect, onHiddenKeysChange, onIsolationSelectionChange, onQueueIntent };
}

// jsdom has no PointerEvent: it dispatches a plain Event, so the pointer fields are set here.
function pointer(kind: "pointerDown" | "pointerUp", target: Element, fields: { button: number; pointerId: number; clientX: number; clientY: number }) {
  const event = createEvent[kind](target);
  for (const [key, value] of Object.entries(fields)) Object.defineProperty(event, key, { value });
  fireEvent(target, event);
}

const label = (id: string) => within(screen.getByTestId("viewport-selection-layer")).getByTestId(`viewport-select-${id}`);
const menuItem = (id: string) => screen.getByTestId(`viewport-context-menu-item-${id}`);

describe("the viewport context menu", () => {
  it("a right-click on an unselected entity selects it first and opens its menu with the viewport's shortcuts", async () => {
    const { model } = await loadBundledMechanicsReference();
    const [first, second] = model.pipe_segments;
    const { onSelect } = mount(model, { type: "pipe", id: first.id });
    fireEvent.contextMenu(label(second.id), { clientX: 120, clientY: 80 });
    expect(onSelect).toHaveBeenCalledExactlyOnceWith({ type: "pipe", id: second.id });
    expect(screen.getByRole("menu", { name: `Commands for pipe ${second.id}` })).toBeInTheDocument();
    expect(menuItem("hide-selection")).toHaveAttribute("aria-keyshortcuts", "H");
    expect(menuItem("isolate-selection")).toHaveAttribute("aria-keyshortcuts", "I");
    expect(menuItem("hide-selection")).toHaveFocus();
  });

  it("Hide and Isolate run the toolbar's own command, with the same result", async () => {
    const { model } = await loadBundledMechanicsReference();
    const pipe = model.pipe_segments[0];
    const { onSelect, onHiddenKeysChange, onIsolationSelectionChange } = mount(model, { type: "pipe", id: pipe.id });
    fireEvent.click(screen.getByRole("button", { name: "Hide" }));
    fireEvent.click(screen.getByRole("button", { name: "Isolate" }));
    const toolbarHidden = onHiddenKeysChange.mock.calls[0][0];
    const toolbarIsolated = onIsolationSelectionChange.mock.calls[0][0];

    fireEvent.contextMenu(label(pipe.id));
    // The entity is selected: the right-click keeps the selection.
    expect(onSelect).not.toHaveBeenCalled();
    fireEvent.click(menuItem("hide-selection"));
    fireEvent.contextMenu(label(pipe.id));
    fireEvent.click(menuItem("isolate-selection"));
    expect(onHiddenKeysChange.mock.calls[1][0]).toEqual(toolbarHidden);
    expect(onIsolationSelectionChange.mock.calls[1][0]).toEqual(toolbarIsolated);
    expect(screen.getByTestId("viewport-view-command-status")).toHaveTextContent("1 selected item isolated");
  });

  it("opens on the canvas from the keyboard for the primary selection and returns focus there", async () => {
    const { model } = await loadBundledMechanicsReference();
    const pipe = model.pipe_segments[0];
    const { onSelect } = mount(model, { type: "pipe", id: pipe.id });
    const canvas = screen.getByTestId("viewport-canvas");
    canvas.focus();
    fireEvent.keyDown(canvas, { key: "F10", shiftKey: true });
    const menu = screen.getByRole("menu", { name: `Commands for pipe ${pipe.id}` });
    fireEvent.keyDown(menu, { key: "End" });
    expect(menuItem("queue-removal")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "Escape" });
    expect(screen.queryByRole("menu")).toBeNull();
    expect(canvas).toHaveFocus();
    // H on the canvas still hides; the menu's keys never reach the viewport.
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("on the canvas, a secondary press released in place opens the menu and a right-drag does not", async () => {
    const { model } = await loadBundledMechanicsReference();
    const pipe = model.pipe_segments[0];
    const { onSelect } = mount(model, { type: "pipe", id: pipe.id });
    const canvas = screen.getByTestId("viewport-canvas");
    pointer("pointerDown", canvas, { button: 2, pointerId: 1, clientX: 10, clientY: 10 });
    pointer("pointerUp", canvas, { button: 2, pointerId: 1, clientX: 40, clientY: 10 });
    expect(screen.queryByRole("menu")).toBeNull();
    pointer("pointerDown", canvas, { button: 2, pointerId: 1, clientX: 10, clientY: 10 });
    pointer("pointerUp", canvas, { button: 2, pointerId: 1, clientX: 12, clientY: 11 });
    expect(screen.getByRole("menu", { name: `Commands for pipe ${pipe.id}` })).toBeInTheDocument();
    expect(fireEvent.contextMenu(canvas)).toBe(false);
    expect(onSelect).not.toHaveBeenCalled();
  });

  it("from the keyboard on an unselected label, keeps the selection and disables the selection commands with the reason", async () => {
    const { model } = await loadBundledMechanicsReference();
    const [first, second] = model.pipe_segments;
    const { onSelect, onHiddenKeysChange } = mount(model, { type: "pipe", id: first.id });
    const target = label(second.id);
    target.focus();
    fireEvent.keyDown(target, { key: "ContextMenu" });
    expect(onSelect).not.toHaveBeenCalled();
    expect(menuItem("hide-selection")).toHaveAccessibleDescription(`This acts on the selection, which does not include ${second.id}. Select it first.`);
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    expect(onHiddenKeysChange).not.toHaveBeenCalled();
    expect(menuItem("show-all")).toHaveAccessibleDescription("All eligible entities are already shown.");
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(target).toHaveFocus();
  });

  it("queues the inspector's support removal intent", async () => {
    const { model } = await loadBundledMechanicsReference();
    const support = model.supports[0];
    const before = structuredClone(model);
    const { onQueueIntent } = mount(model, { type: "support", id: support.id });
    fireEvent.contextMenu(label(support.id));
    fireEvent.click(menuItem("queue-removal"));
    expect(onQueueIntent).toHaveBeenCalledExactlyOnceWith(buildDeleteSupportIntent(support, model));
    expect(model).toEqual(before);
  });
});
