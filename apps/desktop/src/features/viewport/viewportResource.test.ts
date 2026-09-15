import { describe, expect, it, vi } from "vitest";
import { ViewportInvalidationScheduler, ViewportResourceRegistry } from "./viewportResource";

describe("viewport resource primitives", () => {
  it("coalesces invalidations and returns to zero owned RAF callbacks", () => {
    let callback: FrameRequestCallback | null = null;
    const request = vi.fn((next: FrameRequestCallback) => { callback = next; return 7; });
    const cancel = vi.fn();
    const pending = vi.fn();
    const render = vi.fn(() => false);
    const scheduler = new ViewportInvalidationScheduler(render, request, cancel, pending);
    scheduler.invalidate();
    scheduler.invalidate();
    expect(request).toHaveBeenCalledTimes(1);
    expect(scheduler.pendingCount).toBe(1);
    const scheduled = callback as unknown as FrameRequestCallback;
    scheduled(12);
    expect(render).toHaveBeenCalledWith(12);
    expect(scheduler.pendingCount).toBe(0);
    expect(pending).toHaveBeenLastCalledWith(0);
  });

  it("cancels pending work on context pause and disposal", () => {
    const request = vi.fn(() => 9);
    const cancel = vi.fn();
    const scheduler = new ViewportInvalidationScheduler(() => false, request, cancel);
    scheduler.invalidate();
    scheduler.pause();
    expect(cancel).toHaveBeenCalledWith(9);
    expect(scheduler.pendingCount).toBe(0);
    scheduler.resume();
    scheduler.dispose();
    expect(scheduler.pendingCount).toBe(0);
  });

  it("disposes each registered GPU-style resource exactly once", () => {
    const registry = new ViewportResourceRegistry();
    const first = { dispose: vi.fn() };
    const second = { dispose: vi.fn() };
    registry.own(first);
    registry.own(second);
    registry.dispose();
    registry.dispose();
    expect(first.dispose).toHaveBeenCalledTimes(1);
    expect(second.dispose).toHaveBeenCalledTimes(1);
  });
});
