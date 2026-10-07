// I75 REPAIR_01: classify each word by pre-repair V8 shortest form against Rust {:e} (scratch only).
import { readFileSync, writeFileSync } from "node:fs";
const dir = process.argv[2];
const fromWord = w => { const v = new DataView(new ArrayBuffer(8)); v.setBigUint64(0, BigInt(`0x${w}`)); return v.getFloat64(0); };
const pre = x => `${x < 0 || Object.is(x, -0) ? "-" : ""}${Math.abs(x).toExponential().replace("e+", "e")}`;
const out = {};
for (const name of ["random.txt", "ties.txt", "rv101_differences.txt", "edges.txt"]) {
  const words = readFileSync(`${dir}/${name}`, "utf8").trim().split("\n");
  const rust = readFileSync(`${dir}/${name}.rust.txt`, "utf8").trim().split("\n");
  const rows = words.map((w, i) => { const x = fromWord(w); const p = pre(x); const digits = rust[i].replace(/^-/, "").split("e")[0].replace(".", "").length; return { w, rust: rust[i], pre: p, digits, differ: p !== rust[i] }; });
  out[name] = { words: rows.length, pre_mismatches: rows.filter(r => r.differ).length, rust_17_digit: rows.filter(r => r.digits === 17).length };
  if (name === "ties.txt") {
    const differ = rows.filter(r => r.differ), agree17 = rows.filter(r => !r.differ && r.digits === 17), short = rows.filter(r => r.digits < 17);
    // differing words must be exactly one unit of the 17th digit below Rust's (V8 took the lower, even candidate)
    out[name].differ_lower_even = differ.every(r => { const a = BigInt(r.pre.replace(/^-/, "").split("e")[0].replace(".", "")), b = BigInt(r.rust.replace(/^-/, "").split("e")[0].replace(".", "")); return b - a === 1n && a % 2n === 0n; });
    out[name].agree_17_upper_even = agree17.every(r => BigInt(r.rust.replace(/^-/, "").split("e")[0].replace(".", "")) % 2n === 0n);
    out[name].agree_17 = agree17.length; out[name].shorter = short.length;
    const pick = (rs, n) => { const s = [...rs].sort((a, b) => a.w < b.w ? -1 : 1); const step = Math.floor(s.length / n); return Array.from({ length: n }, (_, i) => s[i * step]); };
    writeFileSync(`${dir}/tie_vectors.tsv`, [...pick(differ, 8), ...pick(agree17, 6), ...pick(short, 4)].map(r => `${r.w}\t${r.rust}\t${r.pre}\t${r.differ ? "differ" : r.digits === 17 ? "agree17" : "shorter"}`).join("\n") + "\n");
  }
}
console.log(JSON.stringify(out, null, 1));
