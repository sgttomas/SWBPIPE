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


# The real guard checks the real repository (tools/validation/test_execution_boundary.py); here record calls.
GUARDED = []


@pytest.fixture
def repo(tmp_path, monkeypatch):
    git(tmp_path, "init", "-q")
    git(tmp_path, "config", "user.email", "test@example.invalid")
    git(tmp_path, "config", "user.name", "Documentation CI fixture")
    for path in ("README.md", "docs/note.md", "core/code.py",
                 "execution/PKG-04/DEL-04-08/ScopeOfWork.md",
                 "apps/desktop/src-tauri/resources/instructions.md",
                 "docs/claims_registry.md", "docs/THIRD_PARTY_NOTICES.md",
                 "execution/PKG-01/DEL-01-01/deliverable.yaml", "execution/_Coordination/data.json",
                 "execution/_Coordination/rows.csv"):
        file = tmp_path / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text({".json": "{}\n", ".csv": "a\n"}.get(file.suffix, "before\n"))
    git(tmp_path, "add", ".")
    audit(tmp_path)
    git(tmp_path, "commit", "-qm", "base")
    del GUARDED[:]
    monkeypatch.setattr(docs, "boundary_check", GUARDED.append)
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
    (root / "execution/PKG-04/DEL-04-08/ScopeOfWork.md").write_text("X1–X3\n")
    head = commit(root)
    selected = docs.plan(root, "pull_request", base, head)
    assert selected["mode"] == "documentation"
    assert selected["paths"] == ["docs/note.md", "execution/PKG-04/DEL-04-08/ScopeOfWork.md"]
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
    for event, previous in [("workflow_dispatch", base), ("pull_request", ""), ("pull_request", "missing"),
                            ("merge_group", ""), ("merge_group", "missing")]:
        assert docs.plan(root, event, previous, head)["mode"] == "product"


def queue(root, base, path, content):
    """The merge queue's temporary merge commit of a PR changing `path` onto a
    queue base that has advanced since the PR forked."""
    git(root, "checkout", "-qb", "candidate")
    (root / path).write_text(content)
    commit(root)
    git(root, "checkout", "-qb", "queue-base", base)
    (root / "execution/_Coordination/rows.csv").write_text("a\nb\n")
    queue_base = commit(root)
    git(root, "merge", "-q", "--no-ff", "--no-edit", "candidate")
    return queue_base, git(root, "rev-parse", "HEAD")


def test_merge_group_classifies_the_queued_diff_as_a_pull_request(repo):
    root, base = repo
    queue_base, head = queue(root, base, "docs/note.md", "queued prose\n")
    selected = docs.plan(root, "merge_group", queue_base, head)
    assert selected == docs.plan(root, "pull_request", queue_base, head)
    assert (selected["mode"], selected["base"], selected["paths"]) == ("documentation", queue_base, ["docs/note.md"])
    docs.verify(root, selected)
    with pytest.raises(ValueError):
        docs.verify(root, {**selected, "paths": []})


def test_merge_group_product_change_keeps_product_verification(repo):
    root, base = repo
    queue_base, head = queue(root, base, "core/code.py", "queued code\n")
    selected = docs.plan(root, "merge_group", queue_base, head)
    assert "core/code.py" in selected["paths"]
    assert not any(path.startswith("execution/") for path in selected["paths"])  # the base's advance
    assert selected == docs.plan(root, "pull_request", queue_base, head)
    assert selected["mode"] == "product"


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


