import { describe, expect, it, vi } from "vitest";
import { ViewportSizeWatch } from "./viewportSizeWatch";

describe("the view's size watches", () => {
  it("attaches a watch made before the resource, detaches before its disposal, and stops once", () => {
    const sizes = new ViewportSizeWatch();
    const element = document.createElement("div");
    const callback = vi.fn();
    const stop = sizes.watch(element, callback);
    const unobserve = vi.fn();
    const observe = vi.fn(() => unobserve);
    sizes.attach(observe);
    expect(observe).toHaveBeenCalledWith(element, callback);
    sizes.detach();
    expect(unobserve).toHaveBeenCalledTimes(1);
    sizes.attach(observe);
    expect(observe).toHaveBeenCalledTimes(2);
    stop();
    stop();
    expect(unobserve).toHaveBeenCalledTimes(2);
    sizes.detach();
    expect(unobserve).toHaveBeenCalledTimes(2);
  });
});
