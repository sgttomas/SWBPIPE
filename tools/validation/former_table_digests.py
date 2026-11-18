"""The known former digests of the result tables, derived from the repository's history.

A stored run whose record binds a table digest this version does not hold is refused with
ANALYSIS_SEMANTIC_CONTRACT_MISMATCH. The words of that refusal depend on one thing: whether the
digest is a former revision of the same table (earlier-revision words) or not (unrecognised words).
The list never admits a record. Three copies hold it, row for row:
`core/analysis_runs/compatibility.py`, `apps/desktop/src/services/semanticContractRefusal.ts` and
`apps/desktop/src-tauri/src/analysis_run_v03_validate.rs`. `stored_custody_tests.rs` requires the
three to be equal, and this tool writes them.

A row is (the table's `semantic_contract_id`, the SHA-256 of the table file's bytes) for each
distinct content that a table file `fixtures/results/semantic_contract_v0_3_*.json` had on the main
line, other than the contents of a table file at HEAD. The main line is the first-parent history of
the merge base of HEAD and the main branch (`--main`, default `origin/main`, else `main`). A
branch's intermediate states therefore do not count, and the list derived on a branch equals the one
derived on main after the branch merges. It also takes the digest bound by each committed recorded
run under `fixtures/persistence/recorded_model_*` that the tables' history lacks. The table id is
read from the content at each revision. Rows come by path, then oldest first, then the recorded
runs' rows.

The history is read locally (git `--no-lazy-fetch`; nothing is fetched). A shallow clone, or a
partial clone that lacks a table revision's object, is refused, never answered with a short list.
The Repository checks job clones full history without a blob filter and runs
`test_former_table_digests.py`; other jobs do not read history.

  python3 tools/validation/former_table_digests.py check [--main REF]   exit 1 when a copy differs
  python3 tools/validation/former_table_digests.py write [--main REF]   rewrite the three copies
"""
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from fnmatch import fnmatchcase
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TABLE_DIRECTORY = "fixtures/results/"
TABLE_FILES = TABLE_DIRECTORY + "semantic_contract_v0_3_*.json"
TABLES = ":(glob)" + TABLE_FILES
ESTABLISHING_RECORDS = ":(glob)fixtures/persistence/recorded_model_*/*.analysis_run.json"
MAIN_REFS = ("origin/main", "main")
WRITTEN_BY = "tools/validation/former_table_digests.py"


class HistoryUnavailable(Exception):
    """The main line's history of the tables cannot be read completely here."""


def git(*args: str, binary: bool = False):
    result = subprocess.run(["git", "--no-lazy-fetch", "-C", str(ROOT), *args], capture_output=True)
    if result.returncode:
        raise HistoryUnavailable(f"git {' '.join(args)} failed: {result.stderr.decode('utf-8', 'replace').strip()}")
    return result.stdout if binary else result.stdout.decode("utf-8").strip()


def main_line(main_ref: str | None) -> str:
    """The merge base of HEAD and the main branch, refusing a shallow clone."""
    if git("rev-parse", "--is-shallow-repository") != "false":
        raise HistoryUnavailable("this is a shallow clone: the tables' main-line history is incomplete here, "
                                 "so the known former digests cannot be derived; a full clone (not shallow) is needed")
    for ref in ([main_ref] if main_ref else MAIN_REFS):
        try:
            git("rev-parse", "--verify", "--quiet", ref + "^{commit}")
        except HistoryUnavailable:
            continue
        return git("merge-base", ref, "HEAD")
    raise HistoryUnavailable(f"no main branch here ({', '.join([main_ref] if main_ref else MAIN_REFS)}): "
                             "name it with --main")


def blob(revision: str, path: str) -> bytes | None:
    """The file's bytes at the revision; None when the revision does not hold the file."""
    if not git("ls-tree", "--name-only", revision, "--", path):
        return None
    try:
        return git("cat-file", "blob", f"{revision}:{path}", binary=True)
    except HistoryUnavailable as error:
        raise HistoryUnavailable(f"{path} at {revision} is not held here (a partial clone?): {error}") from None


