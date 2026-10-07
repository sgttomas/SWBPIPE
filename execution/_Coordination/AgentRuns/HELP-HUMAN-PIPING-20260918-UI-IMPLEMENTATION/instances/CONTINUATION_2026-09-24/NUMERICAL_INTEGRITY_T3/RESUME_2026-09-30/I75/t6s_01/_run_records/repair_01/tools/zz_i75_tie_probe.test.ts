/** I75 REPAIR_01 scratch probe (never committed): the repaired product `rustLowerExp` against Rust `{:e}`
 * over the seeded word lists; the pre-repair form is recomputed inline for the record. */
import { expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { rustLowerExp } from "./retainedPrecisionDisclosure";
const preRepair = (v: number) => `${v < 0 || Object.is(v, -0) ? "-" : ""}${Math.abs(v).toExponential().replace("e+", "e")}`;
const decode = (h: string) => { const b = new DataView(new ArrayBuffer(8)); b.setBigUint64(0, BigInt(`0x${h}`)); return b.getFloat64(0); };
it("matches Rust on every word", () => {
  const dir = process.env.I75_TIES!;
  const summary: Record<string, unknown> = {};
  let repairedTotal = 0;
  for (const name of ["random", "ties", "rv101_differences", "edges", "ties16", "pow2"]) {
    const words = readFileSync(`${dir}/${name}.txt`, "utf8").trim().split("\n");
    const rust = readFileSync(`${dir}/${name}.txt.rust.txt`, "utf8").trim().split("\n");
    expect(rust).toHaveLength(words.length);
    let pre = 0, repaired = 0, plus = 0; const firstRepaired: string[] = [];
    for (let i = 0; i < words.length; i++) {
      const v = decode(words[i]), r = rustLowerExp(v);
      if (preRepair(v) !== rust[i]) pre++;
      if (r !== rust[i]) { repaired++; if (firstRepaired.length < 5) firstRepaired.push(`${words[i]} ${r} ${rust[i]}`); }
      if (r.includes("+")) plus++;
    }
    summary[name] = { words: words.length, pre_repair_mismatches: pre, repaired_mismatches: repaired, plus_signs: plus, first_repaired_mismatches: firstRepaired };
    repairedTotal += repaired;
  }
  writeFileSync(`${dir}/tie_probe_summary.json`, JSON.stringify(summary, null, 1));
  expect(repairedTotal).toBe(0);
});
