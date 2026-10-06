#!/usr/bin/env python3
"""I77: build U8's citations.json in #1082's index format (u9-citation-index-v1).

Read-only on Git (GIT_OPTIONAL_LOCKS=0; `rev-parse`, `ls-tree`, `cat-file`). Standard library only.
Writes only --out.

Usage: python3 build_index.py --num <NUM checkout> --f1082 <main's F2A_D1 citations.json> --out <citations.json>

The tool-checked part (citations, documents, code_anchors) is what main's check_citations.py reads.
`named_references` is outside the tool: the tool has no class for review findings, decision ids,
instance attributions, commit ids, or a code-line citation wrapped across two comment lines.
verify_named.py checks those entries at the pinned NUM commit.
"""
import argparse, json, subprocess, os

ap = argparse.ArgumentParser()
ap.add_argument("--num", required=True); ap.add_argument("--f1082", required=True); ap.add_argument("--out", required=True)
a = ap.parse_args()
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
def git(*args):
    return subprocess.run(["git", "-C", a.num, *args], capture_output=True, env=env, check=True).stdout.decode()

NUM = git("rev-parse", "fd3990a710").strip()
HEAD = git("rev-parse", "bd6b4be2c3").strip()
BASE = git("rev-parse", "b1e2d7741e").strip()
P = "projects/chirality-piping/"
T3 = P + "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3"
R = T3 + "/RESUME_2026-09-30"
RR = T3 + "/ROOT_RULINGS_V1.md"
PP_TESTS = P + "core/product_physics/src/retained_facade_tests.rs"

rr = git("cat-file", "blob", f"{NUM}:{RR}").split("\n")
def heading(n):
    line = rr[n - 1]; assert line.startswith("## "), (n, line); return line[3:]

f1082 = json.load(open(a.f1082, encoding="utf-8"))

# PLAN: every T3 plan file named PLAN.md at the pin (sealed copies and run inputs excluded).
plans = sorted(p for p in git("ls-tree", "-r", "--name-only", NUM, "--", T3).split("\n")
               if p.endswith("/PLAN.md") and "/_run_records/" not in p and "/sealed_" not in p)
U8_PLAN = R + "/I61/u8_plan_01/PLAN.md"
assert U8_PLAN in plans

documents = dict(f1082["documents"])  # #1082's detection vocabulary, carried unchanged
documents["PLAN"] = {
    "aliases": ["PLAN"],
    "what": "a bare plan name; anchored citations only (PLAN §1.2, PLAN §1.3)",
    "candidates": plans,
    "rules": [{
        "file_prefix": PP_TESTS,
        "resolve": U8_PLAN,
        "why": ("U8's PP section opens by citing RR \"I61's U8 plan ruled…\", whose first line names R/I61/u8_plan_01/PLAN.md; "
                "§1.2 is \"The three witnesses\" (the L = 0 base and its \"Tests and controls\" row) and §1.3 \"The W-C2 construction\". "
                "The other candidates are other units' plans. Proposed by I77; ROOT confirms (as for #1082's COMP)")}],
    "note": "only PP's U8 section cites PLAN; no other citing file has a rule, so any other anchored PLAN citation would be listed as ambiguous"}
documents["SNAPSHOT_05_PLAN"] = {
    "aliases": ["SNAPSHOT_05_PLAN", "SNAPSHOT_05_PLAN.md"],
    "what": "I62's snapshot-05 plan (checkpoint C0); §1.2 is \"Absent kind, and L=0 against L≠0\", the source of 07l's isolated-node mutations",
    "path": R + "/I62/coverage_shared_python_01/SNAPSHOT_05_PLAN.md"}

