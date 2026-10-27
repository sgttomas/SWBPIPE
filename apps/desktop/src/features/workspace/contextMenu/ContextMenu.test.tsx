// B9: the shared context menu's WAI-ARIA menu behaviour: opening from the keyboard, Arrow,
// Home, End, Enter, Space, Escape and Tab, type-ahead, disabled items with their reasons,
// focus returning to the invoking element, and focus recovery when a command hides or removes
// that element.
import { fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { ContextMenu, isContextMenuKey, keyboardMenuRequest, type ContextMenuEntry, type ContextMenuRequest } from "./ContextMenu";
import { useModelledVisibility } from "../../../test-support/modelledVisibility";

useModelledVisibility();

function Harness({ entries }: { entries: readonly ContextMenuEntry[] }) {
  const [request, setRequest] = useState<ContextMenuRequest | null>(null);
  return (
    <>
      <button type="button" data-testid="before">Before</button>
      <button type="button" data-testid="invoker" onKeyDown={(event) => {
        if (!isContextMenuKey(event)) return;
        event.preventDefault();
        setRequest(keyboardMenuRequest(event.currentTarget));
      }}>Invoker</button>
      <button type="button" data-testid="elsewhere">Elsewhere</button>
      {request ? <ContextMenu label="Commands for node N1" entries={entries} request={request} onClose={() => setRequest(null)} /> : null}
    </>
  );
}

function entries(run = vi.fn()) {
  const list: ContextMenuEntry[] = [
    { kind: "command", id: "hide", label: "Hide selected", shortcut: "H", run: () => run("hide") },
    { kind: "command", id: "isolate", label: "Isolate selected", disabled: true, reason: "Nothing drawable is selected.", run: () => run("isolate") },
    { kind: "separator", id: "sep" },
    { kind: "command", id: "fit", label: "Fit selection", run: () => run("fit") },
    { kind: "command", id: "show", label: "Show all", run: () => run("show") }
  ];
  return { list, run };
}

function open(key: { key: string; shiftKey?: boolean } = { key: "F10", shiftKey: true }) {
  const invoker = screen.getByTestId("invoker");
  invoker.focus();
  fireEvent.keyDown(invoker, key);
  return invoker;
}

const item = (id: string) => screen.getByTestId(`context-menu-item-${id}`);

describe("the context menu", () => {
  it("recognises Shift+F10 and the ContextMenu key only", () => {
    const base = { shiftKey: false, ctrlKey: false, metaKey: false, altKey: false };
    expect(isContextMenuKey({ ...base, key: "F10", shiftKey: true })).toBe(true);
    expect(isContextMenuKey({ ...base, key: "ContextMenu" })).toBe(true);
    expect(isContextMenuKey({ ...base, key: "F10" })).toBe(false);
    expect(isContextMenuKey({ ...base, key: "F10", shiftKey: true, ctrlKey: true })).toBe(false);
    expect(isContextMenuKey({ ...base, key: "ContextMenu", altKey: true })).toBe(false);
  });

  it("opens from the keyboard with menu semantics and focuses the first enabled item", () => {
    render(<Harness entries={entries().list} />);
    open({ key: "ContextMenu" });
    const menu = screen.getByRole("menu", { name: "Commands for node N1" });
    expect(screen.getAllByRole("menuitem").map((element) => element.textContent)).toEqual([
      "Hide selectedH", "Isolate selectedNothing drawable is selected.", "Fit selection", "Show all"
    ]);
    expect(screen.getByRole("separator")).toBeInTheDocument();
    expect(menu).toContainElement(document.activeElement as HTMLElement);
    expect(item("hide")).toHaveFocus();
    expect(item("hide")).toHaveAttribute("aria-keyshortcuts", "H");
  });

  it("moves with the Arrow keys (wrapping), Home and End, and over disabled items", () => {
    render(<Harness entries={entries().list} />);
    open();
    const menu = screen.getByRole("menu");
    fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(item("isolate")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(item("fit")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "End" });
    expect(item("show")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "ArrowDown" });
    expect(item("hide")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "ArrowUp" });
    expect(item("show")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "Home" });
    expect(item("hide")).toHaveFocus();
    fireEvent.keyDown(menu, { key: "s" });
    expect(item("show")).toHaveFocus();
  });

  it("acts on Enter and Space, closes and returns focus to the invoker", () => {
    const { list, run } = entries();
    render(<Harness entries={list} />);
    const invoker = open();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    expect(run).toHaveBeenCalledWith("hide");
    expect(screen.queryByRole("menu")).toBeNull();
    expect(invoker).toHaveFocus();

    open();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "End" });
    fireEvent.keyDown(screen.getByRole("menu"), { key: " " });
    expect(run).toHaveBeenLastCalledWith("show");
    expect(invoker).toHaveFocus();
  });

  it("keeps a disabled item focusable, states its reason and does not run it", () => {
    const { list, run } = entries();
    render(<Harness entries={list} />);
    open();
    const isolate = item("isolate");
    expect(isolate).toHaveAttribute("aria-disabled", "true");
    expect(isolate).toHaveAccessibleDescription("Nothing drawable is selected.");
    fireEvent.keyDown(screen.getByRole("menu"), { key: "ArrowDown" });
    expect(isolate).toHaveFocus();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    fireEvent.click(isolate);
    expect(run).not.toHaveBeenCalled();
    expect(screen.getByRole("menu")).toBeInTheDocument();
  });

  it("closes on Escape with focus back on the invoker, and keeps keys from the surface below", () => {
    const surfaceKeys = vi.fn();
    render(<div onKeyDown={surfaceKeys}><Harness entries={entries().list} /></div>);
    const invoker = open();
    surfaceKeys.mockClear();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "h" });
    fireEvent.keyDown(screen.getByRole("menu"), { key: "ArrowDown" });
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(surfaceKeys).not.toHaveBeenCalled();
    expect(screen.queryByRole("menu")).toBeNull();
    expect(invoker).toHaveFocus();
  });

  it("closes on Tab and moves to the tab stop after the invoker, and on Shift+Tab to the one before", () => {
    const surfaceKeys = vi.fn();
    const { list, run } = entries();
    render(<div onKeyDown={surfaceKeys}><Harness entries={list} /></div>);
    open();
    surfaceKeys.mockClear();
    const tab = fireEvent.keyDown(screen.getByRole("menu"), { key: "Tab" });
    expect(tab).toBe(false);
    expect(screen.queryByRole("menu")).toBeNull();
    expect(screen.getByTestId("elsewhere")).toHaveFocus();
    open();
    surfaceKeys.mockClear();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Tab", shiftKey: true });
    expect(screen.queryByRole("menu")).toBeNull();
    expect(screen.getByTestId("before")).toHaveFocus();
    expect(surfaceKeys).not.toHaveBeenCalled();
    expect(run).not.toHaveBeenCalled();
  });

  it("closes on a pointer press outside without taking focus back", () => {
    render(<Harness entries={entries().list} />);
    open();
    const elsewhere = screen.getByTestId("elsewhere");
    fireEvent.pointerDown(elsewhere);
    elsewhere.focus();
    expect(screen.queryByRole("menu")).toBeNull();
    expect(elsewhere).toHaveFocus();
  });

  it("runs a clicked item", () => {
    const { list, run } = entries();
    render(<Harness entries={list} />);
    open();
    fireEvent.click(item("fit"));
    expect(run).toHaveBeenCalledWith("fit");
    expect(screen.queryByRole("menu")).toBeNull();
  });
});

