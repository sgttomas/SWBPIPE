#!/usr/bin/env python3
"""Build the evidence map and the canonical assignments (R0 ruling, item 1).

Deterministic and read-only against the frozen commit. It writes:
  EVIDENCE_MAP.csv           Every Scope-of-Work parity record in the frozen tree
                             (found by content signature, whatever its file name),
                             its verdict and production hash, whether that hash
                             equals the frozen Scope of Work, and a per-deliverable
                             `AnyPassMatchesFrozen` summary.
  CANONICAL_ASSIGNMENTS.csv  For each required claim key that matches a canonical
                             situation (see CANONICAL_SITUATIONS.md): the
                             situation ID, the mechanical check result where the
                             situation needs one, and the inherited fields.

Mechanical checks compare `_CONTEXT.md` restatements with the frozen companion
sources: every field of the SOFTWARE_DECOMP package table row (name, scope,
assigned scope items, exclusions), docs/_Registers/Deliverables.csv
(objectives, scope items) and docs/_Registers/ScopeLedger.csv (scope-item
statements). The situation definitions and their field values are Agent
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

    # ---- evidence map: every parity record, found by content signature
    sow_path = {k["DeliverableID"]: k["SourcePath"] for k in keys if k["Surface"] == "SOW" and k["UnitKind"] == "SURFACE"}
    frozen = {d: sha_bytes(subprocess.run(["git", "-C", a.repo_root, "show", f"{c}:{p}"], check=True,
                                          capture_output=True).stdout) for d, p in sow_path.items()}
    sig = "Production Scope-of-Work SHA-256"
    hits = subprocess.run(["git", "-C", a.repo_root, "grep", "-l", sig, c, "--", "execution/_Coordination/AgentRuns",
                           f"{P}/execution"], capture_output=True, text=True).stdout.split("\n")
    tree = set(git(a.repo_root, "ls-tree", "-r", "-z", "--name-only", c).split("\0"))
    emap = []
    for h in sorted(x.split(":", 1)[1] for x in hits if x):
        body = git(a.repo_root, "show", f"{c}:{h}")
        m = re.search(r"(DEL-\d\d-\d\d)", h) or re.search(r"(DEL-\d\d-\d\d)", body)
        if not m:
            continue
        d = m.group(1)
        for block in re.split(r"(?=Verdict:)", body)[1:] or [body]:
            verdict = (re.search(r"Verdict:\s*\*{0,2}(\w+)", block) or [None, ""])[1]
            prod = (re.search(sig + r":\s*`([0-9a-f]{64})`", block) or [None, ""])[1]
            if not prod:
                continue
            cmap = re.sub(r"parity[^/]*\.md$", "claim-map.csv", h)
            emap.append([d, h, verdict, prod, frozen.get(d, ""), "YES" if prod == frozen.get(d) else "NO",
                         cmap if cmap in tree else ""])
    anymatch = {d for d, _h, v, _p, _f, mt, _c in emap if v == "PASS" and mt == "YES"}
    with open(f"{a.run_dir}/EVIDENCE_MAP.csv", "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(["DeliverableID", "ParityPath", "Verdict", "ParityProductionSHA256", "FrozenSOWSHA256",
                    "HashMatchesFrozen", "ClaimMapPath", "AnyPassMatchesFrozen"])
        for r in emap:
            w.writerow(r + ["YES" if r[0] in anymatch else "NO"])
        for d in sorted(set(sow_path) - {r[0] for r in emap}):
            w.writerow([d, "NONE_FOUND", "", "", frozen[d], "NO", "", "NO"])

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
            cur = pkg_rows.get(pk)  # PackageID | Name | Scope Description | Assigned Scope Items | Exclusions
            name = re.sub(r"^PKG-\d\d\s*", "", field(block, "Package")).strip()
            parts = {
                "name": bool(cur) and name == cur[1],
                "scope": bool(cur) and field(block, "Package Scope") == cur[2],
                "assigned": bool(cur) and ids(field(block, "Package Assigned Scope Items"), "SOW") == ids(cur[3], "SOW")
                            and bool(ids(cur[3], "SOW")),
                "exclusions": bool(cur) and field(block, "Package Exclusions") == cur[4],
            }
            bad = [k for k, v in parts.items() if not v]
            sit = "CS-06-OK" if not bad else "CS-06-DRIFT"
            check = f"{pk} matches SOFTWARE_DECOMP rev 0.12 on name, scope, assigned items and exclusions" if not bad \
                else f"{pk} differs from SOFTWARE_DECOMP rev 0.12 on {', '.join(bad)}"
        elif slug.startswith("objective-support"):
            ok = ids(block, "OBJ") == ids(dreg.get(d, {}).get("SupportsObjectives", ""), "OBJ")
            sit, check = ("CS-06-OK" if ok else "CS-06-DRIFT"), f"objectives {'match' if ok else 'differ from'} Deliverables.csv"
        elif slug.startswith("scope-coverage"):
            ok = ids(block, "SOW") == ids(dreg.get(d, {}).get("CoversScopeItems", ""), "SOW")
            sit, check = ("CS-06-OK" if ok else "CS-06-DRIFT"), f"scope items {'match' if ok else 'differ from'} Deliverables.csv"
        elif slug.startswith("scope-detail"):
            # split on every "- SOW-NNN:" occurrence, including items run together on one line
            body = block.split("\n", 1)[1] if "\n" in block else ""
            parts = re.split(r"(?:^|\s*)-\s*(SOW-\d{3}):\s*", body)
            items = [(parts[i], parts[i + 1].strip()) for i in range(1, len(parts) - 1, 2)]
            joined = any(re.search(r"\S-\s*SOW-\d{3}:", ln) for ln in body.split("\n"))
            bad = [sid for sid, txt in items if sreg.get(sid, {}).get("ScopeItemStatement", "").strip() != txt.strip()]
            if items and not bad and not joined:
                sit, check = "CS-06-OK", "scope-item statements match ScopeLedger.csv"
            elif items and bad:
                sit, check = "CS-06-DRIFT", f"scope-item statements differ from ScopeLedger.csv: {', '.join(bad)}"
            else:
                sit = None  # statements match but items are run together, or nothing parsed: judged normally
        elif slug.startswith("context-budget-qa"):
            sit, check = "CS-07", "setup context-budget metadata, no product claim"
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
