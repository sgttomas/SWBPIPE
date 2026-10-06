#!/usr/bin/env python3
"""I77: negative controls for U8's citations.json under main's check_citations.py.

Usage: python3 negative_controls.py <check_citations.py> <repo> <citations.json> <out dir>
Writes six altered indices and their check outputs to <out dir>; prints one line per control.
Each control must not PASS cleanly: nc1 lists ambiguity, nc2-nc6 exit 1.
"""
import copy, json, os, subprocess, sys

tool, repo, index, out = sys.argv[1:5]
os.makedirs(out, exist_ok=True)
base = json.load(open(index, encoding="utf-8"))
def drop_rule(d): d["documents"]["PLAN"].pop("rules")
def no_anchor(d): d["code_anchors"] = []
def num_without_u8(d): d["num_commit"] = "b1e2d7741e03b69427dd71045e024dfd2deff0f5"
def anchor_text(d): d["code_anchors"][0]["lines"][1]["text"] += " "
def rr_heading(d): d["citations"][1]["heading_line"] = 11966
def missing_entry(d): d["citations"] = [c for c in d["citations"] if c["token"] != "I68's probe verified"]
controls = [("nc1_plan_no_rule", drop_rule, 0), ("nc2_no_code_anchor", no_anchor, 1), ("nc3_num_without_u8_records", num_without_u8, 1),
            ("nc4_anchor_text", anchor_text, 1), ("nc5_rr_heading", rr_heading, 1), ("nc6_missing_entry", missing_entry, 1)]
ok = True
for name, alter, want in controls:
    d = copy.deepcopy(base); alter(d)
    p = os.path.join(out, name + ".json"); json.dump(d, open(p, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    r = subprocess.run([sys.executable, tool, "--repo", repo, "--base", "b1e2d7741e", "--head", "bd6b4be2c3", "--index", p, "--package", out],
                       capture_output=True, text=True)
    text = (r.stdout + r.stderr).replace(os.path.dirname(os.path.dirname(os.path.abspath(out))), "WT/scratch")
    open(os.path.join(out, name + ".out"), "w", encoding="utf-8").write(text)
    last = [l for l in text.splitlines() if l.startswith(("RESULT", "IndexError", "AMBIGUOUS"))]
    good = r.returncode == want and (name != "nc1_plan_no_rule" or "ambiguous 3" in text)
    ok &= good
    print(f"{name}: rc={r.returncode} (want {want}) {'as expected' if good else 'UNEXPECTED'}; {last[-1] if last else ''}")
    os.remove(p)
print("NEGATIVE CONTROLS", "ALL AS EXPECTED" if ok else "UNEXPECTED")
sys.exit(0 if ok else 1)
