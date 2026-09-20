import { afterEach, describe, expect, it, vi } from "vitest";
import { createNativeSelectPopupGuard } from "./nativeSelectPopup";

const cleanups: Array<() => void> = [];
afterEach(() => { cleanups.splice(0).forEach((cleanup) => cleanup()); document.body.innerHTML = ""; vi.restoreAllMocks(); });
function setup() {
  const root = document.createElement("div");
  const select = document.createElement("select");
  const other = document.createElement("button");
  root.append(select, other); document.body.append(root);
  let open = false;
  vi.spyOn(select, "matches").mockImplementation((selector) => selector === ":open" && open);
  const guard = createNativeSelectPopupGuard(window, () => root);
  cleanups.push(guard.dispose);
  const key = (type: string, key: string) => {
    const event = new KeyboardEvent(type, { key, bubbles: true, cancelable: true });
    select.dispatchEvent(event); return event;
  };
  const observeOpen = () => { open = true; key("keyup", " "); open = false; };
  return { guard, select, other, key, observeOpen, setOpen: (value: boolean) => { open = value; } };
}
describe("native select popup cancellation", () => {
  it("requires observed open state, not just focus or Space", () => {
    const { guard, select, key } = setup(); select.focus(); key("keyup", " ");
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
  it("remembers host closure and classifies the same event consistently for both handlers", () => {
    const { guard, key, observeOpen } = setup(); observeOpen();
    const escape = key("keydown", "Escape");
    expect(guard.isCancellation(escape)).toBe(true);
    expect(guard.isCancellation(escape)).toBe(true);
    expect(escape.defaultPrevented).toBe(false);
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
  it("observes a still-open popup on Escape without preventing native cancellation", () => {
    const { guard, key, setOpen } = setup(); setOpen(true);
    const event = key("keydown", "Escape");
    expect(guard.isCancellation(event)).toBe(true); expect(event.defaultPrevented).toBe(false);
  });
  it.each(["input", "change"])("clears on committed %s", (type) => {
    const { guard, select, key, observeOpen } = setup(); observeOpen();
    select.dispatchEvent(new Event(type, { bubbles: true }));
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
  it.each(["Enter", " ", "Escape"])("clears on closed keyup %s including same-value commit and native-only cancel", (commit) => {
    const { guard, key, observeOpen } = setup(); observeOpen(); key("keyup", commit);
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
  it.each(["pointerdown", "pointerup", "click", "focusin", "focusout"])("clears on outside %s", (type) => {
    const { guard, other, key, observeOpen } = setup(); observeOpen();
    other.dispatchEvent(new Event(type, { bubbles: true }));
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
  it("tracks pointer-open, same-value pointer commit and reopen", () => {
    const { guard, select, key, setOpen } = setup();
    setOpen(true); select.click(); setOpen(false); select.click();
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
    setOpen(true); select.click(); setOpen(false);
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(true);
  });
  it("does not claim unsupported :open or a select outside the root", () => {
    const { guard, select, key, setOpen } = setup();
    vi.mocked(select.matches).mockImplementation(() => { throw new DOMException(); });
    key("keyup", " "); expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
    vi.mocked(select.matches).mockReturnValue(true); setOpen(true); document.body.append(select);
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
  it("preserves child consumption and stops observing after disposal", () => {
    const { guard, key, observeOpen } = setup(); observeOpen();
    const event = key("keydown", "Escape"); event.preventDefault();
    expect(event.defaultPrevented).toBe(true);
    guard.dispose(); observeOpen();
    expect(guard.isCancellation(event)).toBe(false);
    expect(guard.isCancellation(key("keydown", "Escape"))).toBe(false);
  });
});
