#!/usr/bin/env python3
"""Deterministic claim-key extractor v2 (owner-ruled at R0; R0_REVIEW.md §5).

Changes from v1 (tools/extract_claims.py, retained as calibration provenance):
  1. `###` claim blocks are parented to their `##` section; a claim block ends
     at the first unquoted, non-blank line, so unquoted AC/VER/OUT bullets are
     parented to the section, not the preceding block.
  2. Rows of tables that carry no row identifier are issued as deterministic
     `.rNN` ITEM keys under their block, with `Required=NO`: they need a row
     only when a worker splits the block, and then every `.rNN` of that block
     is required (all-or-none). Otherwise the block's row covers them. This
     keeps sub-claim keys reproducible without multiplying rows (R0 ruling;
     Agent 0's disclosed resolution of R0_REVIEW.md §5 item 2 against its
     stated 20-25% reduction).
  3. Section wrappers and heading-only claim blocks are pre-typed
     `PreType=NON_NORMATIVE`.
  4. Where two CSV surfaces of one deliverable are keyed by the same row-ID
     set, each row is issued once (surface `ROWS`), citing both files.
  5. `SharedTextCount` gives how many deliverables carry an identical unit
     text (same TextSHA256), for canonical pre-disposition.
  6. `DuplicateOf` names the first earlier unit in the same deliverable with
     identical text.
Reads from a Git commit, never the working tree. Output is the coverage
denominator: each key is dispositioned exactly once.

Usage
  extract_claims_v2.py --repo-root . --commit <sha> --out CLAIM_KEYS_V2.csv
"""

from __future__ import annotations

import argparse
import collections
import csv
import hashlib
import io
import re
import subprocess
import sys
from dataclasses import dataclass, field

PROJECT = "projects/chirality-piping"
DEL_DIR_RE = re.compile(r"^" + re.escape(PROJECT) + r"/execution/(PKG-\d\d)[^/]*/1_Working/(DEL-\d\d-\d\d)[^/]*/$")
HEADING_RE = re.compile(r"^(#{2,4})\s+(.*\S)\s*$")
CLM_RE = re.compile(r"^(CLM-\d{3})\b")
BULLET_ID_RE = re.compile(r"^- \*\*([A-Z]{2,5}-\d{3})\*\*")
ROW_ID_RE = re.compile(r"^\|\s*`?((?:DEL-\d\d-\d\d-)?[A-Z]{1,6}(?:-[A-Z0-9]{1,6})*-\d{1,4}[A-Za-z]?)`?\s*\|")
TABLE_ROW_RE = re.compile(r"^\|.*\|\s*$")
TABLE_SEP_RE = re.compile(r"^\|(\s*:?-{2,}:?\s*\|)+\s*$")
QUOTED_HEADING_RE = re.compile(r"^#{1,6}\s")
REMAINING_HDR = "## Remaining"
STANDARD = {"_STATUS.md", "MEMORY.md", "_CONTEXT.md", "_REFERENCES.md", "_DEPENDENCIES.md",
            "_SEMANTIC.md", "_SEMANTIC_LENSING.md", "_REVIEW.md", "Review_Findings.csv", "Dependencies.csv",
            ".gitattributes"}

FIELDS = ["ClaimKey", "DeliverableID", "PackageID", "Surface", "SourcePath", "UnitKind", "PreType", "Required",
          "ParentKey", "LineStart", "LineEnd", "TextSHA256", "SharedTextCount", "DuplicateOf",
          "Title", "Commit"]


@dataclass
class Unit:
    key: str
    kind: str
    parent: str
    start: int
    end: int
    text: str
    title: str
    pretype: str = ""
    source: str = ""


def git(repo: str, *args: str) -> str:
    return subprocess.run(["git", "-C", repo, *args], check=True, capture_output=True, text=True).stdout


