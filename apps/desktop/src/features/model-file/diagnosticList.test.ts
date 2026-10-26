import { describe, expect, it } from "vitest";
import { listedItems, SAVE_DIAGNOSTICS_NOT_SHOWN } from "./diagnosticList";

const item = (i: number) => ({ code: `CODE-${i}`, path: `/p/${i}`, message: `m${i}` });
const named = (n: number) => Array.from({ length: n }, (_, i) => item(i));
const summary = (more: number, of: number) => ({
  code: SAVE_DIAGNOSTICS_NOT_SHOWN,
  path: "",
  message: `${more} more of the ${of} diagnostics were not shown; the first 64 are.`
});

describe("listedItems", () => {
  it("counts the true remainder when the native side closed the list with its summary (#1641 review r4)", () => {
    const text = listedItems([...named(64), summary(99_936, 100_000)], 8, true);
    expect(text).toContain("CODE-7 at /p/7: m7");
    expect(text).not.toContain("CODE-8 ");
    expect(text).not.toContain(SAVE_DIAGNOSTICS_NOT_SHOWN);
    expect(text.endsWith(" (+99992 more)")).toBe(true);
    expect(listedItems([...named(64), summary(86, 150)], 4, false).endsWith(" (+146 more)")).toBe(true);
  });

  it("lists a short list as before", () => {
    expect(listedItems(named(2), 8, true)).toBe("CODE-0 at /p/0: m0 | CODE-1 at /p/1: m1");
    expect(listedItems(named(10), 8, false).endsWith(" (+2 more)")).toBe(true);
    expect(listedItems([{ code: "C", path: "", message: "x" }], 4, true)).toBe("C: x");
  });

  it("shows an unrecognised summary as sent instead of counting it", () => {
    const odd = { code: SAVE_DIAGNOSTICS_NOT_SHOWN, path: "", message: "some were not shown" };
    expect(listedItems([...named(10), odd], 8, false)).toBe(`${named(8).map((d) => `${d.code} at ${d.path}`).join(" | ")} (+2 more); some were not shown`);
  });
});
