"""Complete-diff and fail-closed boundaries of the prose verification route."""
import json
import os
from pathlib import Path
import subprocess

import pytest

import documentation_ci as docs


def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


def audit(root):
    inventory = root / docs.INVENTORY
    inventory.parent.mkdir(parents=True, exist_ok=True)
    inventory.write_text(json.dumps({"schema": "swbpipe-documentation-inputs/v1",
                                     "source_digest": docs.source_digest(root, None)}))
    git(root, "add", docs.INVENTORY)


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
    audit(tmp_path)
    git(tmp_path, "commit", "-qm", "base")
    return tmp_path, git(tmp_path, "rev-parse", "HEAD")


def commit(root, refresh=True):
    git(root, "add", "-A")
    if refresh:
        audit(root)
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


@pytest.mark.parametrize("change", ["new-reader", "changed-reader", "config", "new-markdown", "missing-audit", "invalid-audit"])
def test_future_or_unaudited_readers_cannot_silently_enter_documentation_route(repo, change):
    root, _ = repo
    if change == "new-reader":
        (root / "core/reader.py").write_text("from pathlib import Path\nprint(Path('docs/note.md').read_text())\n")
    elif change == "changed-reader":
        (root / "core/code.py").write_text("from pathlib import Path\nprint(Path('docs/note.md').read_text())\n")
    elif change == "config":
        (root / "core/readers.json").write_text('{"input": "docs/note.md"}')
    elif change == "new-markdown":
        (root / "docs/new-input.md").write_text("a new input to an existing directory scan\n")
    elif change == "missing-audit":
        (root / docs.INVENTORY).unlink()
    else:
        (root / docs.INVENTORY).write_text("invalid")
    # This source change is already on the PR base. Checking only the PR diff
    # would miss that its new reader was never included in the content audit.
    base = commit(root, refresh=False)
    (root / "docs/note.md").write_text("subsequent prose change\n")
    selected = docs.plan(root, "pull_request", base, commit(root, refresh=False))
    assert selected["mode"] == "product"
    assert "audit" in selected["reason"]


def test_inventory_is_bound_to_the_committed_source_not_worktree_edits(repo):
    root, base = repo
    (root / "docs/note.md").write_text("prose\n")
    head = commit(root)
    (root / "core/code.py").write_text("unrelated local work\n")
    assert docs.plan(root, "pull_request", base, head)["mode"] == "documentation"


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
