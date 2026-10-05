/** RV92 confirmation: the reader's transport validator on every successor-id probe
 * (what TS's transport route would return if it ran the reader before the header shape). */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { validateRetainedPrecisionTransport } from "./features/results/retainedPrecision";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const dir = process.env.RV92_PROBES, out = process.env.RV92_OUT;
describe.skipIf(!dir || !out)("RV92 reader transport", () => {
  it("runs", async () => {
    const res: Json = {};
    for (const e of JSON.parse(readFileSync(`${dir}/index.json`, "utf8")) as Json[]) {
      const p = JSON.parse(readFileSync(`${dir}/${e.file}`, "utf8"));
      if (p.source?.producer?.semantic_contract_id !== "openpipestress.result_semantics/0.3.0/preview-physics-retained-1") continue;
      try { await validateRetainedPrecisionTransport(p.source); res[p.id] = "ok"; } catch (x) { res[p.id] = (x as Json).code ?? String((x as Json).message); }
    }
    writeFileSync(out!, JSON.stringify(res, null, 0));
    expect(true).toBe(true);
  }, 600_000);
});
