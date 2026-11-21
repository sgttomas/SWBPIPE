import { type ReactNode, useEffect, useId, useLayoutEffect, useRef, useState } from "react";

/**
 * The view's one control bar: groups in priority order, and a More button at the bar's end.
 * While the bar is too narrow for every group, the lowest-priority groups move whole into the
 * More menu, where the same controls are rendered (same test ids, names, pressed states,
 * disabled reasons). A group is never split and never clipped: it is either in the bar or in the
 * menu. `menuGroups` always live in the menu.
 *
 * The rule: each group's width is measured while it is in the bar and kept with a signature of
 * its content; the bar shows the longest prefix of groups whose kept widths fit beside the More
 * button. A group with no kept width for its current signature is measured by showing every
 * group for one layout pass (before paint). With no measurable width (no layout, as in jsdom)
 * every group stays in the bar. A group's kept width is its own; a change of the bar's width (the
 * window, a panel) fits again from the kept widths, so the bar rearranges while More is open too.
 *
 * The menu opens as a second row in the flow beneath the bar, never over the canvas.
 */
export type ViewportBarGroup = {
  key: string;
  /** The group's accessible name. */
  label: string;
  /** Its short visible heading in the menu. */
  heading: string;
  /** Controls in the group that are on; shown on the More button while the group is in the menu. */
  onCount: number;
  /** Changes when the group's content (and so its width) changes. */
  signature: string;
  content: ReactNode;
};


