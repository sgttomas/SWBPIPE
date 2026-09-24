import { describe, expect, it } from "vitest";
import { resolveTableTracks } from "./tableGeometry";

describe("shared table tracks", () => {
  it("resolves the actual gutter-reduced width once, freezing minimum tracks", () => {
    const available = 840 - 18;
    const widths = resolveTableTracks([130, 150, 180, 90], [1.4, 1.5, 1, 1], available);
    expect(widths.reduce((sum, value) => sum + value, 0)).toBeCloseTo(available);
    expect(widths[2]).toBe(180);
    expect(widths[1] / widths[0]).toBeCloseTo(1.5 / 1.4);
    const afterGutterDisappears = resolveTableTracks([130, 150, 180, 90], [1.4, 1.5, 1, 1], 840);
    expect(afterGutterDisappears.reduce((sum, value) => sum + value, 0)).toBeCloseTo(840);
  });
  it("retains all minima in a narrow viewport and expands beyond them without missing width", () => {
    expect(resolveTableTracks([130, 150, 180], [1.4, 1.5, 1], 120)).toEqual([130, 150, 180]);
    expect(resolveTableTracks([130, 150, 180], [1.4, 1.5, 1], 1600).reduce((sum, value) => sum + value, 0)).toBeCloseTo(1600);
  });
});
