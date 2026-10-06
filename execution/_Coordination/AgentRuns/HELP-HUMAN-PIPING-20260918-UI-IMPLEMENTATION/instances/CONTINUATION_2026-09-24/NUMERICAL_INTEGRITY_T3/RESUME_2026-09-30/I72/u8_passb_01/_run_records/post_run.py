"""I72 U8-4: gather the Pass B run's outputs into the records and compare them with I65's u4_g7_06 (F) and
u9_refreeze_01 (F') runs (stdlib only; read-only for every input). Machine paths are replaced by WT and ~.
Usage: I65_T=WT python3 post_run.py <tag> <records out dir>"""
import difflib, hashlib, json, os, re, shutil, subprocess, sys
T = os.environ["I65_T"].rstrip("/"); TAG, OUT = sys.argv[1:3]
R0 = T + "/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
REC = R0 + "/I65/u4_g7_06/_run_records"; FROZ = REC + "/runs/frozen"; REFZ = R0 + "/I65/u9_refreeze_01/_run_records/runs/refreeze"
O = f"{T}/scratch/i72_u8/pass_{TAG}"; L = f"{T}/scratch/i72_u8/logs"
S_FROZ = f"{T}/scratch/i65_u4_g7_01/pass_frozen"; S_REFZ = f"{T}/scratch/i65_u4_g7_01/pass_refreeze"
HOME = os.path.expanduser("~")
def clean(s): return s.replace(T, "WT").replace(HOME, "~")
def put(name, text): open(os.path.join(OUT, name), "w").write(clean(text))
def copy(src, name=None):
    put(name or os.path.basename(src), open(src, encoding="utf-8", errors="replace").read())
os.makedirs(OUT, exist_ok=True)
# 1. the run's own outputs (as I65 recorded runs/frozen)
for f in ["VERDICT.txt", "verdict.tsv", "D.txt", "delta.out.txt", "delta_inventory.json", "statics.json", "linemap.out.json",
          "price_delta.out.json", "noncand_compare.out.json", "controls.out.txt", "sens_pb.summary.json", "cargo_summary.txt"] + \
         sorted(x for x in os.listdir(O) if x.startswith("gate_")):
    if os.path.exists(f"{O}/{f}"): copy(f"{O}/{f}")
copy(f"{O}/ctl/audit_controls_g7.out.json")
copy(f"{T}/scratch/i72_u8/run.out.txt")
for f in [f"pass_{TAG}_pp.outcomes", f"pass_{TAG}_runner.outcomes"]: copy(f"{L}/{f}")
law = open(f"{O}/logs/law.log").read()
REC_LINES = lambda text: [m.group(0) for m in (re.search(r"I65_G5_\w.*", l) for l in text.split("\n")) if m]  # a record line may follow "test <name> ... "
put("law_record.txt", "# law-test record\n" + "\n".join(REC_LINES(law)) + "\n")
ch = open(f"{L}/pass_{TAG}_challenge.log").read()
put("challenge.txt", "\n".join(l for l in ch.split("\n") if l.startswith("I65_G5_CHALLENGE ") or l.startswith("test result")) + "\n")
# 2. the 11 reviewed entries against this run's rows
inv = json.load(open(f"{O}/delta_inventory.json")); reviewed = json.load(open(REC + "/delta_reviewed.json"))["entries"]
rows = []
for e in reviewed:
    m = [r for r in inv["rows"] if r.get("fingerprint") == e["fingerprint"] and r.get("reviewed")]
    rows.append({"fingerprint": e["fingerprint"], "file": os.path.basename(e["file"]), "class": e["class"],
                 "matched_row": "; ".join(f"{os.path.basename(r['file'])} {r.get('new_lines', '')}" for r in m) or None})
put("reviewed_match.json", json.dumps({"entries": len(reviewed), "matched": sum(1 for r in rows if r["matched_row"]),
                                       "unmatched": [r["fingerprint"] for r in rows if not r["matched_row"]], "rows": rows}, indent=1) + "\n")
# 3. delta rows against F (u4_g7_06) and F' (u9_refreeze_01)
def key(r): return json.dumps({k: r.get(k) for k in ("file", "new_lines", "class", "fingerprint", "reason")}, sort_keys=True)
out = []
for name, p in [("u4_g7_06 (F)", FROZ + "/delta_inventory.json"), ("u9_refreeze_01 (F')", REFZ + "/delta_inventory.json")]:
    o = json.load(open(p)); a = {key(r) for r in o["rows"]}; b = {key(r) for r in inv["rows"]}
    out.append(f"# this run ({inv['files']} files, {inv['hunks_and_files']} rows) against {name} ({o['files']} files, {o['hunks_and_files']} rows)")
    out.append(f"classes now {json.dumps(inv['classes'], sort_keys=True)}; then {json.dumps(o['classes'], sort_keys=True)}")
    out += ["ADDED   " + k for k in sorted(b - a)] + ["REMOVED " + k for k in sorted(a - b)]
    rv = lambda d: sorted((r["fingerprint"], (r.get("reviewed") or {}).get("fingerprint")) for r in d["rows"] if r.get("reviewed"))
    out.append(f"reviewed rows identical (fingerprint and entry): {rv(o) == rv(inv)}"); out.append("")
