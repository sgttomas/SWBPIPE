import { afterEach, describe, expect, it, vi } from "vitest";
import { observeWorkspaceCanvasBudget } from "./workspaceCanvasBudget";

const property = "--workspace-modeling-reserve";
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); document.body.innerHTML = ""; });
function harness() {
  const raf = vi.spyOn(window, "requestAnimationFrame");
  let resizeCallback: () => void = () => {};
  let mutationCallback: () => void = () => {};
  const targets = new Set<Element>();
  const resizeDisconnect = vi.fn(() => targets.clear());
  const mutationDisconnect = vi.fn();
  const mutationObserve = vi.fn();
  vi.stubGlobal("ResizeObserver", class {
    constructor(callback: () => void) { resizeCallback = callback; }
    observe(element: Element) { targets.add(element); }
    disconnect = resizeDisconnect;
  });
  vi.stubGlobal("MutationObserver", class {
    constructor(callback: () => void) { mutationCallback = callback; }
    observe = mutationObserve;
    disconnect = mutationDisconnect;
  });
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
    return { height: Number(this.dataset.height ?? 0) } as DOMRect;
  });
  const workspace = document.createElement("div");
  workspace.style.rowGap = "6px";
  document.body.append(workspace);
  const mount = () => {
    workspace.innerHTML = `<section class="modeling-workspace"><div style="border-top:1px solid; border-bottom:1px solid"><div class="viewport-shell"><div class="command-bar" data-height="43"></div><div class="viewport-toolbar" data-height="80"></div><div class="viewport-frame"><canvas></canvas></div></div></div></section><div class="workspace-dock"></div>`;
    mutationCallback();
  };
  return { workspace, mount, targets, resizeDisconnect, mutationDisconnect, mutationObserve, raf,
    resize: () => resizeCallback(), mutate: () => mutationCallback() };
}

describe("workspace canvas budget ownership", () => {
  it("binds late-mounted chrome and rebinds measurement insertion and removal", () => {
    const h = harness();
    const dispose = observeWorkspaceCanvasBudget(h.workspace);
    expect(h.workspace.style.getPropertyValue(property)).toBe("");
    h.mount();
    expect(h.workspace.style.getPropertyValue(property)).toBe("331px");
    expect(h.mutationObserve).toHaveBeenLastCalledWith(h.workspace.querySelector(".viewport-shell"), { childList: true, subtree: false });
    const measurement = document.createElement("div");
    measurement.className = "viewport-measurement-strip";
    measurement.dataset.height = "30";
    h.workspace.querySelector(".viewport-shell")!.append(measurement);
    h.mutate();
    expect(h.targets.has(measurement)).toBe(true);
    expect(h.workspace.style.getPropertyValue(property)).toBe("361px");
    measurement.remove(); h.mutate();
    expect(h.targets.has(measurement)).toBe(false);
    expect(h.workspace.style.getPropertyValue(property)).toBe("331px");
    dispose();
  });
  it("responds to wrapping and width resize without observing flexible canvas or dock", () => {
    const h = harness(); h.mount();
    const dispose = observeWorkspaceCanvasBudget(h.workspace);
    const set = vi.spyOn(h.workspace.style, "setProperty");
    const toolbar = h.workspace.querySelector<HTMLElement>(".viewport-toolbar")!;
    toolbar.dataset.height = "91.25"; h.resize();
    expect(h.workspace.style.getPropertyValue(property)).toBe("343px");
    expect([...h.targets].some((element) => element.matches("canvas, .viewport-frame, .workspace-dock"))).toBe(false);
    set.mockClear(); h.resize(); h.mutate();
    expect(set).not.toHaveBeenCalled();
    dispose();
  });
  it("disconnects both observers and ignores already queued callbacks after disposal or rebinding", () => {
    const h = harness(); h.mount();
    const dispose = observeWorkspaceCanvasBudget(h.workspace);
    dispose();
    expect(h.resizeDisconnect).toHaveBeenCalled();
    expect(h.mutationDisconnect).toHaveBeenCalled();
    expect(h.workspace.style.getPropertyValue(property)).toBe("");
    h.resize(); h.mutate();
    expect(h.workspace.style.getPropertyValue(property)).toBe("");
    const nextDispose = observeWorkspaceCanvasBudget(h.workspace);
    expect(h.workspace.style.getPropertyValue(property)).toBe("331px");
    nextDispose();
    expect(h.workspace.style.getPropertyValue(property)).toBe("");
    expect(h.raf).not.toHaveBeenCalled();
  });
});
