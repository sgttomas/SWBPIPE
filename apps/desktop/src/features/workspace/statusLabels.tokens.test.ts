import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { REGISTERED_STATUS_LABELS } from "./statusLabels";

// The design tokens carry a copy of the registered display-form table
// (`labels`). Nothing in the product reads it: `statusLabels.ts` is the only
// source. This test holds the two tables together so the copy cannot drift.

type TokenLabelRow = { label: string; domain: string; kind: string; chip?: string };

// Vitest runs with the desktop package as its working directory.
const tokens = JSON.parse(readFileSync(path.join(process.cwd(), "src", "design", "tokens.json"), "utf8")) as {
  labels: Record<string, TokenLabelRow>;
};

describe("tokens.json labels agree with statusLabels.ts", () => {
  it("holds the same tokens in the same order", () => {
    expect(Object.keys(tokens.labels)).toEqual(Object.keys(REGISTERED_STATUS_LABELS));
  });

  it.each(Object.values(REGISTERED_STATUS_LABELS))("agrees on $token", (row) => {
    const copy = tokens.labels[row.token];
    expect(copy).toBeDefined();
    expect({ label: copy.label, domain: copy.domain, kind: copy.kind }).toEqual({
      label: row.label,
      domain: row.domain,
      kind: row.kind
    });
  });

  it("gives every copied row nothing the product's table lacks except its chip appearance", () => {
    for (const copy of Object.values(tokens.labels)) {
      expect(Object.keys(copy).sort()).toEqual(["chip", "domain", "kind", "label"]);
    }
  });
});
