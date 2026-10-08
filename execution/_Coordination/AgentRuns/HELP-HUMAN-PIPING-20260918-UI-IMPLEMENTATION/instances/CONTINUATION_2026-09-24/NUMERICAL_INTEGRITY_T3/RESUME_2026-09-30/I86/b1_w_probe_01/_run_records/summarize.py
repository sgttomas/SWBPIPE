"""I86 B1-SW: summarize probe logs (I86_* lines) into one markdown table per log.

Usage: python3 summarize.py <log>... > summary.md
Per (input, mode): the ordinary route (status, results, verdict, reciprocal condition
estimate, K-D5 line, W2 range scaling), the seed at permitted_run, admission, the Direct W1
cause and phase, the native terminal on Direct, notices and byte checks, the successor's
Rust reader verdict, and the witness twin's outcome, native terminal and furthest phase.
"""
import json
import re
import sys


def field(line, key, end=" "):
    i = line.find(key + "=")
    if i < 0:
        return None
    rest = line[i + len(key) + 1:]
    if end is None:
        return rest
    j = rest.find(end)
    return rest if j < 0 else rest[:j]


def bracket(line, key):
    """The balanced [...] or {...} value after key=."""
    i = line.find(key + "=")
    if i < 0:
        return None
    s = line[i + len(key) + 1:]
    if not s or s[0] not in "[{":
        return field(line, key)
    depth, instr, esc = 0, False, False
    for n, ch in enumerate(s):
        if instr:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                instr = False
            continue
        if ch == '"':
            instr = True
        elif ch in "[{":
            depth += 1
        elif ch in "]}":
            depth -= 1
            if depth == 0:
                return s[: n + 1]
    return s


def native_summary(line):
    m = re.match(r"I86_NATIVE_OUTCOME (\w+)(?: reason=(\w+))?", line)
    if not m:
        return line[:80]
    kind, reason = m.group(1), m.group(2)
    attempts = re.findall(r"\((\d+), \"(\w+)\", \"(\w+)", line)
    ladder = ",".join(f"p{p}:{o}" for p, _, o in attempts)
    return f"{kind}" + (f"({reason})" if reason else "") + (f" [{ladder}]" if ladder else "")


def summarize(path):
    rows = {}
    order = []
    census = {}
    cur = None
    stage = None
    for raw in open(path, encoding="utf-8", errors="replace"):
        line = raw.rstrip("\n")
        # A test's first print shares its line with libtest's "test <name> ... " prefix.
        if line.startswith("test ") and " I86_" in line:
            line = line[line.index(" I86_") + 1:]
        if line.startswith("I86_CENSUS "):
            label = line.split()[1]
            census[label] = line
            continue
        if line.startswith("I86_BEGIN "):
            _, label, mode = line.split()[:3]
            cur = (label, mode)
            stage = "direct"
            if cur not in rows:
                order.append(cur)
                rows[cur] = {"native_direct": [], "native_witness": [], "refusal_direct": [], "refusal_witness": []}
            rows[cur]["input_sha"] = field(line, "input_sha")
            continue
        if cur is None:
            continue
        r = rows[cur]
        if line.startswith("I86_ORDINARY "):
            st = bracket(line, "status")
            try:
                r["mechanics"] = json.loads(st)["mechanics"]
            except Exception:
                r["mechanics"] = st
            r["results"] = field(line, "results")
            v = bracket(line, "published_verdicts")
            try:
                r["verdict"] = ",".join(c["solve_quality"] for c in json.loads(v))
            except Exception:
                r["verdict"] = v
            r["rcond"] = bracket(line, "rcond")
            d5 = bracket(line, "d5") or ""
            r["d5"] = "none" if d5 in ('["none"]', "[]") else re.sub(r"; doubled_correction.*?(trigger_ratio=[^;\"]*).*", r"; \1", d5)[:160]
            r["range"] = bracket(line, "range_scaling")
            r["plain_sha"] = (field(line, "plain_sha") or "")[:12]
        elif line.startswith("I86_SEEDS site=permitted_run"):
            seeds = bracket(line, "seeds")
            try:
                s = json.loads(seeds)[0]
                ini = s["initial"]
                r["seed"] = f"{ini['kind']}/{ini.get('outcome') or ini.get('tag') or ''}; w2={s['w2']['kind']}" + (f" b={s['w2'].get('force_scale_exponent')}" if s["w2"]["kind"] == "published" else "") + ("; D-5" if s.get("d5_ref") else "") + ("; load_row" if s.get("load_row_finding") else "")
            except Exception:
                r["seed"] = (seeds or "")[:120]
        elif line.startswith("I86_NATIVE_OUTCOME"):
            r["native_" + stage].append(native_summary(line))
        elif line.startswith("I86_CANDIDATE_REFUSAL") or line.startswith("I86_PREPARATION_FAILURE") or line.startswith("I86_PRECOMMIT_ERROR"):
            r["refusal_" + stage].append(line[:220])
        elif line.startswith("I86_ADMISSION "):
            r["admission"] = f"domain={field(line, 'domain')} refusal={field(line, 'refusal')} required={field(line, 'required')}"
        elif line.startswith("I86_W1 "):
            r["cause"] = field(line, "cause")
            r["phase"] = field(line, "phase", " counts")
            r["notices"] = field(line, "notices")
            r["eq_notice"] = field(line, "bytes_eq_with_notice_plain_case_none")
            r["successor"] = field(line, "published_successor")
            r["counts_ok"] = field(line, "one_run_through_g_c")
            r["pub_sha"] = (field(line, "published_sha") or "")[:12]
            stage = "witness"
        elif line.startswith("I86_SUCCESSOR "):
            r["reader"] = (field(line, "rust_reader", " invocation_bound") or "")
        elif line.startswith("I86_WITNESS "):
            r["witness"] = field(line, "ran", " furthest_phase")
            r["witness_phase"] = field(line, "furthest_phase", " ms=")
            stage = None
    out = [f"### {path.split('/')[-1]}", ""]
    for label, line in census.items():
        out.append(f"- **{label}** census: domain={field(line, 'domain')}; loads_per_case={bracket(line, 'loads_per_case')}; over={bracket(line, 'over')}; first_over_excluding_LoadCasesCapacity={field(line, 'first_over_excluding_LoadCasesCapacity')}; rows={bracket(line, 'rows')}")
    out.append("")
    out.append("| Input | Mode | Ordinary | Verdict | rcond | K-D5 line | Seed (permitted_run) | Admission | Direct W1 cause | Phase | Native (Direct) | Notices / = with_notice | Successor reader | Witness twin | Twin phase | Native (twin) |")
    out.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for key in order:
        r = rows[key]
        out.append("| " + " | ".join(str(x).replace("|", "/") for x in [
            key[0], key[1], f"{r.get('mechanics')}, {r.get('results')} results", r.get("verdict"), r.get("rcond"), r.get("d5"), r.get("seed", "-"),
            r.get("admission"), r.get("cause"), r.get("phase"), "; ".join(r["native_direct"]) or "-",
            f"{r.get('notices')} / {r.get('eq_notice')}", r.get("reader", "-"), r.get("witness"), r.get("witness_phase"), "; ".join(r["native_witness"]) or "-"]) + " |")
    for key in order:
        r = rows[key]
        for s in ("direct", "witness"):
            for x in r["refusal_" + s]:
                out.append(f"- {key[0]} {key[1]} {s}: `{x}`")
    return "\n".join(out)


if __name__ == "__main__":
    for p in sys.argv[1:]:
        print(summarize(p))
        print()
