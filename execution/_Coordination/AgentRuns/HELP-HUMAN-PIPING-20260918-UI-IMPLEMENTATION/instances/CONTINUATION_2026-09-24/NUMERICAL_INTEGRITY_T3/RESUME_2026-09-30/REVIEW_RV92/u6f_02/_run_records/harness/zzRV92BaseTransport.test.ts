/** RV92 confirmation: TS's base preview transport-metadata check on RV92's extra inputs (base lane). */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { validatePreviewPhysicsTransportMetadata } from "./features/results/previewPhysicsEvidence";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const dir = process.env.RV92_EXTRA, out = process.env.RV92_OUT;
describe.skipIf(!dir || !out)("RV92 base transport", () => {
  it("runs", () => {
    const res: Json = {};
    for (const e of JSON.parse(readFileSync(`${dir}/index.json`, "utf8")) as Json[]) {
      const p = JSON.parse(readFileSync(`${dir}/${e.file}`, "utf8"));
      try { validatePreviewPhysicsTransportMetadata(p.source); res[p.id] = "ok"; } catch (x) { res[p.id] = String((x as Error).message); }
    }
    writeFileSync(out!, JSON.stringify(res, null, 1));
    expect(true).toBe(true);
  });
});
