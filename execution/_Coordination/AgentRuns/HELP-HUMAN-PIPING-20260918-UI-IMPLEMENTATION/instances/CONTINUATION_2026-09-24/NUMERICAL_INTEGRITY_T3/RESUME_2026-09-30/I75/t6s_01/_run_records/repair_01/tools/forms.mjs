// I75 REPAIR_01: three candidate printers against Rust {:e} on every word list (scratch; node only).
import { readFileSync } from "node:fs";
const dir = process.argv[2];
const fromWord = w => { const v = new DataView(new ArrayBuffer(8)); v.setBigUint64(0, BigInt(`0x${w}`)); return v.getFloat64(0); };
const sg = x => (x < 0 || Object.is(x, -0) ? "-" : "");
const nd = t => t.split("e")[0].replace(".", "").length;
const pre = x => `${sg(x)}${Math.abs(x).toExponential().replace("e+", "e")}`;
const form1 = x => { const m = Math.abs(x); let t = m.toExponential(); if (nd(t) === 17) t = m.toExponential(16); return `${sg(x)}${t.replace("e+", "e")}`; };
const form2 = x => { const m = Math.abs(x); const s = m.toExponential(); const r = m.toExponential(nd(s) - 1); return `${sg(x)}${(Number(r) === m ? r : s).replace("e+", "e")}`; };
const res = {};
for (const name of ["random", "ties", "rv101_differences", "edges", "ties16", "pow2"]) {
  const words = readFileSync(`${dir}/${name}.txt`, "utf8").trim().split("\n"), rust = readFileSync(`${dir}/${name}.txt.rust.txt`, "utf8").trim().split("\n");
  const r = { words: words.length, pre: 0, form1: 0, form2: 0, pre_by_digits: {}, form1_misses: [], guard_kept_shortest: 0, guard_examples: [] };
  words.forEach((w, i) => {
    const x = fromWord(w), m = Math.abs(x), s = m.toExponential(), n = nd(s), near = m.toExponential(n - 1);
    if (pre(x) !== rust[i]) { r.pre++; r.pre_by_digits[n] = (r.pre_by_digits[n] ?? 0) + 1; }
    if (form1(x) !== rust[i]) { r.form1++; if (r.form1_misses.length < 4) r.form1_misses.push(`${w} ${form1(x)} rust ${rust[i]}`); }
    if (form2(x) !== rust[i]) r.form2++;
    if (near !== s && Number(near) !== m) { r.guard_kept_shortest++; if (r.guard_examples.length < 4) r.guard_examples.push(`${w} shortest ${s} nearest ${near} rust ${rust[i]}`); }
  });
  res[name] = r;
}
console.log(JSON.stringify(res, null, 1));
