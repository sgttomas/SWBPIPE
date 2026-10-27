import { afterEach, beforeEach } from "vitest";
import { setSurfaceVisibilityProbeForTests } from "../features/workspace/surfaceReveal";

/**
 * jsdom does no layout, so the browser's visibility probe says nothing is shown. A test that
 * depends on what is shown (a reveal, or where a closing context menu puts focus) models the
 * browser instead: attached, not under [hidden] or [inert], and not in a pane the shell's
 * stylesheet hides (`.inactive` panes and sections are display: none). Chromium itself is
 * covered by the review probes.
 */
export const modelledVisibility = (element: Element) => element.isConnected && !element.closest("[hidden], [inert], .inactive");

export function useModelledVisibility(): void {
  beforeEach(() => setSurfaceVisibilityProbeForTests(modelledVisibility));
  afterEach(() => setSurfaceVisibilityProbeForTests(null));
}
