import { useEffect, useId, useLayoutEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import { createPortal, flushSync } from "react-dom";
import { canSurfaceRevealNow } from "../surfaceReveal";

// B9: one context menu for the viewport, the tree and the grids. It lists commands that
// already exist elsewhere (the menu bar, a toolbar, the inspector, a shortcut); each entry
// runs that command's own handler. WAI-ARIA menu semantics: role="menu" with menuitems, the
// first item focused on open, Arrow keys, Home and End to move, Enter or Space to act,
// Escape to close, Tab and Shift+Tab to close and move to the next or previous tab stop. A
// disabled item stays focusable (aria-disabled) and states why. An item runs first; focus is
// then recovered only if the command left it nowhere (see `recoverMenuFocus`).

export type ContextMenuCommand = Readonly<{
  kind: "command";
  id: string;
  label: string;
  /** The shortcut that runs the same command without the menu, where one exists. */
  shortcut?: string;
  disabled?: boolean;
  reason?: string | null;
  run: () => void;
}>;
export type ContextMenuEntry = ContextMenuCommand | Readonly<{ kind: "separator"; id: string }>;

export type ContextMenuAnchor = Readonly<{ x: number; y: number }>;
/**
 * Where the menu opens, which element takes focus back when it closes, and, for when that
 * element is gone or no longer shown, the surface's current equivalents and stable places
 * (the same entity's row or label, the tree, the table footer, the canvas), in order.
 */
export type ContextMenuRequest = Readonly<{
  anchor: ContextMenuAnchor;
  returnFocus: HTMLElement | null;
  fallbacks?: () => ReadonlyArray<HTMLElement | null | undefined>;
}>;

/** Shift+F10 or the ContextMenu key: the keyboard's way to open a context menu. */
export function isContextMenuKey(event: Pick<KeyboardEvent, "key" | "shiftKey" | "ctrlKey" | "metaKey" | "altKey">): boolean {
  if (event.ctrlKey || event.metaKey || event.altKey) return false;
  return event.key === "ContextMenu" || (event.key === "F10" && event.shiftKey);
}

/** A keyboard-opened menu sits at the invoking element's lower-left corner. */
export function keyboardMenuRequest(element: HTMLElement, returnFocus: HTMLElement | null = element, fallbacks?: ContextMenuRequest["fallbacks"]): ContextMenuRequest {
  const rect = element.getBoundingClientRect();
  return { anchor: { x: rect.left, y: rect.bottom }, returnFocus, fallbacks };
}

export function pointerMenuRequest(event: { clientX: number; clientY: number }, returnFocus: HTMLElement | null, fallbacks?: ContextMenuRequest["fallbacks"]): ContextMenuRequest {
  return { anchor: { x: event.clientX, y: event.clientY }, returnFocus, fallbacks };
}

const TABBABLE = 'a[href], button, input, select, textarea, summary, [tabindex], [contenteditable]:not([contenteditable="false"])';

/** Shown, connected and able to take focus. */
function focusable(element: Element | null | undefined): element is HTMLElement {
  return element instanceof HTMLElement && canSurfaceRevealNow(element) && !(element as HTMLButtonElement).disabled;
}

function tryFocus(element: HTMLElement): boolean {
  element.focus();
  return document.activeElement === element;
}

/** The candidates in order: the invoker, then the surface's equivalents and stable places. */
function candidates(request: ContextMenuRequest): HTMLElement[] {
  return [request.returnFocus, ...(request.fallbacks?.() ?? [])].filter((element): element is HTMLElement => element instanceof HTMLElement);
}

/**
 * Puts focus back after the menu closed and its command ran. A command that placed focus
 * itself (an editor) keeps it. Otherwise focus goes to the first candidate still connected
 * and shown. When none is (the command hid the whole surface, as queuing a removal opens the
 * Pending changes page in the tree's pane), it goes to the first shown focusable element in
 * the nearest shown ancestor of the surface: the page that replaced it.
 */
export function recoverMenuFocus(request: ContextMenuRequest): HTMLElement | null {
  const active = document.activeElement;
  if (active instanceof HTMLElement && active !== document.body && active.isConnected) return active;
  const list = candidates(request);
  for (const element of list) if (focusable(element) && tryFocus(element)) return element;
  const start = list.find((element) => element.isConnected);
  for (let ancestor = start?.parentElement ?? null; ancestor; ancestor = ancestor.parentElement) {
    if (!canSurfaceRevealNow(ancestor)) continue;
    if (ancestor.hasAttribute("tabindex") && tryFocus(ancestor)) return ancestor;
    for (const element of ancestor.querySelectorAll<HTMLElement>("[tabindex]")) {
      if (focusable(element) && tryFocus(element)) return element;
    }
  }
  return null;
}

/** The tab stop after (or before) the invoker, as Tab would have moved from it. */
export function adjacentTabStop(reference: HTMLElement, backward: boolean, exclude: Element | null): HTMLElement | null {
  const stops = [...document.querySelectorAll<HTMLElement>(TABBABLE)].filter((element) =>
    element.tabIndex >= 0 && focusable(element) && !exclude?.contains(element) && element !== reference && !reference.contains(element)
  );
  const follows = (element: HTMLElement) => Boolean(reference.compareDocumentPosition(element) & Node.DOCUMENT_POSITION_FOLLOWING);
  return backward ? [...stops].reverse().find((element) => !follows(element)) ?? null : stops.find(follows) ?? null;
}

const VIEWPORT_MARGIN = 4;

export function ContextMenu({ label, entries, request, onClose, testId = "context-menu" }: {
  label: string;
  entries: readonly ContextMenuEntry[];
  request: ContextMenuRequest;
  onClose: () => void;
  testId?: string;
}) {
  const menuRef = useRef<HTMLDivElement>(null);
  const reasonPrefix = useId();
  const commands = entries.filter((entry): entry is ContextMenuCommand => entry.kind === "command");
  const [activeId, setActiveId] = useState<string | null>(() => (commands.find((command) => !command.disabled) ?? commands[0])?.id ?? null);
  const [position, setPosition] = useState(request.anchor);
  const closed = useRef(false);
  const latest = useRef({ onClose, request });
  latest.current = { onClose, request };

  /** Removes the menu now (and runs `then` in the same commit), so that focus can be judged after. */
  function dismiss(then?: () => void) {
    if (closed.current) return;
    closed.current = true;
    flushSync(() => {
      latest.current.onClose();
      then?.();
    });
  }

  function close(restoreFocus: boolean) {
    if (closed.current) return;
    dismiss();
    if (restoreFocus) recoverMenuFocus(latest.current.request);
  }

  function tabAway(backward: boolean) {
    const request = latest.current.request;
    const menu = menuRef.current;
    dismiss();
    const reference = candidates(request).find((element) => element.isConnected) ?? null;
    const next = reference ? adjacentTabStop(reference, backward, menu) : null;
    if (!next || !tryFocus(next)) recoverMenuFocus(request);
  }

  // Keep the menu inside the window.
  useLayoutEffect(() => {
    const menu = menuRef.current;
    if (!menu) return;
    const rect = menu.getBoundingClientRect();
    const x = Math.max(VIEWPORT_MARGIN, Math.min(request.anchor.x, window.innerWidth - rect.width - VIEWPORT_MARGIN));
    const y = Math.max(VIEWPORT_MARGIN, Math.min(request.anchor.y, window.innerHeight - rect.height - VIEWPORT_MARGIN));
    setPosition((current) => current.x === x && current.y === y ? current : { x, y });
  }, [request.anchor.x, request.anchor.y]);

  // The active item holds DOM focus.
  const currentActive = commands.some((command) => command.id === activeId) ? activeId : commands[0]?.id ?? null;
  useLayoutEffect(() => {
    const menu = menuRef.current;
    if (!menu) return;
    const item = [...menu.querySelectorAll<HTMLElement>("[data-menu-item-id]")].find((candidate) => candidate.dataset.menuItemId === currentActive);
    (item ?? menu).focus();
  }, [currentActive]);

  // A pointer press outside, a resize or the window losing focus closes it; focus stays where
  // the user put it.
  useEffect(() => {
    const outside = (event: Event) => {
      if (event.target instanceof Node && menuRef.current?.contains(event.target)) return;
      close(false);
    };
    const leave = () => close(false);
    document.addEventListener("pointerdown", outside, true);
    window.addEventListener("resize", leave);
    window.addEventListener("blur", leave);
    return () => {
      document.removeEventListener("pointerdown", outside, true);
      window.removeEventListener("resize", leave);
      window.removeEventListener("blur", leave);
    };
  }, []);

  function activate(command: ContextMenuCommand) {
    if (command.disabled || closed.current) return;
    // Close and run in one commit, then recover focus only if the command left it nowhere:
    // a command that moves focus (an editor) keeps it, and one that hides or removes the
    // invoker sends it to the surface's equivalent or a stable place.
    dismiss(() => command.run());
    recoverMenuFocus(latest.current.request);
  }

  function move(step: number | "first" | "last") {
    if (commands.length === 0) return;
    const index = Math.max(0, commands.findIndex((command) => command.id === currentActive));
    const next = step === "first" ? 0 : step === "last" ? commands.length - 1 : (index + step + commands.length) % commands.length;
    setActiveId(commands[next].id);
  }

  function handleKeyDown(event: ReactKeyboardEvent<HTMLDivElement>) {
    // The menu owns every key while open: none reaches the tree, the grid or the viewport.
    event.stopPropagation();
    const command = commands.find((candidate) => candidate.id === currentActive);
    if (event.key === "ArrowDown") move(1);
    else if (event.key === "ArrowUp") move(-1);
    else if (event.key === "Home") move("first");
    else if (event.key === "End") move("last");
    else if (event.key === "Enter" || event.key === " ") { if (command) activate(command); }
    else if (event.key === "Tab") tabAway(event.shiftKey);
    else if (event.key === "Escape" || isContextMenuKey(event)) close(true);
    else if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      // Type-ahead: the next item whose label starts with the typed character.
      const start = Math.max(0, commands.findIndex((candidate) => candidate.id === currentActive));
      const character = event.key.toLowerCase();
      const ordered = [...commands.slice(start + 1), ...commands.slice(0, start + 1)];
      const match = ordered.find((candidate) => candidate.label.toLowerCase().startsWith(character));
      if (match) setActiveId(match.id);
    } else return;
    event.preventDefault();
  }

  const container = request.returnFocus?.closest<HTMLElement>(".app-shell") ?? document.body;
  return createPortal(
    <div
      ref={menuRef}
      role="menu"
      aria-label={label}
      aria-orientation="vertical"
      className="context-menu"
      data-testid={testId}
      tabIndex={-1}
      style={{ left: position.x, top: position.y }}
      onKeyDown={handleKeyDown}
      onContextMenu={(event) => { event.preventDefault(); event.stopPropagation(); }}
      onPointerDown={(event) => event.stopPropagation()}
      onPointerUp={(event) => event.stopPropagation()}
      onClick={(event) => event.stopPropagation()}
      onDoubleClick={(event) => event.stopPropagation()}
    >
      {entries.map((entry) => entry.kind === "separator" ? (
        <div key={entry.id} role="separator" className="context-menu-separator" />
      ) : (
        <div
          key={entry.id}
          role="menuitem"
          tabIndex={-1}
          className={`context-menu-item${entry.id === currentActive ? " active" : ""}`}
          data-menu-item-id={entry.id}
          data-testid={`${testId}-item-${entry.id}`}
          aria-disabled={entry.disabled ? true : undefined}
          aria-keyshortcuts={entry.shortcut}
          aria-describedby={entry.disabled && entry.reason ? `${reasonPrefix}-${entry.id}` : undefined}
          title={entry.disabled && entry.reason ? entry.reason : undefined}
          onPointerMove={() => { if (entry.id !== currentActive) setActiveId(entry.id); }}
          onClick={() => activate(entry)}
        >
          <span className="context-menu-label">{entry.label}</span>
          {entry.shortcut ? <kbd className="context-menu-shortcut" aria-hidden="true">{entry.shortcut}</kbd> : null}
          {entry.disabled && entry.reason ? <small id={`${reasonPrefix}-${entry.id}`} className="context-menu-reason">{entry.reason}</small> : null}
        </div>
      ))}
    </div>,
    container
  );
}
