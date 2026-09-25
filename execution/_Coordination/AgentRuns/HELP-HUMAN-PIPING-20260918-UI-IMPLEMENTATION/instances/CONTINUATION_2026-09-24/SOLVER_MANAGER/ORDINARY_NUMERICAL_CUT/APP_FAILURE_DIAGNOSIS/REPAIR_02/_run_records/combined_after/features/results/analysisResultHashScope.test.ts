import { describe, expect, it } from "vitest";
import { analysisResultHashScope } from "./analysisResultHashScope";

describe("analysis received-result hash dispatch", () => {
  it.each([["0.1.0", "result_envelope"], ["0.2.0", "received_result"], ["0.3.0", "received_result"]])("maps exact version %s", (version, scope) => {
    expect(analysisResultHashScope(version)).toBe(scope);
  });
  it.each([undefined, null, "", "0.4.0", "0.3.1", "1.0.0"])("refuses unsupported version %s", version => {
    expect(analysisResultHashScope(version)).toBeNull();
  });
});
