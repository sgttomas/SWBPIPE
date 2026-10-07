# I81 B1-0: summarize the probe log (Direct entry) and the committed witness run, and classify
# each input under DESIGN_v2 T-4 (decision 1) and decision 21. Read-only over the logs.
import json, re, sys
EXCL = {"mechanism", "asymmetric", "invalid_input"}
def classify(verdicts, seeds, exact_selected):
    if exact_selected:
        return "T-3(c) coexistence (before T-4; no W1)"
    out = []
    for i, s in enumerate(seeds):
        v = verdicts[i]["solve_quality"] if i < len(verdicts) else None
        ini, w2 = s["initial"], s["w2"]
        if v == "checks_passed":
            out.append("not_required")
        elif ini.get("kind") == "structural_failure" and ini.get("tag") in EXCL and w2["kind"] != "published":
            out.append("excluded (decision 21)")
        else:
            out.append("A")
    return ",".join(out) if out else "no seed"
def short_initial(ini):
    k = ini.get("kind")
    if k == "report": return "report/" + {"NUMERICAL_INTEGRITY_SENSITIVE": "sensitive", "NUMERICAL_INTEGRITY_CHECKS_PASSED": "checks_passed"}.get(ini["outcome"], ini["outcome"])
    if k == "structural_failure": return "structural_failure/" + ini["tag"]
    if k == "formation_failure": return "formation_failure (" + ini["error"][:60] + ")"
    return k
def short_w2(w2):
    if w2["kind"] == "published": return "published b=%d" % w2["force_scale_exponent"]
    if w2["kind"] == "failed": return "failed (" + w2["failure"][:60] + ")"
    return w2["kind"]
def direct(path):
    rows, cur = [], None
    for raw in open(path):
        m = re.search(r'(I81_[A-Z0-9_]+) (.*)$', raw.rstrip("\n"))
        if not m: continue
        tag, rest = m.groups()
        if tag == "I81_BEGIN":
            label, mode = re.match(r'(.*) (sparse_interactive|dense_scrutiny) registered=', rest).groups()
            cur = {"input": label, "mode": mode, "sha": rest.split("input_sha=")[1], "w1_start": False, "native": None, "seeds_site": []}
            rows.append(cur)
        elif tag == "I81_ORDINARY":
            cur["mechanics"] = re.search(r'"mechanics":"([A-Z_]+)"', rest).group(1)
            cur["verdicts"] = json.loads(re.search(r'published_verdicts=(\[.*?\]) range_scaling', rest).group(1))
            cur["results"] = int(re.search(r'results=(\d+)', rest).group(1))
        elif tag == "I81_SEEDS":
            site = re.search(r'site=(\S+)', rest).group(1)
            cur["seeds_site"].append(site)
            if site == "permitted_run":
                cur["seeds"] = json.loads(rest.split(" seeds=", 1)[1])
                cur["exact"] = "exact_selected=true" in rest
                cur["q_at_run"] = json.loads(re.search(r'quality=(\[.*?\]) seeds=', rest).group(1))
        elif tag == "I81_W1_START": cur["w1_start"] = True
        elif tag == "I81_NATIVE_OUTCOME": cur["native"] = rest.split(" attempts=")[0]
        elif tag == "I81_PREPARATION_FAILURE": cur["prep"] = rest[:160]
        elif tag == "I81_CANDIDATE_REFUSAL": cur["cand"] = re.sub(r'^len=\d+ error=', '', rest)[:110]
        elif tag == "I81_PRECOMMIT_ERROR": cur["precommit"] = rest[:160]
        elif tag == "I81_ADMISSION": cur["refusal"] = re.search(r'refusal=(\S+)', rest).group(1); cur["domain"] = re.search(r'domain=(\S+)', rest).group(1)
        elif tag == "I81_W1":
            if "direct=Err" in rest: cur["cause"] = rest; continue
            cur["cause"] = re.search(r'cause=(.*?) counts=', rest).group(1)
            cur["notices"] = int(re.search(r'notices=(\d+)', rest).group(1))
            cur["eq_notice"] = "bytes_eq_with_notice_plain_case_none=true" in rest
            cur["eq_plain"] = "bytes_eq_plain=true" in rest
            cur["one_run"] = "one_run_through_g_c=true" in rest
        elif tag == "I81_SUCCESSOR": cur["reader"] = re.search(r'rust_reader=(\S+)', rest).group(1)
    return rows
