"""I98 B2-W: one table row per input and mode from the probe logs (read-only; VENV).

Usage: python summarize.py <log>... > summary.md
Per input and mode: the published verdict, the seed (initial, w2, legacy, D-5), and per path
(Direct; the witness twin at 4 MiB; the bypass at R, when run) the W1 outcome, its furthest
phase, the native terminal, and the refusal that ended W1 (candidate or precommit).
"""
import json
import re
import sys


def field(line, key):
    m = re.search(r"(?:^| )" + re.escape(key) + r"=(\[[^\]]*\]|\{.*?\}(?= \w+=|$)|\S+)", line)
    return m.group(1) if m else None


def short_native(rest):
    if rest.startswith("Selected"):
        return "Selected"
    m = re.match(r"(Unresolved|Refused) (?:reason|refusal)=(\S+)", rest)
    return f"{m.group(1)}({m.group(2)})" if m else rest[:60]


def main():
    rows = {}
    order = []
    for path in sys.argv[1:]:
        label = mode = None
        ctx = None
        for raw in open(path, encoding="utf-8", errors="replace"):
            line = raw.rstrip("\n")
            if line.startswith("test ") and " I98_" in line:
                line = line[line.index(" I98_") + 1:]
            if not line.startswith("I98_"):
                continue
            kind, _, rest = line.partition(" ")
            if kind == "I98_BEGIN":
                label, mode = rest.split()[:2]
                ctx = "direct"
                key = (label, mode)
                if key not in rows:
                    rows[key] = {"paths": {}}
                    order.append(key)
                continue
            if label is None:
                continue
            r = rows[(label, mode)]
            if kind == "I98_ORDINARY" and rest.startswith(f"{label} {mode} "):
                r["verdict"] = json.loads(field(rest, "published_verdicts"))[0]["solve_quality"] if field(rest, "published_verdicts") not in (None, "[]") else "none"
                r["rcond"] = field(rest, "rcond")
                d5 = re.search(r"d5=\[(.*?)\] range_scaling", rest)
                r["d5"] = "none" if d5 is None or d5.group(1) == '"none"' else re.sub(r".*row=([^;]+);.*trigger_ratio=([0-9.e-]+).*", r"row \1, ratio \2", d5.group(1))[:40]
                r["range"] = field(rest, "range_scaling")
            elif kind == "I98_SEEDS" and "site=permitted_run" in rest or (kind == "I98_SEEDS" and "seed" not in r):
                seeds = json.loads(rest.split(" seeds=", 1)[1])
                if seeds:
                    s = seeds[0]
                    ini = s["initial"]
                    r["seed"] = (f"{ini['kind']}/{ini.get('outcome') or ini.get('tag')}; w2 {s['w2']['kind']}"
                                 + (f" b={s['w2'].get('force_scale_exponent')}" if s["w2"]["kind"] == "published" else "")
                                 + f"; legacy {s['legacy']}")
            elif kind == "I98_NATIVE_OUTCOME":
                r["paths"].setdefault(ctx, {})["native"] = short_native(rest)
            elif kind == "I98_CANDIDATE_REFUSAL":
                m = re.search(r"cause: (\w+)\((\w+)", rest)
                r["paths"].setdefault(ctx, {})["refusal"] = f"candidate: {m.group(1)}({m.group(2)})" if m else "candidate"
            elif kind == "I98_PRECOMMIT_ERROR":
                m = re.search(r'gate: "(\w+)", code: "(\w+)"', rest)
                r["paths"].setdefault(ctx, {})["refusal"] = f"precommit {m.group(1)} {m.group(2)}"
            elif kind == "I98_W1" and rest.startswith(f"{label} {mode} "):
                d = r["paths"].setdefault("direct", {})
                d["outcome"] = re.search(r"cause=(.*?) phase=", rest).group(1) if "cause=None" not in rest else "no W1 (G-A refused)"
                d["phase"] = re.search(r"phase=(.*?) counts=", rest).group(1)
                d["notices"] = field(rest, "notices")
                d["successor"] = field(rest, "published_successor")
                ctx = "twin"
            elif kind == "I98_SUCCESSOR" and rest.startswith(f"{label} {mode} "):
                r["reader"] = field(rest, "rust_reader")
            elif kind == "I98_WITNESS" and rest.startswith(f"{label} {mode} "):
                d = r["paths"].setdefault("twin", {})
                d["outcome"] = re.search(r"ran=(.*?) furthest_phase=", rest).group(1)
                d["phase"] = re.search(r"furthest_phase=(.*?) ms=", rest).group(1)
                ctx = None
            elif kind == "I98_BYPASS_GA":
                ctx = "bypass"
                r["paths"].setdefault("bypass", {})["G-A"] = re.search(r"refusal=(.*?) domain=", rest).group(1)
            elif kind == "I98_BYPASS_GB":
                r["paths"]["bypass"]["G-B"] = re.search(r"late_refusal=(.*?) exact_selected", rest).group(1)
            elif kind == "I98_BYPASS_GC":
                r["paths"]["bypass"]["G-C"] = field(rest, "complete")
            elif kind == "I98_BYPASS" and rest.startswith(f"{label} {mode} "):
                d = r["paths"]["bypass"]
                d["outcome"] = re.search(r"ran=(.*?) furthest_phase=", rest).group(1)
                d["phase"] = re.search(r"furthest_phase=(.*?) ms=", rest).group(1)
                ctx = None
    print("| Input | Mode | Verdict | Seed | D-5 | Path | W1 outcome | Furthest phase | Native | Refusal | Notices / reader |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for key in order:
        r = rows[key]
        for p in ("direct", "twin", "bypass"):
            d = r["paths"].get(p)
            if not d or "outcome" not in d:
                continue
            extra = ""
            if p == "direct":
                extra = f"{d.get('notices')} notice(s)" + (f"; RS {r.get('reader')}" if d.get("successor") == "true" else "")
            if p == "bypass":
                extra = f"G-A {d.get('G-A')}; G-B {d.get('G-B')}; G-C {d.get('G-C')}"
            print(f"| {key[0]} | {key[1]} | {r.get('verdict')} | {r.get('seed')} | {r.get('d5')} | {p} | {d.get('outcome')} | {d.get('phase')} | "
                  f"{d.get('native', '-')} | {d.get('refusal', '-')} | {extra} |")


if __name__ == "__main__":
    main()
