import { useEffect, useRef, useState } from "react";

/**
 * How many times a page has been shown again after it was hidden: 0 while it
 * has not yet been shown, and on the first showing, so a page read on mount is
 * not read twice. A retained page that sleeps while hidden takes the count as a
 * prop, so a new count reaches it when it is shown again.
 */
export function useReturnCount(shown: boolean): number {
  const [returns, setReturns] = useState(0);
  const wasShown = useRef(shown);
  const everShown = useRef(shown);
  useEffect(() => {
    if (shown && !wasShown.current && everShown.current) setReturns((count) => count + 1);
    wasShown.current = shown;
    if (shown) everShown.current = true;
  }, [shown]);
  return returns;
}
