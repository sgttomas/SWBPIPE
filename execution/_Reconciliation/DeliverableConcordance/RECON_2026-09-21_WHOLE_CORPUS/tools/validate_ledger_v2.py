#!/usr/bin/env python3
"""Structural validator v2 for RECON_2026-09-21_WHOLE_CORPUS ledgers.

Enforces the bound conventions (CONVENTIONS.md Part D and C1-C10) against
CLAIM_KEYS_V2.csv and CANONICAL_ASSIGNMENTS.csv. Deterministic and read-only.
Exit codes: 0 = PASS, 1 = findings, 2 = input error. Structural validity is not
semantic correctness; the package verifier judges semantics.

Single-deliverable mode:
  validate_ledger_v2.py --run-dir <run> --repo-root <repo> --deliverable DEL-XX-YY \
      --forward <file> [--reverse <file> --inventory <file>]

Batch consistency mode (checks rows across several validated forward ledgers):
  validate_ledger_v2.py --run-dir <run> --repo-root <repo> --batch <forward.csv> [<forward.csv> ...]
  Flags any two rows with the same BodySHA256, or the same CanonicalSituation,
  whose Disposition, CauseTag, AuthorityTier or DivergenceLayers differ, unless
  the departing row's Notes carry `CANONICAL_DEPARTURE:` with a reason.
  `--resolutions <RESOLUTIONS.csv>` (CONVENTIONS F6) substitutes Agent 0's
  recorded resolution values for the listed keys before comparing, and exempts
  rows recorded as RESOLVED_PAIR (a verifier judged the pair not a conflict).
  CP-04 rows are compared only within the same variant (tier and baseline
  class), as CANONICAL_SITUATIONS.md defines three CP-04 variants.
Evidence tokens may contain spaces only when they resolve to a path at the
freeze (deliverable folders have spaces in their names).

Part F checks (CONVENTIONS F1, F2, F4), required from gate wave 2 onward:
  add `--notes-gap` to single mode. It fails an ALIGNED row whose RemainingWork or Notes
  (path tokens removed) carry gap wording, unless Notes give
  `GAP_WORDING_CHECKED: <reason>` or (Remaining rows) `OPEN_ACTION: <key>`;
  requires ClaimType DECLARED_STATE on STATUS#remaining units; and requires an
  OPEN_ACTION key to name a non-aligned row of the same ledger.
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
PROJECT = "projects/chirality-piping/"
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
RULING_RE = re.compile(r"\b(D-\d{2,3}[a-z]?|DEC-\d{3}|SCA-\d{3}|D-GOV-\d+|D-APP-\d+)\b")
REPO_PREFIX = re.compile(r"^(projects|execution|docs|tools|agents|workflows|\.github|_DomainEngines|domains|exports)/")
PROJ_PREFIX = re.compile(r"^(core|apps|schemas|fixtures|tests|validation|examples|api|governance|provenance|_harness)/")
EVIDENCE_COLS = ("ImplementationEvidence", "VerificationEvidence", "ValidationEvidence")
CANON_FIELDS = ["ClaimType", "Disposition", "CauseTag", "AuthorityTier", "BaselineClass", "DivergenceLayers", "AuthorityNeeded"]
CONSISTENCY_FIELDS = ["Disposition", "CauseTag", "AuthorityTier", "DivergenceLayers"]
ANSWERS = {"CLAIMED_BY", "PARTIAL", "COVERS", "CONSTRAINS", "UNKEYED", "NOT_MINE"}


def load(path, fields, count_field, f):
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
    if end[count_field].strip() != str(len(body)):
        f.append(f"{path}: sentinel {count_field} {end[count_field]!r} != {len(body)} body records")
    return body


class Paths:
    def __init__(self, repo, run_dir):
        self.repo, self.run_dir, self.cache = repo, run_dir, {}

    def resolve(self, token):
        """Return (ok, reason) for an evidence token."""
        t = re.split(r"::|#L", token, maxsplit=1)[0].strip()
        is_proj = bool(PROJ_PREFIX.match(t))
        t = t.rstrip("/")
        if t.startswith("GATE:"):
            rel = t[5:]
            if not rel.startswith("GATE_EVIDENCE/"):
                return False, "GATE: tokens must point under GATE_EVIDENCE/"
            return os.path.exists(os.path.join(self.run_dir, rel)), "not in the run folder"
        if is_proj:
            t = PROJECT + t
        elif not REPO_PREFIX.match(t):
            return False, "not a repository-root, project-root or GATE: token"
        elif not t.startswith("projects/") and self._exists(PROJECT + t):
            # the same path exists in the project and at the root; require the explicit form
            return False, f"ambiguous: a project copy exists; cite {PROJECT + t} or keep the root file with a Notes reason 'ROOT_DOC:'"
        return self._exists(t), "not at the frozen commit"

    def _exists(self, t):
        if t not in self.cache:
            self.cache[t] = subprocess.run(["git", "-C", self.repo, "cat-file", "-e", f"{FREEZE}:{t}"],
                                           capture_output=True).returncode == 0
        return self.cache[t]


def validate_one(a, f):
    keys = {r["ClaimKey"]: r for r in csv.DictReader(open(f"{a.run_dir}/CLAIM_KEYS_V2.csv", newline="", encoding="utf-8"))
            if r["DeliverableID"] == a.deliverable}
    if not keys:
        print(f"no claim keys for {a.deliverable}", file=sys.stderr)
        return None
    canon = {r["ClaimKey"]: r for r in csv.DictReader(open(f"{a.run_dir}/CANONICAL_ASSIGNMENTS.csv", newline="", encoding="utf-8"))
             if r["DeliverableID"] == a.deliverable}
    paths = Paths(a.repo_root, a.run_dir)
    rows = load(a.forward, FORWARD_FIELDS, "Notes", f)
    seen = collections.Counter(r["ClaimKey"] for r in rows)
    byk = {r["ClaimKey"]: r for r in rows}
    for r in rows:
        k, tag = r["ClaimKey"], f"{a.forward}: {r['ClaimKey']}"
        d, ct = r["Disposition"], r["ClaimType"]
        if r["DeliverableID"] != a.deliverable:
            f.append(f"{tag}: DeliverableID {r['DeliverableID']!r}")
        if r["SourceStateSHA"] != FREEZE:
            f.append(f"{tag}: SourceStateSHA not the frozen commit")
        for col, allowed in V.items():
            if r[col] not in allowed:
                f.append(f"{tag}: {col}={r[col]!r} not in vocabulary")
        for col, allowed in LISTS.items():
            vals = r[col].split(";") if r[col] else []
            if " " in r[col] or not vals or any(v not in allowed for v in vals) or len(vals) != len(set(vals)):
                f.append(f"{tag}: {col}={r[col]!r} (a ';' list, no spaces, from the vocabulary)")
        layers = r["DivergenceLayers"].split(";")
        if not AUTH_RE.match(r["AuthorityNeeded"]):
            f.append(f"{tag}: AuthorityNeeded={r['AuthorityNeeded']!r}")
        if (d == "COVERED_BY_CHILDREN") != (ct == "CONTAINER"):
            f.append(f"{tag}: COVERED_BY_CHILDREN must pair with CONTAINER")
        if (d == "NOT_ASSESSED") != (ct == "NON_NORMATIVE"):
            f.append(f"{tag}: NOT_ASSESSED must pair with NON_NORMATIVE")
        if d in QUIET:
            if r["CauseTag"] or r["AuthorityTier"] or r["BaselineClass"]:
                f.append(f"{tag}: CauseTag, AuthorityTier and BaselineClass must be empty for {d}")
            if layers != ["NONE"]:
                f.append(f"{tag}: quiet rows take DivergenceLayers NONE only")
        else:
            if not r["CauseTag"] or not r["AuthorityTier"] or not r["BaselineClass"]:
                f.append(f"{tag}: non-aligned row needs CauseTag, AuthorityTier and BaselineClass (NONE by default)")
            if "NONE" in layers:
                f.append(f"{tag}: DivergenceLayers NONE is valid only on quiet rows")
        if r["CauseTag"] == "EVIDENCE_NOT_LOCATED" and d != "UNKNOWN":
            f.append(f"{tag}: EVIDENCE_NOT_LOCATED is only for UNKNOWN")
        if d == "VERIFIED_NOT_VALIDATED" and r["AuthorityTier"] != "INVARIANT":
            f.append(f"{tag}: VERIFIED_NOT_VALIDATED rows are INVARIANT (C3)")
        if d == "LIFECYCLE_REASSESSMENT_REQUIRED" and not re.match(r"^FG-DEL-\d\d-\d\d-\d{2}$", r["FindingGroup"]):
            f.append(f"{tag}: LIFECYCLE_REASSESSMENT_REQUIRED needs a FindingGroup FG-<DEL>-NN (C6(d))")
        if r["FindingGroup"] and not re.match(rf"^FG-{a.deliverable}-\d{{2}}$", r["FindingGroup"]):
            f.append(f"{tag}: FindingGroup {r['FindingGroup']!r} must be FG-{a.deliverable}-NN")
        if d == "REMAINING_STATE_MISMATCH" and "#remaining/" not in k:
            f.append(f"{tag}: REMAINING_STATE_MISMATCH is only for STATUS#remaining units (C6(c))")
        if r["CauseTag"] == "OTHER" and len(r["Notes"].strip()) < 20:
            f.append(f"{tag}: CauseTag OTHER needs an explanation in Notes")
        if d == "UNKNOWN" and not r["RemainingWork"].strip():
            f.append(f"{tag}: UNKNOWN needs the smallest next check in RemainingWork")
        if d == "ACCEPTED_DIVERGENCE":
            a3a = "OWNER_DIRECTION_RECORD:" in r["Notes"]
            if not (RULING_RE.search(r["DecisionBasis"]) or a3a):
                f.append(f"{tag}: ACCEPTED_DIVERGENCE needs a permitting ruling ID in DecisionBasis or an A3a OWNER_DIRECTION_RECORD")
            if r["CauseTag"] == "RENAME_OR_IDENTITY" or re.search(r"\bDEC-101\b", r["DecisionBasis"]):
                f.append(f"{tag}: DEC-101 does not reach deliverable files; rename residue is a finding (ruling item 3)")
        if len(r["ClaimSummary"]) > 200:
            f.append(f"{tag}: ClaimSummary over 200 characters")
        for col in EVIDENCE_COLS + ("ContextRefs",):
            if col in EVIDENCE_COLS and not r[col].strip():
                f.append(f"{tag}: {col} is empty; use NONE_FOUND or NOT_APPLICABLE")
            # Spaces are allowed only inside a token that resolves to a path at the freeze
            # (deliverable folders such as "PKG-00_Software Architecture Runway" contain spaces);
            # free text still fails because it does not resolve.
            for tok in (t.strip() for t in r[col].split(";")):
                if tok in ("", "NONE_FOUND", "NOT_APPLICABLE"):
                    continue
                if col == "ContextRefs" and not (tok.startswith("GATE:") or REPO_PREFIX.match(tok) or PROJ_PREFIX.match(tok)):
                    continue  # ContextRefs may also hold PR numbers and other non-path references
                ok, why = paths.resolve(tok)
                if not ok and not (why.startswith("ambiguous") and "ROOT_DOC:" in r["Notes"]):
                    f.append(f"{tag}: {col} token {tok!r}: {why}")
        ki = keys.get(k)
        if ki:
            if r["UnitKind"] != ki["UnitKind"]:
                f.append(f"{tag}: UnitKind {r['UnitKind']} differs from the issued {ki['UnitKind']}")
            if ki["PreType"] == "NON_NORMATIVE" and ct != "NON_NORMATIVE" \
                    and not re.search(r"PRETYPE_OVERRIDE:\s*\S.{9,}", r["Notes"]):
                f.append(f"{tag}: pre-typed NON_NORMATIVE; overriding needs 'PRETYPE_OVERRIDE: <reason>' in Notes")
            if ki["DuplicateOf"] and f"DUPLICATE_OF {ki['DuplicateOf']}" not in r["Notes"]:
                f.append(f"{tag}: duplicate unit; write 'DUPLICATE_OF {ki['DuplicateOf']}' in Notes")
            tgt = byk.get(ki["DuplicateOf"]) if ki["DuplicateOf"] else None
            if tgt and any(r[c] != tgt[c] for c in CONSISTENCY_FIELDS):
                f.append(f"{tag}: DUPLICATE_OF {ki['DuplicateOf']} must take the same disposition, cause, tier and layers (C1)")
        if k in canon:
            cv = canon[k]
            if r["CanonicalSituation"] != cv["CanonicalSituation"]:
                f.append(f"{tag}: CanonicalSituation should be {cv['CanonicalSituation']}")
            diff = [c for c in CANON_FIELDS if r[c] != cv[c]]
            if diff and not re.search(r"CANONICAL_DEPARTURE:\s*\S.{9,}", r["Notes"]):
                f.append(f"{tag}: departs from {cv['Variant']} on {diff}; needs 'CANONICAL_DEPARTURE: <reason>' in Notes")
        elif r["CanonicalSituation"] and not re.match(r"^CP-\d{2}$", r["CanonicalSituation"]):
            f.append(f"{tag}: CanonicalSituation {r['CanonicalSituation']!r} is not a pattern ID (CP-NN) and no CS assignment exists")
    for k, n in seen.items():
        if n > 1:
            f.append(f"{a.forward}: key {k} appears {n} times")
    required = {k for k, v in keys.items() if v["Required"] == "YES"}
    for k in sorted(required - set(seen)):
        f.append(f"{a.forward}: missing required claim key {k}")
    opts_by_parent = collections.defaultdict(set)
    for k, v in keys.items():
        if v["Required"] == "NO":
            opts_by_parent[v["ParentKey"]].add(k)
    for parent, opts in opts_by_parent.items():
        present = opts & set(seen)
        if present and present != opts:
            f.append(f"{a.forward}: block {parent} split partially; include all of its .rNN keys or none "
                     f"(missing {sorted(opts - present)[:3]})")
    for k in sorted(set(seen) - set(keys)):
        m = re.match(r"^(.*)\.s\d{2}$", k)
        if not m or m.group(1) not in keys:
            f.append(f"{a.forward}: unknown key {k} (only .sNN sub-claims of issued keys may be added)")
            continue
        if byk[k]["UnitKind"] != "SUBCLAIM":
            f.append(f"{a.forward}: sub-claim {k} must have UnitKind SUBCLAIM")
        if opts_by_parent.get(m.group(1)):
            f.append(f"{a.forward}: {k}: block {m.group(1)} has .rNN keys; use them, not .sNN sub-claims")
    if a.reverse:
        inv = {r["CapabilityID"] for r in csv.DictReader(open(a.inventory, newline="", encoding="utf-8"))
               if r["CapabilityID"] != "#END"} if a.inventory else set()
        answered = collections.Counter()
        for r in load(a.reverse, REVERSE_FIELDS, "Reason", f):
            cid = r["CapabilityID"]
            answered[cid] += 1
            if inv and cid not in inv:
                f.append(f"{a.reverse}: unknown CapabilityID {cid}")
            if r["Answer"] not in ANSWERS:
                f.append(f"{a.reverse}: {cid}: Answer {r['Answer']!r}")
            cks = [x for x in r["ClaimKey"].split(";") if x]
            if r["Answer"] == "NOT_MINE":
                if cks:
                    f.append(f"{a.reverse}: {cid}: NOT_MINE takes no ClaimKey")
            elif not cks:
                f.append(f"{a.reverse}: {cid}: {r['Answer']} needs a ClaimKey")
            for ck in cks:
                if "#remaining" in ck:
                    f.append(f"{a.reverse}: {cid}: STATUS#remaining keys are not ownership anchors")
                elif ck not in seen:
                    f.append(f"{a.reverse}: {cid}: ClaimKey {ck!r} not in the forward ledger")
            if not r["Reason"].strip():
                f.append(f"{a.reverse}: {cid}: Reason is empty")
        for cid, n in answered.items():
            if n > 1:
                f.append(f"{a.reverse}: capability {cid} answered {n} times")
        for cid in sorted(inv - set(answered)):
            f.append(f"{a.reverse}: capability {cid} unanswered")
    return rows, required, canon


GAP_WORDING = re.compile(
    r"not located|not found|carried on|carried by|dispositioned on|partial|unmet|missing|gaps?\b|"
    r"\bno\b[^.;]{0,40}\btests?\b|untested|not asserted|not exercised|not evidenced|absent|lacks|without",
    re.I)
# Evidence columns hold path tokens only (Part D), so they are not scanned; path-like tokens are
# stripped from Notes and RemainingWork before matching.
GAP_COLUMNS = ("RemainingWork", "Notes")
# Only path-shaped tokens are stripped: tokens under a known root, tokens ending in a file
# extension, `path::symbol` tokens, and backticked spans with no spaces (identifiers or paths).
PATH_TOKEN = re.compile(
    r"`[^`\s]+`|"
    r"(?<![\w/])(?:\{?[A-Z_]+\}?/|\.{0,2}/)?(?:projects|core|apps|docs|tools|schemas|fixtures|tests|validation|"
    r"examples|governance|provenance|api|execution|workflows|scripts|src|src-tauri|features|_run_records|RUN|FREEZE)"
    r"/[^\s`;,)]+|"
    r"(?<![\w/])[\w.\-/]+\.(?:py|rs|tsx?|mjs|js|json|ya?ml|md|csv|toml|txt)\b(?:#L\d+(?:-L?\d+)?)?|"
    r"(?<![\w/])[\w.\-/]+::[\w.<>\-]+")
# The GAP_WORDING_CHECKED clause must be the last clause of Notes (CONVENTIONS F4); only it,
# from the marker to the end of Notes, is excluded from the scan.
ESCAPES = re.compile(r"GAP_WORDING_CHECKED:.*\Z", re.S)


def part_f(a, rows, f):
    byk = {r["ClaimKey"]: r for r in rows}
    listed = 0
    for r in rows:
        k, tag = r["ClaimKey"], f"{a.forward}: {r['ClaimKey']}"
        if "STATUS#remaining/" in k and r["ClaimType"] != "DECLARED_STATE":
            f.append(f"{tag}: F2: Remaining units are ClaimType DECLARED_STATE, not {r['ClaimType']!r}")
        oa = re.search(r"OPEN_ACTION:\s*(\S+)", r["Notes"])
        if oa:
            tgt = oa.group(1).rstrip(".,;)")
            if "STATUS#remaining/" not in k:
                f.append(f"{tag}: F2: OPEN_ACTION is for Remaining units only")
            elif tgt not in byk:
                f.append(f"{tag}: F2: OPEN_ACTION names {tgt!r}, not a row of this ledger")
            elif byk[tgt]["Disposition"] == "ALIGNED":
                f.append(f"{tag}: F2: OPEN_ACTION row {tgt} is ALIGNED, so it does not carry the open work")
        if r["Disposition"] != "ALIGNED":
            continue
        hit = next((m.group(0) for c in GAP_COLUMNS
                    for m in [GAP_WORDING.search(PATH_TOKEN.sub(" ", ESCAPES.sub(" ", r[c])))] if m), None)
        if hit and not re.search(r"GAP_WORDING_CHECKED:\s*\S.{24,}", r["Notes"]) and not oa:
            listed += 1
            f.append(f"{tag}: F4: ALIGNED row carries gap wording {hit!r}; re-dispose it (F1) or add "
                     "GAP_WORDING_CHECKED: <why this is not an unmet element of the claim>")
    return listed


def load_resolutions(path):
    if not path:
        return {}
    out = {}
    for r in csv.DictReader(open(path, newline="", encoding="utf-8")):
        if r["ClaimKey"] == "#END" or not r["Disposition"]:
            continue
        out[r["ClaimKey"]] = r
    return out


def batch(a, f):
    """Cross-ledger consistency. CS rows are checked field-by-field in single mode, so here
    CS groups are keyed by their mechanical Variant; CP groups compare only rows that took
    the same disposition (a CP pattern may allow more than one outcome)."""
    keys = {r["ClaimKey"]: r for r in csv.DictReader(open(f"{a.run_dir}/CLAIM_KEYS_V2.csv", newline="", encoding="utf-8"))}
    variant = {r["ClaimKey"]: r["Variant"] for r in csv.DictReader(open(f"{a.run_dir}/CANONICAL_ASSIGNMENTS.csv", newline="", encoding="utf-8"))}
    groups = collections.defaultdict(list)
    res = load_resolutions(a.resolutions)
    for path in a.batch:
        for r in load(path, FORWARD_FIELDS, "Notes", f):
            if r["ClaimKey"] in res:
                r = dict(r, **{c: res[r["ClaimKey"]][c] for c in CONSISTENCY_FIELDS},
                          Notes=r["Notes"] + " [resolved in " + os.path.basename(a.resolutions) + "]")
            ki = keys.get(r["ClaimKey"])
            if ki and int(ki["SharedTextCount"]) > 1 and ki["UnitKind"] != "SURFACE" and r["ClaimKey"] not in variant:
                groups[("body", ki["BodySHA256"])].append((path, r))
            cs = r["CanonicalSituation"]
            if cs.startswith("CS-"):
                groups[("variant", variant.get(r["ClaimKey"], cs))].append((path, r))
            elif cs.startswith("CP-"):
                # CP-04 defines variants with their own tier and baseline class (CANONICAL_SITUATIONS.md):
                # compare only rows of the same variant.
                sub = f"/{r['AuthorityTier']}/{r['BaselineClass']}" if cs == "CP-04" else ""
                groups[("pattern", f"{cs}/{r['Disposition']}{sub}")].append((path, r))
    for (kind, gid), members in groups.items():
        profiles = collections.Counter(tuple(r[c] for c in CONSISTENCY_FIELDS) for _, r in members)
        if len(profiles) < 2:
            continue
        majority = profiles.most_common(1)[0][0]
        for path, r in members:
            prof = tuple(r[c] for c in CONSISTENCY_FIELDS)
            if r["ClaimKey"] in res and res[r["ClaimKey"]]["Class"] == "RESOLVED_PAIR":
                continue  # a verifier judged this pair not a conflict; Agent 0 recorded it (F6)
            if prof != majority and not re.search(r"CANONICAL_DEPARTURE:\s*\S.{9,}", r["Notes"]):
                f.append(f"{path}: {r['ClaimKey']}: same {kind} {gid[:16]} as {len(members) - 1} other row(s) "
                         f"but {dict(zip(CONSISTENCY_FIELDS, prof))} differs from the majority "
                         f"{dict(zip(CONSISTENCY_FIELDS, majority))} without CANONICAL_DEPARTURE")


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--deliverable")
    ap.add_argument("--forward")
    ap.add_argument("--reverse")
    ap.add_argument("--inventory")
    ap.add_argument("--batch", nargs="+")
    ap.add_argument("--resolutions", help="WAVES/<W>/RESOLUTIONS.csv (batch mode, CONVENTIONS F6)")
    ap.add_argument("--notes-gap", action="store_true", help="apply Part F checks (F1/F2/F4) in single mode")
    a = ap.parse_args(argv)
    f: list[str] = []
    if a.batch:
        batch(a, f)
        for x in f:
            print("FINDING", x)
        print(f"{'PASS' if not f else 'FAIL'} batch of {len(a.batch)} ledgers: {len(f)} consistency findings")
        return 1 if f else 0
    if not (a.deliverable and a.forward):
        print("need --deliverable and --forward, or --batch", file=sys.stderr)
        return 2
    res = validate_one(a, f)
    if res is None:
        return 2
    rows, required, canon = res
    if a.notes_gap:
        part_f(a, rows, f)
    for x in f:
        print("FINDING", x)
    print(f"{'PASS' if not f else 'FAIL'} {a.deliverable}: {len(rows)} forward rows, "
          f"{len(required)} required keys, {len(canon)} canonical, {len(f)} findings")
    return 1 if f else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
