import { invoke } from "@tauri-apps/api/core";

export type PopupSnapshot = { generation: number; windowEpoch: number; tracking: number; action: number; endKey: number };
// Diagnostic prototype: a native generation is owned only after a snapshot
// confirms tracking while the same DOM select is genuinely observed open.
export function createCorrelatedPopup(
  host: Window, getView: () => string,
  snapshot: () => Promise<PopupSnapshot> = () => invoke("native_popup_snapshot"),
  log: (record: unknown) => void = record => { void invoke("native_probe_log", { line: JSON.stringify({ kind: "correlated", record }) }); }
) {
  let generation = 0, input = 0, view = getView(), disposed = false;
  let owner: { select: HTMLSelectElement; generation: number; escapePending: boolean; native: Promise<PopupSnapshot | null> } | null = null;
  const claimed = new WeakSet<KeyboardEvent>();
  const clear = () => { generation++; owner = null; };
  const refreshView = () => { const next = getView(); if (view !== next) { view = next; clear(); input++; } };
  const observe = (event: Event) => {
    refreshView();
    if (event.type === "keydown" || event.type === "pointerdown") input++;
    if (event.type === "blur" || event.type === "focusout" || event.type === "change" || event.type === "input") { clear(); return; }
    const select = event.target;
    if (!(select instanceof HTMLSelectElement)) { if (event.type === "pointerdown") clear(); return; }
    if (select.matches(":open")) {
      if (owner?.select === select) return;
      clear();
      const observedGeneration = generation;
      const item = { select, generation, escapePending: false, native: Promise.resolve(null) as Promise<PopupSnapshot | null> };
      owner = item;
      item.native = snapshot().then(state => {
        const valid = !disposed && owner === item && generation === observedGeneration && state.tracking === 1 && select.matches(":open");
        log({ phase: "bind", observedGeneration, valid, state });
        return valid ? state : null;
      }).catch(() => null);
    } else if (event.type === "keyup" && (event as KeyboardEvent).key === "Escape") {
      // WKWebView may deliver only keyup for native cancellation. No ownership
      // may survive that input and suppress a later intentional Escape.
      if (owner?.escapePending) owner = null;
      else clear();
    } else if (event.type === "pointerdown" || (event.type === "keydown" && (event as KeyboardEvent).key !== "Escape")) {
      clear();
    }
  };
  const events = ["keydown", "keyup", "pointerdown", "pointerup", "click", "focusout", "change", "input", "blur"];
  events.forEach(type => host.addEventListener(type, observe, true));
  return {
    refreshView,
    handle(event: KeyboardEvent, close: () => void): boolean {
      if (event.key !== "Escape" || event.defaultPrevented) return false;
      if (claimed.has(event)) return true;
      refreshView();
      const item = owner;
      if (!item || event.target !== item.select) return false;
      claimed.add(event);
      item.escapePending = true;
      const capturedInput = input, capturedView = view, capturedGeneration = generation;
      // One event is shared by React narrow and window-global consumers.
      // Do not preventDefault: popup cancellation belongs to the native child.
      void Promise.all([item.native, snapshot()]).then(([bound, state]) => {
        refreshView();
        const current = !disposed && input === capturedInput && view === capturedView && generation === capturedGeneration && document.activeElement === item.select && !event.defaultPrevented;
        const correlated = bound !== null && state.generation === bound.generation && state.windowEpoch === bound.windowEpoch;
        const cancel = correlated && !state.action && (state.tracking === 1 || state.endKey === 53);
        log({ phase: "decision", capturedInput, current, correlated, cancel, bound, state });
        if (!current) return;
        clear();
        if (!correlated) { log({ phase: "STOP", reason: "Native generation cannot be correlated" }); return; }
        if (!cancel) close();
      }).catch(() => { clear(); log({ phase: "STOP", reason: "snapshot failed" }); });
      return true;
    },
    dispose() { disposed = true; clear(); events.forEach(type => host.removeEventListener(type, observe, true)); }
  };
}
