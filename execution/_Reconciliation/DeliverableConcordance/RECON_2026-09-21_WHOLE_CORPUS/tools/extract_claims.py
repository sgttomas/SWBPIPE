#!/usr/bin/env python3
"""Deterministic claim-key extractor for the Piping reconciliation run.

Reads deliverable scope and declared-state surfaces from a Git commit (never
the working tree), and emits one row per auditable unit with a stable key,
line range and SHA-256 of the unit's exact text. The output is the coverage
denominator: every key must be dispositioned exactly once in the ledgers.

The extractor makes no judgement about meaning. Workers may add sub-claims
beneath a key using the `.sNN` suffix rule; they never mint top-level keys.

Key grammar
  <DEL>:<SURFACE>                       whole-surface unit
  <DEL>:<SURFACE>#<SECTION>             heading-delimited block
  <DEL>:<SURFACE>#<SECTION>/<ITEM>      numbered item inside a block
  <DEL>:<SURFACE>/<ITEM>                numbered item outside any block
where SURFACE is SOW, AB, STATUS, MEMORY, CONTEXT, or a bespoke file stem;
SECTION is a CLM-NNN identifier when present, else a slug of the heading;
ITEM is the item's own identifier (REQ, AC, OUT, VER, table row id, CSV id).

Usage
  extract_claims.py --repo-root . --commit <sha> --out claims.csv [--deliverable DEL-XX-YY ...]
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import re
import subprocess
import sys
from dataclasses import dataclass

PROJECT = "projects/chirality-piping"
DEL_DIR_RE = re.compile(r"^" + re.escape(PROJECT) + r"/execution/(PKG-\d\d)[^/]*/1_Working/(DEL-\d\d-\d\d)[^/]*/$")
HEADING_RE = re.compile(r"^(#{2,4})\s+(.*\S)\s*$")
CLM_RE = re.compile(r"^(CLM-\d{3})\b")
BULLET_ID_RE = re.compile(r"^- \*\*([A-Z]{2,5}-\d{3})\*\*")
# first table cell holding an identifier, optionally inside a blockquote
ROW_ID_RE = re.compile(r"^(?:>\s*)?\|\s*`?((?:DEL-\d\d-\d\d-)?[A-Z]{1,6}(?:-[A-Z0-9]{1,6})*-\d{1,4}[A-Za-z]?)`?\s*\|")
REMAINING_HDR = "## Remaining"

FIELDS = ["ClaimKey", "DeliverableID", "PackageID", "Surface", "SourcePath", "UnitKind",
          "ParentKey", "LineStart", "LineEnd", "TextSHA256", "Title", "Commit"]


@dataclass
class Unit:
    key: str
    kind: str
    parent: str
    start: int
    end: int
    text: str
    title: str


def git(repo: str, *args: str) -> str:
    return subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True, text=True).stdout


def sha(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def slug(title: str) -> str:
    s = re.sub(r"[^A-Za-z0-9]+", "-", title).strip("-").lower()
    return s[:48] or "untitled"


def unique(key: str, seen: dict[str, int]) -> str:
    n = seen.get(key, 0)
    seen[key] = n + 1
    return key if n == 0 else f"{key}~{n + 1}"


def markdown_units(del_id: str, surface: str, text: str) -> list[Unit]:
    lines = text.split("\n")
    units: list[Unit] = []
    seen: dict[str, int] = {}
    base = f"{del_id}:{surface}"
    units.append(Unit(base, "SURFACE", "", 1, len(lines), text, surface))
    # blocks: from a heading (level 2-4) to the next heading of the same or higher level
    heads = []
    for i, line in enumerate(lines):
        m = HEADING_RE.match(line)
        if m:
            heads.append((i, len(m.group(1)), m.group(2)))
    block_of = [None] * len(lines)
    for idx, (i, level, title) in enumerate(heads):
        j = len(lines)
        for (k, lv, _t) in heads[idx + 1:]:
            if lv <= level:
                j = k
                break
        cm = CLM_RE.match(title)
        section = cm.group(1) if cm else slug(title)
        key = unique(f"{base}#{section}", seen)
        body = "\n".join(lines[i:j])
        units.append(Unit(key, "BLOCK", base, i + 1, j, body, title))
        for r in range(i, j):
            # innermost block wins
            if block_of[r] is None or heads[[h[0] for h in heads].index(block_of[r][0])][1] < level:
                block_of[r] = (i, key)
    for r, line in enumerate(lines):
        m = BULLET_ID_RE.match(line) or ROW_ID_RE.match(line)
        if not m:
            continue
        parent = block_of[r][1] if block_of[r] else base
        item = m.group(1)
        key = unique(f"{parent}/{item}", seen)
        units.append(Unit(key, "ITEM", parent, r + 1, r + 1, line, item))
    return units


def status_units(del_id: str, text: str) -> list[Unit]:
    units = markdown_units(del_id, "STATUS", text)
    lines = text.split("\n")
    seen: dict[str, int] = {}
    inside = False
    n = 0
    for r, line in enumerate(lines):
        if line.startswith("## "):
            inside = line.strip() == REMAINING_HDR
            continue
        if inside and line.startswith("- "):
            n += 1
            key = unique(f"{del_id}:STATUS#remaining/R{n:02d}", seen)
            units.append(Unit(key, "ITEM", f"{del_id}:STATUS#remaining", r + 1, r + 1, line, f"Remaining item {n}"))
    return units


def csv_units(del_id: str, surface: str, text: str) -> list[Unit]:
    units = [Unit(f"{del_id}:{surface}", "SURFACE", "", 1, text.count("\n") + 1, text, surface)]
    reader = csv.reader(io.StringIO(text))
    header = next(reader, None)
    seen: dict[str, int] = {}
    for row in reader:
        if not row or not row[0].strip():
            continue
        line = reader.line_num
        key = unique(f"{del_id}:{surface}/ROW-{row[0].strip()}", seen)
        units.append(Unit(key, "ITEM", f"{del_id}:{surface}", line, line, ",".join(row), row[1] if len(row) > 1 else row[0]))
    return units


def surfaces_for(files: list[str]) -> list[tuple[str, str, str]]:
    """Return (surface-label, filename, handler) for the auditable files present."""
    out = []
    names = set(files)
    if "ScopeOfWork.md" in names:
        out.append(("SOW", "ScopeOfWork.md", "md"))
    if "ArchitectureBasis.md" in names:
        out.append(("AB", "ArchitectureBasis.md", "md"))
    if "ScopeOfWork.md" not in names and "ArchitectureBasis.md" not in names:
        # bespoke representation: every top-level .md/.csv that is not a standard surface
        standard = {"_STATUS.md", "MEMORY.md", "_CONTEXT.md", "_REFERENCES.md", "_DEPENDENCIES.md",
                    "_SEMANTIC.md", "_SEMANTIC_LENSING.md", "_REVIEW.md", "Review_Findings.csv", "Dependencies.csv"}
        for f in sorted(names - standard):
            stem = re.sub(r"[^A-Za-z0-9]+", "_", f.rsplit(".", 1)[0]).upper()
            if f.endswith(".md"):
                out.append((stem, f, "md"))
            elif f.endswith(".csv"):
                out.append((stem, f, "csv"))
    if "_STATUS.md" in names:
        out.append(("STATUS", "_STATUS.md", "status"))
    if "_CONTEXT.md" in names:
        out.append(("CONTEXT", "_CONTEXT.md", "md"))
    if "MEMORY.md" in names:
        out.append(("MEMORY", "MEMORY.md", "surface"))
    return out


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--commit", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--deliverable", action="append", default=[])
    a = ap.parse_args(argv)
    commit = git(a.repo_root, "rev-parse", a.commit).strip()
    tree = git(a.repo_root, "ls-tree", "-r", "-z", "--name-only", commit, "--", f"{PROJECT}/execution").split("\0")
    dels: dict[str, tuple[str, str, list[str]]] = {}
    for path in tree:
        parts = path.split("/")
        if len(parts) != 7 or parts[4] != "1_Working":
            continue
        d = "/".join(parts[:6]) + "/"
        m = DEL_DIR_RE.match(d)
        if not m:
            continue
        dels.setdefault(m.group(2), (m.group(1), d, []))[2].append(parts[6])
    rows = []
    for del_id in sorted(dels):
        if a.deliverable and del_id not in a.deliverable:
            continue
        pkg, d, files = dels[del_id]
        for surface, fname, handler in surfaces_for(files):
            path = d + fname
            text = git(a.repo_root, "show", f"{commit}:{path}")
            if handler == "md":
                units = markdown_units(del_id, surface, text)
            elif handler == "csv":
                units = csv_units(del_id, surface, text)
            elif handler == "status":
                units = status_units(del_id, text)
            else:
                units = [Unit(f"{del_id}:{surface}", "SURFACE", "", 1, text.count("\n") + 1, text, surface)]
            for u in units:
                rows.append([u.key, del_id, pkg, surface, path, u.kind, u.parent, u.start, u.end,
                             sha(u.text), u.title[:120], commit])
    keys = [r[0] for r in rows]
    if len(keys) != len(set(keys)):
        dup = sorted({k for k in keys if keys.count(k) > 1})[:10]
        print(f"DUPLICATE KEYS: {dup}", file=sys.stderr)
        return 1
    with open(a.out, "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(FIELDS)
        w.writerows(rows)
    print(f"{len(rows)} units from {len({r[1] for r in rows})} deliverables at {commit}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
