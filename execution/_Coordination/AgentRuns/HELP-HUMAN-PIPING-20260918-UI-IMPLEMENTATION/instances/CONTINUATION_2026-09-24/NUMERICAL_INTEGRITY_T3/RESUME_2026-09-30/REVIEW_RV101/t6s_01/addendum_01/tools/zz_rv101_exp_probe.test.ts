/** RV101 ADDENDUM_01 probe (scratch copy only): the product rustLowerExp over RV101's word lists, plus the
 * pre-repair form and RV101's suggested 17-digit form for comparison. Env: RV101_WORDS (dir). */
import { it, expect } from "vitest";
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { rustLowerExp } from "../features/results/retainedPrecisionDisclosure";
import { decodeBinary64 } from "../features/results/retainedPrecision";
const pre = (v: number) => `${v < 0 || Object.is(v, -0) ? "-" : ""}${Math.abs(v).toExponential().replace("e+", "e")}`;
const rule17 = (v: number) => { const a = Math.abs(v); let s = a.toExponential(); if (s.split("e")[0].replace(".", "").length === 17) s = a.toExponential(16); return `${v < 0 || Object.is(v, -0) ? "-" : ""}${s.replace("e+", "e")}`; };
it("RV101 rustLowerExp probe", () => {
  const dir = process.env.RV101_WORDS!;
  for (const f of readdirSync(dir).filter(n => n.endsWith(".txt") && !n.includes(".rust.") && !n.includes(".ts"))) {
    const words = readFileSync(resolve(dir, f), "utf8").trim().split("\n");
    const values = words.map(w => decodeBinary64(w));
    writeFileSync(resolve(dir, `${f}.ts_product.txt`), values.map(v => rustLowerExp(v)).join("\n") + "\n");
    writeFileSync(resolve(dir, `${f}.ts_pre.txt`), values.map(pre).join("\n") + "\n");
    writeFileSync(resolve(dir, `${f}.ts_rule17.txt`), values.map(rule17).join("\n") + "\n");
  }
  expect(true).toBe(true);
}, 600_000);
