#!/usr/bin/env python3
"""Tests for the lane-local public-observation check (tools/caepipe_correlation/check_public_observations.py).

Every term here is invented: "Acmepipe" stands for a product name, "Acme Labs" for a vendor, "9.99" for a
version, and the tokens for distinctive observed strings.
"""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools.caepipe_correlation import check_public_observations as check  # noqa: E402

TERMS = """# invented
[names]
Acmepipe
Acme Labs
9.99
[markers]
observed
trial
[tokens]
Widget overflow at
Acmepipe Demo
Loads on Gadgets
"""


def git(repo: Path, *args: str) -> str:
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True, check=True).stdout


def repo_with(tmp_path: Path, files: dict[str, str]) -> Path:
    repo = tmp_path / "repo"
    repo.mkdir(parents=True)
    git(repo, "init", "-q", "-b", "main")
    git(repo, "config", "user.email", "invented@example.invalid")
    git(repo, "config", "user.name", "Invented")
    (repo / "README.md").write_text("base\n", encoding="utf-8")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "base")
    git(repo, "checkout", "-q", "-b", "topic")
    for name, text in files.items():
        path = repo / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "topic change")
    return repo


def run(tmp_path: Path, files: dict[str, str]) -> list[str]:
    repo = repo_with(tmp_path, files)
    terms = check.load_terms(write_terms(tmp_path))
    return check.check_range(repo, "main", "topic", terms)


def write_terms(tmp_path: Path) -> Path:
    tmp_path.mkdir(parents=True, exist_ok=True)
    path = tmp_path / "terms.txt"
    path.write_text(TERMS, encoding="utf-8")
    return path


def test_named_observation_in_prose_is_blocked_and_terms_are_not_printed(tmp_path):
    findings = run(tmp_path, {"doc/design.md": "Intro.\nAcmepipe 9.99 was observed to stop early.\n"})
    assert findings and all("Acmepipe" not in item and "observed" not in item for item in findings)
    assert any(item.startswith("doc/design.md:2:") for item in findings)


def test_generic_wording_passes(tmp_path):
    assert run(tmp_path, {"doc/design.md": "The target tool stops early (private notes).\n"}) == []


def test_name_without_observation_marker_passes(tmp_path):
    assert run(tmp_path, {"doc/design.md": "The Acmepipe manual documents batch runs.\n"}) == []


def test_observed_token_is_blocked_anywhere_in_prose(tmp_path):
    findings = run(tmp_path, {"doc/notes.md": "It reported Widget overflow at startup.\n"})
    assert findings == ["doc/notes.md:1: observed token #1"]


def test_code_literals_pass_but_comments_and_test_names_are_prose(tmp_path):
    code = ('TITLE = "Loads on Gadgets"\n'
            "# Acmepipe trial build quirk\n"
            "def test_acmepipe_trial_quirk():\n"
            "    pass\n")
    findings = run(tmp_path, {"pkg/mod.py": code})
    assert {item.split(":")[1] for item in findings} == {"2", "3"}
    findings = run(tmp_path / "t", {"tests/test_mod.py": "def test_widget_overflow_at_start():\n    pass\n"})
    assert any("observed token #1" in item for item in findings)


def test_identifier_and_path_forms_of_a_name_do_not_match(tmp_path):
    findings = run(tmp_path, {"core/acmepipe_mbf/notes.md": "See core/acmepipe_mbf/ and acmepipe_export; trial.\n"})
    assert findings == []


def test_fixture_data_keeps_layout_but_not_the_product_label(tmp_path):
    findings = run(tmp_path, {"fixtures/x/invented/data.csv": "Acmepipe Demo,MODEL,\nLoads on Gadgets: case,\n"})
    assert findings == ["fixtures/x/invented/data.csv:1: named observed token #2"]


def test_owner_words_are_not_screened(tmp_path):
    text = ("<!-- owner-words -->\n- Owner: \"I installed a trial version of Acmepipe.\"\n<!-- /owner-words -->\n"
            "Acmepipe trial limits were observed.\n")
    findings = run(tmp_path, {"doc/design.md": text})
    assert findings and all(item.startswith("doc/design.md:4:") for item in findings)


