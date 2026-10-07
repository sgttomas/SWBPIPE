#!/usr/bin/env python3
"""I80: build T6S's citations.json (u9-citation-index-v1) for main's check_citations.py.

Read-only Git (GIT_OPTIONAL_LOCKS=0); standard library only. Writes only the --out file.
Usage: python3 build_index.py --repo <checkout with NUM objects> --num <NUM head> --base <slice base> \
    --head <slice head> --vocab <#1082's citations.json> --out <citations.json>

The documents table is #1082's 23 names unchanged (detection vocabulary, as S-I1 and U8 carry it).
`citations` holds the two tool-class citations the slice adds. `named_references` lists the citations
the tool has no class for; each entry's cited_at sites are found here by its site regex in the added
lines of base..head, and its targets are checked by verify_named_t6s.py.
"""
import argparse, json, os, re, subprocess, sys
ap = argparse.ArgumentParser()
for k in ("repo", "num", "base", "head", "vocab", "out"): ap.add_argument("--" + k, required=True)
a = ap.parse_args()
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def git(*args): return subprocess.run(["git", "-C", a.repo, *args], capture_output=True, env=env, check=True).stdout.decode()
P = "projects/chirality-piping/"
T3 = P + "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
num, base, head = (git("rev-parse", x).strip() for x in (a.num, a.base, a.head))
S = [n for n in git("diff", "--name-only", base, head, "--", ".", ":!" + P + "execution").split("\n") if n]
diff = git("diff", "-U0", base, head, "--", *S)
added = []
f = None; ln = 0
for line in diff.split("\n"):
    if line.startswith("+++ "): f = line[6:].replace(P, "") if line.startswith("+++ b/") else None; continue
    m = re.match(r"@@ -\S+ \+(\d+)(?:,\d+)? @@", line)
    if m: ln = int(m.group(1)); continue
    if line.startswith("+") and f: added.append((f, ln, line[1:])); ln += 1