citations = [
    {"class": "record_path", "token": "R/I68/u8_probe_01", "num_paths": ["I68/u8_probe_01", "I68/u8_probe_01/PROBE.md"],
     "note": "I68's U8-0 probe. Cited at PP retained_facade_tests.rs (the U8 header) and again with \"§3\" (U8_L0_PINNED): PROBE.md's \"## 3. Item 3: the L = 0 base\", which records the receipt and published-byte hashes the pins carry. The tool does not check the § anchor; I77 did (named_references)"},
    {"class": "rr_title", "token": "I61's U8 plan ruled", "heading_line": 11898, "heading": heading(11898),
     "note": "cited as RR \"I61's U8 plan ruled…\" (PP) and RR \"I61's U8 plan ruled ...\", decision 6 (Python); decisions 2 (W-C1/W-C2), 4 (tiny_spring), 6 (07l's claim) and 7 (U8's PR)"},
    {"class": "rr_title", "token": "I68's probe verified", "heading_line": 12344, "heading": heading(12344),
     "note": "cited as RR \"I68's probe verified…\" (PP, W-C1's input). The PP header's second title on the same line (… and \"I68's probe verified…\") carries no RR prefix, so the tool does not see it there; it is the same heading"},
]

ZZ = R + "/REVIEW_RV93/u3_grant2_01/evidence/probe/zz_rv93.rs"
zz = git("cat-file", "blob", f"{NUM}:{ZZ}").split("\n")
code_anchors = [{
    "citing_file": PP_TESTS, "token": "zz_rv93.rs:292–315", "kind": "record", "rev": NUM, "file": ZZ,
    "lines": [{"n": n, "text": zz[n - 1]} for n in (292, 296, 298, 299, 315)],
    "why": ("a record, not maintained code: RV93's probe module, fixed at the pinned NUM commit, so the lines cannot move. "
            "Lines 292–315 are fn zz_rv93_input_fallbacks, which builds tiny_spring (296) and first_load_only (298–299). "
            "PLAN §1.2 and I68's brief cite this exact path and range. An identical copy of these lines is in "
            "evidence/addendum_01/zz_rv93.rs (lines 292–315, same text). Pinned by I77; ROOT rules whether a record-file "
            "line citation stands as a pinned exception (#1082's exceptions were generated and corpus-data lines only)")}]

def nref(token, kind, cited_at, resolves, note, carried=False):
    e = {"token": token, "kind": kind, "cited_at": cited_at, "resolves_to": resolves, "note": note}
    if carried: e["carried"] = True
    return e
def rec(path, line=None, contains=None):
    d = {"num_path": path}
    if line: d["line"] = line
    if contains: d["contains"] = contains
    return d
def rrl(line, contains):
    return {"rr_line": line, "heading_line": max(i + 1 for i in range(line) if rr[i].startswith("## ")), "contains": contains}