def derive(main_ref: str | None = None) -> list[tuple[str, str]]:
    base = main_line(main_ref)
    installed = {hashlib.sha256(blob("HEAD", path)).hexdigest()
                 for path in git("ls-tree", "-r", "--name-only", "HEAD", "--", TABLE_DIRECTORY).splitlines()
                 if fnmatchcase(path, TABLE_FILES)}
    rows: list[tuple[str, str]] = []

    def add(contract_id: object, digest: object, where: str) -> None:
        if not isinstance(contract_id, str) or not isinstance(digest, str) or len(digest) != 64:
            raise HistoryUnavailable(f"{where} names no table binding")
        if digest not in installed and (contract_id, digest) not in rows:
            rows.append((contract_id, digest))

    paths = sorted(set(git("log", "--first-parent", "--format=", "--name-only", base, "--", TABLES).split()))
    for path in paths:
        for revision in git("rev-list", "--first-parent", "--reverse", base, "--", path).split():
            content = blob(revision, path)
            if content is not None:
                add(json.loads(content).get("semantic_contract_id"), hashlib.sha256(content).hexdigest(), f"{path} at {revision}")
    for path in git("ls-files", "--", ESTABLISHING_RECORDS).splitlines():
        binding = json.loads(blob("HEAD", path))["analysis_run"]["reproducibility"]["semantic_contract"]
        add(binding.get("id"), binding.get("sha256"), path)
    return rows


def python_block(rows) -> list[str]:
    return ["KNOWN_FORMER_TABLE_DIGESTS: tuple[tuple[str, str], ...] = (",
            *(f'    ("{i}", "{s}"),' for i, s in rows), ")"]


def typescript_block(rows) -> list[str]:
    return ["const KNOWN_FORMER_TABLE_DIGESTS: ReadonlyArray<Readonly<{ id: string; sha256: string }>> = Object.freeze([",
            *(f'  {{ id: "{i}", sha256: "{s}" }},' for i, s in rows), "]);"]


def rust_block(rows) -> list[str]:
    return [f"pub(crate) const KNOWN_FORMER_TABLE_DIGESTS: [(&str, &str); {len(rows)}] = [",
            *(f'    ("{i}", "{s}"),' for i, s in rows), "];"]


# Each copy: its path, the start of its declaration line, its block, and its closing line.
COPIES = (
    ("core/analysis_runs/compatibility.py", "KNOWN_FORMER_TABLE_DIGESTS:", python_block, ")"),
    ("apps/desktop/src/services/semanticContractRefusal.ts", "const KNOWN_FORMER_TABLE_DIGESTS:", typescript_block, "]);"),
    ("apps/desktop/src-tauri/src/analysis_run_v03_validate.rs", "pub(crate) const KNOWN_FORMER_TABLE_DIGESTS:", rust_block, "];"),
)


def rewritten(text: str, start: str, block: list[str], close: str, path: str) -> str:
    lines = text.split("\n")
    begins = [n for n, line in enumerate(lines) if line.startswith(start)]
    if len(begins) != 1:
        raise ValueError(f"{path}: {len(begins)} declarations of KNOWN_FORMER_TABLE_DIGESTS, not one")
    end = next((n for n in range(begins[0], len(lines)) if lines[n] == close), None)
    if end is None:
        raise ValueError(f"{path}: KNOWN_FORMER_TABLE_DIGESTS has no closing {close!r} line")
    return "\n".join(lines[:begins[0]] + block + lines[end + 1:])


def problems(rows) -> list[str]:
    """The copies that differ from the derived rows."""
    found = []
    for path, start, block, close in COPIES:
        text = (ROOT / path).read_text(encoding="utf-8")
        if rewritten(text, start, block(rows), close, path) != text:
            found.append(f"{path}: KNOWN_FORMER_TABLE_DIGESTS differs from the tables' history; "
                         f"run python3 {WRITTEN_BY} write")
    return found


def write(rows) -> None:
    for path, start, block, close in COPIES:
        target = ROOT / path
        text = target.read_text(encoding="utf-8")
        updated = rewritten(text, start, block(rows), close, path)
        if updated != text:
            target.write_text(updated, encoding="utf-8")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("mode", choices=("check", "write"))
    parser.add_argument("--main", default=None)
    args = parser.parse_args(argv)
    try:
        rows = derive(args.main)
    except HistoryUnavailable as error:
        print(f"FORMER-TABLE-DIGESTS-HISTORY-UNAVAILABLE: {error}", file=sys.stderr)
        return 2
    if args.mode == "write":
        write(rows)
        print(f"{len(rows)} rows written to {len(COPIES)} copies")
        return 0
    found = problems(rows)
    for problem in found:
        print(problem, file=sys.stderr)
    return 1 if found else 0


if __name__ == "__main__":
    raise SystemExit(main())
