/** Brings a table's current row into view once per request (a governing value opened from the
 * Results stage's summary): after each commit, until the row marked current is laid out, so a row
 * that arrives with a later read (a native lookup, a tab brought forward) is still brought into
 * view. Focus does not move. */
import { useEffect, useRef } from "react";
import { canSurfaceRevealNow } from "../workspace/surfaceReveal";

export function useRevealCurrentRow<T extends HTMLElement>(serial: number | null | undefined) {
  const ref = useRef<T>(null);
  const done = useRef<number | null>(null);
  useEffect(() => {
    if (!serial || done.current === serial) return;
    const row = ref.current?.querySelector<HTMLElement>('[aria-current="true"]');
    if (!row || !canSurfaceRevealNow(row)) return;
    row.scrollIntoView?.({ block: "center" });
    done.current = serial;
  });
  return ref;
}
