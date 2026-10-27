import { useEffect, useId, useLayoutEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import { createPortal } from "react-dom";

// B9: one context menu for the viewport, the tree and the grids. It lists commands that
// already exist elsewhere (the menu bar, a toolbar, the inspector, a shortcut); each entry
// runs that command's own handler. WAI-ARIA menu semantics: role="menu" with menuitems, the
// first item focused on open, Arrow keys, Home and End to move, Enter or Space to act,
// Escape or Tab to close. A disabled item stays focusable (aria-disabled) and states why.
// Focus returns to the invoking element when the menu closes from the keyboard or by an item.

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
/** Where the menu opens and which element takes focus back when it closes. */
export type ContextMenuRequest = Readonly<{ anchor: ContextMenuAnchor; returnFocus: HTMLElement | null }>;

/** Shift+F10 or the ContextMenu key: the keyboard's way to open a context menu. */
export function isContextMenuKey(event: Pick<KeyboardEvent, "key" | "shiftKey" | "ctrlKey" | "metaKey" | "altKey">): boolean {
  if (event.ctrlKey || event.metaKey || event.altKey) return false;
  return event.key === "ContextMenu" || (event.key === "F10" && event.shiftKey);
}

/** A keyboard-opened menu sits at the invoking element's lower-left corner. */
export function keyboardMenuRequest(element: HTMLElement, returnFocus: HTMLElement | null = element): ContextMenuRequest {
  const rect = element.getBoundingClientRect();
  return { anchor: { x: rect.left, y: rect.bottom }, returnFocus };
}

export function pointerMenuRequest(event: { clientX: number; clientY: number }, returnFocus: HTMLElement | null): ContextMenuRequest {
  return { anchor: { x: event.clientX, y: event.clientY }, returnFocus };
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

  function close(restoreFocus: boolean) {
    if (closed.current) return;
    closed.current = true;
    latest.current.onClose();
    const target = latest.current.request.returnFocus;
    if (restoreFocus && target?.isConnected) target.focus();
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
    const dismiss = () => close(false);
    document.addEventListener("pointerdown", outside, true);
    window.addEventListener("resize", dismiss);
    window.addEventListener("blur", dismiss);
    return () => {
      document.removeEventListener("pointerdown", outside, true);
      window.removeEventListener("resize", dismiss);
      window.removeEventListener("blur", dismiss);
    };
  }, []);

  function activate(command: ContextMenuCommand) {
    if (command.disabled) return;
    // Close and restore focus first, so a command that moves focus (an editor) keeps it.
    close(true);
    command.run();
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
    else if (event.key === "Escape" || event.key === "Tab" || isContextMenuKey(event)) close(true);
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
