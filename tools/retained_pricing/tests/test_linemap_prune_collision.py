"""A deleted old key cannot overwrite/prune a different live mapped key."""
import json
import os
from pathlib import Path
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
P = "projects/chirality-piping/"
F = "core/x/src/lib.rs"


@pytest.mark.parametrize("deleted_first", [False, True])
def test_pruning_uses_original_dictionary_rule_identity(tmp_path, deleted_first):
    repo = tmp_path / "repo"
    repo.mkdir()
    env = dict(os.environ, GIT_AUTHOR_NAME="test", GIT_AUTHOR_EMAIL="test@example.invalid",
               GIT_COMMITTER_NAME="test", GIT_COMMITTER_EMAIL="test@example.invalid",
               GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM="1")
    def git(*args):
        return subprocess.check_output(["git", "-C", str(repo), *args], env=env, text=True).strip()
    git("init", "-q")
    source = repo / P / F
    source.parent.mkdir(parents=True)
    def commit(text):
        source.write_text(text)
        git("add", ".")
        git("commit", "-qm", "fixture")
        return git("rev-parse", "HEAD")
    old = commit('fn live() {\n    let s = format!("live");\n}\nfn gone() {\n    let s = format!("gone");\n}\n')
    new = commit('// one\n// two\n// three\nfn live() {\n    let s = format!("live");\n}\n')
    rules = tmp_path / "rules.json"
    entries = [(F + ":2", {"live": 10}), (F + ":5", {"gone": 20})]
    if deleted_first:
        entries.reverse()
    rules.write_text(json.dumps({"id_audit": dict(entries)}))
    crates = tmp_path / "crates.txt"
    crates.write_text("core/x/src\n")
    out = tmp_path / "mapped"
    result = subprocess.run([sys.executable, str(PRICING / "lib/linemap_rules.py"), "--prune-deleted",
        str(repo), old, new, str(out), str(rules)],
        env=dict(env, LINEMAP_CRATES=str(crates)), text=True, capture_output=True)
    assert result.returncode == 0, result.stdout + result.stderr
    assert json.loads((out / "rules.json").read_text())["id_audit"] == {F + ":5": {"live": 10}}
    summary = json.loads((out / "g7_linemap.out.json").read_text())["summary"]
    assert not summary["unmapped"]
    assert [(p["key"], p["rule"]) for p in summary["pruned_deleted"]] == [(F + ":5", {"gone": 20})]
