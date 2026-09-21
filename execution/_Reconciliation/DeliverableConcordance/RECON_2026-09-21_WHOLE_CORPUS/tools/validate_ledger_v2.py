#!/usr/bin/env python3
"""Structural validator v2 for RECON_2026-09-21_WHOLE_CORPUS ledgers.

Enforces the bound conventions (CONVENTIONS.md Part D and C1-C10) against
CLAIM_KEYS_V2.csv and CANONICAL_ASSIGNMENTS.csv. Deterministic and read-only.

Exit codes: 0 = PASS, 1 = findings, 2 = input error.

Structural validity is not semantic correctness. The package verifier judges
semantics.

Usage
  validate_ledger_v2.py --run-dir <run> --repo-root <repo> --deliverable DEL-XX-YY \
      --forward <file> [--reverse <file> --inventory <file>]
"""

from __future__ import annotations

import argparse
import collections
import csv
import io
import os
import re
import subprocess
import sys

FREEZE = "00115c71931bcae79909602d653740d3bb72dfa1"
FORWARD_FIELDS = ["ClaimKey", "DeliverableID", "UnitKind", "ClaimType", "ClaimClass", "ClaimSummary",
                  "NormativeSource", "DecisionBasis", "DeclaredState", "RecordedRemaining",
                  "ImplementationEvidence", "VerificationEvidence", "VerificationClass",
                  "ValidationEvidence", "SourceReliability", "LifecycleState", "Disposition",
                  "Confidence", "CauseTag", "AuthorityTier", "BaselineClass", "DivergenceLayers",
                  "FindingGroup", "CanonicalSituation", "AdoptedByReference", "ContextRefs",
                  "RemainingWork", "AuthorityNeeded", "SelectableUnderCurrentLoop", "Notes",
                  "SourceStateSHA"]
REVERSE_FIELDS = ["CapabilityID", "Answer", "ClaimKey", "Reason"]
V = {
    "UnitKind": {"SURFACE", "BLOCK", "ITEM", "SUBCLAIM"},
    "ClaimType": {"REQUIREMENT", "ACCEPTANCE", "EXCLUSION", "DECLARED_STATE", "REMAINING_WORK",
                  "CONTEXT", "CONTAINER", "NON_NORMATIVE", "HISTORY"},
    "ClaimClass": {"GOVERNANCE", "SCHEMA", "MECHANICS", "WORKFLOW", "GUI", "REPORTING", "INTEROP",
                   "VALIDATION", "SECURITY", "DOCUMENTATION"},
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
                 "REPRESENTATION_MIGRATED", "BASIS_POINTER_STALE", "OWNERSHIP_ELSEWHERE", "PARTIAL_SLICE",
                 "NOT_STARTED", "DEFERRED_BY_RULING", "EVIDENCE_OVERTAKEN", "EVIDENCE_NOT_LOCATED",
                 "VERIFICATION_REMOVED", "VALIDATION_GAP", "RECORD_DRIFT", "AUTHORITY_UNCLEAR",
                 "POSSIBLE_DEFECT", "OTHER"},
    "AuthorityTier": {"", "LOCAL_DESIGN", "PROJECT_BASELINE", "INVARIANT"},
    "BaselineClass": {"", "ISSUED", "FROZEN_CONTRACT", "PROTECTED_CHECK", "RULED_CRITERION",
                      "OWNER_HOLD", "NONE"},
    "AdoptedByReference": {"", "YES"},
    "SelectableUnderCurrentLoop": {"NOT_APPLICABLE"},
}
LISTS = {"DivergenceLayers": {"CLAIMS", "VALIDATION", "IP_DATA", "BASELINE", "SECURITY", "LIFECYCLE", "RECORD", "NONE"},
         "VerificationClass": {"UNIT", "BROWSER_E2E", "NATIVE", "STATIC_CHECK", "DOCUMENT_REVIEW", "NONE"}}