named = [
    nref("R/I68/u8_probe_01 §3", "record_anchor", ["core/product_physics/src/retained_facade_tests.rs:931"],
         [rec("I68/u8_probe_01/PROBE.md", 88, "## 3. Item 3: the L = 0 base")], "the § anchor of the record_path citation"),
    nref("RV93 N-5", "review_finding",
         ["core/product_physics/src/retained_facade_tests.rs:832", ":836", ":893"],
         [rec("REVIEW_RV93/u3_grant2_01/REVIEW.md", 61, "| N-5 | NOTE |")],
         "W1 fallbacks from real inputs: first_load_only at Candidate, the 1e-300 spring at Preparation. \"RV93's Preparation sibling\" (:843) is the same row"),
    nref("D-U6-5", "decision",
         ["core/product_physics/src/retained_facade_tests.rs:834", ":1044", "tests/test_retained_precision_contract.py:903",
          "fixtures/results/retained_precision_cases.json:118181", ":125725"],
         [rec("I66/u6_scoping_01/PLAN.md", 306, "| **D-U6-5** |"), rrl(9456, "**D-U6-5:**")],
         "proposed in I66's U6 plan §7, adopted by RR \"U6 plan accepted: D-U6-1 to D-U6-9; …\". For an embedded corpus base, D-U6-5 is value equality with the pinned fixture (RR \"I69's corpus 07l committed; …\", reading 1)"),
    nref("decision 6", "decision", ["tests/test_retained_precision_contract.py:884", ":903"],
         [rrl(11919, "6. **Snapshot 07l:**")], "RR \"I61's U8 plan ruled…\" decision 6: producer-solved bases with case-level provenance (D-U6-5 copies) and the amended top-level claim"),
    nref("W-C1, W-C2", "decision", ["core/product_physics/src/retained_facade_tests.rs:832", ":850", ":870", ":893"],
         [rrl(11911, "2. **The Ceiling witness splits:**"), rrl(12378, "**W-C1 uses two-body case B,**")],
         "W-C1 (U8) is a real-input Native fallback inside D1; W-C2 (B1) is the receipt's Ceiling row. W-C1's input is two-body case B"),
    nref("U8-0 probe; I68", "instance_attribution",
         ["core/product_physics/src/retained_facade_tests.rs:829", ":833", "tests/test_retained_precision_contract.py:887",
          "apps/desktop/src/features/results/retainedPrecision.test.ts:988"],
         [rec("I68/u8_probe_01/PROBE.md", 1, "# I68 U8-0: the probe"), rec("I68/u8_witnesses_01/RETURN.md", 1, "# I68 U8-1")],
         "I68 wrote the probe (U8-0) and U8-1; \"the class counts all three readers observed (I68)\" is PROBE.md §3"),
    nref("I69", "instance_attribution", ["core/reporting/result_export/tests/retained_precision_contract.rs:551", ":553"],
         [rec("I69/u8_corpus_07l_01/RETURN.md", 1, "# I69 U8-2: corpus 07l")],
         "I69 built 07l and added the translation-plus-rotation stop pair (RETURN §2.2). The slice tag I70_OUTCOME_07L (:558) is an output tag, not a citation"),
    nref("W6's body; PHYS-R4's cantilever; retained_memory_witness_tests.rs :181–199", "code_line_wrapped",
         ["core/product_physics/src/retained_facade_tests.rs:851–852"],
         [{"repo_path": P + "core/product_physics/src/retained_memory_witness_tests.rs", "rev": HEAD, "line": 181,
           "contains": "pub(super) fn w6_input() -> Value {"},
          {"repo_path": P + "core/product_physics/src/retained_memory_witness_tests.rs", "rev": HEAD, "line": 199, "contains": "raw"}],
         ("a bare FILE:line citation of maintained code, the form #1082's rule forbids; the tool misses it because the line "
          "number wraps onto the next comment line. Lines 181–199 are fn w6_input() at the U8 head and on main c1bfc460fc "
          "(the same blob, 9745e3fe38ef). For ROOT: accept as accurate at the PR head, or name w6_input() at the file's next touch")),
    nref("U5 criterion", "record_method", ["core/product_physics/src/retained_facade_tests.rs:938", ":952", ":954", ":1002"],
         [rec("I61/u5_reference_01/_run_records/u5_compare.py", 104, 'if cls == "input_derived":')],
         "the comment states the criterion inline (relative within |v|/1e9, absolute within the published bound, input-derived exactly); u5_compare.py:104–113 is U5's per-class test, as I68's RETURN §2 cites it"),
    nref("u8_head d44909708529c6277fc1fd3b22997218296c8dd3", "commit",
         ["fixtures/results/retained_precision_cases.json:110680", ":118194", "tests/test_retained_precision_contract.py:926"],
         [{"commit": "d44909708529c6277fc1fd3b22997218296c8dd3", "subject": "piping(T3 U8-1): real-input fallback witnesses and the L = 0 producer-solved successor (test-only)",
           "reachable_from": "origin/codex/piping-f2a-u8-20261005 (pushed); not NUM fd3990a710; not main"}],
         ("the commit that added the two L = 0 fixtures (U8-1). The compact PR carries the 7 files, not the U8 branch's history, so "
          "after the merge this id resolves on the U8 branch and, once ROOT merges U8 into NUM, on NUM")),
    nref("RV94 N-3", "review_finding", ["core/reporting/result_export/tests/retained_precision_contract.rs:278", "tests/test_retained_precision_contract.py:640"],
         [rec("REVIEW_RV94/u7_01/REVIEW.md", 52, "| N-3 | NOTE |")], "the G7 probe that made 07k's mutation 278 (index 277)", carried=True),
    nref("D-U7-2", "decision", ["core/reporting/result_export/tests/retained_precision_contract.rs:207", "tests/test_retained_precision_contract.py:149", ":642"],
         [rrl(10885, "**D-U7-2:**")], "snapshot 07i's shared eligible expectation", carried=True),
    nref("RV90 S1, N1, N2, N4", "review_finding", ["core/reporting/result_export/tests/retained_precision_contract.rs:850", "tests/test_retained_precision_contract.py:642"],
         [rec("REVIEW_RV90/u6e_reader_round_01/REVIEW.md", 43, "| S1 |"), rec("REVIEW_RV90/u6e_reader_round_01/REVIEW.md", 44, "| N1 |"),
          rec("REVIEW_RV90/u6e_reader_round_01/REVIEW.md", 45, "| N2 |"), rec("REVIEW_RV90/u6e_reader_round_01/REVIEW.md", 47, "| N4 |")],
         "07h's items", carried=True),
    nref("C04", "review_finding", ["core/reporting/result_export/tests/retained_precision_contract.rs:850", "tests/test_retained_precision_contract.py:642"],
         [rrl(11041, "**C04, a coverage gap**"), rrl(11156, "**07j closes C04.**")], "07j's not_required must-pass entry", carried=True),
    nref("D11, D37", "decision", ["tests/test_retained_precision_contract.py:643"],
         [rrl(8066, "**D11. Corpus 07**"), rrl(8717, "**D37, D35 widened.**")], "the corpus's rehash rule and the D37 table", carried=True),
]

