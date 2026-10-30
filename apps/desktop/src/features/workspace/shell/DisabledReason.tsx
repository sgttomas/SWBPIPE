import { useEffect, useRef, useState } from "react";

// One Escape can dismiss a hovered reason and a different focused reason.
// A child-consumed Escape must still be left alone.
const reasonEscapes = new WeakSet<KeyboardEvent>();

/**
 * A disabled control stays focusable and keeps its aria-describedby reason
 * mounted, including after Escape dismisses the visual tooltip.
 */
export function DisabledReason({ id, text }: { id: string; text: string }) {
  const reasonRef = useRef<HTMLSpanElement>(null);
  const [dismissed, setDismissed] = useState(false);

  useEffect(() => {
    const reason = reasonRef.current;
    const anchor = reason?.parentElement;
    if (!reason || !anchor) return;
    const ownerDocument = reason.ownerDocument;
    const reopen = () => setDismissed(false);
    const dismiss = (event: KeyboardEvent) => {
      if (event.key !== "Escape" || (event.defaultPrevented && !reasonEscapes.has(event))) return;
      if (!reason.getClientRects().length || getComputedStyle(reason).visibility !== "visible") return;
      reasonEscapes.add(event);
      event.preventDefault();
      event.stopPropagation();
      setDismissed(true);
    };
    anchor.addEventListener("pointerenter", reopen);
    anchor.addEventListener("focusin", reopen);
    // Bubble after child handlers, before the shell's window Escape handler.
    ownerDocument.addEventListener("keydown", dismiss);
    return () => {
      anchor.removeEventListener("pointerenter", reopen);
      anchor.removeEventListener("focusin", reopen);
      ownerDocument.removeEventListener("keydown", dismiss);
    };
  }, []);

  return <span ref={reasonRef} className="shell-reason" id={id} role="tooltip" data-dismissed={dismissed || undefined}>{text}</span>;
}