QUIET = {"ALIGNED", "COVERED_BY_CHILDREN", "NOT_ASSESSED"}
AUTH_RE = re.compile(r"^(NO|OWNER|ENGINEERING|SCOPE_CHANGE|REVIEW|D-[0-9A-Za-z-]+|DEC-\d{3})$")
ROOT_PREFIX = re.compile(r"^(projects|execution|docs|tools|agents|workflows|\.github|_DomainEngines|domains|exports)/")
SPECIAL = {"NONE_FOUND", "NOT_APPLICABLE", ""}
CANON_FIELDS = ["ClaimType", "Disposition", "CauseTag", "AuthorityTier", "BaselineClass", "DivergenceLayers", "AuthorityNeeded"]
ANSWERS = {"CLAIMED_BY", "PARTIAL", "COVERS", "CONSTRAINS", "UNKEYED", "NOT_MINE"}


def load(path, fields, f):
    raw = open(path, "rb").read()
    if not raw.split(b"\n", 1)[0].endswith(b"\r") or not raw.endswith(b"\r\n"):
        f.append(f"{path}: records must end with CRLF")
    rows = list(csv.reader(io.StringIO(raw.decode("utf-8"), newline="")))
    if not rows or rows[0] != fields:
        f.append(f"{path}: header mismatch; expected the {len(fields)} Part D columns")
        return []
    out = []
    for n, r in enumerate(rows[1:], start=2):
        if len(r) != len(fields):
            f.append(f"{path}: record {n} has {len(r)} fields, expected {len(fields)}")
            continue
        out.append(dict(zip(fields, r)))
    if not out or out[-1][fields[0]] != "#END":
        f.append(f"{path}: missing #END sentinel as final record")
        return out
    body, end = out[:-1], out[-1]
    if end[fields[-1]].strip() != str(len(body)):
        f.append(f"{path}: sentinel count {end[fields[-1]]!r} != {len(body)} body records")
    return body