def sites(rx): return [f"{f}:{n}" for f, n, t in added if re.search(rx, t)]
D2 = T3 + "DESIGN_STANDING/DESIGN.md"
NR = [
 dict(token="RR decisions 2, 3, 4, 5, 6, 11 and 12", kind="rr_decision", site_regex=r"RR decisions? [0-9]",
      resolves_to=[dict(rr_line=l, heading_line=12394, contains=c) for l, c in
                   ((12431, "| 2 |"), (12432, "| 3 |"), (12433, "| 4 |"), (12434, "| 5 |"), (12435, "| 6 |"), (12440, "| 11 |"), (12441, "| 12 |"))],
      note="The slice writes 'RR decision(s) N' without a heading. In T6S's files these are I74's decisions as ruled in the table of RR \"I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched\" (decision 2: the gate split and policy; 3: no activation switch; 4: Rust derive_document's form; 5: S-d; 6: standing and claims; 11: the test-built positive witness; 12: the reworded refusal text)"),
 dict(token="I74 PLAN 1.1, 1.2, 1.3 and 4.3", kind="document_section", site_regex=r"PLAN [0-9]",
      resolves_to=[dict(num_path="I74/t6_slice_plan_01/PLAN.md", line=l, contains=c) for l, c in
                   ((64, "### 1.1 The two panels and the shared gate"), (73, "### 1.2 Result export"), (82, "### 1.3 Stress-neutral export"), (262, "### 4.3 Staying consistent"))],
      note="Written without '§', so the tool's document class does not see them. 'PLAN 4.3' (retainedPrecisionDisclosure.ts) is the same plan by context. The plan's blob on main equals NUM's (#1101 carried it)"),
 dict(token="D2 4.9.4, 4.9.7 and 4.9.9", kind="document_section", site_regex=r"\bD2 4\.9\.[0-9]",
      resolves_to=[dict(repo_path=D2, rev=num, line=l, contains=c) for l, c in
                   ((596, "#### 4.9.4 The standing basis"), (622, "#### 4.9.7 Binding"), (639, "#### 4.9.9 Verified-accuracy classes"))],
      note="T3's design D2 (result standing). Written without '§', so outside the tool's document class. D2's blob on main equals NUM's"),
 dict(token="D-U6-2", kind="decision", site_regex=r"D-U6-2",
      resolves_to=[dict(num_path="I66/u6_scoping_01/PLAN.md", line=303, contains="| **D-U6-2** |"), dict(rr_line=9453, heading_line=9439, contains="**D-U6-2, option (A):**")],
      note="How the derivative discloses classes: absolute_verified and not_covered rows are disclosed, not valued, with the two reason codes"),
 dict(token="D-U6-5", kind="decision", site_regex=r"D-U6-5",
      resolves_to=[dict(num_path="I66/u6_scoping_01/PLAN.md", line=306, contains="| **D-U6-5** |"), dict(rr_line=9456, heading_line=9439, contains="**D-U6-5:**")],
      note="The pinned successor fixtures are byte-identical copies of PP's files, checked by sha256"),
 dict(token="D-U7-4", kind="decision", site_regex=r"D-U7-4",
      resolves_to=[dict(rr_line=10889, heading_line=10836, contains="**D-U7-4:**")], note="TS standing requires the live native capture"),
 dict(token="D-U7-6", kind="decision", site_regex=r"D-U7-6",
      resolves_to=[dict(rr_line=10891, heading_line=10836, contains="**D-U7-6, confirmed.**")], note="No producer-origin claim"),
 dict(token="CQ-1, CQ-4, CQ-5, CQ-7, CQ-10 and CQ-11", kind="plan_question", site_regex=r"\bCQ-(?:1|4|5|7|10|11)\b",
      resolves_to=[dict(num_path="I74/t6_slice_plan_01/PLAN.md", line=l, contains=c) for l, c in
                   ((168, "**CQ-1."), (184, "**CQ-4."), (186, "**CQ-5."), (193, "**CQ-7."), (210, "**CQ-10"), (217, "**CQ-11."))],
      note="I74's contract questions (PLAN §3), ruled through decisions 4, 6, 7, 10 and 11"),
 dict(token="RV101 SF-1", kind="review_finding", site_regex=r"RV101 SF-1",
      resolves_to=[dict(num_path="REVIEW_RV101/t6s_01/REVIEW.md", line=46, contains="| SF-1 | SHOULD-FIX |"),
                   dict(num_path="REVIEW_RV101/t6s_01/ADDENDUM_01.md", line=15, contains="## Verdict: **CONFIRMED**")],
      note="rustLowerExp on exact decimal ties; repaired by I75 (REPAIR_01) and confirmed by RV101 (ADDENDUM_01, with erratum E-1: ties occur at 16 and 17 digits)"),
 dict(token="REPAIR_01", kind="record", site_regex=r"REPAIR_01",
      resolves_to=[dict(num_path="I75/t6s_01/REPAIR_01.md", line=1, contains="# I75 T6S REPAIR_01")],
      note="I75's repair of SF-1, committed by ROOT as fdcdb5e024"),
 dict(token="I67's F4", kind="finding", site_regex=r"I67's F4",
      resolves_to=[dict(rr_line=9810, heading_line=9788, contains="**F4:**")], note="A header-only stress-neutral packet could not carry the receipt; noted for T6, closed by T6S-5"),
 dict(token="checkpoint readings R-1 and R-5", kind="ruling", site_regex=r"reading R-[15]\b",
      resolves_to=[dict(rr_line=12618, heading_line=12600, contains="**R-1, accepted.**"), dict(rr_line=12624, heading_line=12600, contains="**R-5, accepted.**")],
      note="I75's checkpoint readings, ruled in RR \"I75's checkpoint: S-1 granted; readings R-1 to R-7; …\""),
 dict(token="I76's CHECKPOINT_1 section 2", kind="record_anchor", site_regex=r"CHECKPOINT_1 section 2",
      resolves_to=[dict(num_path="I76/t6s_01/CHECKPOINT_1.md", line=28, contains="## 2. The fixed desktop-shaped base and origin")], note="I76's fixed stand-ins"),
 dict(token="RV91 N-5", kind="review_finding", site_regex=r"RV91 N-5",
      resolves_to=[dict(num_path="REVIEW_RV91/u6d_01/REVIEW.md", line=38, contains="| N-5 | NOTE |")],
      note="Carried in rewritten lines from main's text (U7 slice T)"),
 dict(token="RV95 N-5", kind="review_finding", site_regex=r"RV95 N-5",
      resolves_to=[dict(num_path="REVIEW_RV95/u9_01/REVIEW.md", line=52, contains="| N-5 | NOTE |")],
      note="The 2^53-1 integer bound; the public-API masking test is in the slice, the direct unit test goes with PR-B1 (decision 9)"),
 dict(token="F-U6c-2", kind="finding", site_regex=r"F-U6c-2",
      resolves_to=[dict(rr_line=9785, heading_line=9760, contains="**F-U6c-2,**")], note="The dispatcher's v0.3 branch knew only precision-1"),
 dict(token="T6S-1 to T6S-5; I75 and I76", kind="attribution", site_regex=r"T6S-[1-5]|\bI7[56]\b",
      resolves_to=[dict(num_path="I74/t6_slice_plan_01/PLAN.md", line=l, contains=f"| **T6S-{i}** |") for i, l in zip(range(1, 6), range(229, 234))]
                  + [dict(num_path="I76/t6s_01/RETURN.md", line=1, contains="# I76 RETURN"), dict(num_path="I75/t6s_01/RETURN.md", line=1, contains="# I75 RETURN")],
      note="The slice labels (PLAN §4) and the implementers' records"),
 dict(token="055ee0c0bc", kind="commit", site_regex=r"\b055ee0c0bc\b",
      resolves_to=[dict(commit="055ee0c0bca7d4f0c1ff5f5f6c8f1e6c1d5d5b0e", subject="", reachable_from="origin/codex/piping-t6-successor-outputs-20261005", ancestor_of_num=True, ancestor_of_main=False)],
      note="I76's commit, which holds the goldens. Reachable from the pushed T6 branch and from NUM (T6S merged at the pin); not from main. The compact PR carries the files, not this history"),
]
cid = git("rev-parse", "055ee0c0bc").strip()
NR[-1]["resolves_to"][0]["commit"] = cid
NR[-1]["resolves_to"][0]["subject"] = git("log", "-1", "--format=%s", cid).strip()
for e in NR:
    e["cited_at"] = sites(e["site_regex"])
    if not e["cited_at"]: sys.exit(f"no site for {e['token']}")
