#!/usr/bin/env python3
"""Build the mechanical R1 indexes (workflow contract "Artifacts and schemas").

Deterministic and read-only against the frozen commit. It writes:
  DELIVERABLE_INVENTORY.csv              one row per deliverable: package, name,
                                         lifecycle, representation, scope-surface
                                         hash, Remaining count, claim-key counts,
                                         canonical count, parity summary
  VERIFICATION_INDEX.csv                 one row per test file: suite (the gate
                                         surface that runs it), language, case
                                         count (static), and the gate record
                                         that covers the suite
  VALIDATION_AND_PROVENANCE_INDEX.csv    one row per validation or provenance
                                         asset, including the archived evidence
                                         records under validation/evidence/
                                         (kind `evidence_record`): family, path,
                                         kind
Static counts are syntax-based (test function and case declarations). They are
not pass counts; suite pass status comes only from GATE_EVIDENCE (CONVENTIONS A6).

Usage
  build_r1_indexes.py --repo-root . --run-dir <run> --commit <sha>
"""

from __future__ import annotations

import argparse
import collections
import csv
import hashlib
import re
import subprocess
import sys

P = "projects/chirality-piping/"


def git(repo, *a):
    return subprocess.run(["git", "-C", repo, *a], check=True, capture_output=True, text=True).stdout


def write(path, header, rows):
    with open(path, "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(header)
        w.writerows(rows)


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--commit", required=True)
    a = ap.parse_args(argv)
    c = git(a.repo_root, "rev-parse", a.commit).strip()
    files = [f for f in git(a.repo_root, "ls-tree", "-r", "-z", "--name-only", c, "--", P).split("\0") if f]
    show = lambda p: git(a.repo_root, "show", f"{c}:{p}")

    # ---- deliverable inventory
    keys = list(csv.DictReader(open(f"{a.run_dir}/CLAIM_KEYS_V2.csv", newline="", encoding="utf-8")))
    canon = collections.Counter(r["DeliverableID"] for r in csv.DictReader(open(f"{a.run_dir}/CANONICAL_ASSIGNMENTS.csv", newline="", encoding="utf-8")))
    emap = list(csv.DictReader(open(f"{a.run_dir}/EVIDENCE_MAP.csv", newline="", encoding="utf-8")))
    parity = collections.defaultdict(lambda: [0, "NO"])
    for r in emap:
        if r["ParityPath"] != "NONE_FOUND":
            parity[r["DeliverableID"]][0] += 1
        if r["AnyPassMatchesFrozen"] == "YES":
            parity[r["DeliverableID"]][1] = "YES"
    reg = {r["DeliverableID"]: r for r in csv.DictReader(show(P + "docs/_Registers/Deliverables.csv").splitlines())}
    by_del = collections.defaultdict(list)
    for k in keys:
        by_del[k["DeliverableID"]].append(k)
    rows = []
    for d in sorted(by_del):
        ks = by_del[d]
        folder = next(k["SourcePath"] for k in ks if k["Surface"] == "STATUS").rsplit("/", 1)[0] + "/"
        status = show(folder + "_STATUS.md")
        life = (re.search(r"\*\*Current State:\*\*\s*(\w+)", status) or [None, "UNKNOWN"])[1]
        surfaces = sorted({k["Surface"] for k in ks})
        rep = "SOW" if "SOW" in surfaces else "AB" if "AB" in surfaces else "BESPOKE"
        scope = next((k for k in ks if k["Surface"] in ("SOW", "AB") and k["UnitKind"] == "SURFACE"), None)
        scope_sha = hashlib.sha256(subprocess.run(["git", "-C", a.repo_root, "show", f"{c}:{scope['SourcePath']}"],
                                                  check=True, capture_output=True).stdout).hexdigest() if scope else ""
        req = [k for k in ks if k["Required"] == "YES"]
        rows.append([d, ks[0]["PackageID"], reg.get(d, {}).get("Name", ""), life, rep, folder, scope_sha,
                     sum(1 for k in ks if "#remaining/R" in k["ClaimKey"]), len(ks), len(req),
                     sum(1 for k in req if k["PreType"] == "NON_NORMATIVE"), canon.get(d, 0),
                     parity[d][0] if rep == "SOW" else "", parity[d][1] if rep == "SOW" else ""])
    write(f"{a.run_dir}/DELIVERABLE_INVENTORY.csv",
          ["DeliverableID", "PackageID", "Name", "Lifecycle", "Representation", "Folder", "ScopeSurfaceSHA256",
           "RemainingItems", "ClaimUnits", "RequiredUnits", "PreTypedNonNormative", "CanonicalAssigned",
           "ParityRecords", "AnyPassParityMatchesFrozen"], rows)

    # ---- verification index
    def suite_of(f):
        if f.endswith(".rs"):
            return "cargo_crate_sweep", "GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json"
        if f.startswith(P + "apps/desktop/e2e/"):
            return "desktop_playwright_e2e", "GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json;GATE:GATE_EVIDENCE/PR834_CI/HOSTED_SUMMARY.json"
        if f.startswith(P + "apps/desktop/"):
            return "desktop_vitest", "GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json"
        return "python_pytest", "GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json"
    tests = []
    for f in files:
        base = f.rsplit("/", 1)[-1]
        is_py = f.endswith(".py") and (base.startswith("test_") or base.endswith("_test.py"))
        is_ts = re.search(r"\.(test|spec)\.(ts|tsx|mjs|js)$", base)
        is_rs = f.endswith(".rs") and ("/tests/" in f or base == "lib.rs" or base.endswith(".rs"))
        if not (is_py or is_ts or is_rs):
            continue
        if f.startswith(P + "execution/"):
            continue
        body = show(f)
        if is_py:
            n = len(re.findall(r"^\s*def test_\w+", body, re.M)) + len(re.findall(r"^\s*async def test_\w+", body, re.M))
            lang = "python"
        elif is_ts:
            n = len(re.findall(r"\b(?:it|test)(?:\.each\([^)]*\))?\s*\(\s*['\"`]", body))
            lang = "typescript"
        else:
            n = len(re.findall(r"#\[(?:tokio::)?test\]", body))
            lang = "rust"
            if n == 0:
                continue
        suite, gate = suite_of(f)
        tests.append([f, suite, lang, n, gate])
    write(f"{a.run_dir}/VERIFICATION_INDEX.csv", ["TestPath", "Suite", "Language", "StaticCaseCount", "GateRecord"], tests)

    # ---- validation and provenance index
    val = []
    for f in files:
        rel = f[len(P):]
        if rel.startswith(("validation/", "provenance/", "docs/validation_manual/")):
            fam = "/".join(rel.split("/")[:3]) if rel.startswith("validation/evidence/") else "/".join(rel.split("/")[:2])
            kind = ("evidence_record" if rel.startswith("validation/evidence/") else
                    "benchmark" if "/benchmarks/" in f else "hand_calc" if "/hand_calcs/" in f else
                    "witness" if "/witness/" in f else "provenance" if rel.startswith("provenance/") else
                    "validation_manual" if rel.startswith("docs/validation_manual/") else "policy")
            val.append([f, fam, kind])
    write(f"{a.run_dir}/VALIDATION_AND_PROVENANCE_INDEX.csv", ["Path", "Family", "Kind"], val)
    print(f"deliverables {len(rows)}; test files {len(tests)} ({sum(t[3] for t in tests)} static cases); validation/provenance assets {len(val)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
