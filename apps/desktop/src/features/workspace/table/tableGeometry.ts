import { useLayoutEffect, useState, type RefObject } from "react";

/** Resolve CSS-style minmax(minimum, weight fr) tracks once for both grids. */
export function resolveTableTracks(minimums: readonly number[], weights: readonly number[], available: number): number[] {
  const tracks = [...minimums], pending = new Set(minimums.map((_, i) => i));
  let remaining = Math.max(available, minimums.reduce((sum, value) => sum + value, 0));
  while (pending.size) {
    const unit = remaining / [...pending].reduce((sum, i) => sum + weights[i], 0);
    const fixed = [...pending].filter((i) => minimums[i] > unit * weights[i]);
    if (!fixed.length) { for (const i of pending) tracks[i] = unit * weights[i]; break; }
    for (const i of fixed) { remaining -= minimums[i]; pending.delete(i); }
  }
  return tracks;
}

export function useTableGeometry(root: RefObject<HTMLDivElement | null>, minimums: readonly number[], weights: readonly number[], compact: boolean, active: boolean, revision: string) {
  const [geometry, setGeometry] = useState({ clientWidth: 0, gutter: 0, visible: false, minimums: [...minimums] });
  const basis = JSON.stringify([minimums, weights]);
  useLayoutEffect(() => {
    if (!active) { setGeometry((old) => old.visible ? { ...old, visible: false } : old); return; }
    const host = root.current; if (!host) return;
    const list = host.querySelector<HTMLElement>('[role="rowgroup"]'); if (!list) return;
    function measure() {
      if (!host!.isConnected || !list!.clientWidth || host!.closest("[hidden], [inert]") || !host!.getClientRects().length) {
        setGeometry((old) => old.visible ? { ...old, visible: false } : old); return;
      }
      const headings = [...host!.querySelectorAll<HTMLElement>('.engineering-table-header [role="columnheader"]')];
      const measuredMinimums = minimums.map((minimum, i) => {
        if (!compact || !headings[i]) return minimum;
        const element = headings[i].querySelector("button") ?? headings[i];
        const range = document.createRange(); range.selectNodeContents(element);
        const style = getComputedStyle(element);
        return Math.max(minimum, Math.ceil((range.getBoundingClientRect?.().width ?? 0) + parseFloat(style.paddingLeft || "0") + parseFloat(style.paddingRight || "0") + 2));
      });
      const next = { clientWidth: list!.clientWidth, gutter: list!.offsetWidth - list!.clientWidth, visible: true, minimums: measuredMinimums };
      setGeometry((old) => JSON.stringify(old) === JSON.stringify(next) ? old : next);
    }
    measure();
    const resize = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(measure);
    resize?.observe(host); resize?.observe(list);
    const mutation = typeof MutationObserver === "undefined" ? null : new MutationObserver(measure);
    for (let node: HTMLElement | null = host; node; node = node.parentElement) mutation?.observe(node, { attributes: true, attributeFilter: ["hidden", "inert", "class", "style"] });
    return () => { resize?.disconnect(); mutation?.disconnect(); };
  }, [root, active, compact, basis, revision]);
  return { ...geometry, tracks: resolveTableTracks(geometry.minimums, weights, geometry.clientWidth) };
}
