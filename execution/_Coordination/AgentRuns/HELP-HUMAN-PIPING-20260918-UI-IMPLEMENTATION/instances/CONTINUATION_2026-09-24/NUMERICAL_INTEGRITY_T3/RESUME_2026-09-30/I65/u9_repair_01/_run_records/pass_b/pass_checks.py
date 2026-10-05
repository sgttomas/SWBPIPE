"""I65 U4 G7 Pass B gates (RV89 G7 S-1(b)-(e), RV87 G7 SF-1, N-1): each subcommand checks one
condition, prints one JSON line and exits with its stop code (0 when the condition holds).
  tree <listing> <root>                  2  the copy differs from the revision's tree
  entry <target rm.rs> <reference rm.rs> 3  REGISTERED_PROFILES differs byte for byte (identity,
                                            inputs, layouts, threshold) from the registered entry
  law <law log> <target rm.rs>           3  the compiled identity/inputs/layouts differ from the
                                            entry, or a law test failed, or a registered test is absent
  premise <pins json> <root>             4  a pinned dead-branch premise line is not as reviewed
  forms <target rm.rs> <tree json> <g5_profile.py> <work dir>
                                         6  the generated block is not the regeneration of the tree
  text <run dir> <tag> <reference dir> <text_row_diff.py>
                                         6  a TEXT run is incomplete (unclassified, scc, self-recursion,
                                            id-unaudited, stale keys) or differs from the reference
  statics <statics json>                 6  a production static or embedded file added or removed
  noncand <compare json>                 6  the non-candidate set is not the reviewed 410
  controls <controls json> <n>           6  fewer than n controls pass
  outcomes <reference> <run>             6  a test outcome differs
"""
import difflib, json, os, re, shutil, subprocess, sys
cmd, args = sys.argv[1], sys.argv[2:]
def done(code, **kw):
    print(json.dumps({"check": cmd, "code": code, **kw}))
    sys.exit(code)
def entry_block(src):
    i = src.index("static REGISTERED_PROFILES")
    return src[i:src.index("\n}];", i) + 4]
if cmd == "tree":
    out = subprocess.run([sys.executable, os.path.join(os.path.dirname(os.path.abspath(__file__)), "tree_check.py"), *args], capture_output=True, text=True).stdout.strip()
    m = re.search(r"mismatched (\d+), extra (\d+)", out)
    done(0 if m and m.group(1) == "0" and m.group(2) == "0" else 2, result=out)
if cmd == "entry":
    a, b = entry_block(open(args[0]).read()), entry_block(open(args[1]).read())
    done(0 if a == b else 3, equal=a == b, threshold=re.search(r"threshold_bytes: ([\d_]+)", a).group(1),
         diff=[l for l in difflib.unified_diff(b.split("\n"), a.split("\n"), lineterm="")][:20])
if cmd == "law":
    L = open(args[0]).read(); e = entry_block(open(args[1]).read())
    try:
        ident = re.search(r"I65_G5_IDENTITY (\S+)", L).group(1); inputs = re.search(r"I65_G5_REVIEWED_INPUTS (\S+)", L).group(1)
        lay = [(int(s), int(a)) for _, s, a in re.findall(r"I65_G5_READER_LAYOUT (\w+) size=(\d+) align=(\d+)", L)]
    except AttributeError:
        done(3, reason="the law log carries no identity record (build or test failure)")
    res = {"identity_equal": ident == re.search(r'identity: "([^"]+)"', e).group(1),
           "inputs_equal": inputs == re.search(r'reviewed_inputs: "([^"]+)"', e).group(1) and "unavailable" not in inputs,
           "layouts_equal": lay == [(int(s), int(a)) for s, a in re.findall(r"TypeLayout \{ size: (\d+), align: (\d+) \}", e)],
           "summary": re.findall(r"^test result: .*$", L, re.M)}
    must = ["the_registered_profile_is_the_only_permit_source", "admit_grants_a_permit_for_the_milestone_in_the_registered_build",
            "registered_g_c_declines_only_unattempted_solves", "profile_in_build_record", "challenge_bounds_are_the_profile",
            "admission_bound_adds_r_before_comparing_with_m", "reviewed_inputs_bind_the_lock_and_the_reader_statics"]
    # ran (named in the run) and the run has no failure: with 0 failed, every test that ran passed
    res["registered_tests"] = {t: f"test retained_memory::law_tests::{t} ..." in L for t in must}
    passed = bool(res["summary"]) and all(re.search(r"ok\. \d+ passed; 0 failed", s) for s in res["summary"])
    ok = res["identity_equal"] and res["inputs_equal"] and res["layouts_equal"] and passed and all(res["registered_tests"].values())
    done(0 if ok else 3, **res)