class Paths:
    def __init__(self, repo, run_dir):
        self.repo, self.run_dir, self.cache = repo, run_dir, {}

    def ok(self, token):
        t = re.split(r"::|#L", token, maxsplit=1)[0].strip().rstrip("/")
        if t.startswith("GATE:"):
            return os.path.exists(os.path.join(self.run_dir, t[5:]))
        if t not in self.cache:
            self.cache[t] = subprocess.run(["git", "-C", self.repo, "cat-file", "-e", f"{FREEZE}:{t}"],
                                           capture_output=True).returncode == 0
        return self.cache[t]


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--deliverable", required=True)
    ap.add_argument("--forward", required=True)
    ap.add_argument("--reverse")
    ap.add_argument("--inventory")
    a = ap.parse_args(argv)
    f: list[str] = []
    keys = {r["ClaimKey"]: r for r in csv.DictReader(open(f"{a.run_dir}/CLAIM_KEYS_V2.csv", newline="", encoding="utf-8"))
            if r["DeliverableID"] == a.deliverable}
    if not keys:
        print(f"no claim keys for {a.deliverable}", file=sys.stderr)
        return 2
    canon = {r["ClaimKey"]: r for r in csv.DictReader(open(f"{a.run_dir}/CANONICAL_ASSIGNMENTS.csv", newline="", encoding="utf-8"))
             if r["DeliverableID"] == a.deliverable}
    paths = Paths(a.repo_root, a.run_dir)
    rows = load(a.forward, FORWARD_FIELDS, f)
    seen = collections.Counter(r["ClaimKey"] for r in rows)
    byk = {r["ClaimKey"]: r for r in rows}
    for r in rows:
        k, tag = r["ClaimKey"], f"{a.forward}: {r['ClaimKey']}"
        if r["DeliverableID"] != a.deliverable:
            f.append(f"{tag}: DeliverableID {r['DeliverableID']!r}")
        if r["SourceStateSHA"] != FREEZE:
            f.append(f"{tag}: SourceStateSHA not the frozen commit")
        for col, allowed in V.items():
            if r[col] not in allowed:
                f.append(f"{tag}: {col}={r[col]!r} not in vocabulary")
        for col, allowed in LISTS.items():
            vals = r[col].split(";") if r[col] else []
            if " " in r[col] or any(v not in allowed for v in vals) or not vals:
                f.append(f"{tag}: {col}={r[col]!r} (a ';' list, no spaces, from the vocabulary)")
        if not AUTH_RE.match(r["AuthorityNeeded"]):
            f.append(f"{tag}: AuthorityNeeded={r['AuthorityNeeded']!r}")
        d, ct = r["Disposition"], r["ClaimType"]
        if (d == "COVERED_BY_CHILDREN") != (ct == "CONTAINER"):
            f.append(f"{tag}: COVERED_BY_CHILDREN must pair with CONTAINER")
        if (d == "NOT_ASSESSED") != (ct == "NON_NORMATIVE"):
            f.append(f"{tag}: NOT_ASSESSED must pair with NON_NORMATIVE")
        if d in QUIET:
            if r["CauseTag"] or r["AuthorityTier"]:
                f.append(f"{tag}: CauseTag/AuthorityTier must be empty for {d}")
        else:
            if not r["CauseTag"] or not r["AuthorityTier"]:
                f.append(f"{tag}: non-aligned row needs CauseTag and AuthorityTier")
            if "NONE" in r["DivergenceLayers"].split(";"):
                f.append(f"{tag}: DivergenceLayers NONE is valid only on quiet rows")
            if not r["BaselineClass"]:
                f.append(f"{tag}: non-aligned row needs BaselineClass (NONE by default)")
        if r["CauseTag"] == "EVIDENCE_NOT_LOCATED" and d != "UNKNOWN":
            f.append(f"{tag}: EVIDENCE_NOT_LOCATED is only for UNKNOWN")
        if d == "VERIFIED_NOT_VALIDATED" and r["AuthorityTier"] != "INVARIANT":
            f.append(f"{tag}: VERIFIED_NOT_VALIDATED rows are INVARIANT (C3)")
        if r["CauseTag"] == "OTHER" and len(r["Notes"].strip()) < 20:
            f.append(f"{tag}: CauseTag OTHER needs an explanation in Notes")
        if d == "UNKNOWN" and not r["RemainingWork"].strip():
            f.append(f"{tag}: UNKNOWN needs the smallest next check in RemainingWork")
        if d == "ACCEPTED_DIVERGENCE" and not (r["DecisionBasis"].strip() not in ("", "NONE_FOUND") or "OWNER_DIRECTION_RECORD:" in r["Notes"]):
            f.append(f"{tag}: ACCEPTED_DIVERGENCE needs a permitting ruling in DecisionBasis or an A3a OWNER_DIRECTION_RECORD")
        if d == "ACCEPTED_DIVERGENCE" and r["CauseTag"] == "RENAME_OR_IDENTITY":
            f.append(f"{tag}: rename residue is a finding, never ACCEPTED_DIVERGENCE (ruling item 3)")
        if len(r["ClaimSummary"]) > 200:
            f.append(f"{tag}: ClaimSummary over 200 characters")
        for col in ("ImplementationEvidence", "VerificationEvidence", "ValidationEvidence", "ContextRefs"):
            if " " in r[col] and col != "ContextRefs":
                f.append(f"{tag}: {col} holds spaces; free text belongs in Notes")
            for tok in (t.strip() for t in r[col].split(";")):
                if tok in SPECIAL:
                    continue
                if tok.startswith("GATE:") or ROOT_PREFIX.match(tok):
                    if not paths.ok(tok):
                        f.append(f"{tag}: {col} path not found at the freeze / in the run folder: {tok}")
                elif col != "ContextRefs":
                    f.append(f"{tag}: {col} token is not a repository-root path or GATE: token: {tok}")
        ki = keys.get(k)
        if ki:
            if ki["PreType"] == "NON_NORMATIVE" and ct != "NON_NORMATIVE" and "PRETYPE_OVERRIDE" not in r["Notes"]:
                f.append(f"{tag}: pre-typed NON_NORMATIVE; overriding needs PRETYPE_OVERRIDE and a reason in Notes")
            if ki["DuplicateOf"] and "DUPLICATE_OF" not in r["Notes"]:
                f.append(f"{tag}: duplicate unit; write DUPLICATE_OF {ki['DuplicateOf']} in Notes")
        if k in canon:
            cv = canon[k]
            if r["CanonicalSituation"] != cv["CanonicalSituation"]:
                f.append(f"{tag}: CanonicalSituation should be {cv['CanonicalSituation']}")
            diff = [c for c in CANON_FIELDS if r[c] != cv[c]]
            if diff and "CANONICAL_DEPARTURE" not in r["Notes"]:
                f.append(f"{tag}: departs from {cv['Variant']} on {diff}; needs CANONICAL_DEPARTURE and a justification in Notes")
        elif r["CanonicalSituation"] and not re.match(r"^CP-\d{2}$", r["CanonicalSituation"]):
            f.append(f"{tag}: CanonicalSituation {r['CanonicalSituation']!r} is not a pattern ID (CP-NN) and no CS assignment exists")
    for k, n in seen.items():
        if n > 1:
            f.append(f"{a.forward}: key {k} appears {n} times")
    required = {k for k, v in keys.items() if v["Required"] == "YES"}
    for k in sorted(required - set(seen)):
        f.append(f"{a.forward}: missing required claim key {k}")
    optional_by_parent = collections.defaultdict(set)
    for k, v in keys.items():
        if v["Required"] == "NO":
            optional_by_parent[v["ParentKey"]].add(k)
    for parent, opts in optional_by_parent.items():
        present = opts & set(seen)
        if present and present != opts:
            f.append(f"{a.forward}: block {parent} split partially; include all of its .rNN keys or none (missing {sorted(opts - present)[:3]}...)")
        if present and byk.get(parent, {}).get("ClaimType") != "CONTAINER":
            f.append(f"{a.forward}: block {parent} is split into .rNN rows, so it must be CONTAINER")
    for k in sorted(set(seen) - set(keys)):
        m = re.match(r"^(.*)\.s\d{2}$", k)
        if not m or m.group(1) not in keys:
            f.append(f"{a.forward}: unknown key {k} (only .sNN sub-claims of issued keys may be added)")
        elif byk[k]["UnitKind"] != "SUBCLAIM":
            f.append(f"{a.forward}: sub-claim {k} must have UnitKind SUBCLAIM")
    if a.reverse:
        inv = {r["CapabilityID"] for r in csv.DictReader(open(a.inventory, newline="", encoding="utf-8"))
               if r["CapabilityID"] != "#END"} if a.inventory else set()
        answered = set()
        for r in load(a.reverse, REVERSE_FIELDS, f):
            cid = r["CapabilityID"]
            answered.add(cid)
            if inv and cid not in inv:
                f.append(f"{a.reverse}: unknown CapabilityID {cid}")
            if r["Answer"] not in ANSWERS:
                f.append(f"{a.reverse}: {cid}: Answer {r['Answer']!r}")
            if r["Answer"] != "NOT_MINE":
                for ck in filter(None, r["ClaimKey"].split(";")):
                    if "#remaining/" in ck:
                        f.append(f"{a.reverse}: {cid}: STATUS#remaining keys are not ownership anchors")
                    elif ck not in seen:
                        f.append(f"{a.reverse}: {cid}: ClaimKey {ck!r} not in the forward ledger")
        for cid in sorted(inv - answered):
            f.append(f"{a.reverse}: capability {cid} unanswered")
    for x in f:
        print("FINDING", x)
    print(f"{'PASS' if not f else 'FAIL'} {a.deliverable}: {len(rows)} forward rows, "
          f"{len(required)} required keys, {len(canon)} canonical, {len(f)} findings")
    return 1 if f else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