@pytest.mark.parametrize("change", ["yaml", "json", "csv", "add", "delete", "rename", "mixed-kinds"])
def test_declarative_specification_changes_are_documents_without_an_audit_refresh(repo, change):
    root, base = repo
    work = root / "execution/PKG-01/DEL-01-01"
    records = root / "execution/_Coordination"
    if change == "yaml":
        (work / "deliverable.yaml").write_text("id: DEL-01-01\n")
    elif change == "json":
        (records / "data.json").write_text('{"a": 1}\n')
    elif change == "csv":
        (records / "rows.csv").write_text("a,b\n1,2\n")
    elif change == "add":
        (root / "execution/PKG-02/DEL-02-01/ScopeOfWork.md").parent.mkdir(parents=True)
        (root / "execution/PKG-02/DEL-02-01/ScopeOfWork.md").write_text("new\n")
    elif change == "delete":
        (records / "rows.csv").unlink()
    elif change == "rename":
        (records / "data.json").rename(records / "renamed.json")
    else:
        (work / "deliverable.yaml").write_text("id: DEL-01-01\n")
        (records / "notes.txt").write_text("notes\n")
        (records / "hashes.sha256").write_text("0" * 64 + "  x\n")
    head = commit(root, refresh=False)
    assert docs.source_digest(root, head) == docs.source_digest(root, base)
    selected = docs.plan(root, "pull_request", base, head)
    assert selected["mode"] == "documentation", selected["reason"]
    del GUARDED[:]
    docs.verify(root, selected)
    assert GUARDED == [root]


@pytest.mark.parametrize("change", ["python", "script", "executable", "symlink", "binary", "code-and-spec", "unknown"])
def test_runnable_or_mixed_execution_changes_keep_product_checks(repo, change):
    root, base = repo
    records = root / "execution/_Coordination"
    if change == "python":
        (records / "derive.py").write_text("print(1)\n")
    elif change == "script":
        (records / "run.sh").write_text("echo\n")
    elif change == "executable":
        (records / "data.json").chmod(0o755)
    elif change == "symlink":
        (records / "data.json").unlink()
        (records / "data.json").symlink_to("rows.csv")
    elif change == "binary":
        (records / "data.json").write_bytes(b"\x00\xff")
    elif change == "code-and-spec":
        (records / "data.json").write_text('{"b": 2}\n')
        (root / "core/code.py").write_text("changed\n")
    else:
        (root / "unknown.cfg").write_text("x\n")
        (records / "data.json").write_text('{"b": 2}\n')
    assert docs.plan(root, "pull_request", base, commit(root, refresh=False))["mode"] == "product"


@pytest.mark.parametrize("path,text", [("execution/_Coordination/data.json", '{"a": 1, "a": 2}'), ("execution/_Coordination/data.json", "{"),
                                       ("execution/PKG-01/DEL-01-01/deliverable.yaml", "a: [1\n"), ("execution/_Coordination/rows.csv", "a,b\n1,2,3\n")])
def test_invalid_declarative_specifications_fail_document_checks(repo, path, text):
    root, base = repo
    (root / path).write_text(text)
    selected = docs.plan(root, "pull_request", base, commit(root, refresh=False))
    assert selected["mode"] == "documentation"
    with pytest.raises(ValueError, match="Invalid declarative specification"):
        docs.verify(root, selected)


def test_boundary_failure_and_selected_consumers_fail_the_document_route(repo, monkeypatch):
    root, base = repo
    (root / "execution/_Fixture").mkdir(parents=True)
    (root / "execution/_Fixture/note.md").write_text("note\n")
    selected = docs.plan(root, "pull_request", base, commit(root, refresh=False))
    assert selected["mode"] == "documentation"
    monkeypatch.setitem(docs.DOCUMENT_TESTS, "execution/_Fixture/", ["-c", "raise SystemExit(1)"])
    with pytest.raises(subprocess.CalledProcessError):
        docs.verify(root, selected)
    monkeypatch.setitem(docs.DOCUMENT_TESTS, "execution/_Fixture/", ["-c", "pass"])

    def fail(root):
        raise subprocess.CalledProcessError(1, "execution_boundary")
    monkeypatch.setattr(docs, "boundary_check", fail)
    with pytest.raises(subprocess.CalledProcessError):
        docs.verify(root, selected)


def test_the_real_guard_rejects_a_runnable_specification(tmp_path):
    """The verify path's boundary_check runs the maintained guard, not a stand-in."""
    git(tmp_path, "init", "-q")
    (tmp_path / "execution/PKG-01").mkdir(parents=True)
    (tmp_path / "execution/PKG-01/derive.py").write_text("print(1)\n")
    git(tmp_path, "add", ".")
    with pytest.raises(subprocess.CalledProcessError):
        docs.boundary_check(tmp_path)
