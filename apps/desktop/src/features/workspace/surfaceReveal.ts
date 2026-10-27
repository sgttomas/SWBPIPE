// Whether a list surface can reveal a row now (B8). The tree, the bulk grid and the engineering
// tables keep a reveal asked for while they cannot show it, and do it once they can: a list
// under [hidden] or [inert], or one the stylesheet does not lay out (display: none, as the
// shell's inactive panes are), cannot scroll a row into view. One rule for the three surfaces,
// so that they cannot drift apart.

export type SurfaceVisibilityProbe = (element: Element) => boolean;

type CheckVisibility = (options?: Record<string, boolean>) => boolean;

/**
 * The browser's own answer: attached, not under [hidden] or [inert], and laid out. Where
 * `checkVisibility` exists it decides (it sees display: none and content-visibility on any
 * ancestor); otherwise an element with no client rectangles is not laid out.
 */
export function layoutVisibility(element: Element): boolean {
  if (!element.isConnected || element.closest("[hidden], [inert]")) return false;
  const check = (element as Element & { checkVisibility?: CheckVisibility }).checkVisibility;
  if (typeof check === "function") return check.call(element);
  return element.getClientRects().length > 0;
}

let visibilityProbe: SurfaceVisibilityProbe = layoutVisibility;

/** True when `host` exists and can show a revealed row now. */
export function canSurfaceRevealNow(host: Element | null | undefined): host is Element {
  return Boolean(host) && visibilityProbe(host as Element);
}

/**
 * Replaces the visibility probe; null restores the browser's. For tests only: jsdom does no
 * layout, so it reports no client rectangles and has no `checkVisibility`, and a test states the
 * visibility it means to model. The browser behaviour is covered by the review's Chromium probe.
 */
export function setSurfaceVisibilityProbeForTests(probe: SurfaceVisibilityProbe | null): void {
  visibilityProbe = probe ?? layoutVisibility;
}
