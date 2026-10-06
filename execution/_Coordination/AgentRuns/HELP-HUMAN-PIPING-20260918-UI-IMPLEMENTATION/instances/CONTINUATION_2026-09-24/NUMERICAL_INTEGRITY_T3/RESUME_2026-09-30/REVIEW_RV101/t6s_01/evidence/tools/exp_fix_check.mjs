// RV101: checks the suggested repair of rustLowerExp against Rust {:e} over the oracle's word list.
import { readFileSync } from "node:fs";
const [words, rust] = [process.argv[2], process.argv[3]].map(p => readFileSync(p, "utf8").trim().split("\n"));
const decode = h => { const b = new DataView(new ArrayBuffer(8)); b.setBigUint64(0, BigInt("0x" + h)); return b.getFloat64(0); };
const current = v => `${v < 0 || Object.is(v, -0) ? "-" : ""}${Math.abs(v).toExponential().replace("e+", "e")}`;
const repaired = v => { const a = Math.abs(v); let s = a.toExponential(); if (s.split("e")[0].replace(".", "").length === 17) s = a.toExponential(16); return `${v < 0 || Object.is(v, -0) ? "-" : ""}${s.replace("e+", "e")}`; };
let c = 0, r = 0; for (let i = 0; i < words.length; i++) { const v = decode(words[i]); if (current(v) !== rust[i]) c++; if (repaired(v) !== rust[i]) r++; }
console.log(JSON.stringify({ words: words.length, current_mismatches: c, repaired_mismatches: r }));