// A surface that a command can change: the invoker can be hidden or removed, and the surface
// names its equivalents and stable places as the menu request's fallbacks.
function Surface({ effect, fallbacks = true }: { effect: "remove" | "hide" | "hide-surface" | "focus-editor" | "none"; fallbacks?: boolean }) {
  const [request, setRequest] = useState<ContextMenuRequest | null>(null);
  const [state, setState] = useState<"shown" | "removed" | "hidden" | "surface-hidden" | "editing">("shown");
  const run = () => setState(effect === "remove" ? "removed" : effect === "hide" ? "hidden" : effect === "hide-surface" ? "surface-hidden" : effect === "focus-editor" ? "editing" : "shown");
  return (
    <div data-testid="page">
      <div data-testid="surface" className={state === "surface-hidden" ? "inactive" : undefined}>
        {state === "removed" ? null : (
          <button type="button" data-testid="row" hidden={state === "hidden"} onKeyDown={(event) => {
            if (!isContextMenuKey(event)) return;
            event.preventDefault();
            setRequest(keyboardMenuRequest(event.currentTarget, event.currentTarget, fallbacks ? () => [document.querySelector<HTMLElement>('[data-testid="row"]'), document.querySelector<HTMLElement>('[data-testid="container"]')] : undefined));
          }}>Row</button>
        )}
        <div data-testid="container" tabIndex={-1}>Container</div>
        {state === "editing" ? <input aria-label="Editor" autoFocus /> : null}
      </div>
      <section data-testid="replacement" tabIndex={-1}>Replacement page</section>
      {request ? <ContextMenu label="Commands" entries={[{ kind: "command", id: "act", label: "Act", run }]} request={request} onClose={() => setRequest(null)} /> : null}
    </div>
  );
}

