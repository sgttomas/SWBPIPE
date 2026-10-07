// I75 REPAIR_01: pick Rust-computed vectors for 16-digit ties and the round-trip guard (scratch; node only).
import { readFileSync } from "node:fs";
const dir = process.argv[2];
const fromWord = w => { const v = new DataView(new ArrayBuffer(8)); v.setBigUint64(0, BigInt(`0x${w}`)); return v.getFloat64(0); };
const nd = t => t.replace(/^-/, "").split("e")[0].replace(".", "").length;
const load = n => { const w = readFileSync(`${dir}/${n}.txt`, "utf8").trim().split("\n"), r = readFileSync(`${dir}/${n}.txt.rust.txt`, "utf8").trim().split("\n"); return w.map((x, i) => ({ w: x, rust: r[i], v: fromWord(x) })); };
const pick = (rs, n) => { const s = [...rs].sort((a, b) => a.w < b.w ? -1 : 1); const step = Math.max(1, Math.floor(s.length / n)); return Array.from({ length: Math.min(n, s.length) }, (_, i) => s[i * step]); };
const pre = r => `${r.v < 0 ? "-" : ""}${Math.abs(r.v).toExponential().replace("e+", "e")}`;
const t16 = load("ties16"), rnd = load("random"), p2 = load("pow2");
const differ16 = t16.filter(r => pre(r) !== r.rust), agree16 = t16.filter(r => pre(r) === r.rust && nd(r.rust) === 16 && (() => { const m = Math.abs(r.v), s = m.toExponential(); return true; })());
// agree16 must be real ties whose lower candidate is odd: exact value is exactly halfway at the 16th digit
const half16 = r => { const m = Math.abs(r.v); const e = Math.floor(Math.log10(m)); return null; };
const guard = p2.filter(r => { const m = Math.abs(r.v), s = m.toExponential(), near = m.toExponential(nd(s) - 1); return near !== s && Number(near) !== m; });
const p2pre = p2.filter(r => pre(r) !== r.rust);
const rnd16 = rnd.filter(r => pre(r) !== r.rust && nd(r.rust) === 16);
const show = (tag, rs) => rs.forEach(r => console.log(`${tag}\t${r.w}\t${r.rust}\t${pre(r)}`));
show("rnd16", rnd16); show("differ16", pick(differ16, 6)); show("guard", pick(guard, 6)); show("p2pre", p2pre);
console.log("counts differ16", differ16.length, "guard", guard.length, "p2pre", p2pre.length);
