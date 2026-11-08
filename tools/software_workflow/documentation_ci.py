#!/usr/bin/env python3
"""Bounded verification for documents; code and runtime documents keep product CI.

Two document kinds take this route. `execution/` holds declarative specifications only
(tools/validation/execution_boundary.py enforces it, and product jobs check out the tree without
it), so any regular-file change there of a declarative kind, including additions, deletions and
renames, is a document change verified by structure, parity, boundary, conflict and credential
checks. Human prose outside it (docs/, loop/, README.md, AGENTS.md) is a document only for in-place
Markdown edits under a current reader audit. This is a separate input contract, not a change to
Python's conservative source analysis: runtime resources, fixtures, licences and document-content
consumers outside execution/ are not plain prose merely because their names end in .md.
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys

from hosted_ci import RAW_DIFF_HEADER, git, text_at

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = "tools/software_workflow/documentation-inputs.json"
BOUNDARY = "tools/validation/execution_boundary.py"
# Declarative specifications: never a product input (the boundary guard and product checkouts).
DECLARATIVE_ROOT = "execution/"
DECLARATIVE_SUFFIXES = (".md", ".yaml", ".yml", ".json", ".csv", ".txt", ".sha256", ".patch")
# Human prose outside execution/: reader-sensitive, admitted under the reader audit.
# Keep source-bound resources and verification inputs on their owning route.
# In particular, embedded-agent assets under apps/ are never human docs here.
DOCUMENT_ROOTS = ("docs/", "loop/")
ROOT_DOCUMENTS = {"README.md", "AGENTS.md"}
PRODUCT_DOCUMENTS = (
    "docs/third-party/", "docs/THIRD_PARTY_NOTICES.md",
    "docs/claims_registry.md",  # Vitest reads the claims as a contract input
)
# Content checks are selected at their actual document-reading boundary. They
# need no Rust session authorities; pytest runs without the product conftest.
# Keys are exact paths or, ending in "/", directory prefixes.
DOCUMENT_TESTS = {
    "docs/security/threat_model.md": ["-m", "pytest", "--noconftest", "-q", "tests/test_security_threat_model.py"],
    "docs/security/redaction_export_controls.md": ["-m", "pytest", "--noconftest", "-q", "tests/security/test_redaction_export_controls.py::test_changed_files_do_not_embed_disallowed_example_content"],
    "docs/security/secret_private_library_handling.md": ["-m", "pytest", "--noconftest", "-q", "tests/security/test_secret_private_library_handling.py::test_changed_files_do_not_embed_disallowed_example_content"],
    "docs/architecture/plugin_boundary.md": ["tests/test_api_boundary_contract.py"],
    "docs/local_analysis/local_fea_handoff_guidance.md": ["tests/test_local_fea_handoff_contract.py"],
    "execution/PKG-04/DEL-04-11/Work/hanger-sizing.md": ["-m", "pytest", "--noconftest", "-q", "tools/validation/test_hanger_sizing_design.py"],
    "execution/PKG-09/DEL-09-01/Work/reference-system.md": ["-m", "pytest", "--noconftest", "-q", "tools/validation/test_reference_system_ledger.py"],
    "execution/_DAG/": ["-m", "pytest", "--noconftest", "-q", "tools/validation/test_execution_dag.py"],
}


def declarative(path: str) -> bool:
    return path.startswith(DECLARATIVE_ROOT)


def source_digest(root: Path, ref: str | None) -> str:
    """Bind the reader audit to all non-Markdown bytes and every tracked path/mode outside execution/.

    Read object identities, not file contents or protected evidence. A new,
    removed or changed reader/config/fixture invalidates the audit. Markdown
    existence/mode also matters: a new file can enter an existing dynamic scan.
    Only in-place Markdown content may vary without refreshing the audit.
    Declarative execution/ records are outside the audit: no product reader
    may read them (the boundary guard), so their bytes and paths cannot stale it.
    """
    args = ["ls-tree", "-rz", ref] if ref else ["ls-files", "--stage", "-z"]
    raw = subprocess.check_output(["git", "-C", str(root), *args])
    records = []
    for entry in filter(None, raw.split(b"\0")):
        info, path = entry.split(b"\t", 1)
        mode, oid, kind = info.split() if not ref else (info.split()[0], info.split()[2], info.split()[1])
        if path.decode() == INVENTORY or path.startswith(DECLARATIVE_ROOT.encode()):
            continue
        if not ref and kind != b"0":
            raise ValueError("Unmerged index cannot establish a documentation-input audit")
        records.append(mode + b" " + (b"markdown" if path.endswith(b".md") else oid) + b"\t" + path + b"\0")
    return hashlib.sha256(b"".join(sorted(records))).hexdigest()


def audited_sources(root: Path, ref: str) -> bool:
    try:
        inventory = json.loads(git(root, "show", f"{ref}:{INVENTORY}"))
        return (inventory.get("schema") == "swbpipe-documentation-inputs/v1"
                and inventory.get("source_digest") == source_digest(root, ref))
    except (subprocess.CalledProcessError, UnicodeError, ValueError, AttributeError):
        return False


def safe_path(path: str) -> bool:
    pure = PurePosixPath(path)
    return not pure.is_absolute() and ".." not in pure.parts and "\\" not in path


def prose_path(path: str) -> bool:
    """Only documentation localities, never arbitrary Markdown source/assets."""
    return (path.endswith(".md") and safe_path(path)
            and (path in ROOT_DOCUMENTS or path.startswith(DOCUMENT_ROOTS))
            and not any(path == p or path.startswith(p) for p in PRODUCT_DOCUMENTS))


def changes(root: Path, merge_base: str, head: str) -> dict[str, tuple[str, str, str]]:
    """{path: (old mode, new mode, status)} for the complete diff; any malformed record raises."""
    raw = git(root, "diff", "--raw", "--no-renames", "--no-abbrev", "-z", merge_base, head, "--")
    if raw and not raw.endswith("\0"):
        raise ValueError("Unterminated raw diff")
    fields = raw[:-1].split("\0") if raw else []
    if len(fields) % 2 or any(not field for field in fields):
        raise ValueError("Malformed raw diff")
    found = {}
    for header, path in zip(fields[0::2], fields[1::2]):
        match = RAW_DIFF_HEADER.fullmatch(header)
        if match is None or path in found:
            raise ValueError("Malformed raw diff entry")
        old_mode, new_mode, _, _, status = match.groups()
        found[path] = (old_mode, new_mode, status)
    return found


def declarative_change(root: Path, head: str, path: str, entry: tuple[str, str, str]) -> str | None:
    """Why a change under execution/ is not a declarative document change, or None."""
    old_mode, new_mode, status = entry
    if not safe_path(path):
        return "unsafe path"
    if any(mode not in {"000000", "100644"} for mode in (old_mode, new_mode)) or status not in "ADM":
        return "not a regular non-executable file before and after"
    if new_mode != "000000":
        if not PurePosixPath(path).name.endswith(DECLARATIVE_SUFFIXES):
            return "not a declarative specification file kind"
        if not text_at(root, head, path):
            return "not UTF-8 text"
    return None


def plan(root: Path, event: str, base: str, head: str) -> dict:
    resolved = git(root, "rev-parse", "--verify", head + "^{commit}").strip()
    result = {"schema": "swbpipe-documentation-ci/v1", "head": resolved,
              "base": None, "paths": [], "mode": "product",
              "reason": "Non-PR or unavailable complete diff uses product verification"}
    if event != "pull_request" or not base:
        return result
    try:
        previous = git(root, "merge-base", base, resolved).strip()
        changed = changes(root, previous, resolved)
        paths = sorted(changed)
        result.update(base=previous, paths=paths)
        if not paths:
            return result
        prose = False
        for path in paths:
            if declarative(path):
                problem = declarative_change(root, resolved, path, changed[path])
                if problem:
                    result["reason"] = f"{path}: {problem}"
                    return result
                continue
            old_mode, new_mode, status = changed[path]
            if not prose_path(path) or status != "M" or old_mode != new_mode or new_mode != "100644":
                result["reason"] = f"{path}: not a declarative specification or an in-place human-document edit"
                return result
            if not text_at(root, previous, path) or not text_at(root, resolved, path):
                result["reason"] = f"{path}: not UTF-8 Markdown text"
                return result
            prose = True
        if prose and not audited_sources(root, resolved):
            result["reason"] = "Document-reader audit is absent or stale; product verification required"
            return result
    except (subprocess.CalledProcessError, UnicodeError, ValueError):
        return result
    result.update(mode="documentation", reason="Complete diff changes declarative specifications or human prose only")
    return result


def _unique(pairs):
    keys = [key for key, _ in pairs]
    if len(keys) != len(set(keys)):
        raise ValueError("duplicate key")
    return dict(pairs)


def structure_problems(root: Path, paths: list[str]) -> list[str]:
    """Parse each changed declarative JSON, YAML and CSV file as it stands in the checkout."""
    problems = []
    for path in paths:
        target = root / path
        if not declarative(path) or not target.is_file() or target.is_symlink():
            continue
        try:
            text = target.read_text(encoding="utf-8")
            if path.endswith(".json"):
                json.loads(text, object_pairs_hook=_unique)
            elif path.endswith((".yaml", ".yml")):
                import yaml
                list(yaml.safe_load_all(text))
            elif path.endswith(".csv"):
                rows = [row for row in csv.reader(io.StringIO(text, newline=""), strict=True) if row]
                if rows and len({len(row) for row in rows}) != 1:
                    raise ValueError("rows differ in field count from the header")
        except Exception as error:  # every parser failure is a document failure
            problems.append(f"{path}: {type(error).__name__}: {error}")
    return problems


def boundary_check(root: Path) -> None:
    """The maintained execution/ boundary guard over the checked-out candidate; a finding fails."""
    subprocess.run([sys.executable, str(ROOT / BOUNDARY), "--root", str(root)], cwd=root, check=True)


def document_checks(root: Path, paths: list[str], *, consumers: bool, boundary: bool = False) -> None:
    """Structure and boundary checks for changed specifications, then their content consumers.
    The boundary guard runs for any execution/ change, and always when `boundary` is set."""
    problems = structure_problems(root, paths)
    if problems:
        raise ValueError("Invalid declarative specification:\n" + "\n".join(problems))
    env = {**os.environ, "PYTHONPATH": str(root)}
    if boundary or any(declarative(path) for path in paths):
        boundary_check(root)
    if consumers:
        commands = []
        for key, command in DOCUMENT_TESTS.items():
            if any(path == key or (key.endswith("/") and path.startswith(key)) for path in paths):
                commands.append(command)
        for command in commands:
            subprocess.run([sys.executable, *command], cwd=root, check=True, env=env)


def require_results(results: dict) -> bool:
    """Missing, cancelled or skipped selected work never supplies a green gate."""
    classification = results.get("changes", {})
    if classification.get("result") != "success":
        return False
    mode = classification.get("outputs", {}).get("mode")
    expected = {"documentation": "skipped", "product": "success"}.get(mode)
    return expected is not None and all(
        results.get(name, {}).get("result") == expected
        for name in ("repository-checks", "piping"))


def verify(root: Path, selected: dict) -> None:
    """Revalidate the exact candidate, then run only its documentation consumers."""
    expected = plan(root, "pull_request", selected.get("base", ""), "HEAD")
    if expected != selected or selected["mode"] != "documentation":
        raise ValueError("Documentation plan differs from the checked-out complete diff")
    document_checks(root, selected["paths"], consumers=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    select = commands.add_parser("plan")
    select.add_argument("--event", required=True)
    select.add_argument("--base", default="")
    select.add_argument("--head", required=True)
    select.add_argument("--output", required=True)
    commands.add_parser("aggregate")
    commands.add_parser("refresh-audit", help="After reviewing document-content readers, bind the inventory to staged source inputs")
    check = commands.add_parser("verify")
    check.add_argument("--plan", required=True)
    documents = commands.add_parser("documents", help="Product route: structure and boundary checks of the complete diff's specifications")
    documents.add_argument("--base", required=True)
    args = parser.parse_args()
    if args.command == "refresh-audit":
        inventory = {"schema": "swbpipe-documentation-inputs/v1", "source_digest": source_digest(ROOT, None)}
        (ROOT / INVENTORY).write_text(json.dumps(inventory, indent=2) + "\n")
        return 0
    if args.command == "aggregate":
        if not require_results(json.loads(os.environ["RESULTS"])):
            print("Selected verification did not succeed.", file=sys.stderr)
            return 1
        return 0
    if args.command == "verify":
        verify(ROOT, json.loads(Path(args.plan).read_text()))
        return 0
    if args.command == "documents":
        # The repository-checks pytest step runs the tools/validation consumers themselves.
        previous = git(ROOT, "merge-base", args.base, "HEAD").strip()
        document_checks(ROOT, sorted(changes(ROOT, previous, "HEAD")), consumers=False, boundary=True)
        return 0
    if git(ROOT, "rev-parse", "HEAD").strip() != git(ROOT, "rev-parse", args.head).strip():
        raise ValueError("Classification must use the checked-out candidate")
    result = plan(ROOT, args.event, args.base, args.head)
    Path(args.output).write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as stream:
            stream.write(f"mode={result['mode']}\nbase={result['base'] or ''}\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