export function ViewportOverflowBar({ label, groups, menuGroups }: { label: string; groups: readonly ViewportBarGroup[]; menuGroups: readonly ViewportBarGroup[] }) {
  const barRef = useRef<HTMLDivElement | null>(null);
  const groupsRef = useRef<HTMLDivElement | null>(null);
  const moreRef = useRef<HTMLDivElement | null>(null);
  const buttonRef = useRef<HTMLButtonElement | null>(null);
  const menuRef = useRef<HTMLDivElement | null>(null);
  const widthsRef = useRef(new Map<string, { signature: string; width: number }>());
  const [shownCount, setShownCount] = useState(groups.length);
  const [open, setOpen] = useState(false);
  const [, setResizeTick] = useState(0);
  const menuId = useId();

  const shown = groups.slice(0, Math.min(shownCount, groups.length));
  const overflowed = groups.slice(shown.length);
  const onInMenu = overflowed.reduce((sum, group) => sum + group.onCount, 0) +
    menuGroups.reduce((sum, group) => sum + group.onCount, 0);

  useLayoutEffect(() => {
    const groupsElement = groupsRef.current;
    const bar = barRef.current;
    if (!groupsElement || !bar) return;
    const widths = widthsRef.current;
    const bySignature = new Map(groups.map((group) => [group.key, group.signature] as const));
    for (const element of groupsElement.querySelectorAll<HTMLElement>(":scope > [data-bar-group]")) {
      const key = element.dataset.barGroup ?? "";
      const signature = bySignature.get(key);
      if (signature === undefined) continue;
      widths.set(key, { signature, width: element.getBoundingClientRect().width });
    }
    const barWidth = bar.getBoundingClientRect().width;
    if (!(barWidth > 0)) {
      if (shownCount !== groups.length) setShownCount(groups.length);
      return;
    }
    const unmeasured = groups.some((group) => widths.get(group.key)?.signature !== group.signature);
    if (unmeasured) {
      // Measure: show every group for one pass, unless a person is working in the open menu.
      if (shownCount !== groups.length && !open) setShownCount(groups.length);
      return;
    }
    const moreWidth = moreRef.current?.getBoundingClientRect().width ?? 0;
    const available = barWidth - moreWidth - horizontalSpace(bar);
    const gap = gapOf(groupsElement);
    let used = 0;
    let fit = 0;
    for (const group of groups) {
      used += widths.get(group.key)!.width + (fit > 0 ? gap : 0);
      if (used > available + 0.5) break;
      fit += 1;
    }
    if (fit !== shownCount) setShownCount(fit);
  });

  // The bar's width (window, panels, view switch) decides what fits: a change fits again from the
  // kept widths, open menu or not. The shown groups are measured again on every pass, so a change of
  // their own width (fonts, density, a count in a label) is kept as it happens.
  const lastSizesRef = useRef("");
  useEffect(() => {
    const bar = barRef.current;
    if (!bar || typeof ResizeObserver === "undefined") return;
    const observer = new ResizeObserver(() => {
      const sizes = [bar, ...(groupsRef.current?.children ?? [])].map((element) => Math.round(element.getBoundingClientRect().width)).join(":");
      if (sizes === lastSizesRef.current) return;
      lastSizesRef.current = sizes;
      setResizeTick((tick) => tick + 1);
    });
    observer.observe(bar);
    for (const element of groupsRef.current?.children ?? []) observer.observe(element);
    return () => observer.disconnect();
  });

  useEffect(() => {
    if (!open) return;
    const outside = (event: PointerEvent) => {
      const target = event.target as Node | null;
      if (!target) return;
      if (menuRef.current?.contains(target) || buttonRef.current?.contains(target)) return;
      setOpen(false);
    };
    const escape = (event: KeyboardEvent) => {
      if (event.key !== "Escape" || event.defaultPrevented) return;
      event.preventDefault();
      setOpen(false);
      buttonRef.current?.focus();
    };
    document.addEventListener("pointerdown", outside, true);
    document.addEventListener("keydown", escape);
    return () => {
      document.removeEventListener("pointerdown", outside, true);
      document.removeEventListener("keydown", escape);
    };
  }, [open]);

  const moreName = `More view controls${onInMenu > 0 ? `, ${onInMenu} on` : ""}`;
  return (
    <div className="viewport-bar" ref={barRef} role="group" aria-label={label} data-testid="viewport-bar">
      <div className="viewport-bar-groups" ref={groupsRef}>
        {shown.map((group) => (
          <div className="viewport-bar-group" data-bar-group={group.key} key={group.key} role="group" aria-label={group.label}>
            {group.content}
          </div>
        ))}
      </div>
      <div className="viewport-more" ref={moreRef}>
        <button
          aria-controls={open ? menuId : undefined}
          aria-expanded={open}
          aria-label={moreName}
          className="viewport-more-button"
          data-testid="viewport-more"
          onClick={() => setOpen((value) => !value)}
          ref={buttonRef}
          title={moreName}
          type="button"
        >
          More
          {onInMenu > 0 ? <span className="viewport-more-count" aria-hidden="true" data-testid="viewport-more-count">{onInMenu}</span> : null}
          <span className="viewport-more-caret" aria-hidden="true">▾</span>
        </button>
      </div>
      {open ? (
        <div
          aria-label="More view controls"
          className="viewport-more-menu"
          data-testid="viewport-more-menu"
          id={menuId}
          ref={menuRef}
          role="group"
          onClick={(event) => {
            // A command (Fit, Hide, Isolate, Show all) closes the menu; a toggle, a selector or
            // the Deformation disclosure leaves it open so its state can be read and changed.
            const control = event.target instanceof Element ? event.target.closest("button") : null;
            if (!control || control.disabled || control.hasAttribute("aria-pressed") || control.closest("details")) return;
            setOpen(false);
            buttonRef.current?.focus();
          }}
        >
          {[...overflowed, ...menuGroups].map((group) => (
            <div className="viewport-more-group" data-menu-group={group.key} key={group.key} role="group" aria-label={group.label}>
              <div className="viewport-more-heading" aria-hidden="true">{group.heading}</div>
              <div className="viewport-more-controls">{group.content}</div>
            </div>
          ))}
        </div>
      ) : null}
    </div>
  );
}

/** The bar's padding and the gap between the groups and More. */
function horizontalSpace(bar: HTMLElement): number {
  const style = getComputedStyle(bar);
  const value = (name: string) => Number.parseFloat(style.getPropertyValue(name)) || 0;
  return value("padding-left") + value("padding-right") + gapOf(bar);
}

function gapOf(element: HTMLElement): number {
  const style = getComputedStyle(element);
  return Number.parseFloat(style.getPropertyValue("column-gap")) || Number.parseFloat(style.getPropertyValue("gap")) || 0;
}
