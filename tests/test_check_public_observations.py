#!/usr/bin/env python3
"""Tests for the lane-local public-observation check (tools/caepipe_correlation/check_public_observations.py).

Every term here is invented: "Acmepipe" stands for a product name, "Acme Labs" for a vendor, "9.99" for a
version, and the tokens for distinctive strings from private evaluation notes. "Crashing" stands in for any
defect or limit language about the target.
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
    findings = run(tmp_path, {"doc/design.md": "Intro.\nAcmepipe 9.99 was crashing to stop early.\n"})
    assert findings and all("Acmepipe" not in item and "crashing" not in item for item in findings)
    assert any(item.startswith("doc/design.md:2:") for item in findings)


def test_generic_wording_about_the_target_is_still_screened(tmp_path):
    assert run(tmp_path, {"doc/design.md": "The target tool stops early (private notes).\n"}) == [
        "doc/design.md:1: defect or limit about the target tool"]


def test_name_without_observation_marker_passes(tmp_path):
    assert run(tmp_path, {"doc/design.md": "The Acmepipe manual documents batch runs.\n"}) == []


def test_observed_token_is_blocked_anywhere_in_prose(tmp_path):
    findings = run(tmp_path, {"doc/notes.md": "It reported Widget overflow at startup.\n"})
    assert findings == ["doc/notes.md:1: private token #1"]


def test_code_literals_pass_but_comments_and_test_names_are_prose(tmp_path):
    code = ('TITLE = "Loads on Gadgets"\n'
            "# Acmepipe trial build quirk\n"
            "X = 1\n"
            "def test_acmepipe_trial_quirk():\n"
            "    pass\n")
    findings = run(tmp_path, {"pkg/mod.py": code})
    assert {item.split(":")[1] for item in findings} == {"2", "4"}
    findings = run(tmp_path / "t", {"tests/test_mod.py": "def test_widget_overflow_at_start():\n    pass\n"})
    assert any("private token #1" in item for item in findings)


def test_identifier_and_path_forms_of_a_name_do_not_match(tmp_path):
    findings = run(tmp_path, {"core/acmepipe_mbf/notes.md": "See core/acmepipe_mbf/ and acmepipe_export; trial.\n"})
    assert findings == []


def test_private_tokens_are_blocked_on_every_fixture_line_but_the_product_name_is_not(tmp_path):
    findings = run(tmp_path, {"fixtures/x/invented/data.csv": "Acmepipe Demo,MODEL,\nLoads on Gadgets: case,\nAcmepipe,9.99,\n"})
    assert findings == ["fixtures/x/invented/data.csv:1: private token #2", "fixtures/x/invented/data.csv:2: private token #3"]


def test_owner_words_are_not_screened(tmp_path):
    text = ("<!-- owner-words -->\n- Owner: \"I installed a trial version of Acmepipe.\"\n<!-- /owner-words -->\n"
            "Acmepipe trial limits were crashing.\n")
    findings = run(tmp_path, {"doc/design.md": text})
    assert findings and all(item.startswith("doc/design.md:4:") for item in findings)


def test_commit_messages_are_screened(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "generic\n"})
    (repo / "doc/b.md").write_text("generic\n", encoding="utf-8")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "Acmepipe 9.99 crashing limit")
    findings = check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path)))
    assert any(item.startswith("commit ") for item in findings)


def test_whole_file_mode_and_exit_codes(tmp_path):
    terms = write_terms(tmp_path)
    clean = tmp_path / "clean.md"
    clean.write_text("Generic text.\n", encoding="utf-8")
    dirty = tmp_path / "dirty.md"
    dirty.write_text("Acmepipe was crashing.\n", encoding="utf-8")
    assert check.main(["--terms-file", str(terms), "--file", str(clean)]) == 0
    assert check.main(["--terms-file", str(terms), "--file", str(dirty)]) == 1
    assert check.main(["--terms-file", str(tmp_path / "missing.txt"), "--file", str(clean)]) == 2


def test_names_match_in_any_case(tmp_path):
    findings = run(tmp_path, {"doc/a.md": "ACMEPIPE was crashing.\n\nacme labs trial notes.\n"})
    assert {item.split(":")[1] for item in findings} == {"1", "3"}


def test_added_line_is_screened_against_unchanged_context(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "The Acmepipe manual documents batch runs.\n"})
    git(repo, "branch", "-f", "main", "topic")
    (repo / "doc/a.md").write_text("The Acmepipe manual documents batch runs.\nIt was crashing to stop.\n",
                                   encoding="utf-8")
    git(repo, "commit", "-q", "-am", "more")
    findings = check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path)))
    assert findings == ["doc/a.md:1: defect or limit about the target tool"]


def test_unchanged_findings_are_not_reported(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "Acmepipe was crashing.\n\n\nGeneric.\n"})
    git(repo, "branch", "-f", "main", "topic")
    (repo / "doc/a.md").write_text("Acmepipe was crashing.\n\n\nGeneric.\nMore generic text.\n", encoding="utf-8")
    git(repo, "commit", "-q", "-am", "more")
    assert check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path))) == []


def test_tokens_and_names_wrapped_across_lines_match(tmp_path):
    findings = run(tmp_path, {"doc/a.md": "It printed Widget\n  overflow at startup.\nThe Acme\nLabs build was crashing.\n"})
    assert "doc/a.md:1: private token #1" in findings
    assert "doc/a.md:3: defect or limit about the target tool" in findings


def test_non_ascii_and_quoted_file_names_are_screened(tmp_path):
    findings = run(tmp_path, {"doc/na\u00efve \"notes\".md": "Acmepipe was crashing.\n"})
    assert findings and all(item.startswith('doc/na\u00efve "notes".md:1:') for item in findings)


def test_one_line_single_quoted_docstring_is_prose(tmp_path):
    code = "def helper():\n    'Acmepipe trial quirk.'\n    return 1\n"
    findings = run(tmp_path, {"pkg/mod.py": code})
    assert {item.split(":")[1] for item in findings} == {"2"}


def test_owner_words_keep_text_outside_the_span_and_reject_unpaired_markers(tmp_path):
    text = '<!-- owner-words -->"Acmepipe trial."<!-- /owner-words --> Acmepipe was crashing.\n'
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: defect or limit about the target tool"]
    with pytest.raises(ValueError):
        run(tmp_path / "u", {"doc/a.md": "<!-- owner-words -->\nAcmepipe was crashing.\n"})
    with pytest.raises(ValueError):
        run(tmp_path / "c", {"doc/a.md": "Acmepipe was crashing.\n<!-- /owner-words -->\n"})


def test_path_and_file_name_forms_are_masked_but_hyphenated_prose_is_not(tmp_path):
    text = "See `Design/acmepipe-notes.md`, acmepipe-pilot.md and core/acmepipe-x/; the check screens trial text.\n"
    assert run(tmp_path, {"doc/a.md": text}) == []
    findings = run(tmp_path / "h", {"doc/a.md": "Acmepipe-compatible output was crashing.\n"})
    assert findings == ["doc/a.md:1: defect or limit about the target tool"]
    findings = run(tmp_path / "s", {"doc/a.md": "The Acmepipe/Other comparison was crashing.\n"})
    assert findings == ["doc/a.md:1: defect or limit about the target tool"]

    for index, text in enumerate(("Acmepipe/9.99 was crashing.\n", "It was crashing in Acmepipe/Other.\n")):
        assert "doc/a.md:1: defect or limit about the target tool" in run(tmp_path / f"p{index}", {"doc/a.md": text})


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
    assert run(tmp_path, {name: text}) == [f"{name}:{line}: private token #1"]


def test_single_quoted_docstring_beside_a_comment_is_prose_but_code_literals_are_not(tmp_path):
    code = ("def helper():\n    'Acmepipe trial quirk.'  # note\n    return 1\n\n\n"
            "def test_values(): 'Acmepipe trial quirk.'\n\n\nTITLE = 'Acmepipe trial'  # generic note\n")
    findings = run(tmp_path, {"pkg/mod.py": code})
    assert {item.split(":")[1] for item in findings} == {"2", "6"}


def test_concatenated_and_non_ascii_docstrings_are_prose(tmp_path):
    code = "def helper(): 'generic ' 'Widget overflow at startup.'\n\n\ndef caf\u00e9(): 'Widget overflow at startup.'\n"
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:1: private token #1", "pkg/mod.py:4: private token #1"]


def test_markdown_link_destinations_are_masked(tmp_path):
    assert run(tmp_path, {"doc/a.md": "See [details](Design/acmepipe-notes.md); trial notes.\n"}) == []
    findings = run(tmp_path / "t", {"doc/a.md": "See [Acmepipe](Design/notes.md); it was crashing.\n"})
    assert findings == ["doc/a.md:1: defect or limit about the target tool"]


def test_concatenated_docstring_fragments_join(tmp_path):
    code = "def helper():\n    'Widget ' 'overflow at startup.'\n"
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:2: private token #1"]


@pytest.mark.parametrize("text", ["Widget **overflow** at startup.\n", "> Widget\n> overflow at startup.\n",
                                  "Widget `overflow` _at_ startup.\n"])
def test_markdown_formatting_does_not_hide_a_phrase(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: private token #1"]


@pytest.mark.parametrize("name, text", [
    ("doc/a.md", "Widget [overflow](errors.md) at startup.\n"),
    ("doc/b.md", "Widget [overflow][ref] at startup.\n"),
    ("src/lib.rs", "/// Widget **overflow** at startup.\nfn f() {}\n"),
    ("pkg/mod.py", "def helper():\n    \"\"\"Widget `overflow` at startup.\"\"\"\n"),
    ("pkg/two.py", "x = 1  # Widget [overflow](notes.md) at startup.\n"),
])
def test_links_and_formatting_do_not_hide_a_phrase_in_any_prose(tmp_path, name, text):
    line = 1 + text.split("Widget")[0].count("\n")
    assert run(tmp_path, {name: text}) == [f"{name}:{line}: private token #1"]



@pytest.mark.parametrize("text", [
    "Widget [overflow] at startup.\n\n[overflow]: notes.md\n",
    "Widget [overflow][ref] at startup.\n\n[ref]: notes.md\n",
    "Widget ![overflow](figure.png \"title\") at startup.\n",
    "Widget [overflow](https://example.invalid/a_(b)) at startup.\n",
    "Widget <em>overflow</em> at startup.\n",
    "Widget&nbsp;overflow&#32;at startup.\n",
])
def test_markdown_is_read_as_visible_text(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: private token #1"]


def test_a_link_wrapped_across_lines_keeps_its_label(tmp_path):
    findings = run(tmp_path, {"doc/a.md": "It printed Widget [overflow\nat](notes.md) startup.\n"})
    assert findings == ["doc/a.md:1: private token #1"]


@pytest.mark.parametrize("name, text", [
    ("doc/a.md", "The Acmepipe run on the\ninvented model, with its default settings,\nwas crashing to stop.\n"),
    ("pkg/mod.py", "# The Acmepipe run on the\n# invented model, with its default settings,\n# was crashing to stop.\n"),
    ("src/lib.rs", "// The Acmepipe run on the\n// invented model, with its default settings,\n// was crashing to stop.\n"),
    ("src/a.ts", "/* The Acmepipe run on the\n * invented model, with its default settings,\n * was crashing to stop. */\n"),
    ("tools/a.ps1", "# The Acmepipe run on the\n# invented model, with its default settings,\n# was crashing to stop.\n"),
])
def test_a_name_and_a_marker_in_one_sentence_match_across_lines(tmp_path, name, text):
    assert run(tmp_path, {name: text}) == [f"{name}:1: defect or limit about the target tool"]


def test_separate_sentences_and_paragraphs_more_than_a_line_apart_do_not_match(tmp_path):
    text = ("The Acmepipe manual documents batch runs. It has\nmany chapters on\ninputs. A run was crashing.\n\n"
            "The Acmepipe manual is public.\n\nA run was crashing.\n")
    assert run(tmp_path, {"doc/a.md": text}) == []


def test_escaped_continuations_in_docstrings_join(tmp_path):
    code = 'def helper():\n    """Widget \\\n    overflow at startup."""\n'
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:2: private token #1"]



def test_table_rows_and_code_block_lines_are_separate_sentences(tmp_path):
    text = ("| Item | Value |\n|---|---|\n| Target | Acmepipe |\n| Notes | none |\n| Run | crashing |\n\n"
            "```\nacmepipe_out/   Acmepipe files\nx/\nparsed/   crashing\n```\n")
    assert run(tmp_path, {"doc/a.md": text}) == []
    text = "| Target | Acmepipe |\n| Run | crashing |\n"
    assert run(tmp_path / "w", {"doc/a.md": text}) == ["doc/a.md:1: defect or limit about the target tool"]



@pytest.mark.parametrize("text", [
    "Widget <!-- note -->overflow at startup.\n",
    "Widget<br>overflow at startup.\n",
    "Widget<p>overflow at startup.</p>\n",
    "Widget [overflow](https://example.invalid/a_(b_(c))) at startup.\n",
])
def test_html_comments_breaks_and_nested_destinations_do_not_hide_a_phrase(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: private token #1"]


def test_html_comment_bodies_are_screened(tmp_path):
    assert run(tmp_path, {"doc/a.md": "Generic text <!-- Widget overflow at startup --> here.\n"}) == [
        "doc/a.md:1: private token #1"]


def test_a_continuation_that_splits_a_word_reads_it_whole(tmp_path):
    code = 'def helper():\n    """The Acme\\\npipe run was crashing."""\n'
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:2: defect or limit about the target tool"]
    code = 'def helper():\n    """Two\\\n    words stay apart."""\n'
    assert check.python_prose_lines(code)[3].lstrip("\x00").startswith("    words")



@pytest.mark.parametrize("text", [
    'Widget [overflow](notes.md "1) Details") at startup.\n',
    "Widget [overflow](notes.md '(a) b') at startup.\n",
    "Widget [overflow](notes\\).md) at startup.\n",
    "Widget [overflow](<notes (1).md>) at startup.\n",
    'Widget <span title="usage > capacity">overflow</span> at startup.\n',
    "Widget<br class='a > b'>overflow at startup.\n",
])
def test_link_titles_escapes_and_quoted_attributes_do_not_hide_a_phrase(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: private token #1"]


def test_whitespace_before_a_continuation_still_separates_words(tmp_path):
    code = '"""Acmepipe \\\nwas crashing."""\n'
    assert run(tmp_path, {"pkg/mod.py": code}) == ["pkg/mod.py:1: defect or limit about the target tool"]



@pytest.mark.parametrize("name, text", [
    ("doc/a.md", "The Acmepipe run used default settings, i.e.\nunchanged tolerances and limits,\nand was crashing to stop early.\n"),
    ("pkg/mod.py", "# The Acmepipe run used defaults, e.g.\n# unchanged tolerances and limits,\n# and was crashing to stop.\n"),
    ("src/lib.rs", "// The Acmepipe run (cf. the notes, etc.)\n// with unchanged tolerances and limits\n// was crashing to stop.\n"),
])
def test_abbreviations_do_not_end_a_sentence(tmp_path, name, text):
    assert run(tmp_path, {name: text}) == [f"{name}:1: defect or limit about the target tool"]


def test_real_sentence_ends_still_separate(tmp_path):
    text = "The Acmepipe manual is public. It has\nmany chapters on\ninputs. A run was crashing.\n"
    assert run(tmp_path, {"doc/a.md": text}) == []


def test_a_markdown_hard_break_does_not_hide_a_phrase(tmp_path):
    assert run(tmp_path, {"doc/a.md": "It printed Widget\\\noverflow at startup.\n"}) == ["doc/a.md:1: private token #1"]


@pytest.mark.parametrize("text", [
    "Acmepipe 9.99 writes displacements to 0.001 mm and forces to 1 N in its CSV.\n",
    "The record states whether the Acmepipe and SWBPIPE values match.\n",
    "Acmepipe results are compared per DEL-17-05 §7.1 (2026-10-26, REQ-018) and matched by node id.\n",
    "Acmepipe's MBF import does not accept FIXR records, so the writer refuses partial rotational restraints.\n",
    "SWBPIPE fails to converge when the model has 3 free nodes.\n",
    "The comparison record carries three values and three pairwise checks, so Acmepipe differs per quantity.\n",
    "Public material about Acmepipe is limited to invented models and the 3 m and 12 m spans.\n",
    "Acme Labs documents the batch route in its public manual.\n",
])
def test_interoperability_facts_and_the_bare_name_pass(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == []


@pytest.mark.parametrize("text", [
    "Acmepipe matched SWBPIPE in 20 of 35 quantities.\n",
    "The external prover's tip deflection differs from SWBPIPE by 0.04 mm.\n",
    "SWBPIPE matches the target tool within 0.5 % on every case.\n",
    "Acmepipe agreed with the hand calculation in 8 of 8 cases.\n",
    "The prover's 12 m residual was\na few ppm, 3 parts per million.\n",
])
def test_results_about_the_target_are_blocked(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: result about the target tool"]


@pytest.mark.parametrize("text", [
    "The batch route of Acmepipe crashes with a gadget fault.\n",
    "The external prover's evaluation build limits models to a small size.\n",
    "Acmepipe solves are slow on large models.\n",
    "Outputs of the target tool carry an evaluation stamp.\n",
    "Acmepipe 9.99 needs a workaround for its batch CSV.\n",
])
def test_defects_and_limits_of_the_target_are_blocked(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: defect or limit about the target tool"]


def test_screenshot_references_about_the_target_or_survey_are_blocked(tmp_path):
    assert run(tmp_path, {"doc/a.md": "See the Acmepipe screenshot of the results dialog.\n"}) == ["doc/a.md:1: screenshot reference"]
    assert run(tmp_path / "c", {"pkg/mod.py": "# from the survey's screen capture\nX = 1\n"}) == ["pkg/mod.py:1: screenshot reference"]
    assert run(tmp_path / "g", {"doc/a.md": "Keep generated reports and screenshots private by default.\n"}) == []


def test_a_neighbouring_line_counts_as_the_same_context(tmp_path):
    text = "Acmepipe 9.99 was run on the pilot cases.\nIts batch route crashed on every model.\n"
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: defect or limit about the target tool"]



def test_policy_text_is_not_screened_and_must_be_paired(tmp_path):
    text = ("<!-- policy-text -->Acmepipe defects, trial limits and screenshots stay private.<!-- /policy-text -->\n"
            "The Acmepipe trial crashed.\n")
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:2: defect or limit about the target tool"]
    with pytest.raises(ValueError):
        run(tmp_path / "u", {"doc/a.md": "<!-- policy-text -->\nAcmepipe crashed.\n"})


def test_the_check_does_not_screen_its_own_source(tmp_path):
    files = {"tools/caepipe_correlation/check_public_observations.py": "# Acmepipe crashes are screened here.\n",
             "tests/test_check_public_observations.py": "# Acmepipe crashed.\n"}
    assert run(tmp_path, files) == []


def test_a_unit_ratio_is_not_a_result(tmp_path):
    assert run(tmp_path, {"doc/a.md": "An Acmepipe difference uses standard gravity 9.80665 m/s2 as assumed.\n"}) == []



@pytest.mark.parametrize("text", [
    "Acmepipe differs from SWBPIPE by 3 mm at the tip.\n",
    "Acmepipe matched all 12 quantities.\n",
])
def test_integer_results_and_counts_are_blocked(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: result about the target tool"]


@pytest.mark.parametrize("text", [
    "Acmepipe CSV sections have 2 rows of headers.\n",
    "The importer converts Acmepipe inch lengths at 25.4 mm per inch, so values match exactly.\n",
    "Acmepipe weights in kgf convert at 9.80665 N/kgf, and the writer differs only in units.\n",
    "The survey notes list a trial of the editor's undo stack.\n",
    "The Acmepipe user's manual gives no default for the gravitational acceleration.\n",
])
def test_format_facts_conversions_and_survey_context_pass(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == []


def test_a_row_limit_is_blocked(tmp_path):
    assert run(tmp_path, {"doc/a.md": "Acmepipe imports at most 20 rows.\n"}) == [
        "doc/a.md:1: defect or limit about the target tool"]


def test_private_tokens_are_screened_inside_exempt_spans_and_the_checks_own_files(tmp_path):
    text = "<!-- policy-text -->Rule text: Widget overflow at start.<!-- /policy-text -->\n"
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: private token #1"]
    text = "<!-- owner-words -->\"Widget overflow at start\"<!-- /owner-words -->\n"
    assert run(tmp_path / "o", {"doc/a.md": text}) == ["doc/a.md:1: private token #1"]
    files = {"tools/caepipe_correlation/check_public_observations.py": "# Widget overflow at start\n"}
    assert run(tmp_path / "s", files) == ["tools/caepipe_correlation/check_public_observations.py:1: private token #1"]



def test_signed_results_are_blocked(tmp_path):
    assert run(tmp_path, {"doc/a.md": "The Acmepipe residual is -3 mm against SWBPIPE.\n"}) == [
        "doc/a.md:1: result about the target tool"]


@pytest.mark.parametrize("text", [
    "The Acmepipe importer matches 3 values to X/Y/Z.\n",
    "Acmepipe units differ: 1 in = 25.4 mm.\n",
    "Acmepipe writes no delimiter after the final field.\n",
    "See [REQ-018](notes.md): Acmepipe records match per DEL-17-04.\n",
])
def test_mapping_counts_equations_and_format_omissions_pass(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == []


def test_image_links_of_the_target_are_screenshot_references(tmp_path):
    assert run(tmp_path, {"doc/a.md": "![Acmepipe dialog](screenshots/result.png)\n"}) == [
        "doc/a.md:1: screenshot reference"]
    assert run(tmp_path / "d", {"doc/a.md": "![Load path diagram](diagram.png)\n"}) == []



def test_a_result_equality_is_not_a_unit_conversion(tmp_path):
    assert run(tmp_path, {"doc/a.md": "Acmepipe agreement: 12/12 = 100%.\n"}) == ["doc/a.md:1: result about the target tool"]


def test_mapping_with_a_scope_word_passes(tmp_path):
    assert run(tmp_path, {"doc/a.md": "The Acmepipe importer matches all 3 values to X/Y/Z.\n"}) == []


def test_reference_style_images_of_the_target_are_screenshot_references(tmp_path):
    text = "![Acmepipe dialog][shot]\n\n[shot]: result.png\n"
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: screenshot reference"]
    assert run(tmp_path / "s", {"doc/a.md": "![Acmepipe dialog]\n"}) == ["doc/a.md:1: screenshot reference"]


def test_original_format_diagrams_pass(tmp_path):
    assert run(tmp_path, {"doc/a.md": "![Acmepipe MBF record layout](layout.svg)\n"}) == []
    assert run(tmp_path / "r", {"doc/a.md": "![Acmepipe record diagram][d]\n\n[d]: d.png\n"}) == []



@pytest.mark.parametrize("text", [
    "Acmepipe and SWBPIPE agree: 3 mm = 3 mm.\n",
    "Acmepipe matched all 12 values to 3 decimal places.\n",
])
def test_same_unit_equalities_and_precision_matches_are_results(tmp_path, text):
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: result about the target tool"]


def test_screenshot_cues_beat_generic_diagram_words(tmp_path):
    assert run(tmp_path, {"doc/a.md": "![Acmepipe layout window](screenshots/run.png)\n"}) == [
        "doc/a.md:1: screenshot reference"]


def test_reference_image_destinations_are_resolved(tmp_path):
    assert run(tmp_path, {"doc/a.md": "![Acmepipe MBF][d]\n\n[d]: layout.svg\n"}) == []
    assert run(tmp_path / "s", {"doc/a.md": "![Acmepipe MBF][d]\n\n[d]: shots/run.png\n"}) == [
        "doc/a.md:1: screenshot reference"]



def test_a_changed_image_definition_alone_is_screened(tmp_path):
    repo = repo_with(tmp_path, {"doc/a.md": "![Acmepipe MBF][d]\n\n[d]: layout.svg\n"})
    git(repo, "branch", "-f", "main", "topic")
    (repo / "doc/a.md").write_text("![Acmepipe MBF][d]\n\n[d]: shots/run.png\n", encoding="utf-8")
    git(repo, "commit", "-q", "-am", "swap")
    assert check.check_range(repo, "main", "topic", check.load_terms(write_terms(tmp_path))) == [
        "doc/a.md:1: screenshot reference"]


def test_equations_reporting_agreement_are_results(tmp_path):
    assert run(tmp_path, {"doc/a.md": "Acmepipe and Otherpipe agree: 3 mm = 0.3 cm.\n"}) == [
        "doc/a.md:1: result about the target tool"]


def test_a_mapping_to_listed_fields_passes(tmp_path):
    assert run(tmp_path, {"doc/a.md": "The Acmepipe importer matches all 3 values to `x`, `y`, and `z` fields.\n"}) == []



def test_a_reference_image_keeps_its_own_line_context(tmp_path):
    assert run(tmp_path, {"doc/a.md": "Acmepipe output.\n![View][d]\n\n[d]: run.png\n"}) == [
        "doc/a.md:1: screenshot reference"]


def test_a_precision_clause_is_not_a_mapping(tmp_path):
    text = "Acmepipe matches all 12 values to 3 decimal places in the output fields.\n"
    assert run(tmp_path, {"doc/a.md": text}) == ["doc/a.md:1: result about the target tool"]
