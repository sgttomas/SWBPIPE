import { describe, expect, it } from "vitest";
import { createModalHolds } from "./modalHolds";

// Modal ownership (W-12 S1; Codex r4): the shared flag stays set until the last
// open prompt closes, whichever closes first.
describe("modal holds", () => {
  it("keeps the flag set until the last hold is released, in any order", () => {
    const flag = { current: false };
    const holds = createModalHolds(flag);
    const first = holds.hold();
    const second = holds.hold();
    expect(flag.current).toBe(true);
    first();
    expect(flag.current).toBe(true);
    expect(holds.count()).toBe(1);
    second();
    expect(flag.current).toBe(false);
    expect(holds.count()).toBe(0);
  });

  it("releases each hold once", () => {
    const flag = { current: false };
    const holds = createModalHolds(flag);
    const first = holds.hold();
    const second = holds.hold();
    first();
    first();
    expect(flag.current).toBe(true);
    second();
    expect(flag.current).toBe(false);
    holds.hold();
    expect(flag.current).toBe(true);
  });
});
