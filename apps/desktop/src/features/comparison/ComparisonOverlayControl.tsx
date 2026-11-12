import { useEffect, useState } from "react";
import type { ComparisonRowRequest } from "../history/useModelStateHistory";

// The comparison views' part of the comparison overlay (DEL-07-08 REQ-011): the control that turns
// the overlay on and off, operable by keyboard as a button, and the handling of the overlay's
// request to show an entity's rows in the table.

export type ComparisonOverlayLink = {
  /** The overlay is on for this comparison. */
  shown: boolean;
  /** Why this comparison cannot be drawn on the model, or null when it can. */
  unavailable: string | null;
  onShow: () => void;
  onHide: () => void;
  /** The overlay's last request for an entity's rows, and how the view reports it spent. */
  rowRequest: ComparisonRowRequest | null;
  onRowRequestHandled: (serial: number) => void;
};

export function ComparisonOverlayControl({ link, testId }: { link: ComparisonOverlayLink; testId: string }) {
  const disabled = !link.shown && link.unavailable !== null;
  return (
    <button
      type="button"
      data-testid={testId}
      aria-pressed={link.shown}
      disabled={disabled}
      title={disabled ? link.unavailable ?? undefined : link.shown ? "The comparison is shown as a schematic in the viewport pane; press to leave it." : "Show this comparison as a read-only schematic in the viewport pane."}
      onClick={link.shown ? link.onHide : link.onShow}
    >
      Show on the model
    </button>
  );
}

/**
 * Applies the overlay's row request to a view: when the request names this comparison (`ids`) and
 * one of its rows, `reveal` shows that row (clearing the filters and turning to its page) and returns
 * its key; the row with that key is then focused. The request is reported spent either way.
 */
export function useComparisonRowRequest(
  link: ComparisonOverlayLink | undefined,
  ids: readonly [string, string],
  reveal: (request: ComparisonRowRequest) => string | null
): string | null {
  const [focusedKey, setFocusedKey] = useState<string | null>(null);
  const [focusSerial, setFocusSerial] = useState(0);
  const request = link?.rowRequest ?? null;
  useEffect(() => {
    if (!link || !request) return;
    if (request.ids[0] !== ids[0] || request.ids[1] !== ids[1]) return;
    setFocusedKey(reveal(request));
    setFocusSerial(request.serial);
    link.onRowRequestHandled(request.serial);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [request?.serial]);
  useEffect(() => {
    if (!focusSerial || !focusedKey) return;
    const row = Array.from(document.querySelectorAll<HTMLElement>("[data-overlay-row-key]")).find((element) => element.dataset.overlayRowKey === focusedKey);
    row?.focus();
    row?.scrollIntoView?.({ block: "nearest" });
  }, [focusSerial, focusedKey]);
  return focusedKey;
}