def test_commit_messages_are_screened(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "generic\n"})
    (repo / "doc/b.md").write_text("generic\n", encoding="utf-8")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "Acmepipe 9.99 observed limit")
    findings = check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path)))
    assert any(item.startswith("commit ") for item in findings)


def test_whole_file_mode_and_exit_codes(tmp_path):
    terms = write_terms(tmp_path)
    clean = tmp_path / "clean.md"
    clean.write_text("Generic text.\n", encoding="utf-8")
    dirty = tmp_path / "dirty.md"
    dirty.write_text("Acmepipe was observed.\n", encoding="utf-8")
    assert check.main(["--terms-file", str(terms), "--file", str(clean)]) == 0
    assert check.main(["--terms-file", str(terms), "--file", str(dirty)]) == 1
    assert check.main(["--terms-file", str(tmp_path / "missing.txt"), "--file", str(clean)]) == 2


def test_names_match_in_any_case(tmp_path):
    findings = run(tmp_path, {"doc/a.md": "ACMEPIPE was observed.\nacme labs trial notes.\n"})
    assert {item.split(":")[1] for item in findings} == {"1", "2"}


def test_added_line_is_screened_against_unchanged_context(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "The Acmepipe manual documents batch runs.\n"})
    git(repo, "branch", "-f", "main", "topic")
    (repo / "doc/a.md").write_text("The Acmepipe manual documents batch runs.\nIt was observed to stop.\n",
                                   encoding="utf-8")
    git(repo, "commit", "-q", "-am", "more")
    findings = check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path)))
    assert findings == ["doc/a.md:1: name #1 near observation marker #1"]


def test_unchanged_findings_are_not_reported(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "Acmepipe was observed.\n\n\nGeneric.\n"})
    git(repo, "branch", "-f", "main", "topic")
    (repo / "doc/a.md").write_text("Acmepipe was observed.\n\n\nGeneric.\nMore generic text.\n", encoding="utf-8")
    git(repo, "commit", "-q", "-am", "more")
    assert check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path))) == []


def test_tokens_and_names_wrapped_across_lines_match(tmp_path):
    findings = run(tmp_path, {"doc/a.md": "It printed Widget\n  overflow at startup.\nThe Acme\nLabs build was observed.\n"})
    assert "doc/a.md:1: observed token #1" in findings
    assert any(item.startswith("doc/a.md:3: name #2") for item in findings)


def test_non_ascii_and_quoted_file_names_are_screened(tmp_path):
    findings = run(tmp_path, {"doc/na\u00efve \"notes\".md": "Acmepipe was observed.\n"})
    assert findings and all(item.startswith('doc/na\u00efve "notes".md:1:') for item in findings)


def test_one_line_single_quoted_docstring_is_prose(tmp_path):
    code = "def helper():\n    'Acmepipe trial quirk.'\n    return 1\n"
    findings = run(tmp_path, {"pkg/mod.py": code})
    assert {item.split(":")[1] for item in findings} == {"2"}


def test_owner_words_keep_text_outside_the_span_and_reject_unpaired_markers(tmp_path):
    text = '<!-- owner-words -->"Acmepipe trial."<!-- /owner-words --> Acmepipe was observed.\n'
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: name #1 near observation marker #1"]
    with pytest.raises(ValueError):
        run(tmp_path / "u", {"doc/a.md": "<!-- owner-words -->\nAcmepipe was observed.\n"})
    with pytest.raises(ValueError):
        run(tmp_path / "c", {"doc/a.md": "Acmepipe was observed.\n<!-- /owner-words -->\n"})


def test_path_and_file_name_forms_are_masked_but_hyphenated_prose_is_not(tmp_path):
    text = "See `Design/acmepipe-notes.md`, acmepipe-pilot.md and core/acmepipe-x/; the check screens trial text.\n"
    assert run(tmp_path, {"doc/a.md": text}) == []
    findings = run(tmp_path / "h", {"doc/a.md": "Acmepipe-compatible output was observed.\n"})
    assert findings == ["doc/a.md:1: name #1 near observation marker #1"]
    findings = run(tmp_path / "s", {"doc/a.md": "The Acmepipe/Other comparison was observed.\n"})
    assert findings == ["doc/a.md:1: name #1 near observation marker #1"]

    for index, text in enumerate(("Acmepipe/9.99 was observed.\n", "It was observed in Acmepipe/Other.\n")):
        assert "doc/a.md:1: name #1 near observation marker #1" in run(tmp_path / f"p{index}", {"doc/a.md": text})


