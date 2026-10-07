"""I84 B1-P: a static reading of the reader-alignment cascade on a corpus snapshot.

Read-only. Usage: python3 cascade_static.py <corpus.json>
(the corpus bytes taken with `git show <rev>:projects/chirality-piping/fixtures/results/retained_precision_cases.json`).

It reports, for DESIGN_v2 §2 and §3.3's alignment:
- per base: case statuses, the linear_solver_mode_basis and parity rows per case,
  ordinary_attempts[].w2 kinds and requested modes (does each base already meet P1-P4?);
- the mutations expected at G8, and any with expected_by_reader;
- every mutation or must-pass entry that edits a mode or parity row, a requested_mode or
  a material_basis_ref, with its expected first failure (does any reach G8 today?);
- the mutations nearest to R-D38 (4b): native stage set to failed, or run_ref set to null.
It runs no reader. The mechanical census (each aligned reader over the snapshot) is B1's.
"""
import collections
import json
import sys

corpus = json.load(open(sys.argv[1]))
bases = {c["id"]: c for c in corpus["cases"]}
print("snapshot: bases", len(corpus["cases"]), "mutations", len(corpus["mutations"]), "must_pass", len(corpus["must_pass"]))

print("\n# bases")
for c in corpus["cases"]:
    src = c["source"]
    body = src.get("retained_precision", {}).get("body", {})
    rows = src.get("results", [])
    modes = collections.Counter((r.get("basis_ref") or {}).get("ref_id") for r in rows if r.get("kind") == "linear_solver_mode_basis")
    parity = collections.Counter((r.get("basis_ref") or {}).get("ref_id") for r in rows if r.get("kind") == "sparse_live_path_dense_parity_relative_delta")
    print(c["id"], "| cases", [x.get("status") for x in body.get("cases", [])],
          "| mode rows", dict(modes), "| parity rows", dict(parity),
          "| w2", [(x.get("w2") or {}).get("kind") for x in body.get("ordinary_attempts", [])],
          "| requested", [x.get("requested_mode") for x in body.get("ordinary_attempts", [])])

print("\n# G8 expectations and per-reader declarations")
for m in corpus["mutations"]:
    e = m.get("expected")
    if (isinstance(e, dict) and e.get("gate") == "G8") or "expected_by_reader" in m:
        print(m["id"], "|", m["base"], "|", e, "| by reader:", m.get("expected_by_reader"))


def touched(m):
    out = []
    for edit in m.get("edits", []):
        path = edit.get("path", [])
        text = "/".join(map(str, path))
        if "requested_mode" in text or "material_basis_ref" in text or "solver_mode" in text:
            out.append(text)
        if path and path[0] == "results" and len(path) > 1 and isinstance(path[1], int):
            rows = bases[m["base"]]["source"]["results"]
            if path[1] < len(rows) and rows[path[1]].get("kind") in ("linear_solver_mode_basis", "sparse_live_path_dense_parity_relative_delta"):
                out.append(text + " (" + rows[path[1]]["kind"] + ")")
        value = json.dumps(edit.get("value"))
        if "linear_solver_mode_basis" in value or "parity_relative_delta" in value:
            out.append(text + " (value names a mode or parity row)")
    return out


print("\n# entries editing mode or parity rows, requested_mode or material_basis_ref")
for m in corpus["mutations"] + corpus["must_pass"]:
    t = touched(m)
    if t:
        print(m["id"], "|", m["base"], "|", m.get("expected"), "|", t[:2])

print("\n# entries nearest R-D38 (4b)")
for m in corpus["mutations"]:
    for edit in m.get("edits", []):
        text = "/".join(map(str, edit.get("path", [])))
        if (text.endswith("/stages/native") and edit.get("value") == "failed") or (text.endswith("/run_ref") and edit.get("value") is None):
            print(m["id"], "|", m["base"], "|", m.get("expected"), "|", text, "=", json.dumps(edit.get("value")))
