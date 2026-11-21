"""The known former table digests are the repository history's, in all three copies (DEL-05-02,
DEL-14-02). Runs in the Repository checks job, which clones full history without a blob filter."""
import subprocess

import pytest

from tools.validation import former_table_digests as tool


def test_the_three_copies_are_the_tables_history():
    assert tool.problems(tool.derive()) == []


def test_no_row_is_the_digest_of_an_installed_table():
    installed = {line.split()[2] for line in tool.git("ls-tree", "-r", "HEAD", "--", tool.TABLE_DIRECTORY).splitlines()
                 if tool.fnmatchcase(line.split("\t")[1], tool.TABLE_FILES)}
    digests = {tool.hashlib.sha256(tool.git("cat-file", "blob", blob, binary=True)).hexdigest() for blob in installed}
    rows = tool.derive()
    assert rows and not {digest for _, digest in rows} & digests
    assert len(rows) == len(set(rows))


def refusing(command_word, stdout=b"", code=0):
    real = subprocess.run

    def run(command, *args, **kwargs):
        if command[:2] == ["git", "--no-lazy-fetch"] and command_word in command:
            return subprocess.CompletedProcess(command, code, stdout, b"refused here")
        return real(command, *args, **kwargs)
    return run


def test_a_shallow_clone_is_refused_not_answered_with_a_short_list(monkeypatch):
    monkeypatch.setattr(tool.subprocess, "run", refusing("--is-shallow-repository", b"true\n"))
    with pytest.raises(tool.HistoryUnavailable, match="shallow clone"):
        tool.derive()
    assert tool.main(["check"]) == 2


def test_a_table_revision_missing_from_a_partial_clone_is_refused(monkeypatch):
    monkeypatch.setattr(tool.subprocess, "run", refusing("cat-file", code=128))
    with pytest.raises(tool.HistoryUnavailable, match="not held here"):
        tool.derive()


def test_a_missing_main_branch_is_refused(monkeypatch):
    with pytest.raises(tool.HistoryUnavailable, match="no main branch"):
        tool.derive("refs/heads/no-such-branch-for-this-test")