idx = {
    "schema": f1082["schema"],
    "about": ("Resolves every record, design-document and code-line citation that U8's compact PR adds to maintained source "
              "(the 7 files of b1e2d7741e..bd6b4be2c3). Records stay on the integration branch, pinned at num_commit; documents "
              "identical on main resolve there; code-line citations are forbidden except the pinned code_anchors. Verify with "
              "main's IMPLEMENTATION/F2A_D1/check_citations.py (#1082's tool, reused unchanged). The documents table carries #1082's "
              "23 names unchanged as detection vocabulary and adds PLAN and SNAPSHOT_05_PLAN. named_references lists the citations "
              "the tool has no class for (review findings, decision ids, instance attributions, a commit id, a wrapped code line). "
              "The tool ignores that key; R/I77/u8_package_01/_run_records/verify_named.py checks it. In named_references, "
              "cited_at paths are relative to projects/chirality-piping/ at source_basis (a bare :n repeats the previous file), "
              "num_path is relative to records_root at num_commit, and rr_line is a line of rr_path at num_commit"),
    "num_branch": f1082["num_branch"],
    "num_commit": NUM,
    "github": f1082["github"],
    "records_root": f1082["records_root"],
    "rr_path": f1082["rr_path"],
    "source_basis": HEAD,
    "source_base": BASE,
    "copies": {},
    "citations": citations,
    "documents": documents,
    "code_anchors": code_anchors,
    "named_references": named,
}
assert idx["records_root"] == R and idx["rr_path"] == RR
with open(a.out, "w", encoding="utf-8") as w:
    json.dump(idx, w, indent=1, ensure_ascii=False); w.write("\n")
print(f"wrote {a.out}: {len(citations)} citations, {len(documents)} documents ({len(plans)} PLAN candidates), "
      f"{len(code_anchors)} code anchor(s), {len(named)} named references")