voc = json.load(open(a.vocab, encoding="utf-8"))
idx = {
 "schema": "u9-citation-index-v1",
 "about": ("Resolves every record and review citation that the T6 successor-output slice's compact PR (T6S) adds to maintained source "
           "(the 19 files of c1bfc460fc..fdcdb5e024; main's later commits through d8c88774d0 touch none of them). Records stay on the "
           "integration branch, pinned at num_commit. The slice's added lines cite no design document in the tool's document class: its "
           "D2 and I74 PLAN references are written without '§' or ':', so the documents table (#1082's 23 names, unchanged, as S-I1 and U8 carry it) "
           "finds none. Verify with main's IMPLEMENTATION/F2A_D1/check_citations.py (#1082's tool, unchanged). named_references lists the "
           "citations the tool has no class for; the tool ignores that key, and I80's verify_named_t6s.py checks it. cited_at paths are "
           "relative to projects/chirality-piping/ at source_basis; num_path is relative to records_root at num_commit; rr_line is a line "
           "of rr_path at num_commit; repo_path is read at rev"),
 "num_branch": "codex/piping-numerical-integrity-20260926",
 "num_commit": num,
 "github": voc["github"], "records_root": voc["records_root"], "rr_path": voc["rr_path"],
 "source_basis": head, "source_base": base,
 "copies": {},
 "citations": [
  {"class": "record_path", "token": "R/I76/t6s_01/inputs", "num_paths": ["I76/t6s_01/inputs"],
   "note": "I76's recorded golden inputs (the exact canonical base and origin files), cited in the result-export golden-parity test's header with the goldens' commit 055ee0c0bc (named_references). The folder's tree is the same on main"},
  {"class": "review_path", "token": "REVIEW_RV101/t6s_01/evidence/oracle/exp_differences.txt",
   "num_paths": ["REVIEW_RV101/t6s_01/evidence/oracle/exp_differences.txt"],
   "note": "RV101's nine exact-tie words, the first source of the {:e} test's Rust-computed vectors (SF-1). On NUM, and on main since records PR #1103 (d8c88774d0)"},
 ],
 "documents": voc["documents"],
 "code_anchors": [],
 "named_references": NR,
}
json.dump(idx, open(a.out, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
open(a.out, "a", encoding="utf-8").write("\n")
print(f"wrote {a.out}: num {num[:10]}, citations {len(idx['citations'])}, documents {len(idx['documents'])}, named_references {len(NR)}, sites {sum(len(e['cited_at']) for e in NR)}")
