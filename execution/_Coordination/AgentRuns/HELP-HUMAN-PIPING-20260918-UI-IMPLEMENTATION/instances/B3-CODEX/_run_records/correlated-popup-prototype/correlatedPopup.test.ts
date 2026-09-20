import { afterEach, expect, it, vi } from "vitest";
import { createCorrelatedPopup, type PopupSnapshot } from "./correlatedPopup";
const base: PopupSnapshot = { generation: 1, windowEpoch: 0, tracking: 1, action: 0, endKey: -1 };
const cleanups: (() => void)[] = [];
afterEach(() => { cleanups.splice(0).forEach(fn => fn()); document.body.innerHTML = ""; });
function setup() {
  const select = document.createElement("select"); document.body.append(select); select.focus();
  let open = true, view = "both";
  vi.spyOn(select, "matches").mockImplementation(() => open);
  const pending: ((s: PopupSnapshot) => void)[] = [];
  const snapshot = vi.fn(() => new Promise<PopupSnapshot>(resolve => pending.push(resolve)));
  const log = vi.fn(); const guard = createCorrelatedPopup(window, () => view, snapshot, log); cleanups.push(guard.dispose);
  const fire = (type: string, key = "") => { const event = new KeyboardEvent(type, { key, bubbles: true, cancelable: true }); select.dispatchEvent(event); return event; };
  fire("keyup", " ");
  return { select, pending, guard, log, fire, close: vi.fn(), setOpen: (value: boolean) => { open = value; }, setView: (value: string) => { view = value; guard.refreshView(); } };
}
const flush = async () => { await Promise.resolve(); await Promise.resolve(); await Promise.resolve(); };
async function bound() { const c = setup(); c.pending.shift()!(base); await flush(); c.setOpen(false); return c; }
it("shares one decision across duplicate consumers and cancels a correlated Escape", async () => {
  const c = await bound(); const e = c.fire("keydown", "Escape");
  expect(c.guard.handle(e, c.close)).toBe(true); expect(c.guard.handle(e, c.close)).toBe(true);
  expect(c.pending).toHaveLength(1); c.pending.shift()!({ ...base, tracking: 0, endKey: 53 }); await flush(); expect(c.close).not.toHaveBeenCalled();
  expect(c.guard.handle(c.fire("keydown", "Escape"), c.close)).toBe(false);
});
it.each(["same value action", "outside pointer"])("allows intentional Escape after %s, including keyup before reply", async kind => {
  const c = await bound(); c.guard.handle(c.fire("keydown", "Escape"), c.close); c.fire("keyup", "Escape");
  c.pending.shift()!({ ...base, tracking: 0, action: kind === "same value action" ? 1 : 0 }); await flush(); expect(c.close).toHaveBeenCalledTimes(1);
});
it("retires keyup-only cancellation before later Escape", async () => {
  const c = await bound(); c.fire("keyup", "Escape"); expect(c.guard.handle(c.fire("keydown", "Escape"), c.close)).toBe(false);
});
it.each(["input", "focus", "view", "reopen", "dispose"])("rejects delayed response after %s generation changes", async kind => {
  const c = await bound(); c.guard.handle(c.fire("keydown", "Escape"), c.close);
  if (kind === "input") c.fire("keydown", "x");
  if (kind === "focus") c.fire("focusout");
  if (kind === "view") { c.setView("model"); c.setView("both"); }
  if (kind === "reopen") { c.fire("keydown", " "); c.setOpen(true); c.fire("keyup", " "); }
  if (kind === "dispose") c.guard.dispose();
  c.pending.shift()!({ ...base, tracking: 0, action: 1 }); await flush(); expect(c.close).not.toHaveBeenCalled();
});
it.each(["native generation", "native window"])("rejects unmatched %s", async kind => {
  const c = await bound(); c.guard.handle(c.fire("keydown", "Escape"), c.close);
  c.pending.shift()!({ ...base, tracking: 0, action: 1, ...(kind === "native generation" ? { generation: 2 } : { windowEpoch: 1 }) }); await flush(); expect(c.close).not.toHaveBeenCalled(); expect(c.log).toHaveBeenCalledWith(expect.objectContaining({ phase: "STOP" }));
});
it("does not retroactively bind a closed session when replies reorder", async () => {
  const c = setup(); c.setOpen(false); c.guard.handle(c.fire("keydown", "Escape"), c.close);
  c.pending[1]({ ...base, tracking: 0, action: 1 }); c.pending[0]({ ...base, tracking: 0, action: 1 }); await flush(); expect(c.close).not.toHaveBeenCalled(); expect(c.log).toHaveBeenCalledWith(expect.objectContaining({ phase: "STOP" }));
});
it("respects child consumed Escape, including consumption after claim", async () => {
  const c = await bound(); const e = c.fire("keydown", "Escape"); e.preventDefault(); expect(c.guard.handle(e, c.close)).toBe(false);
  const next = c.fire("keydown", "Escape"); c.guard.handle(next, c.close); next.preventDefault(); c.pending.shift()!({ ...base, tracking: 0, action: 1 }); await flush(); expect(c.close).not.toHaveBeenCalled();
});
