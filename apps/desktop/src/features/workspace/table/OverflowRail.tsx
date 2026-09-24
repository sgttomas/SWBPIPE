import { createContext, useLayoutEffect, useRef, useState, type Dispatch, type ReactNode, type SetStateAction } from "react";

export const TableChromeContext = createContext<{
  target: HTMLElement | null; owner: string | null; setOwner: Dispatch<SetStateAction<string | null>>;
}>({ target: null, owner: null, setOwner: () => {} });

/** Width-only navigation. The rail and its children stay mounted as presentation
 * changes; only its named controls are conditional. No scrollbar consumes height. */
export function OverflowRail({ enabled, name, owner, children }: {
  enabled: boolean; name: "table controls" | "table status"; owner?: string | null; children: ReactNode;
}) {
  const viewport = useRef<HTMLDivElement>(null), content = useRef<HTMLDivElement>(null), pans = useRef<HTMLDivElement>(null);
  const pendingPanFocus = useRef<{ button: HTMLButtonElement; pair: HTMLDivElement; owner: string | undefined } | null>(null);
  const [extent, setExtent] = useState({ left: 0, maximum: 0 });
  function measure() {
    const node = viewport.current;
    if (!enabled || !node || !node.clientWidth) return;
    const next = { left: node.scrollLeft, maximum: Math.max(0, node.scrollWidth - node.clientWidth) };
    setExtent((old) => old.left === next.left && old.maximum === next.maximum ? old : next);
  }
  useLayoutEffect(() => {
    if (!enabled) { setExtent({ left: 0, maximum: 0 }); return; }
    measure();
    const observer = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(measure);
    if (viewport.current) observer?.observe(viewport.current);
    if (content.current) observer?.observe(content.current);
    return () => observer?.disconnect();
  }, [enabled]);
  useLayoutEffect(() => {
    const request = pendingPanFocus.current; pendingPanFocus.current = null;
    const pair = pans.current, active = document.activeElement;
    if (!enabled || !pair?.isConnected || !pair.getClientRects().length || pair.closest("[hidden], [inert]")) return;
    const current = pair.contains(active) && (active as HTMLButtonElement).disabled ? active as HTMLButtonElement : null;
    const captured = request?.pair === pair && request.button.isConnected && pair.contains(request.button) && request.button.dataset.tableChromeOwner === request.owner && request.button.disabled && (active === document.body || active === request.button) ? request.button : null;
    if (current || captured) pair.querySelector<HTMLButtonElement>("button:not(:disabled)")?.focus();
  }, [enabled, extent.left, extent.maximum]);
  function pan(direction: number, button: HTMLButtonElement) {
    pendingPanFocus.current = null;
    const node = viewport.current, pair = pans.current; if (!node || !pair) return;
    const before = node.scrollLeft;
    const next = Math.max(0, Math.min(node.scrollWidth - node.clientWidth, before + direction * Math.max(28, node.clientWidth * .75)));
    if (next === before) return;
    // Some engines blur synchronously as React disables the clicked endpoint.
    // Retain only this activation's focus owner before that DOM mutation.
    if (document.activeElement === button) pendingPanFocus.current = { button, pair, owner: button.dataset.tableChromeOwner };
    node.scrollLeft = next;
    if (node.scrollLeft === before) pendingPanFocus.current = null;
    measure();
  }
  return <div className={`table-overflow-rail${enabled ? " enabled" : ""}`}>
    <div className="table-overflow-viewport" ref={viewport} onScroll={measure}
      onFocusCapture={(event) => {
        if (!enabled) return;
        const node = viewport.current, target = event.target as HTMLElement;
        if (!node || target.closest("[popover]")) return;
        const v = node.getBoundingClientRect(), t = target.getBoundingClientRect();
        if (t.left < v.left) node.scrollLeft += t.left - v.left;
        else if (t.right > v.right) node.scrollLeft += t.right - v.right;
        measure();
      }}>
      <div className="table-overflow-content" ref={content}>{children}</div>
    </div>
    {enabled && extent.maximum > 0 ? <div ref={pans} className="table-pan-pair" role="group" aria-label={`Pan ${name}`}>
      <button type="button" aria-label={`Earlier ${name}`} data-table-chrome-owner={owner ?? undefined} disabled={extent.left <= 0}
        onClick={(event) => pan(-1, event.currentTarget)}>‹</button>
      <button type="button" aria-label={`Later ${name}`} data-table-chrome-owner={owner ?? undefined} disabled={extent.left >= extent.maximum - .5}
        onClick={(event) => pan(1, event.currentTarget)}>›</button>
    </div> : null}
  </div>;
}