if cmd == "premise":
    pins = json.load(open(args[0]))["pins"]; bad = []
    for p in pins:
        f, ln = p["key"].rsplit(":", 1)
        L = open(os.path.join(args[1], f), encoding="utf-8").read().split("\n")
        got = L[int(ln) - 1].strip() if int(ln) <= len(L) else ""
        if got != p["text"]: bad.append({"key": p["key"], "expected": p["text"], "found": got})
    done(4 if bad else 0, pins=len(pins), bad=bad)
if cmd == "forms":
    rm, tree, g5, wd = args
    regen = os.path.join(wd, "retained_memory.regen.rs"); shutil.copy(rm, regen)
    subprocess.run([sys.executable, g5, tree, regen], capture_output=True, check=True)
    a, b = open(rm).read(), open(regen).read()
    done(0 if a == b else 6, equal=a == b, diff=[l for l in difflib.unified_diff(a.split("\n"), b.split("\n"), lineterm="", n=0)][2:12])
if cmd == "text":
    run, tag, ref, rowdiff = args; issues = []
    for v in ("", "_W", "_X", "_env"):
        t = json.load(open(f"{run}/sens_{tag}/text_budget{v}.caps.out.json"))
        if not t["complete"] or t["unclassified_args"] or t["unmapped_loop_headers"]:
            issues.append({"run": v or "whole", "incomplete": t["unclassified_args"][:10], "unmapped": t["unmapped_loop_headers"][:5]})
        d = json.loads(subprocess.run([sys.executable, rowdiff, f"{ref}/text_budget{v}.caps.out.json", f"{run}/sens_{tag}/text_budget{v}.caps.out.json"],
                                      capture_output=True, text=True, check=True).stdout.split("\n")[0])
        if d["changed"] or d["unmatched_positive_in_first"] or d["only_in_second"] or d["tav"][0] != d["tav"][1] or d["D"][0] != d["D"][1]:
            issues.append({"run": v or "whole", "rows": d})
    for f in ("composite_text.caps.json", "g4_caps.caps.eps2.out.json", "ordinary_caps.caps.out.json", "producer_caps.caps.out.json",
              "profile_tree.json", "t07_repair.caps.out.json", "t08_closure.caps.log", "t25_g4.caps.eps2.out.json"):
        if open(f"{ref}/{f}", "rb").read() != open(f"{run}/sens_{tag}/{f}", "rb").read(): issues.append({"differs": f})
    ra, rb = json.load(open(f"{ref}/sens_g7.summary.json")), json.load(open(f"{run}/sens_{tag}.summary.json"))
    if ra != rb: issues.append({"differs": "summary"})
    done(6 if issues else 0, issues=issues, D=rb.get("D"))
if cmd == "statics":
    d = json.load(open(args[0])); done(6 if d["added"] or d["removed"] else 0, added=d["added"], removed=d["removed"])
if cmd == "noncand":
    d = json.load(open(args[0])); bad = d["new_noncandidates"] or d["rv87_rows_absent_now"] or d["run_rows"] != 410
    done(6 if bad else 0, run_rows=d["run_rows"], new=len(d["new_noncandidates"]), absent=len(d["rv87_rows_absent_now"]))
if cmd == "controls":
    d = json.load(open(args[0])); n = sum(1 for r in d if r["pass"])
    done(0 if n == int(args[1]) and len(d) == int(args[1]) else 6, passed=n, of=len(d), failing=[r["control"] for r in d if not r["pass"]])
if cmd == "outcomes":
    norm = lambda p: [re.sub(r" \(.*", "", l.rstrip("\n")) for l in open(p)]
    a, b = norm(args[0]), norm(args[1])
    done(0 if a == b and a else 6, lines=[len(a), len(b)], diff=[l for l in difflib.unified_diff(a, b, lineterm="", n=0)][2:12])
done(1, error="unknown check")
