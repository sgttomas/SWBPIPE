/** B5 (DEL-07-01 Design `split-and-slope-entry.md` §3): the length-and-direction, angle and
 * "1 in n" slope conversions, checked against independent hand values. The unit conversion is
 * the engine's (wasm display facade). */
import { describe, expect, it } from "vitest";
import { computeDirectionEnd, directionDraftIssue, emptyDirectionDraft, exactCoordinateText, type DirectionDraft } from "./directionArithmetic";

const draft = (extra: Partial<DirectionDraft>): DirectionDraft => ({ ...emptyDirectionDraft("ft"), ...extra });
const dir = (x: string, y: string, z: string) => ({ direction: { x, y, z } });
const ULP = (value: number) => Math.abs(value) * Number.EPSILON;

describe("slope: 1 in n over the horizontal run (no trigonometry)", () => {
  it("a drain falling 1 in 48 over 12 ft along +X, +Z up: (12, 0, −1/4) exactly", async () => {
    const end = await computeDirectionEnd(draft({ mode: "slope", length: "12", ...dir("1", "0", "0"), up: "+z", slopeN: "48", slopeSense: "falls" }), { x: 2, y: 3, z: 0.5 }, "ft");
    expect(end.increment).toEqual({ x: 12, y: 0, z: -0.25 });
    expect(end.position).toEqual({ x: 14, y: 3, z: 0.25 });
  });

  it("rises, with −Y up: the vertical increment is along −Y", async () => {
    const end = await computeDirectionEnd(draft({ mode: "slope", length: "10", ...dir("0", "0", "-2"), up: "-y", slopeN: "100", slopeSense: "rises" }), { x: 0, y: 0, z: 0 }, "ft");
    expect(end.position).toEqual({ x: 0, y: -0.1, z: -10 });
  });

  it("a diagonal horizontal direction: each horizontal component is run / √2 within one ulp", async () => {
    const end = await computeDirectionEnd(draft({ mode: "slope", length: "12", ...dir("1", "1", "0"), up: "+z", slopeN: "50", slopeSense: "falls" }), { x: 0, y: 0, z: 0 }, "ft");
    const hand = 8.485281374238570; // 12 / √2 = 6√2
    expect(Math.abs(end.position.x - hand)).toBeLessThanOrEqual(ULP(hand));
    expect(end.position.y).toBe(end.position.x);
    expect(end.position.z).toBe(-0.24);
  });

  it("the run in another unit is converted first (engine conversion), then divided: 120 in = 10 ft", async () => {
    const end = await computeDirectionEnd(draft({ mode: "slope", length: "120", lengthUnit: "in", ...dir("0", "1", "0"), up: "+z", slopeN: "40", slopeSense: "falls" }), { x: 1, y: 1, z: 1 }, "ft");
    expect(end.lengthInProjectUnit).toBe(10);
    expect(end.position).toEqual({ x: 1, y: 11, z: 0.75 });
  });
});

describe("angle from the horizontal", () => {
  it("2 ft at 30° rising along +X, +Z up: (√3, 0, 1) within one ulp", async () => {
    const end = await computeDirectionEnd(draft({ mode: "angle", length: "2", ...dir("1", "0", "0"), up: "+z", angle: "30", angleUnit: "deg" }), { x: 0, y: 0, z: 0 }, "ft");
    expect(Math.abs(end.position.x - 1.7320508075688772)).toBeLessThanOrEqual(ULP(1.7320508075688772));
    expect(end.position.y).toBe(0);
    expect(Math.abs(end.position.z - 1)).toBeLessThanOrEqual(ULP(1));
  });

  it("−45° falls; 0° is exactly horizontal", async () => {
    const down = await computeDirectionEnd(draft({ mode: "angle", length: "4", ...dir("0", "3", "0"), up: "+z", angle: "-45", angleUnit: "deg" }), { x: 0, y: 0, z: 0 }, "ft");
    const hand = 2.8284271247461903; // 4 / √2
    expect(Math.abs(down.position.y - hand)).toBeLessThanOrEqual(ULP(hand));
    expect(Math.abs(down.position.z + hand)).toBeLessThanOrEqual(ULP(hand));
    const flat = await computeDirectionEnd(draft({ mode: "angle", length: "4", ...dir("0", "3", "0"), up: "+z", angle: "0", angleUnit: "rad" }), { x: 1, y: 1, z: 1 }, "ft");
    expect(flat.position).toEqual({ x: 1, y: 5, z: 1 });
  });

  it("90° is refused: a vertical element is entered as a direction vector", async () => {
    await expect(computeDirectionEnd(draft({ mode: "angle", length: "4", ...dir("1", "0", "0"), up: "+z", angle: "90", angleUnit: "deg" }), { x: 0, y: 0, z: 0 }, "ft"))
      .rejects.toThrow("strictly between −90° and 90°");
  });
});