function openRowMenu() {
  const row = screen.getByTestId("row");
  row.focus();
  fireEvent.keyDown(row, { key: "F10", shiftKey: true });
  return row;
}

describe("focus after a command changes the invoking surface", () => {
  it("returns to the invoker when it is still shown", () => {
    render(<Surface effect="none" />);
    const row = openRowMenu();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    expect(row).toHaveFocus();
  });

  it.each(["remove", "hide"] as const)("goes to the surface's stable place when the command %ss the invoker", (effect) => {
    render(<Surface effect={effect} />);
    openRowMenu();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    expect(document.activeElement).not.toBe(document.body);
    expect(screen.getByTestId("container")).toHaveFocus();
  });

  it("goes to the shown region that replaced a hidden surface", () => {
    render(<Surface effect="hide-surface" />);
    openRowMenu();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    expect(screen.getByTestId("replacement")).toHaveFocus();
  });

  it("leaves focus where the command put it", () => {
    render(<Surface effect="focus-editor" />);
    openRowMenu();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
    expect(screen.getByRole("textbox", { name: "Editor" })).toHaveFocus();
  });

  it("on Escape after the invoker was removed while the menu was open, goes to the stable place", () => {
    const view = render(<Surface effect="none" />);
    openRowMenu();
    view.getByTestId("row").remove();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    expect(screen.getByTestId("container")).toHaveFocus();
  });
});

// Tab from the menu reaches what the browser's Tab from the invoker would (the review's
// Chromium probes, TAB_ORDER): disabled controls, those in a disabled fieldset, inert and
// CSS-invisible ones are skipped, a radio group is one stop, an iframe is a stop, and the ends
// wrap. Positive tabindex and shadow roots are outside the product's scope (Design).
type Layout = "fieldset" | "radio" | "radio-unchecked" | "visibility" | "inert" | "iframe" | "edge-first" | "edge-last";
function TabPage({ layout }: { layout: Layout }) {
  const [request, setRequest] = useState<ContextMenuRequest | null>(null);
  return (
    <div>
      {layout !== "edge-first" ? <button type="button" id="before">Before</button> : null}
      <button type="button" id="invoker" onKeyDown={(event) => {
        if (!isContextMenuKey(event)) return;
        event.preventDefault();
        setRequest(keyboardMenuRequest(event.currentTarget));
      }}>Invoker</button>
      {layout === "fieldset" ? <fieldset disabled><button type="button" id="inherited">Disabled inherited</button></fieldset> : null}
      {layout === "radio" || layout === "radio-unchecked" ? <>
        <input id="radio-a" type="radio" name="invented-radio" />
        <input id="radio-b" type="radio" name="invented-radio" defaultChecked={layout === "radio"} />
      </> : null}
      {layout === "visibility" ? <button type="button" id="invisible" style={{ visibility: "hidden" }}>Invisible</button> : null}
      {layout === "inert" ? <div inert><button type="button" id="inert-button">Inert</button></div> : null}
      {layout === "iframe" ? <iframe id="frame" title="Invented frame" /> : null}
      {layout !== "edge-last" ? <button type="button" id="after">After</button> : null}
      {request ? <ContextMenu label="Commands" entries={[{ kind: "command", id: "act", label: "Act", run: () => undefined }]} request={request} onClose={() => setRequest(null)} /> : null}
    </div>
  );
}