def witness(path):
    out, seeds = [], None
    for raw in open(path):
        m = re.search(r'I81_SEEDS site=retained_w1 mechanics="([A-Z_]+)" exact_selected=(\w+) quality=(\[.*?\]) seeds=(.*)$', raw)
        if m:
            seeds = (m.group(1), m.group(2) == "true", json.loads(m.group(3)), json.loads(m.group(4)))
            continue
        m = re.search(r'I65_G5_WITNESS (.*?) stack=(\d+) ran=(.*)$', raw)
        if m:
            out.append((m.group(1), m.group(2), m.group(3), seeds)); seeds = None
    return out
if __name__ == "__main__":
    rows = direct(sys.argv[1])
    print("## Direct entry (probe log %s)" % sys.argv[1].split("/")[-1])
    print("| Input | Mode | Mechanics | Published verdict | Seed `initial` | Seed `w2` | Legacy | W1 runs | W1 outcome | Native | Notices | Bytes | T-4 class |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        s = r.get("seeds", [])
        v = ",".join(x["solve_quality"] for x in r["verdicts"]) or "(absent)"
        ini = ";".join(short_initial(x["initial"]) for x in s) or "(no seed)"
        w2 = ";".join(short_w2(x["w2"]) for x in s) or "-"
        leg = ";".join(x["legacy"] for x in s) or "-"
        bytes_ = "plain" if r.get("eq_plain") else ("with_notice" if r.get("eq_notice") else ("successor" if r.get("cause") == "Ok(successor)" else "other"))
        extra = r.get("cand") or r.get("prep") or r.get("precommit") or ""
        print("| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (r["input"], r["mode"].split("_")[0], r["mechanics"], v, ini, w2, leg,
            "yes" if r["w1_start"] else "no", r.get("cause"), (r["native"] or "-"), r.get("notices"), bytes_, classify(r["verdicts"], s, r.get("exact"))))
    print()
    print("## Detail lines")
    for r in rows:
        for k in ("prep", "cand", "precommit", "reader"):
            if k in r: print("- %s %s %s: %s" % (r["input"], r["mode"], k, r[k]))
        if not r.get("one_run", True): print("- %s %s: counts are not ONE_RUN_THROUGH_G_C" % (r["input"], r["mode"]))
        if r.get("q_at_run") != r["verdicts"]: print("- %s %s: verdicts at permitted_run differ from plain: %s" % (r["input"], r["mode"], r.get("q_at_run")))
    if len(sys.argv) > 2:
        print()
        print("## Committed witnesses (witness log %s)" % sys.argv[2].split("/")[-1])
        print("| Witness | Stack | Ran | Mechanics | Verdict | Seed `initial` | Seed `w2` | T-4 class (of the input) |")
        print("|---|---|---|---|---|---|---|---|")
        for name, stack, ran, seeds in witness(sys.argv[2]):
            if seeds is None:
                print("| %s | %s | %s | (no retained_w1 seed line before it) | | | | |" % (name, stack, ran)); continue
            mech, exact, q, s = seeds
            print("| %s | %s | %s | %s | %s | %s | %s | %s |" % (name, stack, ran, mech, ",".join(x["solve_quality"] for x in q),
                ";".join(short_initial(x["initial"]) for x in s), ";".join(short_w2(x["w2"]) for x in s), classify(q, s, exact)))
