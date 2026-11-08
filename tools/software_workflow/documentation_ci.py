#!/usr/bin/env python3
"""Bounded verification for human documentation; executable Markdown keeps product CI.

This is a separate input contract, not a change to Python's conservative source
analysis. Runtime resources, fixtures, licences and document-content consumers
are not plain prose merely because their names end in .md.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys

from hosted_ci import changed_paths, git, text_at

ROOT = Path(__file__).resolve().parents[2]
# Keep source-bound resources and verification inputs on their owning route.
# In particular, embedded-agent assets under apps/ are never human docs here.
DOCUMENT_ROOTS = ("docs/", "loop/", "execution/")
ROOT_DOCUMENTS = {"README.md", "AGENTS.md"}
PRODUCT_DOCUMENTS = (
    "docs/third-party/", "docs/THIRD_PARTY_NOTICES.md",
    "docs/claims_registry.md",  # Vitest reads the claims as a contract input
    "execution/_ScopeChange/",  # preserved source/evidence assets
)
# Content checks are selected at their actual document-reading boundary. They
# need no Rust session authorities; pytest runs without the product conftest.
DOCUMENT_TESTS = {
    "docs/security/threat_model.md": ["-m", "pytest", "--noconftest", "-q", "tests/test_security_threat_model.py"],
    "docs/security/redaction_export_controls.md": ["-m", "pytest", "--noconftest", "-q", "tests/security/test_redaction_export_controls.py::test_changed_files_do_not_embed_disallowed_example_content"],
    "docs/security/secret_private_library_handling.md": ["-m", "pytest", "--noconftest", "-q", "tests/security/test_secret_private_library_handling.py::test_changed_files_do_not_embed_disallowed_example_content"],
    "docs/architecture/plugin_boundary.md": ["tests/test_api_boundary_contract.py"],
    "docs/local_analysis/local_fea_handoff_guidance.md": ["tests/test_local_fea_handoff_contract.py"],
    "execution/PKG-04/DEL-04-11/Work/hanger-sizing.md": ["-m", "pytest", "--noconftest", "-q", "tests/test_hanger_sizing_evidence.py::test_the_verifiers_fixed_lists_are_the_schemas_and_the_designs"],
}


def prose_path(path: str) -> bool:
    """Only documentation localities, never arbitrary Markdown source/assets."""
    return (path.endswith(".md") and not PurePosixPath(path).is_absolute()
            and ".." not in PurePosixPath(path).parts
            and (path in ROOT_DOCUMENTS or path.startswith(DOCUMENT_ROOTS))
            and not any(path == p or path.startswith(p) for p in PRODUCT_DOCUMENTS))


def plan(root: Path, event: str, base: str, head: str) -> dict:
    resolved = git(root, "rev-parse", "--verify", head + "^{commit}").strip()
    result = {"schema": "swbpipe-documentation-ci/v1", "head": resolved,
              "base": None, "paths": [], "mode": "product",
              "reason": "Non-PR or unavailable complete diff uses product verification"}
    if event != "pull_request" or not base:
        return result
    try:
        previous = git(root, "merge-base", base, resolved).strip()
        paths, edits = changed_paths(root, previous, resolved)
        result.update(base=previous, paths=paths)
        if not paths:
            return result
        for path in paths:
            if not prose_path(path) or path not in edits:
                result["reason"] = f"{path}: not an in-place human-document edit"
                return result
            if not text_at(root, previous, path) or not text_at(root, resolved, path):
                result["reason"] = f"{path}: not UTF-8 Markdown text"
                return result
    except (subprocess.CalledProcessError, UnicodeError, ValueError):
        return result
    result.update(mode="documentation", reason="Complete diff changes human prose only")
    return result


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
    checks = [DOCUMENT_TESTS[p] for p in selected["paths"] if p in DOCUMENT_TESTS]
    for command in checks:
        subprocess.run([sys.executable, *command], cwd=root, check=True,
                       env={**os.environ, "PYTHONPATH": str(root)})


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    select = commands.add_parser("plan")
    select.add_argument("--event", required=True)
    select.add_argument("--base", default="")
    select.add_argument("--head", required=True)
    select.add_argument("--output", required=True)
    commands.add_parser("aggregate")
    check = commands.add_parser("verify")
    check.add_argument("--plan", required=True)
    args = parser.parse_args()
    if args.command == "aggregate":
        if not require_results(json.loads(os.environ["RESULTS"])):
            print("Selected verification did not succeed.", file=sys.stderr)
            return 1
        return 0
    if args.command == "verify":
        verify(ROOT, json.loads(Path(args.plan).read_text()))
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
