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
  pinIndex?: number | null;
  overscan?: number;
  threshold?: number;
  ariaLabel?: string;
  ariaActiveDescendant?: string;
  ariaMultiselectable?: boolean;
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
  pinIndex = null,
  overscan = 8,
  threshold = 100,
  ariaLabel,
  ariaActiveDescendant,
  ariaMultiselectable,
  className,
  role,
  testId,
  tabIndex,
  onKeyDown
}: Props<T>) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const virtual = items.length >= threshold;
  const window = useMemo(() => {
    if (!virtual) return { start: 0, end: items.length };
    const start = Math.max(0, Math.floor(scrollTop / rowHeight) - overscan);
    const count = Math.ceil(height / rowHeight) + overscan * 2;
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
    if (!virtual || activeIndex === null || activeIndex < 0 || activeIndex >= items.length) return;
    const host = hostRef.current;
    if (!host) return;
    const top = activeIndex * rowHeight;
    const bottom = top + rowHeight;
    if (top < host.scrollTop) host.scrollTop = top;
    else if (bottom > host.scrollTop + height) host.scrollTop = bottom - height;
    if (host.scrollTop !== scrollTop) setScrollTop(host.scrollTop);
  }, [activeIndex, height, items.length, rowHeight, scrollTop, virtual]);

  const hostStyle: CSSProperties = { height, overflowY: virtual ? "auto" : "visible" };
  return (
    <div
      aria-label={ariaLabel}
      aria-activedescendant={ariaActiveDescendant}
      aria-multiselectable={ariaMultiselectable}
      className={className}
      data-testid={testId}
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
            } : { minHeight: rowHeight }}
          >
            {renderItem(items[index], index)}
          </div>
        ))}
      </div>
    </div>
  );
}
