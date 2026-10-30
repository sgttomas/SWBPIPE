import {
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type AriaRole,
  type KeyboardEventHandler,
  type ReactNode
} from "react";

type Props<T> = Readonly<{
  items: readonly T[];
  itemKey: (item: T, index: number) => string;
  renderItem: (item: T, index: number) => ReactNode;
  height: number;
  rowHeight: number;
  activeIndex?: number | null;
  revealActiveRequest?: number;
  pinIndex?: number | null;
  overscan?: number;
  threshold?: number;
  ariaLabel?: string;
  ariaActiveDescendant?: string;
  ariaMultiselectable?: boolean;
  id?: string;
  className?: string;
  role?: AriaRole;
  testId?: string;
  tabIndex?: number;
  onKeyDown?: KeyboardEventHandler<HTMLDivElement>;
}>;

export function VirtualList<T>({
  items,
  itemKey,
  renderItem,
  height,
  rowHeight,
  activeIndex = null,
  revealActiveRequest,
  pinIndex = null,
  overscan = 8,
  threshold = 100,
  ariaLabel,
  ariaActiveDescendant,
  ariaMultiselectable,
  id,
  className,
  role,
  testId,
  tabIndex,
  onKeyDown
}: Props<T>) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const lastRevealRequestRef = useRef(revealActiveRequest);
  const [scrollTop, setScrollTop] = useState(0);
  const virtual = items.length >= threshold;
  const window = useMemo(() => {
    if (!virtual) return { start: 0, end: items.length };
    const count = Math.ceil(height / rowHeight) + overscan * 2;
    const requestedStart = Math.max(0, Math.floor(scrollTop / rowHeight) - overscan);
    const start = Math.min(requestedStart, Math.max(0, items.length - count));
    return { start, end: Math.min(items.length, start + count) };
  }, [height, items.length, overscan, rowHeight, scrollTop, virtual]);
  const indexes = useMemo(() => {
    const values = Array.from({ length: window.end - window.start }, (_, offset) => window.start + offset);
    if (pinIndex !== null && pinIndex >= 0 && pinIndex < items.length && !values.includes(pinIndex)) {
      values.push(pinIndex);
      values.sort((left, right) => left - right);
    }
    return values;
  }, [items.length, pinIndex, window.end, window.start]);

  useLayoutEffect(() => {
    if (activeIndex === null || activeIndex < 0 || activeIndex >= items.length) return;
    const host = hostRef.current;
    if (!host) return;
    const top = activeIndex * rowHeight;
    const bottom = top + rowHeight;
    if (top < host.scrollTop) host.scrollTop = top;
    else if (bottom > host.scrollTop + height) host.scrollTop = bottom - height;
    setScrollTop(host.scrollTop);
  }, [activeIndex, height, items.length, rowHeight]);

  useLayoutEffect(() => {
    if (revealActiveRequest === undefined || revealActiveRequest === lastRevealRequestRef.current) return;
    if (activeIndex === null || activeIndex < 0 || activeIndex >= items.length) {
      lastRevealRequestRef.current = revealActiveRequest;
      return;
    }
    const host = hostRef.current;
    const activeRow = host?.querySelector<HTMLElement>(`[data-virtual-index="${activeIndex}"]`);
    // A virtual row may mount only after the inner scroll updates the window.
    // Leave the request pending until that row exists, then reveal it through
    // every clipping scroll ancestor without moving keyboard focus.
    if (!activeRow) return;
    activeRow.scrollIntoView?.({ block: "nearest", inline: "nearest" });
    lastRevealRequestRef.current = revealActiveRequest;
  }, [activeIndex, indexes, items.length, revealActiveRequest]);

  useLayoutEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    const maximum = Math.max(0, items.length * rowHeight - height);
    if (host.scrollTop <= maximum) return;
    host.scrollTop = maximum;
    setScrollTop(maximum);
  }, [height, items.length, rowHeight]);

  const hostStyle: CSSProperties = { height, overflowY: items.length * rowHeight > height ? "auto" : "hidden" };
  return (
    <div
      aria-label={ariaLabel}
      aria-activedescendant={ariaActiveDescendant}
      aria-multiselectable={ariaMultiselectable}
      className={className}
      data-testid={testId}
      id={id}
      onKeyDown={onKeyDown}
      onScroll={(event) => setScrollTop(event.currentTarget.scrollTop)}
      ref={hostRef}
      role={role}
      style={hostStyle}
      tabIndex={tabIndex}
    >
      <div style={virtual ? { height: items.length * rowHeight, position: "relative" } : undefined}>
        {indexes.map((index) => (
          <div
            data-virtual-index={index}
            key={itemKey(items[index], index)}
            style={virtual ? {
              height: rowHeight,
              left: 0,
              position: "absolute",
              right: 0,
              top: index * rowHeight
            } : { boxSizing: "border-box", height: rowHeight, overflow: "hidden" }}
          >
            {renderItem(items[index], index)}
          </div>
        ))}
      </div>
    </div>
  );
}
