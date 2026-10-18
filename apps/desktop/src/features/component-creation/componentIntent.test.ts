import { describe, expect, it } from "vitest";
import { tieRodIdsDistinct } from "./componentIntent";

describe("tie-rod ids (T4-U5b round 4)", () => {
  it("compares ids as the producer does: raw and encoded (':' as '-')", () => {
    expect(tieRodIdsDistinct(["rod:a", "rod:b"])).toBe(true);
    expect(tieRodIdsDistinct(["rod:a", "rod:a"])).toBe(false);
    expect(tieRodIdsDistinct(["rod:a", " rod:a "])).toBe(false);
    expect(tieRodIdsDistinct(["rod:a", "rod-a"])).toBe(false);
    expect(tieRodIdsDistinct(["a:b:c", "a-b:c"])).toBe(false);
  });
});
