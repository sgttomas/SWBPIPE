import { describe, expect, it } from "vitest";
import { analysisResultHashScope } from "./analysisResultHashScope";

describe("analysis received-result hash dispatch", () => {
  it("maps the current version", () => {
    expect(analysisResultHashScope("0.3.0")).toBe("received_result");
  });
  it.each([undefined, null, "", "0.1.0", "0.2.0", "0.4.0", "0.3.1", "1.0.0"])("refuses a retired or unsupported version %s", version => {
    expect(analysisResultHashScope(version)).toBeNull();
  });
});
