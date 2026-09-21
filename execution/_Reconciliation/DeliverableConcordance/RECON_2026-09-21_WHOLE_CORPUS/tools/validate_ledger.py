#!/usr/bin/env python3
"""Structural validator for RECON_2026-09-21_WHOLE_CORPUS ledgers.

Checks a forward ledger (and optionally the reverse answers) for one
deliverable against the candidate schema in R0_CALIBRATION/CANDIDATE_CONVENTIONS.md
and the claim keys in CLAIM_KEYS.csv. Deterministic and read-only. Exit 0 =
PASS, 1 = findings, 2 = input error. Structural validity is not semantic
correctness; the package verifier judges that.

Usage
  validate_ledger.py --run-dir <run> --repo-root <repo> --deliverable DEL-XX-YY \
      --forward <file> [--reverse <file> --inventory <file>]
"""

from __future__ import annotations

import argparse
import csv
import io
import re
import subprocess
import sys

FREEZE = "00115c71931bcae79909602d653740d3bb72dfa1"
FORWARD_FIELDS = ["ClaimKey", "DeliverableID", "UnitKind", "ClaimType", "ClaimClass", "ClaimSummary",
                  "NormativeSource", "DecisionBasis", "DeclaredState", "RecordedRemaining",
                  "ImplementationEvidence", "VerificationEvidence", "VerificationClass",
                  "ValidationEvidence", "SourceReliability", "LifecycleState", "Disposition",
                  "Confidence", "CauseTag", "AuthorityTier", "BaselineClass", "DivergenceLayers",
                  "ContextRefs", "RemainingWork", "AuthorityNeeded", "SelectableUnderCurrentLoop",
                  "Notes", "SourceStateSHA"]
REVERSE_FIELDS = ["CapabilityID", "Answer", "ClaimKey", "Reason"]
V = {
    "UnitKind": {"SURFACE", "BLOCK", "ITEM", "SUBCLAIM"},
    "ClaimType": {"REQUIREMENT", "ACCEPTANCE", "EXCLUSION", "DECLARED_STATE", "REMAINING_WORK",
                  "CONTEXT", "CONTAINER", "NON_NORMATIVE", "HISTORY"},
    "ClaimClass": {"GOVERNANCE", "SCHEMA", "MECHANICS", "WORKFLOW", "GUI", "REPORTING", "INTEROP",
                   "VALIDATION", "SECURITY", "DOCUMENTATION"},
    "VerificationClass": {"UNIT", "BROWSER_E2E", "NATIVE", "STATIC_CHECK", "NONE"},
    "SourceReliability": {"VETTED", "REVIEWED", "UNVERIFIED", "NOT_APPLICABLE"},
    "Disposition": {"ALIGNED", "IMPLEMENTED_UNDOCUMENTED", "DOCUMENTED_UNIMPLEMENTED",
                    "PARTIALLY_IMPLEMENTED", "IMPLEMENTED_DIFFERENTLY", "ACCEPTED_DIVERGENCE",
                    "LIFECYCLE_REASSESSMENT_REQUIRED", "DEFERRED_AGENT_WORKFLOW", "AUTHORITY_CONFLICT",
                    "UNKNOWN", "STALE_INPUT", "STALE_SETUP_SPECIFICATION", "STALE_REVIEW_OR_EVIDENCE",
                    "VERIFIED_NOT_VALIDATED", "REMAINING_STATE_MISMATCH",
                    "ENGINEERING_AUTHORITY_REQUIRED", "COVERED_BY_CHILDREN", "NOT_ASSESSED"},
    "Confidence": {"HIGH", "MEDIUM", "LOW"},
    "CauseTag": {"", "DOC_BEHIND_CODE", "SCOPE_GREW_BY_DIRECTION", "SCOPE_REDIRECTED_BY_RULING",
                 "REDESIGN_SUPERSEDED", "RENAME_OR_IDENTITY", "CONTRACT_VERSION_ADVANCED",
                 "OWNERSHIP_ELSEWHERE", "PARTIAL_SLICE", "NOT_STARTED", "DEFERRED_BY_RULING",
                 "EVIDENCE_OVERTAKEN", "VALIDATION_GAP", "RECORD_DRIFT", "AUTHORITY_UNCLEAR",
                 "POSSIBLE_DEFECT", "OTHER"},
    "AuthorityTier": {"", "LOCAL_DESIGN", "PROJECT_BASELINE", "INVARIANT"},
    "BaselineClass": {"", "ISSUED", "FROZEN_CONTRACT", "PROTECTED_CHECK", "RULED_CRITERION",
                      "OWNER_HOLD", "NONE"},
}
LAYERS = {"CLAIMS", "VALIDATION", "IP_DATA", "BASELINE", "SECURITY", "LIFECYCLE", "NONE"}
QUIET = {"ALIGNED", "COVERED_BY_CHILDREN", "NOT_ASSESSED"}
AUTH_RE = re.compile(r"^(NO|OWNER|ENGINEERING|SCOPE_CHANGE|REVIEW|D-[0-9A-Za-z-]+|DEC-\d{3})$")
PATH_TOKEN_RE = re.compile(r"^(projects/|tools/|docs/|workflows/|\.github/|_DomainEngines/)")
SPECIAL = {"NONE_FOUND", "NOT_APPLICABLE", ""}


