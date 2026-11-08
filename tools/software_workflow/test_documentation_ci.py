"""Complete-diff and fail-closed boundaries of the prose verification route."""
import json
import os
from pathlib import Path
import subprocess

import pytest

import documentation_ci as docs


def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


@pytest.fixture
def repo(tmp_path):
    git(tmp_path, "init", "-q")
    git(tmp_path, "config", "user.email", "test@example.invalid")
    git(tmp_path, "config", "user.name", "Documentation CI fixture")
    for path in ("README.md", "docs/note.md", "core/code.py",
                 "execution/PKG-04/DEL-04-08/Work/retained-pricing-rules.md",
                 "apps/desktop/src-tauri/resources/instructions.md",
                 "docs/claims_registry.md", "docs/THIRD_PARTY_NOTICES.md"):
        file = tmp_path / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text("before\n")
    git(tmp_path, "add", ".")
    git(tmp_path, "commit", "-qm", "base")
    return tmp_path, git(tmp_path, "rev-parse", "HEAD")


def commit(root):
    git(root, "add", "-A")
    git(root, "commit", "-qm", "candidate")
    return git(root, "rev-parse", "HEAD")


def test_pricing_prose_and_multiple_commits_use_the_complete_diff(repo):
    root, base = repo
    (root / "docs/note.md").write_text("first\n")
    commit(root)
    (root / "execution/PKG-04/DEL-04-08/Work/retained-pricing-rules.md").write_text("X1–X3\n")
    head = commit(root)
    selected = docs.plan(root, "pull_request", base, head)
    assert selected["mode"] == "documentation"
    assert selected["paths"] == ["docs/note.md", "execution/PKG-04/DEL-04-08/Work/retained-pricing-rules.md"]
    docs.verify(root, selected)
    # An omitted path or stale head must not be accepted as a documentation pass.
    for wrong in ({**selected, "paths": ["docs/note.md"]}, {**selected, "head": base}):
        with pytest.raises(ValueError):
            docs.verify(root, wrong)


@pytest.mark.parametrize("path", ["core/code.py", "apps/desktop/src-tauri/resources/instructions.md",
                                 "docs/claims_registry.md", "docs/THIRD_PARTY_NOTICES.md"])
def test_code_mixed_changes_and_runtime_documents_keep_product_checks(repo, path):
    root, base = repo
    (root / "README.md").write_text("updated prose\n")
    (root / path).write_text("changed input\n")
    assert docs.plan(root, "pull_request", base, commit(root))["mode"] == "product"


@pytest.mark.parametrize("change", ["add", "delete", "rename", "symlink", "executable", "binary", "empty"])
def test_structural_and_unreadable_changes_do_not_claim_prose_only(repo, change):
    root, base = repo
    path = root / "docs/note.md"
    if change == "add":
        (root / "docs/new.md").write_text("new\n")
    elif change == "delete":
        path.unlink()
    elif change == "rename":
        path.rename(root / "docs/renamed.md")
    elif change == "symlink":
        path.unlink()
        path.symlink_to("../README.md")
    elif change == "executable":
        path.chmod(0o755)
    elif change == "binary":
        path.write_bytes(b"bad\x00\xff")
    head = base if change == "empty" else commit(root)
    assert docs.plan(root, "pull_request", base, head)["mode"] == "product"


def test_missing_diff_or_dispatch_keeps_product_verification(repo):
    root, base = repo
    (root / "README.md").write_text("changed\n")
    head = commit(root)
    for event, previous in [("workflow_dispatch", base), ("pull_request", ""), ("pull_request", "missing")]:
        assert docs.plan(root, event, previous, head)["mode"] == "product"


def test_failed_document_consumer_cannot_pass(monkeypatch, repo):
    root, base = repo
    (root / "docs/note.md").write_text("changed\n")
    head = commit(root)
    monkeypatch.setitem(docs.DOCUMENT_TESTS, "docs/note.md", ["-c", "raise SystemExit(1)"])
    with pytest.raises(subprocess.CalledProcessError):
        docs.verify(root, docs.plan(root, "pull_request", base, head))


@pytest.mark.parametrize("mode", ["documentation", "product"])
def test_aggregate_requires_classification_and_exact_selected_job_states(mode):
    expected = "skipped" if mode == "documentation" else "success"
    results = {"changes": {"result": "success", "outputs": {"mode": mode}},
               "repository-checks": {"result": expected}, "piping": {"result": expected}}
    assert docs.require_results(results)
    for name in results:
        for state in ("success", "failure", "cancelled", "skipped", None):
            changed = json.loads(json.dumps(results))
            changed[name]["result"] = state
            assert docs.require_results(changed) == (state == results[name]["result"])
        changed = dict(results)
        del changed[name]
        assert not docs.require_results(changed)
    for output in ({}, {"mode": "invalid"}):
        assert not docs.require_results({**results, "changes": {"result": "success", "outputs": output}})