put("delta_rows_vs_F_and_Fprime.txt", "\n".join(out))
# 4. the U8 rows read in full (every row whose file U8 touched or added)
u8 = subprocess.run(["git", "-C", T + "/numerics", "diff", "--name-only", "b1e2d7741e", "bd6b4be2c3", "--", "projects/chirality-piping"],
                    capture_output=True, text=True, check=True, env=dict(os.environ, GIT_OPTIONAL_LOCKS="0")).stdout.split()
u8 = [f[len("projects/chirality-piping/"):] for f in u8]
put("u8_rows.json", json.dumps({"u8_files": u8, "rows": [r for r in inv["rows"] if r["file"] in u8],
                                "rows_classed_other_than_test_or_not_d1": [r for r in inv["rows"] if r["file"] in u8 and r["class"] not in ("test", "not-d1")]}, indent=1) + "\n")
# 5. outcomes: against Pass A's reference (in full) and against F's run
norm = lambda p: [re.sub(r" \(.*", "", l.rstrip("\n")) for l in open(p)]
cmp = []
for what, ref_a, ref_f, mine in [("PP", REC + "/reference/a_pp.outcomes", FROZ + "/pass_frozen_pp.outcomes", f"{L}/pass_{TAG}_pp.outcomes"),
                                 ("runner/headless", REC + "/reference/a_runner.outcomes", FROZ + "/pass_frozen_runner.outcomes", f"{L}/pass_{TAG}_runner.outcomes")]:
    m = norm(mine)
    for nm, ref in [("Pass A reference", ref_a), ("u4_g7_06 (F)", ref_f)]:
        r = norm(ref); d = [l for l in difflib.unified_diff(r, m, lineterm="", n=0)][2:]
        cmp.append(f"# {what} against {nm}: {len(r)} -> {len(m)} lines; " + ("identical" if r == m else f"{sum(1 for l in d if l.startswith('+'))} added, {sum(1 for l in d if l.startswith('-'))} removed"))
        cmp += [l for l in d if not l.startswith("@@")]
    cmp.append(f"{what} now: {sum(1 for l in m if l.endswith(' ok'))} passed, {sum(1 for l in m if 'FAILED' in l)} failed, {sum(1 for l in m if 'ignored' in l)} ignored")
    cmp += ["  FAILED: " + l for l in m if "FAILED" in l]; cmp.append("")
wit = lambda p: [l for l in open(p).read().split("\n") if l.startswith("I65_G5_WITNESS") or l.startswith("I65_G5_CHALLENGE")]
wf = wit(FROZ + "/cargo_summary.txt"); wm = wit(f"{O}/cargo_summary.txt")
cmp.append(f"# witness and challenge lines against u4_g7_06 (F): " + ("identical" if wf == wm else "DIFFER"))
cmp += [l for l in difflib.unified_diff(wf, wm, lineterm="", n=0)][2:]
lf = [l for l in open(FROZ + "/law_record.txt").read().split("\n") if l.startswith("I65_G5_")]
lm = REC_LINES(law)
tests = lambda text: [re.sub(r" \.\.\. (ok|FAILED|ignored).*", r" ... \1", l) for l in text.split("\n") if l.startswith("test ")]
tf = tests(open(S_FROZ + "/logs/law.log").read()) if os.path.exists(S_FROZ + "/logs/law.log") else None; tm = tests(law)
cmp.append(f"# law run test lines ({len(tm)}) against u4_g7_06 (F) pass_frozen law.log ({len(tf) if tf is not None else 'absent'}): " + ("identical" if tf == tm else "DIFFER"))
cmp += [l for l in difflib.unified_diff(tf or [], tm, lineterm="", n=0)][2:]
cmp.append(f"# law record against u4_g7_06 (F): " + ("identical" if lf == lm else "DIFFER"))
cmp += [l for l in difflib.unified_diff(lf, lm, lineterm="", n=0)][2:]
cmp.append("maxima now: " + "; ".join(re.findall(r"I65_G5_PROFILE mode=(\w+) .*?fraction_of_M=([\d.]+)", law) and
                                      [f"{a} {b} M" for a, b in re.findall(r"I65_G5_PROFILE mode=(\w+) .*?fraction_of_M=([\d.]+)", law)]))
put("vs_u4_g7_06.txt", "\n".join(cmp) + "\n")
# 6. every output file, byte for byte, against I65's scratch outputs for F and F'
for nm, other in [("frozen", S_FROZ), ("refreeze", S_REFZ)]:
    if os.path.isdir(other):
        res = subprocess.run([sys.executable, os.path.join(os.path.dirname(os.path.abspath(__file__)), "compare_outputs.py"), O, other], capture_output=True, text=True, check=True).stdout
        put(f"outputs_vs_{nm}.txt", f"# pass_{TAG} against I65's pass_{nm} (WT/scratch/i65_u4_g7_01/pass_{nm}), byte for byte; work/, tmp/, rr/, logs/ skipped\n" + res)
print(open(os.path.join(OUT, "VERDICT.txt")).read().strip())