def load(path: str, fields: list[str], findings: list[str]) -> list[dict]:
    raw = open(path, "rb").read()
    first = raw.split(b"\n", 1)[0]
    if not first.endswith(b"\r") or not raw.endswith(b"\r\n"):
        findings.append(f"{path}: records must end with CRLF")
    rows = list(csv.reader(io.StringIO(raw.decode("utf-8"), newline="")))
    if not rows or rows[0] != fields:
        findings.append(f"{path}: header mismatch; expected {len(fields)} named columns")
        return []
    out = []
    for n, r in enumerate(rows[1:], start=2):
        if len(r) != len(fields):
            findings.append(f"{path}: record {n} has {len(r)} fields, expected {len(fields)}")
            continue
        out.append(dict(zip(fields, r)))
    if not out or out[-1][fields[0]] != "#END":
        findings.append(f"{path}: missing #END sentinel as final record")
        return out
    body, end = out[:-1], out[-1]
    note_field = "Notes" if "Notes" in fields else "Reason"
    if end[note_field].strip() != str(len(body)):
        findings.append(f"{path}: sentinel count {end[note_field]!r} != {len(body)} body records")
    return body


def path_exists(repo: str, token: str, cache: dict[str, bool]) -> bool:
    p = re.split(r"::|#L", token, maxsplit=1)[0].strip().rstrip("/")
    if p in cache:
        return cache[p]
    ok = subprocess.run(["git", "-C", repo, "cat-file", "-e", f"{FREEZE}:{p}"],
                        capture_output=True).returncode == 0
    cache[p] = ok
    return ok


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--deliverable", required=True)
    ap.add_argument("--forward", required=True)
    ap.add_argument("--reverse")
    ap.add_argument("--inventory")
    a = ap.parse_args(argv)
    f: list[str] = []
    keys = {r["ClaimKey"]: r for r in csv.DictReader(open(f"{a.run_dir}/CLAIM_KEYS.csv", newline="", encoding="utf-8"))
            if r["DeliverableID"] == a.deliverable}
    if not keys:
        print(f"no claim keys for {a.deliverable}", file=sys.stderr)
        return 2
    rows = load(a.forward, FORWARD_FIELDS, f)
    seen: dict[str, int] = {}
    cache: dict[str, bool] = {}
    containers = set()
    for r in rows:
        k = r["ClaimKey"]
        seen[k] = seen.get(k, 0) + 1
        tag = f"{a.forward}: {k}"
        if r["DeliverableID"] != a.deliverable:
            f.append(f"{tag}: DeliverableID {r['DeliverableID']!r}")
        if r["SourceStateSHA"] != FREEZE:
            f.append(f"{tag}: SourceStateSHA not the frozen commit")
        for col, allowed in V.items():
            if r[col] not in allowed:
                f.append(f"{tag}: {col}={r[col]!r} not in vocabulary")
        for layer in filter(None, (x.strip() for x in r["DivergenceLayers"].split(";"))):
            if layer not in LAYERS:
                f.append(f"{tag}: DivergenceLayers value {layer!r}")
        if not AUTH_RE.match(r["AuthorityNeeded"]):
            f.append(f"{tag}: AuthorityNeeded={r['AuthorityNeeded']!r}")
        d, ct = r["Disposition"], r["ClaimType"]
        if (d == "COVERED_BY_CHILDREN") != (ct == "CONTAINER"):
            f.append(f"{tag}: COVERED_BY_CHILDREN must pair with CONTAINER")
        if (d == "NOT_ASSESSED") != (ct == "NON_NORMATIVE"):
            f.append(f"{tag}: NOT_ASSESSED must pair with NON_NORMATIVE")
        if ct == "CONTAINER":
            containers.add(k)
        if d in QUIET:
            if r["CauseTag"] or r["AuthorityTier"]:
                f.append(f"{tag}: CauseTag/AuthorityTier must be empty for {d}")
        else:
            if not r["CauseTag"]:
                f.append(f"{tag}: non-aligned row lacks CauseTag")
            if not r["AuthorityTier"]:
                f.append(f"{tag}: non-aligned row lacks AuthorityTier")
        if r["CauseTag"] == "OTHER" and len(r["Notes"].strip()) < 20:
            f.append(f"{tag}: CauseTag OTHER needs an explanation in Notes")
        if d == "UNKNOWN" and not r["RemainingWork"].strip():
            f.append(f"{tag}: UNKNOWN needs the smallest next check in RemainingWork")
        if len(r["ClaimSummary"]) > 200:
            f.append(f"{tag}: ClaimSummary over 200 characters")
        for col in ("ImplementationEvidence", "VerificationEvidence", "ValidationEvidence"):
            for tok in (t.strip() for t in r[col].split(";")):
                if tok in SPECIAL:
                    continue
                if PATH_TOKEN_RE.match(tok) and not path_exists(a.repo_root, tok, cache):
                    f.append(f"{tag}: {col} path not in frozen tree: {tok}")
    for k, n in seen.items():
        if n > 1:
            f.append(f"{a.forward}: key {k} appears {n} times")
    missing = sorted(set(keys) - set(seen))
    for k in missing:
        f.append(f"{a.forward}: missing claim key {k}")
    for k in sorted(set(seen) - set(keys)):
        m = re.match(r"^(.*)\.s\d{2}$", k)
        if not m or m.group(1) not in keys:
            f.append(f"{a.forward}: unknown key {k} (only .sNN sub-claims of issued keys may be added)")
        elif m.group(1) not in containers:
            f.append(f"{a.forward}: sub-claim {k} whose parent is not CONTAINER")
    if a.reverse:
        inv = {r["CapabilityID"] for r in csv.DictReader(open(a.inventory, newline="", encoding="utf-8"))
               if r["CapabilityID"] != "#END"} if a.inventory else set()
        rrows = load(a.reverse, REVERSE_FIELDS, f)
        answered = set()
        for r in rrows:
            cid = r["CapabilityID"]
            answered.add(cid)
            if inv and cid not in inv:
                f.append(f"{a.reverse}: unknown CapabilityID {cid}")
            if r["Answer"] not in {"CLAIMED_BY", "PARTIAL", "NOT_MINE"}:
                f.append(f"{a.reverse}: {cid}: Answer {r['Answer']!r}")
            if r["Answer"] in {"CLAIMED_BY", "PARTIAL"} and r["ClaimKey"] not in seen:
                f.append(f"{a.reverse}: {cid}: ClaimKey {r['ClaimKey']!r} not in forward ledger")
        for cid in sorted(inv - answered):
            f.append(f"{a.reverse}: capability {cid} unanswered")
    for x in f:
        print("FINDING", x)
    print(f"{'PASS' if not f else 'FAIL'} {a.deliverable}: {len(rows)} forward rows, {len(keys)} issued keys, {len(f)} findings")
    return 1 if f else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