def sha(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def slug(title: str) -> str:
    return re.sub(r"[^A-Za-z0-9]+", "-", title).strip("-").lower()[:48] or "untitled"


class Keys:
    def __init__(self) -> None:
        self.seen: dict[str, int] = {}

    def __call__(self, key: str) -> str:
        n = self.seen.get(key, 0)
        self.seen[key] = n + 1
        return key if n == 0 else f"{key}~{n + 1}"


def unquote(line: str) -> str:
    return re.sub(r"^>\s?", "", line)


def markdown_units(del_id: str, surface: str, text: str, quoted_blocks: bool) -> list[Unit]:
    """quoted_blocks: SOW-style files, where ### claim blocks hold blockquoted text."""
    lines = text.split("\n")
    keys = Keys()
    base = f"{del_id}:{surface}"
    units = [Unit(base, "SURFACE", "", 1, len(lines), text, surface)]
    heads = [(i, len(m.group(1)), m.group(2)) for i, l in enumerate(lines) if (m := HEADING_RE.match(l))]
    owner = [base] * len(lines)          # innermost block key covering each line
    level_of = {base: 1}
    stack: list[tuple[int, str]] = []    # (level, key)
    for idx, (i, level, title) in enumerate(heads):
        nxt = next((k for (k, lv, _t) in heads[idx + 1:] if lv <= level), len(lines))
        end = nxt
        if quoted_blocks and level >= 3:
            # a quoted claim block ends at the first unquoted, non-blank line after its heading
            for r in range(i + 1, nxt):
                if lines[r].strip() and not lines[r].startswith(">"):
                    end = r
                    break
        while stack and stack[-1][0] >= level:
            stack.pop()
        parent = stack[-1][1] if stack else base
        cm = CLM_RE.match(title)
        key = keys(f"{base}#{cm.group(1) if cm else slug(title)}")
        body_lines = lines[i + 1:end]
        body = "\n".join(lines[i:end])
        substantive = [unquote(l).strip() for l in body_lines
                       if unquote(l).strip() and not QUOTED_HEADING_RE.match(unquote(l).strip())
                       and not unquote(l).strip().startswith("<!--")]
        pretype = ""
        if not substantive:
            pretype = "NON_NORMATIVE"
        elif level == 2 and quoted_blocks:
            # a section wrapper: only child headings and unquoted ID bullets beneath it
            own = [l for l in lines[i + 1:(heads[idx + 1][0] if idx + 1 < len(heads) else nxt)]
                   if l.strip() and not BULLET_ID_RE.match(l)]
            if not own:
                pretype = "NON_NORMATIVE"
        units.append(Unit(key, "BLOCK", parent, i + 1, end, body, title, pretype))
        level_of[key] = level
        for r in range(i, end):
            if level_of.get(owner[r], 1) < level:
                owner[r] = key
        stack.append((level, key))
    # items
    table_counter: dict[str, int] = collections.Counter()
    for r, line in enumerate(lines):
        raw = unquote(line) if line.startswith(">") else line
        parent = owner[r]
        m = BULLET_ID_RE.match(line)
        if m:  # unquoted ID bullets belong to their block (the section, after v2 block bounds)
            units.append(Unit(keys(f"{parent}/{m.group(1)}"), "ITEM", parent, r + 1, r + 1, line, m.group(1)))
            continue
        if not TABLE_ROW_RE.match(raw) or TABLE_SEP_RE.match(raw):
            continue
        prev = unquote(lines[r - 1]) if r > 0 else ""
        nxt = unquote(lines[r + 1]) if r + 1 < len(lines) else ""
        if TABLE_SEP_RE.match(nxt):
            continue  # header row
        m = ROW_ID_RE.match(raw)
        if m:
            units.append(Unit(keys(f"{parent}/{m.group(1)}"), "ITEM", parent, r + 1, r + 1, line, m.group(1)))
        else:
            table_counter[parent] += 1
            n = table_counter[parent]
            cells = [c.strip() for c in raw.strip().strip("|").split("|")]
            units.append(Unit(keys(f"{parent}.r{n:02d}"), "ITEM", parent, r + 1, r + 1, line, (cells[0] if cells else "")[:120]))
    return units


def status_units(del_id: str, text: str) -> list[Unit]:
    units = markdown_units(del_id, "STATUS", text, quoted_blocks=False)
    units = [u for u in units if not (u.kind == "ITEM" and u.parent.endswith("#remaining"))]
    lines = text.split("\n")
    inside, n = False, 0
    for r, line in enumerate(lines):
        if line.startswith("## "):
            inside = line.strip() == REMAINING_HDR
            continue
        if inside and line.startswith("- "):
            n += 1
            units.append(Unit(f"{del_id}:STATUS#remaining/R{n:02d}", "ITEM", f"{del_id}:STATUS#remaining",
                              r + 1, r + 1, line, f"Remaining item {n}"))
    return units


def csv_rows(text: str) -> tuple[list[str], list[tuple[int, list[str]]]]:
    reader = csv.reader(io.StringIO(text))
    header = next(reader, [])
    out = []
    for row in reader:
        if row and row[0].strip():
            out.append((reader.line_num, row))
    return header, out


def bespoke_surfaces(del_id: str, files: dict[str, str]) -> list[Unit]:
    units: list[Unit] = []
    csvs = {f: csv_rows(t) for f, t in files.items() if f.endswith(".csv")}
    idsets = {f: tuple(r[0].strip() for _, r in rows) for f, (_, rows) in csvs.items()}
    merged: set[str] = set()
    names = sorted(csvs)
    for a in range(len(names)):
        for b in range(a + 1, len(names)):
            fa, fb = names[a], names[b]
            if idsets[fa] and idsets[fa] == idsets[fb] and fa not in merged and fb not in merged:
                merged |= {fa, fb}
                base = f"{del_id}:ROWS"
                units.append(Unit(base, "SURFACE", "", 1, 1, files[fa] + files[fb], f"{fa} + {fb}", source=f"{fa};{fb}"))
                ra, rb = csvs[fa][1], csvs[fb][1]
                for (la, rowa), (lb, rowb) in zip(ra, rb):
                    units.append(Unit(f"{base}/ROW-{rowa[0].strip()}", "ITEM", base, la, lb,
                                      ",".join(rowa) + "\n" + ",".join(rowb), (rowa[1] if len(rowa) > 1 else rowa[0])[:120],
                                      source=f"{fa};{fb}"))
    for f in sorted(files):
        stem = re.sub(r"[^A-Za-z0-9]+", "_", f.rsplit(".", 1)[0]).upper()
        if f.endswith(".md"):
            for u in markdown_units(del_id, stem, files[f], quoted_blocks=False):
                u.source = f
                units.append(u)
        elif f.endswith(".csv") and f not in merged:
            base = f"{del_id}:{stem}"
            units.append(Unit(base, "SURFACE", "", 1, 1, files[f], stem, source=f))
            for line, row in csvs[f][1]:
                units.append(Unit(f"{base}/ROW-{row[0].strip()}", "ITEM", base, line, line, ",".join(row),
                                  (row[1] if len(row) > 1 else row[0])[:120], source=f))
    return units


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--commit", required=True)
    ap.add_argument("--out", required=True)
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
        if m:
            dels.setdefault(m.group(2), (m.group(1), d, []))[2].append(parts[6])
    rows: list[list] = []
    all_units: list[tuple[str, str, Unit, str]] = []
    for del_id in sorted(dels):
        pkg, d, files = dels[del_id]
        names = set(files)
        read = lambda f: git(a.repo_root, "show", f"{commit}:{d}{f}")
        units: list[tuple[Unit, str]] = []
        if "ScopeOfWork.md" in names:
            units += [(u, "SOW") for u in markdown_units(del_id, "SOW", read("ScopeOfWork.md"), True)]
        if "ArchitectureBasis.md" in names:
            units += [(u, "AB") for u in markdown_units(del_id, "AB", read("ArchitectureBasis.md"), False)]
        if not names & {"ScopeOfWork.md", "ArchitectureBasis.md"}:
            bespoke = {f: read(f) for f in sorted(names - STANDARD) if f.endswith((".md", ".csv"))}
            units += [(u, u.key.split(":", 1)[1].split("#")[0].split("/")[0]) for u in bespoke_surfaces(del_id, bespoke)]
        if "_STATUS.md" in names:
            units += [(u, "STATUS") for u in status_units(del_id, read("_STATUS.md"))]
        if "_CONTEXT.md" in names:
            units += [(u, "CONTEXT") for u in markdown_units(del_id, "CONTEXT", read("_CONTEXT.md"), False)]
        if "MEMORY.md" in names:
            t = read("MEMORY.md")
            units.append((Unit(f"{del_id}:MEMORY", "SURFACE", "", 1, t.count("\n") + 1, t, "MEMORY"), "MEMORY"))
        fname = {"SOW": "ScopeOfWork.md", "AB": "ArchitectureBasis.md", "STATUS": "_STATUS.md",
                 "CONTEXT": "_CONTEXT.md", "MEMORY": "MEMORY.md"}
        for u, surface in units:
            src = u.source if u.source else fname.get(surface, "")
            path = ";".join(d + s for s in src.split(";")) if src else d
            all_units.append((del_id, pkg, u, f"{surface}\t{path}"))
    # shared-text counts across deliverables and duplicates within a deliverable
    by_hash: dict[str, set[str]] = collections.defaultdict(set)
    for del_id, _pkg, u, _sp in all_units:
        if u.kind != "SURFACE":
            by_hash[sha(u.text)].add(del_id)
    first_in_del: dict[tuple[str, str], str] = {}
    for del_id, pkg, u, sp in all_units:
        surface, path = sp.split("\t")
        h = sha(u.text)
        dup = ""
        if u.kind != "SURFACE":
            k = (del_id, h)
            if k in first_in_del:
                dup = first_in_del[k]
            else:
                first_in_del[k] = u.key
        required = "NO" if re.search(r"\.r\d{2}$", u.key) else "YES"
        rows.append([u.key, del_id, pkg, surface, path, u.kind, u.pretype, required, u.parent, u.start, u.end, h,
                     len(by_hash[h]) if u.kind != "SURFACE" else 1, dup, u.title[:120], commit])
    keys = [r[0] for r in rows]
    if len(keys) != len(set(keys)):
        print(f"DUPLICATE KEYS: {sorted(k for k, c in collections.Counter(keys).items() if c > 1)[:10]}", file=sys.stderr)
        return 1
    with open(a.out, "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(FIELDS)
        w.writerows(rows)
    print(f"{len(rows)} units from {len({r[1] for r in rows})} deliverables at {commit}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
