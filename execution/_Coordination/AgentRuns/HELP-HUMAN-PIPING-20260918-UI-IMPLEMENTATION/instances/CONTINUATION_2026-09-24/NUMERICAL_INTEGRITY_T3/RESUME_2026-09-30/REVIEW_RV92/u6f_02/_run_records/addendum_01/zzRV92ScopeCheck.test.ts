/** RV92 addendum: the gate and code of TS's transport route on N-9's probes. Review harness only. */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { sourceContractTransport } from "./features/results/numericalResultQuality";
import { validateRetainedPrecisionTransport, RetainedPrecisionError } from "./features/results/retainedPrecision";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const dir = process.env.RV92_PROBES, out = process.env.RV92_OUT;
const ids = (process.env.RV92_IDS ?? "").split(",");
describe.skipIf(!dir || !out)("RV92 scope check", () => {
  it("runs", async () => {
    const index: Json[] = JSON.parse(readFileSync(`${dir}/index.json`, "utf8"));
    const res: Json = {};
    for (const e of index) {
      if (!ids.includes(e.id)) continue;
      const src = JSON.parse(readFileSync(`${dir}/${e.file}`, "utf8")).source;
      const run = async (fn: () => Promise<unknown>) => { try { await fn(); return { ok: true }; } catch (x) { return { reader_error: x instanceof RetainedPrecisionError, gate: (x as Json).gate ?? null, code: (x as Json).code ?? String((x as Error).message) }; } };
      res[e.id] = { route: await run(() => sourceContractTransport(src)), reader_transport: await run(() => validateRetainedPrecisionTransport(src)) };
    }
    writeFileSync(out!, JSON.stringify(res, null, 1));
    expect(Object.keys(res).length).toBe(ids.length);
  }, 600_000);
});
