/** Remembers observed native popup state across WebKit's pre-keydown close. */
export function createNativeSelectPopupGuard(host: Window, getRoot: () => HTMLElement | null) {
  let opened: HTMLSelectElement | null = null;
  let disposed = false;
  const cancellations = new WeakMap<KeyboardEvent, boolean>();

  function selectFor(event: Event): HTMLSelectElement | null {
    const target = event.target;
    const root = getRoot();
    return target instanceof HTMLSelectElement && root?.contains(target) ? target : null;
  }

  function isOpen(select: HTMLSelectElement | null): boolean {
    // Older engines may not implement :open. Focus or an opening key alone is
    // not evidence that a native popup opened.
    try { return Boolean(select?.matches(":open")); } catch { return false; }
  }

  function isCancellation(event: KeyboardEvent): boolean {
    if (disposed || event.key !== "Escape") return false;
    const cached = cancellations.get(event);
    if (cached !== undefined) return cached;
    const select = selectFor(event);
    const cancel = Boolean(select && (opened === select || isOpen(select)));
    cancellations.set(event, cancel);
    opened = null;
    return cancel;
  }

  function observe(event: Event) {
    const select = selectFor(event);
    if (event.type === "keydown" && (event as KeyboardEvent).key === "Escape") {
      isCancellation(event as KeyboardEvent);
      return;
    }
    // Each lifecycle event replaces, rather than extends, prior evidence. This
    // includes keyup-only native cancellation and same-value commit (no change
    // event), and pointer/focus transitions outside the select.
    if (event.type === "input" || event.type === "change" ||
        event.type === "focusout" || event.type === "blur" || event.type === "pointercancel") {
      opened = null;
      return;
    }
    opened = isOpen(select) ? select : null;
  }

  const types = ["keydown", "keyup", "pointerdown", "pointerup", "pointercancel",
    "click", "input", "change", "focusin", "focusout", "blur"] as const;
  for (const type of types) host.addEventListener(type, observe, true);
  return {
    isCancellation,
    dispose() {
      disposed = true;
      opened = null;
      for (const type of types) host.removeEventListener(type, observe, true);
    }
  };
}