function tabFromMenu(backward = false) {
  const invoker = document.getElementById("invoker")!;
  invoker.focus();
  fireEvent.keyDown(invoker, { key: "F10", shiftKey: true });
  fireEvent.keyDown(screen.getByRole("menu"), { key: "Tab", shiftKey: backward });
  expect(screen.queryByRole("menu")).toBeNull();
  return (document.activeElement as HTMLElement).id;
}

describe("Tab from the menu follows the browser's tab order", () => {
  it.each([
    ["fieldset", "Tab", "after"],
    ["radio", "Tab", "radio-b"],
    ["radio-unchecked", "Tab", "radio-a"],
    ["visibility", "Tab", "after"],
    ["inert", "Tab", "after"],
    ["iframe", "Tab", "frame"],
    ["edge-last", "Tab", "before"],
    ["edge-first", "Shift+Tab", "after"],
    ["radio", "Shift+Tab", "before"]
  ] as const)("in a %s layout, %s reaches #%s", (layout, key, expected) => {
    render(<TabPage layout={layout} />);
    expect(tabFromMenu(key === "Shift+Tab")).toBe(expected);
  });
});

// Commands that throw, or that put focus somewhere and then hide it (COMMAND_FOCUS), and the
// generic fallback's preference for a place over an action (fallback-negative).
function CommandPage({ effect }: { effect: "throws" | "focus-hidden" | "destructive-neighbour" }) {
  const [request, setRequest] = useState<ContextMenuRequest | null>(null);
  const [hidden, setHidden] = useState(false);
  const run = () => {
    if (effect === "throws") throw new Error("INVENTED_COMMAND_FAILURE");
    if (effect === "focus-hidden") document.getElementById("invoker")?.focus();
    setHidden(true);
  };
  return (
    <div>
      <div className={hidden ? "inactive" : undefined}>
        <button type="button" id="invoker" onKeyDown={(event) => {
          if (!isContextMenuKey(event)) return;
          event.preventDefault();
          setRequest(keyboardMenuRequest(event.currentTarget, event.currentTarget,
            effect === "destructive-neighbour" ? undefined : () => [document.getElementById("stable")]));
        }}>Invoker</button>
        <div id="stable" tabIndex={-1}>Stable place</div>
      </div>
      {effect === "throws" ? null : <>
        <button type="button" id="destructive" tabIndex={-1}>Delete permanently</button>
        <section id="replacement" tabIndex={-1} aria-label="Replacement page">Replacement page</section>
      </>}
      {request ? <ContextMenu label="Commands" entries={[{ kind: "command", id: "act", label: "Act", run }]} request={request} onClose={() => setRequest(null)} /> : null}
    </div>
  );
}

function runFromMenu() {
  const invoker = document.getElementById("invoker")!;
  invoker.focus();
  fireEvent.keyDown(invoker, { key: "F10", shiftKey: true });
  fireEvent.keyDown(screen.getByRole("menu"), { key: "Enter" });
}

describe("focus after unusual commands", () => {
  it("recovers focus when the command throws, and the error still propagates", () => {
    const errors: string[] = [];
    const onError = (event: ErrorEvent) => { errors.push(event.error?.message ?? event.message); event.preventDefault(); };
    window.addEventListener("error", onError);
    try {
      render(<CommandPage effect="throws" />);
      runFromMenu();
    } finally {
      window.removeEventListener("error", onError);
    }
    expect(errors).toEqual(["INVENTED_COMMAND_FAILURE"]);
    expect(screen.queryByRole("menu")).toBeNull();
    expect(document.getElementById("invoker")).toHaveFocus();
  });

  it("does not keep focus a command put on an element it then hid", () => {
    render(<CommandPage effect="focus-hidden" />);
    runFromMenu();
    expect(document.activeElement).not.toBe(document.getElementById("invoker"));
    expect(document.getElementById("replacement")).toHaveFocus();
  });

  it("prefers a page region over a tabindex=-1 button when nothing designated is shown", () => {
    render(<CommandPage effect="destructive-neighbour" />);
    runFromMenu();
    expect(document.getElementById("destructive")).not.toHaveFocus();
    expect(document.getElementById("replacement")).toHaveFocus();
  });
});