@pytest.mark.parametrize("name, text", [
    ("pkg/mod.py", "x = 1  # Widget\n# overflow at startup.\n"),
    ("src/lib.rs", "let x = 1; // Widget\n/// overflow at startup.\n"),
    ("tools/a.ps1", "$x = 1 # Widget\n# overflow at startup.\n"),
    ("src/a.ts", "/* Widget\n * overflow at startup. */\n"),
    ("tools/b.ps1", "<# Widget\n#>\n# overflow at startup.\n"),
    ("src/b.rs", "fn f<'a>(x: &'a str) -> &'a str { x } // Widget overflow at startup.\n"),
    ("src/c.rs", "/* outer\n/* inner */\nWidget overflow at startup.\n*/\n"),
    ("src/b.ts", "const r = /\"/g; // Widget overflow at startup.\n"),
    ("tools/c.ps1", "$p = \"C:\\\" # Widget overflow at startup.\n"),
])
def test_wrapped_comments_join_without_comment_syntax(tmp_path, name, text):
    line = 1 + text.split("Widget")[0].count("\n")
    assert run(tmp_path, {name: text}) == [f"{name}:{line}: observed token #1"]


def test_single_quoted_docstring_beside_a_comment_is_prose_but_code_literals_are_not(tmp_path):
    code = ("def helper():\n    'Acmepipe trial quirk.'  # note\n    return 1\n\n\n"
            "def test_values(): 'Acmepipe trial quirk.'\n\n\nTITLE = 'Acmepipe trial'  # generic note\n")
    findings = run(tmp_path, {"pkg/mod.py": code})
    assert {item.split(":")[1] for item in findings} == {"2", "6"}


def test_concatenated_and_non_ascii_docstrings_are_prose(tmp_path):
    code = "def helper(): 'generic ' 'Widget overflow at startup.'\n\n\ndef caf\u00e9(): 'Widget overflow at startup.'\n"
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:1: observed token #1", "pkg/mod.py:4: observed token #1"]


def test_markdown_link_destinations_are_masked(tmp_path):
    assert run(tmp_path, {"doc/a.md": "See [details](Design/acmepipe-notes.md); trial notes.\n"}) == []
    findings = run(tmp_path / "t", {"doc/a.md": "See [Acmepipe](Design/notes.md); it was observed.\n"})
    assert findings == ["doc/a.md:1: name #1 near observation marker #1"]


def test_concatenated_docstring_fragments_join(tmp_path):
    code = "def helper():\n    'Widget ' 'overflow at startup.'\n"
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:2: observed token #1"]


@pytest.mark.parametrize("text", ["Widget **overflow** at startup.\n", "> Widget\n> overflow at startup.\n",
                                  "Widget `overflow` _at_ startup.\n"])
def test_markdown_formatting_does_not_hide_a_phrase(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: observed token #1"]


@pytest.mark.parametrize("name, text", [
    ("doc/a.md", "Widget [overflow](errors.md) at startup.\n"),
    ("doc/b.md", "Widget [overflow][ref] at startup.\n"),
    ("src/lib.rs", "/// Widget **overflow** at startup.\nfn f() {}\n"),
    ("pkg/mod.py", "def helper():\n    \"\"\"Widget `overflow` at startup.\"\"\"\n"),
    ("pkg/two.py", "x = 1  # Widget [overflow](notes.md) at startup.\n"),
])
def test_links_and_formatting_do_not_hide_a_phrase_in_any_prose(tmp_path, name, text):
    line = 1 + text.split("Widget")[0].count("\n")
    assert run(tmp_path, {name: text}) == [f"{name}:{line}: observed token #1"]