describe("length along a direction vector", () => {
  it("(3, 4, 0) is normalized: 10 ft gives (6, 8, 0) exactly", async () => {
    const end = await computeDirectionEnd(draft({ mode: "vector", length: "10", ...dir("3", "4", "0") }), { x: 0.5, y: 0.25, z: 2 }, "ft");
    expect(end.position).toEqual({ x: 6.5, y: 8.25, z: 2 });
  });

  it("an axis direction gives the length exactly; no vertical axis is needed", async () => {
    const end = await computeDirectionEnd(draft({ mode: "vector", length: "3.048", lengthUnit: "m", ...dir("0", "0", "-7") }), { x: 0, y: 0, z: 12.5 }, "ft");
    expect(end.lengthInProjectUnit).toBe(10);
    expect(end.position).toEqual({ x: 0, y: 0, z: 2.5 });
  });

  it("a −0 start coordinate and a −0 direction component give +0, never −0", async () => {
    const end = await computeDirectionEnd(draft({ mode: "vector", length: "2", ...dir("-0", "1", "0") }), { x: -0, y: 0, z: -0 }, "ft");
    expect(Object.is(end.position.x, 0)).toBe(true);
    expect(Object.is(end.position.z, 0)).toBe(true);
    expect(exactCoordinateText(-0)).toBe("0");
  });
});

describe("refusals: nothing is assumed", () => {
  it.each([
    [draft({}), "ft", "Choose how the end point is entered"],
    [draft({ mode: "slope", length: "12", ...dir("1", "0", "0"), slopeN: "48", slopeSense: "falls" }), "ft", "State which global direction is up"],
    [draft({ mode: "slope", length: "12", ...dir("1", "0", "0.5"), up: "+z", slopeN: "48", slopeSense: "falls" }), "ft", "must have a zero Z component"],
    [draft({ mode: "slope", length: "12", ...dir("1", "0", "0"), up: "+z", slopeN: "48" }), "ft", "falls or rises"],
    [draft({ mode: "slope", length: "12", ...dir("1", "0", "0"), up: "+z", slopeN: "0", slopeSense: "falls" }), "ft", "greater than zero"],
    [draft({ mode: "vector", length: "-1", ...dir("1", "0", "0") }), "ft", "greater than zero"],
    [draft({ mode: "vector", length: "1", ...dir("0", "0", "0") }), "ft", "nonzero vector"],
    [draft({ mode: "vector", length: "1", lengthUnit: "", ...dir("1", "0", "0") }), "ft", "Choose the length unit"],
    [draft({ mode: "angle", length: "1", ...dir("1", "0", "0"), up: "+z", angle: "10" }), "ft", "Choose the angle unit"],
    [draft({ mode: "vector", length: "1", ...dir("1", "0", "0") }), undefined, "project length unit is not set"]
  ])("%#: %s", (entry, unit, text) => {
    expect(directionDraftIssue(entry, unit)).toContain(text);
  });
});
