#!/usr/bin/env python3
"""Build the evidence map and the canonical assignments (R0 ruling, item 1).

Deterministic and read-only against the frozen commit. It writes:
  EVIDENCE_MAP.csv           Scope-of-Work parity and claim-map records for each
                             deliverable, with whether the parity production hash
                             equals the frozen Scope of Work.
  CANONICAL_ASSIGNMENTS.csv  For each required claim key that matches a canonical
                             situation (see CANONICAL_SITUATIONS.md): the
                             situation ID, the mechanical check result where the
                             situation needs one, and the inherited fields.

Mechanical checks compare `_CONTEXT.md` restatements with the frozen companion
sources: the SOFTWARE_DECOMP package table, and docs/_Registers/Deliverables.csv
and ScopeLedger.csv. The situation definitions and their field values are Agent
0's judgments under the bound conventions, reviewed before use. Workers inherit
them and may depart only with a justification in Notes (CONVENTIONS C1).

Usage
  build_canonical.py --repo-root . --run-dir <run> --commit <sha>
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import re
import subprocess
import sys

P = "projects/chirality-piping"


def git(repo, *a):
    return subprocess.run(["git", "-C", repo, *a], check=True, capture_output=True, text=True).stdout


def sha_bytes(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


# Canonical field values: ClaimType, Disposition, CauseTag, AuthorityTier, BaselineClass,
# DivergenceLayers, AuthorityNeeded
CS = {
    "CS-01": ("DECLARED_STATE", "STALE_REVIEW_OR_EVIDENCE", "BASIS_POINTER_STALE", "LOCAL_DESIGN", "NONE", "RECORD", "NO"),
    "CS-02": ("HISTORY", "ALIGNED", "", "", "", "NONE", "NO"),
    "CS-03": ("HISTORY", "ALIGNED", "", "", "", "NONE", "NO"),
    "CS-04": ("DECLARED_STATE", "STALE_REVIEW_OR_EVIDENCE", "BASIS_POINTER_STALE", "LOCAL_DESIGN", "NONE", "RECORD", "NO"),
    "CS-05": ("DECLARED_STATE", "STALE_REVIEW_OR_EVIDENCE", "SCOPE_REDIRECTED_BY_RULING", "LOCAL_DESIGN", "NONE", "RECORD", "NO"),
    "CS-06-OK": ("DECLARED_STATE", "ALIGNED", "", "", "", "NONE", "NO"),
    "CS-06-DRIFT": ("DECLARED_STATE", "STALE_REVIEW_OR_EVIDENCE", "BASIS_POINTER_STALE", "LOCAL_DESIGN", "NONE", "RECORD", "NO"),
    "CS-07": ("NON_NORMATIVE", "NOT_ASSESSED", "", "", "", "NONE", "NO"),
}


def parse_md_table(text: str, first: str) -> dict[str, list[str]]:
    out = {}
    for line in text.split("\n"):
        if line.startswith(f"|{first}") or re.match(r"^\|(PKG|SOW|DEL|OBJ)-\d", line):
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            out.setdefault(cells[0], cells)
    return out


def field(block: str, name: str) -> str:
    m = re.search(r"\*\*" + re.escape(name) + r":\*\*\s*(.*)", block)
    return m.group(1).strip() if m else ""


def ids(text: str, prefix: str) -> set[str]:
    return set(re.findall(prefix + r"-\d{3}", text))


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--commit", required=True)
    a = ap.parse_args(argv)
    c = git(a.repo_root, "rev-parse", a.commit).strip()
    keys = list(csv.DictReader(open(f"{a.run_dir}/CLAIM_KEYS_V2.csv", newline="", encoding="utf-8")))

    # ---- evidence map
    tree = git(a.repo_root, "ls-tree", "-r", "-z", "--name-only", c).split("\0")
    parity = [t for t in tree if t.endswith("/parity.md") and "/checks/DEL-" in t]
    sow_path = {}
    for k in keys:
        if k["Surface"] == "SOW" and k["UnitKind"] == "SURFACE":
            sow_path[k["DeliverableID"]] = k["SourcePath"]
    emap = []
    for t in sorted(parity):
        d = re.search(r"/checks/(DEL-\d\d-\d\d)/", t).group(1)
        body = git(a.repo_root, "show", f"{c}:{t}")
        verdict = (re.search(r"Verdict:\s*\*\*(\w+)\*\*", body) or [None, ""])[1]
        prod = (re.search(r"Production Scope-of-Work SHA-256:\s*`([0-9a-f]{64})`", body) or [None, ""])[1]
        frozen = ""
        if d in sow_path:
            frozen = sha_bytes(subprocess.run(["git", "-C", a.repo_root, "show", f"{c}:{sow_path[d]}"],
                                              check=True, capture_output=True).stdout)
        cmap = t.replace("/parity.md", "/claim-map.csv")
        emap.append([d, t, verdict, prod, frozen, "YES" if prod and prod == frozen else "NO",
                     cmap if cmap in set(tree) else ""])
    with open(f"{a.run_dir}/EVIDENCE_MAP.csv", "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(["DeliverableID", "ParityPath", "Verdict", "ParityProductionSHA256", "FrozenSOWSHA256",
                    "HashMatchesFrozen", "ClaimMapPath"])
        w.writerows(emap)

    # ---- companion sources
    decomp = git(a.repo_root, "show", f"{c}:{P}/execution/_Decomposition/SOFTWARE_DECOMP.md")
    pkg_rows = {cells[0]: cells for cells in parse_md_table(decomp, "PKG").values() if cells[0].startswith("PKG-")}
    dreg = {r["DeliverableID"]: r for r in csv.DictReader(io.StringIO(git(a.repo_root, "show", f"{c}:{P}/docs/_Registers/Deliverables.csv")))}
    sreg = {}
    for r in csv.DictReader(io.StringIO(git(a.repo_root, "show", f"{c}:{P}/docs/_Registers/ScopeLedger.csv"))):
        sid = r.get("ScopeID") or r.get("ScopeItemID") or next(iter(r.values()))
        sreg[sid] = r

    # ---- canonical assignments
    rows = []
    cache: dict[str, list[str]] = {}
    for k in keys:
        if k["Required"] != "YES" or k["Surface"] != "CONTEXT" or k["UnitKind"] != "BLOCK" or k["PreType"]:
            continue
        slug = k["ClaimKey"].split("#", 1)[1]
        path = k["SourcePath"]
        if path not in cache:
            cache[path] = git(a.repo_root, "show", f"{c}:{path}").split("\n")
        block = "\n".join(cache[path][int(k["LineStart"]) - 1:int(k["LineEnd"])])
        d = k["DeliverableID"]
        sit, check = None, ""
        if slug.startswith("decomposition-reference") and re.search(r"Accepted Revision:\*\*\s*0\.(?!12\b)\d+", block):
            sit, check = "CS-01", "revision pin is not 0.12"
        elif slug.startswith("preparation-notes"):
            sit, check = "CS-02", ""
        elif re.match(r"sca-\d{3}-control-surface", slug):
            sit, check = "CS-03", ""
        elif slug.startswith("architecture-basis-injection") and re.search(r"revision 0\.(?!12\b)\d+", block):
            sit, check = "CS-04", "revision pin is not 0.12"
        elif slug.startswith("architecture-gate-rule"):
            sit, check = "CS-05", "PKG-00 gate superseded (D-43; HUMAN-STEER-PKG00-EXCLUSION-001)"
        elif slug.startswith("package-reference"):
            pk = (re.search(r"\*\*Package:\*\*\s*(PKG-\d\d)", block) or [None, ""])[1]
            cur = pkg_rows.get(pk)
            scope = field(block, "Package Scope")
            ok = bool(cur) and scope and any(scope == x for x in cur[1:])
            sit, check = ("CS-06-OK" if ok else "CS-06-DRIFT"), f"package scope {'matches' if ok else 'differs from'} SOFTWARE_DECOMP rev 0.12 table row {pk}"
        elif slug.startswith("objective-support"):
            ok = ids(block, "OBJ") == ids(dreg.get(d, {}).get("SupportsObjectives", ""), "OBJ")
            sit, check = ("CS-06-OK" if ok else "CS-06-DRIFT"), f"objectives {'match' if ok else 'differ from'} Deliverables.csv"
        elif slug.startswith("scope-coverage"):
            ok = ids(block, "SOW") == ids(dreg.get(d, {}).get("CoversScopeItems", ""), "SOW")
            sit, check = ("CS-06-OK" if ok else "CS-06-DRIFT"), f"scope items {'match' if ok else 'differ from'} Deliverables.csv"
        elif slug.startswith("context-envelope") or slug.startswith("context-budget-qa"):
            sit, check = "CS-07", "setup process metadata, no product claim"
        if sit:
            rows.append([k["ClaimKey"], d, sit.split("-OK")[0].split("-DRIFT")[0], sit, check, *CS[sit]])
    with open(f"{a.run_dir}/CANONICAL_ASSIGNMENTS.csv", "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(["ClaimKey", "DeliverableID", "CanonicalSituation", "Variant", "MechanicalCheck", "ClaimType",
                    "Disposition", "CauseTag", "AuthorityTier", "BaselineClass", "DivergenceLayers", "AuthorityNeeded"])
        w.writerows(rows)
    print(f"evidence map {len(emap)} parity records; canonical assignments {len(rows)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
