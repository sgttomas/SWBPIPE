import { useId, useLayoutEffect, useRef, useState, type CSSProperties, type KeyboardEvent, type RefObject } from "react";
import { createPortal } from "react-dom";

/** Optional completion on the table's existing input. Navigation never changes
 * its text; only typing, Enter/Tab completion or an explicit option click does. */
export function useEnumEditor({ input, options, token, source, active, text, pending, initialTyped, onChange }: {
  input: RefObject<HTMLInputElement | null>; options?: readonly string[]; token?: number;
  source: string; initialTyped: boolean; active: boolean; text: string; pending: boolean; onChange: (text: string) => void;
}) {
  const id = useId();
  const basis = JSON.stringify([token, source, options]);
  const [opening, setOpening] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [placement, setPlacement] = useState<CSSProperties>({});
  const popup = useRef<HTMLDivElement>(null);
  const pressed = useRef<string | null>(null);
  const typing = useRef({ token, typed: initialTyped });
  if (typing.current.token !== token) typing.current = { token, typed: initialTyped };
  const latest = useRef({ basis, active, pending, options, onChange });
  latest.current = { basis, active, pending, options, onChange };
  const open = opening === basis && active && !pending && Boolean(options);
  const matches = (options ?? []).filter((option) => !text.trim() || option.startsWith(text.trim()));
  // An unsupported source still offers an explicit way to choose a supported token.
  const visible = matches.length ? matches : options ?? [];
  const current = visible.includes(selected ?? "") ? selected : null;
  function unavailable() {
    if (!latest.current.active || latest.current.pending || !input.current?.isConnected) return true;
    for (let node: HTMLElement | null = input.current; node; node = node.parentElement) {
      if (node.hidden || node.hasAttribute("inert") || node.getAttribute("aria-hidden") === "true") return true;
      const style = getComputedStyle(node);
      if (style.display === "none" || style.visibility === "hidden" || style.visibility === "collapse") return true;
    }
    return false;
  }
  function close() { setOpening(null); setSelected(null); pressed.current = null; }
  function show() { if (options && !unavailable()) { setOpening(basis); setSelected(null); } }
  function choose(value: string) {
    if (opening !== latest.current.basis || document.activeElement !== input.current || unavailable() || !latest.current.options?.includes(value)) return;
    latest.current.onChange(value); close();
  }
  useLayoutEffect(() => {
    if (!active || pending || (opening !== null && opening !== basis)) close();
  }, [basis, active, pending, opening]);
  useLayoutEffect(() => {
    if (!open) return;
    function position() {
      if (unavailable()) { close(); return; }
      const rect = input.current!.getBoundingClientRect();
      const width = Math.min(Math.max(rect.width, 120), window.innerWidth - 16);
      const below = window.innerHeight - rect.bottom - 8;
      setPlacement({ position: "fixed", left: Math.max(8, Math.min(rect.left, window.innerWidth - width - 8)), width,
        maxHeight: Math.max(0, Math.min(160, below >= 80 ? below : rect.top - 8)), overflowY: "auto",
        ...(below >= 80 ? { top: rect.bottom } : { bottom: window.innerHeight - rect.top }) });
    }
    function outside(event: Event) {
      if (event.target instanceof Node && !popup.current?.contains(event.target) && event.target !== input.current) close();
    }
    position();
    const observer = new MutationObserver(position);
    for (let node: HTMLElement | null = input.current; node; node = node.parentElement) observer.observe(node, { attributes: true, attributeFilter: ["hidden", "inert", "aria-hidden", "class", "style"] });
    const detached = new MutationObserver(() => { if (!input.current?.isConnected) close(); });
    detached.observe(document.body, { childList: true, subtree: true });
    document.addEventListener("pointerdown", outside, true); document.addEventListener("focusin", outside);
    window.addEventListener("blur", close); window.addEventListener("resize", position); window.addEventListener("scroll", position, true);
    return () => { observer.disconnect(); detached.disconnect(); document.removeEventListener("pointerdown", outside, true); document.removeEventListener("focusin", outside);
      window.removeEventListener("blur", close); window.removeEventListener("resize", position); window.removeEventListener("scroll", position, true); };
  }, [open, basis]);
  function keyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (!options || pending || event.nativeEvent.isComposing || event.ctrlKey || event.metaKey || event.altKey || event.isDefaultPrevented()) return;
    if (event.key === "Escape" && open) { event.preventDefault(); event.stopPropagation(); close(); return; }
    if (event.key === "ArrowDown" || event.key === "ArrowUp") {
      event.preventDefault(); event.stopPropagation();
      if (!open) show();
      else { const index = visible.indexOf(current ?? ""); setSelected(visible[Math.max(0, Math.min(visible.length - 1, index + (event.key === "ArrowDown" ? 1 : -1)))] ?? null); }
      return;
    }
    if (open && (event.key === "Enter" || event.key === "Tab")) {
      const completion = current ?? (typing.current.typed && text.trim() && matches.length === 1 ? matches[0] : undefined);
      if (completion) choose(completion);
      else close();
      // The table owns exactly one Apply/Keep and navigation after completion.
    }
  }
  return { show, close, keyDown, typed: () => { typing.current.typed = true; show(); },
    attributes: options ? { role: "combobox", "aria-autocomplete": "list" as const, "aria-expanded": open,
      "aria-controls": open ? id : undefined, "aria-activedescendant": open && current ? `${id}-${visible.indexOf(current)}` : undefined } : {},
    popup: open ? createPortal(<div ref={popup} id={id} role="listbox" aria-label="Supported values" className="engineering-table-enum-popup" style={placement}
      onPointerDown={(event) => { event.preventDefault(); event.stopPropagation(); }}>
      {visible.map((option, index) => <div key={option} id={`${id}-${index}`} role="option" aria-selected={current === option}
        onPointerDown={() => { pressed.current = option; }} onPointerCancel={() => { pressed.current = null; }}
        onPointerLeave={() => { pressed.current = null; }} onClick={(event) => { if (event.detail === 0 || pressed.current === option) choose(option); pressed.current = null; }}>{option}</div>)}
    </div>, document.body) : null
  };
}
